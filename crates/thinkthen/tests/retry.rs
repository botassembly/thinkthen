#![cfg(test)]
//! Retry, split and cancellation through the public library API.

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

use loopback::{KEY, Loopback, Reply, Request, UnavailableBackend};
use serde_json::{Value, json};
use thinkthen::{BackendName, Cancel, Decide, Engine, Error, Key, Record, Retry, Settings};

fn settings(base: &str) -> Settings {
    Settings::default()
        .url(base)
        .api_key(Key::new(KEY).unwrap())
        .model("test")
        .retry(Retry::new(2, Duration::from_millis(1), Duration::from_secs(2)).unwrap())
}
fn engine(server: &Loopback) -> Engine {
    settings(server.base()).engine(|_| None).unwrap()
}

fn answers(request: &Request) -> Reply {
    let body = request.json().unwrap();
    let answers: serde_json::Map<String, Value> = body["questions"]
        .as_object()
        .unwrap()
        .keys()
        .map(|key| (key.clone(), json!({"type":"noul", "noul":0.9})))
        .collect();
    Reply::json(200, &json!({"answers": answers}))
}

fn run(engine: &Engine, texts: &[&str]) -> Result<Vec<Value>, Error> {
    engine
        .rows(
            &Decide::new("Is it relevant?").unwrap(),
            texts
                .iter()
                .map(|text| Record::text(*text))
                .collect::<Vec<_>>(),
        )?
        .map(|row| row.map(|row| serde_json::to_value(row).unwrap()))
        .collect()
}

#[test]
fn retryable_statuses_retry_up_to_the_policy_limit() {
    for status in [429, 500, 502, 503, 504, 529] {
        let count = Arc::new(AtomicUsize::new(0));
        let seen = Arc::clone(&count);
        let server = Loopback::start(move |request| {
            if seen.fetch_add(1, Ordering::SeqCst) == 0 {
                Reply::json(status, &json!({}))
            } else {
                answers(request)
            }
        })
        .unwrap();
        assert_eq!(run(&engine(&server), &["one"]).unwrap()[0]["answer"], true);
        assert_eq!(server.count(), 2, "status {status}");
    }
    for status in [429, 500, 502, 503, 504, 529, 400, 401, 403, 408, 501] {
        for max_retries in [0, 1, 2] {
            let server = Loopback::start(move |_| Reply::json(status, &json!({}))).unwrap();
            assert_eq!(
                run(
                    &settings(server.base())
                        .retry(
                            Retry::new(
                                max_retries,
                                Duration::from_millis(1),
                                Duration::from_secs(2)
                            )
                            .unwrap()
                        )
                        .engine(|_| None)
                        .unwrap(),
                    &["one"]
                )
                .unwrap()[0]["error"]["code"],
                "backend.status"
            );
            let expected = if matches!(status, 429 | 500 | 502 | 503 | 504 | 529) {
                max_retries + 1
            } else {
                1
            };
            assert_eq!(
                server.count(),
                usize::try_from(expected).unwrap(),
                "status {status}, limit {max_retries}"
            );
        }
    }
}

#[test]
fn oversized_requests_split_their_questions_only_once() {
    for status in [413, 400] {
        for refuse_halves in [false, true] {
            let server = Loopback::start(move |request| {
                let count = request.json().unwrap()["questions"]
                    .as_object()
                    .unwrap()
                    .len();
                if count > 2 || refuse_halves {
                    Reply::json(status, &json!({"error":"max_tokens_exceeded"}))
                } else {
                    answers(request)
                }
            })
            .unwrap();
            let rows = run(&engine(&server), &["one", "two", "three", "four", "four"]).unwrap();
            assert_eq!(server.count(), 3);
            assert_eq!(
                rows.iter()
                    .map(|row| row["position"].as_u64().unwrap())
                    .collect::<Vec<_>>(),
                [1, 2, 3, 4, 5]
            );
            for row in rows {
                assert_eq!(
                    row["status"],
                    if refuse_halves { "failed" } else { "answered" }
                );
            }
            let sizes: Vec<usize> = server
                .requests()
                .iter()
                .map(|request| {
                    request.json().unwrap()["questions"]
                        .as_object()
                        .unwrap()
                        .len()
                })
                .collect();
            assert_eq!(sizes, [4, 2, 2]);
        }
    }
    let server = Loopback::start(|_| Reply::json(400, &json!({"error":"other"}))).unwrap();
    assert_eq!(
        run(&engine(&server), &["one", "two"]).unwrap()[0]["status"],
        "failed"
    );
    assert_eq!(server.count(), 1);
    let server = Loopback::start(|_| Reply::json(413, &json!({}))).unwrap();
    assert_eq!(
        run(&engine(&server), &["one"]).unwrap()[0]["status"],
        "failed"
    );
    assert_eq!(server.count(), 1);
}

