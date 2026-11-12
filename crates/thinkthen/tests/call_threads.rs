#![cfg(test)]
//! A client's later calls run on the threads its first calls started. This file holds one
//! test, so no other test starts a thread in its process.
#![cfg(target_os = "linux")]
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use thinkthen::{BackendName, Code, Decide, Record, Settings};

const RECORDS: usize = 200;

fn threads() -> BTreeSet<String> {
    std::fs::read_dir("/proc/self/task")
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect()
}

#[test]
fn sequential_calls_start_no_threads_after_the_first_two() {
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let question = Decide::new("Ready?").unwrap();
    let seen = Arc::new(Mutex::new(BTreeSet::new()));
    // Four requests, so every worker of a call is running when its input is read.
    let call = || {
        let seen = seen.clone();
        let records = (0..RECORDS).map(move |place| {
            seen.lock().unwrap().extend(threads());
            Record::text(format!("record {place}"))
        });
        let rows = engine.rows(&question, records).unwrap();
        assert_eq!(rows.filter(|row| row.is_ok()).count(), RECORDS);
    };
    call();
    call();
    let warm = threads();
    seen.lock().unwrap().clear();
    for _ in 0..10 {
        call();
    }
    // A panic on a kept thread fails its call and leaves the thread to the next call.
    let failing = (0..RECORDS).map(|place| {
        assert!(place < RECORDS / 2, "the record source failed");
        Record::text(format!("record {place}"))
    });
    let error = engine
        .rows(&question, failing)
        .unwrap()
        .find_map(Result::err)
        .unwrap();
    assert_eq!(error.code(), Code::InternalBug);
    assert!(error.to_string().contains("the call worker failed"));
    // The failed call's threads are free once it has settled.
    let _ = engine.settled_usage();
    call();
    let mut seen = seen.lock().unwrap().clone();
    assert!(seen.len() > 6, "a call's threads were seen: {seen:?}");
    seen.extend(threads());
    let started = seen.difference(&warm).collect::<Vec<_>>();
    assert!(
        started.is_empty(),
        "later calls started {} threads",
        started.len()
    );
}
