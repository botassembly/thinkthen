#![cfg(test)]
//! Literal command-line text uses the same single-record driver as stdin.
#![cfg(feature = "command")]

mod support;

use serde_json::Value;
use std::io::Write;
use std::process::{Output, Stdio};

fn run(args: &[&str], input: &[u8], framing: Option<&str>) -> Output {
    let mut command = support::command();
    command.args(args).args(["--backend", "stub"]);
    if let Some(framing) = framing {
        command.env("THINKTHEN_FROM", framing);
    }
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // Usage rejection and literal input may finish without reading the pipe.
    let _ = child.stdin.take().unwrap().write_all(input);
    child.wait_with_output().unwrap()
}

#[test]
fn all_ten_functions_judge_literal_text_as_one_record() {
    let toolbox = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(
        toolbox.path(),
        r#"{"questions":{"refund":{"function":"decide","question":"Ready?"}}}"#,
    )
    .unwrap();
    let cases = [
        vec!["decide", "Ready?"],
        vec![
            "choose", "Which?", "--choice", "billing", "--choice", "shipping",
        ],
        vec!["tag", "Which?", "--label", "billing", "--label", "shipping"],
        vec!["score", "How urgent?", "--level", "low", "--level", "high"],
        vec!["filter", "Ready?"],
        vec!["rank", "How urgent?", "--level", "low", "--level", "high"],
        vec!["find", "Which line?"],
        vec!["recognize", "--kind", "person", "--kind", "company"],
        vec!["relate", "--rule", "person", "company", "works_for"],
        vec!["annotate", toolbox.path().to_str().unwrap()],
    ];
    let text = "Ada works for Acme.\nPlease refund my order.";
    for mut args in cases {
        let function = args[0];
        if function == "filter" {
            args.extend(["--threshold", "0.000001"]);
        } else {
            args.push("--json");
        }
        let mut stdin_args = args.clone();
        stdin_args.extend(["--from", "text"]);
        let expected = run(&stdin_args, text.as_bytes(), None);
        assert_eq!(expected.status.code(), Some(0), "{function}: {expected:?}");
        if function == "filter" {
            assert_eq!(expected.stdout, format!("{text}\n").as_bytes());
        } else {
            let row: Value = serde_json::from_slice(&expected.stdout).unwrap();
            assert_eq!(row["function"], function);
            assert_eq!(row["position"], 1);
            assert!(row.get("answer").is_some());
        }

        args.extend(["--text", text]);
        let actual = run(&args, b"ignored stdin\nsecond ignored record", None);
        assert_eq!(actual.status.code(), expected.status.code(), "{function}");
        assert_eq!(actual.stdout, expected.stdout, "{function}: {actual:?}");
        assert!(actual.stderr.is_empty(), "{function}: {actual:?}");
    }
}

#[test]
fn text_conflicts_with_files_and_explicit_framing() {
    for extra in [vec!["missing.txt"], vec!["-"], vec!["--from", "text"]] {
        let mut args = vec!["decide", "Ready?", "--text", "refund"];
        args.extend(extra);
        let output = run(&args, b"", None);
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert_eq!(support::reported(&output), "usage.option");
        assert!(String::from_utf8_lossy(&output.stderr).contains("conflicting arguments"));
        assert!(output.stdout.is_empty());
    }
}

#[test]
fn explicit_text_overrides_inherited_framing_and_is_not_a_filename() {
    let args = [
        "decide",
        "Ready?",
        "--json",
        "--text",
        "--missing.json\nsecond\n",
    ];
    let actual = run(&args, b"not the requested text", Some("jsonl"));
    let expected = run(
        &["decide", "Ready?", "--json", "--from", "text"],
        b"--missing.json\nsecond\n\n",
        None,
    );
    assert_eq!(actual.status.code(), Some(0), "{actual:?}");
    assert_eq!(actual.stdout, expected.stdout);
    assert!(actual.stderr.is_empty());
}

#[test]
fn text_check_returns_the_same_verdict_and_failure_codes_as_stdin() {
    for (options, code) in [
        (vec!["--threshold", "0.01"], 0),
        (vec!["--threshold", "1"], 1),
        (vec!["--low", "0.01", "--high", "0.99"], 3),
    ] {
        let mut args = vec!["decide", "Ready?", "--check"];
        args.extend(options);
        let expected = run(&args, b"[]", None);
        assert_eq!(expected.status.code(), Some(code), "{expected:?}");
        args.extend(["--text", "[]"]);
        let actual = run(&args, b"ignored", None);
        assert_eq!(actual.status.code(), expected.status.code(), "{actual:?}");
        assert_eq!(actual.stdout, expected.stdout);
        assert_eq!(actual.stderr, expected.stderr);
    }
    let empty = run(&["decide", "Ready?", "--check"], b"", None);
    let failed = run(
        &["decide", "Ready?", "--check", "--text", ""],
        b"ignored",
        None,
    );
    assert_eq!(empty.status.code(), Some(2), "{empty:?}");
    assert_eq!(failed.status.code(), empty.status.code(), "{failed:?}");
    assert_eq!(support::reported(&failed), "record.empty");
}

#[cfg(unix)]
#[test]
fn literal_text_finishes_while_standard_input_is_still_open() {
    let mut child = support::command()
        .args([
            "decide",
            "Ready?",
            "--backend",
            "stub",
            "--text",
            "refund",
            "--json",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let _input = child.stdin.take().unwrap();
    let _deadline = support::deadline(child.id());
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let row: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(row["position"], 1);
    assert!(row.get("answer").is_some());
}

#[test]
fn function_help_lists_text_without_adding_it_to_saved_runs() {
    for function in [
        "decide",
        "choose",
        "tag",
        "score",
        "filter",
        "rank",
        "find",
        "recognize",
        "relate",
        "annotate",
    ] {
        let output = support::command()
            .args([function, "--help"])
            .output()
            .unwrap();
        assert!(output.status.success());
        assert!(
            String::from_utf8(output.stdout)
                .unwrap()
                .contains("--text <TEXT>"),
            "{function}"
        );
    }
    let output = support::command().args(["run", "--help"]).output().unwrap();
    assert!(!String::from_utf8(output.stdout).unwrap().contains("--text"));
}
