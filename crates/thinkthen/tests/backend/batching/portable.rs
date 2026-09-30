//! The portable corpus through a compiled command and its actual listener.
//! The content cut is gone from the command, by ADR 0111, so every record
//! rides one request. Each record's question keeps the fixture's bytes, so
//! its key is the fixture question's key.

use super::{KEY, details, text};
use crate::harness::{Canned, Listener, spawn};
use crate::support::keys;

const CORPUS: &str =
    include_str!("../../../../../specification/fixtures/batching/portable-records.json");
const BODIES: [&str; 3] = [
    include_str!("../../../../../specification/fixtures/batching/portable-1.request.json"),
    include_str!("../../../../../specification/fixtures/batching/portable-2.request.json"),
    include_str!("../../../../../specification/fixtures/batching/portable-3.request.json"),
];

fn answering(body: &[u8]) -> Canned {
    let request: serde_json::Value = serde_json::from_slice(body).expect("request JSON");
    let questions = request["questions"].as_object().expect("wire questions");
    let answers: serde_json::Map<String, serde_json::Value> = questions
        .keys()
        .map(|name| (name.clone(), serde_json::json!({"type":"noul","noul":0.9})))
        .collect();
    Canned::ok(&serde_json::json!({"model":"jev-1.13.0","answers":answers}).to_string())
}

/// The keys of every question the fixture bodies hold, in order.
fn fixture_keys(url: &str, bodies: &[&str]) -> Vec<String> {
    bodies
        .iter()
        .flat_map(|body| {
            keys(
                url,
                body.strip_suffix('\n').expect("fixture newline").as_bytes(),
            )
        })
        .collect()
}

#[test]
fn complete_cli_lines_keep_literal_question_bytes_and_keys() {
    let corpus: serde_json::Value = serde_json::from_str(CORPUS).expect("literal corpus");
    for (framing, input) in [
        ("--lines", "alpha\ncafé-5544\nomega\nline 2907\ntail\n"),
        (
            "--jsonl",
            "\"alpha\"\n\"caf\\u00e9-5544\"\n\"omega\"\n\"line 2907\"\n\"tail\"\n",
        ),
    ] {
        let listener = Listener::answering(answering).expect("listener");
        let output = spawn(
            &[
                "decide",
                "Is it relevant?",
                framing,
                "--details",
                "--batch",
                "max",
                "--no-cache",
                "--max-retries",
                "0",
                "--jobs",
                "1",
                "--model",
                "jev-1.13.0",
                "--url",
                listener.base(),
            ],
            &[KEY],
            input.as_bytes(),
        )
        .expect("compiled command");
        assert_eq!(
            output.status.code(),
            Some(0),
            "{framing}: {}",
            text(&output.stderr)
        );
        let rows = details(&output);
        assert_eq!(rows.len(), 5, "{framing}");
        let sent = listener.requests();
        assert_eq!(sent.len(), 1, "{framing}");
        let expected = fixture_keys(listener.url(), &BODIES);
        assert_eq!(keys(listener.url(), &sent[0].body), expected, "{framing}");
        for (at, row) in rows.iter().enumerate() {
            assert_eq!(row["input"], corpus["texts"][at]);
            assert_eq!(row["meta"]["requests"], serde_json::json!([expected[at]]));
            assert!(row["meta"].get("batch").is_none());
        }
    }
}

#[test]
fn structured_cli_values_keep_order_and_question_bytes() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../../specification/fixtures/batching/portable-structured.json"
    ))
    .expect("structured oracle");
    let input =
        include_str!("../../../../../specification/fixtures/batching/portable-structured.jsonl");
    let bodies = [
        include_str!(
            "../../../../../specification/fixtures/batching/portable-structured-1.request.json"
        ),
        include_str!(
            "../../../../../specification/fixtures/batching/portable-structured-2.request.json"
        ),
    ];
    let listener = Listener::answering(answering).expect("listener");
    let output = spawn(
        &[
            "decide",
            "Is it relevant?",
            "--jsonl",
            "--details",
            "--batch",
            "max",
            "--no-cache",
            "--max-retries",
            "0",
            "--jobs",
            "1",
            "--model",
            "jev-1.13.0",
            "--url",
            listener.base(),
        ],
        &[KEY],
        input.as_bytes(),
    )
    .expect("compiled command");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let rows = details(&output);
    assert_eq!(rows.len(), 3);
    let sent = listener.requests();
    assert_eq!(sent.len(), 1);
    let expected = fixture_keys(listener.url(), &bodies);
    assert_eq!(keys(listener.url(), &sent[0].body), expected);
    for (at, row) in rows.iter().enumerate() {
        let input: serde_json::Value =
            serde_json::from_str(oracle["compact"][at].as_str().expect("compact value"))
                .expect("expected input");
        assert_eq!(row["input"], input);
        assert_eq!(row["meta"]["requests"], serde_json::json!([expected[at]]));
    }
}
