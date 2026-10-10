//! The door's own rules: its refusals, the failed marker, a failed decide
//! row, chunked and sliced columns, the caller's frame columns, and empty
//! columns. Only `batching` sets a throttle, of one, and no request is held.

#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

mod batching;
use crate::common;

use conformance_backend::Backend;
use thinkthen::polars::prelude::{DataFrame, DataType, IntoColumn, NamedFrom, Series};
use thinkthen::{
    CallOptions, Error, ErrorKind, LoadedQuestion, PolarsEngine, Question, QuestionSet,
};

fn decide() -> Question {
    Question::decide("Does this ask for a refund?")
        .expect("a question")
        .cut()
}

fn score() -> Question {
    Question::score("How urgent is this?")
        .and_then(|builder| builder.level("low", None))
        .and_then(|builder| builder.level("high", None))
        .and_then(thinkthen::ScoreBuilder::build)
        .expect("a score question")
}

fn frame(columns: Vec<Series>) -> DataFrame {
    let height = columns.first().map_or(0, |series| series.len());
    DataFrame::new(
        height,
        columns.into_iter().map(IntoColumn::into_column).collect(),
    )
    .expect("a frame")
}

#[path = "plan_probability.rs"]
mod plan_probability;

#[test]
fn a_profiled_question_keeps_its_identity_through_a_series() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("../../../../conformance/calibration.json"))
            .expect("shared calibration fixture");
    let question = match Question::from_json(&fixture["question"].to_string()).expect("question") {
        LoadedQuestion::Question(question) => Some(question),
        LoadedQuestion::Banded(_) => None,
    }
    .expect("the fixture is one decide question");
    let backend = Backend::start().expect("backend");
    let engine = common::builder(&format!("{}/generic/v1", backend.origin()))
        .profile_json(&fixture["runtime_profile"].to_string())
        .expect("runtime profile")
        .build()
        .expect("engine");
    let evidence = fixture["evidence"].as_str().expect("evidence");
    let answered = engine
        .decide_series(&question, &common::column(&[evidence]), CallOptions::new())
        .expect("series");
    assert_eq!(
        answered.value().bool().expect("boolean series").get(0),
        Some(true)
    );
    let details = engine.details(&question, evidence).expect("details");
    assert_eq!(
        details.value().question_sha256(),
        fixture["question_sha256"]
    );
    assert_eq!(details.value().profile_warning(), Some(("old", "new")));
    assert_eq!(backend.count(), 1);
}

/// Each refusal names its cause whole, in `Display` and `Debug`, and sends nothing.
#[test]
fn every_refusal_is_pinned_and_sends_nothing() {
    let backend = Backend::start().expect("a backend");
    let engine = common::engine(&format!("{}/generic/v1", backend.origin()));
    let options = CallOptions::new;
    let counts = Series::new("counts".into(), [1i64, 2, 3]);
    let flags = Series::new("flags".into(), [true, false]);
    let nulls = Series::new("body".into(), [Some("refund me"), None]);
    let texts = frame(vec![common::column(&["refund me"])]);
    let set = QuestionSet::from_json(
        r#"{"version": 1, "questions": {"body": {"decide": "Does this ask for a refund?"}}}"#,
    )
    .expect("a set");
    let reserved = QuestionSet::from_json(
        r#"{"version": 1, "questions": {"failed": {"decide": "Does this ask for a refund?"}}}"#,
    )
    .expect("a reserved set");
    let other = QuestionSet::from_json(
        r#"{"version": 1, "questions": {"refund": {"decide": "Does this ask for a refund?"}}}"#,
    )
    .expect("another set");
    let refusals: Vec<(Result<(), Error>, &str)> = vec![
        (
            engine
                .decide_series(&decide(), &counts, options())
                .map(drop),
            "the column counts is i64, not text",
        ),
        (
            engine.decide_series(&decide(), &flags, options()).map(drop),
            "the column flags is bool, not text",
        ),
        (
            engine.tag_series(&score(), &nulls, options()).map(drop),
            "question kind does not match the requested function",
        ),
        (
            engine
                .score_series(&decide(), &common::column(&["refund me"]), options())
                .map(drop),
            "this function does not accept this threshold",
        ),
        (
            engine
                .annotate_frame(&set, &texts, "text", options())
                .map(drop),
            "the frame holds no column text",
        ),
        (
            engine
                .annotate_frame(&set, &texts, "body", options())
                .map(drop),
            "the frame already holds a column named body",
        ),
        (
            engine
                .annotate_frame(&reserved, &texts, "body", options())
                .map(drop),
            "the question name failed is reserved for frame failures",
        ),
        (
            engine
                .annotate_frame(
                    &other,
                    &frame(vec![
                        common::column(&["refund me"]),
                        Series::new("failed".into(), [1i64]),
                    ]),
                    "body",
                    options(),
                )
                .map(drop),
            "the frame already holds a column named failed",
        ),
    ];
    for (refused, sentence) in refusals {
        let error = refused.expect_err(sentence);
        assert_eq!(error.kind(), ErrorKind::Usage, "{sentence}");
        assert_eq!(error.to_string(), sentence);
        assert_eq!(
            format!("{error:?}"),
            format!(
                "Usage(ErrorDetail {{ message: {sentence:?}, retryable: false, send_budget_denial: None, facts: None }})"
            )
        );
    }
    assert_eq!(backend.count(), 0, "a refusal reached the backend");
}

