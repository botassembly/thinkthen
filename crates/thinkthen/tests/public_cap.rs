//! Engine snapshots select limits against one retained process reservation count.
#![allow(
    clippy::expect_used,
    reason = "a failed loopback fixture stops this proof"
)]

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use conformance_backend::{Canned, Listener, Rendezvous};
use thinkthen::{
    BatchSetting, CallOptions, Engine, ErrorKind, Question, SendBudget, SendBudgetDenial,
};

const ANSWER: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}}}"#;

#[test]
fn held_request_exhausts_one_limit_while_raised_and_unset_engines_keep_counting() {
    let gate = Arc::new(Rendezvous::new(2));
    let held = Arc::clone(&gate);
    let held_listener =
        Listener::answering(move |_| Canned::ok(ANSWER).after_release(Arc::clone(&held)))
            .expect("held listener");
    let later_listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("later listener");
    let build = |base: &str, limit| {
        Engine::builder()
            .base_url(base)
            .expect("base")
            .api_key("sk-test")
            .expect("key")
            .no_cache()
            .max_retries(0)
            .max_requests_total(limit)
            .build()
            .expect("engine")
    };
    let low = build(held_listener.base(), Some(1));
    let other_low = build(later_listener.base(), Some(1));
    let high = build(later_listener.base(), Some(2));
    let unlimited = build(later_listener.base(), None);
    let question = Question::decide("asks for a refund")
        .expect("question")
        .cut();
    thread::scope(|scope| {
        let first = scope.spawn(|| low.decide(&question, "first"));
        let until = Instant::now() + Duration::from_secs(3);
        while held_listener.count() == 0 {
            assert!(Instant::now() < until, "first request did not arrive");
            thread::sleep(Duration::from_millis(5));
        }
        let denied = other_low
            .decide(&question, "second")
            .expect_err("a second ordinary request cannot cross the low limit");
        assert_eq!(denied.kind(), ErrorKind::Usage);
        assert_eq!(
            denied.send_budget_denial(),
            Some(SendBudgetDenial::BeforeFirstSend)
        );
        assert_eq!(held_listener.count(), 1, "the first request is held");
        assert_eq!(
            later_listener.count(),
            0,
            "a held request already owns the slot"
        );
        gate.wait();
        first.join().expect("first worker").expect("first answer");
    });
    high.decide(&question, "third")
        .expect("raised limit permits one more");
    unlimited
        .decide(&question, "fourth")
        .expect("unset limit permits a send");
    assert_eq!(later_listener.count(), 2);
    assert!(
        high.decide(&question, "fifth").is_err(),
        "unset send remains counted"
    );
    assert_eq!(later_listener.count(), 2);
    let next = build(later_listener.base(), Some(4));
    let later = next
        .decide_many_with(
            &question,
            ["first of two", "second of two"],
            CallOptions::new().batch(BatchSetting::Records(std::num::NonZeroUsize::MIN)),
        )
        .collect::<Result<Vec<_>, _>>()
        .expect_err("the second ordinary request crosses this engine's limit");
    assert_eq!(later.kind(), ErrorKind::Usage);
    assert_eq!(
        later.send_budget_denial(),
        Some(SendBudgetDenial::BeforeAdditionalSend)
    );
    assert_eq!(
        later_listener.count(),
        3,
        "only the first ordinary body arrived"
    );
    let bounded = build(later_listener.base(), Some(5));
    both_budgets_reserve(&bounded, &question, &later_listener);
}

fn both_budgets_reserve(engine: &Engine, question: &Question, listener: &Listener) {
    let explicit = SendBudget::new();
    engine
        .decide_with(
            question,
            "explicit and process budget",
            CallOptions::new().send_budget(&explicit, Some(1)),
        )
        .expect("both reservations admit this attempt");
    assert_eq!(listener.count(), 4);
    assert_eq!(
        engine
            .decide(question, "process cap now spent")
            .expect_err("the explicit budget did not replace the process count")
            .send_budget_denial(),
        Some(SendBudgetDenial::BeforeFirstSend)
    );
    assert_eq!(listener.count(), 4);
}
