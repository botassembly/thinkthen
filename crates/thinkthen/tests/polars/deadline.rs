//! The caller's options reach the engine whole (error-index rows R1-24,
//! R4-23, and R2-24, Polars halves, and the port guide's `18-cancel-mid-batch`).
//!
//! `CallOptions` has no getters, so the door passes the caller's value whole
//! or not at all. One deadline test therefore covers the deadline, the
//! cancel token, and the interrupt check. The throttle is process-wide, so
//! the test reruns itself alone in a fresh process.

#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

use crate::common;

use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use thinkthen::RecordObservation;

use conformance_backend::Backend;
use thinkthen::PolarsEngine;
use thinkthen::polars::prelude::{NamedFrom, Series};
use thinkthen::{BatchSetting, CallOptions, ErrorKind, Question};

#[test]
fn a_deadline_stops_a_score_column_mid_batch() {
    common::alone(
        "deadline::a_deadline_stops_a_score_column_mid_batch",
        || {
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
            let prefix = Mutex::new(Vec::new());
            let observer = |event: RecordObservation<'_>| {
                record_score(&prefix, event);
            };
            let expired = engine
                .score_series(
                    &question,
                    &series,
                    CallOptions::new()
                        .deadline_at(std::time::Instant::now() - Duration::from_secs(1)),
                )
                .expect_err("expired deadline");
            assert_eq!(expired.kind(), ErrorKind::Deadline);
            assert_eq!(backend.count(), 0, "an expired deadline sends nothing");
            let options = CallOptions::new()
                .observe(&observer)
                .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
                .deadline_after(Duration::from_secs(1))
                .expect("a deadline");
            thread::scope(|scope| {
                let running = scope.spawn(|| engine.score_series(&question, &series, options));
                // A loaded host may spend the deadline before sending. If a
                // reply is already held, release that round; no second send
                // is required for correct expiry.
                backend.wait(1);
                backend.round();
                // The watchdog is wider than the deadline; it proves a stop, not speed.
                thread::sleep(Duration::from_millis(1100));
                backend.release();
                let error = running
                    .join()
                    .expect("the score call")
                    .expect_err("deadline");
                assert_eq!(error.kind(), ErrorKind::Deadline, "{error}");
                let sends = backend.count();
                assert!(sends <= 2, "no third singleton follows expiry");
                assert_eq!(
                    error.facts().map(|facts| facts.requests_sent()),
                    Some(sends as u64)
                );
                let prefix = prefix.lock().expect("completed prefix");
                assert_eq!(
                    *prefix,
                    (0..prefix.len())
                        .map(|index| (index, 0.1))
                        .collect::<Vec<_>>()
                );
                assert!(prefix.len() <= sends);
                assert!(prefix.len() <= 1);
            });
        },
    );
}

#[expect(
    clippy::expect_used,
    reason = "a poisoned fixture stops the prefix proof"
)]
fn record_score(prefix: &Mutex<Vec<(usize, f64)>>, event: RecordObservation<'_>) {
    if let RecordObservation::Question { index, detail, .. } = event
        && let Some(thinkthen::Judgment::Score(score)) = detail.value()
    {
        prefix.lock().expect("prefix").push((index, *score));
    }
}
