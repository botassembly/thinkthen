//! Ticket 0147, test 9: recognize's new failure paths keep the text and the
//! kind descriptions off standard output and standard error.

use std::fs;
use std::path::PathBuf;
use std::process::Output;

use crate::harness::Listener;
use crate::recognize::{REFUSED, automatic, failing_on, local};
use crate::secrecy::{EVIDENCE, KEY, KIND};

const RESERVED: &str = "thinkthen: recognize reserves the kind names none of these, ENTITY and ANY in any ASCII case\n";

fn recognize(listener: &Listener, options: &[&str], text: &str) -> Output {
    local(
        listener,
        &[options, &["--no-cache"]].concat(),
        Some(KEY),
        text.as_bytes(),
    )
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
    let description = format!("person={KIND}");
    let output = recognize(
        &listener,
        &["--kind", &description, "--max-text-bytes", "10"],
        &format!("Ada met {EVIDENCE}."),
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
    let listener = failing_on("none of these", wrong);
    let description = format!("person={KIND}");
    let output = recognize(
        &listener,
        &["--kind", &description],
        &format!("Ada met {EVIDENCE}."),
    );
    quiet(&output, 4, REFUSED);
    assert_eq!(listener.requests().len(), 2);
}

#[test]
fn reserved_kinds_refuse_at_exit_2_on_the_command_line_and_5_from_a_file() {
    let listener = Listener::answering(automatic).expect("listener");
    let description = format!("entity={KIND}");
    let reserved: [&[&str]; 4] = [
        &["ENTITY"],
        &["person", "any"],
        &["None Of These"],
        &["--kind", &description],
    ];
    for options in reserved {
        quiet(&recognize(&listener, options, EVIDENCE), 2, RESERVED);
    }
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-reserved.json");
    fs::write(
        &path,
        format!(r#"{{"version":1,"recognize":{{"kinds":{{"entity":"{KIND}"}}}}}}"#),
    )
    .expect("question file");
    quiet(
        &recognize(&listener, &[&format!("@{}", path.display())], EVIDENCE),
        5,
        RESERVED,
    );
    assert_eq!(listener.connections(), 0);
}
