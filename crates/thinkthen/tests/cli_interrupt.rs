#![cfg(feature = "command")]
#![cfg(test)]
#![cfg(unix)]
//! Interrupt handling through the command.

mod support;

use std::io::{BufRead, BufReader, Write};
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
    let row: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(row["error"]["code"], "local.cancelled");
    assert!(output.stderr.is_empty());
    assert_eq!(server.count(), 1);
}

#[test]
fn two_interrupts_exit_130_with_input_still_open() {
    let (server, received, closed) = Loopback::holding(HeldReply::Headers).unwrap();
    let mut child = command(&server);
    let mut input = child.stdin.take().unwrap();
    // The next record flushes the first batch while input remains open.
    input.write_all(b"one\ntwo\n").unwrap();
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
            "--lines",
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
