//! The input pause: an open batch goes out when piped input pauses for 50 ms,
//! and a test's longer pause holds it until input ends (Debt 030).

use crate::child::ChildEnvironment as _;
use std::io::Write as _;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use super::{KEY, QUESTION, answering, folder, lines, places, rows, text};
use crate::harness::{Listener, finish};

/// A pause row: its name, its extra arguments and its environment.
type Mode<'a> = (&'a str, &'a [&'a str], &'a [(&'a str, &'a str)]);

/// Start `decide` on lines, write three records and keep the pipe open.
fn held(listener: &Listener, extra: &[&str], environment: &[(&str, &str)]) -> (Child, ChildStdin) {
    let fixed = ["decide", QUESTION, "--lines", "--url", listener.base()];
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .clear_environment()
        .home(folder("home"))
        .env(KEY.0, KEY.1)
        .envs(environment.iter().copied())
        .args(fixed)
        .args(["--model", "jev-1.13.0"])
        .args(extra)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the command starts");
    let mut writer = child.stdin.take().expect("an input pipe");
    writer
        .write_all(lines(1..=3).as_bytes())
        .expect("three records are written");
    writer.flush().expect("the records reach the pipe");
    (child, writer)
}

#[test]
fn a_pause_sends_the_open_batch() {
    let typed = folder("typed");
    let named = folder("named");
    let modes: [Mode<'_>; 3] = [
        ("no folder", &["--no-cache"], &[]),
        ("the default cache", &[], &[("THINKTHEN_CACHE", &named)]),
        ("a typed folder", &["--cache", &typed], &[]),
    ];
    for (name, extra, environment) in modes {
        let listener = Listener::answering(answering).expect("a loopback listener");
        let (child, writer) = held(&listener, extra, environment);
        // The count moves before the body is recorded, so wait on the
        // recorded request itself (ticket 0352).
        let deadline = Instant::now() + Duration::from_secs(30);
        let mut sent = listener.requests();
        while sent.is_empty() && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(10));
            sent = listener.requests();
        }
        drop(writer);
        let output = finish(child, name).expect("the command ends");
        assert_eq!(sent.len(), 1, "{name}: a request while the pipe stays open");
        assert_eq!(places(&sent[0].body).len(), 3, "{name}");
        assert_eq!(text(&output.stdout), rows(1..=3), "{name}");
    }
}

/// The harness sets this pause so a stalled reader thread never closes a
/// batch early. No request can go out while it holds, so the zero count
/// after 300 ms never depends on timing.
#[test]
fn a_test_pause_holds_the_open_batch_until_input_ends() {
    let listener = Listener::answering(answering).expect("a loopback listener");
    let pause = [("THINKTHEN_TEST_INPUT_PAUSE_MS", "10000")];
    let (child, writer) = held(&listener, &["--no-cache"], &pause);
    thread::sleep(Duration::from_millis(300));
    assert_eq!(listener.count(), 0, "no request while the test pause holds");
    drop(writer);
    let output = finish(child, "held").expect("the command ends");
    let sent = listener.requests();
    assert_eq!(sent.len(), 1, "end of input sends the open batch");
    assert_eq!(places(&sent[0].body).len(), 3);
    assert_eq!(text(&output.stdout), rows(1..=3));
}
