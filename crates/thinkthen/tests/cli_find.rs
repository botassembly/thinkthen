#![cfg(test)]
//! Find command rendering and record framing.
#![cfg(feature = "command")]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

mod support;

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use std::io::Write;
use std::process::Stdio;

#[test]
fn prints_found_text_null_and_unsure_with_their_exit_codes() {
    for (probabilities, text, args, expected, code) in [
        (
            json!({"u1":0.8,"none":0.2}),
            "answer",
            vec![],
            "answer\n",
            0,
        ),
        (json!({"u1":0.2,"none":0.8}), "answer", vec![], "\n", 0),
        (
            json!({"u1":0.6,"none":0.4}),
            "answer",
            vec!["--threshold", "0.7"],
            "unsure\n",
            0,
        ),
        (json!({"u1":0.8,"none":0.2}), " \n", vec![], "\n", 0),
        (
            json!({"u1":0.8,"none":0.2}),
            "answer",
            vec!["--check"],
            "answer\n",
            0,
        ),
        (
            json!({"u1":0.2,"none":0.8}),
            "answer",
            vec!["--check"],
            "\n",
            1,
        ),
        (
            json!({"u1":0.6,"none":0.4}),
            "answer",
            vec!["--threshold", "0.7", "--check"],
            "unsure\n",
            3,
        ),
    ] {
        let server =
            Loopback::answering(move |_| json!({"type":"choice","probabilities":probabilities}))
                .unwrap();
        let mut child = support::command()
            .args(["find", "Which?"])
            .args(args)
            .env("THINKTHEN_BASE_URL", server.base())
            .env("THINKTHEN_API_KEY", KEY)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(text.as_bytes())
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(
            output.status.code(),
            Some(code),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
    }
}

#[test]
fn jsonl_selects_each_text_and_details_include_offsets() {
    let server =
        Loopback::answering(|_| json!({"type":"choice","probabilities":{"u1":0.8,"none":0.2}}))
            .unwrap();
    let mut child = support::command()
        .args([
            "find",
            "Which?",
            "--from",
            "jsonl",
            "--json",
            "--evidence",
            "/body",
        ])
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"{\"id\":\"x\",\"body\":\"answer\"}\n{\"id\":\"empty\",\"body\":\"\"}\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let rows: Vec<Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| {
            let mut row: Value = serde_json::from_str(line).unwrap();
            row.as_object_mut().unwrap().remove("source");
            row
        })
        .collect();
    assert_eq!(
        rows[0]["answer"],
        json!({"text":"answer","start":0,"end":6})
    );
    assert_eq!(
        rows[1],
        json!({"function":"find","position":2,"id":"empty","status":"answered","answer":null})
    );
}
