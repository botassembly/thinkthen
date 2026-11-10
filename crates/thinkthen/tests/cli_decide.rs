#![cfg(test)]
//! `thinkthen decide` on the command line.
#![cfg(feature = "command")]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

mod support;

use std::io::Write;
use std::process::{Output, Stdio};

use loopback::{KEY, Loopback, Reply};
use serde_json::json;
use support::reported;

fn run(server: &Loopback, args: &[&str], key: Option<&str>) -> Output {
    run_on(server, args, key, b"Please refund my order.")
}

fn run_on(server: &Loopback, args: &[&str], key: Option<&str>, input: &[u8]) -> Output {
    let mut command = support::command();
    command
        .args(args)
        .env("THINKTHEN_BASE_URL", server.base())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(key) = key {
        command.env("THINKTHEN_API_KEY", key);
    }
    let mut child = command.spawn().unwrap();
    // A command that refuses its arguments may exit before it reads the text.
    let _ = child.stdin.take().unwrap().write_all(input);
    child.wait_with_output().unwrap()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[test]
fn the_answer_word_and_exit_code_follow_the_verdict() {
    let cases: [(f64, &[&str], &str, i32); 2] = [(0.9, &[], "true\n", 0), (0.1, &[], "false\n", 0)];
    for (p, extra, stdout, code) in cases {
        let server = Loopback::noul(p).unwrap();
        let mut args = vec!["decide", "Is it a refund request?"];
        args.extend_from_slice(extra);
        let output = run(&server, &args, Some(KEY));
        assert_eq!(text(&output.stdout), stdout, "p {p}");
        assert_eq!(output.status.code(), Some(code), "p {p}");
    }
}

#[test]
fn details_print_the_row_as_json() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(&server, &["decide", "Is it?", "--json"], Some(KEY));
    let row: serde_json::Value = {
        let mut row: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        row.as_object_mut().unwrap().remove("source");
        row
    };
    assert_eq!(
        row,
        json!({"function":"decide","position": 1, "status": "answered", "answer": true, "probability": 0.9})
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn the_url_option_beats_the_environment() {
    let server = Loopback::noul(0.9).unwrap();
    let mut command = support::command();
    let source = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(source.path(), "evidence").unwrap();
    let output = command
        .args(["decide", "Is it?", "--url", server.base()])
        .arg(source.path())
        .env("THINKTHEN_BASE_URL", "http://127.0.0.1:9/v1")
        .env("THINKTHEN_URL", "http://127.0.0.1:9/v1")
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(server.count(), 1);
}

#[test]
fn a_bad_threshold_exits_2_and_sends_nothing() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(
        &server,
        &["decide", "Is it?", "--threshold", "90"],
        Some(KEY),
    );
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(reported(&output), "usage.threshold");
    assert_eq!(server.count(), 0);
}

#[test]
fn the_model_option_names_the_model_in_the_request() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(&server, &["decide", "Is it?", "--model", "m-x"], Some(KEY));
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(server.requests()[0].json().unwrap()["model"], "m-x");
}

#[test]
fn input_that_is_not_utf8_returns_a_failed_record_and_sends_nothing() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run_on(
        &server,
        &["decide", "Is it?", "--json"],
        Some(KEY),
        b"caf\xe9",
    );
    assert_eq!(output.status.code(), Some(6));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap()["error"]["code"],
        "record.invalid"
    );
    assert_eq!(server.count(), 0);
}

#[test]
fn a_missing_key_exits_2_and_sends_nothing() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(&server, &["decide", "Is it?"], None);
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(reported(&output), "usage.key-missing");
    assert_eq!(server.count(), 0);
}

#[test]
fn a_backend_failure_exits_4_and_never_prints_the_key() {
    let server =
        Loopback::start(|_| Reply::json(401, &json!({"error": format!("bad key {KEY}")}))).unwrap();
    for args in [
        &["decide", "Is it?"][..],
        &["decide", "Is it?", "--json"][..],
    ] {
        let output = run(&server, args, Some(KEY));
        assert_eq!(output.status.code(), Some(4), "{args:?}");
        assert_eq!(reported(&output), "backend.status");
        assert!(output.stdout.is_empty());
        assert!(!text(&output.stdout).contains(KEY));
        assert!(!text(&output.stderr).contains(KEY));
    }
}

#[test]
fn empty_text_is_usage_and_never_calls_the_backend() {
    for input in [b"".as_slice(), b" \r\n\t", "\u{2003}\u{a0}".as_bytes()] {
        let server = Loopback::noul(0.9).unwrap();
        let output = run_on(&server, &["decide", "Ready?"], Some(KEY), input);
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
        assert_eq!(server.count(), 0);
    }
}

