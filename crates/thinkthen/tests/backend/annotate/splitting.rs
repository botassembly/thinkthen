//! Backend-profile splitting at the compiled `annotate` boundary.

#![allow(
    clippy::expect_used,
    reason = "a failed fixture setup should stop the boundary test"
)]

use std::fs;
use std::path::{Path, PathBuf};

use super::set;
use crate::harness::{Canned, Listener, spawn};
use crate::support::{digest, plant_backend_identity, plant_recording};

const FIRST_TWO: &str = concat!(
    r#"{"state":"evidence","model":"local-1","questions":{"q1":{"type":"noul","instructions":"first?"},"#,
    r#""q2":{"type":"noul","instructions":"second?"}}}"#,
);
const THIRD: &str = r#"{"state":"evidence","model":"local-1","questions":{"q1":{"type":"noul","instructions":"third?"}}}"#;
const FIRST_TWO_REPLY: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"#,
    r#""q2":{"type":"noul","noul":0.1}},"#,
    r#""usage":{"input_tokens":3,"output_tokens":1}}"#,
);
const THIRD_REPLY: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.8}},"#,
    r#""usage":{"input_tokens":5,"output_tokens":2}}"#,
);

fn profile(name: &str, limits: &str) -> PathBuf {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("annotate-splitting");
    fs::create_dir_all(&folder).expect("profile folder");
    let path = folder.join(format!("{name}.json"));
    fs::write(
        &path,
        format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"{name}",{limits}}}"#),
    )
    .expect("profile");
    path
}

fn three_questions(name: &str) -> PathBuf {
    set(
        name,
        concat!(
            r#"{"version":1,"questions":{"first":{"decide":"first?"},"#,
            r#""second":{"decide":"second?"},"third":{"decide":"third?"}}}"#,
        ),
    )
}

fn run(
    set: &Path,
    profile: &Path,
    listener: &Listener,
    extra: &[&str],
    environment: &[(&str, &str)],
) -> std::process::Output {
    let mut arguments = vec![
        "annotate",
        set.to_str().expect("set path"),
        "--profile",
        profile.to_str().expect("profile path"),
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--details",
    ];
    arguments.extend_from_slice(extra);
    spawn(&arguments, environment, b"evidence").expect("annotate")
}

#[test]
fn exact_limits_keep_the_historical_body_and_one_unit_over_splits_longest_prefix() {
    let questions = three_questions("exact-and-split");
    let complete = concat!(
        r#"{"state":"evidence","model":"local-1","questions":{"q1":{"type":"noul","instructions":"first?"},"#,
        r#""q2":{"type":"noul","instructions":"second?"},"q3":{"type":"noul","instructions":"third?"}}}"#,
    );
    let complete_reply = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"#,
        r#""q2":{"type":"noul","noul":0.1},"q3":{"type":"noul","noul":0.8}}}"#,
    );
    let exact = profile(
        "exact-body",
        &format!(r#""max_request_bytes":{}"#, complete.len()),
    );
    let exact_listener =
        Listener::serving(vec![Canned::ok(complete_reply)]).expect("exact listener");
    let exact_output = run(
        &questions,
        &exact,
        &exact_listener,
        &[],
        &[("THINKTHEN_API_KEY", "key")],
    );
    assert_eq!(exact_output.status.code(), Some(0));
    let exact_requests = exact_listener.requests();
    assert_eq!(exact_requests.len(), 1);
    assert_eq!(exact_requests[0].body, complete.as_bytes());
    let exact_row: serde_json::Value =
        serde_json::from_slice(&exact_output.stdout).expect("exact result");
    assert_eq!(
        exact_row["meta"]["requests"],
        serde_json::json!([digest(exact_listener.url(), complete.as_bytes())])
    );

    let split = profile(
        "split-body",
        &format!(r#""max_request_bytes":{}"#, complete.len() - 1),
    );
    let split_listener =
        Listener::serving(vec![Canned::ok(FIRST_TWO_REPLY), Canned::ok(THIRD_REPLY)])
            .expect("split listener");
    let split_output = run(
        &questions,
        &split,
        &split_listener,
        &[],
        &[("THINKTHEN_API_KEY", "key")],
    );
    assert_eq!(
        split_output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&split_output.stderr)
    );
    let requests = split_listener.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].body, FIRST_TWO.as_bytes());
    assert_eq!(requests[1].body, THIRD.as_bytes());
    let row: serde_json::Value = serde_json::from_slice(&split_output.stdout).expect("result");
    assert_eq!(
        row["value"],
        serde_json::json!({"first":true,"second":false,"third":true})
    );
    assert_eq!(
        row["meta"]["usage"],
        serde_json::json!({"input_tokens":8,"output_tokens":3})
    );
    assert_eq!(row["meta"]["requests_sent"], 2);
    assert_eq!(row["meta"]["cached"], false);
    assert_eq!(
        row["meta"]["requests"],
        serde_json::json!([
            digest(split_listener.url(), FIRST_TWO.as_bytes()),
            digest(split_listener.url(), THIRD.as_bytes())
        ])
    );
}

