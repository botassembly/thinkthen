#![cfg(test)]
//! A completed call cannot close a pooled socket while another call owns it.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback, Reply};
use serde_json::{Value, json};
use std::sync::{Mutex, mpsc};
use std::time::Duration;
use thinkthen::{Cancel, Decide, Record, Settings};

#[test]
fn completed_call_cancellation_does_not_close_a_reused_connection() {
    let (arrived, arrivals) = mpsc::channel();
    let (release, released) = mpsc::channel();
    let released = Mutex::new(released);
    let server = Loopback::start(move |request| {
        if request.body.windows(6).any(|part| part == b"second") {
            arrived.send(()).unwrap();
            released.lock().unwrap().recv().unwrap();
        }
        let body = request.json().unwrap();
        let answers: serde_json::Map<String, Value> = body["questions"]
            .as_object()
            .unwrap()
            .keys()
            .map(|key| (key.clone(), json!({"type":"noul","noul":0.9})))
            .collect();
        Reply::json(200, &json!({"answers":answers}))
    })
    .unwrap();
    server.keep_alive();
    let engine = Settings::default()
        .url(server.base())
        .api_key(KEY.parse().unwrap())
        .engine(|_| None)
        .unwrap();
    let old = Cancel::default();
    let question = Decide::new("Ready?").unwrap();
    assert_eq!(
        engine
            .clone()
            .cancel(old.clone())
            .run(&question, Record::text("first"))
            .unwrap()
            .into_result()
            .unwrap(),
        Some(true)
    );
    let worker = std::thread::spawn(move || engine.run(&question, Record::text("second")));
    arrivals.recv_timeout(Duration::from_secs(30)).unwrap();
    old.cancel();
    release.send(()).unwrap();
    let row = worker.join().unwrap().unwrap();
    assert_eq!(row.into_result().unwrap(), Some(true));
    assert_eq!(server.count(), 2);
    assert_eq!(server.connections(), 1);
}
