use super::*;
use thinkthen::Tally;

#[test]
fn priced_cached_and_replayed_calls_add_zero_cost_without_inventing_tokens() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}},"usage":{"input_tokens":49,"output_tokens":0}}"#)).unwrap();
    let place = folder();
    let build = || {
        Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .api_key("complete-private")
            .unwrap()
            .prices_usd_per_million("0.01", "0")
            .unwrap()
    };
    let engine = build().cache_at(&place).unwrap().build().unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let mixed = Tally::new();
    for item in ["First.", "Second."] {
        let call = mixed.run(|| engine.decide(&question, item)).unwrap();
        assert_eq!(call.facts().estimated_cost_usd(), Some("0.000000"));
    }
    assert_eq!(
        mixed.facts_with_engine(&engine).estimated_cost_usd(),
        Some("0.000001")
    );
    for replay in [false, true] {
        let reader = if replay {
            build().replay(&place).unwrap().build().unwrap()
        } else {
            engine.clone()
        };
        let tally = Tally::new();
        for item in ["First.", "Second."] {
            let call = tally.run(|| reader.decide(&question, item)).unwrap();
            assert_eq!(call.facts().requests_sent(), 0);
            assert_cost_without_tokens(call.facts(), "0.000000");
            mixed.start().finish(call.facts()).unwrap();
        }
        let snapshot = tally.facts_with_engine(&engine);
        assert_cost_without_tokens(&snapshot, "0.000000");
        let snapshot = mixed.facts_with_engine(&engine);
        assert_cost_without_tokens(&snapshot, "0.000001");
        assert_eq!(snapshot.requests_sent(), 2);
    }
    mixed.run(|| engine.decide(&question, "Third.")).unwrap();
    assert_eq!(
        mixed.facts_with_engine(&engine).estimated_cost_usd(),
        Some("0.000001")
    );
    assert_eq!(mixed.facts().estimated_cost_usd(), Some("0.000000"));
    let plain = super::engine(&listener);
    let unpriced = Tally::new();
    unpriced
        .run(|| plain.decide(&question, "Unpriced."))
        .unwrap();
    assert_eq!(
        unpriced.facts_with_engine(&engine).estimated_cost_usd(),
        None
    );
    assert_eq!(unpriced.facts_with_engine(&engine).input_tokens(), Some(49));
    assert_eq!(listener.count(), 4);
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}

#[cfg(test)]
fn assert_cost_without_tokens(facts: &thinkthen::Facts, cost: &str) {
    assert_eq!(facts.estimated_cost_usd(), Some(cost));
    assert_eq!(facts.input_tokens(), None);
    assert_eq!(facts.output_tokens(), None);
}

#[test]
fn a_cached_call_keeps_raw_token_overflow_checked_and_the_tally_atomic() {
    let listener = Listener::answering(|body| {
        let body: Value = serde_json::from_slice(body).unwrap();
        let tokens = if body["questions"]["q1"]["instructions"].as_str().unwrap().contains("Largest.") { u64::MAX } else { 1 };
        Canned::ok(&json!({"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}},"usage":{"input_tokens":tokens,"output_tokens":0}}).to_string())
    }).unwrap();
    let place = folder();
    let build = || {
        Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .api_key("complete-private")
            .unwrap()
            .prices_usd_per_million("0", "0")
            .unwrap()
    };
    let cached = build().cache_at(&place).unwrap().build().unwrap();
    let live = build().no_cache().build().unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    cached.decide(&question, "Cached.").unwrap();
    let tally = Tally::new();
    tally.run(|| live.decide(&question, "Largest.")).unwrap();
    tally.run(|| cached.decide(&question, "Cached.")).unwrap();
    let before = serde_json::to_value(tally.facts_with_engine(&live)).unwrap();
    assert_eq!(before["estimated_cost_usd"], "0.000000");
    assert!(before.get("input_tokens").is_none());
    let error = tally.run(|| live.decide(&question, "Small.")).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Defect);
    assert_eq!(
        error.detail().message(),
        "defect: tally token count overflowed"
    );
    assert_eq!(error.facts().unwrap().input_tokens(), Some(1));
    assert_eq!(error.facts().unwrap().requests_sent(), 1);
    assert_eq!(
        serde_json::to_value(tally.facts_with_engine(&live)).unwrap(),
        before
    );
    assert_eq!(listener.count(), 3);
    drop(cached);
    std::fs::remove_dir_all(place).unwrap();
}

#[test]
fn tally_retains_reported_input_when_output_is_absent() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}},"usage":{"input_tokens":887}}"#)).unwrap();
    let engine = engine(&listener);
    let question = Question::decide("Refund?").unwrap().cut();
    let tally = Tally::new();
    let call = tally
        .run(|| engine.decide_complete_with(&question, "Refund me.", CallOptions::new()))
        .unwrap();
    assert_eq!(call.facts().input_tokens(), Some(887));
    assert_eq!(call.facts().output_tokens(), None);
    let facts = tally.facts();
    assert_eq!(facts.input_tokens(), Some(887));
    assert_eq!(facts.output_tokens(), None);
    assert_eq!(facts.estimated_cost_usd(), None);
    assert_eq!(listener.count(), 1);
}

