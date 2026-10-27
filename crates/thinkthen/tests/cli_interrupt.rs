#![cfg(test)]
//! Interrupt handling through the command.
#![cfg(feature = "command")]
#![cfg(unix)]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

mod support;

use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};

use loopback::{HeldReply, KEY, Loopback};

#[test]
fn one_interrupt_closes_the_open_request_and_exits_130() {
    let (server, received, closed) = Loopback::holding(HeldReply::Headers).unwrap();
    let mut child = command(&server);
    child.stdin.take().unwrap().write_all(b"one\n").unwrap();
    received.recv().unwrap();
    interrupt_and_observe(&mut child);
    assert_peer_closed(closed.recv().unwrap());
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(130));
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
    assert_eq!(server.count(), 1);
}

#[test]
fn two_interrupts_exit_130_with_input_still_open() {
    let (server, received, closed) = Loopback::holding(HeldReply::Headers).unwrap();
    let mut child = command(&server);
    let mut input = child.stdin.take().unwrap();
    // The push driver flushes an available record while input remains open.
    input.write_all(b"one\n").unwrap();
    received.recv().unwrap();
    interrupt_and_observe(&mut child);
    assert!(child.try_wait().unwrap().is_none());
    interrupt(child.id());
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(130));
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
    assert_eq!(server.count(), 1);
    assert_peer_closed(closed.recv().unwrap());
    drop(input);
}

#[test]
fn cancellation_exits_while_the_notice_sink_is_full() {
    use std::os::fd::OwnedFd;
    use std::os::unix::net::UnixStream;

    let (server, received, closed) = Loopback::holding(HeldReply::Headers).unwrap();
    let (_reader, mut writer) = UnixStream::pair().unwrap();
    writer.set_nonblocking(true).unwrap();
    loop {
        match writer.write(&[0; 8192]) {
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
            Err(error) => panic!("could not fill stderr: {error}"),
        }
    }
    writer.set_nonblocking(false).unwrap();
    let mut child = support::command()
        .args([
            "decide",
            "Relevant?",
            "--from",
            "lines",
            "--batch",
            "1",
            "--url",
            server.base(),
        ])
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::from(OwnedFd::from(writer)))
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"one\n").unwrap();
    received.recv().unwrap();
    interrupt(child.id());
    // Observe cancellation without reading bytes from the full notice sink.
    assert_peer_closed(closed.recv().unwrap());
    assert_eq!(child.wait().unwrap().code(), Some(130));
}

fn assert_peer_closed(ending: std::io::Result<usize>) {
    assert!(
        matches!(ending, Ok(0))
            || matches!(ending, Err(ref error) if error.kind() == std::io::ErrorKind::ConnectionReset),
        "cancelled connection remained open: {ending:?}"
    );
}

fn command(server: &Loopback) -> Child {
    support::command()
        .args([
            "decide",
            "Is it relevant?",
            "--from",
            "lines",
            "--json",
            "--batch",
            "1",
            "--url",
            server.base(),
        ])
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}

fn interrupt(pid: u32) {
    assert!(
        Command::new("kill")
            .args(["-INT", &pid.to_string()])
            .status()
            .unwrap()
            .success()
    );
}

fn interrupt_and_observe(child: &mut Child) {
    interrupt(child.id());
    let mut notice = String::new();
    BufReader::new(child.stderr.as_mut().unwrap())
        .read_line(&mut notice)
        .unwrap();
    assert_eq!(
        notice,
        "thinkthen: cancelling; press Ctrl-C again to exit\n"
    );
}

#[test]
fn completion_cancellation_precedes_a_later_broken_stdout() {
    use std::os::fd::OwnedFd;
    use std::os::unix::net::UnixStream;
    for saved in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let record =
            serde_json::json!({"id": "x".repeat(4 * 1024 * 1024), "body": "evidence"}).to_string();
        let source = directory.path().join("records.jsonl");
        std::fs::write(&source, format!("{record}\n")).unwrap();
        let entry = directory.path().join("run.json");
        std::fs::write(
            &entry,
            serde_json::json!({"run":[{"shell":["/bin/cat", source]}]}).to_string(),
        )
        .unwrap();
        let server = Loopback::noul(0.9).unwrap();
        let (mut reader, writer) = UnixStream::pair().unwrap();
        let mut command = support::command();
        if saved {
            command.args(["run", "--allow-shell"]).arg(&entry);
        } else {
            command.args([
                "filter",
                "Keep?",
                "--from",
                "jsonl",
                "--json",
                "--evidence",
                "/body",
            ]);
        }
        let mut child = command
            .env("THINKTHEN_API_KEY", KEY)
            .env("THINKTHEN_BASE_URL", server.base())
            .stdin(Stdio::piped())
            .stdout(Stdio::from(OwnedFd::from(writer)))
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut input = child.stdin.take().unwrap();
        if !saved {
            writeln!(input, "{record}").unwrap();
        }
        drop(input);
        // A partial large row proves that stdout is being written. Keep its
        // reader open until the interrupt handler confirms cancellation.
        reader.read_exact(&mut [0]).unwrap();
        interrupt_and_observe(&mut child);
        drop(reader);
        let output = child.wait_with_output().unwrap();
        assert_eq!(
            output.status.code(),
            Some(130),
            "saved={saved}, stderr={:?}",
            output.stderr
        );
    }
}
