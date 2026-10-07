//! All ten named routes delegate to one engine and its existing schedulers.
use super::{ResultHandle, inputs, metadata, observations, rows, structured};
use crate::current::{QuestionHandle, SourceHandle, Storage, question::Native};
use crate::failures::Failure;
use crate::ffi::carriers::{
    OptionalDiscriminatorV1, OptionalMetaV1, OptionalStringV1, RowObservationV1, SummaryV1,
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
    let records = inputs::records(engine, source, kind, question.reading.as_ref())?;
    let rows = execute(
        engine,
        kind,
        question,
        records,
        options,
        (&mut storage, &events, completed),
    )?;
    let events = events.into_inner().unwrap_or_else(PoisonError::into_inner);
    let authors = std::mem::take(&mut storage.3);
    let member_authors = std::mem::take(&mut storage.4);
    let observation_authors = observations::authors(&mut storage, &events, &rows, &authors)?;
    let row_details = std::mem::take(&mut storage.1);
    let observation_details = observation_details(&mut storage, &events, &rows, &row_details)?;
    let source_recognition = std::mem::take(&mut storage.6);
    let source_relations = std::mem::take(&mut storage.7);
    let rank_members = std::mem::take(&mut storage.5);
    let observations = observations::convert(&mut storage, &events, &rows)?;
    let f = completed
        .as_ref()
        .ok_or_else(|| Failure::defect("complete native execution held no final facts"))?;
    let facts = metadata::facts(&mut storage, f)?;
    let attempts = storage.attempts(f.attempts());
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
        state: 1,
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
        member_authors,
        observation_authors,
        rank_members,
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
        (Native::Atomic(LoadedQuestion::Question(q)), 1) => collect!(completed;
            engine.decide_records_complete_with(q, records, options),
            |r| rows::decide(s, r)
        ),
        (Native::Atomic(LoadedQuestion::Banded(q)), 1) => collect!(completed;
            engine.decide_records_complete_with(q, records, options),
            |r| rows::decide(s, r)
        ),
        (Native::Atomic(LoadedQuestion::Question(q)), 2) => collect!(completed;
            engine.choose_records_complete_with(q, records, options),
            |r| rows::choose(s, r)
        ),
        (Native::Atomic(LoadedQuestion::Question(q)), 3) => {
            collect!(completed; engine.tag_records_complete_with(q, records, options), |r| {
                rows::tag(s, r)
            })
        }
        (Native::Atomic(LoadedQuestion::Question(q)), 4) => collect!(completed;
            engine.score_records_complete_with(q, records, options),
            |r| rows::score(s, r)
        ),
        (Native::Atomic(LoadedQuestion::Question(q)), 5) => collect!(completed;
            engine.filter_records_complete_with(q, records, options),
            |r| rows::filter(s, r)
        ),
        (Native::Rank(q), 6) => collect!(completed;
            engine.rank_records_complete_with(q, records, options),
            |r| rows::rank(
                s,
                r,
                observations::raw(
                    &events.lock().unwrap_or_else(PoisonError::into_inner),
                    r.ordinal(),
                    None
                )
                .as_deref()
            )
        ),
        (Native::Atomic(LoadedQuestion::Question(_)), 6) => {
            let q = question.rank_reading()?;
            collect!(completed;
                engine.rank_records_complete_with(&q, records, options),
                |r| rows::rank(
                    s,
                    r,
                    observations::raw(
                        &events.lock().unwrap_or_else(PoisonError::into_inner),
                        r.ordinal(),
                        None
                    )
                    .as_deref()
                )
            )
        }
        (Native::DynamicChoose(q), 2) => {
            collect!(completed; engine.choose_dynamic_records_complete_with(q, records, options), |r| rows::choose(s, r))
        }
        (Native::Find(q), 7) => {
            let call = engine.find_records_complete_with(q, records, options)?;
            *completed = Some(call.facts().clone());
            Ok(vec![rows::find(
                s,
                call.value(),
                &events.lock().unwrap_or_else(PoisonError::into_inner),
            )?])
        }
        (Native::Set(q), 8) => collect!(completed;
            engine.annotate_records_complete_with(q, records, options),
            |r| structured::annotate(s, r, &events.lock().unwrap_or_else(PoisonError::into_inner))
        ),
        (Native::Recognize(q), 9) => collect!(completed;
            engine.recognize_records_complete_with(q, records, options),
            |r| structured::recognize(s, r)
        ),
        (Native::Relate(q), 10) => {
            let call = engine.relate_records_complete_with(q, records, options)?;
            *completed = Some(call.facts().clone());
            Ok(vec![structured::relate(s, call.value())?])
        }
        (Native::RankSet(set), 6) => collect!(completed;
            engine.rank_set_records_complete_with(set, records, options),
            |r| super::rank_set::row(s, r, &events.lock().unwrap_or_else(PoisonError::into_inner))
        ),
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
