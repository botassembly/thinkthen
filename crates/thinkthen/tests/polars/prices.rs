//! Dataframe prices come from one native call's checked raw token total.
use super::common;
use conformance_backend::{Canned, Listener};
use thinkthen::{BatchSetting, CallOptions, PolarsEngine, Question};

#[test]
fn column_cost_rounds_once_and_omits_overflow_without_poisoning_the_next_call() {
    let listener = Listener::answering(|body| {
        let tokens = if String::from_utf8_lossy(body).contains("overflow") { u64::MAX } else { 49 };
        Canned::ok(&format!(r#"{{"model":"jev-latest","answers":{{"q1":{{"type":"noul","noul":0.9}}}},"usage":{{"input_tokens":{tokens},"output_tokens":0}}}}"#))
    }).expect("listener");
    let engine = common::builder(listener.base())
        .no_cache()
        .prices_usd_per_million("0.01", "0")
        .expect("exact native prices")
        .build()
        .expect("engine");
    let question = Question::decide("Relevant?").expect("question").cut();
    let options = CallOptions::new().batch(BatchSetting::Records(std::num::NonZeroUsize::MIN));
    let call = engine
        .decide_series(&question, &common::column(&["first", "second"]), options)
        .expect("column");
    assert_eq!(call.facts().input_tokens(), Some(98));
    assert_eq!(call.facts().output_tokens(), Some(0));
    assert_eq!(call.facts().estimated_cost_usd(), Some("0.000001"));
    let overflow = engine
        .decide_series(
            &question,
            &common::column(&["overflow one", "overflow two"]),
            options,
        )
        .expect("answers survive unknown usage");
    assert_eq!(overflow.facts().requests_sent(), 2);
    assert_eq!(overflow.facts().input_tokens(), None);
    assert_eq!(overflow.facts().estimated_cost_usd(), None);
    let clean = engine
        .decide_series(
            &question,
            &common::column(&["fresh one", "fresh two"]),
            options,
        )
        .expect("next call");
    assert_eq!(clean.facts().estimated_cost_usd(), Some("0.000001"));
    assert_eq!(listener.count(), 6);
}

#[test]
fn priced_recognition_collection_refuses_before_any_send() {
    let listener = Listener::answering(|_| Canned::ok("{}")).expect("listener");
    let engine = common::builder(listener.base())
        .prices_usd_per_million("0.01", "0")
        .expect("prices")
        .build()
        .expect("engine");
    let ask = thinkthen::Recognize::builder().build().expect("recognize");
    let error = engine
        .recognize_series(&ask, &common::column(&["one note"]), CallOptions::new())
        .expect_err("native aggregate pricing is required");
    assert_eq!(error.kind(), thinkthen::ErrorKind::Usage);
    assert_eq!(
        error.to_string(),
        "priced recognition collections require native aggregate pricing"
    );
    assert_eq!(listener.count(), 0);
}
