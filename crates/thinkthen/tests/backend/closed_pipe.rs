//! The compiled filter stops asking after its output reader closes.
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::{Map, Value, json};

use crate::harness::{Canned, Listener, finish, start};

fn reply(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap_or(Value::Null);
    let state = request
        .get("state")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let answers: Map<String, Value> = request
        .get("questions")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|questions| questions.iter())
        .map(|(name, question)| {
            let text = question
                .get("instructions")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let yes = state.contains("keep first") || text.contains("keep first");
            (
                name.clone(),
                json!({"type": "noul", "noul": if yes { 0.99 } else { 0.01 }}),
            )
        })
        .collect();
    Canned::ok(&json!({"model": "jev-1.13.0", "answers": answers}).to_string()).after(20)
}

#[test]
fn a_reader_that_closes_the_pipe_stops_filter_between_dispatches() {
    let input = [
        "keep first\n".to_owned(),
        (2..=301).map(|n| format!("drop {n}\n")).collect(),
    ]
    .concat();
    for batch in ["1", "10"] {
        let listener = Listener::answering(reply).expect("a loopback listener");
        let mut child = start(
            &[
                "filter",
                "Keep?",
                "--batch",
                batch,
                "--jobs",
                "4",
                "--no-cache",
                "--url",
                listener.base(),
                "--model",
                "jev-1.13.0",
            ],
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            input.as_bytes(),
        )
        .expect("the command starts");
        let mut reader = BufReader::new(child.stdout.take().expect("stdout pipe"));
        let mut first = String::new();
        reader.read_line(&mut first).expect("first kept row");
        assert_eq!(first, "keep first\n");
        drop(reader);
        let output = finish(child, "filter after reader closes").expect("the command ends");
        assert_eq!(
            output.status.code(),
            Some(0),
            "batch {batch}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let sent = listener.count();
        assert!(sent <= 12, "batch {batch} kept scheduling: {sent}");
    }
}

#[test]
fn a_closed_reader_ends_filter_while_its_input_remains_open() {
    let listener = Listener::answering(reply).expect("a loopback listener");
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .args([
            "filter",
            "Keep?",
            "--batch",
            "1",
            "--jobs",
            "1",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "jev-1.13.0",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("filter starts");
    let mut input = child.stdin.take().expect("stdin pipe");
    input.write_all(b"keep first\n").expect("first record");
    input.flush().expect("record reaches the command");
    let mut reader = BufReader::new(child.stdout.take().expect("stdout pipe"));
    let mut first = String::new();
    reader.read_line(&mut first).expect("first kept row");
    assert_eq!(first, "keep first\n");
    drop(reader);

    let deadline = Instant::now() + Duration::from_secs(30);
    let status = loop {
        if let Some(status) = child.try_wait().expect("filter status") {
            break status;
        }
        if Instant::now() >= deadline {
            child.kill().expect("kill hung filter");
            child.wait().expect("reap hung filter");
            panic!("filter waited for open stdin after its output reader closed");
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    let output = child
        .wait_with_output()
        .expect("collect the stopped command");
    assert_eq!(status.code(), Some(0));
    assert!(output.stderr.is_empty(), "{:?}", output.stderr);
    assert_eq!(listener.count(), 1);
    drop(input);
}
