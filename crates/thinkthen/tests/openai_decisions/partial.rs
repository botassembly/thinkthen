use super::{builder, folder, response};
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use thinkthen::{CallOptions, ErrorKind, Question, QuestionSet};

#[test]
fn partial_refusal_keeps_valid_observations_and_only_missing_piece_is_sent_again() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let seen = attempts.clone();
    let listener = Listener::answering(move |body| {
        if seen.fetch_add(1,Ordering::SeqCst)==0 {
            Canned::ok(r#"{"model":"fixed","answers":[{"type":"predicate","name":"q1","probability":0.9},{"type":"refusal","name":"q2","reason":"private"}],"usage":{"input_tokens":17,"output_tokens":0}}"#)
        } else { response(body) }
    }).unwrap();
    let place = folder();
    let engine = builder(&listener)
        .cache_at(&place)
        .unwrap()
        .build()
        .unwrap();
    let set = QuestionSet::from_json(
        r#"{"version":1,"questions":{"good":{"decide":"Good?"},"failed":{"decide":"Retry?"}}}"#,
    )
    .unwrap();
    let first = engine
        .annotate_complete_with(&set, ["Input."], CallOptions::new())
        .unwrap();
    let document: Value =
        serde_json::from_str(&first.value()[0].result().to_json().unwrap()).unwrap();
    assert_eq!(document["meta"]["failed_questions"], 1);
    let mut members = first.value()[0].result().members();
    let good = members.next().unwrap();
    let failed = members.next().unwrap();
    assert!(good.answer_id().is_some());
    assert!(failed.answer_id().is_none());
    assert!(failed.failure_id().is_some());
    assert!(!document.to_string().contains("private"));
    let second = engine
        .annotate_complete_with(&set, ["Input."], CallOptions::new())
        .unwrap();
    assert_eq!(
        second.value()[0]
            .result()
            .members()
            .next()
            .unwrap()
            .observations(),
        good.observations()
    );
    assert_eq!(listener.count(), 2);
    let sent = listener.requests();
    let remainder: Value = serde_json::from_slice(&sent[1].body).unwrap();
    assert_eq!(remainder["questions"].as_array().unwrap().len(), 1);
    assert_eq!(remainder["questions"][0]["name"], "q1");
    assert!(
        remainder["questions"][0]["instructions"]
            .as_str()
            .unwrap()
            .contains("Retry?")
    );
    let db = rusqlite::Connection::open(place.join("thinkthen.sqlite")).unwrap();
    db.execute("UPDATE answers SET answer='{broken'", [])
        .unwrap();
    drop(db);
    let error = engine
        .annotate_complete_with(&set, ["Input."], CallOptions::new())
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert_eq!(listener.count(), 2);
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}
#[test]
fn failed_expanded_tag_member_is_not_false_and_its_valid_siblings_are_reused() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let seen = attempts.clone();
    let listener = Listener::answering(move |body| {
        if seen.fetch_add(1,Ordering::SeqCst)==0 { Canned::ok(&json!({"model":"fixed","answers":[{"type":"predicate","name":"q1","probability":0.9},{"type":"refusal","name":"q2"}]}).to_string()) }
        else { response(body) }
    }).unwrap();
    let place = folder();
    let engine = builder(&listener)
        .cache_at(&place)
        .unwrap()
        .build()
        .unwrap();
    let question = Question::tag_labels("Applies?")
        .unwrap()
        .label("first", None)
        .unwrap()
        .label("second", None)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(
        engine
            .tag_complete_with(&question, "Input.", CallOptions::new())
            .unwrap_err()
            .kind(),
        ErrorKind::Backend
    );
    engine
        .tag_complete_with(&question, "Input.", CallOptions::new())
        .unwrap();
    assert_eq!(listener.count(), 2);
    let sent = listener.requests();
    let remainder: Value = serde_json::from_slice(&sent[1].body).unwrap();
    assert_eq!(remainder["questions"].as_array().unwrap().len(), 1);
    assert!(
        remainder["questions"][0]["instructions"]
            .as_str()
            .unwrap()
            .contains("second")
    );
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}
