//! The caller's options reach the engine whole (error-index rows R1-24,
//! R4-23, and R2-24, Polars halves, and the port guide's `18-cancel-mid-batch`).
//!
//! `CallOptions` has no getters, so the door passes the caller's value whole
//! or not at all. One deadline test therefore covers the deadline, the
//! cancel token, and the interrupt check. The throttle is process-wide, so
//! this file holds one test and runs in its own process.

#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

mod common;

use std::time::{Duration, Instant};

use conformance_backend::Backend;
use thinkthen::{CallOptions, ErrorKind, Question};
use thinkthen_polars::PolarsEngine;
use thinkthen_polars::polars::prelude::{NamedFrom, Series};

#[test]
fn a_deadline_stops_a_score_column_mid_batch() {
    let backend = Backend::start().expect("a backend");
    let engine = common::builder(&format!("{}/arm/delay/100/v1", backend.origin()))
        .throttle(8)
        .and_then(thinkthen::EngineBuilder::build)
        .expect("an engine at throttle 8");
    let question = Question::score("How urgent is this?")
        .and_then(|builder| builder.level("low", None))
        .and_then(|builder| builder.level("high", None))
        .and_then(thinkthen::ScoreBuilder::build)
        .expect("a score question");
    let texts = common::distinct(200);
    let series = Series::new("body".into(), texts);
    let options = CallOptions::new()
        .deadline_after(Duration::from_secs(1))
        .expect("a deadline");

    let started = Instant::now();
    let stopped = engine.score_series(&question, &series, options);
    let took = started.elapsed();

    let error = stopped.expect_err("the deadline stops the column");
    assert_eq!(error.kind(), ErrorKind::Deadline, "{error}");
    assert!(
        (Duration::from_millis(900)..Duration::from_millis(1500)).contains(&took),
        "the column stopped at {took:?}, not near its 1 s deadline"
    );
    let counted = backend.count();
    assert!(counted <= 96, "{counted} requests reached the backend");
}
