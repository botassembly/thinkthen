#![cfg(feature = "command")]
#![cfg(test)]
//! Filtering input bytes and rendering details through the command.

mod support;

use std::io::Write;
use std::process::{Command, Output, Stdio};

use loopback::{KEY, Loopback};
use serde_json::{Value, json};

fn run(args: &[&str], input: &[u8]) -> Output {
    let server = Loopback::answering(|question| {
        let text = question["instructions"].as_str().unwrap();
        let p = if text.contains("refund") {
            0.9
        } else if text.contains("maybe") {
            0.5
        } else if text.contains("broken") {
            2.0
        } else {
            0.1
        };
        json!({"type": "noul", "noul": p})
    })
    .unwrap();
    let mut child = command(&server, args).spawn().unwrap();
    let _ = child.stdin.take().unwrap().write_all(input);
    child.wait_with_output().unwrap()
}

fn command(server: &Loopback, args: &[&str]) -> Command {
    let mut command = support::command();
    command
        .args(["filter", "Keep?", "--threshold", "0.2:0.8"])
        .args(args)
        .env("THINKTHEN_API_KEY", KEY)
        .env("THINKTHEN_BASE_URL", server.base())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
}

#[test]
fn passing_records_keep_their_exact_input_bytes() {
    type Case = (&'static [&'static str], &'static [u8], &'static [u8]);
    let cases: [Case; 4] = [
        (&["--lines"], b" refund first \r\nparcel\nmaybe\nbroken\nrefund last", b" refund first \r\nrefund last"),
        (&["--jsonl", "--on", "/body"], b" { \"id\": 1, \"body\": \"refund\" }\r\n{\"body\":\"parcel\"}\ninvalid\n{\"body\":\"refund last\"}", b" { \"id\": 1, \"body\": \"refund\" }\r\n{\"body\":\"refund last\"}"),
        (&[], b"refund\nwhole text\n", b"refund\nwhole text\n"),
        (&["--lines"], b"", b""),
    ];
    for (args, input, expected) in cases {
        let output = run(args, input);
        assert_eq!(output.stdout, expected, "{args:?}");
        let code = if input.windows(6).any(|bytes| bytes == b"broken") {
            4
        } else if input.windows(7).any(|bytes| bytes == b"invalid") {
            2
        } else {
            0
        };
        assert_eq!(output.status.code(), Some(code), "{args:?}");
    }
}

#[test]
fn details_print_every_row_including_no_unsure_and_failure() {
    let output = run(
        &["--lines", "--details"],
        b"refund\nparcel\nmaybe\nbroken\nrefund last\n",
    );
    let rows: Vec<Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(rows.len(), 5);
    assert_eq!(
        rows.iter().map(|row| row["id"].clone()).collect::<Vec<_>>(),
        vec![json!(1), json!(2), json!(3), json!(4), json!(5)]
    );
    assert_eq!(rows[0]["answer"], true);
    assert_eq!(rows[1]["answer"], false);
    assert_eq!(rows[2]["status"], "unsure");
    assert_eq!(rows[3]["status"], "failed");
    assert_eq!(rows[3]["error"]["code"], "backend.reply");
    assert_eq!(output.status.code(), Some(4));
}

#[test]
fn empty_single_text_still_has_a_details_row() {
    let output = run(&["--details"], b"");
    let row: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(row["id"], 1);
    assert_eq!(row["answer"], false);
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn invalid_text_reports_failure_and_keeps_preceding_passes() {
    for (args, input, expected) in [
        (&[][..], &b"refund\xff"[..], &b""[..]),
        (
            &["--lines"][..],
            &b"refund\r\ninvalid\xff\nrefund lost\n"[..],
            &b"refund\r\n"[..],
        ),
    ] {
        let output = run(args, input);
        assert_eq!(output.stdout, expected);
        assert_eq!(output.status.code(), Some(5));
        assert!(
            String::from_utf8(output.stderr)
                .unwrap()
                .contains("local.io")
        );
    }
}

#[test]
fn closed_output_reports_local_failure_for_originals_and_details() {
    for args in [&["--lines"][..], &["--lines", "--details"][..]] {
        let server = Loopback::noul(0.9).unwrap();
        let (stdout, inherited_reader) = support::closed_output();
        let mut child = command(&server, args).stdout(stdout).spawn().unwrap();
        let _ = child.stdin.take().unwrap().write_all(b"one\ntwo\nthree\n");
        let output = child.wait_with_output().unwrap();
        drop(inherited_reader);
        let diagnostic = format!(
            "mode={args:?}, status={}, stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(output.status.code(), Some(5), "{diagnostic}");
        assert_eq!(support::reported(&output), "local.io", "{diagnostic}");
    }
}