#[test]
fn cancel_before_the_call_sends_nothing() {
    let server = Loopback::noul(0.9).unwrap();
    let cancel = Cancel::default();
    cancel.cancel();
    assert_eq!(
        run(&engine(&server).cancel(cancel), &["one"])
            .unwrap_err()
            .code()
            .as_str(),
        "local.cancelled"
    );
    assert_eq!(server.count(), 0);
}

#[test]
fn cancel_stops_a_retry_wait_without_another_send() {
    let (received, first) = mpsc::channel();
    let server = Loopback::start(move |_| {
        received.send(()).unwrap();
        Reply::json(503, &json!({}))
    })
    .unwrap();
    let cancel = Cancel::default();
    let runner = settings(server.base())
        .retry(Retry::new(2, Duration::from_secs(60), Duration::from_secs(2)).unwrap())
        .engine(|_| None)
        .unwrap()
        .cancel(cancel.clone());
    let (done, result) = mpsc::channel();
    let worker = thread::spawn(move || done.send(run(&runner, &["one"])).unwrap());
    first.recv_timeout(Duration::from_secs(5)).unwrap();
    cancel.cancel();
    let rows = result.recv_timeout(Duration::from_secs(5)).unwrap();
    worker.join().unwrap();
    assert_eq!(rows.unwrap_err().code().as_str(), "local.cancelled");
    assert_eq!(server.count(), 1);
}

#[test]
fn a_sent_request_that_times_out_is_never_resent() {
    let (received, first) = mpsc::channel();
    let (release, held) = mpsc::channel();
    let held = std::sync::Mutex::new(held);
    let server = Loopback::start(move |request| {
        received.send(()).unwrap();
        held.lock().unwrap().recv().unwrap();
        answers(request)
    })
    .unwrap();
    let runner = engine(&server);
    let worker = thread::spawn(move || run(&runner, &["one"]).unwrap());
    first.recv().unwrap();
    let rows = worker.join().unwrap();
    release.send(()).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["error"]["code"], "backend.transport");
    assert_eq!(server.count(), 1);
}

#[test]
fn a_refused_loopback_connection_finishes_as_a_transport_failure() {
    let unavailable = UnavailableBackend::new().unwrap();
    let runner = settings(unavailable.base())
        .retry(Retry::new(2, Duration::ZERO, Duration::from_secs(2)).unwrap())
        .engine(|_| None)
        .unwrap();
    assert_eq!(
        run(&runner, &["one"]).unwrap()[0]["error"]["code"],
        "backend.transport"
    );
}

#[test]
fn cancel_after_an_oversized_reply_prevents_both_split_sends() {
    let cancel = Cancel::default();
    let signal = cancel.clone();
    let server = Loopback::start(move |_| {
        signal.cancel();
        Reply::json(413, &json!({}))
    })
    .unwrap();
    let error = run(&engine(&server).cancel(cancel), &["one", "two"]).unwrap_err();
    assert_eq!(server.count(), 1);
    assert_eq!(error.code().as_str(), "local.cancelled");
}

#[test]
fn splitting_preserves_all_labels_and_shared_questions_per_record() {
    let server = Loopback::start(|request| {
        let body = request.json().unwrap();
        if body["questions"].as_object().unwrap().len() > 2 {
            Reply::json(413, &json!({}))
        } else {
            answers(request)
        }
    })
    .unwrap();
    let question = thinkthen::Tag::new("What applies?", ["billing", "shipping"]).unwrap();
    let mut rows = Vec::new();
    engine(&server)
        .rows(&question, ["one", "one", "two", "two"].map(Record::text))
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(server.count(), 3);
    assert_eq!(rows.len(), 4);
    for (index, row) in rows.iter().enumerate() {
        assert_eq!(row["position"], index + 1);
        assert_eq!(row["status"], "answered");
        assert_eq!(row["answer"], json!(["billing", "shipping"]));
        assert!(row.get("probability").is_none());
    }
    let sizes: Vec<usize> = server
        .requests()
        .iter()
        .map(|request| {
            request.json().unwrap()["questions"]
                .as_object()
                .unwrap()
                .len()
        })
        .collect();
    assert_eq!(sizes, [4, 2, 2]);
}

#[test]
fn openai_does_not_split_on_a_system_one_error_code() {
    let server =
        Loopback::start(|_| Reply::json(400, &json!({"error":"max_tokens_exceeded"}))).unwrap();
    let engine = settings(server.base())
        .with_backend(BackendName::OpenAi)
        .batch("50".parse().unwrap())
        .engine(|_| panic!("explicit settings must not consult environment"))
        .unwrap();
    let rows = run(&engine, &["first", "second"]).unwrap();
    assert_eq!(rows.len(), 2);
    assert!(
        rows.iter()
            .all(|row| row["error"]["code"] == "backend.status")
    );
    assert_eq!(server.count(), 1);
}

