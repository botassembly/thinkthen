//! Explicit backend profiles at the compiled command boundary.

#![allow(
    clippy::expect_used,
    reason = "a failed fixture setup should stop the boundary test"
)]

use std::fs;
use std::path::PathBuf;

use crate::harness::{Canned, Listener, spawn};
use crate::support::{digest, encoded_decide, plant_recording};

const ANSWER: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.92}},"#,
    r#""usage":{"input_tokens":3,"output_tokens":1}}"#,
);
const NO_ANSWER: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.08}},"#,
    r#""usage":{"input_tokens":3,"output_tokens":1}}"#,
);

mod secrecy;
mod warnings;

fn file(name: &str, text: &str) -> PathBuf {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("profiles");
    fs::create_dir_all(&folder).expect("profile folder");
    let path = folder.join(name);
    fs::write(&path, text).expect("profile file");
    path
}

fn profile(name: &str, limits: &str) -> PathBuf {
    file(
        &format!("{name}-{limits}.json"),
        &format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"{name}",{limits}}}"#),
    )
}

#[test]
fn evidence_and_exact_request_bytes_pass_at_the_edge_and_fail_one_past_it() {
    let exact_evidence = profile("edge-evidence", r#""max_evidence_bytes":4"#);
    let passed = spawn(
        &[
            "decide",
            "Is this relevant?",
            "--dry-run",
            "--profile",
            &exact_evidence.to_string_lossy(),
        ],
        &[],
        b"four",
    )
    .expect("command");
    assert_eq!(passed.status.code(), Some(0));

    let failed = spawn(
        &[
            "decide",
            "Is this relevant?",
            "--dry-run",
            "--profile",
            &exact_evidence.to_string_lossy(),
        ],
        &[],
        b"five!",
    )
    .expect("command");
    assert_eq!(failed.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&failed.stderr),
        "thinkthen: profile edge-evidence allows at most 4 evidence bytes; this request has 5\n"
    );
    assert!(failed.stdout.is_empty());

    let body = encoded_decide("four", "jev-latest", "Is this relevant?");
    let exact_request = profile(
        "edge-request",
        &format!(r#""max_request_bytes":{}"#, body.len()),
    );
    let passed = spawn(
        &[
            "decide",
            "Is this relevant?",
            "--dry-run",
            "--profile",
            &exact_request.to_string_lossy(),
        ],
        &[],
        b"four",
    )
    .expect("command");
    assert_eq!(passed.status.code(), Some(0));
    let too_small = profile(
        "small-request",
        &format!(r#""max_request_bytes":{}"#, body.len() - 1),
    );
    let failed = spawn(
        &[
            "decide",
            "Is this relevant?",
            "--dry-run",
            "--profile",
            &too_small.to_string_lossy(),
        ],
        &[],
        b"four",
    )
    .expect("command");
    assert_eq!(failed.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&failed.stderr),
        format!(
            "thinkthen: profile small-request allows at most {} request bytes; this request has {}\n",
            body.len() - 1,
            body.len()
        )
    );
}

#[test]
fn expanded_tags_count_as_wire_questions_and_over_limit_sends_nothing() {
    let one = profile("one-question", r#""max_questions":1"#);
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let output = spawn(
        &[
            "tag",
            "Which topics?",
            "billing",
            "urgent",
            "--profile",
            &one.to_string_lossy(),
            "--url",
            listener.base(),
        ],
        &[("THINKTHEN_API_KEY", "secret-key")],
        b"private evidence",
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(listener.connections(), 0);
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: profile one-question allows at most 1 questions; this request has 2\n"
    );
    assert!(!String::from_utf8_lossy(&output.stderr).contains("private"));
    assert!(!String::from_utf8_lossy(&output.stderr).contains("secret-key"));
}

#[test]
fn every_record_framing_preflights_before_a_key_or_request() {
    let tiny = profile("tiny", r#""max_evidence_bytes":1"#);
    let cases: [(&[&str], &[u8]); 4] = [
        (&["--lines"], b"long\n"),
        (&["--jsonl", "--field", "/body"], b"{\"body\":\"long\"}\n"),
        (&["--csv", "--field", "/body"], b"body\nlong\n"),
        (&["--tsv", "--field", "/body"], b"body\nlong\n"),
    ];
    let tiny_name = tiny.to_string_lossy().into_owned();
    for (framing, input) in cases {
        let mut arguments = vec!["decide", "Is this relevant?"];
        arguments.extend_from_slice(framing);
        arguments.extend(["--profile", tiny_name.as_str()]);
        let output = spawn(&arguments, &[], input).expect("command");
        assert_eq!(output.status.code(), Some(2), "{framing:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("evidence bytes"),
            "{framing:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn find_preflights_its_complete_request_before_key_or_network() {
    let tiny = profile("tiny-find", r#""max_evidence_bytes":1"#);
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let output = spawn(
        &[
            "find",
            "Which unit answers?",
            "--profile",
            &tiny.to_string_lossy(),
            "--url",
            listener.base(),
        ],
        &[],
        b"first line\nsecond line\n",
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(listener.connections(), 0);
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(
        error.starts_with(
            "thinkthen: profile tiny-find allows at most 1 evidence bytes; this request has "
        ),
        "{error}"
    );
    assert!(output.stdout.is_empty());
}

#[test]
fn a_profile_mismatch_warns_once_and_reaches_detailed_metadata() {
    let question = file(
        "calibrated-question.json",
        r#"{"decide":"Is this relevant?","profile":"old"}"#,
    );
    let running = profile("new", r#""max_evidence_bytes":100"#);
    let listener = Listener::serving(vec![Canned::ok(ANSWER)]).expect("listener");
    let output = spawn(
        &[
            "decide",
            &format!("@{}", question.to_string_lossy()),
            "--details",
            "--profile",
            &running.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        b"four",
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: warning: threshold calibrated for profile old is running under profile new\n"
    );
    let row: serde_json::Value = serde_json::from_slice(&output.stdout).expect("result JSON");
    assert_eq!(
        row["meta"]["profile_warning"],
        serde_json::json!({"calibrated":"old","running":"new"})
    );
}

#[test]
fn grouped_annotate_checks_every_group_before_starting_one() {
    let set = file(
        "grouped.json",
        concat!(
            r#"{"version":1,"questions":{"first":{"decide":"First?","on":"/a"},"#,
            r#""second":{"decide":"Second?","on":"/b"}}}"#,
        ),
    );
    let tiny = profile("tiny-groups", r#""max_evidence_bytes":1"#);
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let output = spawn(
        &[
            "annotate",
            &set.to_string_lossy(),
            "--profile",
            &tiny.to_string_lossy(),
            "--url",
            listener.base(),
        ],
        &[("THINKTHEN_API_KEY", "key")],
        br#"{"a":"x","b":"long"}"#,
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(listener.connections(), 0);
    assert!(output.stdout.is_empty());

    let dry = spawn(
        &[
            "annotate",
            &set.to_string_lossy(),
            "--dry-run",
            "--profile",
            &tiny.to_string_lossy(),
        ],
        &[],
        br#"{"a":"x","b":"long"}"#,
    )
    .expect("dry command");
    assert_eq!(dry.status.code(), Some(2));
    assert!(dry.stdout.is_empty());
    assert!(String::from_utf8_lossy(&dry.stderr).contains("evidence bytes"));
}

#[test]
fn invalid_profile_text_is_a_safe_local_file_failure() {
    let path = file(
        "invalid.json",
        r#"{"name":"PRIVATE-MARKER","token":"secret"}"#,
    );
    let output = spawn(
        &[
            "decide",
            "Is this relevant?",
            "--profile",
            &path.to_string_lossy(),
        ],
        &[],
        b"EVIDENCE-MARKER",
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(5));
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("the profile file"));
    assert!(!error.contains("PRIVATE-MARKER"));
    assert!(!error.contains("EVIDENCE-MARKER"));
}

#[test]
fn replay_warns_once_without_a_key_and_keeps_request_identity() {
    let question = file(
        "replay-question.json",
        r#"{"decide":"Is this relevant?","profile":"old"}"#,
    );
    let running = profile("replay-new", r#""max_request_bytes":1000"#);
    let recordings = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("profile-replay");
    let body = encoded_decide("four", "local-1", "Is this relevant?");
    let url = "https://api.typesafe.ai/v1/systemone";
    plant_recording(&recordings, url, &body, ANSWER).expect("recording");
    let output = spawn(
        &[
            "decide",
            &format!("@{}", question.to_string_lossy()),
            "--details",
            "--profile",
            &running.to_string_lossy(),
            "--replay",
            &recordings.to_string_lossy(),
            "--model",
            "local-1",
        ],
        &[],
        b"four",
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr)
            .matches("warning: threshold calibrated")
            .count(),
        1
    );
    let row: serde_json::Value = serde_json::from_slice(&output.stdout).expect("result");
    assert_eq!(row["meta"]["cached"], true);
    assert_eq!(row["meta"]["requests"][0], digest(url, &body));
}

#[test]
fn over_limit_precedes_replay_and_cache_answers() {
    let tiny = profile("stored-tiny", r#""max_evidence_bytes":1"#);
    let body = encoded_decide("four", "local-1", "Is this relevant?");
    let url = "https://api.typesafe.ai/v1/systemone";
    for option in ["--replay", "--cache"] {
        let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
            .join(format!("profile-stored-{}", &option[2..]));
        plant_recording(&folder, url, &body, ANSWER).expect("stored answer");
        let output = spawn(
            &[
                "decide",
                "Is this relevant?",
                option,
                &folder.to_string_lossy(),
                "--profile",
                &tiny.to_string_lossy(),
                "--model",
                "local-1",
            ],
            &[],
            b"four",
        )
        .expect("command");
        assert_eq!(output.status.code(), Some(2), "{option}");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "thinkthen: profile stored-tiny allows at most 1 evidence bytes; this request has 4\n",
            "{option}"
        );
        assert!(output.stdout.is_empty(), "{option}");
    }
}
