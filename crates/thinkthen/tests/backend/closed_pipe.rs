//! The compiled filter stops asking after its output reader closes.
use crate::child::ChildEnvironment as _;
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
        .clear_environment()
        .home(env!("CARGO_TARGET_TMPDIR"))
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

#[cfg(unix)]
#[test]
fn native_readiness_emits_a_row_and_cancels_without_waiting_for_input_eof() {
    use std::os::unix::process::ExitStatusExt as _;
    let questions = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("reader-questions-{}.json", std::process::id()));
    std::fs::write(
        &questions,
        r#"{"version":1,"questions":{"keep":{"decide":"Keep?"}}}"#,
    )
    .expect("questions");
    for verb in ["decide", "annotate", "recognize"] {
        let listener = Listener::answering(move |body| {
            if verb == "recognize" {
                super::recognize::automatic(body)
            } else {
                reply(body)
            }
        })
        .expect("loopback listener");
        let question = match verb {
            "annotate" => questions.to_str().expect("question path"),
            "recognize" => "person,organization",
            _ => "Keep?",
        };
        let mut child = crate::harness::command(
            &[
                verb,
                question,
                "--lines",
                "--jobs",
                "1",
                "--no-cache",
                "--url",
                listener.base(),
                "--model",
                "jev-1.13.0",
            ],
            &[("THINKTHEN_API_KEY", "sk-reader-loopback")],
        )
        .env("THINKTHEN_TEST_INPUT_PAUSE_MS", "1")
        .stdin(Stdio::piped())
        .spawn()
        .expect("command");
        let mut input = child.stdin.take().expect("stdin");
        input.write_all(b"Ada met Acme.\n").expect("first record");
        input.flush().expect("flush");
        let stdout = child.stdout.take().expect("stdout");
        let (sent, received) = std::sync::mpsc::channel();
        let reader = std::thread::spawn(move || {
            let mut row = String::new();
            let mut stdout = BufReader::new(stdout);
            let result = stdout.read_line(&mut row);
            let _sent = sent.send((result, row, stdout));
        });
        let first = received.recv_timeout(Duration::from_secs(5));
        if first.is_ok() {
            assert!(
                crate::child::command("kill", &[])
                    .args(["-INT", &child.id().to_string()])
                    .status()
                    .expect("signal")
                    .success()
            );
        } else {
            child.kill().expect("stop hung reader");
        }
        let output = finish(child, "cancel input reader").expect("command ends with input open");
        reader.join().expect("stdout reader");
        drop(input);
        let (result, row, _stdout) = first.expect("a row arrives before input EOF");
        assert!(result.expect("read row") > 0, "{verb}");
        let row: Value = serde_json::from_str(&row).expect("native row");
        assert_eq!(row["input"], "Ada met Acme.", "{verb}");
        assert_eq!(
            output.status.signal(),
            Some(signal_hook::consts::signal::SIGINT),
            "{verb}: {:?}",
            output.stderr
        );
        assert!(listener.count() > 0, "{verb}");
        assert!(!String::from_utf8_lossy(&output.stderr).contains("sk-reader-loopback"));
    }
    std::fs::remove_file(questions).expect("remove questions");
}
