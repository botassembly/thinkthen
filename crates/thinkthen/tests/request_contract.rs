//! Typed and canonical requests share native answers, identities and wire bodies.
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};
use thinkthen::*;

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
                let criteria = q["criteria"].as_object().unwrap();
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
                for value in fields.values_mut() { clean(value); }
            }
            Value::Array(items) => for value in items { clean(value); },
            _ => {}
        }
    }
    clean(&mut value);
    value
}
#[test]
fn every_function_keeps_native_answers_identities_and_outgoing_bodies() {
    let listener = Listener::answering(response).unwrap();
        let primitive = |q: Question| args(q.into(), RequestInput::Records { items: vec![item("Alpha.")] });
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
                items: vec![item("Alpha.")],
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
            Recognize::builder().build().unwrap().into(),
            RequestInput::Records {
                items: vec![item("Alpha.")],
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
        eprintln!("request equivalence: {function:?}");
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
            assert_eq!(
                requests[at].body,
                requests[count + at].body,
                "{function:?}"
            );
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