/// Shared case 17 keeps typed answers and the exact nested failure cause.
#[test]
fn a_failed_question_keeps_typed_columns_and_a_marker() {
    let backend = Backend::start().expect("a backend");
    let engine = common::engine(&format!("{}/case/17-annotate-partial/v1", backend.origin()));
    let set = QuestionSet::from_json(
        r#"{"version": 1, "questions": {
            "refund": {"decide": "Does this ask for a refund?", "threshold": "0.1:0.9"},
            "team": {"choose": "Which team owns this?", "options": ["billing", "other"], "threshold": 0.5},
            "severity": {"score": "How severe is this?", "levels": ["low", "medium", "high"]},
            "topics": {"tag": "Which labels apply?", "labels": ["billing", "urgent"], "threshold": 0.5}
        }}"#,
    )
    .expect("the case's set");
    let texts = frame(vec![Series::new(
        "body".into(),
        [None, Some("Refund requested for a failed payment."), None],
    )]);
    let out = engine
        .annotate_frame(&set, &texts, "body", CallOptions::new())
        .expect("the frame");
    let team = out.value().column("team").expect("team");
    assert_eq!(team.dtype(), &DataType::String);
    assert_eq!(team.null_count(), 3);
    let refund = out.value().column("refund").expect("refund");
    assert_eq!(refund.dtype(), &DataType::Boolean);
    assert_eq!(refund.null_count(), 3, "not sure reads null");
    let severity = out.value().column("severity").expect("severity");
    assert_eq!(severity.f64().expect("Float64").get(1), Some(1.2));
    assert!(matches!(
        out.value().column("topics").expect("topics").dtype(),
        DataType::List(_)
    ));
    let failed = out
        .value()
        .column("failed")
        .expect("failed")
        .struct_()
        .expect("Struct");
    assert_eq!(failed.null_count(), 2);
    let team = failed.field_by_name("team").expect("team marker");
    let marker = team
        .struct_()
        .expect("question Struct")
        .field_by_name("failed")
        .expect("marker");
    assert_eq!(
        marker
            .struct_()
            .expect("marker Struct")
            .field_by_name("kind")
            .expect("kind")
            .str()
            .expect("text")
            .get(1),
        Some("backend")
    );
    assert_eq!(
        marker
            .struct_()
            .expect("marker Struct")
            .field_by_name("cause")
            .expect("cause")
            .str()
            .expect("text")
            .get(1),
        Some("missing_probability")
    );
}

