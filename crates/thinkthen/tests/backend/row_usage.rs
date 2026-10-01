//! Ticket 0358: every row sums its usage by ADR 0111 section 7. A missing
//! share removes the usage, and otherwise a sum that does not fit fails the
//! row, whatever order the shares arrive in.

#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "a failed fixture setup or a missing field should stop the boundary test"
)]

use std::fs;
use std::path::PathBuf;
use std::process::Output;
use std::sync::atomic::{AtomicBool, Ordering};

use serde_json::{Value, json};

use crate::harness::{Canned, Listener, spawn};

const KEY: (&str, &str) = ("THINKTHEN_API_KEY", "sk-row-usage");
const OVERFLOW: &str = concat!(
    "thinkthen: the backend reported token counts whose total is too large\n",
    "thinkthen: usage counters could not be updated; ",
    "check the usage folder permissions and free space\n",
);

fn file(name: &str, text: &str) -> String {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("row-usage");
    fs::create_dir_all(&folder).expect("folder");
    let path = folder.join(name);
    let staged = folder.join(format!("{name}.{}", std::process::id()));
    fs::write(&staged, text).expect("staged file");
    fs::rename(&staged, &path).expect("file");
    path.to_string_lossy().into_owned()
}

fn one_question() -> String {
    file(
        "one-question.json",
        r#"{"schema":"thinkthen.backend-profile/1","name":"one-question","max_questions":1}"#,
    )
}

/// A yes to every question of `body`, with `usage` as given.
fn yes(body: &[u8], usage: Option<u64>) -> Canned {
    let request: Value = serde_json::from_slice(body).expect("request");
    let answers: serde_json::Map<String, Value> = request["questions"]
        .as_object()
        .expect("questions")
        .keys()
        .map(|name| (name.clone(), json!({"type":"noul","noul":0.9})))
        .collect();
    let mut reply = json!({"model":"local-1","answers":answers});
    if let Some(input) = usage {
        reply["usage"] = json!({"input_tokens":input,"output_tokens":1});
    }
    Canned::ok(&reply.to_string())
}

/// The first request reports the largest count and later ones 1.
fn overflowing() -> Listener {
    let first = AtomicBool::new(true);
    Listener::answering(move |body| {
        let input = if first.swap(false, Ordering::SeqCst) {
            u64::MAX
        } else {
            1
        };
        yes(body, Some(input))
    })
    .expect("listener")
}

fn run(listener: &Listener, arguments: &[&str], input: &[u8]) -> Output {
    let common = ["--url", listener.base(), "--model", "local-1", "--no-cache"];
    spawn(&[arguments, &common].concat(), &[KEY], input).expect("command")
}

/// The one detailed row of a finished run, which prints `stderr`.
fn row(output: &Output, stderr: &str) -> Value {
    assert_eq!(String::from_utf8_lossy(&output.stderr), stderr);
    assert_eq!(output.status.code(), Some(0));
    serde_json::from_slice(&output.stdout).expect("one detailed row")
}

fn assert_overflow(output: &Output, stop: &str) {
    let (cause, warning) = OVERFLOW.split_at(OVERFLOW.find('\n').expect("two lines") + 1);
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!("{cause}{stop}{warning}")
    );
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
}

/// `cli/asking/judged.rs` dropped the usage of a `tag` row whose labels
/// overflow together and printed the row.
#[test]
fn a_tag_row_whose_label_shares_overflow_fails() {
    let listener = overflowing();
    let profile = one_question();
    let output = run(
        &listener,
        &[
            "tag",
            "Which?",
            "billing",
            "urgent",
            "--lines",
            "--details",
            "--profile",
            &profile,
        ],
        b"one\n",
    );
    assert_overflow(
        &output,
        "thinkthen: stopped at record 1; 0 records finished\n",
    );
    assert_eq!(listener.count(), 2);
}

/// `annotate`'s question reader dropped the usage of a `tag` question whose
/// labels overflow together and printed the row.
#[test]
fn an_annotate_tag_question_whose_label_shares_overflow_fails() {
    let listener = overflowing();
    let set = file(
        "tag-set.json",
        r#"{"version":1,"questions":{"topics":{"tag":"Topics?","labels":["a","b"]}}}"#,
    );
    let profile = one_question();
    let output = run(
        &listener,
        &["annotate", &set, "--details", "--profile", &profile],
        br#"{"body":"one"}"#,
    );
    assert_overflow(&output, "");
    assert_eq!(listener.count(), 2);
}

/// `annotate`'s assembly failed on an overflow met before a later missing
/// share. The missing share now removes the usage in any order.
#[test]
fn a_missing_share_outranks_an_earlier_overflow() {
    let listener = Listener::answering(|body| {
        let text = String::from_utf8_lossy(body);
        let usage = if text.contains("First?") {
            Some(u64::MAX)
        } else if text.contains("Second?") {
            Some(1)
        } else {
            None
        };
        yes(body, usage)
    })
    .expect("listener");
    let set = file(
        "three-set.json",
        concat!(
            r#"{"version":1,"questions":{"one":{"decide":"First?"},"#,
            r#""two":{"decide":"Second?"},"three":{"decide":"Third?"}}}"#,
        ),
    );
    let profile = one_question();
    let output = run(
        &listener,
        &["annotate", &set, "--details", "--profile", &profile],
        br#"{"body":"one"}"#,
    );
    // The process totals still overflow, so the usage file is not updated.
    let row = row(&output, OVERFLOW.split_once('\n').expect("two lines").1);
    assert!(row["meta"].get("usage").is_none(), "{row}");
    assert_eq!(row["meta"]["requests_sent"], 3);
    assert_eq!(listener.count(), 3);
}

/// `relate` kept the shares present when one reply reported no usage.
#[test]
fn a_relate_row_with_one_reply_lacking_usage_has_no_usage() {
    let first = AtomicBool::new(true);
    let listener = Listener::answering(move |body| {
        let usage = first.swap(false, Ordering::SeqCst).then_some(10);
        yes(body, usage)
    })
    .expect("listener");
    let profile = one_question();
    let output = run(
        &listener,
        &[
            "relate",
            "follows=person:person",
            "--details",
            "--profile",
            &profile,
        ],
        br#"[{"name":"Ada","kind":"person"},{"name":"Grace","kind":"person"}]"#,
    );
    let row = row(&output, "");
    assert!(row["meta"].get("usage").is_none(), "{row}");
    assert_eq!(listener.count(), 2);
}

/// `recognize` kept the step-1 counts when the kind step reported none.
#[test]
fn a_recognize_row_with_one_step_lacking_usage_has_no_usage() {
    let listener = Listener::answering(|body| {
        let kinds = String::from_utf8_lossy(body).contains("none of these");
        super::recognize::reported(body, !kinds)
    })
    .expect("listener");
    let output = run(
        &listener,
        &["recognize", "person", "organization", "--details"],
        b"Ada met Acme.",
    );
    let row = row(&output, "");
    assert!(row["meta"].get("usage").is_none(), "{row}");
    assert_eq!(listener.count(), 2);
}
