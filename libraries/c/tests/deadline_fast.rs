//! The deadline through the door: a spent budget refuses before anything
//! is sent, and a budget spent mid-batch ends the wait within a tick (lane
//! B item 5, the poll-bug shape).
//!
//! On the null backend the wait's channel never idles, so the poll tick
//! must run on the busy arm too: a budget spent mid-batch must end the
//! call within about a tick, not after the whole batch, which is what the
//! bug did (SIGINT one second into a three-million-record null batch
//! surfaced 8.48 s later, at batch end). The two-million-record batch runs
//! about 5.6 s. The budget crosses the header's `deadline_ms` argument and
//! the engine's own tick check runs beneath the door; the door's poll
//! callback argument stays deferred (DESIGN.md).
//!
//! Run through `./check.sh`, which sets `THINKTHEN_NULL=1`. Serialized with
//! `--test-threads=1`, because the spent-budget row compares the
//! stand-in's process-global request counter.

use std::ffi::{c_char, CStr, CString};
use std::time::{Duration, Instant};

use thinkthen::thinkthen_engine;

const EDEADLINE: i32 = 3;

/// A judgment that was never written, so a refusal is visible in the out.
const UNWRITTEN: thinkthen::thinkthen_answer = thinkthen::thinkthen_answer {
    outcome: 7,
    probability: -1.0,
};


/// The null backend, as `check.sh` sets it: a wire run would send real
/// requests, and these tests need the fast, silent backend.
fn null_backend() {
    assert_eq!(
        std::env::var("THINKTHEN_NULL").as_deref(),
        Ok("1"),
        "run this through check.sh, which sets THINKTHEN_NULL=1"
    );
}

unsafe fn message(engine: *const thinkthen_engine) -> String {
    CStr::from_ptr(thinkthen::thinkthen_error_message(engine))
        .to_string_lossy()
        .into_owned()
}

