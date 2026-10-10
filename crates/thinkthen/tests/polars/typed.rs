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
            examples: None,
            seed_spans: None,
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

#[test]
fn complete_score_keeps_owned_nontext_originals_and_nullable_positions() {
    struct Original(u32);
    impl thinkthen::InputEvidence for Original {
        fn question_input(&self) -> thinkthen::QuestionInput {
            thinkthen::QuestionInput::Text(format!("item {}", self.0))
        }
    }
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"score","probabilities":{"0":0.25,"1":0.75}}},"usage":{"input_tokens":49,"output_tokens":0}}"#)).expect("listener");
    let engine = common::engine(listener.base());
    let question = Question::score("How urgent?")
        .expect("score")
        .level("low", None)
        .expect("low")
        .level("high", None)
        .expect("high")
        .build()
        .expect("question");
    let column = Series::new("original".into(), [Some(7u32), None, Some(7)]);
    let records = [Some(7), None, Some(7)]
        .into_iter()
        .map(|value| {
            value.map(|value| RecordInput {
                original: Original(value),
                context: None,
                options: None,
                seed_spans: None,
                examples: None,
            })
        })
        .collect();
    let (call, positions) = engine
        .score_input_column_complete(&question, &column, records, CallOptions::new())
        .expect("score");
    assert_eq!(positions, [0, 2]);
    assert_eq!(
        column.dtype(),
        &thinkthen::polars::prelude::DataType::UInt32
    );
    for (at, row) in call.value().iter().enumerate() {
        assert_eq!(row.ordinal(), at);
        assert_eq!(row.original().0, 7);
        assert_eq!(row.result().value(), 0.75);
        assert!(!row.result().answer_id().as_str().is_empty());
    }
    assert_eq!(call.facts().records(), 2);
    assert_eq!(listener.count(), 1);
    let texts = Series::new("text".into(), [Some("item 7"), None, Some("item 7")]);
    let (call, positions) = engine
        .score_series_complete(&question, &texts, CallOptions::new())
        .expect("text score");
    assert_eq!(positions, [0, 2]);
    assert_eq!(call.value().len(), 2);
    assert_eq!(call.value()[1].original(), "item 7");
    assert_eq!(call.value()[1].result().value(), 0.75);
}

#[test]
fn complete_score_empty_refused_and_cancelled_columns_send_nothing() {
    let listener = Listener::answering(|_| Canned::ok("{}")).expect("listener");
    let engine = common::engine(listener.base());
    let question = Question::score("How urgent?")
        .expect("score")
        .level("low", None)
        .expect("low")
        .level("high", None)
        .expect("high")
        .build()
        .expect("question");
    for cells in [vec![], vec![None, None]] {
        let column = Series::new("text".into(), cells as Vec<Option<&str>>);
        let (call, positions) = engine
            .score_series_complete(&question, &column, CallOptions::new())
            .expect("empty score");
        assert!(positions.is_empty());
        assert!(call.value().is_empty());
        assert_eq!(call.facts().requests_sent(), 0);
    }
    let column = Series::new("text".into(), ["one"]);
    let wrong = Question::decide("Relevant?").expect("decide").cut();
    assert_eq!(
        engine
            .score_series_complete(&wrong, &column, CallOptions::new())
            .expect_err("wrong kind")
            .kind(),
        thinkthen::ErrorKind::Usage
    );
    let token = thinkthen::CancelToken::new();
    token.cancel();
    let error = engine
        .score_series_complete(&question, &column, CallOptions::new().cancel(&token))
        .expect_err("cancelled");
    assert_eq!(error.kind(), thinkthen::ErrorKind::Cancelled);
    assert_eq!(listener.count(), 0);
}
