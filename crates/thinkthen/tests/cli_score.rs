#![cfg(feature = "command")]
#![cfg(test)]
//! `score` through the command.

mod support;

use std::io::Write;
use std::process::{Output, Stdio};

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use support::reported;

fn run(server: &Loopback, args: &[&str], input: &str) -> Output {
    let mut child = support::command()
        .arg("score")
        .arg("How urgent?")
        .args(args)
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
fn level_arguments_print_the_number_or_backend_failure() {
    let cases: [(&[&str], Value, Option<f64>); 2] = [
        (&["low", "high"], json!({"0": 0.2, "1": 0.8}), Some(0.8)),
        (&["low", "high"], json!({"0": 0.2, "1": 0.5}), None),
    ];
    for (levels, probabilities, expected) in cases {
        let server =
            Loopback::answering(move |_| json!({"type": "score", "probabilities": probabilities}))
                .unwrap();
        let args: Vec<&str> = levels
            .iter()
            .flat_map(|level| ["--level", *level])
            .collect();
        let output = run(&server, &args, "now");
        match expected {
            Some(expected) => {
                assert_eq!(output.status.code(), Some(0));
                let answer: f64 = String::from_utf8(output.stdout)
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap();
                assert!((answer - expected).abs() < 1e-12);
            }
            None => {
                assert_eq!(output.status.code(), Some(4));
                assert_eq!(reported(&output), "backend.reply");
                assert!(output.stdout.is_empty());
            }
        }
        assert_eq!(
            server.requests()[0].json().unwrap()["questions"]["q1"]["criteria"],
            json!(levels)
        );
    }
}

#[test]
fn details_and_record_modes_print_score_rows() {
    for extra in [
        &["--details"][..],
        &["--lines"][..],
        &["--jsonl", "--on", "/body"][..],
    ] {
        let server = Loopback::answering(
            |_| json!({"type": "score", "probabilities": {"0": 0.2, "1": 0.8}}),
        )
        .unwrap();
        let mut args = vec!["--level", "low", "--level", "high"];
        args.extend_from_slice(extra);
        let input = if extra.contains(&"--jsonl") {
            "{\"id\":\"a\",\"body\":\"now\"}\n"
        } else {
            "now\n"
        };
        let output = run(&server, &args, input);
        assert_eq!(output.status.code(), Some(0));
        let row: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(row["answer"], 0.8);
        assert_eq!(row["detail"]["probabilities"], json!([0.2, 0.8]));
        assert!(row.get("p").is_none());
    }
}

#[test]
fn invalid_levels_exit_two_and_send_nothing() {
    for args in [
        &[][..],
        &["--level", "low"][..],
        &["--level", "low", "--level", "low"][..],
    ] {
        let server = Loopback::noul(0.9).unwrap();
        let output = run(&server, args, "now");
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(reported(&output), "usage.question");
        assert_eq!(server.count(), 0);
    }
}
