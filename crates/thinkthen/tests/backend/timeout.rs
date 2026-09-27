//! Timeout validation at the compiled process boundary.

use std::path::Path;

use crate::harness::{Canned, Listener, spawn};

const KEY: &str = "test-key";
const MESSAGE: &str = "thinkthen: --timeout takes a whole number of seconds from 1 to 86400\n";
const ANSWER: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#;

#[test]
fn a_timeout_outside_one_to_a_day_precedes_input_key_and_connection_in_both_argument_homes() {
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("a loopback listener");
    let absent = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("timeout")
        .join("absent-input");
    let absent = absent.to_string_lossy();

    // 9223372036854775807 panicked with exit 101 in the HTTP client's clock
    // arithmetic before the bound.
    for timeout in ["0", "86401", "9223372036854775807", "18446744073709551615"] {
        for command in [
            ["decide", "Does this pass?"],
            ["find", "Which unit answers?"],
        ] {
            let mut arguments = command.to_vec();
            arguments.extend([
                "--timeout",
                timeout,
                "--input",
                absent.as_ref(),
                "--url",
                listener.base(),
            ]);
            let output = spawn(&arguments, &[], b"").expect("the compiled binary runs");
            assert_eq!(output.status.code(), Some(2), "{arguments:?}");
            assert!(output.stdout.is_empty(), "{arguments:?}");
            assert_eq!(String::from_utf8_lossy(&output.stderr), MESSAGE);
        }
    }

    assert!(
        listener.requests().is_empty(),
        "a refused timeout sent a request"
    );
    assert_eq!(
        listener.connections(),
        0,
        "a refused timeout opened a connection"
    );

    let day = spawn(
        &[
            "decide",
            "Does this pass?",
            "--timeout",
            "86400",
            "--dry-run",
        ],
        &[],
        b"yes",
    )
    .expect("the compiled binary runs");
    assert_eq!(day.status.code(), Some(0), "a day is the largest timeout");
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
