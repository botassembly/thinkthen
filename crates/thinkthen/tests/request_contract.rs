//! Typed and canonical requests share native answers, identities and wire bodies.
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};
use thinkthen::*;
#[path = "request_contract/native_feed.rs"]
mod native_feed;
#[path = "request_contract/preview.rs"]
mod preview;
#[path = "request_contract/projections.rs"]
mod projections;

#[allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::panic,
    reason = "a malformed isolated loopback fixture stops the proof"
)]
fn engine(listener: &Listener) -> Engine {
    Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("request-fixture")
        .unwrap()
        .no_cache()
        .max_retries(0)
        .build()
        .unwrap()
}
#[allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::panic,
    reason = "a malformed isolated loopback fixture stops the proof"
)]
fn response(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap();
    let answers = request["questions"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, q)| {
            let answer = if q["type"] == "noul" || q.get("criteria").is_none() {
                json!({"type":"noul","noul":0.9})
            } else {
                let labels = match &q["criteria"] {
                    Value::Object(fields) => fields.keys().cloned().collect::<Vec<_>>(),
                    Value::Array(values) => (0..values.len()).map(|n| n.to_string()).collect(),
                    _ => panic!("fixture criteria shape: {q}"),
                };
                let criteria = labels
                    .iter()
                    .map(|label| (label.clone(), Value::Null))
                    .collect::<serde_json::Map<_, _>>();
                let winner = if criteria.contains_key("OUT") {
                    "OUT"
                } else {
                    criteria.keys().next().unwrap()
                };
                let probabilities = criteria
                    .keys()
                    .map(|key| (key.clone(), json!(u8::from(key == winner))))
                    .collect::<serde_json::Map<_, _>>();
                json!({"type":q["type"],"probabilities":probabilities})
            };
            (name.clone(), answer)
        })
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
}
fn item(text: &str) -> RequestItem {
    RequestItem {
        original: Some(RequestOriginal::Text { text: text.into() }),
        context: None,
        options: None,
        examples: None,
        seed_spans: None,
        images: vec![],
    }
}
fn args(definition: RequestDefinition, input: RequestInput) -> RequestArguments {
    RequestArguments {
        question: RequestQuestion::Definition { value: definition },
        input,
        options: RequestOptions::default(),
    }
}
#[allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::panic,
    reason = "a malformed isolated loopback fixture stops the proof"
)]
fn direct(
    engine: &Engine,
    function: RequestFunction,
    definition: &RequestDefinition,
    rows: Vec<RecordInput<QuestionInput>>,
) -> Value {
    let controls = CallOptions::new();
    macro_rules! value {
        ($call:expr) => {
            serde_json::to_value($call.unwrap().value()).unwrap()
        };
    }
    match (function, definition) {
        (RequestFunction::Decide, RequestDefinition::Atomic(q)) => {
            value!(engine.decide_records_complete_with(q, rows, controls))
        }
        (RequestFunction::Choose, RequestDefinition::Atomic(q)) => {
            value!(engine.choose_records_complete_with(q, rows, controls))
        }
        (RequestFunction::Tag, RequestDefinition::Atomic(q)) => {
            value!(engine.tag_records_complete_with(q, rows, controls))
        }
        (RequestFunction::Score, RequestDefinition::Atomic(LoadedQuestion::Question(q))) => {
            value!(engine.score_records_complete_with(q, rows, controls))
        }
        (RequestFunction::Filter, RequestDefinition::Atomic(LoadedQuestion::Question(q))) => {
            value!(engine.filter_records_complete_with(q, rows, controls))
        }
        (RequestFunction::Rank, RequestDefinition::Rank(q)) => {
            value!(engine.rank_records_complete_with(q, rows, controls))
        }
        (RequestFunction::Find, RequestDefinition::Atomic(LoadedQuestion::Question(q))) => {
            value!(engine.find_records_complete_with(q, rows, controls))
        }
        (RequestFunction::Annotate, RequestDefinition::Annotate(q)) => {
            value!(engine.annotate_records_complete_with(q, rows, controls))
        }
        (RequestFunction::Recognize, RequestDefinition::Recognition(q)) => {
            value!(engine.recognize_records_complete_with(q, rows, controls))
        }
        (RequestFunction::Relate, RequestDefinition::Relate(q)) => {
            value!(engine.relate_records_complete_with(q, rows, controls))
        }
        _ => panic!("the declared case has the expected native definition"),
    }
}
// A fresh live observation has its own identifier even for identical wire bytes.
fn semantic(mut value: Value) -> Value {
    fn clean(value: &mut Value) {
        match value {
            Value::Object(fields) => {
                fields.remove("answer_id");
                fields.remove("observations");
                for value in fields.values_mut() {
                    clean(value);
                }
            }
            Value::Array(items) => {
                for value in items {
                    clean(value);
                }
            }
            _ => {}
        }
    }
    clean(&mut value);
    value
}
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "the ten fixtures and their common comparison stay together"
)]
fn every_function_keeps_native_answers_identities_and_outgoing_bodies() {
    let listener = Listener::answering(response).unwrap();
    let primitive = |q: Question| {
        args(
            q.into(),
            RequestInput::Records {
                items: vec![item("Alpha.")],
            },
        )
    };
    let choice = Question::choose_labels("Which?")
        .unwrap()
        .label("a", None)
        .unwrap()
        .label("b", None)
        .unwrap()
        .build()
        .unwrap();
    let tags = Question::tag_labels("Which?")
        .unwrap()
        .label("a", None)
        .unwrap()
        .build()
        .unwrap();
    let score = Question::score("Grade?")
        .unwrap()
        .level("low", None)
        .unwrap()
        .level("high", None)
        .unwrap()
        .build()
        .unwrap();
    let entity_items = [
        r#"{"name":"Ada","kind":"person"}"#,
        r#"{"name":"Acme","kind":"company"}"#,
    ]
    .map(|raw| RequestItem {
        original: Some(RequestOriginal::Json {
            value: RawRecord::json(raw).unwrap(),
        }),
        context: None,
        options: None,
        examples: None,
        seed_spans: None,
        images: vec![],
    })
    .to_vec();
    let relate = Relate::builder()
        .relation(RelationRule::one_way("works_for", "person", "company").unwrap())
        .unwrap()
        .build()
        .unwrap();
    let cases = vec![
        RequestCall::Decide(primitive(Question::decide("Fits?").unwrap().cut())),
        RequestCall::Choose(primitive(choice)),
        RequestCall::Tag(primitive(tags)),
        RequestCall::Score(primitive(score)),
        RequestCall::Filter(primitive(Question::decide("Fits?").unwrap().cut())),
        RequestCall::Rank(args(
            Question::rank("Fits?").unwrap().into(),
            RequestInput::Records {
                items: vec![item("Alpha.")],
            },
        )),
        RequestCall::Find(args(
            RequestDefinition::Atomic(LoadedQuestion::Question(Question::find("Which?").unwrap())),
            RequestInput::Units {
                items: vec![item("Alpha."), item("Beta.")],
            },
        )),
        RequestCall::Annotate(args(
            QuestionSet::builder()
                .question("fits", Question::decide("Fits?").unwrap().cut())
                .unwrap()
                .build()
                .unwrap()
                .into(),
            RequestInput::Records {
                items: vec![item("Alpha.")],
            },
        )),
        RequestCall::Recognize(args(
            Recognize::builder()
                .build()
                .unwrap()
                .with_examples(vec![RecognitionExample::Brackets(
                    "[Zoë | ENTITY] met Orbit.".into(),
                )])
                .unwrap()
                .into(),
            RequestInput::Records {
                items: vec![
                    item("Alpha."),
                    RequestItem {
                        seed_spans: None,
                        examples: Some(vec![]),
                        ..item("Beta.")
                    },
                ],
            },
        )),
        RequestCall::Relate(args(
            relate.into(),
            RequestInput::Entities {
                items: entity_items,
            },
        )),
    ];
    for call in cases {
        let function = call.function();
        let request = Request::new(call);
        let typed = request.clone().admit().unwrap();
        let definition = typed.resolve_question().unwrap();
        let items = match &request.call.arguments().input {
            RequestInput::Records { items }
            | RequestInput::Units { items }
            | RequestInput::Entities { items } => items,
            _ => unreachable!(),
        };
        let rows = items
            .iter()
            .map(|item| RecordInput {
                seed_spans: None,
                original: match item.original.as_ref().unwrap() {
                    RequestOriginal::Text { text } => QuestionInput::Text(text.clone()),
                    RequestOriginal::Json { value } => RecordReading::new(&[], None, None)
                        .unwrap()
                        .compose(value.clone())
                        .unwrap()
                        .original
                        .question_input(),
                },
                context: None,
                options: None,
                examples: item.examples.clone(),
            })
            .collect();
        let before = listener.count();
        let expected = direct(&engine(&listener), function, &definition, rows);
        let RequestOutcome::Complete(native) = engine(&listener)
            .execute_request(&typed, RequestEnvironment::default())
            .unwrap()
        else {
            panic!("complete typed call")
        };
        let canonical = Request::from_json(&serde_json::to_string(&request).unwrap())
            .unwrap()
            .admit()
            .unwrap();
        let RequestOutcome::Complete(decoded) = engine(&listener)
            .execute_request(&canonical, RequestEnvironment::default())
            .unwrap()
        else {
            panic!("complete canonical call")
        };
        assert_eq!(
            semantic(serde_json::to_value(native.value()).unwrap()),
            semantic(expected.clone()),
            "{function:?}"
        );
        assert_eq!(
            semantic(serde_json::to_value(decoded.value()).unwrap()),
            semantic(expected),
            "{function:?}"
        );
        let requests = listener.requests();
        let count = (listener.count() - before) / 3;
        assert!(count > 0, "{function:?}");
        for at in 0..count {
            assert_eq!(requests[at].body, requests[count + at].body, "{function:?}");
            assert_eq!(
                requests[at].body,
                requests[2 * count + at].body,
                "{function:?}"
            );
        }
        assert_eq!(
            native.facts().requests_sent(),
            decoded.facts().requests_sent()
        );
    }
}