/// A failed row ends a series call with the engine's error.
#[test]
fn a_failed_row_ends_a_series_call() {
    let backend = Backend::start().expect("a backend");
    let engine = common::engine(&format!(
        "{}/arm/malformed/invalid_probability/v1",
        backend.origin()
    ));
    let error = engine
        .decide_series(
            &decide(),
            &common::column(&["one", "two"]),
            CallOptions::new(),
        )
        .expect_err("a failed row ends the call");
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert_eq!(
        error
            .facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((1, 1))
    );
    assert_eq!(error.to_string(), "a backend question failed in a batch");
    // A one-question reply whose only answer failed is refused whole by the
    // engine's reply reader, so a failed row ends a score column the way it
    // ends a decide column. No cell can hold the marker there.
    let malformed = common::engine(&format!(
        "{}/arm/malformed/missing_probability/v1",
        backend.origin()
    ));
    let error = malformed
        .score_series(
            &score(),
            &common::column(&["one", "two"]),
            CallOptions::new(),
        )
        .expect_err("a failed row ends the call");
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert_eq!(error.to_string(), "a backend question failed in a batch");
}

/// A three-chunk column sliced at offset 5 gives each row its own answer
/// (error-index R1-3 with R2-12, on the texts of shared case `27-decide-many`).
#[test]
fn a_chunked_and_sliced_column_answers_each_row() {
    let backend = Backend::start().expect("a backend");
    let engine = common::engine(&format!("{}/case/27-decide-many/v1", backend.origin()));
    let question = Question::decide("Does the writer ask for a refund?")
        .and_then(|builder| builder.cut_at(0.5))
        .expect("the case's question");
    let mut chunked = common::column(&["pad", "pad", "pad", "pad", "pad", "refund now"]);
    chunked
        .append(&common::column(&["good morning", "maybe so"]))
        .expect("a second chunk");
    chunked
        .append(&common::column(&["refund again", "bye"]))
        .expect("a third chunk");
    assert_eq!(chunked.n_chunks(), 3);
    let sliced = chunked.slice(5, 5);
    let answered = engine
        .decide_series(
            &question,
            &sliced,
            CallOptions::new().batch(thinkthen::BatchSetting::Records(
                std::num::NonZeroUsize::MIN,
            )),
        )
        .expect("the answers");
    let answers: Vec<Option<bool>> = answered.value().bool().expect("Boolean").iter().collect();
    assert_eq!(
        answers,
        [Some(true), Some(false), Some(true), Some(true), Some(false)]
    );
    assert_eq!(answered.value().name().as_str(), "body");
}

/// The caller's number, Boolean, and list columns come back with their
/// dtypes and values (error-index R1-4).
#[test]
fn the_callers_columns_come_back_unchanged() {
    let backend = Backend::start().expect("a backend");
    let engine = common::engine(&format!("{}/generic/v1", backend.origin()));
    let counted = Series::new("counted".into(), [1i64, 2]);
    let flagged = Series::new("flagged".into(), [true, false]);
    let listed = Series::new(
        "listed".into(),
        [
            Series::new("".into(), [1i64]),
            Series::new("".into(), [2i64, 3]),
        ],
    );
    let theirs = vec![
        common::column(&["refund me", "hello"]),
        counted,
        flagged,
        listed,
    ];
    let set = QuestionSet::from_json(
        r#"{"version": 1, "questions": {"refund": {"decide": "Does this ask for a refund?"}}}"#,
    )
    .expect("a set");
    let out = engine
        .annotate_frame(&set, &frame(theirs.clone()), "body", CallOptions::new())
        .expect("the frame");
    for series in &theirs {
        let back = out
            .value()
            .column(series.name())
            .expect("the caller's column");
        assert_eq!(back.dtype(), series.dtype(), "{}", series.name());
        assert!(
            back.as_materialized_series().equals_missing(series),
            "{}",
            series.name()
        );
    }
    assert_eq!(out.value().width(), theirs.len() + 2);
    assert_eq!(
        out.value()
            .column("failed")
            .expect("failure column")
            .null_count(),
        out.value().height()
    );
}