#[test]
fn cancelling_an_open_exchange_closes_its_connection_without_retry() {
    use loopback::HeldReply;

    for stage in [HeldReply::Headers, HeldReply::Body, HeldReply::RefusalBody] {
        let (server, arrived, closed) = Loopback::holding(stage).unwrap();
        let cancel = Cancel::default();
        let runner = settings(server.base())
            .retry(Retry::new(2, Duration::from_millis(1), Duration::from_secs(120)).unwrap())
            .engine(|_| None)
            .unwrap()
            .cancel(cancel.clone());
        let (done, result) = mpsc::channel();
        let worker = thread::spawn(move || done.send(run(&runner, &["one"])).unwrap());
        arrived.recv_timeout(Duration::from_secs(30)).unwrap();
        cancel.cancel();
        let rows = result.recv_timeout(Duration::from_secs(30)).unwrap();
        let ending = closed.recv_timeout(Duration::from_secs(30)).unwrap();
        assert!(
            matches!(ending, Ok(0))
                || matches!(ending, Err(ref error) if error.kind() == std::io::ErrorKind::ConnectionReset),
            "{stage:?}: connection remained open: {ending:?}"
        );
        worker.join().unwrap();
        assert_eq!(
            rows.unwrap_err().code().as_str(),
            "local.cancelled",
            "{stage:?}"
        );
        assert_eq!(server.count(), 1, "{stage:?}: cancelled attempt was resent");
    }
}

#[test]
fn cancellation_belongs_to_one_call_and_a_client_can_call_again() {
    let (received, arrivals) = mpsc::channel();
    let (release, held) = mpsc::channel();
    let held = std::sync::Mutex::new(held);
    let server = Loopback::start(move |request| {
        received.send(()).unwrap();
        held.lock().unwrap().recv().unwrap();
        answers(request)
    })
    .unwrap();
    let base = engine(&server);
    let stopped = Cancel::default();
    let (done, results) = mpsc::channel();
    let runner = base.clone().cancel(stopped.clone());
    let worker = thread::spawn(move || done.send(run(&runner, &["one"])).unwrap());
    arrivals.recv_timeout(Duration::from_secs(30)).unwrap();
    let runner = base.clone().cancel(Cancel::default());
    let (other_done, other_result) = mpsc::channel();
    let other = thread::spawn(move || other_done.send(run(&runner, &["two"]).unwrap()).unwrap());
    arrivals.recv_timeout(Duration::from_secs(30)).unwrap();
    stopped.cancel();
    let rows = results.recv_timeout(Duration::from_secs(30)).unwrap();
    assert_eq!(rows.unwrap_err().code().as_str(), "local.cancelled");
    // Release both server handlers. The uncancelled call must still read its reply.
    release.send(()).unwrap();
    release.send(()).unwrap();
    let rows = other_result.recv_timeout(Duration::from_secs(30)).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["answer"], true);
    worker.join().unwrap();
    other.join().unwrap();
    // A new operation on the same engine can establish a healthy connection.
    release.send(()).unwrap();
    let rows = run(&base.cancel(Cancel::default()), &["again"]).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["answer"], true);
    assert_eq!(server.count(), 3);
}

#[test]
fn cancellation_closes_a_reused_connection() {
    let server = Loopback::start(answers).unwrap();
    server.keep_alive();
    let cancel = Cancel::default();
    let runner = settings(server.base())
        .retry(Retry::new(2, Duration::from_millis(1), Duration::from_secs(120)).unwrap())
        .engine(|_| None)
        .unwrap()
        .cancel(cancel.clone());
    assert_eq!(run(&runner, &["first"]).unwrap()[0]["answer"], true);
    let (arrived, closed) = server.hold(loopback::HeldReply::Headers);
    let worker = thread::spawn(move || run(&runner, &["second"]));
    arrived.recv_timeout(Duration::from_secs(30)).unwrap();
    cancel.cancel();
    let ending = closed.recv_timeout(Duration::from_secs(30)).unwrap();
    assert!(
        matches!(ending, Ok(0))
            || matches!(ending, Err(ref error) if error.kind() == std::io::ErrorKind::ConnectionReset)
    );
    let rows = worker.join().unwrap();
    assert_eq!(rows.unwrap_err().code().as_str(), "local.cancelled");
    assert_eq!(server.count(), 2);
    assert_eq!(
        server.connections(),
        1,
        "the call must reuse its pooled socket"
    );
}