#[test]
fn native_and_canonical_replays_keep_the_original_answer_identity() {
    let listener = Listener::answering(response).unwrap();
    let folder = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("request-identity-{}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("request-fixture")
        .unwrap()
        .cache_at(&folder)
        .unwrap()
        .max_retries(0)
        .build()
        .unwrap();
    let q = Question::decide("Fits?").unwrap().cut();
    let rows = vec![RecordInput {
        original: QuestionInput::Text("Alpha.".into()),
        context: None,
        options: None,
        examples: None,
        seed_spans: None,
    }];
    let live = direct(&engine, RequestFunction::Decide, &q.clone().into(), rows);
    let request = Request::new(RequestCall::Decide(args(
        q.into(),
        RequestInput::Records {
            items: vec![item("Alpha.")],
        },
    )));
    for request in [
        request.clone(),
        Request::from_json(&serde_json::to_string(&request).unwrap()).unwrap(),
    ] {
        let admitted = request.admit().unwrap();
        let RequestOutcome::Complete(call) = engine
            .execute_request(&admitted, RequestEnvironment::default())
            .unwrap()
        else {
            panic!("replay completes")
        };
        let value = serde_json::to_value(call.value()).unwrap();
        assert_eq!(value[0]["answer_id"], live[0]["answer_id"]);
        assert_eq!(
            value[0]["meta"]["observations"],
            live[0]["meta"]["observations"]
        );
        assert_eq!(call.facts().requests_sent(), 0);
        assert_eq!(call.facts().cache_answers(), 1);
    }
    assert_eq!(listener.count(), 1);
}

#[test]
fn text_request_band_keeps_native_threshold_reading() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let question = Question::decide("Fits?").unwrap().band(0.8, 0.95).unwrap();
    let expected = engine
        .decide_records_complete_with(
            &question,
            vec![RecordInput {
                original: QuestionInput::Text("Alpha.".into()),
                context: None,
                options: None,
                examples: None,
                seed_spans: None,
            }],
            CallOptions::new(),
        )
        .unwrap();
    let request = Request::new(RequestCall::Decide(RequestArguments {
        question: RequestQuestion::Text {
            text: "Fits?".into(),
        },
        input: RequestInput::Records {
            items: vec![item("Alpha.")],
        },
        options: RequestOptions {
            threshold: Some(RequestThreshold::Rule("0.8:0.95".into())),
            ..RequestOptions::default()
        },
    }))
    .admit()
    .unwrap();
    let RequestOutcome::Complete(actual) = engine
        .execute_request(&request, RequestEnvironment::default())
        .unwrap()
    else {
        panic!("complete banded call")
    };
    assert_eq!(
        semantic(serde_json::to_value(actual.value()).unwrap()),
        semantic(serde_json::to_value(expected.value()).unwrap())
    );
    let requests = listener.requests();
    assert_eq!(requests[0].body, requests[1].body);
}

