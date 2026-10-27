#![cfg(test)]
//! Choice command arguments, rendering, and exit codes.
#![cfg(feature = "command")]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

mod support;

use loopback::{KEY, Loopback};
use serde_json::json;
use std::process::{Output, Stdio};
use support::reported;

fn run(server: &Loopback, extra: &[&str]) -> Output {
    support::command()
        .args(["choose", "Which?", "--choice", "z", "--choice", "a"])
        .args(extra)
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::null())
        .output()
        .unwrap()
}

#[test]
fn choose_prints_the_label_or_unsure_with_the_promised_exit_code() {
    for (p, extra, expected, code) in [
        (0.8, vec![], "z\n", 0),
        (0.8, vec!["--check"], "z\n", 0),
        (0.6, vec!["--threshold", "0.8"], "unsure\n", 0),
        (0.6, vec!["--threshold", "0.8", "--check"], "unsure\n", 3),
    ] {
        let server = Loopback::answering(
            move |_| json!({"type":"choice", "probabilities":{"z":p,"a":1.0-p}}),
        )
        .unwrap();
        let output = run(&server, &extra);
        assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
        assert_eq!(output.status.code(), Some(code));
    }
}

#[test]
fn details_print_the_chosen_label_and_its_probability() {
    let server =
        Loopback::answering(|_| json!({"type":"choice", "probabilities":{"z":0.8,"a":0.2}}))
            .unwrap();
    let output = run(&server, &["--json"]);
    assert_eq!(
        {
            let mut row: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            row.as_object_mut().unwrap().remove("source");
            row
        },
        json!({"function":"choose","position":1,"status":"answered","answer":"z","probability":0.8,"detail":{"labels":{"z":0.8,"a":0.2}}})
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn duplicate_options_exit_with_usage_error_before_sending() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(&server, &["--choice", "z"]);
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(reported(&output), "usage.question");
    assert_eq!(server.count(), 0);
}

#[test]
fn an_unknown_reply_option_exits_with_backend_error() {
    let server = Loopback::answering(
        |_| json!({"type":"choice", "choice":"unknown", "probabilities":{"z":0.8,"a":0.2}}),
    )
    .unwrap();
    let output = run(&server, &[]);
    assert_eq!(output.status.code(), Some(6));
    assert_eq!(output.stdout, b"[1] failed\n");
    assert_eq!(reported(&output), "backend.reply");
}
