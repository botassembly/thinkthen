#![cfg(feature = "command")]
#![cfg(test)]
//! Function command questions remain literal even when they resemble filenames.
mod support;
use std::io::Write;
use std::process::Stdio;

#[test]
fn filename_and_hash_question_strings_are_literal() {
    for question in ["missing.json", "missing.json#ready"] {
        let mut child = support::command()
            .args(["decide", question, "--backend", "stub"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(b"alpha").unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(
            matches!(output.status.code(), Some(0 | 1 | 3)),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stderr.is_empty());
    }
}
