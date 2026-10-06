use super::*;
use std::sync::Mutex;
use thinkthen::{
    OwnedObservedRow, OwnedRecordObservation, QuestionInput, RawRecord, RecordReading,
    ResolvedThreshold, SourceLocation,
};

#[test]
fn owned_question_events_survive_engine_destruction_with_originals_locations_and_stable_cached_observations()
 {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":887}}"#)).unwrap();
    let folder = folder();
    let snapshots = Mutex::new(Vec::new());
    let capture =
        |event: thinkthen::RecordObservation<'_>| snapshots.lock().unwrap().push(event.to_owned());
    let expected;
    {
        let engine = Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .api_key("complete-private")
            .unwrap()
            .cache_at(&folder)
            .unwrap()
            .max_retries(0)
            .build()
            .unwrap();
        let question = Question::decide("Refund?").unwrap().cut_at(0.8).unwrap();
        let input = || {
            let mut record = RecordReading::new(&["/body"], None, None)
                .unwrap()
                .compose(
                    RawRecord::json(r#"{"private":false,"body":"Refund please.","z":null}"#)
                        .unwrap(),
                )
                .unwrap();
            record.original = record.original.with_location(
                SourceLocation::new("private-file.jsonl".into(), Some(4), Some(4)).unwrap(),
            );
            record
        };
        let first = engine
            .decide_records_complete_with(
                &question,
                [input()],
                CallOptions::new().observe(&capture),
            )
            .unwrap();
        expected = first.value()[0].result().answer_id().clone();
        let held = engine
            .decide_records_complete_with(
                &question,
                [input()],
                CallOptions::new().observe(&capture),
            )
            .unwrap();
        assert_eq!(held.value()[0].result().answer_id(), &expected);
        assert_eq!(held.facts().requests_sent(), 0);
    }
    let events = snapshots.into_inner().unwrap();
    assert_eq!(events.len(), 4);
    for (at, event) in events.iter().enumerate().step_by(2) {
        let OwnedRecordObservation::Question {
            index,
            member,
            stage,
            position,
            detail,
        } = event
        else {
            panic!("question first")
        };
        let view = detail.detail();
        assert_eq!(
            (*index, member.as_deref(), *stage, *position),
            (0, None, None, 0)
        );
        assert_eq!(view.answer_id(), Some(&expected));
        assert!(view.failure_id().is_none());
        assert_eq!(view.question().text().text(), Some("Refund?"));
        assert_eq!(view.threshold(), Some(ResolvedThreshold::Cut(0.8)));
        assert_eq!(view.reported_usage().unwrap().input_tokens(), Some(887));
        assert_eq!(view.reported_usage().unwrap().output_tokens(), None);
        assert_eq!(
            view.question_sources()[0].origin(),
            if at == 0 { Origin::Live } else { Origin::Cache }
        );
        assert_eq!(view.question_sources()[0].batch_size(), Some(1));
        assert_owned_input(event, view);
        assert!(!format!("{event:?}").contains("Refund please"));
    }
    assert_eq!(
        question_detail(&events[0]).observations(),
        question_detail(&events[2]).observations()
    );
    assert_eq!(listener.count(), 1);
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn owned_failed_annotation_events_keep_admitted_rules_actual_failure_ids_and_partial_usage() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"complete-private","answers":{"q1":{"type":"noul","noul":0.5},"q2":{"type":"choice","probabilities":{"wrong":1}}},"usage":{"input_tokens":887}}"#)).unwrap();
    let snapshots = Mutex::new(Vec::new());
    let capture =
        |event: thinkthen::RecordObservation<'_>| snapshots.lock().unwrap().push(event.to_owned());
    let set = thinkthen::QuestionSet::from_json(r#"{"version":1,"questions":{"uncertain":{"decide":"Sure?","threshold":"0.2:0.8"},"failed":{"decide":"Failed?","threshold":0.9}}}"#).unwrap();
    let engine = engine(&listener);
    let call = engine
        .annotate_complete_with(&set, ["Original."], CallOptions::new().observe(&capture))
        .unwrap();
    let members = call.value()[0].result().members().collect::<Vec<_>>();
    let events = snapshots.into_inner().unwrap();
    assert_eq!(events.len(), 3);
    for (at, event) in events[..2].iter().enumerate() {
        let OwnedRecordObservation::Question {
            index,
            member,
            position,
            detail,
            ..
        } = event
        else {
            panic!("member")
        };
        let view = detail.detail();
        assert_eq!(*index, 0);
        assert_eq!(member.as_deref(), Some(members[at].name()));
        assert_eq!(*position, at);
        assert_eq!(view.answer_id(), members[at].answer_id());
        assert_eq!(view.failure_id(), members[at].failure_id());
        assert_eq!(view.observations(), members[at].observations());
        assert_eq!(view.threshold(), members[at].threshold());
        assert_eq!(
            view.reported_usage().unwrap().input_tokens(),
            Some(if at == 0 { 444 } else { 443 })
        );
        assert_eq!(view.reported_usage().unwrap().output_tokens(), None);
        assert_eq!(view.question_sources()[0].answered_by(), "complete-private");
        assert_eq!(view.question_sources()[0].batch_size(), Some(2));
        assert!(!format!("{event:?}").contains("complete-private"));
    }
    assert_owned_annotation_row(&events[2]);
    assert_eq!(listener.count(), 1);
    assert_eq!(listener.questions(), 2);
}

#[cfg(test)]
fn assert_owned_annotation_row(event: &OwnedRecordObservation) {
    let OwnedRecordObservation::Row {
        value: OwnedObservedRow::Annotated(values),
        ..
    } = event
    else {
        panic!("owned finished row")
    };
    assert_eq!(
        values[0].value(),
        &thinkthen::Annotated::Decision(Answer::Unsure)
    );
    assert!(matches!(values[1].value(), thinkthen::Annotated::Failed(_)));
}

#[cfg(test)]
fn question_detail(event: &OwnedRecordObservation) -> thinkthen::QuestionDetail<'_> {
    let OwnedRecordObservation::Question { detail, .. } = event else {
        panic!("question event")
    };
    detail.detail()
}
#[cfg(test)]
fn assert_owned_input(event: &OwnedRecordObservation, view: thinkthen::QuestionDetail<'_>) {
    let Some(QuestionInput::Record(input)) = view.input() else {
        panic!("native record")
    };
    assert_eq!(input.location().unwrap().file(), "private-file.jsonl");
    assert_eq!(input.location().unwrap().first_line(), Some(4));
    assert_eq!(
        input.original().content().unwrap().to_json().unwrap(),
        r#"{"private":false,"body":"Refund please.","z":null}"#
    );
    assert!(!format!("{event:?} {input:?}").contains("private-file"));
}
