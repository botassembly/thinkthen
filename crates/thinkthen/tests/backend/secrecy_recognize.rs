//! Ticket 0147, test 9: recognize's new failure paths keep the text and the
//! kind descriptions off standard output and standard error.

use std::fs;
use std::path::PathBuf;
use std::process::Output;

use crate::harness::{Canned, Listener, spawn};
use crate::recognize::automatic;
use crate::secrecy::{EVIDENCE, KEY, KIND};

const RESERVED: &str = "thinkthen: recognize reserves the kind names none of these, ENTITY and ANY in any ASCII case\n";

fn recognize(listener: &Listener, options: &[&str], text: &str) -> Output {
    let mut arguments = vec![
        "recognize",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--no-cache",
    ];
    arguments.extend_from_slice(options);
    spawn(&arguments, &[("THINKTHEN_API_KEY", KEY)], text.as_bytes()).expect("command")
}

/// The run printed nothing, and its message is exactly `says`.
fn quiet(output: &Output, code: i32, says: &str) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(code), "{stderr}");
    assert!(output.stdout.is_empty());
    assert_eq!(stderr, says);
    for secret in [EVIDENCE, KIND, KEY] {
        assert!(!stderr.contains(secret), "{secret}");
    }
}

#[test]
fn the_guard_names_sizes_and_never_the_text() {
    let listener = Listener::answering(automatic).expect("listener");
    let text = format!("Ada met {EVIDENCE}.");
    let description = format!("person={KIND}");
    let output = recognize(
        &listener,
        &["--kind", &description, "--max-text-bytes", "10"],
        &text,
    );
    quiet(
        &output,
        2,
        "thinkthen: recognize: the text is 31 bytes, over the limit of 10; raise it with --max-text-bytes\n",
    );
    assert_eq!(listener.connections(), 0);
}

#[test]
fn a_failed_step_two_request_quotes_neither_text_nor_description() {
    let wrong = format!(
        r#"{{"model":"local-1","answers":{{"q1":{{"type":"noul","noul":0.5}}}},"echo":"{EVIDENCE} {KIND}"}}"#
    );
    let listener = Listener::answering(move |body| {
        if String::from_utf8_lossy(body).contains("none of these") {
            Canned::ok(&wrong)
        } else {
            automatic(body)
        }
    })
    .expect("listener");
    let description = format!("person={KIND}");
    let output = recognize(
        &listener,
        &["--kind", &description],
        &format!("Ada met {EVIDENCE}."),
    );
    quiet(
        &output,
        4,
        "thinkthen: the reply was refused: the answer to question `q1` is not the shape the question asked for\n",
    );
    assert_eq!(listener.requests().len(), 2);
}

#[test]
fn reserved_kinds_refuse_at_exit_2_on_the_command_line_and_5_from_a_file() {
    let listener = Listener::answering(automatic).expect("listener");
    let description = format!("entity={KIND}");
    for options in [
        vec!["ENTITY"],
        vec!["person", "any"],
        vec!["None Of These"],
        vec!["--kind", &description],
    ] {
        let output = recognize(&listener, &options, EVIDENCE);
        quiet(&output, 2, RESERVED);
    }
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-reserved.json");
    fs::write(
        &path,
        format!(r#"{{"version":1,"recognize":{{"kinds":{{"entity":"{KIND}"}}}}}}"#),
    )
    .expect("question file");
    let output = recognize(&listener, &[&format!("@{}", path.display())], EVIDENCE);
    quiet(&output, 5, RESERVED);
    assert_eq!(listener.connections(), 0);
}
