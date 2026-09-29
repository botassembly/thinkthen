//! The portable corpus through a compiled command and its actual listener.

use sha2::{Digest as _, Sha256};

use super::{KEY, details, text};
use crate::harness::{Canned, Listener, spawn};

const CORPUS: &str =
    include_str!("../../../../../specification/fixtures/batching/portable-records.json");
const BODIES: [&str; 3] = [
    include_str!("../../../../../specification/fixtures/batching/portable-1.request.json"),
    include_str!("../../../../../specification/fixtures/batching/portable-2.request.json"),
    include_str!("../../../../../specification/fixtures/batching/portable-3.request.json"),
];

fn digest(url: &str, body: &[u8]) -> String {
    let mut bytes = b"systemone\n".to_vec();
    bytes.extend_from_slice(url.as_bytes());
    bytes.push(b'\n');
    bytes.extend_from_slice(body);
    Sha256::digest(&bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn answering(body: &[u8]) -> Canned {
    let request: serde_json::Value = serde_json::from_slice(body).expect("request JSON");
    let questions = request["questions"].as_object().expect("wire questions");
    let answers: serde_json::Map<String, serde_json::Value> = questions
        .keys()
        .map(|name| (name.clone(), serde_json::json!({"type":"noul","noul":0.9})))
        .collect();
    Canned::ok(&serde_json::json!({"model":"jev-1.13.0","answers":answers}).to_string())
}

#[test]
fn complete_cli_lines_keep_literal_cuts_bodies_and_request_identities() {
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
        assert_eq!(sent.len(), 3, "{framing}");
        let groups = [0, 0, 1, 1, 2];
        let mut digests = Vec::new();
        for (at, (request, literal)) in sent.iter().zip(BODIES).enumerate() {
            let body = literal
                .strip_suffix('\n')
                .expect("fixture newline")
                .as_bytes();
            assert_eq!(request.body, body, "{framing} request {at}");
            digests.push(digest(listener.url(), body));
        }
        for (at, row) in rows.iter().enumerate() {
            assert_eq!(row["input"], corpus["texts"][at]);
            assert_eq!(row["meta"]["requests"][0], digests[groups[at]]);
            if at < 4 {
                assert_eq!(row["meta"]["batch"]["closed"], "content");
                assert_eq!(row["meta"]["batch"]["records"], 2);
            } else {
                assert!(
                    row["meta"].get("batch").is_none(),
                    "singleton keeps old shape"
                );
            }
        }
    }
}

#[test]
fn structured_cli_values_keep_order_and_numeric_cut_distinctions() {
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
    assert_eq!(sent.len(), 2);
    for (at, (request, literal)) in sent.iter().zip(bodies).enumerate() {
        let body = literal
            .strip_suffix('\n')
            .expect("fixture newline")
            .as_bytes();
        assert_eq!(request.body, body, "structured request {at}");
    }
    assert_eq!(rows[0]["meta"]["batch"]["closed"], "content");
    assert_eq!(rows[1]["meta"]["batch"]["closed"], "content");
    assert!(rows[2]["meta"].get("batch").is_none());
    for (at, row) in rows.iter().enumerate() {
        let expected: serde_json::Value =
            serde_json::from_str(oracle["compact"][at].as_str().expect("compact value"))
                .expect("expected input");
        assert_eq!(row["input"], expected);
        let body = bodies[usize::from(at == 2)]
            .strip_suffix('\n')
            .expect("fixture newline")
            .as_bytes();
        assert_eq!(row["meta"]["requests"][0], digest(listener.url(), body));
    }
}
