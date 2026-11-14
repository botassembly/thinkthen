#![cfg(test)]
//! Choice behavior through the Rust API.

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback};
use serde_json::json;
use thinkthen::{Choose, Cut, Engine, Key, Outcome, Question, Record, Settings};

fn engine(server: &Loopback) -> Engine {
    Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .engine(|_| None)
        .unwrap()
}

#[test]
fn the_largest_probability_wins_with_first_option_breaking_ties() {
    for (probabilities, labels, threshold, expected, p) in [
        (
            json!({"z": 0.2, "a": 0.8}),
            ["z", "a"],
            Cut::new(0.5).unwrap(),
            Some("a"),
            0.8,
        ),
        (
            json!({"z": 0.5, "a": 0.5}),
            ["z", "a"],
            Cut::new(0.5).unwrap(),
            Some("z"),
            0.5,
        ),
        (
            json!({"z": 0.5, "a": 0.5}),
            ["a", "z"],
            Cut::new(0.5).unwrap(),
            Some("a"),
            0.5,
        ),
        (
            json!({"z": 0.3, "a": 0.7}),
            ["z", "a"],
            Cut::new(0.8).unwrap(),
            None,
            0.7,
        ),
    ] {
        let server =
            Loopback::answering(move |_| json!({"type": "choice", "probabilities": probabilities}))
                .unwrap();
        let question = Choose::with_strings("Which?", labels)
            .unwrap()
            .threshold(threshold);
        let row = engine(&server)
            .run(&question, Record::text("evidence"))
            .unwrap();
        match (row.outcome(), expected) {
            (
                Outcome::Answered {
                    answer: label,
                    probability: Some(got),
                    ..
                },
                Some(expected),
            ) => {
                assert_eq!(label, expected);
                assert_eq!(got.get(), p);
            }
            (
                Outcome::Unsure {
                    probability: got, ..
                },
                None,
            ) => assert_eq!(got.get(), p),
            other => panic!("unexpected choice outcome {other:?}"),
        }
    }
}

#[test]
fn invalid_questions_and_threshold_bands_are_refused_at_construction() {
    for (text, labels) in [
        ("Which?", vec!["a", "a"]),
        ("Which?", vec![]),
        ("Which?", vec!["a"]),
        (" ", vec!["a", "b"]),
    ] {
        assert_eq!(
            Choose::with_strings(text, labels)
                .unwrap_err()
                .code()
                .as_str(),
            "usage.question"
        );
    }
    assert_eq!(
        Choose::with_strings("Which?", [" ", "a"])
            .unwrap_err()
            .code()
            .as_str(),
        "usage.question"
    );
    let error = Question::from_json(r#"{"function":"choose","question":"Which?","choices":["a","b"],"threshold":{"low":0.2,"high":0.8}}"#).unwrap_err();
    assert_eq!(error.code().as_str(), "usage.threshold");
}

#[test]
fn criteria_preserve_caller_order() {
    let server =
        Loopback::answering(|_| json!({"type":"choice", "probabilities":{"z":0.9,"a":0.1}}))
            .unwrap();
    let question = Choose::with_strings("Which?", ["z", "a"]).unwrap();
    engine(&server)
        .run(&question, Record::text("Say \"hi\""))
        .unwrap();
    let requests = server.requests();
    let body = String::from_utf8(requests[0].body.clone()).unwrap();
    assert!(body.contains(r#""criteria":{"z":null,"a":null}"#), "{body}");
    assert_eq!(
        requests[0].json().unwrap()["questions"]["q1"]["instructions"],
        "The text is \"Say \\\"hi\\\"\". Which?"
    );
}

#[test]
fn malformed_choice_replies_fail_the_scalar_call() {
    for answer in [
        json!({"type":"choice", "probabilities":{"z":0.8,"a":0.2,"unknown":0.1}}),
        json!({"type":"choice", "probabilities":{"z":0.8}}),
        json!({"type":"choice", "choice":"unknown", "probabilities":{"z":0.8,"a":0.2}}),
        json!({"type":"choice", "probabilities":{"z":1.2,"a":0.2}}),
        json!({"type":"noul", "noul":0.9}),
    ] {
        let server = Loopback::answering(move |_| answer.clone()).unwrap();
        let question = Choose::with_strings("Which?", ["z", "a"]).unwrap();
        let error = engine(&server)
            .run(&question, Record::text("evidence"))
            .unwrap_err();
        assert_eq!(error.code().as_str(), "backend.reply");
    }
}
