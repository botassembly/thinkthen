#![cfg(test)]
//! Choice behavior through the Rust API.

use loopback::{KEY, Loopback};
use serde_json::json;
use thinkthen::{Answer, Backend, ChoiceOption, Choose, Engine, Outcome, Record, Threshold};

fn backend(server: &Loopback) -> Backend {
    Backend::resolve(Some(server.base()), None, |name| {
        (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
    })
    .unwrap()
}

fn options(labels: &[&str]) -> Vec<ChoiceOption> {
    labels
        .iter()
        .map(|label| ChoiceOption::new(*label).unwrap())
        .collect()
}

#[test]
fn the_largest_probability_wins_with_first_option_breaking_ties() {
    for (probabilities, labels, threshold, expected, p) in [
        (
            json!({"z": 0.2, "a": 0.8}),
            ["z", "a"],
            "0.5",
            Some("a"),
            0.8,
        ),
        (
            json!({"z": 0.5, "a": 0.5}),
            ["z", "a"],
            "0.5",
            Some("z"),
            0.5,
        ),
        (
            json!({"z": 0.5, "a": 0.5}),
            ["a", "z"],
            "0.5",
            Some("a"),
            0.5,
        ),
        (json!({"z": 0.3, "a": 0.7}), ["z", "a"], "0.8", None, 0.7),
        (
            json!({"z": 0.2, "a": 0.8}),
            ["z", "a"],
            "0.2:0.8",
            Some("a"),
            0.8,
        ),
    ] {
        let server =
            Loopback::answering(move |_| json!({"type": "choice", "probabilities": probabilities}))
                .unwrap();
        let question = Choose::new("Which?", options(&labels))
            .unwrap()
            .threshold(threshold.parse::<Threshold>().unwrap());
        let mut rows = Vec::new();
        Engine::new(backend(&server))
            .call(&question, [Record::text("evidence")], |row| rows.push(row))
            .unwrap();
        assert_eq!(rows.len(), 1);
        match (rows[0].outcome(), expected) {
            (
                Outcome::Answered {
                    answer: Answer::Label(label),
                    p: Some(got),
                    ..
                },
                Some(expected),
            ) => {
                assert_eq!(label, expected);
                assert_eq!(got.get(), p);
            }
            (Outcome::Unsure { p: got, .. }, None) => assert_eq!(got.get(), p),
            other => panic!("unexpected choice outcome {other:?}"),
        }
    }
}

#[test]
fn invalid_questions_are_refused_at_construction() {
    for (text, labels) in [
        ("Which?", vec!["a", "a"]),
        ("Which?", vec![]),
        ("Which?", vec!["a"]),
        (" ", vec!["a", "b"]),
    ] {
        assert_eq!(
            Choose::new(text, options(&labels))
                .unwrap_err()
                .kind()
                .code(),
            "usage.question"
        );
    }
    assert_eq!(
        ChoiceOption::new(" \n").unwrap_err().kind().code(),
        "usage.question"
    );
}

#[test]
fn criteria_preserve_order_and_optional_descriptions() {
    let server =
        Loopback::answering(|_| json!({"type":"choice", "probabilities":{"z":0.9,"a":0.1}}))
            .unwrap();
    let question = Choose::new(
        "Which?",
        [
            ChoiceOption::new("z").unwrap().description("First."),
            ChoiceOption::new("a").unwrap(),
        ],
    )
    .unwrap();
    Engine::new(backend(&server))
        .call(&question, [Record::text("Say \"hi\"")], |_| {})
        .unwrap();
    let requests = server.requests();
    let body = String::from_utf8(requests[0].body.clone()).unwrap();
    assert!(
        body.contains(r#""criteria":{"z":"First.","a":null}"#),
        "{body}"
    );
    assert_eq!(
        requests[0].json().unwrap()["questions"]["q1"]["instructions"],
        "The text is \"Say \\\"hi\\\"\". Which?"
    );
}

#[test]
fn malformed_choice_replies_fail_the_record() {
    for answer in [
        json!({"type":"choice", "probabilities":{"z":0.8,"a":0.2,"unknown":0.1}}),
        json!({"type":"choice", "probabilities":{"z":0.8}}),
        json!({"type":"choice", "choice":"unknown", "probabilities":{"z":0.8,"a":0.2}}),
        json!({"type":"choice", "probabilities":{"z":1.2,"a":0.2}}),
        json!({"type":"noul", "noul":0.9}),
    ] {
        let server = Loopback::answering(move |_| answer.clone()).unwrap();
        let question = Choose::new("Which?", options(&["z", "a"])).unwrap();
        let mut rows = Vec::new();
        Engine::new(backend(&server))
            .call(&question, [Record::text("evidence")], |row| rows.push(row))
            .unwrap_err();
        assert_eq!(rows.len(), 1);
        match rows[0].outcome() {
            Outcome::Failed { error } => assert_eq!(error.kind().code(), "backend.reply"),
            other => panic!("expected failure, got {other:?}"),
        }
    }
}
