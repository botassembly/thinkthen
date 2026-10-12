//! Owned row snapshots from the existing fallible native batch scheduler.
use super::{ResultHandle, execute, inputs, rows, structured};
use crate::current::{QuestionHandle, SourceHandle, Storage};
use crate::failures::Failure;
use crate::ffi::carriers::RowObservationV1;
use crate::ffi::values::{
    THINKTHEN_FUNCTION_ANNOTATE_V1 as ANNOTATE, THINKTHEN_FUNCTION_CHOOSE_V1 as CHOOSE,
    THINKTHEN_FUNCTION_DECIDE_V1 as DECIDE, THINKTHEN_FUNCTION_FILTER_V1 as FILTER,
    THINKTHEN_FUNCTION_SCORE_V1 as SCORE, THINKTHEN_FUNCTION_TAG_V1 as TAG,
};
use std::sync::{Arc, Mutex, PoisonError};
use thinkthen::{CallOptions, Engine, Facts, OwnedRecordObservation};
pub(crate) struct NativeBatch<'a>(thinkthen::RequestRows<'a, inputs::Original>);
impl NativeBatch<'_> {
    pub(crate) fn facts(&self) -> Option<&Facts> {
        self.0.facts()
    }
}
impl std::fmt::Debug for NativeBatch<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeBatch").finish_non_exhaustive()
    }
}
pub(crate) fn begin<'a>(
    engine: &'a Engine,
    kind: u32,
    question: &'a QuestionHandle,
    source: &'a SourceHandle,
    options: CallOptions<'a>,
) -> Result<NativeBatch<'a>, Failure> {
    let request = super::request::prepare(
        kind,
        question,
        thinkthen::RequestInput::Feed {
            name: "c-batch".into(),
            framing: Default::default(),
            reading: Default::default(),
            images: Vec::new(),
        },
    )?
    .admit()?;
    let reading = question
        .reading
        .clone()
        .unwrap_or(request.record_reading()?);
    let records = source.read()?.enumerate().map(move |(at, record)| {
        engine.check_record_limit(at)?;
        inputs::compose(record?, kind, Some(&reading))
    });

    Ok(NativeBatch(
        engine.execute_request_rows(&request, records, options)?,
    ))
}

pub(crate) type Events = Arc<Mutex<Vec<OwnedRecordObservation>>>;
fn finish(
    events: &Events,
    index: usize,
    kind: u32,
    convert: impl FnOnce(&mut Storage, &[OwnedRecordObservation]) -> Result<RowObservationV1, Failure>,
) -> Result<ResultHandle, Failure> {
    let mut pending = events.lock().unwrap_or_else(PoisonError::into_inner);
    let (selected, rest) =
        std::mem::take(&mut *pending)
            .into_iter()
            .partition(|event| match event {
                OwnedRecordObservation::Question { index: at, .. }
                | OwnedRecordObservation::Row { index: at, .. } => *at == index,
            });
    *pending = rest;
    drop(pending);
    let mut storage = Storage::default();
    let row = convert(&mut storage, &selected)?;
    execute::result(storage, vec![row], &selected, kind, None)
}
impl NativeBatch<'_> {
    pub(crate) fn next(&mut self, events: &Events) -> Option<Result<ResultHandle, Failure>> {
        macro_rules! pull {
            ($batch:expr, $kind:expr, $convert:ident) => {
                $batch.next().map(|result| {
                    let row = result?;
                    finish(events, row.ordinal(), $kind, |storage, _| {
                        rows::$convert(storage, &row)
                    })
                })
            };
        }
        match &mut self.0 {
            thinkthen::RequestRows::Decisions(batch) => pull!(batch, DECIDE, decide),
            thinkthen::RequestRows::Choices(batch) => pull!(batch, CHOOSE, choose),
            thinkthen::RequestRows::Tags(batch) => pull!(batch, TAG, tag),
            thinkthen::RequestRows::Scores(batch) => pull!(batch, SCORE, score),
            thinkthen::RequestRows::Filtered(batch) => pull!(batch, FILTER, filter),
            thinkthen::RequestRows::Annotations(batch) => batch.next().map(|result| {
                let row = result?;
                finish(events, row.ordinal(), ANNOTATE, |storage, selected| {
                    structured::annotate(storage, &row, selected)
                })
            }),
        }
    }
}
pub(crate) fn final_result(kind: u32, facts: &Facts) -> Result<ResultHandle, Failure> {
    execute::result(Storage::default(), Vec::new(), &[], kind, Some(facts))
}