#[test]
fn engine_priced_tally_rounds_raw_concurrent_totals_once_and_empty_work_is_zero() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}},"usage":{"input_tokens":49,"output_tokens":0}}"#)).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("complete-private")
        .unwrap()
        .no_cache()
        .prices_usd_per_million("0.01", "0")
        .unwrap()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let tally = Tally::new();
    assert_eq!(
        tally.facts_with_engine(&engine).estimated_cost_usd(),
        Some("0.000000")
    );
    std::thread::scope(|scope| {
        for item in ["First.", "Second."] {
            let tally = tally.clone();
            let engine = &engine;
            let question = &question;
            scope.spawn(move || {
                let call = tally
                    .run(|| engine.decide_complete_with(question, item, CallOptions::new()))
                    .unwrap();
                assert_eq!(call.facts().estimated_cost_usd(), Some("0.000000"));
            });
        }
    });
    assert_eq!(tally.facts().estimated_cost_usd(), Some("0.000000"));
    let snapshot = tally.facts_with_engine(&engine);
    assert_eq!(snapshot.input_tokens(), Some(98));
    assert_eq!(snapshot.output_tokens(), Some(0));
    assert_eq!(snapshot.estimated_cost_usd(), Some("0.000001"));
    assert_eq!(snapshot.requests_sent(), 2);
    assert_eq!(listener.count(), 2);
}

#[test]
fn priced_tally_preserves_partial_usage_and_failed_call_facts_without_fabricated_cost() {
    let seen = AtomicUsize::new(0);
    let listener = Listener::answering(move |_| {
        if seen.fetch_add(1, Ordering::Relaxed) == 0 {
            Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}},"usage":{"input_tokens":887}}"#)
        } else { Canned::status(422, "private rejected body") }
    }).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("complete-private")
        .unwrap()
        .max_retries(0)
        .no_cache()
        .prices_usd_per_million("0.01", "0.02")
        .unwrap()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let tally = Tally::new();
    tally
        .run(|| engine.decide_complete_with(&question, "First.", CallOptions::new()))
        .unwrap();
    let snapshot = tally.facts_with_engine(&engine);
    assert_eq!(snapshot.input_tokens(), Some(887));
    assert_eq!(snapshot.output_tokens(), None);
    assert_eq!(snapshot.estimated_cost_usd(), None);
    let error = tally
        .run(|| engine.decide_complete_with(&question, "Second.", CallOptions::new()))
        .unwrap_err();
    assert_eq!(error.facts().unwrap().requests_sent(), 1);
    let snapshot = tally.facts_with_engine(&engine);
    assert_eq!(snapshot.requests_sent(), 2);
    assert_eq!(snapshot.input_tokens(), None);
    assert_eq!(snapshot.output_tokens(), None);
    assert_eq!(snapshot.estimated_cost_usd(), None);
    assert_eq!(listener.count(), 2);
    assert!(!format!("{error:?}").contains("private rejected body"));
}
