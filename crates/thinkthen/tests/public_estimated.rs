//! Final encoded-body admission, measured at the listener in one process.
#![allow(
    clippy::expect_used,
    reason = "a failed loopback fixture stops this proof"
)]

use std::num::NonZeroUsize;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use conformance_backend::{Canned, Listener, Rendezvous};
use thinkthen::{BatchSetting, CallOptions, Engine, ErrorKind, EstimatedInputDenial, Question};

const ANSWER: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":312,"output_tokens":48000}}"#;

#[test]
fn final_body_charge_selects_each_limit_and_distinguishes_later_and_retry_denials() {
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let question = Question::decide("asks for a refund")
        .expect("question")
        .cut();
    let build = |limit| {
        Engine::builder()
            .base_url(listener.base())
            .expect("base")
            .api_key("sk-test")
            .expect("fake key")
            .no_cache()
            .max_retries(0)
            .max_estimated_input_tokens_total(limit)
            .build()
            .expect("engine")
    };
    let first_call = build(None)
        .decide(&question, "item one")
        .expect("unbounded admission counts");
    let bodies = listener.requests();
    let first = bodies.first().expect("one captured body").body.len() as u64;
    let charge = (first * 908).div_ceil(1000);
    assert!(
        charge < 312,
        "reported input can exceed the admission estimate"
    );
    assert_eq!(first_call.facts().input_tokens(), Some(312));
    assert_eq!(first_call.facts().output_tokens(), Some(48000));
    assert_eq!(listener.count(), 1);
    let limited = build(Some(charge * 2));
    limited
        .decide(&question, "item two")
        .expect("raised limit admits one");
    let bodies = listener.requests();
    assert_eq!(
        bodies.first().expect("captured body").body.len() as u64,
        first,
        "same-length independent body"
    );
    let denied = limited
        .decide(&question, "item tri")
        .expect_err("spent before first send");
    assert_eq!(denied.kind(), ErrorKind::Usage);
    assert_eq!(
        denied.estimated_input_denial(),
        Some(EstimatedInputDenial::InitialRequest { limit: charge * 2 })
    );
    assert_eq!(
        denied.detail().message(),
        format!(
            "max_estimated_input_tokens_total={} (encoded-body-bytes-908-v1) would be exceeded before this call's first request",
            charge * 2
        )
    );
    assert_eq!(listener.count(), 2, "denial sent nothing");

    let batch_engine = build(Some(charge * 3));
    let later = batch_engine
        .decide_many_with(
            &question,
            ["item red", "item blu"],
            CallOptions::new().batch(BatchSetting::Records(NonZeroUsize::MIN)),
        )
        .collect::<Result<Vec<_>, _>>()
        .expect_err("second request exceeds limit");
    assert_eq!(
        later.estimated_input_denial(),
        Some(EstimatedInputDenial::AdditionalRequest { limit: charge * 3 })
    );
    assert_eq!(listener.count(), 3, "one batch body arrived");
    assert_eq!(
        listener
            .requests()
            .first()
            .expect("captured body")
            .body
            .len() as u64,
        first
    );
    assert!(later.facts().is_some(), "started call retains facts");
    assert!(
        !format!("{later:?}").contains("item blu"),
        "private input withheld"
    );

    retry_refusal(charge, first, &question);
    held_race(charge, first, &question);
    cached_answer(charge, first, &question);
    replayed_answer(charge, first, &question);
}

fn retry_refusal(charge: u64, first: u64, question: &Question) {
    let retry_listener =
        Listener::answering(|_| Canned::status(503, "busy").asking("retry-after-ms", "0"))
            .expect("retry listener");
    let retry = Engine::builder()
        .base_url(retry_listener.base())
        .expect("base")
        .api_key("sk-test")
        .expect("fake key")
        .no_cache()
        .max_retries(1)
        .max_estimated_input_tokens_total(Some(charge * 4))
        .build()
        .expect("retry engine");
    let refused = retry
        .decide(question, "item six")
        .expect_err("same exchange retry refused");
    assert_eq!(retry_listener.count(), 1);
    assert_eq!(
        retry_listener
            .requests()
            .first()
            .expect("captured body")
            .body
            .len() as u64,
        first
    );
    assert_eq!(
        refused.estimated_input_denial(),
        Some(EstimatedInputDenial::Retry {
            limit: charge * 4,
            last_status: 503
        })
    );
    assert!(
        refused.facts().is_some(),
        "failed started attempt retains facts"
    );
}

