#![cfg(feature = "command")]
#![cfg(test)]
//! Label arguments, rendering and exit codes through the command.

mod support;

use std::io::Write;
use std::process::{Output, Stdio};

use loopback::{KEY, Loopback};
use serde_json::json;

fn run(server: &Loopback, extra: &[&str], input: &str) -> Output {
    let mut child = support::command()
        .args([
            "tag",
            "What applies?",
            "--label",
            "billing",
            "--label",
            "shipping",
        ])
        .args(extra)
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let _ = child.stdin.take().unwrap().write_all(input.as_bytes());
    child.wait_with_output().unwrap()
}

#[test]
fn the_command_prints_each_matching_label_and_always_exits_zero() {
    for (billing, shipping, expected) in [(0.1, 0.2, ""), (0.5, 0.9, "billing\nshipping\n")] {
        let server = Loopback::answering(move |q| {
            let p = if q["instructions"]
                .as_str()
                .unwrap()
                .ends_with("\"billing\" apply?")
            {
                billing
            } else {
                shipping
            };
            json!({"type": "noul", "noul": p})
        })
        .unwrap();
        let output = run(&server, &[], "message");
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn details_and_record_streams_print_json_rows_without_a_scalar_probability() {
    for (args, input, count) in [
        (vec!["--details"], "message", 1),
        (vec!["--lines", "--threshold", "0.8"], "first\nsecond\n", 2),
        (
            vec!["--jsonl", "--on", "/body"],
            "{\"body\":\"message\"}\n",
            1,
        ),
    ] {
        let server = Loopback::noul(0.8).unwrap();
        let output = run(&server, &args, input);
        assert_eq!(output.status.code(), Some(0));
        let text = String::from_utf8(output.stdout).unwrap();
        let rows: Vec<serde_json::Value> = text
            .lines()
            .map(|line| loopback::without_source(serde_json::from_str(line).unwrap()))
            .collect();
        assert_eq!(rows.len(), count);
        assert_eq!(
            rows[0],
            json!({"id": 1, "status": "answered", "answer": ["billing", "shipping"],
            "detail": {"billing": 0.8, "shipping": 0.8}})
        );
    }
}

#[test]
fn a_threshold_band_exits_two_without_a_send() {
    let server = Loopback::noul(0.9).unwrap();
    let output = run(&server, &["--threshold", "0.2:0.8"], "message");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(support::reported(&output), "usage.threshold");
    assert!(output.stdout.is_empty());
    assert_eq!(server.count(), 0);
}
