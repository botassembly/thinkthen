//! Explicit backend profiles at the compiled command boundary.

#![allow(
    clippy::expect_used,
    reason = "a failed fixture setup should stop the boundary test"
)]

use std::fs;
use std::path::{Path, PathBuf};

use crate::harness::{Canned, Listener, spawn};
use crate::support::encoded_decide;

const ANSWER: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.92}},"#,
    r#""usage":{"input_tokens":3,"output_tokens":1}}"#,
);
const NO_ANSWER: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.08}},"#,
    r#""usage":{"input_tokens":3,"output_tokens":1}}"#,
);

mod secrecy;
mod structured;
mod warnings;

fn file(name: &str, text: &str) -> PathBuf {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("profiles");
    fs::create_dir_all(&folder).expect("profile folder");
    let path = folder.join(name);
    fs::write(&path, text).expect("profile file");
    path
}

fn profile(name: &str, body: &str) -> PathBuf {
    // Windows refuses quotes and colons in a file name.
    let limits: String = body
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();
    file(
        &format!("{name}-{limits}.json"),
        &format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"{name}",{body}}}"#),
    )
}

#[test]
fn evidence_and_exact_request_bytes_pass_at_the_edge_and_fail_one_past_it() {
    // A profile's evidence limit bounds each record and the request's state.
    // Under ADR 0111 a record travels quoted in its question, and the state
    // of a run with no context is the fixed 44-byte sentence.
    let exact_evidence = profile("edge-evidence", r#""max_evidence_bytes":44"#);
    let small_evidence = profile("edge-evidence", r#""max_evidence_bytes":43"#);
    let at_edge = "Please refund order 4417; the jug is broken.";
    let one_past = "Please refund order 44170; the jug is broken.";
    for (limit, record, refusal) in [
        (&exact_evidence, at_edge, None),
        (
            &exact_evidence,
            one_past,
            Some("44 evidence bytes; this request has 45"),
        ),
        (
            &small_evidence,
            "four",
            Some("43 evidence bytes; this request has 44"),
        ),
    ] {
        let output = spawn(
            &[
                "decide",
                "Is this relevant?",
                "--plan",
                "--profile",
                &limit.to_string_lossy(),
            ],
            &[],
            record.as_bytes(),
        )
        .expect("command");
        let Some(refusal) = refusal else {
            assert_eq!(output.status.code(), Some(0), "{record}");
            continue;
        };
        assert_eq!(output.status.code(), Some(2), "{record}");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            format!("thinkthen: profile edge-evidence allows at most {refusal}\n")
        );
        assert!(output.stdout.is_empty());
    }

    let body = encoded_decide("four", crate::support::DEFAULT_MODEL, "Is this relevant?");
    let exact_request = profile(
        "edge-request",
        &format!(r#""max_request_bytes":{}"#, body.len()),
    );
    let passed = spawn(
        &[
            "decide",
            "Is this relevant?",
            "--plan",
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
            "--plan",
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
        "thinkthen: warning: threshold tuned for profile old is running under profile new\n"
    );
    let row: serde_json::Value = serde_json::from_slice(&output.stdout).expect("result JSON");
    assert_eq!(
        row["meta"]["profile_warning"],
        serde_json::json!({"tuned_for":"old","running":"new"})
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
            "--plan",
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
    let _removed = fs::remove_dir_all(&recordings);
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let run = |store: &str, environment: &[(&str, &str)]| {
        spawn(
            &[
                "decide",
                &format!("@{}", question.to_string_lossy()),
                "--details",
                "--profile",
                &running.to_string_lossy(),
                store,
                &recordings.to_string_lossy(),
                "--url",
                listener.base(),
                "--model",
                "local-1",
            ],
            environment,
            b"four",
        )
        .expect("command")
    };
    let recorded = run("--record", &[("THINKTHEN_API_KEY", "secret-key")]);
    assert_eq!(
        recorded.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&recorded.stderr)
    );
    let sent = listener.requests();
    assert_eq!(sent.len(), 1);
    let output = run("--replay", &[]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(listener.count(), 1, "a replay sends nothing");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr)
            .matches("warning: threshold tuned for")
            .count(),
        1
    );
    let row: serde_json::Value = serde_json::from_slice(&output.stdout).expect("result");
    assert_eq!(row["meta"]["cached"], true);
    assert_eq!(
        row["meta"]["requests"],
        serde_json::json!(crate::support::keys(listener.url(), &sent[0].body))
    );
}

#[test]
fn over_limit_precedes_replay_and_cache_answers() {
    let tiny = profile("stored-tiny", r#""max_evidence_bytes":1"#);
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("profile-stored");
    let _removed = fs::remove_dir_all(&folder);
    let fixed = [
        "decide",
        "Is this relevant?",
        "--url",
        listener.base(),
        "--model",
        "local-1",
    ];
    let folder_name = folder.to_string_lossy();
    let stored = spawn(
        &[&fixed[..], &["--record", &folder_name]].concat(),
        &[("THINKTHEN_API_KEY", "secret-key")],
        b"four",
    )
    .expect("command");
    assert_eq!(
        stored.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&stored.stderr)
    );
    for option in ["--replay", "--cache"] {
        let profile = tiny.to_string_lossy();
        let output = spawn(
            &[&fixed[..], &[option, &folder_name, "--profile", &profile]].concat(),
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
