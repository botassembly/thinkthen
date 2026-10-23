#![cfg(feature = "command")]
#![cfg(test)]
//! Choice command arguments, rendering, and exit codes.

mod support;

use loopback::{KEY, Loopback};
use serde_json::json;
use std::process::{Output, Stdio};
use support::reported;

fn run(server: &Loopback, extra: &[&str]) -> Output {
    support::command()
        .args(["choose", "Which?", "--option", "z", "--option", "a"])
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
        (0.6, vec!["--threshold", "0.8"], "unsure\n", 3),
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
    let output = run(&server, &["--details"]);
    assert_eq!(
        loopback::without_source(
            serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap()
        ),
        json!({"id":1,"status":"answered","answer":"z","p":0.8})
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn duplicate_options_exit_with_usage_error_before_sending() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(&server, &["--option", "z"]);
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
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(reported(&output), "backend.reply");
}
