//! Host signal proofs: a host signal never fails a call on an engine worker.
//! The file-size proofs run in a child of this binary that skips command
//! setup, the way an embedding host would call the engine.

use std::io::{ErrorKind, Read as _, Write as _};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::Duration;

use nix::sys::pthread::{pthread_kill, pthread_self};
use nix::sys::signal::{SigSet, Signal};

use crate::core::Url;
use crate::core::recording::Exchange as Recorded;
use crate::engine::error::Error;
use crate::engine::http::{Client, Exchange, Key};
use crate::engine::recorder::{PreparedRecording, Recorder};
use crate::engine::{Cancel, Widths, workers};

const SECOND: Duration = Duration::from_secs(1);

/// Read one request through its body, then hold the reply until released.
fn held_reply() -> (
    String,
    Receiver<()>,
    Sender<()>,
    thread::JoinHandle<TcpListener>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let url = format!(
        "http://{}/v1/systemone",
        listener.local_addr().expect("address")
    );
    let (holding, held) = channel();
    let (release, released) = channel::<()>();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("request");
        let mut request = Vec::new();
        let mut chunk = [0_u8; 4096];
        while !request.ends_with(b"\r\n\r\n{}") {
            let read = stream.read(&mut chunk).expect("request bytes");
            assert!(read > 0, "the request ended early");
            request.extend_from_slice(chunk.get(..read).expect("read length"));
        }
        holding.send(()).expect("hold announced");
        released.recv().expect("release");
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 2\r\n\r\n{}")
            .expect("response");
        listener
    });
    (url, held, release, server)
}

#[test]
fn a_host_signal_during_a_held_send_on_a_worker_leaves_the_call_whole() {
    let handled = Arc::new(AtomicBool::new(false));
    let _handler =
        signal_hook::flag::register(signal_hook::consts::signal::SIGUSR1, Arc::clone(&handled))
            .expect("a no-op host handler");
    let (url, held, release, server) = held_reply();
    let widths: &'static Widths = Box::leak(Box::default());
    let client = Client::new(SECOND * 5, false, crate::engine::process_width()).gated(widths);
    let key = Key::of("sk-test-value");
    let exchange = Exchange {
        url: &url,
        body: b"{}",
        key: &key,
        max_retries: 2,
        retry_wait: Duration::from_millis(10),
    };
    let (published, worker) = channel();
    let (results, answers) = channel();

    workers::scoped_observed(
        1,
        results,
        &|()| client.post_observed(&exchange, &Cancel::default(), || ()),
        &|| published.send(pthread_self()),
        |work| {
            work.send(()).expect("work queued");
            let worker = worker.recv_timeout(SECOND * 2).expect("worker thread");
            held.recv_timeout(SECOND * 2).expect("the send is held");
            // Several sends, so an unmasked worker meets one inside its read.
            for _ in 0..10 {
                pthread_kill(worker, Signal::SIGUSR1).expect("signal delivered");
                thread::sleep(Duration::from_millis(10));
            }
            release.send(()).expect("reply released");
        },
    );

    // A blocked signal stays pending on the worker and ends with it, so the
    // handler never runs. This holds however the scheduler times the sends.
    assert!(
        !handled.load(Ordering::SeqCst),
        "the worker kept SIGUSR1 blocked"
    );
    let calling = SigSet::thread_get_mask().expect("calling thread mask");
    assert!(
        !calling.contains(Signal::SIGUSR1),
        "the calling thread keeps the host's mask"
    );
    let answer = answers
        .recv()
        .expect("one result")
        .expect("the normal answer");
    assert_eq!(answer.body, b"{}");
    assert_eq!(answer.requests_sent, 1);
    let listener = server.join().expect("listener");
    listener.set_nonblocking(true).expect("nonblocking");
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock),
        "the listener counts exactly one send"
    );
}

/// Run `file_size_child` under a one-block file-size limit.
fn limited_child(mode: &str) -> std::process::Output {
    crate::test_deadline::output(
        crate::test_deadline::child::command("sh", &[])
            .arg("-c")
            .arg(concat!(
                "ulimit -f 1; exec \"$0\" --exact ",
                "engine::host_signal_tests::file_size_child --ignored --nocapture --quiet"
            ))
            .arg(std::env::current_exe().expect("test binary"))
            .env("THINKTHEN_HOST_XFSZ", mode),
    )
    .expect("limited child")
}

#[test]
fn a_host_sigxfsz_action_stays_installed_through_a_recording() {
    let output = limited_child("handled");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{output:?}");
    assert!(
        stdout
            .lines()
            .any(|line| line == "storage failure; host action ran; still installed"),
        "{stdout}"
    );
}

#[test]
fn a_host_with_the_default_sigxfsz_action_is_killed_by_it() {
    use std::os::unix::process::ExitStatusExt as _;

    let output = limited_child("default");
    assert_eq!(
        output.status.signal(),
        Some(Signal::SIGXFSZ as i32),
        "{output:?}"
    );
}

#[test]
#[ignore = "subprocess harness"]
fn file_size_child() {
    let Ok(mode) = std::env::var("THINKTHEN_HOST_XFSZ") else {
        return;
    };
    let ran = Arc::new(AtomicBool::new(false));
    if mode == "handled" {
        let _action =
            signal_hook::flag::register(signal_hook::consts::signal::SIGXFSZ, Arc::clone(&ran))
                .expect("host action");
    }
    let folder = std::env::temp_dir().join(format!("thinkthen-host-xfsz-{}", std::process::id()));
    let recorder = Recorder::of(Some(&folder), None).expect("recorder");
    let url = Url::new("http://127.0.0.1:1/v1/systemone").expect("url");
    let exchange = Recorded::new(&url, b"{}");
    let digest = exchange.digest();
    let Ok(PreparedRecording::Live(permit)) = recorder.prepare(&exchange, &digest) else {
        panic!("a live write permit")
    };
    let response = format!("{{\"pad\":\"{}\"}}", "x".repeat(8_192));

    let name = digest.file_name();
    let (results, finished) = channel();
    // The write runs on an engine worker, whose mask must leave SIGXFSZ open.
    workers::scoped_observed(
        1,
        results,
        &|permit: crate::engine::recorder::WritePermit| {
            permit.finish(&exchange, response.as_bytes(), &name)
        },
        &|| (),
        |work| work.send(permit).expect("write queued"),
    );
    let result = finished.recv().expect("one write");

    assert!(matches!(result, Err(Error::RecordingStorage)), "{result:?}");
    assert!(ran.swap(false, Ordering::SeqCst), "the host action ran");
    nix::sys::signal::raise(Signal::SIGXFSZ).expect("raise");
    assert!(
        ran.load(Ordering::SeqCst),
        "the host action is still installed"
    );
    let left: Vec<_> = std::fs::read_dir(&folder)
        .expect("folder")
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
        .map(|entry| entry.file_name())
        .filter(|name| name != ".thinkthen-backend.json")
        .collect();
    assert!(left.is_empty(), "no final or temporary entry: {left:?}");
    std::fs::remove_dir_all(&folder).expect("cleanup");
    let mut stdout = std::io::stdout().lock();
    writeln!(stdout, "storage failure; host action ran; still installed").expect("announce");
}
