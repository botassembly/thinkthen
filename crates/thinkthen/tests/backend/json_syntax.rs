//! Invalid JSON names its input, gives a safe place, and opens no connection.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;

use crate::harness::{Canned, Listener, spawn};

const QUESTION: &str = "Does this report a payment failure?";
type PrivateCase<'a> = (&'a str, &'a [u8], &'a [&'a str]);

fn written(name: &str, bytes: &[u8]) -> io::Result<PathBuf> {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    fs::write(&path, bytes)?;
    Ok(path)
}

fn said(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn listener() -> io::Result<Listener> {
    Listener::answering(|_| Canned::ok("{}"))
}

fn assert_refused(output: &Output, listener: &Listener, code: i32, message: &str) {
    assert_eq!(output.status.code(), Some(code));
    assert!(output.stdout.is_empty());
    assert_eq!(said(output), format!("thinkthen: {message}\n"));
    assert_eq!(listener.connections(), 0);
    assert!(listener.requests().is_empty());
}

#[test]
fn question_file_syntax_names_the_question_file_and_sends_nothing() {
    let cases: [(&str, &[u8], usize); 3] = [
        ("empty", b"", 0),
        ("bom", b"\xef\xbb\xbf{\"decide\":\"q\"}", 1),
        ("trailing", b"{\"decide\":\"q\",}\n", 15),
    ];
    for (name, bytes, column) in cases {
        let listener = listener().expect("a loopback listener");
        let path =
            written(&format!("syntax-question-{name}.json"), bytes).expect("a question file");
        let question = format!("@{}", path.display());
        let output = spawn(
            &["decide", &question, "--url", listener.base()],
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            b"evidence",
        )
        .expect("the compiled binary runs");
        assert_refused(
            &output,
            &listener,
            5,
            &format!(
                "the question file is not valid JSON: the JSON at line 1 column {column} is not one"
            ),
        );
    }
}

#[test]
fn whole_input_syntax_names_stdin_or_the_input_file_and_sends_nothing() {
    let cases: [(&str, &[u8], usize); 3] = [
        ("empty", b"", 0),
        ("bom", b"\xef\xbb\xbf{\"a\":\"x\"}", 1),
        ("trailing", b"{\"a\":\"x\",}\n", 10),
    ];
    for (name, bytes, column) in cases {
        for from_file in [false, true] {
            let listener = listener().expect("a loopback listener");
            let path = written(&format!("syntax-input-{name}.json"), bytes).expect("an input file");
            let mut arguments = vec![
                "decide",
                QUESTION,
                "--url",
                listener.base(),
                "--field",
                "/a",
            ];
            let path = path.to_string_lossy();
            if from_file {
                arguments.extend(["--input", path.as_ref()]);
            }
            let input = if from_file { &b""[..] } else { bytes };
            let output = spawn(&arguments, &[("THINKTHEN_API_KEY", "sk-test-value")], input)
                .expect("the compiled binary runs");
            assert_refused(
                &output,
                &listener,
                2,
                &format!(
                    "the input is not valid JSON: the JSON at line 1 column {column} is not one"
                ),
            );
        }
    }
}

#[test]
fn jsonl_syntax_keeps_the_record_sentence_and_sends_nothing() {
    let listener = listener().expect("a loopback listener");
    let output = spawn(
        &[
            "decide",
            QUESTION,
            "--url",
            listener.base(),
            "--jsonl",
            "--field",
            "/a",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"private-invalid-token\n",
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(
        said(&output),
        concat!(
            "thinkthen: the record is not valid JSON\n",
            "thinkthen: stopped at record 1; 0 records finished\n",
        )
    );
    assert_eq!(listener.connections(), 0);
    assert!(listener.requests().is_empty());
}

#[test]
fn syntax_diagnostics_repeat_no_input_bytes() {
    const MARKER: &str = "private-marker-7b3ac5";
    let cases: [PrivateCase<'_>; 2] = [
        (
            "question",
            b"{\"decide\":\"private-marker-7b3ac5\",}\n",
            &["decide"],
        ),
        (
            "input",
            b"{\"a\":\"private-marker-7b3ac5\",}\n",
            &["decide", QUESTION, "--field", "/a"],
        ),
    ];
    for (name, bytes, prefix) in cases {
        let listener = listener().expect("a loopback listener");
        let path =
            written(&format!("syntax-private-{name}.json"), bytes).expect("a private fixture");
        let named = if name == "question" {
            format!("@{}", path.display())
        } else {
            path.to_string_lossy().into_owned()
        };
        let mut arguments = prefix.to_vec();
        if name == "question" {
            arguments.push(&named);
        } else {
            arguments.extend(["--input", &named]);
        }
        arguments.extend(["--url", listener.base()]);
        let output = spawn(&arguments, &[("THINKTHEN_API_KEY", "sk-test-value")], b"")
            .expect("the compiled binary runs");
        let message = said(&output);
        assert!(!message.contains(MARKER), "{name}: {message}");
        assert!(!message.contains("decide"), "{name}: {message}");
        assert!(!message.contains("\"a\""), "{name}: {message}");
        assert_eq!(listener.connections(), 0, "{name}");
        assert!(listener.requests().is_empty(), "{name}");
    }
}

#[test]
fn a_directory_is_not_an_input_file_and_no_record_was_framed() {
    let listener = listener().expect("a loopback listener");
    let directory = Path::new(env!("CARGO_TARGET_TMPDIR")).join("input-is-directory");
    fs::create_dir_all(&directory).expect("a test directory");
    let named = directory.to_string_lossy();
    let output = spawn(
        &[
            "decide",
            QUESTION,
            "--lines",
            "--input",
            named.as_ref(),
            "--url",
            listener.base(),
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"ignored",
    )
    .expect("the compiled binary runs");
    assert_refused(
        &output,
        &listener,
        5,
        "`--input` names a directory, and a directory is not an input file",
    );
    assert!(!said(&output).contains("stopped at record"));
}

#[test]
fn invalid_utf8_names_a_stream_record_and_a_whole_document_as_evidence() {
    for (framing, noun, summary) in [
        (Some("--lines"), "record", true),
        (Some("--jsonl"), "record", true),
        (None, "evidence", false),
    ] {
        let listener = listener().expect("a loopback listener");
        let mut arguments = vec!["decide", QUESTION, "--url", listener.base()];
        if let Some(flag) = framing {
            arguments.push(flag);
        }
        let output = spawn(
            &arguments,
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            b"private\xff\n",
        )
        .expect("the compiled binary runs");
        let expected = if summary {
            format!(
                "thinkthen: the {noun} is not valid UTF-8\nthinkthen: stopped at record 1; 0 records finished\n"
            )
        } else {
            format!("thinkthen: the {noun} is not valid UTF-8\n")
        };
        assert_eq!(output.status.code(), Some(5));
        assert_eq!(said(&output), expected);
        assert!(output.stdout.is_empty());
        assert_eq!(listener.connections(), 0);
        assert!(listener.requests().is_empty());
        assert!(!said(&output).contains("private"));
    }
}

const TOO_DEEP: &str =
    "the JSON nests more than 127 levels of arrays and objects, the most this tool reads";

/// A decide reply that answers every question the request asks.
fn answered(body: &[u8]) -> Canned {
    let request: serde_json::Value = serde_json::from_slice(body).unwrap_or_default();
    let answers: serde_json::Map<String, serde_json::Value> = request["questions"]
        .as_object()
        .into_iter()
        .flat_map(|questions| questions.keys())
        .map(|name| (name.clone(), serde_json::json!({"type": "noul", "noul": 0.9})))
        .collect();
    Canned::ok(&serde_json::json!({"model": "jev-1.13.0", "answers": answers}).to_string())
}

/// A record nests at most 127 levels of arrays and objects. At 128 it is
/// refused by that limit, not as bad syntax, and nothing is sent.
#[test]
fn a_jsonl_record_nests_at_most_127_levels() {
    let nested = |depth: usize, open: &str, close: &str| {
        format!("{}\"x\"{}\n", open.repeat(depth), close.repeat(depth))
    };
    for (depth, open, close) in [
        (127, "[", "]"),
        (127, "{\"a\":", "}"),
        (128, "[", "]"),
        (128, "{\"a\":", "}"),
    ] {
        let listener = Listener::answering(answered).expect("a loopback listener");
        let output = spawn(
            &[
                "decide",
                QUESTION,
                "--jsonl",
                "--no-cache",
                "--url",
                listener.base(),
            ],
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            nested(depth, open, close).as_bytes(),
        )
        .expect("the compiled binary runs");
        if depth == 127 {
            assert_eq!(output.status.code(), Some(0), "{depth} {open}: {}", said(&output));
            assert_eq!(listener.count(), 1, "{depth} {open}");
            continue;
        }
        assert_eq!(output.status.code(), Some(2), "{depth} {open}");
        assert!(output.stdout.is_empty());
        assert_eq!(
            said(&output),
            format!("thinkthen: {TOO_DEEP}\nthinkthen: stopped at record 1; 0 records finished\n")
        );
        assert_eq!(listener.connections(), 0);
    }
}

#[test]
fn a_question_file_past_the_depth_limit_names_the_limit() {
    let listener = listener().expect("a loopback listener");
    let deep = format!("{{\"decide\":\"q\",\"x\":{}1{}}}", "[".repeat(127), "]".repeat(127));
    let path = written("syntax-question-too-deep.json", deep.as_bytes()).expect("a question file");
    let question = format!("@{}", path.display());
    let output = spawn(
        &["decide", &question, "--url", listener.base()],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"evidence",
    )
    .expect("the compiled binary runs");
    assert_refused(
        &output,
        &listener,
        5,
        &format!("the question file is not JSON this tool reads: {TOO_DEEP}"),
    );
}
