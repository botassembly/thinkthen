//! A retry wait never runs longer than the attempt timeout (main's ticket
//! 0064 and ADR 0040's parity note in
//! `sdlc/records/surfaces-notes/NOTES-main-parity.md`).
//!
//! A header wait is the least of the header, sixty seconds, and the
//! timeout. A doubled backoff is capped at the timeout too. Each listener
//! counts the requests it read, so the tests also pin that every retry
//! was sent. The engine value is built from an explicit config and no
//! environment is written.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use thinkthen_contract::{Connector, Engine, EngineConfig, ErrorKind, Question};
use thinkthen_standin::StandinConnector;

/// A listener that answers every request with `reply` and counts them.
fn listener(reply: &'static [u8]) -> (String, Arc<AtomicUsize>) {
    let bound = TcpListener::bind("127.0.0.1:0").expect("binds");
    let address = bound.local_addr().expect("an address");
    let seen = Arc::new(AtomicUsize::new(0));
    let count = Arc::clone(&seen);
    std::thread::spawn(move || {
        for stream in bound.incoming() {
            let mut stream = stream.expect("accepts");
            let mut buffer = [0_u8; 4096];
            let _ = stream.read(&mut buffer);
            count.fetch_add(1, Ordering::SeqCst);
            let _ = stream.write_all(reply);
            let _ = stream.flush();
        }
    });
    (format!("http://{address}/v1"), seen)
}

/// An engine with a 250 ms attempt timeout and `retries` retries.
fn engine(address: &str, retries: u32) -> Arc<dyn Engine> {
    StandinConnector
        .connect(&EngineConfig {
            address: Some(address.to_owned()),
            timeout: Some(Duration::from_millis(250)),
            max_retries: Some(retries),
            width: Some(1),
            ..EngineConfig::default()
        })
        .expect("builds")
}

/// Ask once and return the failure and the wall time.
fn ask(engine: &Arc<dyn Engine>) -> (thinkthen_contract::Error, Duration) {
    let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("parses");
    let started = Instant::now();
    let failed = engine
        .decide(&question, "i want a refund")
        .expect_err("every reply refuses");
    (failed, started.elapsed())
}

/// A `Retry-After` of thirty seconds waits the 250 ms timeout instead.
#[test]
fn a_header_wait_stops_at_the_attempt_timeout() {
    let (address, seen) = listener(
        b"HTTP/1.1 429 Too Many Requests\r\nretry-after: 30\r\ncontent-length: 0\r\nconnection: close\r\n\r\n",
    );
    let (failed, wall) = ask(&engine(&address, 1));
    assert_eq!(failed.kind, ErrorKind::Backend, "{failed}");
    assert_eq!(
        seen.load(Ordering::SeqCst),
        2,
        "the first attempt and one retry were sent"
    );
    assert!(
        wall < Duration::from_secs(3),
        "the 30 s header wait stopped at the 250 ms timeout, wall was {wall:?}"
    );
}

/// The doubled backoff, one second then two, waits 250 ms each time.
#[test]
fn a_doubled_backoff_stops_at_the_attempt_timeout() {
    let (address, seen) = listener(
        b"HTTP/1.1 503 Service Unavailable\r\ncontent-length: 0\r\nconnection: close\r\n\r\n",
    );
    let (failed, wall) = ask(&engine(&address, 2));
    assert_eq!(failed.kind, ErrorKind::Backend, "{failed}");
    assert_eq!(
        seen.load(Ordering::SeqCst),
        3,
        "the first attempt and two retries were sent"
    );
    assert!(
        wall < Duration::from_millis(2500),
        "the 1 s and 2 s waits stopped at 250 ms each, wall was {wall:?}"
    );
}
