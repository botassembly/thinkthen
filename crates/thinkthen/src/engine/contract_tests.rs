#![cfg(test)]
//! A panic in the collecting loop has no outside trigger, so this drives the engine directly.
use super::*;
use crate::core::Decide;
use crate::{BackendName, Code, Settings};
use std::sync::mpsc::channel;
use std::time::Duration;

#[test]
fn a_panic_while_collecting_rows_ends_the_call_as_a_fault() {
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap()
        .batch("1".parse().unwrap());
    let (finished, result) = channel();
    // A build that hangs here must fail the test, so the call runs on its own thread.
    thread::spawn(move || {
        let question = Decide::new("Yes?").unwrap();
        // More one-record requests than the window holds, so the producer waits on a permit.
        let records = (0..REORDER_WINDOW * 4)
            .map(|place| RecordEvent::Record(Record::text(format!("record {place}"))));
        finished.send(engine.call_parallel(
            &question,
            records,
            |_| panic!("the row consumer failed"),
            String::new(),
        ))
    });
    let error = result
        .recv_timeout(Duration::from_secs(60))
        .unwrap()
        .unwrap_err();
    assert_eq!(error.code(), Code::InternalBug);
}
