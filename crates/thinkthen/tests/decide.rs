#![cfg(test)]
//! `decide` through the library.

use loopback::{KEY, Loopback, Reply, UnavailableBackend};
use serde_json::json;
use thinkthen::{Decide, Engine, Key, Outcome, Record, Row, Settings, Threshold};

fn engine(base: &str) -> Engine {
    Settings::default()
        .url(base)
        .api_key(Key::new(KEY).unwrap())
        .engine(|_| None)
        .unwrap()
}

fn decide(server: &Loopback, threshold: Option<&str>, text: &str) -> Row<Decide> {
    let mut question = Decide::new("Is it a refund request?").unwrap();
    if let Some(threshold) = threshold {
        question = question.threshold(threshold.parse().unwrap());
    }
    one(&engine(server.base()), &question, text)
}

fn one(engine: &Engine, question: &Decide, text: &str) -> Row<Decide> {
    engine.run(question, Record::text(text)).unwrap()
}

fn failure_code(row: &Row<Decide>) -> &'static str {
    match row.outcome() {
        Outcome::Failed { error } => error.code().as_str(),
        other => panic!("expected a failure, got {other:?}"),
    }
}

#[test]
fn the_threshold_turns_the_probability_into_yes_no_or_unsure() {
    let cases = [
        (0.5, None, Some(true)),
        (0.49, None, Some(false)),
        (0.7, Some("0.7"), Some(true)),
        (0.69, Some("0.7"), Some(false)),
        (1.0, Some("1"), Some(true)),
        (0.8, Some("0.2:0.8"), Some(true)),
        (0.79, Some("0.2:0.8"), None),
        (0.2, Some("0.2:0.8"), None),
        (0.19, Some("0.2:0.8"), Some(false)),
        (0.01, Some("0.01:0.5"), None),
        (1.0, Some("0.5:1"), Some(true)),
    ];
    for (p, threshold, expected) in cases {
        let server = Loopback::noul(p).unwrap();
        let row = decide(&server, threshold, "Please refund me.");
        let got = match row.outcome() {
            Outcome::Answered { answer: yes, .. } => Some(*yes),
            Outcome::Unsure { .. } => None,
            other => panic!("p {p} at {threshold:?}: {other:?}"),
        };
        assert_eq!(got, expected, "p {p} at {threshold:?}");
    }
}

#[test]
fn a_threshold_outside_the_rule_is_refused() {
    for text in [
        "0", "1.5", "90", "-0.1", "0:0.5", "0:1", "0.5:1.5", "0.9:0.1", "0.5:0.5", ":0.5", "0.5:",
        "nan", "inf", "x",
    ] {
        let error = text.parse::<Threshold>().unwrap_err();
        assert_eq!(error.code().as_str(), "usage.threshold", "{text}");
    }
}

#[test]
fn a_blank_question_is_refused() {
    assert_eq!(
        Decide::new(" \n").unwrap_err().code().as_str(),
        "usage.question"
    );
}

#[test]
fn the_request_quotes_the_text_inside_its_question() {
    let instructions = "The text is \"Say \\\"refund\\\"\\nnow\". Is it a refund request?";
    let question = Decide::new("Is it a refund request?").unwrap();
    let wire = json!({"type":"noul", "instructions":instructions});
    {
        let server = Loopback::noul(0.9).unwrap();
        one(&engine(server.base()), &question, "Say \"refund\"\nnow");
        let requests = server.requests();
        let [request] = requests.as_slice() else {
            panic!("expected one request, got {}", requests.len());
        };
        assert_eq!(request.path, "/v1/systemone");
        assert_eq!(
            request.header("authorization"),
            Some(format!("Bearer {KEY}").as_str())
        );
        assert_eq!(
            request.json().unwrap(),
            json!({
                "state": "Each question quotes the text it asks about.",
                "model": "jev-1.13.0",
                "questions": {"q1": wire}
            })
        );
    }
}

#[test]
fn a_bad_reply_fails_the_row_with_its_code() {
    let cases: [(u16, serde_json::Value, &str); 7] = [
        (200, json!({"answers": {"q1": 5}}), "backend.reply"),
        (
            200,
            json!({"answers": {"q1": {"noul": 0.5}}}),
            "backend.reply",
        ),
        (503, json!({}), "backend.status"),
        (200, json!("not a reply"), "backend.reply"),
        (200, json!({"answers": {}}), "backend.reply"),
        (
            200,
            json!({"answers": {"q1": {"type": "noul", "noul": 1.5}}}),
            "backend.reply",
        ),
        (
            200,
            json!({"answers": {"q1": {"type": "choice"}}}),
            "backend.reply",
        ),
    ];
    for (status, body, code) in cases {
        let server = Loopback::start(move |_| Reply::json(status, &body)).unwrap();
        assert_eq!(
            failure_code(&decide(&server, None, "text")),
            code,
            "{status}"
        );
    }
}

#[test]
fn an_unreachable_backend_fails_the_row() {
    let unavailable = UnavailableBackend::new().unwrap();
    let question = Decide::new("Is it?").unwrap();
    let row = one(&engine(unavailable.base()), &question, "text");
    assert_eq!(failure_code(&row), "backend.transport");
}

#[test]
fn the_key_is_sent_without_surrounding_whitespace() {
    let server = Loopback::noul(0.9).unwrap();
    let base = server.base().to_owned();
    let engine = Settings::default()
        .url(base)
        .engine(|name| (name == "THINKTHEN_API_KEY").then(|| format!(" {KEY}\n")))
        .unwrap();
    one(&engine, &Decide::new("Is it?").unwrap(), "text");
    let requests = server.requests();
    assert_eq!(
        requests[0].header("authorization"),
        Some(format!("Bearer {KEY}").as_str())
    );
}

#[test]
fn a_missing_key_is_refused_without_exposing_credential_input() {
    for key in [None, Some(""), Some(" \t\n")] {
        let error = Settings::default()
            .engine(|name| {
                (name == "THINKTHEN_API_KEY")
                    .then(|| key.map(str::to_owned))
                    .flatten()
            })
            .unwrap_err();
        assert_eq!(error.code().as_str(), "usage.key-missing");
        assert!(!error.to_string().contains(KEY));
    }
}

#[test]
fn the_key_appears_in_no_result_or_debug_line() {
    let server =
        Loopback::start(|_| Reply::json(401, &json!({"error": format!("bad key {KEY}")}))).unwrap();
    let engine = engine(server.base());
    let row = one(&engine, &Decide::new("Is it?").unwrap(), "text");
    assert_eq!(failure_code(&row), "backend.status");
    for shown in [
        format!("{row:?}"),
        serde_json::to_string(&row).unwrap(),
        format!("{engine:?}"),
    ] {
        assert!(!shown.contains(KEY), "{shown}");
    }
}
