#![cfg(test)]
//! Request allowances through the public client and its terminal stream outcome.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
use loopback::{KEY, Loopback, Reply, Request};
use serde_json::{Value, json};
use std::sync::{
    Arc, Barrier,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;
use thinkthen::{BackendName, Code, Decide, Engine, Error, Key, Record, Retry, Settings};

fn settings(base: &str, limit: u64) -> Settings {
    Settings::default()
        .url(base)
        .api_key(Key::new(KEY).unwrap())
        .max_requests(Some(limit))
}
fn run(engine: &Engine, texts: &[&str]) -> Result<Vec<Value>, Error> {
    engine
        .rows(
            &Decide::new("Relevant?").unwrap(),
            texts
                .iter()
                .map(|text| Record::text(*text))
                .collect::<Vec<_>>(),
        )?
        .map(|row| row.map(|row| serde_json::to_value(row).unwrap()))
        .collect()
}
fn answered(request: &Request) -> Reply {
    let answers: serde_json::Map<String, Value> = request.json().unwrap()["questions"]
        .as_object()
        .unwrap()
        .keys()
        .map(|key| (key.clone(), json!({"type":"noul","noul":0.9})))
        .collect();
    Reply::json(200, &json!({"answers":answers}))
}

#[test]
fn zero_refuses_remote_work_but_stub_needs_no_allowance_and_exact_boundary_completes() {
    let server = Loopback::noul(0.9).unwrap();
    let zero = settings(server.base(), 0).engine(|_| None).unwrap();
    assert_eq!(
        run(&zero, &["one"]).unwrap_err().code(),
        Code::LocalRequestLimit
    );
    assert_eq!(server.count(), 0);
    assert_eq!(zero.settled_usage().attempts, 0);
    let stub = Settings::default()
        .with_backend(BackendName::Stub)
        .max_requests(Some(0))
        .engine(|_| None)
        .unwrap();
    assert_eq!(run(&stub, &["one", "two"]).unwrap().len(), 2);
    assert_eq!(stub.usage().attempts, 0);
    let exact = settings(server.base(), 1).engine(|_| None).unwrap();
    let rows = run(&exact, &["one", "two"]).unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|row| row["status"] == "answered"));
    assert_eq!(server.count(), 1);
    assert_eq!(exact.settled_usage().attempts, 1);
    assert_eq!(
        run(&exact, &["three"]).unwrap_err().code(),
        Code::LocalRequestLimit
    );
    assert_eq!(server.count(), 1);
}

#[test]
fn allowance_is_atomic_across_concurrent_calls_on_clones() {
    let server = Loopback::noul(0.9).unwrap();
    let engine = settings(server.base(), 3).engine(|_| None).unwrap();
    let barrier = Arc::new(Barrier::new(9));
    let workers: Vec<_> = (0..8)
        .map(|_| {
            let engine = engine.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                run(&engine, &["record"])
            })
        })
        .collect();
    barrier.wait();
    let mut successes = 0;
    let mut exhausted = 0;
    for worker in workers {
        match worker.join().unwrap() {
            Ok(rows) => {
                assert_eq!(rows.len(), 1);
                successes += 1;
            }
            Err(error) => {
                assert_eq!(error.code(), Code::LocalRequestLimit);
                exhausted += 1;
            }
        }
    }
    assert_eq!((successes, exhausted), (3, 5));
    assert_eq!(server.count(), 3);
    assert_eq!(engine.settled_usage().attempts, 3);
    let independent = settings(server.base(), 1).engine(|_| None).unwrap();
    assert_eq!(run(&independent, &["another"]).unwrap().len(), 1);
    assert_eq!(server.count(), 4);
}

#[test]
fn exhaustion_is_one_terminal_error_not_failed_rows_or_successful_eof() {
    let server = Loopback::noul(0.9).unwrap();
    let engine = settings(server.base(), 1)
        .batch("1".parse().unwrap())
        .engine(|_| None)
        .unwrap();
    let records: Vec<_> = (0..20)
        .map(|index| Record::text(format!("record {index}")))
        .collect();
    let rows: Vec<_> = engine
        .rows(&Decide::new("Relevant?").unwrap(), records)
        .unwrap()
        .collect();
    assert_eq!(rows.iter().filter(|row| row.is_err()).count(), 1);
    assert_eq!(
        rows.last().unwrap().as_ref().unwrap_err().code(),
        Code::LocalRequestLimit
    );
    assert!(
        rows.iter()
            .filter_map(|row| row.as_ref().ok())
            .all(|row| row.status() != thinkthen::Status::Failed)
    );
    assert!(server.count() <= 1);
    assert_eq!(engine.settled_usage().attempts, 1);
}

