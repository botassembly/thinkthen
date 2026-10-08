use super::*;
use std::sync::Mutex;
use thinkthen::{
    RawRecord, Recognize, RecordInput, RecordObservation, RecordReading, SourceLocation,
};

#[test]
fn an_external_cache_change_between_records_refuses_before_the_next_send() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let folder = folder();
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
    let ask = Recognize::builder().build().unwrap();
    let changed = std::sync::atomic::AtomicBool::new(false);
    let observer = |event: RecordObservation<'_>| {
        if let RecordObservation::Row { index: 0, .. } = event {
            let outside = rusqlite::Connection::open(folder.join("thinkthen.sqlite")).unwrap();
            assert!(
                outside
                    .execute("UPDATE answers SET answer='broken'", [])
                    .unwrap()
                    > 0
            );
            changed.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    };
    let rows = ["Ada met Acme.", "Bob met Corp."].map(|original| RecordInput {
        original,
        context: None,
        options: None,
    });
    let failure = engine
        .recognize_records_complete_with(&ask, rows, CallOptions::new().observe(&observer))
        .unwrap_err();
    assert!(changed.load(std::sync::atomic::Ordering::SeqCst));
    assert_eq!(failure.kind(), ErrorKind::Local);
    assert_eq!(failure.facts().unwrap().records(), 1);
    assert_eq!(listener.count(), 2);
}

#[cfg(test)]
fn record(_id: usize, line: usize) -> RecordInput<thinkthen::RecordEvidence> {
    let raw = RawRecord::text("Ada met Acme.").unwrap();
    let mut record = RecordReading::new(&[], None, None)
        .unwrap()
        .compose(raw)
        .unwrap();
    record.original = record.original.with_location(
        SourceLocation::new("original.jsonl".into(), Some(line), Some(line)).unwrap(),
    );
    record
}

#[test]
fn located_recognition_retains_duplicate_occurrences_and_one_call_with_row_attempts() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let folder = folder();
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
    let ask = Recognize::builder().build().unwrap();
    let held = Mutex::new(Vec::new());
    let observer = |event: RecordObservation<'_>| held.lock().unwrap().push(event.to_owned());
    let call = engine
        .recognize_records_complete_with(
            &ask,
            [record(1, 4), record(2, 5)],
            CallOptions::new()
                .context("Separate.")
                .attempts(true)
                .observe(&observer),
        )
        .unwrap();
    assert_eq!(call.facts().records(), 2);
    assert_eq!(call.facts().requests_sent(), 2);
    assert_eq!(listener.count(), 2);
    let rows = call.value();
    assert_ne!(rows[0].result().answer_id(), rows[1].result().answer_id());
    assert_eq!(
        rows[0].result().identity().observations(),
        rows[1].result().identity().observations()
    );
    assert_eq!(rows[0].result().meta().attempts().unwrap().len(), 2);
    assert_eq!(rows[1].result().meta().attempts(), Some(&[][..]));
    assert_eq!(
        rows[0].result().reported_usage().unwrap().input_tokens(),
        Some(1774)
    );
    assert_eq!(
        rows[0].result().reported_usage().unwrap().output_tokens(),
        None
    );
    let document = serde_json::to_value(&rows[1]).unwrap();
    assert_eq!(document["schema"], "thinkthen.result/2");
    assert_eq!(document["input"], json!("Ada met Acme."));
    assert_eq!(document["value"]["entities"][0]["start"], 0);
    assert_eq!(document["value"]["entities"][1]["end"], 12);
    let requests = listener.requests();
    assert_eq!(
        requests[0].header("x-thinkthen-call-id"),
        call.facts().call_id().map(|id| id.as_str())
    );
    drop(call);
    drop(engine);
    assert_owned_sources(&held.lock().unwrap());
    std::fs::remove_dir_all(folder).unwrap();
}

#[cfg(test)]
fn assert_owned_sources(details: &[thinkthen::OwnedRecordObservation]) {
    let mut observed = 0;
    for event in details.iter() {
        if let thinkthen::OwnedRecordObservation::Question {
            index,
            detail,
            stage,
            ..
        } = event
        {
            assert!(matches!(*stage, Some("boundary" | "edge")));
            let detail = detail.detail();
            assert!(detail.answer_id().is_some());
            assert_eq!(
                match detail.input().unwrap() {
                    thinkthen::QuestionInput::Record(record) => record,
                    _ => panic!("expected original record"),
                }
                .location()
                .unwrap()
                .first_line(),
                Some(4 + index)
            );
            observed += 1;
        }
    }
    assert_eq!(observed, 10);
}

