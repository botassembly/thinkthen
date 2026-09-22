//! The retry wait hears a cancel and a deadline inside the backoff.
//!
//! Group 4 of the review: a backoff may run to sixty seconds, and a plain
//! sleep would ignore a stop gesture for that whole time. One test file
//! per process: the engine value is built from an explicit config and no
//! environment is written.

use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use thinkthen_contract::{Cancel, Connector, Engine, EngineConfig, ErrorKind, Options, Question};
use thinkthen_standin::StandinConnector;

/// An engine whose address refuses instantly, so the retry backoff is the
/// only thing between the call and its error.
fn engine() -> Arc<dyn Engine> {
    StandinConnector
        .connect(&EngineConfig {
            address: Some("http://127.0.0.1:1/v1".into()),
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
    let engine = engine();
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
    let engine = engine();
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
