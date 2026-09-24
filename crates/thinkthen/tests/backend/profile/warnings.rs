//! Warning timing at the ordered command-output boundary.

use super::*;

#[test]
fn a_parallel_record_run_prints_one_profile_warning() {
    let question = file(
        "parallel-question.json",
        r#"{"decide":"Is this relevant?","profile":"old"}"#,
    );
    let running = profile("parallel-new", r#""max_evidence_bytes":100"#);
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let output = spawn(
        &[
            "decide",
            &format!("@{}", question.to_string_lossy()),
            "--lines",
            "--jobs",
            "4",
            "--profile",
            &running.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        b"one\ntwo\nthree\nfour\n",
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout).lines().count(), 4);
    assert_eq!(
        String::from_utf8_lossy(&output.stderr)
            .matches("warning: threshold tuned for")
            .count(),
        1
    );
}

#[test]
fn a_mismatched_filter_warns_once_when_every_successful_row_is_filtered_out() {
    let question = file(
        "filtered-question.json",
        r#"{"decide":"Is this relevant?","profile":"old"}"#,
    );
    let running = profile("filtered-new", r#""max_evidence_bytes":100"#);
    let listener = Listener::answering(|_| Canned::ok(NO_ANSWER)).expect("listener");
    let output = spawn(
        &[
            "filter",
            &format!("@{}", question.to_string_lossy()),
            "--lines",
            "--jobs",
            "2",
            "--profile",
            &running.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        b"one\ntwo\n",
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr)
            .matches("warning: threshold tuned for")
            .count(),
        1
    );
}

#[test]
fn a_filtered_first_row_warns_once_before_a_later_row_is_printed() {
    let question = file(
        "filtered-then-kept-question.json",
        r#"{"decide":"Is this relevant?","profile":"old"}"#,
    );
    let running = profile("filtered-then-kept-new", r#""max_evidence_bytes":100"#);
    let listener =
        Listener::serving(vec![Canned::ok(NO_ANSWER), Canned::ok(ANSWER)]).expect("listener");
    let output = spawn(
        &[
            "filter",
            &format!("@{}", question.to_string_lossy()),
            "--lines",
            "--jobs",
            "1",
            "--profile",
            &running.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        b"one\ntwo\n",
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "two\n");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr)
            .matches("warning: threshold tuned for")
            .count(),
        1
    );
}

#[test]
fn an_over_limit_first_record_suppresses_a_later_parallel_warning_and_output() {
    let question = file(
        "parallel-stop-question.json",
        r#"{"decide":"Is this relevant?","profile":"old"}"#,
    );
    let running = profile("parallel-stop-new", r#""max_evidence_bytes":1"#);
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let mut input = vec![b'x'; 4 * 1024 * 1024];
    input.extend_from_slice(b"\ny\n");
    let output = spawn(
        &[
            "decide",
            &format!("@{}", question.to_string_lossy()),
            "--lines",
            "--jobs",
            "2",
            "--profile",
            &running.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        &input,
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(listener.connections(), 1, "the passing later row completed");
    assert!(output.stdout.is_empty());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("stopped at record 1"), "{error}");
    assert!(!error.contains("warning:"), "{error}");
}

#[test]
fn annotate_suppresses_the_warning_and_never_starts_after_first_preflight_refusal() {
    let set = file(
        "parallel-stop-set.json",
        r#"{"version":1,"profile":"old","questions":{"ready":{"decide":"Ready?"}}}"#,
    );
    let running = profile("parallel-stop-annotate", r#""max_evidence_bytes":1"#);
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let mut input = vec![b'x'; 4 * 1024 * 1024];
    input.extend_from_slice(b"\ny\n");
    let output = spawn(
        &[
            "annotate",
            &set.to_string_lossy(),
            "--lines",
            "--jobs",
            "2",
            "--profile",
            &running.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        &input,
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(listener.connections(), 0, "the later row never starts");
    assert!(output.stdout.is_empty());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("stopped at record 1"), "{error}");
    assert!(!error.contains("warning:"), "{error}");
}
