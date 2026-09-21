//! The fast-backend deadline proof at the engine this door calls (lane B
//! item 5, the poll-bug shape).
//!
//! The C header exposes no cancel token and no deadline (NOTES.md,
//! finding 3), so a C program cannot set a budget yet. This test drives
//! the same engine the door calls — `thinkthen_standin::BlockingEngine`
//! through the contract — and proves the shape the door will inherit when
//! the header grows a budget: on the null backend the wait's channel
//! never idles, so the poll tick must run on the busy arm too, and a
//! budget spent mid-batch must end the call within about a tick, not
//! after the whole batch, which is what the bug did (SIGINT one second
//! into a three-million-record null batch surfaced 8.48 s later, at batch
//! end). The two-million-record batch runs about 5.6 s deaf.
//!
//! Run through `./check.sh`, which sets `ENGINE_NULL=1`.

use std::time::{Duration, Instant};

use thinkthen_contract::{Engine, ErrorKind, Options, Question};
use thinkthen_standin::BlockingEngine;

#[test]
fn a_fast_backend_hears_a_spent_deadline_within_a_tick() {
    assert_eq!(
        std::env::var("ENGINE_NULL").as_deref(),
        Ok("1"),
        "run this through check.sh, which sets ENGINE_NULL=1"
    );
    let engine = BlockingEngine::from_env();
    let question =
        Question::from_json(r#"{"decide":"Is this a complaint?"}"#).expect("the question parses");
    let texts: Vec<String> = (0..2_000_000).map(|i| format!("record {i}")).collect();
    let records: Vec<&str> = texts.iter().map(String::as_str).collect();
    let started = Instant::now();
    let outcome = engine.decide_many_opts(
        &question,
        &records,
        Options::new().deadline_in(Duration::from_secs(1)),
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