#[test]
fn question_and_option_limits_split_or_refuse_before_any_send() {
    let questions = three_questions("question-limit");
    let two = profile("two-questions", r#""max_questions":2"#);
    let listener = Listener::serving(vec![Canned::ok(FIRST_TWO_REPLY), Canned::ok(THIRD_REPLY)])
        .expect("listener");
    let output = run(
        &questions,
        &two,
        &listener,
        &[],
        &[("THINKTHEN_API_KEY", "key")],
    );
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 2);

    let exact_choice = set(
        "option-exact",
        r#"{"version":1,"questions":{"first":{"decide":"first?"},"pick":{"choose":"pick?","options":["a","b"]}}}"#,
    );
    let exact_reply = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"#,
        r#""q2":{"type":"choice","probabilities":{"a":0.8,"b":0.2}}}}"#,
    );
    let options = profile("two-options", r#""max_options":2"#);
    let accepted = Listener::serving(vec![Canned::ok(exact_reply)]).expect("accepted listener");
    let output = run(
        &exact_choice,
        &options,
        &accepted,
        &[],
        &[("THINKTHEN_API_KEY", "key")],
    );
    assert_eq!(output.status.code(), Some(0));
    let requests = accepted.requests();
    assert_eq!(requests.len(), 1);
    assert!(
        String::from_utf8_lossy(&requests[0].body).contains(r#""criteria":{"a":null,"b":null}"#)
    );

    let overflow_choice = set(
        "option-overflow",
        r#"{"version":1,"questions":{"first":{"decide":"first?"},"pick":{"choose":"pick?","options":["a","b","c"]}}}"#,
    );
    let refused = Listener::serving(Vec::new()).expect("refused listener");
    let output = run(&overflow_choice, &options, &refused, &[], &[]);
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(refused.connections(), 0);
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: profile two-options allows at most 2 options; this request has 3\n"
    );
}

