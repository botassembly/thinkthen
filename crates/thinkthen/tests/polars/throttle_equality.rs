//! A Series crosses at the throttle as a slice does: the proof Ian named for
//! Polars on 2026-09-21, applied to Rust.
//!
//! The throttle is process-wide (0077), so this file holds one test and runs
//! in its own process. Each run gets its own backend, cache folder, and
//! engine at throttle 2. Each backend is the one the caller's `base_url`
//! names, so a door that built its own engine would count nothing here.

#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

mod common;

use std::sync::{Arc, Barrier};
use std::thread;

use conformance_backend::Backend;
use thinkthen::PolarsEngine;
use thinkthen::polars::prelude::{NamedFrom, Series};
use thinkthen::{Answer, BatchSetting, CallOptions, Engine, Question};

const THROTTLE: u8 = 2;
const TEXTS: usize = 3;

fn singleton() -> CallOptions<'static> {
    CallOptions::new().batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
}

/// One way to decide the texts on an engine.
type Call<'a> = dyn Fn(&Engine) -> Vec<Option<bool>> + 'a;

fn engine(base: &str) -> Engine {
    common::builder(base)
        .throttle(THROTTLE)
        .and_then(thinkthen::EngineBuilder::build)
        .expect("an engine at throttle 2")
}

/// Start three distinct singleton calls together and count held arrivals.
fn held_in_flight(call: impl Fn(&Engine, usize) + Sync) -> (usize, usize) {
    let backend = Backend::start().expect("a backend");
    let engine = engine(&format!("{}/arm/held/v1", backend.origin()));
    thread::scope(|scope| {
        let ready = Arc::new(Barrier::new(TEXTS + 1));
        let running: Vec<_> = (0..TEXTS)
            .map(|index| {
                let ready = Arc::clone(&ready);
                let engine = &engine;
                let call = &call;
                scope.spawn(move || {
                    ready.wait();
                    call(engine, index);
                })
            })
            .collect();
        ready.wait();
        let reached = backend.wait(usize::from(THROTTLE));
        let held = backend.count();
        backend.release();
        for worker in running {
            worker.join().expect("the held call");
        }
        assert_eq!(reached, usize::from(THROTTLE), "the held throttle");
        assert_eq!(
            held,
            usize::from(THROTTLE),
            "no third arrival before release"
        );
        (held, backend.count())
    })
}

#[test]
fn a_series_runs_at_the_throttle_as_a_slice_does() {
    let texts = common::distinct(TEXTS);
    let refs: Vec<&str> = texts.iter().map(String::as_str).collect();
    let series = Series::new("body".into(), refs.as_slice());
    let question = Question::decide("Does this ask for a refund?")
        .expect("a question")
        .cut();

    equal_answers(&question, &series, &refs, TEXTS);

    // Exactly two held requests, then the third arrives only after release.
    let slice = held_in_flight(|engine, index| {
        let _answered: Vec<_> = engine
            .decide_many_with(&question, [refs[index]], singleton())
            .collect();
    });
    assert_eq!(slice, (2, TEXTS), "the slice in flight on the held arm");
    let decide = held_in_flight(|engine, index| {
        let one = Series::new("body".into(), &[refs[index]]);
        let _answered = engine.decide_series(&question, &one, singleton());
    });
    assert_eq!(decide, slice, "decide_series in flight on the held arm");
    let score = Question::score("How urgent is this?")
        .and_then(|builder| builder.level("low", None))
        .and_then(|builder| builder.level("high", None))
        .and_then(thinkthen::ScoreBuilder::build)
        .expect("a score question");
    let scored = held_in_flight(|engine, index| {
        let one = Series::new("body".into(), &[refs[index]]);
        let _answered = engine.score_series(&score, &one, singleton());
    });
    assert_eq!(scored, (2, TEXTS), "score_series in flight on the held arm");
}

#[test]
#[ignore = "200-record equality campaign; run sdlc/scripts/test-stress --run"]
fn two_hundred_series_records_match_the_slice() {
    let texts = common::distinct(200);
    let refs: Vec<&str> = texts.iter().map(String::as_str).collect();
    let series = Series::new("body".into(), refs.as_slice());
    let question = Question::decide("Does this ask for a refund?")
        .expect("a question")
        .cut();
    equal_answers(&question, &series, &refs, 200);
}

fn equal_answers(question: &Question, series: &Series, refs: &[&str], count: usize) {
    // Answers and counts on the delay arm, where replies finish out of order.
    let answered = |call: &Call<'_>| {
        let backend = Backend::start().expect("a backend");
        let engine = engine(&format!("{}/arm/delay/100/v1", backend.origin()));
        (call(&engine), backend.count())
    };
    let (from_series, series_count) = answered(&|engine| {
        let answered = engine
            .decide_series(question, series, singleton())
            .expect("the series call");
        answered
            .value()
            .bool()
            .expect("a Boolean series")
            .iter()
            .collect()
    });
    let (from_slice, slice_count) = answered(&|engine| {
        engine
            .decide_many_with(question, refs.iter().copied(), singleton())
            .map(|row| {
                row.map(|row| (*row.value() != Answer::Unsure).then(|| *row.value() == Answer::Yes))
            })
            .collect::<Result<_, _>>()
            .expect("the slice call")
    });
    assert_eq!((series_count, slice_count), (count, count));
    assert_eq!(from_series, from_slice);
}
