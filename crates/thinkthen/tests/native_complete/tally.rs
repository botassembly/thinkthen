use super::*;
use thinkthen::Tally;

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
