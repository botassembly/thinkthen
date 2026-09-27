//! The compiled record readers skip blank text lines without losing input line numbers.
use serde_json::{Map, Value, json};

use crate::harness::{Canned, Listener, spawn};

fn answer(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap_or(Value::Null);
    let answers: Map<String, Value> = request
        .get("questions")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|questions| questions.keys())
        .map(|name| (name.clone(), json!({"type": "noul", "noul": 0.99})))
        .collect();
    Canned::ok(&json!({"model": "jev-1.13.0", "answers": answers}).to_string())
}

#[test]
fn a_blank_line_is_skipped_and_keeps_its_number() {
    let listener = Listener::answering(answer).expect("a loopback listener");
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

#[test]
fn blank_lines_keep_stop_positions_and_jsonl_still_refuses_them() {
    for (framing, input, exit, stopped_at, cause) in [
        (
            "--lines",
            &b"a\n  \n\xff\n"[..],
            5,
            3,
            "the record is not valid UTF-8",
        ),
        (
            "--jsonl",
            &b"{\"body\":\"a\"}\n\n{\"body\":\"b\"}\n"[..],
            2,
            2,
            "the record is not valid JSON",
        ),
    ] {
        let listener = Listener::answering(answer).expect("a loopback listener");
        let output = spawn(
            &[
                "decide",
                "Is it kept?",
                framing,
                "--batch",
                "1",
                "--jobs",
                "1",
                "--no-cache",
                "--url",
                listener.base(),
                "--model",
                "jev-1.13.0",
            ],
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            input,
        )
        .expect("the command runs");
        assert_eq!(output.status.code(), Some(exit), "{framing}");
        assert_eq!(listener.count(), 1, "{framing}");
        assert_eq!(String::from_utf8_lossy(&output.stdout).lines().count(), 1);
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            format!(
                "thinkthen: {cause}\nthinkthen: stopped at record {stopped_at}; 1 record finished\n"
            ),
            "{framing}"
        );
    }
}
