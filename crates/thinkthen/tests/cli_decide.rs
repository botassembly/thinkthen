#![cfg(feature = "command")]
//! `thinkthen decide` on the command line.
#![cfg(test)]

mod support;

use std::io::Write;
use std::process::{Output, Stdio};

use loopback::{KEY, Loopback, Reply};
use serde_json::json;
use support::reported;

fn run(server: &Loopback, args: &[&str], key: Option<&str>) -> Output {
    run_on(server, args, key, b"Please refund my order.")
}

fn run_on(server: &Loopback, args: &[&str], key: Option<&str>, input: &[u8]) -> Output {
    let mut command = support::command();
    command
        .args(args)
        .env("THINKTHEN_BASE_URL", server.base())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(key) = key {
        command.env("THINKTHEN_API_KEY", key);
    }
    let mut child = command.spawn().unwrap();
    // A command that refuses its arguments may exit before it reads the text.
    let _ = child.stdin.take().unwrap().write_all(input);
    child.wait_with_output().unwrap()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[test]
fn the_answer_word_and_exit_code_follow_the_verdict() {
    let cases: [(f64, &[&str], &str, i32); 3] = [
        (0.9, &[], "true\n", 0),
        (0.1, &[], "false\n", 1),
        (0.5, &["--threshold", "0.2:0.8"], "unsure\n", 3),
    ];
    for (p, extra, stdout, code) in cases {
        let server = Loopback::noul(p).unwrap();
        let mut args = vec!["decide", "Is it a refund request?"];
        args.extend_from_slice(extra);
        let output = run(&server, &args, Some(KEY));
        assert_eq!(text(&output.stdout), stdout, "p {p}");
        assert_eq!(output.status.code(), Some(code), "p {p}");
    }
}

#[test]
fn details_print_the_row_as_json() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(&server, &["decide", "Is it?", "--details"], Some(KEY));
    let row: serde_json::Value =
        loopback::without_source(serde_json::from_slice(&output.stdout).unwrap());
    assert_eq!(
        row,
        json!({"id": 1, "status": "answered", "answer": true, "p": 0.9})
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn the_url_option_beats_the_environment() {
    let server = Loopback::noul(0.9).unwrap();
    let mut command = support::command();
    let output = command
        .args(["decide", "Is it?", "--url", server.base()])
        .env("THINKTHEN_BASE_URL", "http://127.0.0.1:9/v1")
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(server.count(), 1);
}

#[test]
fn a_bad_threshold_exits_2_and_sends_nothing() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(
        &server,
        &["decide", "Is it?", "--threshold", "90"],
        Some(KEY),
    );
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(reported(&output), "usage.threshold");
    assert_eq!(server.count(), 0);
}

#[test]
fn the_model_option_names_the_model_in_the_request() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(&server, &["decide", "Is it?", "--model", "m-x"], Some(KEY));
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(server.requests()[0].json().unwrap()["model"], "m-x");
}

#[test]
fn input_that_is_not_utf8_exits_5_and_sends_nothing() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run_on(&server, &["decide", "Is it?"], Some(KEY), b"caf\xe9");
    assert_eq!(output.status.code(), Some(5));
    assert_eq!(reported(&output), "local.io");
    assert_eq!(server.count(), 0);
}

#[test]
fn a_missing_key_exits_2_and_sends_nothing() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(&server, &["decide", "Is it?"], None);
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(reported(&output), "usage.key-missing");
    assert_eq!(server.count(), 0);
}

#[test]
fn a_backend_failure_exits_4_and_never_prints_the_key() {
    let server =
        Loopback::start(|_| Reply::json(401, &json!({"error": format!("bad key {KEY}")}))).unwrap();
    for args in [
        &["decide", "Is it?"][..],
        &["decide", "Is it?", "--details"][..],
    ] {
        let output = run(&server, args, Some(KEY));
        assert_eq!(output.status.code(), Some(4), "{args:?}");
        if !args.contains(&"--details") {
            assert_eq!(reported(&output), "backend.status", "{args:?}");
        }
        assert!(!text(&output.stdout).contains(KEY));
        assert!(!text(&output.stderr).contains(KEY));
    }
}
