//! The compiled record readers skip blank text lines without losing input line numbers.
use serde_json::{Map, Value, json};

use crate::harness::{Canned, Listener, spawn};

#[test]
fn a_blank_line_is_skipped_and_keeps_its_number() {
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("a request");
        let answers: Map<String, Value> = request["questions"]
            .as_object()
            .expect("questions")
            .keys()
            .map(|name| (name.clone(), json!({"type": "noul", "noul": 0.99})))
            .collect();
        Canned::ok(&json!({"model": "jev-1.13.0", "answers": answers}).to_string())
    })
    .expect("a loopback listener");
    let output = spawn(
        &[
            "decide",
            "Is it kept?",
            "--lines",
            "--batch",
            "1",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "jev-1.13.0",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"a\n   \nb\n\r\nc\n\n",
    )
    .expect("the command runs");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(listener.count(), 3);
    assert_eq!(String::from_utf8_lossy(&output.stdout).lines().count(), 3);
    assert!(output.stderr.is_empty());
}
