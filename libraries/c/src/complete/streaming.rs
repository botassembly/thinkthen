//! Owned row snapshots from the existing fallible native batch scheduler.
use super::{ResultHandle, execute, inputs, rows, structured};
use crate::current::{QuestionHandle, SourceHandle, Storage, question::Native};
use crate::failures::Failure;
use crate::ffi::carriers::RowObservationV1;
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
        (Native::Atomic(LoadedQuestion::Question(q)), 1) => {
            NativeBatch::Decide(engine.try_decide_records_complete_with(q, records, options))
        }
        (Native::Atomic(LoadedQuestion::Banded(q)), 1) => {
            NativeBatch::Decide(engine.try_decide_records_complete_with(q, records, options))
        }
        (Native::Atomic(LoadedQuestion::Question(q)), 2) => {
            NativeBatch::Choose(engine.try_choose_records_complete_with(q, records, options))
        }
        (Native::Atomic(LoadedQuestion::Question(q)), 3) => {
            NativeBatch::Tag(engine.try_tag_records_complete_with(q, records, options))
        }
        (Native::Atomic(LoadedQuestion::Question(q)), 4) => {
            NativeBatch::Score(engine.try_score_records_complete_with(q, records, options))
        }
        (Native::Atomic(LoadedQuestion::Question(q)), 5) => {
            NativeBatch::Filter(engine.try_filter_records_complete_with(q, records, options))
        }
        (Native::Set(q), 8) => {
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
            ($batch:expr, $kind:literal, $convert:ident) => {
                $batch.next().map(|result| {
                    let row = result?;
                    finish(events, row.ordinal(), $kind, |storage, _| {
                        rows::$convert(storage, &row)
                    })
                })
            };
        }
        match self {
            Self::Decide(batch) => pull!(batch, 1, decide),
            Self::Choose(batch) => pull!(batch, 2, choose),
            Self::Tag(batch) => pull!(batch, 3, tag),
            Self::Score(batch) => pull!(batch, 4, score),
            Self::Filter(batch) => pull!(batch, 5, filter),
            Self::Annotate(batch) => batch.next().map(|result| {
                let row = result?;
                finish(events, row.ordinal(), 8, |storage, selected| {
                    structured::annotate(storage, &row, selected)
                })
            }),
        }
    }
}
pub(crate) fn final_result(kind: u32, facts: &Facts) -> Result<ResultHandle, Failure> {
    execute::result(Storage::default(), Vec::new(), &[], kind, Some(facts))
}
