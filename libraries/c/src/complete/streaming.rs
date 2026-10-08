//! Owned row snapshots from the existing fallible native batch scheduler.
use super::{ResultHandle, execute, inputs, rows, structured};
use crate::current::{QuestionHandle, SourceHandle, Storage, question::Native};
use crate::failures::Failure;
use crate::ffi::carriers::RowObservationV1;
use crate::ffi::values::{
    THINKTHEN_FUNCTION_ANNOTATE_V1 as ANNOTATE, THINKTHEN_FUNCTION_CHOOSE_V1 as CHOOSE,
    THINKTHEN_FUNCTION_DECIDE_V1 as DECIDE, THINKTHEN_FUNCTION_FILTER_V1 as FILTER,
    THINKTHEN_FUNCTION_SCORE_V1 as SCORE, THINKTHEN_FUNCTION_TAG_V1 as TAG,
};
use std::sync::{Arc, Mutex, PoisonError};
use thinkthen::{
    Batch, CallOptions, CompleteRecord, Engine, Facts, LoadedQuestion, OwnedRecordObservation,
};
type Record<T> = CompleteRecord<inputs::Original, T>;
macro_rules! batches {
    ($($name:ident: $ty:ident),+ $(,)?) => {
        pub(crate) enum NativeBatch<'a> { $($name(Batch<'a, Record<thinkthen::$ty>>)),+ }
        impl NativeBatch<'_> {
            pub(crate) fn facts(&self) -> Option<&Facts> {
                match self { $(Self::$name(batch) => batch.facts()),+ }
            }
        }
    };
}
batches! { Decide: CompleteDecision, Choose: CompleteChoice, Tag: CompleteTags,
Score: CompleteScore, Filter: CompleteFilter, Annotate: CompleteAnnotated }
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
    if !question.fits_complete(kind) {
        return Err(Failure::usage(
            "the question kind does not match the named call",
        ));
    }
    let reading = question.reading.as_ref();
    let records = source.read()?.enumerate().map(move |(at, record)| {
        engine.check_record_limit(at)?;
        inputs::compose(record?, kind, reading)
    });
    Ok(match (&question.native, kind) {
        (Native::Atomic(LoadedQuestion::Question(q)), DECIDE) => {
            NativeBatch::Decide(engine.try_decide_records_complete_with(q, records, options))
        }
        (Native::Atomic(LoadedQuestion::Banded(q)), DECIDE) => {
            NativeBatch::Decide(engine.try_decide_records_complete_with(q, records, options))
        }
        (Native::Atomic(LoadedQuestion::Question(q)), CHOOSE) => {
            NativeBatch::Choose(engine.try_choose_records_complete_with(q, records, options))
        }
        (Native::Atomic(LoadedQuestion::Question(q)), TAG) => {
            NativeBatch::Tag(engine.try_tag_records_complete_with(q, records, options))
        }
        (Native::Atomic(LoadedQuestion::Question(q)), SCORE) => {
            NativeBatch::Score(engine.try_score_records_complete_with(q, records, options))
        }
        (Native::Atomic(LoadedQuestion::Question(q)), FILTER) => {
            NativeBatch::Filter(engine.try_filter_records_complete_with(q, records, options))
        }
        (Native::Set(q), ANNOTATE) => {
            NativeBatch::Annotate(engine.try_annotate_records_complete_with(q, records, options))
        }
        _ => {
            return Err(Failure::usage(
                "this question reading has no native lazy record route",
            ));
        }
    })
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
        match self {
            Self::Decide(batch) => pull!(batch, DECIDE, decide),
            Self::Choose(batch) => pull!(batch, CHOOSE, choose),
            Self::Tag(batch) => pull!(batch, TAG, tag),
            Self::Score(batch) => pull!(batch, SCORE, score),
            Self::Filter(batch) => pull!(batch, FILTER, filter),
            Self::Annotate(batch) => batch.next().map(|result| {
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
