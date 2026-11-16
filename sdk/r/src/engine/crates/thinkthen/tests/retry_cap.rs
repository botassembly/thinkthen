#![cfg(test)]
//! A configured backoff cannot postpone the next attempt beyond the shared cap.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback, Reply};
use serde_json::json;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use thinkthen::{Cancel, Decide, Record, Settings};

#[test]
fn a_large_initial_backoff_is_capped_before_the_next_attempt() {
    let calls = AtomicUsize::new(0);
    let (attempt, observed) = std::sync::mpsc::channel();
    let server = Loopback::start(move |_| {
        let number = calls.fetch_add(1, Ordering::SeqCst);
        attempt.send(number).unwrap();
        Reply::json(if number == 0 { 503 } else { 401 }, &json!({}))
    })
    .unwrap();
    let engine = Settings::from_json(
        &json!({"url":server.base(),"retry":{"initial_backoff_ms":3_600_000,"max_retries":1}})
            .to_string(),
    )
    .unwrap()
    .engine(|name| (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned()))
    .unwrap();
    let cancel = Cancel::default();
    let worker = std::thread::spawn({
        let engine = engine.cancel(cancel.clone());
        move || {
            engine
                .rows(&Decide::new("Ready?").unwrap(), [Record::text("evidence")])
                .unwrap()
                .collect::<Result<Vec<_>, _>>()
        }
    });
    assert_eq!(observed.recv_timeout(Duration::from_secs(10)).unwrap(), 0);
    // Wait for the actual retry event. The margin allows scheduler contention;
    // no fake clock advances while the real socket is active.
    let retried = observed.recv_timeout(Duration::from_secs(70));
    if retried.is_err() {
        cancel.cancel();
    }
    let result = worker.join().unwrap();
    assert_eq!(
        retried.unwrap(),
        1,
        "the retry arrives within its capped wait"
    );
    let error = result.unwrap_err();
    assert_eq!(error.http_status(), Some(401));
    assert_eq!(server.count(), 2);
}
