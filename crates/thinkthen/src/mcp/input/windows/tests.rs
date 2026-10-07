//! Real owned subprocess pipes; these tests require a Windows runner.
use super::{PipeInput, PipeOutput};
use crate::CancelToken;
use crate::test_deadline::child::ChildEnvironment as _;
use std::fs::File;
use std::io::{Read, Write};
use std::os::windows::io::OwnedHandle;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const BOUND: Duration = Duration::from_secs(3);
struct Peer(Child);
impl Peer {
    fn new() -> Self {
        Self(
            Command::new(std::env::current_exe().unwrap())
                .clear_environment()
                .args([
                    "--exact",
                    "mcp::input::windows::tests::pipe_peer",
                    "--nocapture",
                ])
                .env("THINKTHEN_MCP_PIPE_PEER", "1")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .unwrap(),
        )
    }
}
impl Drop for Peer {
    fn drop(&mut self) {
        let _killed = self.0.kill();
        let _joined = self.0.wait();
    }
}

#[test]
fn pipe_peer() {
    if std::env::var_os("THINKTHEN_MCP_PIPE_PEER").is_none() {
        return;
    }
    // Parent proves readiness through this byte, then leaves stdin unread.
    std::io::stdout()
        .write_all(b"partial-frame-marker")
        .unwrap();
    std::io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(30));
}

#[test]
fn blocked_input_is_cancelled_and_joined_while_owned_peer_remains_open() {
    let mut peer = Peer::new();
    let input = File::from(OwnedHandle::from(peer.0.stdout.take().unwrap()));
    let stop = CancelToken::new();
    let token = stop.clone();
    let (ready, started) = mpsc::channel();
    let (sent, done) = mpsc::channel();
    let reader = thread::spawn(move || {
        let mut input = PipeInput::new(input, token).unwrap();
        let mut bytes = [0; 4096];
        let mut observed = Vec::new();
        while !observed
            .windows(20)
            .any(|one| one == b"partial-frame-marker")
        {
            let size = input.read(&mut bytes).unwrap();
            assert_ne!(size, 0);
            observed.extend_from_slice(&bytes[..size]);
        }
        ready.send(()).unwrap();
        let result = input.read(&mut bytes);
        drop(input); // Joins the cancellation watcher before reporting done.
        sent.send(result).unwrap();
    });
    started.recv_timeout(BOUND).unwrap();
    assert!(done.recv_timeout(Duration::from_millis(100)).is_err());
    stop.cancel();
    assert_eq!(done.recv_timeout(BOUND).unwrap().unwrap(), 0);
    reader.join().unwrap();
    assert!(peer.0.try_wait().unwrap().is_none());
}

#[test]
fn backpressured_output_is_cancelled_and_joined_without_draining_peer() {
    let mut peer = Peer::new();
    let output = File::from(OwnedHandle::from(peer.0.stdin.take().unwrap()));
    let stop = CancelToken::new();
    let token = stop.clone();
    let (ready, started) = mpsc::channel();
    let (sent, done) = mpsc::channel();
    let writer = thread::spawn(move || {
        let mut output = PipeOutput::new(output, token).unwrap();
        ready.send(()).unwrap();
        let result = output.write_all(&vec![b'x'; 2 * 1024 * 1024]);
        drop(output);
        sent.send(result).unwrap();
    });
    started.recv_timeout(BOUND).unwrap();
    assert!(done.recv_timeout(Duration::from_millis(100)).is_err());
    stop.cancel();
    assert_eq!(
        done.recv_timeout(BOUND).unwrap().unwrap_err().kind(),
        std::io::ErrorKind::BrokenPipe
    );
    writer.join().unwrap();
    assert!(peer.0.try_wait().unwrap().is_none());
}

#[test]
fn ordinary_files_are_refused_before_any_blocking_io() {
    let path = std::env::current_exe().unwrap();
    let failure = PipeInput::new(File::open(path).unwrap(), CancelToken::new()).unwrap_err();
    assert_eq!(failure.kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
fn owned_peer_exit_becomes_input_eof_and_an_observed_output_error() {
    let mut peer = Peer::new();
    let mut input = PipeInput::new(
        File::from(OwnedHandle::from(peer.0.stdout.take().unwrap())),
        CancelToken::new(),
    )
    .unwrap();
    let mut output = PipeOutput::new(
        File::from(OwnedHandle::from(peer.0.stdin.take().unwrap())),
        CancelToken::new(),
    )
    .unwrap();
    peer.0.kill().unwrap();
    peer.0.wait().unwrap();
    let mut bytes = Vec::new();
    input.read_to_end(&mut bytes).unwrap();
    let failure = output.write_all(b"protocol\n").unwrap_err();
    assert_eq!(failure.kind(), std::io::ErrorKind::BrokenPipe);
    drop(input);
    drop(output);
}
