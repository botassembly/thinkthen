//! A harmless signal that lands on the engine's own worker thread while it
//! waits for the reply does not fail the call (surfaces-review-5, the
//! verifier's worker-thread probe). A host may install a handler without
//! `SA_RESTART` (a child-exit handler, a profiler timer), and the kernel
//! may deliver a process signal to any thread. The read resumes on the
//! same connection: the call answers, and the stub reads one body. Only a
//! token that fired turns the interruption into the cancelled kind.
//!
//! One test per file, because the engine reads the environment once per
//! process.

mod common;

use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};

use thinkthen_contract::{Cancel, Engine, ErrorKind, Options, Question};
use thinkthen_standin::BlockingEngine;

const REPLY: &str = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.97}},"usage":{"input_tokens":10,"output_tokens":2}}"#;
const DELAY: Duration = Duration::from_millis(800);

/// Serve one keep-alive connection: every body read is recorded before the
/// delay, so a request abandoned mid-wait still counts.
fn serve(stream: TcpStream, bodies: &Mutex<Vec<String>>) {
    let mut reader = BufReader::new(stream.try_clone().expect("clones"));
    let mut writer = stream;
    while let Some(body) = common::read_request(&mut reader) {
        bodies.lock().expect("the list").push(String::from_utf8_lossy(&body).into_owned());
        std::thread::sleep(DELAY);
        let head = format!(
            "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n",
            REPLY.len()
        );
        if writer.write_all(head.as_bytes()).is_err() || writer.write_all(REPLY.as_bytes()).is_err() {
            return;
        }
    }
}

/// The bodies whose evidence carries `marker`.
fn count(bodies: &Mutex<Vec<String>>, marker: &str) -> usize {
    bodies.lock().expect("the list").iter().filter(|body| body.contains(marker)).count()
}

/// Signals the handler ran; the proof that a signal landed.
static HANDLED: AtomicUsize = AtomicUsize::new(0);

extern "C" fn ignore(_: libc::c_int) {
    HANDLED.fetch_add(1, Ordering::Relaxed);
}

/// Send SIGUSR1 to every engine worker thread in this process; return how
/// many were signalled.
fn signal_the_workers() -> usize {
    let mut sent = 0;
    for task in std::fs::read_dir("/proc/self/task").expect("the task list").flatten() {
        let name = std::fs::read_to_string(task.path().join("comm")).unwrap_or_default();
        let Ok(tid) = task.file_name().to_string_lossy().parse::<libc::pid_t>() else {
            continue;
        };
        if name.starts_with("ttb-worker") {
            unsafe { libc::syscall(libc::SYS_tgkill, libc::getpid(), tid, libc::SIGUSR1) };
            sent += 1;
        }
    }
    sent
}

#[test]
fn a_harmless_signal_on_the_worker_resumes_the_read() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("binds");
    let port = listener.local_addr().expect("an address").port();
    let bodies = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&bodies);
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let seen = Arc::clone(&seen);
            std::thread::spawn(move || serve(stream, &seen));
        }
    });
    // Sound in this binary: the test owns the process, and no engine call
    // runs before this set.
    unsafe {
        std::env::remove_var("THINKTHEN_NULL");
        std::env::remove_var("ENGINE_NULL");
        std::env::set_var("THINKTHEN_BASE_URL", format!("http://127.0.0.1:{port}/v1"));
        let mut action: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = ignore as *const () as usize;
        libc::sigemptyset(&mut action.sa_mask);
        libc::sigaction(libc::SIGUSR1, &action, std::ptr::null_mut());
    }
    let tt = BlockingEngine::from_env();
    let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("parses");

    // Nobody cancelled: the read resumes and the call answers.
    let signaller = std::thread::spawn(|| {
        std::thread::sleep(Duration::from_millis(300));
        signal_the_workers()
    });
    let answered = tt.decide_many(&question, &["harmless-signal"], None);
    let signalled = signaller.join().expect("the signaller joins");
    std::thread::sleep(DELAY * 2);
    assert!(signalled >= 1, "the probe reached a worker thread");
    assert!(HANDLED.load(Ordering::Relaxed) >= 1, "the handler ran");
    assert!(answered.is_ok(), "a harmless signal does not fail the call: {answered:?}");
    assert_eq!(count(&bodies, "harmless-signal"), 1, "the interrupted read sent the request again");

    // The token fired before the signal, on a single call that no batch
    // poll watches: only the interrupted read can end it before the
    // reply, and the interruption is the cancel.
    let token = Cancel::new();
    let firing = token.clone();
    let (tid_tx, tid_rx) = mpsc::channel();
    let caller = std::thread::spawn(move || {
        tid_tx.send(unsafe { libc::gettid() }).expect("the tid");
        let started = Instant::now();
        let outcome = tt.decide_opts(&question, "cancelled-signal", Options::new().cancel(&token));
        (outcome, started.elapsed())
    });
    let tid = tid_rx.recv().expect("the caller's tid");
    std::thread::sleep(Duration::from_millis(300));
    firing.cancel();
    unsafe { libc::syscall(libc::SYS_tgkill, libc::getpid(), tid, libc::SIGUSR1) };
    let (cancelled, took) = caller.join().expect("the caller joins");
    std::thread::sleep(DELAY * 2);
    assert_eq!(cancelled.expect_err("the fired token stops the call").kind, ErrorKind::Cancelled);
    assert!(took < DELAY, "the interrupted read ended the call before the reply: {took:?}");
    assert_eq!(count(&bodies, "cancelled-signal"), 1, "a cancelled read sent the request again");
}
