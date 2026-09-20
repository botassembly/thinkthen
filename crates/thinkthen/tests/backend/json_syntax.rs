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
            "thinkthen: stopped at record 1; 0 records finished, 0 from a recording\n",
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
