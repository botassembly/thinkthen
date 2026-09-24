//! Timeout validation at the compiled process boundary.

use std::path::Path;

use crate::harness::{Canned, Listener, spawn};

const KEY: &str = "test-key";
const MESSAGE: &str = "thinkthen: --timeout takes a whole number of seconds greater than zero\n";
const ANSWER: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;

#[test]
fn zero_timeout_precedes_input_key_and_connection_in_both_argument_homes() {
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("a loopback listener");
    let absent = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("timeout")
        .join("absent-input");
    let absent = absent.to_string_lossy();

    let cases = [
        vec![
            "decide",
            "Does this pass?",
            "--timeout",
            "0",
            "--input",
            absent.as_ref(),
            "--url",
            listener.base(),
        ],
        vec![
            "find",
            "Which unit answers?",
            "--timeout",
            "0",
            "--input",
            absent.as_ref(),
            "--url",
            listener.base(),
        ],
    ];

    for arguments in cases {
        let output = spawn(&arguments, &[], b"").expect("the compiled binary runs");
        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
        assert_eq!(String::from_utf8_lossy(&output.stderr), MESSAGE);
    }

    assert!(
        listener.requests().is_empty(),
        "zero timeout sent a request"
    );
    assert_eq!(
        listener.connections(),
        0,
        "zero timeout opened a connection"
    );
}

#[test]
fn a_one_document_run_refuses_jobs_and_sends_nothing() {
    for dry_run in [true, false] {
        let listener = Listener::serving(vec![Canned::ok(ANSWER)]).expect("a loopback listener");
        let mut arguments = vec![
            "decide",
            "Does this pass?",
            "--jobs",
            "1",
            "--url",
            listener.base(),
        ];
        if dry_run {
            arguments.push("--dry-run");
        }
        let output = spawn(&arguments, &[], b"yes").expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "dry run {dry_run}");
        assert!(output.stdout.is_empty(), "dry run {dry_run}");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "thinkthen: --jobs bounds the requests in flight, and a single text sends one request\n"
        );
        assert_eq!(listener.connections(), 0, "dry run {dry_run}");
        assert!(listener.requests().is_empty(), "dry run {dry_run}");
    }
}

#[test]
fn a_positive_timeout_still_reaches_the_backend() {
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("a loopback listener");
    let output = spawn(
        &[
            "decide",
            "Does this pass?",
            "--timeout",
            "1",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", KEY)],
        b"yes",
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "true\n");
    assert!(output.stderr.is_empty());
    assert_eq!(listener.requests().len(), 1);
}
