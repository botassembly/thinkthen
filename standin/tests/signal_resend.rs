//! A paid request whose bytes left is never sent a second time
//! (surfaces-review-5). Two shapes, one loopback stub that counts every
//! request body it reads, including a request whose reply nobody reads:
//!
//! 1. A signal interrupts the send. A host's handler without
//!    `SA_RESTART` (a trapped USR1 in Ruby, `statement_timeout`'s SIGALRM
//!    in a PostgreSQL backend) made the read fail with EINTR, and the
//!    connector retried it after the backoff: two bodies, one call. The
//!    read now resumes on the same connection, so the call answers with
//!    one body (the verifier's worker-thread probe; see
//!    `signal_worker.rs`).
//! 2. The first send after a cancelled batch sends once.
//!
//! One test per file, because the engine reads the environment once per
//! process.

mod common;

use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use thinkthen_contract::{Cancel, Engine, ErrorKind, Options, Question};
use thinkthen_standin::BlockingEngine;

const REPLY: &str = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.97}},"usage":{"input_tokens":10,"output_tokens":2}}"#;
const DELAY: Duration = Duration::from_millis(600);

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

extern "C" fn ignore(_: libc::c_int) {}

#[test]
fn no_paid_request_is_sent_twice() {
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
    unsafe { std::env::set_var("THINKTHEN_BASE_URL", format!("http://127.0.0.1:{port}/v1")) };
    let tt = BlockingEngine::from_env();
    let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("parses");

    // Shape 1: a handler without SA_RESTART, and the signal aimed at the
    // thread that is waiting on the reply.
    unsafe {
        let mut action: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = ignore as *const () as usize;
        libc::sigemptyset(&mut action.sa_mask);
        libc::sigaction(libc::SIGUSR1, &action, std::ptr::null_mut());
    }
    let caller = unsafe { libc::pthread_self() };
    let signaller = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(200));
        unsafe { libc::pthread_kill(caller, libc::SIGUSR1) };
    });
    let interrupted = tt.decide(&question, "interrupted-send");
    signaller.join().expect("the signaller joins");
    std::thread::sleep(DELAY * 3);
    assert_eq!(count(&bodies, "interrupted-send"), 1, "an interrupted send was sent again");
    assert!(interrupted.is_ok(), "the interrupted read resumes and answers: {interrupted:?}");

    // Shape 2: a batch cancelled mid-flight, then one single call.
    let token = Cancel::new();
    let firing = token.clone();
    let canceller = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(200));
        firing.cancel();
    });
    let records: Vec<String> = (0..16).map(|n| format!("batch-{n}")).collect();
    let slices: Vec<&str> = records.iter().map(String::as_str).collect();
    let options = Options::new().cancel(&token);
    let cancelled = tt.decide_many_opts(&question, &slices, options, None);
    canceller.join().expect("the canceller joins");
    assert_eq!(cancelled.expect_err("the batch stops").kind, ErrorKind::Cancelled);
    let answer = tt.decide(&question, "after-cancel");
    std::thread::sleep(DELAY * 3);
    assert!(answer.is_ok(), "the next single call answers: {answer:?}");
    assert_eq!(count(&bodies, "after-cancel"), 1, "the first send after a cancel was sent again");
}
