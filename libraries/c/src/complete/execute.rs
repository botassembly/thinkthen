//! All ten named routes delegate to one engine and its existing schedulers.
use super::{ResultHandle, inputs, metadata, observations, rows, structured};
use crate::current::{QuestionHandle, SourceHandle, Storage};
use crate::failures::Failure;
use crate::ffi::carriers::{
    OptionalDiscriminatorV1, OptionalMetaV1, OptionalStringV1, RowObservationV1, SummaryV1,
};
use crate::ffi::values as abi;
use std::sync::{Mutex, PoisonError};
use thinkthen::{CallOptions, Engine, Facts, OwnedRecordObservation, RecordObservation};
pub(crate) fn ask(
    engine: &Engine,
    kind: u32,
    question: &QuestionHandle,
    source: &SourceHandle,
    options: CallOptions<'_>,
    completed: &mut Option<Facts>,
) -> Result<ResultHandle, Failure> {
    let request = super::request::prepare(
        kind,
        question,
        thinkthen::RequestInput::Feed {
            name: "c-complete".into(),
            framing: Default::default(),
            reading: Default::default(),
            images: Vec::new(),
        },
    )?
    .admit()?;
    let reading = request.record_reading()?;
    let reading = question.reading.as_ref().unwrap_or(&reading);
    let events = Mutex::new(Vec::new());
    let observer = |event: RecordObservation<'_>| {
        events
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(event.to_owned());
    };
    let options = options.observe(&observer);
    let mut storage = Storage::default();
    let originals = Mutex::new(Vec::new());
    let records = source.read()?.map(|record| {
        let record = inputs::compose(record?, kind, Some(reading))?;
        originals
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(record.original.clone());
        Ok(record.map_original(|o| o.native))
    });

    let feed = thinkthen::RequestFeed::from_records("c-complete", records);
    // Whole-set functions own count admission and must stop before an unread tail.
    let feed = if matches!(
        kind,
        abi::THINKTHEN_FUNCTION_FIND_V1 | abi::THINKTHEN_FUNCTION_RELATE_V1
    ) {
        feed
    } else {
        feed.eager()
    };
    let feed = if kind == abi::THINKTHEN_FUNCTION_FILTER_V1 {
        feed.with_all_filter_results()
    } else {
        feed
    };
    let call = match engine.execute_request(
        &request,
        thinkthen::RequestEnvironment {
            controls: options,
            feed: Some(feed),
        },
    )? {
        thinkthen::RequestOutcome::Complete(call) => call,
        thinkthen::RequestOutcome::Failed { error, .. } => return Err(error.into()),
    };
    *completed = Some(call.facts().clone());
    let originals = originals
        .into_inner()
        .unwrap_or_else(PoisonError::into_inner);
    let rows = project(&mut storage, call.value(), &originals, &events)?;
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
pub(super) fn project(
    s: &mut Storage,
    value: &thinkthen::RequestValue,
    originals: &[inputs::Original],
    events: &Mutex<Vec<OwnedRecordObservation>>,
) -> Result<Vec<RowObservationV1>, Failure> {
    fn original(
        originals: &[inputs::Original],
        ordinal: usize,
    ) -> Result<inputs::Original, Failure> {
        originals
            .get(ordinal)
            .cloned()
            .ok_or_else(|| Failure::defect("a completed occurrence lost its original"))
    }
    macro_rules! collect {
        ($rows:expr, $convert:expr) => {
            $rows
                .iter()
                .map(|r| {
                    let original = original(originals, r.ordinal())?;
                    $convert(&r.clone().map_original(|_| original))
                })
                .collect()
        };
    }
    use thinkthen::RequestValue;
    match value {
        RequestValue::Decisions(v) => collect!(v, |r| rows::decide(s, r)),
        RequestValue::Choices(v) => collect!(v, |r| rows::choose(s, r)),
        RequestValue::Tags(v) => collect!(v, |r| rows::tag(s, r)),
        RequestValue::Scores(v) => collect!(v, |r| rows::score(s, r)),
        RequestValue::Filtered(v) => collect!(v, |r| rows::filter(s, r)),
        RequestValue::Ranked(v) => collect!(v, |r| rank_row(s, r, events)),
        RequestValue::SetRanked(v) => collect!(v, |r| super::rank_set::row(
            s,
            r,
            &events.lock().unwrap_or_else(PoisonError::into_inner)
        )),
        RequestValue::Annotations(v) => collect!(v, |r| structured::annotate(
            s,
            r,
            &events.lock().unwrap_or_else(PoisonError::into_inner)
        )),
        RequestValue::Recognized(v) => collect!(v, |r| structured::recognize(s, r)),
        RequestValue::Found(v) => {
            if v.candidates()
                .iter()
                .filter(|c| c.input().is_some())
                .count()
                != originals.len()
            {
                return Err(Failure::defect(
                    "a complete find lost its original candidates",
                ));
            }
            let mut original_iter = originals.iter();
            let mapped = v.clone().map_original(|native| {
                original_iter
                    .next()
                    .cloned()
                    .unwrap_or(inputs::Original::fallback(native))
            });
            Ok(vec![rows::find(
                s,
                &mapped,
                &events.lock().unwrap_or_else(PoisonError::into_inner),
            )?])
        }
        RequestValue::Related(v) => {
            let mapped = v.clone().map_original(|_| originals.to_vec());
            Ok(vec![structured::relate(s, &mapped)?])
        }
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