/// The door's own request counter through the JSON door; reading the
/// counters sends nothing.
unsafe fn usage_of(engine: *const thinkthen_engine) -> u64 {
    let request = CString::new(r#"{"usage": true}"#).expect("static");
    let reply = thinkthen::thinkthen_call(engine, request.as_ptr());
    assert!(!reply.is_null(), "the counters answer");
    let text = CStr::from_ptr(reply).to_string_lossy().into_owned();
    thinkthen::thinkthen_free_string(reply);
    let value: serde_json::Value = serde_json::from_str(&text).expect("the counters are JSON");
    value["requests"].as_u64().unwrap_or(0)
}

/// The conformance file's spent-budget row (27-deadline-spent-budget): a
/// budget of zero is spent before the call starts, so nothing is sent and
/// the deadline kind comes back.
#[test]
fn a_spent_budget_refuses_before_anything_is_sent() {
    null_backend();
    unsafe {
        let engine = thinkthen::thinkthen_engine_new();
        let question = CString::new("Does the writer ask for a refund?").expect("static");
        let evidence = CString::new("i want a refund").expect("static");
        let before = usage_of(engine);

        let mut answer = UNWRITTEN;
        let code = thinkthen::thinkthen_decide_opts(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            0,
            std::ptr::null(),
            &mut answer,
        );
        assert_eq!(code, EDEADLINE, "{}", message(engine));
        assert!(
            message(engine).contains("the deadline of 0 s"),
            "the message names the limit and its value: {}",
            message(engine)
        );
        assert_eq!(
            thinkthen::thinkthen_error_retryable(engine),
            1,
            "a spent budget is the caller's own limit, not the backend's health"
        );
        assert_eq!(answer.outcome, 7, "no answer lands on a spent budget");
        assert_eq!(usage_of(engine), before, "a spent budget sends nothing");

        thinkthen::thinkthen_engine_free(engine);
    }
}

/// A budget spent mid-batch ends the wait within about a tick, with no
/// rows.
#[test]
fn a_budget_spent_mid_batch_ends_the_wait_within_a_tick() {
    null_backend();
    unsafe {
        let engine = thinkthen::thinkthen_engine_new();
        let question = CString::new("Is this a complaint?").expect("static");
        let texts: Vec<CString> = (0..2_000_000)
            .map(|index| CString::new(format!("record {index}")).expect("no NUL"))
            .collect();
        let lengths: Vec<usize> = texts.iter().map(|text| text.as_bytes().len()).collect();
        let pointers: Vec<*const c_char> = texts.iter().map(|text| text.as_ptr()).collect();
        let mut out: Vec<_> = (0..texts.len()).map(|_| UNWRITTEN).collect();

        let started = Instant::now();
        let code = thinkthen::thinkthen_decide_many_opts(
            engine,
            question.as_ptr(),
            pointers.as_ptr(),
            lengths.as_ptr(),
            texts.len(),
            1_000,
            std::ptr::null(),
            out.as_mut_ptr(),
        );
        let took = started.elapsed();

        assert_eq!(code, EDEADLINE, "{}", message(engine));
        assert!(
            took < Duration::from_millis(1_500),
            "the deadline landed at {took:?}; the deaf batch runs about 5.6 s"
        );
        assert!(
            message(engine).contains("the deadline of 1 s"),
            "the message names the limit and its value: {}",
            message(engine)
        );
        assert!(
            out.iter().take(4).all(|answer| answer.outcome == 7),
            "no rows land on a spent budget"
        );
        assert_eq!(out[out.len() - 1].outcome, 7);

        thinkthen::thinkthen_engine_free(engine);
    }
}

/// Only the sentinel means "no deadline": every other negative budget is
/// refused with the usage kind before anything is sent, and
/// THINKTHEN_NO_DEADLINE itself leaves the call to answer. Before this,
/// any negative value read as no deadline, so a host that computed a
/// negative budget got an unbounded call instead of a refusal.
#[test]
fn every_other_negative_budget_refuses_before_the_wire() {
    null_backend();
    unsafe {
        let engine = thinkthen::thinkthen_engine_new();
        let question = CString::new("Does the customer ask for a refund?").expect("static");
        let evidence = CString::new("I want a refund for order 9").expect("static");
        let before = usage_of(engine);

        let mut answer = UNWRITTEN;
        let code = thinkthen::thinkthen_decide_opts(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            -2,
            std::ptr::null(),
            &mut answer,
        );
        assert_eq!(code, 1, "a negative budget is the usage kind: {}", message(engine));
        assert!(
            message(engine).contains("negative"),
            "the message names why: {}",
            message(engine)
        );
        assert_eq!(answer.outcome, 7, "a refusal writes nothing");
        assert_eq!(usage_of(engine), before, "a refused budget sends nothing");

        let code = thinkthen::thinkthen_decide_opts(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            -1,
            std::ptr::null(),
            &mut answer,
        );
        assert_eq!(
            code, 0,
            "the sentinel itself is no deadline and the call answers: {}",
            message(engine)
        );
        assert_ne!(answer.outcome, 7, "the sentinel leaves the answer to the engine");

        thinkthen::thinkthen_engine_free(engine);
    }
}

/// A budget too large for the engine is refused with the usage kind before
/// anything is sent: the contract's checked conversion owns the
/// millisecond-to-budget step, so an impossible budget is a code, not a
/// panic in the host. Before the checked conversion this call panicked
/// (the instant plus the budget overflowed) and took the host with it.
#[test]
fn an_impossible_budget_is_refused_not_a_panic() {
    null_backend();
    unsafe {
        let engine = thinkthen::thinkthen_engine_new();
        let question = CString::new("Does the customer ask for a refund?").expect("static");
        let evidence = CString::new("I want a refund for order 9").expect("static");
        let before = usage_of(engine);

        let mut answer = UNWRITTEN;
        let code = thinkthen::thinkthen_decide_opts(
            engine,
            question.as_ptr(),
            evidence.as_ptr(),
            evidence.as_bytes().len(),
            i64::MAX,
            std::ptr::null(),
            &mut answer,
        );
        assert_eq!(code, 1, "the usage kind: {}", message(engine));
        assert!(
            message(engine).contains("larger than"),
            "the message names the limit: {}",
            message(engine)
        );
        assert_eq!(answer.outcome, 7, "a refusal writes nothing");
        assert_eq!(usage_of(engine), before, "an impossible budget sends nothing");

        thinkthen::thinkthen_engine_free(engine);
    }
}