/// Empty, all-null and duplicate cells keep their physical positions and native dtypes.
#[test]
fn nullable_rowwise_columns_keep_types_positions_and_duplicate_answers() {
    let backend = Backend::start().expect("a backend");
    let engine = common::engine(&format!("{}/generic/v1", backend.origin()));
    let empty = common::column(&[]);
    let options = CallOptions::new;
    let choose = Question::choose_labels("Which team owns this?")
        .and_then(|builder| builder.label("billing", None))
        .and_then(|builder| builder.label("other", None))
        .and_then(thinkthen::LabelBuilder::build)
        .expect("a choose question");
    let tag = Question::tag_labels("Which labels apply?")
        .and_then(|builder| builder.label("billing", None))
        .and_then(|builder| builder.cut_at(0.5))
        .expect("a tag question");
    for input in [
        empty.clone(),
        Series::new("body".into(), [None::<&str>, None]),
        Series::new("body".into(), [Some("refund me"), None, Some("refund me")]),
    ] {
        let answered = [
            (
                engine.decide_series(&decide(), &input, options()),
                DataType::Boolean,
            ),
            (
                engine.choose_series(&choose, &input, options()),
                DataType::String,
            ),
            (
                engine.score_series(&score(), &input, options()),
                DataType::Float64,
            ),
            (
                engine.tag_series(&tag, &input, options()),
                DataType::List(Box::new(DataType::String)),
            ),
        ];
        for (series, dtype) in answered {
            let series = series.expect("a typed answer");
            assert_eq!(
                (series.value().len(), series.value().dtype()),
                (input.len(), &dtype)
            );
            assert_eq!(series.value().null_count(), input.null_count());
            for at in 0..input.len() {
                assert_eq!(
                    series.value().get(at).expect("cell").is_null(),
                    input.get(at).expect("input").is_null()
                );
            }
            if input.len() == 3 {
                assert_eq!(
                    series.value().get(0).expect("first"),
                    series.value().get(2).expect("duplicate")
                );
            }
            assert_eq!(
                (series.facts().records(), series.facts().requests_sent()),
                (
                    (input.len() - input.null_count()) as u64,
                    u64::from(input.null_count() < input.len())
                )
            );
        }
    }
    let set = QuestionSet::from_json(
        r#"{"version": 1, "questions": {"refund": {"decide": "Does this ask for a refund?"}}}"#,
    )
    .expect("a set");
    let output = engine
        .annotate_frame(&set, &frame(vec![empty]), "body", options())
        .expect("empty frame");
    assert_eq!(
        (output.facts().records(), output.facts().requests_sent()),
        (0, 0)
    );
    assert_eq!(output.value().height(), 0);
    assert_eq!(
        output.value().column("refund").expect("answer").dtype(),
        &DataType::Boolean
    );
    assert!(matches!(
        output
            .value()
            .column("failed")
            .expect("failure column")
            .dtype(),
        DataType::Struct(_)
    ));
    assert_eq!(backend.count(), 4);
}

/// Cancellation admits no provider work through any named rowwise Request.
#[test]
fn cancelled_rowwise_columns_send_nothing() {
    let backend = Backend::start().expect("backend");
    let engine = common::engine(&format!("{}/generic/v1", backend.origin()));
    let token = thinkthen::CancelToken::new();
    token.cancel();
    let options = || CallOptions::new().cancel(&token);
    let texts = Series::new("body".into(), [None, Some("refund me"), None]);
    let choose = Question::choose_labels("Which team?")
        .and_then(|builder| builder.label("billing", None))
        .and_then(|builder| builder.label("other", None))
        .and_then(thinkthen::LabelBuilder::build)
        .expect("choose");
    let tag = Question::tag_labels("Which labels?")
        .and_then(|builder| builder.label("billing", None))
        .and_then(|builder| builder.cut_at(0.5))
        .expect("tag");
    let set = QuestionSet::builder()
        .question("refund", decide())
        .expect("member")
        .build()
        .expect("set");
    let refused = [
        engine.decide_series(&decide(), &texts, options()).map(drop),
        engine.choose_series(&choose, &texts, options()).map(drop),
        engine.score_series(&score(), &texts, options()).map(drop),
        engine.tag_series(&tag, &texts, options()).map(drop),
        engine
            .annotate_frame(&set, &frame(vec![texts]), "body", options())
            .map(drop),
    ];
    for refused in refused {
        assert_eq!(refused.expect_err("cancelled").kind(), ErrorKind::Cancelled);
    }
    assert_eq!(backend.count(), 0);
}
