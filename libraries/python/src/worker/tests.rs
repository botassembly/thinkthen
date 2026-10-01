use std::{sync::atomic::Ordering, sync::mpsc::sync_channel, time::Duration, time::Instant};

use pyo3::prelude::*;
use thinkthen::CancelToken;

use super::{Controls, LIVE, Outcome, run, wait};

/// A worker whose caller has left finishes its call and ends without a
/// panic. Only this test starts a worker, so the count is its own.
#[test]
fn a_worker_outlives_a_caller_that_left() {
    Python::initialize();
    let token = CancelToken::new();
    let stopper = token.clone();
    let controls = Controls {
        token: Some(token),
        deadline: None,
    };
    let left = Python::attach(|py| {
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(100));
            stopper.cancel();
        });
        let raised = run(py, controls, |_| {
            std::thread::sleep(Duration::from_millis(400));
            Ok(3_u8)
        });
        raised.err().map(|error| error.value(py).to_string())
    });
    assert_eq!(left.as_deref(), Some("the call was cancelled"));
    assert_eq!(LIVE.load(Ordering::SeqCst), 1, "the worker still runs");
    let ended = Instant::now();
    while LIVE.load(Ordering::SeqCst) > 0 && ended.elapsed() < Duration::from_secs(2) {
        std::thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(LIVE.load(Ordering::SeqCst), 0, "the worker ended");
}

/// A channel that closes with no result raises `DefectError`.
#[test]
fn a_closed_channel_with_no_result_is_a_defect() {
    Python::initialize();
    let (sender, receiver) = sync_channel::<Outcome<u8>>(1);
    drop(sender);
    Python::attach(|py| {
        let error = wait(py, receiver, &CancelToken::new(), None, None).err();
        assert_eq!(
            error.map(|error| error.value(py).to_string()).as_deref(),
            Some("defect: the call ended with no result")
        );
    });
}

/// A fired caller token wins when the answer already waits in the channel.
#[test]
fn a_fired_token_beats_an_answer_already_waiting() {
    Python::initialize();
    let (sender, receiver) = sync_channel::<Outcome<u8>>(1);
    assert!(sender.send(Some(Ok(3))).is_ok());
    let caller = CancelToken::new();
    caller.cancel();
    Python::attach(|py| {
        let error = wait(py, receiver, &CancelToken::new(), Some(&caller), None).err();
        assert_eq!(
            error.map(|error| error.value(py).to_string()).as_deref(),
            Some("the call was cancelled")
        );
    });
}
