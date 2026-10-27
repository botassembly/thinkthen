#![cfg(test)]
//! Command completion expectations from the approved design. No case is skipped
//! while the shared engine and command implementation catch up.
#![cfg(feature = "command")]
#![cfg(unix)]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

mod support;

use std::io::Write;
use std::process::{Command, Stdio};

use loopback::{HeldReply, KEY, Loopback};
use serde_json::Value;

#[test]
fn first_interrupt_cancels_a_held_request_on_each_backend() {
    for backend in ["systemone", "openai"] {
        let (server, received, closed) = Loopback::holding(HeldReply::Headers).unwrap();
        let mut child = support::command()
            .args([
                "filter",
                "Ready?",
                "--backend",
                backend,
                "--url",
                server.base(),
                "--batch",
                "1",
                "--from",
                "lines",
                "--json",
            ])
            .env(
                if backend == "openai" {
                    "OPENAI_API_KEY"
                } else {
                    "THINKTHEN_API_KEY"
                },
                KEY,
            )
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(b"one\n").unwrap();
        received.recv().unwrap();
        assert!(
            Command::new("kill")
                .args(["-INT", &child.id().to_string()])
                .status()
                .unwrap()
                .success()
        );
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(130), "{backend}: {output:?}");
        // Cancellation is a call error; it must not synthesize a failed row.
        assert!(output.stdout.is_empty());
        assert!(!String::from_utf8_lossy(&output.stderr).contains(KEY));
        let ending = closed.recv().unwrap();
        assert!(
            matches!(ending, Ok(0))
                || matches!(ending, Err(ref error) if error.kind() == std::io::ErrorKind::ConnectionReset)
        );
        assert_eq!(server.count(), 1);
    }
}

#[test]
fn broken_stdout_exits_silently_with_141() {
    let (output, reader) = support::closed_output();
    let result = support::command()
        .args(["decide", "Ready?", "--backend", "stub", "--json"])
        .stdin(Stdio::null())
        .stdout(output)
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    drop(reader);
    assert_eq!(result.status.code(), Some(141));
    assert!(result.stderr.is_empty());
}

#[test]
fn filter_check_drains_every_row_and_reduces_kept_unsure_and_empty_results() {
    for (probability, input, expected, status) in [
        (0.9, "one\ntwo\n", 0, "answered"),
        (0.1, "one\ntwo\n", 1, "answered"),
        (0.5, "one\ntwo\n", 3, "unsure"),
        (0.9, "", 1, "answered"),
    ] {
        let server = Loopback::noul(probability).unwrap();
        let mut child = support::command()
            .args([
                "filter",
                "Ready?",
                "--url",
                server.base(),
                "--check",
                "--json",
                "--low",
                "0.2",
                "--high",
                "0.8",
                "--batch",
                "1",
            ])
            .env("THINKTHEN_API_KEY", KEY)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(expected), "{output:?}");
        let actual: Vec<Value> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(actual.len(), input.lines().count());
        for row in actual {
            assert_eq!(row["status"], status);
            assert_eq!(row["kept"], probability > 0.8);
            if status == "unsure" {
                assert!(row.get("answer").is_none());
            }
        }
    }
}

#[test]
fn redirected_stderr_has_no_progress_and_verbose_keeps_json_clean() {
    let mut child = support::command()
        .args([
            "decide",
            "Ready?",
            "--backend",
            "stub",
            "--from",
            "lines",
            "--json",
            "--verbose",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"one\ntwo\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let rows: Vec<Value> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(rows.len(), 2);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains('\r'));
    assert!(!stderr.contains('\u{1b}'));
    assert!(!stderr.contains(KEY));
}

#[test]
fn first_interrupt_finishes_while_an_input_writer_remains_open() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("waiting.txt");
    assert!(
        Command::new("mkfifo")
            .arg(&path)
            .status()
            .unwrap()
            .success()
    );
    let child = support::command()
        .args(["decide", "Ready?", "--backend", "stub"])
        .arg(&path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // Opening the writer proves that the child installed its handler and opened
    // the reader. Keep it open without sending bytes until cancellation ends.
    let writer = std::fs::OpenOptions::new().write(true).open(&path).unwrap();
    assert!(
        Command::new("kill")
            .args(["-INT", &child.id().to_string()])
            .status()
            .unwrap()
            .success()
    );
    let output = child.wait_with_output().unwrap();
    drop(writer);
    assert_eq!(output.status.code(), Some(130));
    assert!(output.stdout.is_empty());
}

#[test]
fn first_interrupt_finishes_table_stdin_before_the_writer_closes() {
    let script = r#"
import os, pty, signal, subprocess, sys
master, slave = pty.openpty()
child = subprocess.Popen([sys.argv[1], 'decide', 'Ready?', '--backend', 'stub', '--from', 'csv', '--column', 'body', '--into', 'ready'], stdin=slave, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env={})
os.close(slave)
assert child.stderr.readline().decode().strip() == 'reading standard input; press Ctrl-D to end'
child.send_signal(signal.SIGINT)
stdout, stderr = child.communicate(timeout=10)
os.close(master)
assert child.returncode == 130, (child.returncode, stdout, stderr)
assert not stdout
"#;
    let result = Command::new("python3")
        .env_clear()
        .args(["-c", script, env!("CARGO_BIN_EXE_thinkthen")])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn completed_rows_arrive_before_an_open_writer_sends_its_next_record() {
    use std::io::{BufRead, BufReader};
    let directory = tempfile::tempdir().unwrap();
    let saved = directory.path().join("pipeline.json");
    std::fs::write(
        &saved,
        serde_json::json!({"run":[
            {"question":{"function":"decide","question":"Ready?"}},
            {"shell":["cat"]}
        ]})
        .to_string(),
    )
    .unwrap();
    for arguments in [
        vec!["decide", "Ready?"],
        vec!["run", "--allow-shell", saved.to_str().unwrap()],
    ] {
        let server = Loopback::noul(0.9).unwrap();
        let mut child = support::command()
            .args(arguments)
            .args([
                "--url",
                server.base(),
                "--batch",
                "1",
                "--from",
                "lines",
                "--json",
            ])
            .env("THINKTHEN_API_KEY", KEY)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut writer = child.stdin.take().unwrap();
        let mut reader = BufReader::new(child.stdout.take().unwrap());
        for position in 1..=2 {
            writer.write_all(b"evidence\n").unwrap();
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let row: Value = serde_json::from_str(&line).unwrap();
            assert_eq!(row["position"], position);
            assert_eq!(row["answer"], true);
        }
        drop(writer);
        assert!(child.wait().unwrap().success());
    }
}
