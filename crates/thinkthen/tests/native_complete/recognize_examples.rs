//! Native example admission, complete identities and failure facts through typed calls.
use super::*;
use thinkthen::{Kind, RawRecord, RecognitionExample, Recognize, RecordInput, RecordReading};

fn example(text: &str) -> RecognitionExample {
    RecognitionExample::Brackets(format!("[[ENTITY|{text}]]"))
}

#[test]
fn shared_example_forms_produce_identical_native_requests_and_current_kind_answers() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../../conformance/recognition-examples.json"
    ))
    .unwrap();
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let engine = engine(&listener);
    let ask = Recognize::builder()
        .kind(Kind::new("person", None).unwrap())
        .unwrap()
        .build()
        .unwrap();
    let forms = [
        RecognitionExample::Brackets(fixture["brackets"].as_str().unwrap().into()),
        serde_json::from_value(fixture["spans"].clone()).unwrap(),
    ];
    for form in forms {
        let ask = ask.clone().with_examples(vec![form]).unwrap();
        let result = engine
            .recognize_complete_with(&ask, "Ada", CallOptions::new().context("unchanged"))
            .unwrap();
        assert_eq!(result.facts().requests_sent(), 2);
        let output: Value = serde_json::from_str(&result.value().to_json().unwrap()).unwrap();
        assert!(output["question"].get("examples").is_none());
    }
    let requests = listener.requests();
    assert_eq!(requests[0].body, requests[2].body);
    assert_eq!(requests[1].body, requests[3].body);
    let first: Value = serde_json::from_slice(&requests[0].body).unwrap();
    let rendered = first["state"]["evidence"]["examples"][0].as_str().unwrap();
    let answers: Vec<_> = rendered
        .lines()
        .filter_map(|line| line.strip_prefix("Answer: "))
        .map(|line| serde_json::from_str::<String>(line.split(';').next().unwrap()).unwrap())
        .collect();
    assert_eq!(serde_json::to_value(answers).unwrap(), fixture["answers"]);
    assert!(!rendered.contains("kind: \"organization\""));
    assert_eq!(first["state"]["context"], "unchanged");
}

#[test]
fn native_projection_and_explicit_empty_examples_keep_independent_record_identities() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let engine = engine(&listener);
    let ask = Recognize::builder()
        .build()
        .unwrap()
        .with_examples(vec![example("shared")])
        .unwrap();
    let reading = RecordReading::new(&["/body"], Some("/context"), None)
        .unwrap()
        .with_examples_field("/examples")
        .unwrap();
    let make = |examples: Option<Value>| {
        let mut json = json!({"body":"Ada met Acme.","context":""});
        if let Some(examples) = examples {
            json["examples"] = examples;
        }
        reading
            .compose(RawRecord::json(&json.to_string()).unwrap())
            .unwrap()
    };
    let rows = [
        make(Some(json!(["[[ENTITY|first]]"]))),
        make(Some(json!([]))),
        make(None),
    ];
    let result = engine
        .recognize_records_complete_with(&ask, rows, CallOptions::new().context("shared context"))
        .unwrap();
    assert_eq!(result.facts().requests_sent(), 6);
    let requests = listener.requests();
    let state =
        |at: usize| serde_json::from_slice::<Value>(&requests[at].body).unwrap()["state"].clone();
    assert!(
        state(0)["examples"][0]
            .as_str()
            .unwrap()
            .contains("[[first]]")
    );
    assert_eq!(state(2), "Ada met Acme.");
    assert!(
        state(4)["examples"][0]
            .as_str()
            .unwrap()
            .contains("[[shared]]")
    );
    for at in [1, 3, 5] {
        assert_eq!(state(at), "Ada met Acme.");
    }
    assert_ne!(
        result.value()[0].result().identity(),
        result.value()[1].result().identity()
    );
    let before = listener.count();
    for invalid in [
        r#"{"body":"Ada","context":"","examples":null}"#,
        r#"{"body":"Ada","context":"","examples":{"bad":1}}"#,
    ] {
        assert!(reading.compose(RawRecord::json(invalid).unwrap()).is_err());
    }
    let nested = RecordReading::new(&["/body"], None, None)
        .unwrap()
        .with_examples_field("/examples/list")
        .unwrap();
    assert!(
        nested
            .compose(RawRecord::json(r#"{"body":"Ada","examples":4}"#).unwrap())
            .is_err()
    );
    assert_eq!(listener.count(), before);
}

#[test]
fn native_later_invalid_examples_and_nonrecognition_controls_send_nothing() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let engine = engine(&listener);
    let ask = Recognize::builder().build().unwrap();
    let row = |examples| RecordInput {
        original: "Ada met Acme.",
        context: None,
        options: None,
        examples,
    };
    let failure = engine
        .recognize_records_complete_with(
            &ask,
            [
                row(Some(vec![example("Ada")])),
                row(Some(vec![RecognitionExample::Brackets(
                    "[[unknown|secret]]".into(),
                )])),
            ],
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(failure.kind(), ErrorKind::Usage);
    assert_eq!(failure.stopped().at(), Some(2));
    assert_eq!(listener.count(), 0);
    assert!(!failure.detail().message().contains("secret"));
    let question = Question::decide("Yes?").unwrap().cut();
    let failure = engine
        .decide_records_complete_with(
            &question,
            [row(Some(Vec::new())).map_original(|_| "changed original")],
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(failure.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 0);
    assert!(!format!("{:?}", example("secret")).contains("secret"));
}

#[test]
fn empty_examples_preserve_defaults_and_a_later_stage_failure_retains_actual_facts() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let engine = engine(&listener);
    let ask = Recognize::builder().build().unwrap();
    let first = engine
        .recognize_complete_with(&ask, "Ada met Acme.", CallOptions::new())
        .unwrap();
    let second = engine
        .recognize_complete_with(
            &ask.with_examples(Vec::new()).unwrap(),
            "Ada met Acme.",
            CallOptions::new(),
        )
        .unwrap();
    let requests = listener.requests();
    assert_eq!(requests[0].body, requests[2].body);
    assert_eq!(requests[1].body, requests[3].body);
    let first: Value = serde_json::from_str(&first.value().to_json().unwrap()).unwrap();
    let second: Value = serde_json::from_str(&second.value().to_json().unwrap()).unwrap();
    assert_eq!(first["question"], second["question"]);
    let failing = Listener::answering(|body| {
        let body_value: Value = serde_json::from_slice(body).unwrap();
        if body_value["state"].get("examples").is_some() {
            super::aggregates::recognized_response(body)
        } else {
            Canned::ok(r#"{"model":"fixed","answers":{}}"#)
        }
    })
    .unwrap();
    let ask = Recognize::builder()
        .kind(Kind::new("person", None).unwrap())
        .unwrap()
        .build()
        .unwrap()
        .with_examples(vec![RecognitionExample::Brackets("[[person|Ada]]".into())])
        .unwrap();
    let failure = super::engine(&failing)
        .recognize_complete_with(&ask, "Ada met Acme.", CallOptions::new().attempts(true))
        .unwrap_err();
    assert_eq!(failing.count(), 2);
    assert_eq!(failure.facts().unwrap().requests_sent(), 2);
    assert_eq!(failure.facts().unwrap().attempts().unwrap().len(), 2);
}
