//! All ten named routes delegate to one engine and its existing schedulers.
use super::{ResultHandle, inputs, metadata, observations, rows, structured};
use crate::current::{QuestionHandle, SourceHandle, Storage, question::Native};
use crate::failures::Failure;
use crate::ffi::carriers::{
    OptionalDiscriminatorV1, OptionalMetaV1, OptionalStringV1, RowObservationV1, SummaryV1,
};
use crate::ffi::values as abi;
use crate::ffi::values::{
    THINKTHEN_FUNCTION_ANNOTATE_V1 as ANNOTATE, THINKTHEN_FUNCTION_CHOOSE_V1 as CHOOSE,
    THINKTHEN_FUNCTION_DECIDE_V1 as DECIDE, THINKTHEN_FUNCTION_FILTER_V1 as FILTER,
    THINKTHEN_FUNCTION_FIND_V1 as FIND, THINKTHEN_FUNCTION_RANK_V1 as RANK,
    THINKTHEN_FUNCTION_RECOGNIZE_V1 as RECOGNIZE, THINKTHEN_FUNCTION_RELATE_V1 as RELATE,
    THINKTHEN_FUNCTION_SCORE_V1 as SCORE, THINKTHEN_FUNCTION_TAG_V1 as TAG,
};
use std::sync::{Mutex, PoisonError};
use thinkthen::{
    CallOptions, Engine, Facts, LoadedQuestion, OwnedRecordObservation, RecordObservation,
};
pub(crate) fn ask(
    engine: &Engine,
    kind: u32,
    question: &QuestionHandle,
    source: &SourceHandle,
    options: CallOptions<'_>,
    completed: &mut Option<Facts>,
) -> Result<ResultHandle, Failure> {
    if !question.fits_complete(kind) {
        return Err(Failure::usage(
            "the question kind does not match the named call",
        ));
    }
    let events = Mutex::new(Vec::new());
    let observer = |event: RecordObservation<'_>| {
        events
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(event.to_owned());
    };
    let options = options.observe(&observer);
    let mut storage = Storage::default();
    let rows = match (&question.native, kind) {
        (Native::Find(_), FIND) | (Native::Relate(_), RELATE) => {
            let records = source.read()?.enumerate().map(|(at, record)| {
                engine.check_record_limit(at)?;
                inputs::compose(record?, kind, question.reading.as_ref())
            });
            match &question.native {
                Native::Find(q) => {
                    let call = engine.try_find_records_complete_with(q, records, options)?;
                    *completed = Some(call.facts().clone());
                    vec![rows::find(
                        &mut storage,
                        call.value(),
                        &events.lock().unwrap_or_else(PoisonError::into_inner),
                    )?]
                }
                Native::Relate(q) => {
                    let call = engine.try_relate_records_complete_with(q, records, options)?;
                    *completed = Some(call.facts().clone());
                    vec![structured::relate(&mut storage, call.value())?]
                }
                _ => return Err(Failure::defect("whole-set route lost its question")),
            }
        }
        _ => {
            let records = inputs::records(engine, source, kind, question.reading.as_ref())?;
            execute(
                engine,
                kind,
                question,
                records,
                options,
                (&mut storage, &events, completed),
            )?
        }
    };
    let events = events.into_inner().unwrap_or_else(PoisonError::into_inner);
    let facts = completed
        .as_ref()
        .ok_or_else(|| Failure::defect("complete native execution held no final facts"))?;
    result(storage, rows, &events, kind, Some(facts))
}