#[test]
#[allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "isolated native selector behavior uses fixed loopback fixtures"
)]
fn authorized_saved_definition_preserves_selector_and_refuses_replacement_before_sends() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let saved = Request::new(RequestCall::Decide(RequestArguments {
        question: RequestQuestion::File {
            path: "not-read-by-request.json".into(),
        },
        input: RequestInput::Records {
            items: vec![item("Alpha.")],
        },
        options: RequestOptions::default(),
    }))
    .admit()
    .unwrap();
    let definition: RequestDefinition = Question::decide("Fits?").unwrap().cut().into();
    let mismatch: RequestDefinition =
        Question::from_json(r#"{"choose":"Which?","options":["a","b"]}"#)
            .unwrap()
            .into();
    assert_eq!(
        saved
            .clone()
            .with_resolved_definition(mismatch)
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    let declared: RequestDefinition =
        Question::from_json(r#"{"decide":"Fits?","item_schema":{"type":"object","properties":{"body":{"type":"string"}},"required":["body"]}}"#)
            .unwrap()
            .into();
    assert_eq!(
        saved
            .clone()
            .with_resolved_definition(declared)
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    let resolved = saved.with_resolved_definition(definition.clone()).unwrap();
    assert!(matches!(
        resolved.request().call.arguments().question,
        RequestQuestion::File { .. }
    ));
    assert_eq!(
        resolved
            .clone()
            .with_resolved_definition(definition.clone())
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    let inline = Request::new(RequestCall::Decide(args(
        definition.clone(),
        RequestInput::Records { items: vec![] },
    )))
    .admit()
    .unwrap();
    assert_eq!(
        inline
            .with_resolved_definition(definition)
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    assert_eq!(listener.requests().len(), 0);
    let outcome = engine
        .execute_request(&resolved, RequestEnvironment::default())
        .unwrap();
    assert!(matches!(outcome, RequestOutcome::Complete(_)));
    assert_eq!(listener.requests().len(), 1);
}
