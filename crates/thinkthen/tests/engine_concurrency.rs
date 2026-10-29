#![cfg(test)]
//! Worker progress, ordered output, retry pacing and connection reuse.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
use loopback::{KEY, Loopback, Reply, Request};
use serde_json::{Value, json};
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use thinkthen::{Decide, Engine, Key, Record, Retry, Settings};

const WAIT: Duration = Duration::from_secs(30);
fn engine(server: &Loopback) -> Engine {
    engine_batch(server, 1)
}
fn engine_batch(server: &Loopback, batch: usize) -> Engine {
    Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .model("test")
        .batch(batch.to_string().parse().unwrap())
        .retry(Retry::new(1, Duration::from_millis(100), Duration::from_secs(120)).unwrap())
        .engine(|_| None)
        .unwrap()
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
fn run(engine: &Engine, count: usize) -> Vec<Value> {
    engine
        .rows(
            &Decide::new("Relevant?").unwrap(),
            (0..count).map(|i| Record::text(format!("record {i}"))),
        )
        .unwrap()
        .map(|row| serde_json::to_value(row.unwrap()).unwrap())
        .collect()
}
#[test]
fn a_held_first_request_allows_other_workers_to_finish_more_requests_in_order() {
    let (arrived, arrivals) = mpsc::channel();
    let (release, held) = mpsc::channel();
    let held = Mutex::new(held);
    let server = Loopback::start(move |request| {
        let first = request.json().unwrap().to_string().contains("record 0");
        if first {
            held.lock().unwrap().recv_timeout(WAIT).unwrap();
        }
        arrived.send(first).unwrap();
        answer(request)
    })
    .unwrap();
    let runner = engine(&server);
    let worker = thread::spawn(move || run(&runner, 40));
    // The old window only lets three replies finish while the oldest is held.
    let progressed = (0..7).all(|_| arrivals.recv_timeout(WAIT) == Ok(false));
    release.send(()).unwrap();
    let rows = worker.join().unwrap();
    assert!(progressed, "workers stopped behind the oldest request");
    assert_eq!(rows.len(), 40);
    for (i, row) in rows.iter().enumerate() {
        assert_eq!(row["position"], i + 1);
        assert_eq!(row["answer"], true);
    }
}
#[test]
fn four_sequential_rounds_reuse_the_four_worker_connections() {
    let barrier = Arc::new(Barrier::new(4));
    let server = Loopback::start(move |request| {
        barrier.wait();
        answer(request)
    })
    .unwrap();
    server.keep_alive();
    let runner = engine(&server);
    for _ in 0..4 {
        assert_eq!(run(&runner, 4).len(), 4);
    }
    assert_eq!(server.count(), 16);
    assert_eq!(server.connections(), 4);
}

#[test]
fn four_workers_honor_retry_after() {
    let starts = Arc::new(Mutex::new(std::collections::HashMap::new()));
    let times = starts.clone();
    let (measured, delays) = mpsc::channel();
    let barrier = Barrier::new(4);
    let server = Loopback::start(move |request| {
        let key = request.body.clone();
        let now = Instant::now();
        let mut times = times.lock().unwrap();
        if let Some(start) = times.remove(&key) {
            measured.send(now.duration_since(start)).unwrap();
            drop(times);
            answer(request)
        } else {
            times.insert(key, now);
            drop(times);
            barrier.wait();
            Reply::json(429, &json!({})).header("Retry-After", "1")
        }
    })
    .unwrap();
    let rows = run(&engine(&server), 4);
    assert_eq!(rows.len(), 4);
    assert!(rows.iter().all(|row| row["answer"] == true));
    let delays: Vec<_> = (0..4).map(|_| delays.recv_timeout(WAIT).unwrap()).collect();
    assert!(
        delays.iter().all(|delay| *delay >= Duration::from_secs(1)),
        "{delays:?}"
    );
    assert_eq!(server.count(), 8);
}

#[test]
fn retry_after_accepts_an_http_date() {
    use std::time::SystemTime;
    let (sent, arrived) = mpsc::channel();
    let deadline = Mutex::new(None);
    let server = Loopback::start(move |request| {
        let mut deadline = deadline.lock().unwrap();
        if let Some(deadline) = *deadline {
            sent.send((SystemTime::now(), deadline)).unwrap();
            answer(request)
        } else {
            let value = httpdate::fmt_http_date(SystemTime::now() + Duration::from_secs(2));
            *deadline = Some(httpdate::parse_http_date(&value).unwrap());
            Reply::json(429, &json!({})).header("Retry-After", &value)
        }
    })
    .unwrap();
    let rows = run(&engine(&server), 1);
    assert_eq!(rows[0]["answer"], true);
    let (retried, deadline) = arrived.recv_timeout(WAIT).unwrap();
    assert!(retried >= deadline);
    assert_eq!(server.count(), 2);
}

struct Barrier {
    parties: usize,
    state: Mutex<(usize, usize)>,
    changed: Condvar,
}
impl Barrier {
    fn new(parties: usize) -> Self {
        Self {
            parties,
            state: Mutex::new((0, 0)),
            changed: Condvar::new(),
        }
    }
    fn wait(&self) {
        let mut state = self.state.lock().unwrap();
        let generation = state.1;
        state.0 += 1;
        if state.0 == self.parties {
            state.0 = 0;
            state.1 += 1;
            self.changed.notify_all();
            return;
        }
        let (_state, timeout) = self
            .changed
            .wait_timeout_while(state, WAIT, |state| state.1 == generation)
            .unwrap();
        assert!(
            !timeout.timed_out(),
            "peer requests did not reach the test barrier"
        );
    }
}

#[test]
fn a_held_first_request_bounds_rust_input_read_ahead() {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    let batch = 2_usize;
    let window = loopback::limits()["read_ahead_window"].as_u64().unwrap() as usize;
    let bound = batch * window + 1;
    let first_held = Arc::new(AtomicBool::new(true));
    let consumed = Arc::new(AtomicUsize::new(0));
    let (observed, events) = mpsc::channel();
    let input_observed = observed.clone();
    let (release, held) = mpsc::channel();
    let held = Mutex::new(held);
    let server = Loopback::start(move |request| {
        if request.json().unwrap().to_string().contains("record 0") {
            held.lock().unwrap().recv_timeout(WAIT).unwrap();
        } else {
            observed.send(false).unwrap();
        }
        answer(request)
    })
    .unwrap();
    let runner = engine_batch(&server, batch);
    let held_flag = first_held.clone();
    let count = consumed.clone();
    let worker = thread::spawn(move || {
        let records = (0..bound * 4).map(move |index| {
            let next = count.fetch_add(1, Ordering::SeqCst) + 1;
            if held_flag.load(Ordering::SeqCst) && next > bound {
                input_observed.send(true).unwrap();
            }
            Record::text(format!("record {index}"))
        });
        runner
            .rows(&Decide::new("Ready?").unwrap(), records)
            .unwrap()
            .collect::<Vec<_>>()
    });
    let mut exceeded = false;
    for _ in 0..window - 1 {
        if events.recv_timeout(WAIT).unwrap() {
            exceeded = true;
            break;
        }
    }
    exceeded |= consumed.load(Ordering::SeqCst) > bound;
    first_held.store(false, Ordering::SeqCst);
    release.send(()).unwrap();
    let rows = worker.join().unwrap();
    assert!(
        !exceeded,
        "input advanced beyond batch * window + 1 while first request was held"
    );
    assert_eq!(rows.len(), bound * 4);
}
