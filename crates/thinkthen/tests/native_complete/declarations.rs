use super::*;
use thinkthen::{
    InputDeclaration, InputProperty, InputPropertyType, ObjectDeclaration, RawRecord, RecordInput,
    RecordReading,
};

#[test]
fn metadata_edits_reuse_actual_observations_but_declarations_validate_before_cache_and_replay() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}},"usage":{"input_tokens":887}}"#)).unwrap();
    let folder = folder();
    let build = || {
        Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .api_key("complete-private")
            .unwrap()
            .max_retries(0)
    };
    let engine = build().cache_at(&folder).unwrap().build().unwrap();
    let first = Question::from_json(r#"{"decide":"Refund?","name":"refund","wording_version":1,"item_schema":{"type":"string"}}"#).unwrap();
    let second = Question::from_json(r#"{"decide":"Refund?","name":"other","wording_version":7,"model":"fixed","batch":2,"on":[""],"item_schema":{"type":"string"},"context_schema":{"type":"object","properties":{}}}"#).unwrap();
    let live = engine
        .decide_complete_with(&first, "Refund me.", CallOptions::new())
        .unwrap();
    let cached = engine
        .decide_complete_with(&second, "Refund me.", CallOptions::new())
        .unwrap();
    assert_eq!(live.value().answer_id(), cached.value().answer_id());
    assert_eq!(
        live.value().identity().observations(),
        cached.value().identity().observations()
    );
    assert_eq!(cached.value().question().name().unwrap().as_str(), "other");
    assert_eq!(
        cached.value().question().wording_version().unwrap().get(),
        7
    );
    schema::call(&cached, "completeDecide");
    let output: Value = serde_json::from_str(&cached.value().to_json().unwrap()).unwrap();
    assert_eq!(output["question"]["item_schema"], json!({"type":"string"}));
    assert_eq!(output["question"]["model"], "fixed");
    assert_eq!(output["question"]["batch"], 2);
    assert_eq!(output["question"]["on"], json!([""]));
    assert!(output["question"].get("profile").is_none());
    assert_eq!(
        output["question"]["context_schema"],
        json!({"type":"object","properties":{}})
    );
    let bad = Question::from_json(
        r#"{"decide":"Refund?","item_schema":{"type":"object","properties":{}}}"#,
    )
    .unwrap();
    for engine in [&engine, &build().replay(&folder).unwrap().build().unwrap()] {
        let error = engine
            .decide_complete_with(&bad, "Refund me.", CallOptions::new())
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.detail().message(),
            "the item does not match item_schema"
        );
        assert!(error.facts().is_none());
    }
    assert_eq!(listener.count(), 1);
    assert_eq!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
        json!({"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me.\". Refund?"}}})
    );
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn declarations_admit_typed_projection_false_unicode_and_extras_without_coercion_or_partial_finite_sends()
 {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let schema = InputDeclaration::Object(
        ObjectDeclaration::new(
            vec![
                InputProperty::new("body", InputPropertyType::String).unwrap(),
                InputProperty::new("ready", InputPropertyType::Boolean).unwrap(),
                InputProperty::new("金額", InputPropertyType::Number).unwrap(),
                InputProperty::new("labels", InputPropertyType::StringList).unwrap(),
            ],
            vec!["body".into()],
        )
        .unwrap(),
    );
    let question = Question::decide("Refund?")
        .unwrap()
        .cut()
        .with_item_schema(schema)
        .unwrap()
        .with_context_schema(InputDeclaration::String)
        .unwrap();
    let reading = RecordReading::new(&["/message"], Some("/context"), None).unwrap();
    let valid = r#"{"message":{"body":"Refund me.","ready":false,"金額":12,"labels":[],"extra":null},"context":"","secret":"not sent"}"#;
    let input = || reading.compose(RawRecord::json(valid).unwrap()).unwrap();
    let bad = reading
        .compose(
            RawRecord::json(r#"{"message":{"body":"Refund me.","ready":null},"context":""}"#)
                .unwrap(),
        )
        .unwrap();
    let error = engine
        .decide_records_complete_with(&question, [input(), bad], CallOptions::new())
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(error.stopped().at(), Some(2));
    assert_eq!(listener.count(), 0);
    let call = engine
        .decide_records_complete_with(
            &question,
            [input()],
            CallOptions::new().context("Fallback."),
        )
        .unwrap();
    assert_eq!(call.value()[0].result().value(), Answer::Yes);
    let requests = listener.requests();
    let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(
        body,
        json!({"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":r#"The text is {"body":"Refund me.","ready":false,"金額":12,"labels":[],"extra":null}. Refund?"#}}})
    );
    assert!(
        !String::from_utf8(requests[0].body.clone())
            .unwrap()
            .contains("secret")
    );
}

#[test]
fn closed_question_metadata_grammar_refuses_unknown_features_and_lexically_noninteger_versions_safely()
 {
    for addition in [
        r#""name":"../private""#,
        r#""name":"Refund""#,
        r#""wording_version":0"#,
        r#""wording_version":2147483648"#,
        r#""wording_version":1.0"#,
        r#""wording_version":1e0"#,
        r#""wording_version":null"#,
        r#""item_schema":"string""#,
        r#""item_schema":{"type":"string","private":true}"#,
        r#""item_schema":{"type":"object","properties":{"secret":{"type":"object"}}}"#,
        r#""item_schema":{"type":"object","properties":{},"required":["secret"]}"#,
    ] {
        let error =
            Question::from_json(&format!(r#"{{"decide":"Refund?",{addition}}}"#)).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage, "{addition}");
        assert_eq!(
            error.detail().message(),
            "the question declaration uses an unsupported feature"
        );
        assert!(!format!("{error:?}").contains("private"));
        assert!(!format!("{error:?}").contains("secret"));
    }
    let old = Question::from_json(r#"{"decide":"Refund?"}"#).unwrap();
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}}}"#)
    })
    .unwrap();
    let call = engine(&listener)
        .decide_complete_with(&old, "Refund me.", CallOptions::new())
        .unwrap();
    assert!(call.value().question().name().is_none());
    assert!(call.value().question().wording_version().is_none());
    assert!(!call.value().to_json().unwrap().contains("wording_version"));
}

#[test]
fn annotation_declarations_check_whole_typed_documents_and_every_member_before_dispatch() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let objects = thinkthen::QuestionSet::from_json(r#"{"version":1,"questions":{"refund":{"decide":"Refund?","name":"authored-name","wording_version":4,"item_schema":{"type":"object","properties":{"ready":{"type":"boolean"}},"required":["ready"]}}}}"#).unwrap();
    let text = thinkthen::QuestionSet::from_json(r#"{"version":1,"questions":{"refund":{"decide":"Refund?","item_schema":{"type":"string"}}}}"#).unwrap();
    let document = r#"{"ready":false,"body":"Refund me."}"#;
    let error = engine
        .annotate_complete_with(&text, [document], CallOptions::new())
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.detail().message(),
        "the item does not match item_schema"
    );
    assert_eq!(listener.count(), 0);
    let call = engine
        .annotate_complete_with(&objects, [document], CallOptions::new())
        .unwrap();
    let row: Value = serde_json::from_str(&call.value()[0].result().to_json().unwrap()).unwrap();
    assert_eq!(
        row["answers"]["refund"]["question"]["name"],
        "authored-name"
    );
    assert_eq!(row["answers"]["refund"]["question"]["wording_version"], 4);
    assert_eq!(row["input"], json!({"ready":false,"body":"Refund me."}));
    let literal = RecordReading::new(&[], None, None)
        .unwrap()
        .compose(RawRecord::text(document).unwrap())
        .unwrap();
    engine
        .annotate_records_complete_with(&text, [literal], CallOptions::new())
        .unwrap();
    let mixed = thinkthen::QuestionSet::from_json(r#"{"version":1,"questions":{"valid":{"decide":"Refund?","item_schema":{"type":"object","properties":{}}},"invalid":{"decide":"Ready?","item_schema":{"type":"string"}}}}"#).unwrap();
    assert_eq!(
        engine
            .annotate_complete_with(&mixed, [document], CallOptions::new())
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    assert_eq!(listener.count(), 2);
}

#[test]
fn declarations_refuse_coercion_and_distinguish_absent_context_from_json_looking_text() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let scalar = Question::decide("Refund?")
        .unwrap()
        .cut()
        .with_item_schema(InputDeclaration::String)
        .unwrap();
    let number = RecordReading::new(&["/amount"], None, None)
        .unwrap()
        .compose(RawRecord::json(r#"{"amount":12}"#).unwrap())
        .unwrap();
    assert_eq!(
        engine
            .decide_records_complete_with(&scalar, [number], CallOptions::new())
            .unwrap_err()
            .detail()
            .message(),
        "the item does not match item_schema"
    );
    let object_context = Question::decide("Refund?")
        .unwrap()
        .cut()
        .with_context_schema(InputDeclaration::Object(
            ObjectDeclaration::new(vec![], vec![]).unwrap(),
        ))
        .unwrap();
    let absent = RecordInput {
        examples: None,
        seed_spans: None,
        original: "Refund me.",
        context: None,
        options: None,
    };
    engine
        .decide_records_complete_with(
            &object_context,
            [absent],
            CallOptions::new().context("Shared text remains text."),
        )
        .unwrap();
    let explicit = RecordInput {
        examples: None,
        seed_spans: None,
        original: "Refund me.",
        context: Some("{}".into()),
        options: None,
    };
    assert_eq!(
        engine
            .decide_records_complete_with(&object_context, [explicit], CallOptions::new())
            .unwrap_err()
            .detail()
            .message(),
        "the per-item context does not match context_schema"
    );
    assert_eq!(listener.count(), 1);
}