fn held_race(charge: u64, first: u64, question: &Question) {
    let gate = Arc::new(Rendezvous::new(2));
    let held = Arc::clone(&gate);
    let arrived = Listener::answering(move |_| Canned::ok(ANSWER).after_release(Arc::clone(&held)))
        .expect("held listener");
    let denied = Listener::answering(|_| Canned::ok(ANSWER)).expect("denied listener");
    let build = |base: &str| {
        Engine::builder()
            .base_url(base)
            .expect("base")
            .api_key("sk-test")
            .expect("fake key")
            .no_cache()
            .max_retries(0)
            .max_estimated_input_tokens_total(Some(charge * 5))
            .build()
            .expect("engine")
    };
    let one = build(arrived.base());
    let two = build(denied.base());
    thread::scope(|scope| {
        let worker = scope.spawn(|| one.decide(question, "item ten"));
        let until = Instant::now() + Duration::from_secs(3);
        while arrived.count() == 0 {
            assert!(Instant::now() < until, "held request never arrived");
            thread::sleep(Duration::from_millis(5));
        }
        let refusal = two
            .decide(question, "item six")
            .expect_err("in-flight charge fills total");
        assert_eq!(
            refusal.estimated_input_denial(),
            Some(EstimatedInputDenial::InitialRequest { limit: charge * 5 })
        );
        assert_eq!(denied.count(), 0, "a second request did not arrive");
        gate.wait();
        worker.join().expect("worker").expect("held answer");
    });
    assert_eq!(
        arrived
            .requests()
            .first()
            .expect("captured held body")
            .body
            .len() as u64,
        first
    );
}

fn cached_answer(charge: u64, first: u64, question: &Question) {
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("cache listener");
    let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("estimated-cache-{}", std::process::id()));
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-test")
        .expect("fake key")
        .cache_at(&folder)
        .expect("cache folder")
        .max_retries(0)
        .max_estimated_input_tokens_total(Some(charge * 6))
        .build()
        .expect("engine");
    engine
        .decide(question, "item one")
        .expect("sixth charge admitted");
    assert_eq!(listener.count(), 1);
    assert_eq!(
        listener
            .requests()
            .first()
            .expect("captured body")
            .body
            .len() as u64,
        first
    );
    engine
        .decide(question, "item one")
        .expect("identical cached answer at spent cap");
    assert_eq!(
        listener.count(),
        1,
        "cache hit adds no live attempt or charge"
    );
    let refusal = engine
        .decide(question, "item two")
        .expect_err("different body refused");
    assert_eq!(
        refusal.estimated_input_denial(),
        Some(EstimatedInputDenial::InitialRequest { limit: charge * 6 })
    );
    assert_eq!(listener.count(), 1);
}

fn replayed_answer(charge: u64, first: u64, question: &Question) {
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("record listener");
    let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("estimated-replay-{}", std::process::id()));
    let base = || {
        Engine::builder()
            .base_url(listener.base())
            .expect("base")
            .api_key("sk-test")
            .expect("fake key")
            .no_cache()
            .max_retries(0)
            .max_estimated_input_tokens_total(Some(charge * 7))
    };
    base()
        .record(&folder)
        .expect("record path")
        .build()
        .expect("record engine")
        .decide(question, "item one")
        .expect("seventh charge admitted");
    assert_eq!(listener.count(), 1);
    assert_eq!(
        listener
            .requests()
            .first()
            .expect("recorded body")
            .body
            .len() as u64,
        first
    );
    base()
        .replay(&folder)
        .expect("replay path")
        .build()
        .expect("replay engine")
        .decide(question, "item one")
        .expect("replay at spent total");
    assert_eq!(
        listener.count(),
        1,
        "replay adds no live attempt or estimate"
    );
}