#[test]
fn usage_summary_is_separate_from_answers_and_quiet_suppresses_it() {
    let server = Loopback::start(|_| Reply::json(200, &json!({"answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":19,"output_tokens":0}}))).unwrap();
    let output = run(
        &server,
        &["decide", "Is it relevant?", "--usage"],
        Some(KEY),
    );
    assert!(output.status.success(), "{}", text(&output.stderr));
    assert_eq!(text(&output.stdout), "true\n");
    let summary: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(summary["kind"], "thinkthen.usage/1");
    assert_eq!(summary["status"], "completed");
    assert_eq!(summary["usage"]["input_tokens"]["reported"], 19);
    assert_eq!(summary["usage"]["active_calls"], 0);
    let quiet = run(
        &server,
        &["decide", "Is it relevant?", "--usage", "--quiet"],
        Some(KEY),
    );
    assert!(quiet.status.success());
    assert!(quiet.stdout.is_empty());
    assert!(quiet.stderr.is_empty());
}

#[test]
fn usage_reports_backend_failure_and_invalid_usage_sends_nothing() {
    let server = Loopback::start(|_| {
        Reply::json(401, &json!({"usage":{"input_tokens":3,"output_tokens":0}}))
    })
    .unwrap();
    let output = run(&server, &["decide", "Relevant?", "--usage"], Some(KEY));
    assert_eq!(output.status.code(), Some(4));
    let summary: serde_json::Value = text(&output.stderr)
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .find(|record| record["kind"] == "thinkthen.usage/1")
        .unwrap();
    assert_eq!(summary["status"], "failed");
    assert_eq!(summary["usage"]["input_tokens"]["reported"], 3);
    let invalid = run(
        &server,
        &["decide", "Relevant?", "--usage=maybe"],
        Some(KEY),
    );
    assert_eq!(invalid.status.code(), Some(2));
    assert_eq!(server.count(), 1);
}

#[test]
#[cfg(unix)]
fn first_interrupt_reports_settled_usage_before_exit() {
    use std::sync::{Mutex, mpsc};
    let (arrived, observed) = mpsc::channel();
    let (release, blocked) = mpsc::channel();
    let blocked = Mutex::new(blocked);
    let server = Loopback::start(move |_| {
        arrived.send(()).unwrap();
        blocked.lock().unwrap().recv().unwrap();
        Reply::json(200, &json!({}))
    })
    .unwrap();
    let mut child = support::command()
        .args(["decide", "Relevant?", "--usage"])
        .env("THINKTHEN_API_KEY", KEY)
        .env("THINKTHEN_BASE_URL", server.base())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"evidence").unwrap();
    observed.recv().unwrap();
    assert!(
        std::process::Command::new("kill")
            .args(["-INT", &child.id().to_string()])
            .status()
            .unwrap()
            .success()
    );
    let output = child.wait_with_output().unwrap();
    release.send(()).unwrap();
    assert_eq!(output.status.code(), Some(130));
    let summary: serde_json::Value = text(&output.stderr)
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .find(|record| record["kind"] == "thinkthen.usage/1")
        .unwrap();
    assert_eq!(summary["status"], "cancelled");
    assert_eq!(summary["usage"]["attempts"], 1);
    assert_eq!(summary["usage"]["active_calls"], 0);
    assert_eq!(summary["usage"]["active_attempts"], 0);
    assert_eq!(summary["usage"]["input_tokens"]["unknown_attempts"], 1);
}

#[test]
fn request_allowance_has_explicit_precedence_and_keeps_usage_on_exhaustion() {
    let server = Loopback::noul(0.9).unwrap();
    for (flag, code, attempts) in [(None, 5, 0), (Some("1"), 0, 1)] {
        let mut command = support::command();
        command
            .args(["decide", "Relevant?", "--usage"])
            .env("THINKTHEN_API_KEY", KEY)
            .env("THINKTHEN_BASE_URL", server.base())
            .env("THINKTHEN_MAX_REQUESTS", "0")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if let Some(flag) = flag {
            command.args(["--max-requests", flag]);
        }
        let mut child = command.spawn().unwrap();
        child.stdin.take().unwrap().write_all(b"evidence").unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(code), "{}", text(&output.stderr));
        let summary: serde_json::Value = text(&output.stderr)
            .lines()
            .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
            .find(|record| record["kind"] == "thinkthen.usage/1")
            .unwrap();
        assert_eq!(summary["usage"]["attempts"], attempts);
        assert_eq!(
            summary["status"],
            if code == 0 { "completed" } else { "failed" }
        );
        if code == 5 {
            assert!(text(&output.stderr).contains("local.request-limit"));
            assert!(output.stdout.is_empty());
        }
    }
    assert_eq!(server.count(), 1);
}
