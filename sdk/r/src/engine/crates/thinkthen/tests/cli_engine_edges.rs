#![cfg(test)]
//! Command diagnostics preserve transport causes and required table arguments.
#![cfg(feature = "command")]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
mod support;

use loopback::{KEY, Loopback, Reply};
use serde_json::json;
use std::io::Write;
use std::process::Stdio;

#[test]
fn oversized_backend_reply_names_its_body_limit() {
    let server = Loopback::start(|_| {
        let mut reply = Reply::json(200, &json!({}));
        reply.body = vec![b' '; 10 * 1024 * 1024 + 1];
        reply
    })
    .unwrap();
    let mut child = support::command()
        .args(["decide", "Ready?"])
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"evidence").unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(6));
    assert_eq!(support::reported(&output), "backend.transport");
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(
        error.contains("reply body exceeds 10485760 bytes"),
        "{error}"
    );
    assert!(!error.contains(KEY), "credentials remain private");
    assert_eq!(server.count(), 1);
}

#[test]
fn table_decide_requires_an_answer_column_before_file_work() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("input.csv");
    std::fs::write(&source, "body\nevidence\n").unwrap();
    let output = support::command()
        .args(["decide", "Ready?", "--backend", "stub", "--column", "body"])
        .arg(&source)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(support::reported(&output), "usage.option");
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(
        error.contains("into: expected an answer column name"),
        "{error}"
    );
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 1);
}
