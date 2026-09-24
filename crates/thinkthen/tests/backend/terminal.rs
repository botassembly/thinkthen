//! The one line a user sitting at a terminal is told, and its absence in a pipe.

use std::fs;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::harness::{finish, spawn};

/// The line a run reading from a terminal writes on standard error.
const WAITING: &str =
    "thinkthen: reading evidence from the terminal; end it with Ctrl-D on a line of its own\n";

/// A folder this page owns, remade so each run starts empty.
fn folder(name: &str) -> io::Result<PathBuf> {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    fs::create_dir_all(&path)?;
    Ok(path)
}

/// Run the binary with standard input on a pseudo-terminal, through `script`.
///
/// `script` is the smallest honest pseudo-terminal: it ships with util-linux
/// and with the BSD tools, so the two spellings below cover Linux and macOS,
/// and `sdlc/scripts/install` names it. Nothing here skips when neither works,
/// because a check that skips rots. Standard output and standard error are
/// redirected inside the terminal, so the terminal's own echo of the typed
/// evidence never reaches either file.
fn through_a_terminal(
    arguments: &[&str],
    evidence: &str,
    into: &Path,
) -> io::Result<(String, String)> {
    let out = into.join("out");
    let err = into.join("err");
    let quoted = arguments
        .iter()
        .map(|argument| format!("'{}'", argument.replace('\'', r"'\''")))
        .collect::<Vec<_>>()
        .join(" ");
    let inner = format!(
        "{} {quoted} >{} 2>{}",
        env!("CARGO_BIN_EXE_thinkthen"),
        out.display(),
        err.display(),
    );
    let spellings: [Vec<&str>; 2] = [
        vec!["-q", "-e", "-c", &inner, "/dev/null"],
        vec!["-q", "/dev/null", "sh", "-c", &inner],
    ];

    for spelling in spellings {
        let mut child = Command::new("script")
            .args(&spelling)
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        let mut typed = child
            .stdin
            .take()
            .ok_or_else(|| io::Error::other("no pipe into the terminal"))?;
        let _typed = typed.write_all(evidence.as_bytes());
        drop(typed);
        if finish(child, "the terminal run")?.status.success() && out.exists() && err.exists() {
            return Ok((fs::read_to_string(&out)?, fs::read_to_string(&err)?));
        }
    }
    Err(io::Error::other(
        "neither spelling of `script` gave the binary a pseudo-terminal",
    ))
}

/// A terminal is told what the command waits for, and a pipe is told nothing.
#[test]
fn a_terminal_is_told_what_the_command_waits_for_and_a_pipe_is_not() {
    let into = folder("terminal").expect("a folder for this case");
    let asked = ["decide", "asks for a refund", "--dry-run"];

    let (out, err) =
        through_a_terminal(&asked, "Refund me please.\n", &into).expect("a pseudo-terminal");

    assert_eq!(err, WAITING);
    assert!(
        out.starts_with(r#"{"url":"https://api.typesafe.ai/v1/systemone""#),
        "{out}"
    );

    let piped = spawn(&asked, &[], b"Refund me please.\n").expect("the compiled binary runs");
    assert_eq!(String::from_utf8_lossy(&piped.stderr), "");
    assert_eq!(piped.status.code(), Some(0));
}

/// A file named by `--input` is not a terminal, whatever standard input is.
#[test]
fn a_run_reading_a_file_is_told_nothing_even_at_a_terminal() {
    let into = folder("terminal-input").expect("a folder for this case");
    let evidence = into.join("evidence.txt");
    fs::write(&evidence, "Refund me please.\n").expect("the evidence is written");
    let path = evidence.to_string_lossy().into_owned();
    let asked = ["decide", "asks for a refund", "--dry-run", "--input", &path];

    let (out, err) = through_a_terminal(&asked, "", &into).expect("a pseudo-terminal");

    assert_eq!(err, "");
    assert!(out.contains(r#""state":"Refund me please.\n""#), "{out}");
}
