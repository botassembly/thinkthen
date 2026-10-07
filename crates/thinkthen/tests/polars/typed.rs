//! Complete frame calls keep native occurrences and nullable presentation separately.
use super::common;
use conformance_backend::{Canned, Listener};
use thinkthen::polars::prelude::{NamedFrom, Series};
use thinkthen::{CallOptions, Question, RecordInput};

#[test]
fn complete_text_column_retains_native_ids_and_original_nullable_positions() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":49,"output_tokens":0}}"#)).expect("listener");
    let engine = common::builder(listener.base())
        .no_cache()
        .batch(thinkthen::BatchSetting::Records(
            std::num::NonZeroUsize::MIN,
        ))
        .prices_usd_per_million("0.01", "0")
        .expect("prices")
        .build()
        .expect("engine");
    let question = Question::decide("Relevant?").expect("question").cut();
    let column = Series::new("original".into(), [Some("same"), None, Some("same")]);
    let (call, positions) = engine
        .decide_series_complete(&question, &column, CallOptions::new())
        .expect("complete column");
    assert_eq!(positions, [0, 2]);
    assert_eq!(column.name().as_str(), "original");
    assert_eq!(
        call.value().iter().map(|r| r.ordinal()).collect::<Vec<_>>(),
        [0, 1]
    );
    assert_eq!(call.value()[0].original(), "same");
    assert_eq!(call.value()[1].original(), "same");
    assert!(!call.value()[0].result().answer_id().as_str().is_empty());
    assert!(!call.value()[1].result().answer_id().as_str().is_empty());
    assert_eq!(call.facts().estimated_cost_usd(), Some("0.000000"));
    assert_eq!(call.facts().input_tokens(), Some(49));
    assert_eq!(call.facts().records(), 2);
    assert_eq!(listener.count(), 1);
}

#[test]
fn typed_column_refuses_different_counts_or_null_positions_before_sending() {
    let listener = Listener::answering(|_| Canned::ok("{}")).expect("listener");
    let engine = common::engine(listener.base());
    let question = Question::decide("Relevant?").expect("question").cut();
    let column = Series::new("body".into(), [Some("a"), None]);
    let record = || {
        Some(RecordInput {
            original: "a".to_owned(),
            context: None,
            options: None,
        })
    };
    for (records, want) in [
        (
            vec![record()],
            "the typed input column has a different row count",
        ),
        (
            vec![None, record()],
            "the typed input column has different null positions",
        ),
    ] {
        let error = engine
            .decide_input_column_complete(&question, &column, records, CallOptions::new())
            .expect_err("invalid association");
        assert_eq!(error.kind(), thinkthen::ErrorKind::Usage);
        assert_eq!(error.to_string(), want);
    }
    assert_eq!(listener.count(), 0);
}
