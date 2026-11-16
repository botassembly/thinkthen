#![cfg(test)]
//! Closed command output stops an active stream without waiting for input EOF.
#![cfg(all(unix, feature = "command"))]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
mod support;

use loopback::{HeldReply, KEY, Loopback};
use serde_json::json;
use std::io::Write;
use std::process::Stdio;
use std::sync::{Mutex, mpsc};
use std::time::Duration;

#[test]
#[allow(
    clippy::print_stderr,
    reason = "Loaded repetitions retain the event counts and command exit."
)]
fn an_output_write_error_stops_the_reading() {
    let timeout = Duration::from_secs(30);
    let (release, released) = mpsc::channel();
    let released = Mutex::new(released);
    let (arrived, arrivals) = mpsc::channel();
    let server = Loopback::answering(move |question| {
        let first = question["instructions"].as_str().unwrap().contains("first");
        arrived.send(first).unwrap();
        if first {
            released.lock().unwrap().recv_timeout(timeout).unwrap();
        }
        json!({"type": "noul", "noul": 0.9})
    })
    .unwrap();
    let (pending, closed) = server.hold_matching(HeldReply::Headers, |request| {
        String::from_utf8_lossy(&request.body).contains("pending")
    });
    let (stdout, inherited_reader) = support::closed_output();
    let mut child = support::command()
        .args([
            "decide", "Is it?", "--from", "lines", "--json", "--batch", "1",
        ])
        .env("THINKTHEN_BASE_URL", server.base())
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(stdout)
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    input.write_all(b"first\n").unwrap();
    assert!(arrivals.recv_timeout(timeout).unwrap());
    // Later rows can finish while the first row prevents any output write.
    for n in 2..=6 {
        writeln!(input, "message {n}").unwrap();
        assert!(!arrivals.recv_timeout(timeout).unwrap());
    }
    input
        .write_all("pending\n".repeat(1000).as_bytes())
        .unwrap();
    for _ in 0..3 {
        pending.recv_timeout(timeout).unwrap();
    }
    let before_output = server.count();
    assert!(before_output >= 9);
    release.send(()).unwrap();
    let (finished, completion) = mpsc::channel();
    std::thread::spawn(move || finished.send(child.wait_with_output().unwrap()).unwrap());
    // Keep stdin and backend replies open: neither may delay the broken-pipe exit.
    let output = completion.recv_timeout(timeout).unwrap();
    let total = server.count();
    let diagnostic = format!("before_output={before_output}, total={total}, output={output:?}");
    drop((input, inherited_reader));
    assert_eq!(output.status.code(), Some(141), "{diagnostic}");
    assert!(output.stderr.is_empty(), "{diagnostic}");
    for _ in 6..total {
        assert_eq!(
            closed.recv_timeout(timeout).unwrap().unwrap(),
            0,
            "{diagnostic}"
        );
    }
    // A worker bound cannot constrain arrivals before the first output write.
    // Every request beyond the six completed replies must close on command exit.
    eprintln!("{diagnostic}");
}
