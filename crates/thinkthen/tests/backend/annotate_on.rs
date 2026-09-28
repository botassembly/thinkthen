//! `on` reads inside the JSON value a record selected, and never parses text.

use std::fs;
use std::path::PathBuf;

use crate::harness::{Canned, Listener, spawn};

const PART: &str = r#"{"version":1,"questions":{"unresolved":{"decide":"Still open?","on":"/x"}}}"#;
const BODY: &str =
    r#"{"version":1,"questions":{"unresolved":{"decide":"Still open?","on":"/body"}}}"#;
const WHOLE: &str = r#"{"version":1,"questions":{"unresolved":{"decide":"Still open?"}}}"#;
const YES: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.95}}}"#;
const STREAM: &str = concat!(
    "thinkthen: question `unresolved` reads `on`, and this record's evidence is text with no members\n",
    "thinkthen: stopped at record 1; 0 records finished\n",
);
const DOCUMENT: &str = "thinkthen: question `unresolved` reads `on`, and this record's evidence is text with no members\n";
const LINES: &str =
    "thinkthen: question `unresolved` reads `on`, and a --lines record is text with no members\n";

/// One edge row: the set, the framing arguments, the input, and what the run
/// prints, exits with, and sends as each request's `state`.
struct Row {
    set: &'static str,
    framing: &'static [&'static str],
    input: &'static [u8],
    stdout: &'static str,
    stderr: &'static str,
    code: i32,
    states: &'static str,
}

const ROWS: [Row; 8] = [
    Row {
        set: PART,
        framing: &["--jsonl", "--field", "/body"],
        input: br#"{"body":"{\"x\":1}"}"#,
        stdout: "",
        stderr: STREAM,
        code: 2,
        states: "",
    },
    Row {
        set: PART,
        framing: &["--jsonl", "--field", "/body"],
        input: br#"{"body":"plain words"}"#,
        stdout: "",
        stderr: STREAM,
        code: 2,
        states: "",
    },
    Row {
        set: PART,
        framing: &["--jsonl", "--field", "/body"],
        input: br#"{"body":{"x":1}}"#,
        stdout: "{\"body\":{\"x\":1},\"unresolved\":true}\n",
        stderr: "",
        code: 0,
        states: r#""1""#,
    },
    Row {
        set: PART,
        framing: &["--lines"],
        input: b"{\"x\":1}\n",
        stdout: "",
        stderr: LINES,
        code: 2,
        states: "",
    },
    Row {
        set: PART,
        framing: &["--lines"],
        input: b"",
        stdout: "",
        stderr: LINES,
        code: 2,
        states: "",
    },
    Row {
        set: WHOLE,
        framing: &["--lines"],
        input: b"{\"x\":1}\n",
        stdout: "{\"input\":\"{\\\"x\\\":1}\",\"value\":{\"unresolved\":true}}\n",
        stderr: "",
        code: 0,
        states: r#""{\"x\":1}""#,
    },
    Row {
        set: BODY,
        framing: &[],
        input: br#"{"body":"text"}"#,
        stdout: "{\"body\":\"text\",\"unresolved\":true}\n",
        stderr: "",
        code: 0,
        states: r#""text""#,
    },
    Row {
        set: BODY,
        framing: &[],
        input: b"plain words about a failure",
        stdout: "",
        stderr: DOCUMENT,
        code: 2,
        states: "",
    },
];

fn set(place: usize, text: &str) -> PathBuf {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("annotate-on");
    let _created = fs::create_dir_all(&folder);
    let path = folder.join(format!("row-{place}.json"));
    let _written = fs::write(&path, text);
    path
}

#[test]
fn on_reads_the_selected_value() {
    for (place, row) in ROWS.iter().enumerate() {
        let listener = Listener::answering(|_| Canned::ok(YES)).expect("a listener");
        let file = set(place + 1, row.set);
        let mut arguments = vec![
            "annotate",
            file.to_str().expect("a path"),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--no-cache",
        ];
        arguments.extend_from_slice(row.framing);
        let output = spawn(&arguments, &[("THINKTHEN_API_KEY", "sk-test")], row.input)
            .expect("the command runs");
        let states = listener
            .requests()
            .iter()
            .map(|request| {
                let body: serde_json::Value = serde_json::from_slice(&request.body).expect("JSON");
                body["state"].to_string()
            })
            .collect::<Vec<_>>();
        let out = |bytes: &[u8]| String::from_utf8_lossy(bytes).into_owned();
        assert_eq!(
            (
                out(&output.stdout),
                out(&output.stderr),
                output.status.code()
            ),
            (row.stdout.to_owned(), row.stderr.to_owned(), Some(row.code)),
            "edge row {}",
            place + 1
        );
        assert_eq!(states.join(" "), row.states, "edge row {}", place + 1);
    }
}