#[test]
fn status_retries_spend_the_same_allowance() {
    for limit in [1, 2] {
        let seen = Arc::new(AtomicUsize::new(0));
        let count = seen.clone();
        let server = Loopback::start(move |request| {
            if count.fetch_add(1, Ordering::SeqCst) == 0 {
                Reply::json(503, &json!({}))
            } else {
                answered(request)
            }
        })
        .unwrap();
        let engine = settings(server.base(), limit)
            .retry(Retry::new(1, Duration::ZERO, Duration::from_secs(5)).unwrap())
            .engine(|_| None)
            .unwrap();
        let result = run(&engine, &["record"]);
        if limit == 1 {
            assert_eq!(result.unwrap_err().code(), Code::LocalRequestLimit);
        } else {
            assert_eq!(result.unwrap().len(), 1);
        }
        assert_eq!(server.count() as u64, limit);
        let usage = engine.settled_usage();
        assert_eq!(usage.attempts, limit);
        assert_eq!(usage.status_retries, limit - 1);
    }
}

#[test]
fn request_splitting_cannot_bypass_the_allowance() {
    let server = Loopback::start(|request| {
        if request.json().unwrap()["questions"]
            .as_object()
            .unwrap()
            .len()
            > 2
        {
            Reply::json(413, &json!({}))
        } else {
            answered(request)
        }
    })
    .unwrap();
    let engine = settings(server.base(), 2).engine(|_| None).unwrap();
    assert_eq!(
        run(&engine, &["one", "two", "three", "four"])
            .unwrap_err()
            .code(),
        Code::LocalRequestLimit
    );
    assert_eq!(server.count(), 2);
    assert_eq!(engine.settled_usage().attempts, 2);
}

#[test]
fn settings_admit_unsigned_integers_and_do_not_inherit_a_hidden_environment_cap() {
    for invalid in ["-1", "1.0", "true", "\"1\""] {
        assert!(Settings::from_json(&format!("{{\"max_requests\":{invalid}}}")).is_err());
    }
    assert_eq!(
        Settings::from_json("{\"max_requests\":0}")
            .unwrap()
            .request_limit(),
        Some(0)
    );
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|name| {
            assert_ne!(name, "THINKTHEN_MAX_REQUESTS");
            None
        })
        .unwrap();
    assert_eq!(engine.resolved_settings().request_limit(), None);
}

#[test]
fn stale_connection_resends_require_another_reservation() {
    for limit in [2, 3] {
        let peer = loopback::ClosingPeer::start(loopback::CloseAt::Reused).unwrap();
        let engine = settings(peer.base(), limit).engine(|_| None).unwrap();
        assert_eq!(run(&engine, &["first"]).unwrap().len(), 1);
        let second = run(&engine, &["second"]);
        if limit == 2 {
            assert_eq!(second.unwrap_err().code(), Code::LocalRequestLimit);
        } else {
            assert_eq!(second.unwrap().len(), 1);
        }
        assert_eq!(peer.finish().unwrap() as u64, limit);
        let usage = engine.settled_usage();
        assert_eq!(usage.attempts, limit);
        assert_eq!(usage.connection_retries, limit - 2);
    }
}

#[test]
fn dependent_recognition_rounds_share_the_client_allowance() {
    for limit in [1, 2] {
        let server=Loopback::answering(|question| {
            if question["criteria"].get("BEGIN").is_some() {
                json!({"type":"choice","probabilities":{"BEGIN":0.0,"INSIDE":0.0,"END":0.0,"SINGLE":1.0,"OUT":0.0}})
            } else {json!({"type":"choice","probabilities":{"person":1.0,"none of these":0.0}})}
        }).unwrap();
        let engine = settings(server.base(), limit).engine(|_| None).unwrap();
        let result = engine.run(&thinkthen::Recognize::new(["person"]).unwrap(), "Ada");
        if limit == 1 {
            assert_eq!(result.unwrap_err().code(), Code::LocalRequestLimit);
        } else {
            assert_eq!(result.unwrap().into_answer().unwrap().len(), 1);
        }
        assert_eq!(server.count() as u64, limit);
        assert_eq!(engine.settled_usage().attempts, limit);
    }
}
