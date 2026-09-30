//! A stop during a batch, or during a wait on another engine's cache lock,
//! sends nothing new.

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use conformance_backend::Backend;
use thinkthen::{Answer, BatchSetting, CallOptions, Engine, ErrorKind};

use super::{Runs, engine, kind, question, serial};

#[test]
fn a_stop_during_a_batch_or_a_cache_lock_wait_sends_nothing_new() {
    let _serial = serial();
    let backend = Backend::start().expect("backend");
    let batch = engine(&format!("{}/arm/held/v1", backend.origin()));
    let asked = question();
    let texts = ["one", "two", "three", "four", "five", "six"];
    let runs = Runs::default();
    // Requests 1 to 3 answer one round at a time and request 4 stays held, so
    // the check stops the batch at exactly four sends. The count never passes
    // through 4 unseen, as it could once every reply went free.
    // Ticket 0340 replaced a 400 ms sleep and a free backend, which failed 1 run in 200 under load.
    let stopped = AtomicBool::new(false);
    let check = || {
        let stop = runs.record(backend.count()) >= 3 && backend.count() >= 4;
        stopped.fetch_or(stop, Ordering::SeqCst);
        stop
    };
    let options = CallOptions::new()
        .interrupt(&check)
        .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN));
    let rows: Vec<_> = thread::scope(|scope| {
        scope.spawn(|| {
            for sent in 1..=3 {
                backend.wait(sent);
                backend.round();
            }
            // Sent work finishes: request 4 answers once the stop has fired.
            let bound = Instant::now() + Duration::from_secs(10);
            while !stopped.load(Ordering::SeqCst) && Instant::now() < bound {
                thread::sleep(Duration::from_millis(5));
            }
            backend.release();
        });
        batch.filter_with(&asked, texts, options).collect()
    });
    assert!(
        runs.all_on(thread::current().id()),
        "the check ran on a worker"
    );
    assert_eq!(rows.last().and_then(kind), Some(ErrorKind::Cancelled));
    let kept: Vec<_> = rows.iter().filter_map(|row| row.as_ref().ok()).collect();
    assert!(kept.len() <= 4 && kept.iter().zip(texts).all(|(row, text)| **row == text));
    assert_eq!(backend.count(), 4, "nothing new was sent");

    // A second engine on one cache folder waits for the first's answer.
    let folder = std::env::temp_dir().join(format!("thinkthen-controls-{}", std::process::id()));
    let _gone = std::fs::remove_dir_all(&folder);
    let cached = || {
        Engine::builder()
            .base_url(&format!("{}/arm/held/v1", backend.origin()))
            .and_then(|b| b.api_key("sk-public-controls"))
            .and_then(|b| b.cache_at(&folder))
            .and_then(thinkthen::EngineBuilder::build)
            .expect("engine")
    };
    let (first, second) = (cached(), cached());
    thread::scope(|scope| {
        let owner = scope.spawn(|| first.decide(&asked, "Cache me."));
        assert_eq!(backend.wait(5), 5, "the first engine sends");
        let waited = Runs::default();
        let check = || waited.record(backend.count()) >= 3;
        let options = CallOptions::new().interrupt(&check);
        let result = second.decide_with(&asked, "Cache me.", options);
        assert_eq!(kind(&result), Some(ErrorKind::Cancelled), "{result:?}");
        assert!(waited.all_on(thread::current().id()));
        backend.release();
        assert_eq!(
            owner
                .join()
                .expect("owner")
                .ok()
                .map(thinkthen::Call::into_value),
            Some(Answer::Yes)
        );
    });
    assert_eq!(backend.count(), 5, "the waiting engine sent nothing");
    assert_eq!(second.usage().requests_sent(), 0);
    let _gone = std::fs::remove_dir_all(&folder);
}
