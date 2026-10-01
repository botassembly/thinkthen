//! The library takes the command's timeout bound of one day.
#![allow(
    clippy::expect_used,
    reason = "a failed loopback fixture stops the proof"
)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use conformance_backend::{Canned, Listener};
use thinkthen::{Answer, Engine, ErrorKind, Question};

const ANSWER: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"usage":{"input_tokens":1,"output_tokens":1}}"#;

// Before the bound, `Duration::MAX` or `u64::MAX` seconds hung a loopback call
// with no panic (issue `2026-09-30-library-timeout-has-no-upper-bound.md`).
// Every surface hands its timeout to this setter.
#[test]
fn the_setter_refuses_a_timeout_past_one_day() {
    for (timeout, refusal) in [
        (Duration::ZERO, "a timeout is a time above zero"),
        (
            Duration::from_secs(86_400) + Duration::from_nanos(1),
            "a timeout is at most 86400 seconds",
        ),
        (Duration::from_secs(u64::MAX), "a timeout is at most 86400 seconds"),
        (Duration::MAX, "a timeout is at most 86400 seconds"),
    ] {
        let error = Engine::builder().timeout(timeout).expect_err("refused");
        assert_eq!(
            (error.kind(), error.to_string().as_str()),
            (ErrorKind::Usage, refusal),
            "{timeout:?}"
        );
    }
}

#[test]
fn the_largest_timeout_answers_and_retries() {
    let seen = AtomicUsize::new(0);
    let listener = Listener::answering(move |_| {
        if seen.fetch_add(1, Ordering::SeqCst) == 0 {
            Canned::status(503, "busy").asking("retry-after-ms", "1")
        } else {
            Canned::ok(ANSWER)
        }
    })
    .expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("loopback address")
        .api_key("sk-test")
        .expect("fake key")
        .timeout(Duration::from_secs(86_400))
        .expect("largest timeout")
        .max_retries(1)
        .no_cache()
        .build()
        .expect("engine");
    let question = Question::decide("Is it?").expect("question").cut();

    let answer = engine.decide(&question, "a record").expect("answer");

    assert_eq!(answer.into_value(), Answer::Yes);
    assert_eq!(listener.count(), 2, "one 503, then the answer");
}