#[test]
fn every_chunk_of_every_group_is_preflighted_before_the_first_send() {
    let questions = set(
        "later-impossible-group",
        concat!(
            r#"{"version":1,"questions":{"first":{"decide":"first?","on":"/left"},"#,
            r#""second":{"decide":"second?","on":"/left"},"#,
            r#""impossible":{"tag":"topics?","labels":["a","b","c"],"on":"/right"}}}"#,
        ),
    );
    let limit = profile("two-wire-questions", r#""max_questions":2"#);
    let listener = Listener::serving(Vec::new()).expect("listener");
    let mut arguments = vec![
        "annotate",
        questions.to_str().expect("questions"),
        "--profile",
        limit.to_str().expect("profile"),
        "--url",
        listener.base(),
        "--model",
        "local-1",
    ];
    arguments.push("--no-cache");
    let output = spawn(
        &arguments,
        &[("THINKTHEN_API_KEY", "key")],
        br#"{"left":"evidence","right":"evidence"}"#,
    )
    .expect("annotate");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(listener.connections(), 0);
}

#[test]
fn mixed_cache_and_retry_accounting_keeps_logical_order() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("split-mixed-cache");
    let _absent = fs::remove_dir_all(&root);
    let cache = root.join("answers");
    let questions = three_questions("mixed-cache");
    let two = profile("mixed-two", r#""max_questions":2"#);
    let listener = Listener::serving(vec![Canned::status(500, "{}"), Canned::ok(THIRD_REPLY)])
        .expect("listener");
    plant_recording(
        &cache,
        listener.url(),
        FIRST_TWO.as_bytes(),
        FIRST_TWO_REPLY,
    )
    .expect("first chunk cached");
    plant_backend_identity(&cache, listener.url()).expect("cache binding");
    let root_name = root.to_string_lossy().into_owned();
    let output = run(
        &questions,
        &two,
        &listener,
        &["--cache", cache.to_str().expect("cache")],
        &[
            ("THINKTHEN_API_KEY", "key"),
            ("THINKTHEN_TEST_RETRY_WAIT_MS", "1"),
            ("XDG_CACHE_HOME", root_name.as_str()),
        ],
    );
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let row: serde_json::Value = serde_json::from_slice(&output.stdout).expect("result");
    assert_eq!(row["meta"]["requests_sent"], 2);
    assert_eq!(row["meta"]["cached"], false);
    assert_eq!(listener.requests().len(), 2);

    let replayed = run(
        &questions,
        &two,
        &listener,
        &["--cache", cache.to_str().expect("cache")],
        &[("XDG_CACHE_HOME", root_name.as_str())],
    );
    assert_eq!(replayed.status.code(), Some(0));
    let replayed: serde_json::Value =
        serde_json::from_slice(&replayed.stdout).expect("replayed result");
    assert_eq!(replayed["meta"]["requests_sent"], 0);
    assert_eq!(replayed["meta"]["cached"], true);
    assert!(listener.requests().is_empty());

    let status = spawn(
        &["status", "--json"],
        &[("XDG_CACHE_HOME", root_name.as_str())],
        b"",
    )
    .expect("status");
    let status: serde_json::Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(status["usage"]["this_month"]["requests_sent"], 2);
    assert_eq!(status["usage"]["this_month"]["cache_answers"], 3);
}

#[test]
fn partial_failures_keep_their_chunk_digest_and_missing_usage_removes_the_total() {
    let questions = three_questions("split-partial");
    let two = profile("partial-two", r#""max_questions":2"#);
    let partial = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"#,
        r#""q2":{"type":"choice","probabilities":{"x":1.0}}},"#,
        r#""usage":{"input_tokens":3,"output_tokens":1}}"#,
    );
    let no_usage = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.8}}}"#;
    let listener =
        Listener::serving(vec![Canned::ok(partial), Canned::ok(no_usage)]).expect("listener");
    let output = run(
        &questions,
        &two,
        &listener,
        &[],
        &[("THINKTHEN_API_KEY", "key")],
    );
    assert_eq!(output.status.code(), Some(6));
    let row: serde_json::Value = serde_json::from_slice(&output.stdout).expect("result");
    assert!(row["meta"].get("usage").is_none());
    assert_eq!(row["meta"]["failed_questions"], 1);
    assert_eq!(row["value"]["first"], true);
    assert_eq!(
        row["value"]["second"],
        serde_json::json!({"failed":{"kind":"backend","cause":"wrong_kind"}})
    );
    assert_eq!(row["value"]["third"], true);
    let first_digest = digest(listener.url(), FIRST_TWO.as_bytes());
    let third_digest = digest(listener.url(), THIRD.as_bytes());
    assert_eq!(row["answers"]["first"]["request"], first_digest);
    assert_eq!(row["answers"]["second"]["request"], first_digest);
    assert_eq!(row["answers"]["third"]["request"], third_digest);
}

#[test]
fn different_models_across_chunks_keep_the_safe_failure() {
    let questions = three_questions("model-mismatch");
    let two = profile("model-two", r#""max_questions":2"#);
    let other = THIRD_REPLY.replace("local-1", "other-model");
    let listener =
        Listener::serving(vec![Canned::ok(FIRST_TWO_REPLY), Canned::ok(&other)]).expect("listener");
    let output = run(
        &questions,
        &two,
        &listener,
        &[],
        &[("THINKTHEN_API_KEY", "key")],
    );
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the backend returned different model versions for one record; pin --model and rerun with --record or --cache\n"
    );
}
