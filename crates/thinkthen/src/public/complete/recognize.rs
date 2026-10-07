//! Complete recognition delegates all stages and offsets to the existing scheduler.
use super::{
    contextual,
    trace::{Totals, meta},
};
use crate::core;
use crate::public::options::Stop;
use crate::public::{
    Call, CallOptions, CompleteRecognized, Engine, Error, InputFunction, Recognize, Recognized,
};

impl Engine {
    /// Retain every stage distribution, actual observation and resolved recognition reading.
    /// # Errors
    /// The existing size, kind, stage failure and cancellation boundaries apply.
    pub fn recognize_complete_with(
        &self,
        ask: &Recognize,
        evidence: &str,
        options: CallOptions<'_>,
    ) -> Result<Call<CompleteRecognized>, Error> {
        ask.0
            .metadata
            .validate_item(&crate::public::QuestionInput::Text(evidence.to_owned()))?;
        let engine = contextual(self.for_model(ask.0.model.as_ref())?, &options)?;
        let stop = Stop::begin(options)?.with_prices(self.prices);
        stop.run_call(1, |cancel| {
            let found =
                execute(&engine, ask, evidence, cancel, &stop, (0, None)).map_err(Error::from)?;
            let value = Recognized::from_native(found.value.clone())?;
            stop.observe(crate::public::RecordObservation::Row {
                index: 0,
                value: crate::public::ObservedRow::Recognized(&value),
            });
            Ok((found, value))
        })?
        .try_map(|(found, value)| {
            rendered(
                &engine,
                ask,
                found,
                value,
                (0, &options, stop.facts().attempts().map(<[_]>::to_vec)),
            )
        })
    }
}

pub(super) fn rendered(
    engine: &crate::engine::facade::Engine,
    ask: &Recognize,
    found: crate::engine::facade::Recognition,
    value: Recognized,
    (ordinal, options, attempts): (
        usize,
        &CallOptions<'_>,
        Option<Vec<core::AttemptObservation>>,
    ),
) -> Result<CompleteRecognized, Error> {
    let identity = found
        .meta
        .trace
        .identity(
            InputFunction::Recognize,
            &core::RecordScope { record: ordinal },
            &ask.0,
            &[],
        )
        .map_err(|_| super::wrong())?;
    let aggregate = found.meta;
    let meta = meta(
        engine,
        core::recognize_sha256(&ask.0).map_err(|_| super::wrong())?,
        Totals {
            model: aggregate.model,
            usage: aggregate.usage,
            reported: aggregate.reported_usage,
            cached: !aggregate.live,
            sent: aggregate.requests_sent,
            requests: aggregate.requests,
            failed: 0,
        },
        options,
        attempts,
        ask.0.profile.as_ref(),
    );
    Ok(CompleteRecognized {
        canonical: core::CompleteRecognition {
            identity,
            value: found.value,
            input: None,
            question: ask.0.clone(),
            answer: found.details,
            meta,
        },
        value,
    })
}

pub(super) fn execute(
    engine: &crate::engine::facade::Engine,
    ask: &Recognize,
    evidence: &str,
    cancel: &crate::engine::Cancel<'_>,
    stop: &Stop<'_>,
    scope: (usize, Option<&std::sync::Arc<crate::public::QuestionInput>>),
) -> Result<crate::engine::facade::Recognition, crate::engine::error::Error> {
    let mut positions = [0; 4];
    engine.recognize_observed(
        &ask.0,
        evidence,
        crate::engine::facade::MAX_TEXT_BYTES,
        cancel,
        |stage, question, answered| {
            crate::public::results::observe_question_at(
                stop,
                engine.backend(),
                (
                    InputFunction::Recognize,
                    stage,
                    question,
                    match stage {
                        "boundary" => None,
                        "relation" => Some(ask.0.relation_threshold),
                        _ => Some(ask.0.threshold),
                    },
                ),
                answered,
                &mut positions,
                scope,
            )
        },
    )
}
mod records;