#[test]
fn recognition_admits_all_records_before_sends_and_empty_records_have_no_observations() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let engine = engine(&listener);
    let ask = Recognize::builder().build().unwrap();
    let failed = engine
        .try_recognize_records_complete_with(
            &ask,
            [
                Ok(record(1, 1)),
                Err(thinkthen::Error::new(ErrorKind::Local, "reader stopped")),
            ],
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(failed.kind(), ErrorKind::Local);
    assert_eq!(failed.stopped().at(), Some(2));
    assert!(failed.facts().is_none());
    let mut unsupported = record(2, 2);
    unsupported.options = Some(
        thinkthen::RecordOptions::project(r#"{"options":["one","two"]}"#, "/options").unwrap(),
    );
    assert_eq!(
        engine
            .recognize_records_complete_with(&ask, [record(1, 1), unsupported], CallOptions::new())
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    let empty = RecordInput {
        original: "",
        context: None,
        options: None,
    };
    let result = engine
        .recognize_records_complete_with(&ask, [empty], CallOptions::new().attempts(true))
        .unwrap();
    assert_eq!(result.facts().records(), 1);
    assert_eq!(result.value()[0].result().identity().origin(), None);
    assert_eq!(result.value()[0].result().meta().attempts(), Some(&[][..]));
    assert_eq!(listener.count(), 0);
}

#[test]
fn started_recognition_failure_retains_completed_prefix_and_actual_failure_location() {
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).unwrap();
        if request["state"].as_str() == Some("Broken.") {
            assert_eq!(request["questions"].as_object().unwrap().len(), 2);
            Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"wrong":1.0}},"q2":{"type":"choice","probabilities":{"BEGIN":0.0,"INSIDE":0.0,"END":0.0,"OUT":1.0,"SINGLE":0.0}}}}"#)
        } else {
            super::aggregates::recognized_response(body)
        }
    })
    .unwrap();
    let engine = engine(&listener);
    let ask = Recognize::builder().build().unwrap();
    let held = Mutex::new(Vec::new());
    let observer = |event: RecordObservation<'_>| held.lock().unwrap().push(event.to_owned());
    let reading = RecordReading::new(&[], None, None).unwrap();
    let mut broken = reading
        .compose(RawRecord::text("Broken.").unwrap())
        .unwrap();
    broken.original = broken
        .original
        .with_location(SourceLocation::new("broken.txt".into(), Some(9), Some(9)).unwrap());
    let failure = engine
        .recognize_records_complete_with(
            &ask,
            [record(1, 4), broken],
            CallOptions::new().attempts(true).observe(&observer),
        )
        .unwrap_err();
    assert_eq!(failure.kind(), ErrorKind::Backend);
    assert_eq!(failure.stopped().at(), Some(2));
    assert_eq!(failure.facts().unwrap().records(), 1);
    assert_eq!(failure.facts().unwrap().requests_sent(), 3);
    assert_eq!(failure.facts().unwrap().attempts().unwrap().len(), 3);
    let events = held.lock().unwrap();
    let failed = events
        .iter()
        .find_map(|event| match event {
            thinkthen::OwnedRecordObservation::Question {
                index: 1, detail, ..
            } => Some(detail.detail()),
            _ => None,
        })
        .unwrap();
    assert!(failed.failure_id().is_some());
    assert!(failed.answer_id().is_none());
    let thinkthen::QuestionInput::Record(record) = failed.input().unwrap() else {
        panic!("located failure")
    };
    assert_eq!(record.location().unwrap().first_line(), Some(9));
    assert_eq!(listener.count(), 3);
}

#[test]
fn saved_recognition_selection_uses_native_reading_and_keeps_profile_warning_and_original() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("engine-model")
        .unwrap()
        .api_key("complete-private")
        .unwrap()
        .no_cache()
        .max_retries(0)
        .profile_json(
            r#"{"schema":"thinkthen.backend-profile/1","name":"running","max_questions":400}"#,
        )
        .unwrap()
        .build()
        .unwrap();
    let folder = folder();
    let path = folder.join("recognize.json");
    std::fs::write(
        &path,
        r#"{"version":1,"recognize":{},"on":"/body","model":"saved-model","profile":"calibrated"}"#,
    )
    .unwrap();
    let saved = thinkthen::RecognizeQuestionFile::load(&path).unwrap();
    assert_eq!(Recognize::load(&path).unwrap_err().kind(), ErrorKind::Local);
    let original = RawRecord::json(r#"{"body":"Ada met Acme.","private":false}"#).unwrap();
    let call = engine
        .recognize_records_complete_with(
            saved.question(),
            [saved.reading().compose(original).unwrap()],
            CallOptions::new(),
        )
        .unwrap();
    assert_eq!(
        call.value()[0].result().question().profile(),
        Some("calibrated")
    );
    let meta = call.value()[0].result().meta();
    let warning = meta.profile_warning().unwrap();
    assert_eq!(warning.tuned_for(), "calibrated");
    assert_eq!(warning.running(), "running");
    let document = serde_json::to_value(&call.value()[0]).unwrap();
    assert_eq!(
        document["input"],
        json!({"body":"Ada met Acme.","private":false})
    );
    assert_eq!(
        document["meta"]["profile_warning"],
        json!({"tuned_for":"calibrated","running":"running"})
    );
    for request in listener.requests() {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(body["model"], "saved-model");
        assert_eq!(body["state"], "Ada met Acme.");
    }
    assert_eq!(listener.count(), 2);
    assert_eq!(listener.questions(), 5);
    std::fs::remove_dir_all(folder).unwrap();
}
