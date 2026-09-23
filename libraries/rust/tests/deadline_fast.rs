//! The fast-backend deadline proof at the Rust surface (lane B item 5).
//!
//! Rust has no signal handler, so the stop gesture here is a deadline: on
//! the null backend the wait's channel never idles, so the poll tick must
//! run on the busy arm too, and a budget spent mid-batch must end the call
//! within about a tick — not after the whole batch, which is what the bug
//! did (SIGINT one second into a three-million-record null batch surfaced
//! 8.48 s later, at batch end). The two-million-record batch runs about
//! 5.6 s deaf at the default width, so the 1.5 s bound separates the two
//! behaviors.
//!
//! Run through `./check.sh`, which sets `ENGINE_NULL=1`; the crate forbids
//! `unsafe`, so this test reads the environment rather than setting it, and
//! a bare `cargo test` fails it by name.

use std::time::{Duration, Instant};

use thinkthen::{Engine, ErrorKind, Options, Question};

mod common;

#[test]
fn a_fast_backend_hears_a_spent_deadline_within_a_tick() {
    common::require_backend();
    let tt = Engine::from_env().expect("the stand-in never fails to build");
    let question =
        Question::from_json(r#"{"decide":"Is this a complaint?"}"#).expect("the question parses");
    let texts: Vec<String> = (0..2_000_000).map(|i| format!("record {i}")).collect();
    let records: Vec<&str> = texts.iter().map(String::as_str).collect();
    let budget = Duration::from_secs(1);
    let started = Instant::now();
    let outcome = tt.decide_many_opts(
        &question,
        &records,
        Options::new().deadline_in(budget),
        None,
    );
    let took = started.elapsed();
    let error = outcome.expect_err("a spent budget returns the deadline kind");
    assert_eq!(error.kind, ErrorKind::Deadline, "{}", error.message);
    assert!(
        took < Duration::from_millis(1_500),
        "the deadline landed at {took:?}; the deaf batch runs about 5.6 s"
    );
}
