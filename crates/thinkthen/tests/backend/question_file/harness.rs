//! The pieces every page of this binary shares: files, runs, and reads.

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::thread;

/// Write one question file under the test target directory and name its path.
///
/// Two tests drive the same table at once, so the write is made atomic and a
/// reader sees a whole file. Each case owns its name.
pub(crate) fn written(name: &str, text: &str) -> String {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("question-file");
    let _ = fs::create_dir_all(&folder);
    let path = folder.join(format!("{name}.json"));
    // Two tests write the same case at the same time, as threads of one
    // process or as nextest processes, so the text goes under a name of this
    // process and thread's own and moves into place in one step. A reader
    // then sees the whole file or the whole earlier one.
    let staged = folder.join(format!(
        "{name}-{}-{:?}.part",
        std::process::id(),
        thread::current().id()
    ));
    let _ = fs::write(&staged, text);
    let _ = fs::rename(&staged, &path);
    format!("@{}", path.display())
}

/// Run the binary with no environment over the evidence the case names.
pub(crate) fn run(arguments: &[&str], evidence: &[u8]) -> io::Result<Output> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .env("THINKTHEN_TEST_INPUT_PAUSE_MS", "10000")
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut input = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no pipe to standard input"))?;
    let _ = input.write_all(evidence);
    drop(input);
    super::wait::finish(child, &arguments.join(" "))
}

/// A port nothing listens on, so a connection would be refused at once.
pub(crate) const CLOSED: &str = "http://127.0.0.1:1/v1";

/// Run the binary with a key set, over the evidence the case names.
pub(crate) fn run_with(arguments: &[&str], evidence: &[u8], key: &str) -> io::Result<Output> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("HOME", env!("CARGO_TARGET_TMPDIR"))
        .env("THINKTHEN_TEST_INPUT_PAUSE_MS", "10000")
        .env("THINKTHEN_API_KEY", key)
        .env("THINKTHEN_TEST_RETRY_WAIT_MS", "1")
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut input = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no pipe to standard input"))?;
    let _ = input.write_all(evidence);
    drop(input);
    super::wait::finish(child, &arguments.join(" "))
}

/// What a case reads when the binary never ran, so the failure says which.
pub(crate) const UNRUN: &str = "the compiled binary did not run";

/// Run one command over one line of evidence and read back its standard output.
pub(crate) fn printed(arguments: &[&str]) -> String {
    printed_over(arguments, b"Refund me please.")
}

/// Run one command over the evidence it names, and read back its output.
pub(crate) fn printed_over(arguments: &[&str], evidence: &[u8]) -> String {
    let Ok(output) = run(arguments, evidence) else {
        return UNRUN.to_owned();
    };
    assert_eq!(
        output.status.code(),
        Some(0),
        "{arguments:?} {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Run one command that is refused, and read back its message and its code.
pub(crate) fn refused(arguments: &[&str]) -> (String, Option<i32>) {
    let Ok(output) = run(arguments, b"Refund me please.") else {
        return (UNRUN.to_owned(), None);
    };
    assert!(output.stdout.is_empty(), "{arguments:?}");
    (
        String::from_utf8_lossy(&output.stderr).into_owned(),
        output.status.code(),
    )
}

/// The file every override case starts from.
pub(crate) const REFUND: &str = concat!(
    r#"{"decide":"Does this message ask for a refund?","#,
    r#""true":"The writer asks for money back.","#,
    r#""false":"The writer asks for anything else.","#,
    r#""threshold":"0.2:0.8","model":"jev-1.13.0"}"#,
);
