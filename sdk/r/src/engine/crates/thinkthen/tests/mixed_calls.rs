#![cfg(test)]
//! Cancelled, refused and healthy calls on one keep-alive client leak no connection.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{HeldReply, KEY, Loopback, Reply, Request, limits};
use serde_json::{Value, json};
use std::collections::HashSet;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use thinkthen::{Cancel, Decide, Engine, Error, Key, Record, Retry, Settings};

const ROUNDS: usize = 20;
const WAIT: Duration = Duration::from_secs(30);

fn contains(request: &Request, word: &str) -> bool {
    String::from_utf8_lossy(&request.body).contains(word)
}

fn answer(request: &Request) -> Reply {
    let body = request.json().unwrap();
    let answers: serde_json::Map<String, Value> = body["questions"]
        .as_object()
        .unwrap()
        .keys()
        .map(|key| (key.clone(), json!({"type":"noul","noul":0.9})))
        .collect();
    Reply::json(200, &json!({"answers":answers}))
}

/// One request for each worker, with text that no other round sends.
fn call(engine: &Engine, kind: &str, round: usize, workers: usize) -> Result<Vec<Value>, Error> {
    let records: Vec<_> = (0..workers)
        .map(|worker| Record::text(format!("{kind} {round} {worker}")))
        .collect();
    engine
        .rows(&Decide::new("Ready?").unwrap(), records)?
        .map(|row| row.map(|row| serde_json::to_value(row).unwrap()))
        .collect()
}

fn assert_closed(ending: std::io::Result<usize>) {
    assert!(
        matches!(ending, Ok(0))
            || matches!(ending, Err(ref error)
        if error.kind() == std::io::ErrorKind::ConnectionReset),
        "connection stayed open: {ending:?}"
    );
}

fn assert_answered(rows: &[Value], workers: usize) {
    assert_eq!(rows.len(), workers);
    assert!(rows.iter().all(|row| row["answer"] == true), "{rows:?}");
}

#[test]
fn mixed_calls_on_one_client_open_connections_only_for_cancelled_ones() {
    let workers = usize::try_from(limits()["workers"].as_u64().unwrap()).unwrap();
    let refused = Mutex::new(HashSet::new());
    let server = Loopback::start(move |request| {
        if contains(request, "refused") && refused.lock().unwrap().insert(request.body.clone()) {
            Reply::json(503, &json!({"error":"busy"}))
        } else {
            answer(request)
        }
    })
    .unwrap();
    server.keep_alive();
    let (arrivals, endings) =
        server.hold_matching(HeldReply::Headers, |request| contains(request, "cancelled"));
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .batch("1".parse().unwrap())
        .retry(Retry::new(1, Duration::from_millis(1), Duration::from_secs(120)).unwrap())
        .engine(|_| None)
        .unwrap();
    let (mut cancels, mut requests) = (0, 0);
    for round in 0..ROUNDS {
        match round % 3 {
            0 => {
                let cancel = Cancel::default();
                let cancelled = engine.clone().cancel(cancel.clone());
                let worker = thread::spawn(move || call(&cancelled, "cancelled", round, workers));
                for _ in 0..workers {
                    arrivals.recv_timeout(WAIT).unwrap();
                }
                cancel.cancel();
                for _ in 0..workers {
                    assert_closed(endings.recv_timeout(WAIT).unwrap());
                }
                let error = worker.join().unwrap().unwrap_err();
                assert_eq!(error.code().as_str(), "local.cancelled");
                cancels += 1;
                requests += workers;
            }
            1 => {
                assert_answered(&call(&engine, "refused", round, workers).unwrap(), workers);
                requests += 2 * workers;
            }
            _ => {
                assert_answered(&call(&engine, "healthy", round, workers).unwrap(), workers);
                requests += workers;
            }
        }
        assert_eq!(server.count(), requests, "round {round}");
        let limit = workers + workers * cancels;
        assert!(
            server.connections() <= limit,
            "round {round}: {} connections, limit {limit}",
            server.connections()
        );
    }
}
