//! CLI recognition uses the existing ordered workers with native preparation.
use super::{
    ordered,
    records::{Prepared, Unit, prepare, source_value},
    rendered,
};
use crate::public::{
    Call, CallOptions, CompleteRecognized, CompleteRecord, Engine, Error, Recognize, Recognized,
    RecordInput, RequestValue, options::Stop,
};
use std::{cell::RefCell, collections::BTreeMap};

struct Answered {
    ordinal: usize,
    prepared: Prepared,
    found: crate::engine::facade::Recognition,
    attempts: Vec<crate::core::AttemptObservation>,
}

impl Engine {
    #[expect(
        clippy::type_complexity,
        clippy::too_many_lines,
        reason = "one ordered call retains input admission, caller-owned originals and final facts"
    )]
    pub(crate) fn request_recognize_stream<'a, I>(
        &'a self,
        ask: &'a Recognize,
        records: I,
        options: CallOptions<'a>,
        eager: bool,
        sink: Option<&dyn Fn(RequestValue)>,
    ) -> Result<Call<Vec<CompleteRecord<crate::QuestionInput, CompleteRecognized>>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<crate::QuestionInput>, Error>> + 'a,
    {
        ask.0.validate_mode().map_err(Error::refused)?;
        let options = options.started()?;
        options.admission()?;
        let engine =
            crate::public::complete::contextual(self.for_model(ask.0.model.as_ref())?, &options)?;
        let limit = options
            .cli_text_limit
            .unwrap_or(crate::engine::facade::MAX_TEXT_BYTES);
        let units = records.into_iter().enumerate().map(|(at, record)| {
            options.admission()?;
            self.check_record_limit(at)?;
            record
                .and_then(|record| prepare(&engine, ask, record, options.context_text(), limit))
                .map_err(|error| error.at_record(at))
        });
        let mut units: Box<dyn Iterator<Item = Result<Unit<crate::QuestionInput>, Error>> + '_> =
            if eager {
                Box::new(units.collect::<Result<Vec<_>, _>>()?.into_iter().map(Ok))
            } else {
                Box::new(units)
            };
        let stop = Stop::begin(options)?.with_prices(self.prices);
        stop.run_call(0, |cancel| {
            let cancel = cancel.with_storage_scope();
            let originals = RefCell::new(BTreeMap::new());
            let mut count = 0;
            let read = || match units.next() {
                None => Some(ordered::Input::End),
                Some(Err(error)) => Some(ordered::Input::Failed(error)),
                Some(Ok(Unit { original, prepared })) => {
                    let ordinal = count;
                    count += 1;
                    originals.borrow_mut().insert(ordinal, original);
                    Some(ordered::Input::Item((ordinal, prepared)))
                }
            };
            let mut rows = Vec::new();
            let outcome = ordered::run(
                engine.width(&cancel).map_err(Error::from)?,
                &cancel,
                options.cli_reader,
                read,
                &|(ordinal, prepared)| answer(ordinal, prepared, limit, &cancel),
                |answered| {
                    let original = originals
                        .borrow_mut()
                        .remove(&answered.ordinal)
                        .ok_or_else(|| Error::defect("recognize lost its original"))?;
                    let value = Recognized::from_native(answered.found.value.clone())?;
                    stop.observe(crate::RecordObservation::Row {
                        index: answered.ordinal,
                        value: crate::ObservedRow::Recognized(&value),
                    });
                    let mut result = rendered(
                        &answered.prepared.engine,
                        &answered.prepared.ask,
                        answered.found,
                        value,
                        (
                            answered.ordinal,
                            answered.prepared.context.as_deref(),
                            Some(answered.attempts),
                        ),
                    )?;
                    result.canonical.source =
                        super::super::physical_source(&answered.prepared.input);
                    result.source_value =
                        source_value(&result.canonical.value, &answered.prepared)?;
                    let row = CompleteRecord {
                        original,
                        ordinal: answered.ordinal,
                        result,
                    };
                    match sink {
                        Some(sink) => sink(RequestValue::Recognized(vec![row])),
                        None => rows.push(row),
                    }
                    Ok(true)
                },
                &Error::from,
                || Error::defect("recognize's ordered workers ended early"),
            )?;
            match outcome {
                ordered::Outcome::Complete => Ok(rows),
                ordered::Outcome::Stopped {
                    finished: _finished,
                    replayed: _replayed,
                    cause,
                } => Err(cause),
            }
        })
    }
}

fn answer(
    ordinal: usize,
    prepared: Prepared,
    limit: usize,
    cancel: &crate::engine::Cancel<'_>,
) -> Result<ordered::Row<Answered>, Error> {
    let mut attempts = BTreeMap::new();
    let found = prepared
        .engine
        .recognize_observed(
            &prepared.ask.0,
            &prepared.text,
            limit,
            cancel,
            |_, _, answered| {
                for event in &answered.attempts {
                    attempts.insert(event.ordinal(), event.clone());
                }
                Ok(())
            },
        )
        .map_err(|error| Error::from(error).at_record(ordinal))?;
    Ok(ordered::Row {
        replayed: !found.meta.live,
        value: Answered {
            ordinal,
            prepared,
            found,
            attempts: attempts.into_values().collect(),
        },
    })
}
