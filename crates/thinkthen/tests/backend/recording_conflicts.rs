//! Repeated recording writes. Every function stores one answer per question
//! and `--record` replaces it, by ADR 0111 section 3.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::harness::{Canned, Listener, spawn_one as spawn};
use crate::support::stored;

const QUESTION: &str = "asks for a refund";
const EVIDENCE: &str = "Refund me please.";
const FALSE: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.1}},"#,
    r#""usage":{"input_tokens":10,"output_tokens":2}}"#,
);
const TRUE: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}},"#,
    r#""usage":{"input_tokens":10,"output_tokens":2}}"#,
);
fn folder(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    path
}

fn decide(
    base: &str,
    folder: &Path,
    arguments: &[&str],
    input: &str,
) -> io::Result<std::process::Output> {
    asked("decide", base, folder, arguments, input)
}

fn asked(
    verb: &str,
    base: &str,
    folder: &Path,
    arguments: &[&str],
    input: &str,
) -> io::Result<std::process::Output> {
    let common = [
        verb,
        QUESTION,
        "--url",
        base,
        "--model",
        "local-1",
        "--record",
        &folder.to_string_lossy(),
    ];
    spawn(
        &[&common[..], arguments].concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input.as_bytes(),
    )
}

#[test]
fn equal_records_in_one_record_run_ask_their_question_once() {
    let folder = folder("recording-divergent-duplicate");
    let listener =
        Listener::serving(vec![Canned::ok(FALSE), Canned::ok(TRUE)]).expect("a loopback listener");
    let output = decide(
        listener.base(),
        &folder,
        &["--lines", "--jobs", "1"],
        &format!("{EVIDENCE}\n{EVIDENCE}\n"),
    )
    .expect("the binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            r#"{"input":"Refund me please.","value":false}"#,
            "\n",
            r#"{"input":"Refund me please.","value":false}"#,
            "\n",
        )
    );
    assert_eq!(listener.requests().len(), 1, "equal keys are asked once");
    let answers = stored(&folder).expect("the store");
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0]["answer"], r#"{"type":"noul","noul":0.1}"#);
}

#[test]
fn recording_again_replaces_the_stored_answer() {
    let folder = folder("recording-replaces");
    let listener =
        Listener::serving(vec![Canned::ok(TRUE), Canned::ok(FALSE)]).expect("a loopback listener");
    for (expected, answer) in [
        (0, r#"{"type":"noul","noul":0.9}"#),
        (1, r#"{"type":"noul","noul":0.1}"#),
    ] {
        let output = decide(listener.base(), &folder, &[], EVIDENCE).expect("the binary runs");
        assert_eq!(output.status.code(), Some(expected));
        assert!(output.stderr.is_empty());
        let answers = stored(&folder).expect("the store");
        assert_eq!(answers.len(), 1);
        assert_eq!(answers[0]["answer"], answer);
    }
    assert_eq!(listener.requests().len(), 2, "--record asks every question");
}
