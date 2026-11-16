#![cfg(test)]
//! Client-wide observations through the public engine, independent of answer rows.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
use loopback::{KEY, Loopback, Reply};
use serde_json::{Value, json};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;
use thinkthen::{BackendName, Decide, Key, Retry, Settings};

#[test]
fn reported_tokens_are_counted_per_attempt_and_missing_dimensions_stay_unknown() {
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        let count = Arc::new(AtomicUsize::new(0));
        let seen = count.clone();
        let server = Loopback::start(move |request| {
            let index = seen.fetch_add(1, Ordering::SeqCst);
            if index == 0 {
                return Reply::json(429, &json!({"usage":{"input_tokens":7,"output_tokens":0}}));
            }
            let body = request.json().unwrap();
            let answers = if backend == BackendName::SystemOne {
                Value::Object(
                    body["questions"]
                        .as_object()
                        .unwrap()
                        .keys()
                        .map(|key| (key.clone(), json!({"type":"noul","noul":0.9})))
                        .collect(),
                )
            } else {
                json!([{"type":"predicate","probability":0.9}])
            };
            Reply::json(
                200,
                &json!({"answers":answers,"usage":{"input_tokens":11,"output_tokens":"invalid"}}),
            )
        })
        .unwrap();
        let engine = Settings::default()
            .with_backend(backend)
            .url(server.base())
            .api_key(Key::new(KEY).unwrap())
            .retry(Retry::new(1, Duration::from_millis(1), Duration::from_secs(5)).unwrap())
            .engine(|_| None)
            .unwrap();
        let question = Decide::new("Is this relevant?").unwrap();
        assert!(
            engine
                .clone()
                .run(&question, "first")
                .unwrap()
                .into_answer()
                .unwrap()
        );
        let snapshot = engine.usage();
        assert_eq!(snapshot.attempts, 2);
        assert_eq!(snapshot.responses, 2);
        assert_eq!(snapshot.status_retries, 1);
        assert_eq!(snapshot.active_attempts, 0);
        assert_eq!(snapshot.input_tokens.reported, Some(18));
        assert_eq!(snapshot.input_tokens.unknown_attempts, 0);
        assert_eq!(snapshot.output_tokens.reported, Some(0));
        assert_eq!(snapshot.output_tokens.unknown_attempts, 1);
        assert!(snapshot.request_body_bytes > 0);
        assert!(
            engine
                .run(&question, "second")
                .unwrap()
                .into_answer()
                .unwrap()
        );
        assert_eq!(engine.usage().attempts, 3);
        drop(engine);
        assert_eq!(snapshot.attempts, 2);
    }
}

#[test]
fn stub_usage_is_known_zero_and_independent_clients_do_not_share_accounts() {
    let make = || {
        Settings::default()
            .with_backend(BackendName::Stub)
            .engine(|_| None)
            .unwrap()
    };
    let left = make();
    let right = make();
    let _ = left
        .run(&Decide::new("Is this relevant?").unwrap(), "record")
        .unwrap();
    for snapshot in [left.usage(), right.usage()] {
        assert_eq!(snapshot.attempts, 0);
        assert_eq!(snapshot.input_tokens.reported, Some(0));
        assert_eq!(snapshot.output_tokens.unknown_attempts, 0);
    }
}

#[test]
fn concurrent_clones_count_batches_once_while_other_clients_stay_separate() {
    let server = Loopback::start(|request| {
        let body = request.json().unwrap();
        let answers: serde_json::Map<String, Value> = body["questions"]
            .as_object()
            .unwrap()
            .keys()
            .map(|key| (key.clone(), json!({"type":"noul","noul":0.9})))
            .collect();
        Reply::json(
            200,
            &json!({"answers":answers,"usage":{"input_tokens":13,"output_tokens":2}}),
        )
    })
    .unwrap();
    let make = || {
        Settings::default()
            .url(server.base())
            .api_key(Key::new(KEY).unwrap())
            .engine(|_| None)
            .unwrap()
    };
    let engine = make();
    let independent = make();
    let barrier = Arc::new(std::sync::Barrier::new(3));
    let workers: Vec<_> = (0..2)
        .map(|_| {
            let clone = engine.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                let question = Decide::new("Relevant?").unwrap();
                let records = vec![
                    thinkthen::Record::text("first"),
                    thinkthen::Record::text("second"),
                ];
                let rows: Vec<_> = clone.rows(&question, records).unwrap().collect();
                assert_eq!(rows.len(), 2);
                assert!(
                    rows.into_iter()
                        .all(|row| row.unwrap().into_answer().unwrap())
                );
            })
        })
        .collect();
    barrier.wait();
    for worker in workers {
        worker.join().unwrap();
    }
    let usage = engine.settled_usage();
    assert_eq!(usage.attempts, 2);
    assert_eq!(usage.responses, 2);
    assert_eq!(usage.input_tokens.reported, Some(26));
    assert_eq!(usage.output_tokens.reported, Some(4));
    assert_eq!(usage.active_calls, 0);
    assert_eq!(independent.usage().attempts, 0);
}

#[test]
fn token_subtotal_overflow_is_unavailable_without_corrupting_answers() {
    let count = Arc::new(AtomicUsize::new(0));
    let seen = count.clone();
    let server = Loopback::start(move |_| {
        let input = if seen.fetch_add(1,Ordering::SeqCst)==0 { u64::MAX } else { 1 };
        Reply::json(200,&json!({"answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":input,"output_tokens":0}}))
    }).unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .engine(|_| None)
        .unwrap();
    let question = Decide::new("Relevant?").unwrap();
    assert!(
        engine
            .run(&question, "first")
            .unwrap()
            .into_answer()
            .unwrap()
    );
    assert_eq!(engine.usage().input_tokens.reported, Some(u64::MAX));
    assert!(
        engine
            .run(&question, "second")
            .unwrap()
            .into_answer()
            .unwrap()
    );
    let usage = engine.settled_usage();
    assert_eq!(usage.input_tokens.reported, None);
    assert_eq!(usage.input_tokens.unknown_attempts, 0);
    assert_eq!(usage.output_tokens.reported, Some(0));
    assert_eq!(usage.attempts, 2);
}
