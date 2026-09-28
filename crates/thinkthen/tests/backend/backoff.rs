//! A few compiled-command requests pin retry counts and body reuse.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{fs, path::Path};

use crate::harness::{Canned, Listener, spawn, spawn_one};

const ANSWER: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"usage":{"input_tokens":1,"output_tokens":1}}"#;

fn run(max_retries: Option<&str>, busy: usize) -> (std::process::Output, Listener) {
    let seen = Arc::new(AtomicUsize::new(0));
    let counted = Arc::clone(&seen);
    let listener = Listener::answering(move |_| {
        if counted.fetch_add(1, Ordering::SeqCst) < busy {
            Canned::status(503, "busy").asking("retry-after-ms", "1")
        } else {
            Canned::ok(ANSWER)
        }
    })
    .expect("loopback listener");
    let mut arguments = vec!["decide", "Is it?", "--url", listener.base(), "--no-cache"];
    if let Some(max_retries) = max_retries {
        arguments.extend(["--max-retries", max_retries]);
    }
    let output = spawn_one(&arguments, &[("THINKTHEN_API_KEY", "sk-test")], b"evidence")
        .expect("compiled command");
    (output, listener)
}

#[test]
fn default_and_explicit_retry_counts_are_per_request() {
    for (setting, busy, expected_sends, code) in
        [(None, 3, 4, 0), (Some("2"), 3, 3, 4), (Some("0"), 1, 1, 4)]
    {
        let (output, listener) = run(setting, busy);
        assert_eq!(output.status.code(), Some(code), "{setting:?}");
        assert_eq!(listener.count(), expected_sends, "{setting:?}");
        if code == 0 {
            assert_eq!(output.stdout, b"true\n");
        } else {
            assert!(output.stdout.is_empty());
            assert!(String::from_utf8_lossy(&output.stderr).contains("503"));
        }
    }
}

#[test]
fn a_retried_error_body_is_drained_and_the_connection_is_reused() {
    let (output, listener) = run(Some("1"), 1);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(listener.count(), 2);
    assert_eq!(listener.connections(), 1);
}

#[test]
fn a_line_break_in_the_key_fails_locally_without_a_send() {
    let listener = Listener::serving(vec![]).expect("loopback listener");
    let output = spawn(
        &["decide", "Is it?", "--url", listener.base(), "--no-cache"],
        &[("THINKTHEN_API_KEY", "first\nsecond")],
        b"evidence",
    )
    .expect("compiled command");
    assert_eq!(output.status.code(), Some(2));
    assert!(listener.requests().is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("API key contains a line break"), "{stderr}");
    assert!(!stderr.contains("first"));
    assert!(!stderr.contains("second"));
}

#[test]
fn status_counts_retries_as_a_subset_of_actual_sends() {
    let home = Path::new(env!("CARGO_TARGET_TMPDIR")).join("backoff-status");
    let _old = fs::remove_dir_all(&home);
    fs::create_dir_all(&home).expect("private test folder");
    let cache_home = home.to_str().expect("UTF-8 test folder");
    let listener = Listener::serving(vec![
        Canned::status(503, "busy").asking("retry-after-ms", "1"),
        Canned::ok(ANSWER),
    ])
    .expect("loopback listener");
    let run = spawn_one(
        &["decide", "Is it?", "--url", listener.base(), "--no-cache"],
        &[
            ("THINKTHEN_API_KEY", "sk-test"),
            ("XDG_CACHE_HOME", cache_home),
        ],
        b"evidence",
    )
    .expect("compiled command");
    assert_eq!(run.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 2);
    let status = spawn(
        &["status", "--json"],
        &[("XDG_CACHE_HOME", cache_home)],
        b"",
    )
    .expect("status command");
    assert_eq!(status.status.code(), Some(0));
    let status: serde_json::Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    for field in ["this_month", "total"] {
        assert_eq!(status["usage"][field]["requests_sent"], 2);
        assert_eq!(status["usage"][field]["retries"], 1);
    }
}
