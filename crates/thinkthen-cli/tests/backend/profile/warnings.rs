//! Warning timing at the ordered command-output boundary.

use super::*;
use crate::harness::{spawn_one_local, stderr_is};

fn records(
    listener: &Listener,
    verb: &str,
    question: &str,
    running: &Path,
    jobs: &str,
    evidence: &[u8],
) -> std::io::Result<std::process::Output> {
    spawn_one_local(
        &[
            verb,
            question,
            "--lines",
            "--jobs",
            jobs,
            "--profile",
            &running.to_string_lossy(),
        ],
        listener.base(),
        &[("THINKTHEN_API_KEY", "key")],
        evidence,
    )
}

#[test]
fn a_parallel_record_run_prints_one_profile_warning() {
    let question = calibrated("parallel-question.json");
    let running = profile("parallel-new", r#""max_evidence_bytes":100"#);
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let output = records(
        &listener,
        "decide",
        &format!("@{}", question.to_string_lossy()),
        &running,
        "4",
        b"one\ntwo\nthree\nfour\n",
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(crate::harness::printed(&output).lines().count(), 4);
    assert_eq!(
        crate::harness::said(&output)
            .matches("warning: threshold tuned for")
            .count(),
        1
    );
}

#[test]
fn a_mismatched_filter_warns_once_when_every_successful_row_is_filtered_out() {
    let question = calibrated("filtered-question.json");
    let running = profile("filtered-new", r#""max_evidence_bytes":100"#);
    let listener = Listener::answering(|_| Canned::ok(NO_ANSWER)).expect("listener");
    let output = records(
        &listener,
        "filter",
        &format!("@{}", question.to_string_lossy()),
        &running,
        "2",
        b"one\ntwo\n",
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty());
    assert_eq!(
        crate::harness::said(&output)
            .matches("warning: threshold tuned for")
            .count(),
        1
    );
}

#[test]
fn a_filtered_first_row_warns_once_before_a_later_row_is_printed() {
    let question = calibrated("filtered-then-kept-question.json");
    let running = profile("filtered-then-kept-new", r#""max_evidence_bytes":100"#);
    let listener =
        Listener::serving(vec![Canned::ok(NO_ANSWER), Canned::ok(ANSWER)]).expect("listener");
    let output = records(
        &listener,
        "filter",
        &format!("@{}", question.to_string_lossy()),
        &running,
        "1",
        b"one\ntwo\n",
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(crate::harness::printed(&output), "two\n");
    assert_eq!(
        crate::harness::said(&output)
            .matches("warning: threshold tuned for")
            .count(),
        1
    );
}

#[test]
fn an_over_limit_first_record_suppresses_a_later_parallel_warning_and_output() {
    let question = calibrated("parallel-stop-question.json");
    let running = profile("parallel-stop-new", r#""max_evidence_bytes":1"#);
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let mut input = vec![b'x'; 4 * 1024 * 1024];
    input.extend_from_slice(b"\ny\n");
    let output = records(
        &listener,
        "decide",
        &format!("@{}", question.to_string_lossy()),
        &running,
        "2",
        &input,
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        listener.connections(),
        0,
        "the refused first record sent nothing after it"
    );
    assert!(output.stdout.is_empty());
    let error = crate::harness::said(&output);
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
    let output = records(
        &listener,
        "annotate",
        &set.to_string_lossy(),
        &running,
        "2",
        &input,
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(listener.connections(), 0, "the later row never starts");
    assert!(output.stdout.is_empty());
    let error = crate::harness::said(&output);
    assert!(error.contains("stopped at record 1"), "{error}");
    assert!(!error.contains("warning:"), "{error}");
}

#[test]
fn command_and_public_details_share_the_saved_calibration_digest() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("../../../../../conformance/calibration.json"))
            .expect("shared calibration fixture");
    let question = file(
        "shared-calibration-question.json",
        &fixture["question"].to_string(),
    );
    let running = file(
        "shared-calibration-profile.json",
        &fixture["runtime_profile"].to_string(),
    );
    let listener = Listener::serving(vec![Canned::ok(
        r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}}}"#,
    )])
    .expect("listener");
    let output = spawn_one_local(
        &[
            "decide",
            &format!("@{}", question.display()),
            "--details",
            "--profile",
            &running.to_string_lossy(),
        ],
        listener.base(),
        &[("THINKTHEN_API_KEY", "key")],
        fixture["evidence"].as_str().expect("evidence").as_bytes(),
    )
    .expect("command");
    crate::harness::exits(&output, 0);
    stderr_is(
        &output,
        "thinkthen: warning: threshold tuned for profile old is running under profile new\n",
    );
    let row: serde_json::Value = serde_json::from_slice(&output.stdout).expect("result JSON");
    assert_eq!(row["meta"]["question_sha256"], fixture["question_sha256"]);
    assert_eq!(row["meta"]["profile_warning"], fixture["warning"]);
    assert_eq!(row["meta"]["model"], fixture["model"]);
    assert_eq!(listener.connections(), 1);
}
