//! Find's none option and annotate's record parts: the refusals, limits, order,
//! and secrecy that no shared case reaches. Each row counts requests at a real
//! loopback listener.

use std::sync::{Arc, Mutex};

use conformance_backend::{Backend, Canned, Listener};
use serde_json::{Value, json};
use thinkthen::{Engine, Error, ErrorKind, Question, QuestionSet};

use crate::cases::{Checked, at, engine, said, same};

const TWO_GROUPS: &str = r#"{"version":1,"questions":{"summary":{"decide":"Is this concise?","on":"/summary"},"body":{"decide":"Does this ask for a refund?","on":"/body"}}}"#;
const NOTE: &str = "a private note";

/// A usage error with this exact sentence.
fn usage(error: &Error, sentence: &str) {
    assert_eq!(
        (error.kind(), error.to_string().as_str()),
        (ErrorKind::Usage, sentence)
    );
}

fn units(count: usize) -> Vec<String> {
    (0..count).map(|place| format!("unit {place}")).collect()
}

#[test]
fn offering_none_takes_find_questions_alone() {
    for asked in [
        Question::rank("Which first?").expect("rank"),
        Question::decide("Is it?").expect("decide").cut(),
    ] {
        usage(
            &asked.offering_none().expect_err("refused"),
            "only a find question offers none",
        );
    }
    let once = Question::find("Which one?")
        .and_then(Question::offering_none)
        .expect("once");
    assert_eq!(once.clone().offering_none().expect("twice"), once);
    assert_ne!(once, Question::find("Which one?").expect("plain"));
}

#[test]
fn a_none_question_takes_2_to_254_units() {
    let backend = Backend::start().expect("backend");
    let engine = engine(&format!("{}/generic/v1", backend.origin())).expect("engine");
    let plain = Question::find("Which one?").expect("find");
    let none = plain.clone().offering_none().expect("none");
    let limit = "a find question offering none takes 2 to 254 units";
    usage(&engine.find(&none, units(255)).expect_err("255"), limit);
    usage(&engine.find(&none, units(1)).expect_err("1"), limit);
    assert_eq!(backend.count(), 0);
    let found = engine.find(&none, units(254)).expect("254").into_value();
    let last = found.candidates().last().expect("a candidate");
    assert!(last.is_none() && found.candidates().len() == 255);
    assert_eq!(found.selected().map(String::as_str), Some("unit 0"));
    engine
        .find(&plain, units(255))
        .expect("a plain find of 255");
    assert_eq!(backend.count(), 2);
}

/// Every request body the listener saw, answering each yes question at 0.9.
fn listening() -> (Listener, Arc<Mutex<Vec<Value>>>) {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let kept = Arc::clone(&seen);
    let listener = Listener::answering(move |body| {
        let request: Value = serde_json::from_slice(body).expect("a JSON request");
        let answers: serde_json::Map<String, Value> = request["questions"]
            .as_object()
            .into_iter()
            .flatten()
            .map(|(name, _)| {
                (
                    name.clone(),
                    serde_json::json!({"type": "noul", "noul": 0.9}),
                )
            })
            .collect();
        let reply = serde_json::json!({"model": request["model"], "answers": answers});
        kept.lock().expect("the body list").push(request);
        Canned::ok(&reply.to_string())
    })
    .expect("a listener");
    (listener, seen)
}

/// The record each request quoted in its first question, in listener arrival
/// order, after one annotate call. Every state is the fixed sentence.
fn states(set: &str, record: &str) -> Result<Vec<Value>, Error> {
    let (listener, seen) = listening();
    let engine = engine(listener.base()).expect("engine");
    let set = QuestionSet::from_json(set).expect("a set");
    let row = engine.annotate(&set, [record]).next().expect("one row");
    let sent = seen.lock().expect("the body list").clone();
    row.map(|_| sent.iter().map(quoted).collect())
        .inspect_err(|_| {
            assert!(sent.is_empty(), "a refused record sent {}", sent.len());
        })
}

/// The JSON value a request's first question quotes after "The text is ".
fn quoted(body: &Value) -> Value {
    assert_eq!(
        body["state"],
        "Each question quotes the text it asks about."
    );
    let text = body["questions"]["q1"]["instructions"]
        .as_str()
        .unwrap_or_default();
    let rest = text
        .strip_prefix("The text is ")
        .expect("a quoted question");
    serde_json::Deserializer::from_str(rest)
        .into_iter::<Value>()
        .next()
        .and_then(Result::ok)
        .expect("a quoted record")
}

#[test]
fn each_group_sees_its_part_and_a_bad_part_sends_nothing() {
    let refusals = [
        (
            r#"{"summary":"a private note"}"#,
            "the record holds nothing at `/body`",
        ),
        (
            NOTE,
            "question `summary` reads `on`, and this record's evidence is text with no members",
        ),
    ];
    for (record, sentence) in refusals {
        let error = states(TWO_GROUPS, record).expect_err("refused");
        usage(&error, sentence);
        assert!(!format!("{error} {error:?}").contains(NOTE), "{error:?}");
    }
    let number = states(TWO_GROUPS, r#"{"summary":12.5,"body":"Refund me."}"#);
    let number = number.expect("answered");
    assert_eq!(number.len(), 2);
    assert!(number.contains(&Value::from("12.5")));
    assert!(number.contains(&Value::from("Refund me.")));
    let root = r#"{"version":1,"questions":{"whole":{"decide":"Is this long?"}}}"#;
    assert_eq!(states(root, NOTE).expect("answered"), [Value::from(NOTE)]);
    let twice = r#"{"a":1,"a":2}"#;
    assert_eq!(states(root, twice).expect("answered"), [Value::from(twice)]);
    let mixed = r#"{"version":1,"questions":{"whole":{"decide":"Is this long?"},"body":{"decide":"Does this ask for a refund?","on":"/body"}}}"#;
    let record = r#"{"body":"Refund me."}"#;
    let mixed = states(mixed, record).expect("answered");
    assert_eq!(mixed.len(), 2);
    assert!(mixed.contains(&Value::from(record)));
    assert!(mixed.contains(&Value::from("Refund me.")));
}

/// A find over the case's units, with its none candidate when asked.
pub(crate) fn found(engine: &Engine, asked: &Value, success: &Value) -> Checked {
    let mut question = Question::find(asked["find"].as_str().unwrap_or_default()).map_err(said)?;
    if asked["none"] == json!(true) {
        question = question.offering_none().map_err(said)?;
    }
    let units: Vec<&str> = asked["units"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    let found = engine
        .find(&question, units.clone())
        .map_err(said)?
        .into_value();
    let rows = found.candidates().iter().map(|candidate| {
        let index = candidate.input().and_then(|unit| at(&units, unit));
        json!({"index": index, "probability": candidate.probability()})
    });
    let operation = &success["operation"];
    same("candidates", &rows.collect(), &operation["probabilities"])?;
    let selected = found.selected().and_then(|unit| at(&units, unit));
    same("selected", &json!(selected), &operation["selected"])
}
