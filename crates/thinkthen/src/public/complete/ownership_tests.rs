//! Legacy document projection retains the caller original inside native execution.
//! Stays: Private complete-call primitives retain borrowed originals and snapshot evidence once; these ownership and lifetime cases cover the internal boundary directly.
use crate::ownership_tests::engine;
use crate::public::engine::Sealed as _;
use crate::{Answer, CallOptions, ErrorKind, Question};
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};

#[test]
fn loaded_atomic_and_rank_preparation_selects_original_fields_and_retains_authored_controls() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.8}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let authored = r#"{"name":"selected","wording_version":2,"decide":"Refund?","model":"fixed","batch":1,"on":["/payload"],"item_schema":{"type":"string"}}"#;
    let question = Question::from_json(authored).unwrap();
    let restored = Question::from_json(&question.to_json().unwrap()).unwrap();
    assert_eq!(restored, question);
    for text in [
        r#"{"choose":"Route?","options":{"first":{"z":null,"a":"first description"},"second":null},"threshold":0.8}"#,
        r#"{"score":"Grade?","levels":{"low":null,"high":"Excellent."}}"#,
    ] {
        let loaded = Question::from_json(text).unwrap();
        assert_eq!(
            Question::from_json(&loaded.to_json().unwrap()).unwrap(),
            loaded
        );
    }

    let location = thinkthen::SourceLocation::new("notes.jsonl".into(), Some(7), Some(7)).unwrap();
    let input = thinkthen::QuestionInput::annotation_text(
        r#"{"payload":"Yes.","private":"Never send."}"#,
        location,
    )
    .unwrap();
    let records = || [crate::api::record(input.clone())];
    let call = crate::public::request::atomic(
        &engine,
        crate::InputFunction::Decide,
        restored.question(),
        records(),
        CallOptions::new(),
    )
    .and_then(|call| call.try_map(|rows| super::records::mapped(rows, super::decision)))
    .unwrap();
    let row = &call.value()[0];
    assert_eq!(row.result().value(), Answer::Yes);
    assert_eq!(row.result().question().model(), Some("fixed"));
    assert_eq!(
        row.result().question().batch(),
        Some(thinkthen::BatchSetting::Records(
            std::num::NonZeroUsize::MIN
        ))
    );
    assert_eq!(
        row.result().question().on().collect::<Vec<_>>(),
        ["/payload"]
    );
    assert_eq!(row.result().question().name().unwrap().as_str(), "selected");
    assert_eq!(row.original(), &input);
    let body = &listener.requests()[0].body;
    assert_eq!(
        serde_json::from_slice::<Value>(body).unwrap(),
        json!({"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Yes.\". Refund?"}}})
    );
    let rank = Question::rank_from_json(authored).unwrap();
    let ranked = engine
        .try_rank_records_complete_with(&rank, records().into_iter().map(Ok), CallOptions::new())
        .unwrap();
    assert_eq!(ranked.value()[0].ordinal(), 0);
    assert_eq!(
        ranked.value()[0]
            .result()
            .question()
            .on()
            .collect::<Vec<_>>(),
        ["/payload"]
    );
    assert_eq!(listener.count(), 2);
    let missing = thinkthen::QuestionInput::annotation_text(
        r#"{"other":"No."}"#,
        thinkthen::SourceLocation::new("bad.jsonl".into(), Some(1), Some(1)).unwrap(),
    )
    .unwrap();
    let error = crate::public::request::atomic(
        &engine,
        crate::InputFunction::Decide,
        question.question(),
        [thinkthen::RecordInput {
            original: missing,
            ..records()[0].clone()
        }],
        CallOptions::new(),
    )
    .and_then(|call| call.try_map(|rows| super::records::mapped(rows, super::decision)))
    .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(error.facts().is_none());
    assert_eq!(listener.count(), 2);
}
