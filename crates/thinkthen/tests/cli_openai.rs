#![cfg(feature = "command")]
//! Adapter selection through the command.
#![cfg(test)]

mod support;

use std::io::Write;
use std::process::Stdio;

use loopback::{KEY, Loopback, Reply};
use serde_json::json;

#[test]
fn decide_selects_openai_and_uses_its_key_variable() {
    let server = Loopback::start(|_| {
        Reply::json(
            200,
            &json!({
                "answers":[{"type":"predicate","probability":0.9}]
            }),
        )
    })
    .unwrap();
    let mut command = support::command();
    command
        .args([
            "decide",
            "Is it urgent?",
            "--backend",
            "openai",
            "--url",
            server.base(),
        ])
        .env("OPENAI_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"Please respond now.")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"true\n");
    assert!(output.stderr.is_empty());
    assert_eq!(support::reported(&output), "");
    assert_eq!(server.requests()[0].path, "/v1/decisions");
    assert_eq!(
        server.requests()[0].header("authorization"),
        Some(format!("Bearer {KEY}").as_str())
    );
}
