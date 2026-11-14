#![cfg(test)]
//! How the command finishes: interrupts, closed output, drained rows and quiet stderr.
#![cfg(feature = "command")]
#![cfg(unix)]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

mod support;

use std::io::Write;
use std::process::{Command, Stdio};

use loopback::{KEY, Loopback};
use serde_json::Value;

#[test]
fn broken_stdout_exits_silently_with_141() {
    let (output, reader) = support::closed_output();
    let source = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(source.path(), "evidence").unwrap();
    let result = support::command()
        .args(["decide", "Ready?", "--backend", "stub", "--json"])
        .arg(source.path())
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
fn decide_and_filter_checks_drain_every_row_and_reduce_results() {
    for (function, probability, input, expected, kept) in [
        ("filter", 0.9, "one\ntwo\n", 0, "one\ntwo\n"),
        ("filter", 0.1, "one\ntwo\n", 1, ""),
        ("filter", 0.5, "one\ntwo\n", 1, ""),
        ("filter", 0.9, "", 1, ""),
        ("decide", 0.5, "one\n", 3, ""),
    ] {
        let server = Loopback::noul(probability).unwrap();
        let threshold = if function == "filter" {
            vec!["--threshold", "0.8", "--from", "lines"]
        } else {
            vec!["--low", "0.2", "--high", "0.8", "--json"]
        };
        let mut child = support::command()
            .args([
                function,
                "Ready?",
                "--url",
                server.base(),
                "--check",
                "--batch",
                "1",
            ])
            .args(threshold)
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
        if function == "filter" {
            assert_eq!(output.stdout, kept.as_bytes());
        } else {
            let row: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(row["status"], "unsure");
            assert_eq!(row["probability"], 0.5);
            assert!(row.get("answer").is_none());
        }
        assert_eq!(server.count(), input.lines().count());
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
