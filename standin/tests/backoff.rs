//! The retry wait hears a cancel and a deadline inside the backoff.
//!
//! Group 4 of the review: a backoff may run to sixty seconds, and a plain
//! sleep would ignore a stop gesture for that whole time. The retry here
//! comes from a 503 — a genuinely retryable answer — because a refused
//! connection fails at once since 2026-09-22 (pinned in `refused.rs`).
//! One test file per process: the engine value is built from an explicit
//! config and no environment is written.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use thinkthen_contract::{Cancel, Connector, Engine, EngineConfig, ErrorKind, Options, Question};
use thinkthen_standin::StandinConnector;

/// A listener that answers every request with 503, the retryable status,
/// so the backoff is the only thing between the call and its error.
fn busy_listener() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("binds");
    let address = listener.local_addr().expect("an address");
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.expect("accepts");
            let mut buffer = [0_u8; 4096];
            let _ = stream.read(&mut buffer);
            let _ = stream.write_all(
                b"HTTP/1.1 503 Service Unavailable\r\ncontent-length: 0\r\nconnection: close\r\n\r\n",
            );
            let _ = stream.flush();
        }
    });
    format!("http://{address}/v1")
}

/// An engine pointed at the busy listener, with retries and a short wait.
fn engine(address: &str) -> Arc<dyn Engine> {
    StandinConnector
        .connect(&EngineConfig {
            address: Some(address.to_owned()),
            timeout: Some(Duration::from_millis(250)),
            max_retries: Some(3),
            width: Some(1),
            ..EngineConfig::default()
        })
        .expect("builds")
}

/// A cancel set during the first backoff returns the cancelled kind
/// promptly, instead of waiting out the whole wait.
#[test]
fn a_cancel_lands_inside_the_retry_backoff() {
    let engine = engine(&busy_listener());
    let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("parses");
    let token = Cancel::new();
    let trip = token.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(150));
        trip.cancel();
    });
    let started = Instant::now();
    let failed = engine
        .decide_opts(&question, "i want a refund", Options::new().cancel(&token))
        .expect_err("the cancel stops the retries");
    let wall = started.elapsed();
    assert_eq!(failed.kind, ErrorKind::Cancelled, "{failed}");
    assert!(
        wall < Duration::from_millis(900),
        "the cancel cut the 1 s backoff short, wall was {wall:?}"
    );
}

/// A deadline shorter than the backoff returns the deadline kind when it
/// passes, inside the wait.
#[test]
fn a_deadline_lands_inside_the_retry_backoff() {
    let engine = engine(&busy_listener());
    let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("parses");
    let options = Options::new().with_deadline_seconds(Some(0.2)).expect("legal");
    let started = Instant::now();
    let failed = engine
        .decide_opts(&question, "i want a refund", options)
        .expect_err("the deadline stops the retries");
    let wall = started.elapsed();
    assert_eq!(failed.kind, ErrorKind::Deadline, "{failed}");
    assert!(
        wall < Duration::from_millis(900),
        "the 0.2 s budget ended inside the 1 s backoff, wall was {wall:?}"
    );
    assert!(failed.message.contains("0.2"), "the message names the budget: {failed}");
}
