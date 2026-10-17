//! Recovering atomic rows use the same preparation, packing and canonical renderer.
use super::records::{Records, complete_row, prepare_record};
use crate::engine::pipeline::{self, Asker, Flow};
use crate::public::{
    Call, CallOptions, Engine, Error, InputFunction, QuestionInput, RecordInput,
    RequestRecoveredRow, RequestValue,
};
use crate::public::{options::Stop, pull};
use std::sync::Arc;

impl Engine {
    pub(in crate::public) fn recover_atomic(
        &self,
        function: InputFunction,
        question: &crate::Question,
        records: Vec<Result<RecordInput<QuestionInput>, Error>>,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<RequestRecoveredRow>>, Error> {
        let setting = crate::public::bulk::selected_batch(question, &options, self.batch)?;
        let engine = self.asking(question)?;
        let asker = Records(Arc::clone(&engine), true);
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let (admitted, rejected) = prepare(records, |ordinal, record| {
            let (held, prepared) =
                prepare_record(function, question, record, options.context_text(), ordinal)?;
            asker
                .asks(&prepared)
                .map_err(super::records::pipeline_failure)?;
            Ok((held, prepared))
        })?;
        let mut packing = pull::packing(setting, false, true);
        packing.detailed = stop.facts().attempts().is_some();
        run(
            (&engine, packing),
            &asker,
            &stop,
            admitted,
            rejected,
            |ordinal, original, row| {
                let cause = super::records::recovery_cause(&row);
                let outcome = complete_row(
                    (&engine, self.profile.as_ref(), setting, false, function),
                    &stop,
                    &original,
                    ordinal,
                    row,
                )
                .and_then(|canonical| {
                    atomic_value(function, original.original, ordinal, canonical)
                });
                crate::public::request::recovery_row(ordinal, outcome)
                    .map(|row| row.with_cause(cause))
            },
        )
    }
}

impl Engine {
    pub(in crate::public) fn recover_dynamic_choose(
        &self,
        question: &crate::RecordChooseQuestion,
        records: Vec<Result<RecordInput<QuestionInput>, Error>>,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<RequestRecoveredRow>>, Error> {
        let setting = crate::public::bulk::selected_batch_file(
            question.batch.as_ref(),
            &options,
            self.batch,
        )?;
        let engine = self.for_model(question.model.as_ref())?;
        let asker = Records(Arc::clone(&engine), true);
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let (admitted, rejected) = prepare(records, |at, record| {
            let (held, prepared) =
                super::dynamic_choose::prepare_parts(question, record, options.context_text(), at)?;
            asker
                .asks(&prepared)
                .map_err(super::records::pipeline_failure)?;
            Ok((held, prepared))
        })?;
        let mut packing = pull::packing(setting, false, true);
        packing.detailed = stop.facts().attempts().is_some();
        run(
            (&engine, packing),
            &asker,
            &stop,
            admitted,
            rejected,
            |at, held, row| {
                let cause = super::records::recovery_cause(&row);
                let outcome = complete_row(
                    (
                        &engine,
                        self.profile.as_ref(),
                        setting,
                        false,
                        InputFunction::Choose,
                    ),
                    &stop,
                    &held,
                    at,
                    row,
                )
                .and_then(|canonical| {
                    atomic_value(InputFunction::Choose, held.original, at, canonical)
                });
                crate::public::request::recovery_row(at, outcome).map(|row| row.with_cause(cause))
            },
        )
    }
}

pub(super) fn atomic_value(
    function: InputFunction,
    original: QuestionInput,
    ordinal: usize,
    canonical: crate::core::CompleteAtomic,
) -> Result<RequestValue, Error> {
    use crate::CompleteRecord;
    macro_rules! row {
        ($variant:ident, $convert:path) => {
            RequestValue::$variant(vec![CompleteRecord {
                original,
                ordinal,
                result: $convert(canonical)?,
            }])
        };
    }
    Ok(match function {
        InputFunction::Decide => row!(Decisions, super::decision),
        InputFunction::Choose => row!(Choices, super::choice),
        InputFunction::Score => row!(Scores, super::score),
        InputFunction::Tag => row!(Tags, super::tags),
        InputFunction::Filter => row!(Filtered, super::records::filter),
        _ => {
            return Err(Error::defect(
                "a recovering atomic row has another function",
            ));
        }
    })
}

pub(super) type Admitted<H, P> = Vec<(usize, H, P)>;
type PreparedRows<H, P> = (Admitted<H, P>, Vec<RequestRecoveredRow>);
pub(super) fn prepare<H, P>(
    records: Vec<Result<RecordInput<QuestionInput>, Error>>,
    mut prepare: impl FnMut(usize, RecordInput<QuestionInput>) -> Result<(H, P), Error>,
) -> Result<PreparedRows<H, P>, Error> {
    let mut admitted = Vec::new();
    for (ordinal, record) in records.into_iter().enumerate() {
        let (held, prepared) = record
            .and_then(|record| prepare(ordinal, record))
            .map_err(|error| error.at_record(ordinal))?;
        admitted.push((ordinal, held, prepared));
    }
    Ok((admitted, Vec::new()))
}

pub(super) fn run<A: Asker, H>(
    (engine, packing): (&crate::engine::facade::Engine, pipeline::Packing),
    asker: &A,
    stop: &Stop<'_>,
    admitted: Admitted<H, A::Input>,
    rejected: Vec<RequestRecoveredRow>,
    mut project: impl FnMut(usize, H, pipeline::Row<A>) -> Result<RequestRecoveredRow, Error>,
) -> Result<Call<Vec<RequestRecoveredRow>>, Error> {
    stop.run_call(0, |cancel| {
        let (held, inputs): (Vec<_>, Vec<_>) = admitted
            .into_iter()
            .map(|(ordinal, held, prepared)| ((ordinal, held), prepared))
            .unzip();
        let mut held = held.into_iter();
        let mut output = rejected;
        let mut fatal = None;
        let host = pipeline::eager(inputs, |row| {
            let Some((ordinal, original)) = held.next() else {
                fatal = Some(Error::defect("a recovering row lost its original"));
                return Flow::Stop;
            };
            let outcome = project(ordinal, original, row);
            match outcome {
                Ok(row) => {
                    output.push(row);
                    cancel.finished_records(1);
                }
                Err(error) => {
                    fatal = Some(error);
                    return Flow::Stop;
                }
            }
            Flow::Continue
        });
        engine
            .ask_all(asker, packing, host, cancel)
            .map_err(Error::from)?;
        if let Some(error) = fatal {
            return Err(error);
        }
        output.sort_by_key(RequestRecoveredRow::ordinal);
        Ok(output)
    })
}