pub(super) fn result(
    mut storage: Storage,
    rows: Vec<RowObservationV1>,
    events: &[OwnedRecordObservation],
    kind: u32,
    facts_native: Option<&Facts>,
) -> Result<ResultHandle, Failure> {
    let authors = std::mem::take(&mut storage.3);
    let recognition_tasks = std::mem::take(&mut storage.9);
    let member_authors = std::mem::take(&mut storage.4);
    let observation_authors = observations::authors(&mut storage, events, &rows, &authors)?;
    let row_details = std::mem::take(&mut storage.1);
    let observation_details = observation_details(&mut storage, events, &rows, &row_details)?;
    let source_recognition = std::mem::take(&mut storage.6);
    let source_relations = std::mem::take(&mut storage.7);
    let rank_members = std::mem::take(&mut storage.5);
    let rank_member_details = std::mem::take(&mut storage.8);
    let observations = observations::convert(&mut storage, events, &rows)?;
    let facts = facts_native
        .map(|f| metadata::facts(&mut storage, f))
        .transpose()?
        .unwrap_or_default();
    let attempts = storage.attempts(facts_native.and_then(Facts::attempts));
    let singular = if rows.len() == 1 {
        storage.2.first().copied()
    } else {
        None
    };
    let summary = SummaryV1 {
        answer_id: singular
            .map(|row| OptionalStringV1 {
                present: 1,
                value: row.answer_id,
            })
            .unwrap_or_default(),
        meta: singular
            .map(|row| OptionalMetaV1 {
                present: 1,
                value: row.meta,
            })
            .unwrap_or_default(),
        state: abi::THINKTHEN_RESULT_SUCCESS_V1,
        schema: storage.string("thinkthen.result/2"),
        function: OptionalDiscriminatorV1 {
            present: 1,
            value: kind,
        },
        count: rows.len(),
        observation_count: observations.len(),
        facts,
        attempts,
        ..SummaryV1::default()
    };
    Ok(ResultHandle {
        _storage: storage,
        summary,
        rows,
        observations,
        authors,
        recognition_tasks,
        member_authors,
        observation_authors,
        rank_members,
        rank_member_details,
        source_recognition,
        source_relations,
        row_details,
        observation_details,
    })
}
macro_rules! collect {
    ($completed:ident; $call:expr, $convert:expr) => {{
        let call = $call?;
        *$completed = Some(call.facts().clone());
        call.value().iter().map($convert).collect()
    }};
}
fn execute(
    engine: &Engine,
    kind: u32,
    question: &QuestionHandle,
    records: Vec<thinkthen::RecordInput<inputs::Original>>,
    options: CallOptions<'_>,
    (s, events, completed): (
        &mut Storage,
        &Mutex<Vec<OwnedRecordObservation>>,
        &mut Option<Facts>,
    ),
) -> Result<Vec<RowObservationV1>, Failure> {
    match (&question.native, kind) {
        (Native::Atomic(LoadedQuestion::Question(q)), DECIDE) => {
            collect!(completed;
                engine.decide_records_complete_with(q, records, options),
                |r| rows::decide(s, r)
            )
        }
        (Native::Atomic(LoadedQuestion::Banded(q)), DECIDE) => {
            collect!(completed;
                engine.decide_records_complete_with(q, records, options),
                |r| rows::decide(s, r)
            )
        }
        (Native::Atomic(LoadedQuestion::Question(q)), CHOOSE) => {
            collect!(completed;
                engine.choose_records_complete_with(q, records, options),
                |r| rows::choose(s, r)
            )
        }
        (Native::Atomic(LoadedQuestion::Question(q)), TAG) => {
            collect!(completed; engine.tag_records_complete_with(q, records, options), |r| {
                rows::tag(s, r)
            })
        }
        (Native::Atomic(LoadedQuestion::Question(q)), SCORE) => {
            collect!(completed;
                engine.score_records_complete_with(q, records, options),
                |r| rows::score(s, r)
            )
        }
        (Native::Atomic(LoadedQuestion::Question(q)), FILTER) => {
            collect!(completed;
                engine.filter_records_complete_with(q, records, options),
                |r| rows::filter(s, r)
            )
        }
        (Native::Rank(q), RANK) => collect!(completed;
            engine.rank_records_complete_with(q, records, options),
            |r| rank_row(s, r, events)
        ),
        (Native::Atomic(LoadedQuestion::Question(_)), RANK) => {
            let q = question.rank_reading()?;
            collect!(completed;
                engine.rank_records_complete_with(&q, records, options),
                |r| rank_row(s, r, events)
            )
        }
        (Native::DynamicChoose(q), CHOOSE) => {
            collect!(completed; engine.choose_dynamic_records_complete_with(q, records, options), |r| rows::choose(s, r))
        }
        (Native::Set(q), ANNOTATE) => collect!(completed;
            engine.annotate_records_complete_with(q, records, options),
            |r| structured::annotate(s, r, &events.lock().unwrap_or_else(PoisonError::into_inner))
        ),
        (Native::Recognize(q), RECOGNIZE) => {
            collect!(completed;
                engine.recognize_records_complete_with(q, records, options),
                |r| structured::recognize(s, r)
            )
        }
        (Native::RankSet(set), RANK) => {
            collect!(completed;
                engine.rank_set_records_complete_with(set, records, options),
                |r| super::rank_set::row(s, r, &events.lock().unwrap_or_else(PoisonError::into_inner))
            )
        }
        _ => Err(Failure::usage(
            "this question reading is not admitted by the named complete call",
        )),
    }
}

fn observation_details(
    storage: &mut Storage,
    events: &[OwnedRecordObservation],
    rows: &[RowObservationV1],
    row_details: &[crate::ffi::carriers::DetailsV1],
) -> Result<Vec<crate::ffi::carriers::DetailsV1>, Failure> {
    events
        .iter()
        .map(|event| match event {
            OwnedRecordObservation::Question { detail, .. } => {
                storage.question_details(detail.detail())
            }
            OwnedRecordObservation::Row { index, .. } => rows
                .iter()
                .position(|row| row.index == *index)
                .and_then(|at| row_details.get(at))
                .copied()
                .ok_or_else(|| Failure::defect("native row event lost its details")),
        })
        .collect::<Result<Vec<_>, Failure>>()
}

fn rank_row(
    s: &mut Storage,
    row: &thinkthen::CompleteRecord<inputs::Original, thinkthen::CompleteRank>,
    events: &Mutex<Vec<OwnedRecordObservation>>,
) -> Result<RowObservationV1, Failure> {
    let events = events.lock().unwrap_or_else(PoisonError::into_inner);
    rows::rank(
        s,
        row,
        observations::raw(&events, row.ordinal(), None).as_deref(),
    )
}
