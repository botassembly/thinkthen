//! A caller-owned tally observes complete and partially failed calls.
#![allow(
    clippy::expect_used,
    reason = "a failed loopback fixture stops this proof"
)]

use std::sync::Barrier;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use conformance_backend::{Canned, Listener};
use thinkthen::{Engine, Question, Tally};

#[test]
fn tally_keeps_reported_tokens_distinct_from_a_later_missing_usage_reply() {
    const YES: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":12,"output_tokens":3}}"#;
    let seen = AtomicUsize::new(0);
    let listener = Listener::answering(move |_| {
        if seen.fetch_add(1, Ordering::SeqCst) == 0 {
            Canned::ok(YES)
        } else {
            Canned::status(422, "refused")
        }
    })
    .expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-test")
        .expect("key")
        .max_retries(0)
        .no_cache()
        .build()
        .expect("engine");
    let question = Question::decide("asks for a refund")
        .expect("question")
        .cut();
    let tally = Tally::new();
    let clone = tally.clone();
    let first = clone
        .run(|| engine.decide(&question, "first"))
        .expect("first call");
    assert_eq!(first.facts().requests_sent(), 1);
    assert_eq!(tally.facts().input_tokens(), Some(12));
    let second = tally
        .run(|| engine.decide(&question, "second"))
        .expect_err("second call reached the backend and failed");
    assert_eq!(second.facts().map(thinkthen::Facts::requests_sent), Some(1));
    assert_eq!(listener.count(), 2);
    let facts = tally.facts();
    assert_eq!(facts.requests_sent(), 2);
    assert_eq!(facts.input_tokens(), None, "missing usage is not zero");
    assert_eq!(facts.output_tokens(), None);
    assert!(facts.seconds().is_finite() && facts.seconds() >= 0.0);
}

#[test]
fn cloned_tallies_keep_both_concurrent_call_receipts() {
    const YES: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":12,"output_tokens":3}}"#;
    let listener = Listener::answering(|_| Canned::ok(YES)).expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-test")
        .expect("key")
        .no_cache()
        .build()
        .expect("engine");
    let question = Question::decide("asks for a refund")
        .expect("question")
        .cut();
    let tally = Tally::new();
    let barrier = Barrier::new(3);
    thread::scope(|scope| {
        for text in ["first", "second"] {
            let tally = tally.clone();
            let engine = engine.clone();
            let question = question.clone();
            let barrier = &barrier;
            scope.spawn(move || {
                barrier.wait();
                tally.run(|| engine.decide(&question, text)).expect("call");
            });
        }
        barrier.wait();
    });
    assert_eq!(listener.count(), 2);
    let facts = tally.facts();
    assert_eq!(
        (
            facts.requests_sent(),
            facts.input_tokens(),
            facts.output_tokens()
        ),
        (2, Some(24), Some(6))
    );
    assert!(facts.seconds().is_finite() && facts.seconds() >= 0.0);
}

#[test]
fn a_tally_writes_the_members_a_priced_call_writes() {
    const YES: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":1200000,"output_tokens":300000}}"#;
    let listener = Listener::answering(|_| Canned::ok(YES)).expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-test")
        .expect("key")
        .prices_usd_per_million("0.25", "1.5")
        .expect("prices")
        .no_cache()
        .build()
        .expect("engine");
    let question = Question::decide("asks for a refund")
        .expect("question")
        .cut();
    let tally = Tally::new();
    let one = tally
        .run(|| engine.decide(&question, "first"))
        .expect("first call");
    tally
        .run(|| engine.decide(&question, "second"))
        .expect("second call");
    assert_eq!(one.facts().estimated_cost_usd(), Some("0.750000"));
    let members = |facts: &thinkthen::Facts| {
        let json = serde_json::to_value(facts).expect("facts serialize");
        json.as_object()
            .expect("facts are an object")
            .keys()
            .cloned()
            .collect::<Vec<_>>()
    };
    let summed = tally.facts();
    assert_eq!(members(&summed), members(one.facts()));
    assert_eq!(summed.estimated_cost_usd(), Some("1.500000"));
    assert_eq!(summed.model(), Some("jev-1.13.0"));
}
