//! The caller's options reach the engine whole (error-index rows R1-24,
//! R4-23, and R2-24, Polars halves, and the port guide's `18-cancel-mid-batch`).
//!
//! `CallOptions` has no getters, so the door passes the caller's value whole
//! or not at all. One deadline test therefore covers the deadline, the
//! cancel token, and the interrupt check. The throttle is process-wide, so
//! the test reruns itself alone in a fresh process.

#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

use crate::common;

use std::thread;
use std::time::Duration;

use conformance_backend::Backend;
use thinkthen::PolarsEngine;
use thinkthen::polars::prelude::{NamedFrom, Series};
use thinkthen::{BatchSetting, CallOptions, ErrorKind, Question};

#[test]
fn a_deadline_stops_a_score_column_mid_batch() {
    common::alone(
        "deadline::a_deadline_stops_a_score_column_mid_batch",
        held_past_the_deadline,
    );
}

fn held_past_the_deadline() {
    let backend = Backend::start().expect("a backend");
    let engine = common::builder(&format!("{}/arm/held/v1", backend.origin()))
        .throttle(1)
        .and_then(thinkthen::EngineBuilder::build)
        .expect("an engine at throttle 1");
    let question = Question::score("How urgent is this?")
        .and_then(|builder| builder.level("low", None))
        .and_then(|builder| builder.level("high", None))
        .and_then(thinkthen::ScoreBuilder::build)
        .expect("a score question");
    let texts = common::distinct(3);
    let series = Series::new("body".into(), texts);
    let options = CallOptions::new()
        .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
        .deadline_after(Duration::from_secs(1))
        .expect("a deadline");
    thread::scope(|scope| {
        let running = scope.spawn(|| engine.score_series(&question, &series, options));
        assert_eq!(backend.wait(1), 1, "one singleton reached the held arm");
        backend.round();
        assert_eq!(
            backend.wait(2),
            2,
            "the second singleton reached the held arm"
        );
        // The watchdog is wider than the deadline; it proves a stop, not speed.
        thread::sleep(Duration::from_millis(1100));
        backend.release();
        let error = running
            .join()
            .expect("the score call")
            .expect_err("deadline");
        assert_eq!(error.kind(), ErrorKind::Deadline, "{error}");
        assert_eq!(backend.count(), 2, "the third singleton was not sent");
        assert_eq!(error.facts().map(|facts| facts.requests_sent()), Some(2));
    });
}
