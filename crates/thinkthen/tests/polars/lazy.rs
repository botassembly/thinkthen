//! The public lazy expression family, using real Polars collection and wire sends.

#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

use crate::common;

use conformance_backend::{Canned, Listener};
use serde_json::{Map, Value, json};
use thinkthen::polars::prelude::{
    DataFrame, DataType, IntoLazy, LazyFrame, NamedFrom, Series, col,
};
use thinkthen::{
    BatchSetting, CancelToken, Engine, PolarsEngine, PolarsExprOptions, Question, Tally,
};

fn listener() -> Listener {
    Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request JSON");
        let answers: Map<String, Value> = request
            .get("questions")
            .and_then(Value::as_object)
            .expect("questions")
            .iter()
            .map(|(name, question)| {
                let answer = match question.get("type").and_then(Value::as_str) {
                    Some("choice") => json!({"type":"choice","choice":"billing",
                        "probabilities":{"billing":0.9,"other":0.1}}),
                    Some("score") => json!({"type":"score","score":0.8,
                        "legend":{"0":"low","1":"high"},
                        "probabilities":{"0":0.2,"1":0.8}}),
                    _ => json!({"type":"noul","noul":0.9}),
                };
                (name.clone(), answer)
            })
            .collect();
        Canned::ok(
            &json!({"model":"jev-1.13.0","answers":answers,
            "usage":{"input_tokens":10,"output_tokens":2}})
            .to_string(),
        )
    })
    .expect("listener")
}

fn engine(base: &str) -> Engine {
    common::builder(base)
        .no_cache()
        .build()
        .expect("source engine")
}

fn frame(texts: &[Option<&str>]) -> DataFrame {
    DataFrame::new(texts.len(), vec![Series::new("body".into(), texts).into()]).expect("frame")
}

fn decide() -> Question {
    Question::decide("Does this ask for a refund?")
        .expect("question")
        .cut()
}

fn assert_probability_matches_eager(result: &DataFrame, eager: &DataFrame) {
    let judged = result
        .column("judged")
        .expect("judged")
        .struct_()
        .expect("Struct");
    assert_eq!(
        judged.dtype(),
        &DataType::Struct(vec![
            thinkthen::polars::prelude::Field::new("value".into(), DataType::Boolean),
            thinkthen::polars::prelude::Field::new("probability".into(), DataType::Float64),
        ])
    );
    for name in ["value", "probability"] {
        let actual = judged.field_by_name(name).expect("field");
        let expected = eager.column(name).expect("eager field");
        assert_eq!(actual, expected.as_materialized_series().clone());
    }
}

#[test]
fn lazy_matches_eager_values_bodies_and_shared_tally() {
    let listener = listener();
    let engine = engine(listener.base());
    let source = frame(&[Some("Refund me"), None, Some("Please refund this")]);
    let question = decide();
    let eager = engine
        .column_with(
            &question,
            source
                .column("body")
                .expect("body")
                .as_materialized_series(),
            thinkthen::PolarsCallOptions::new(),
        )
        .expect("eager probability");
    assert_eq!(eager.facts().records(), 2);
    assert_eq!(eager.facts().requests_sent(), 1);
    let eager_body = listener.requests()[0].body.clone();
    let tally = Tally::new();
    let options = PolarsExprOptions::new().tally(tally.clone());
    let ordinary = engine
        .decide_expr(&question, col("body"), options.clone())
        .expect("ordinary expression");
    let singleton = engine
        .decide_expr(
            &question,
            col("body"),
            options.batch(BatchSetting::Records(std::num::NonZeroUsize::MIN)),
        )
        .expect("singleton expression");
    assert_eq!(listener.count(), 1, "both constructions send nothing");
    for (single, expression) in [(false, ordinary), (true, singleton)] {
        let result = source
            .clone()
            .lazy()
            .with_columns([expression.alias("judged")])
            .collect()
            .expect("lazy collect");
        assert_probability_matches_eager(&result, eager.value());
        let requests = listener.requests();
        if single {
            let mut actual = requests
                .iter()
                .map(|request| request.body.clone())
                .collect::<Vec<_>>();
            actual.sort();
            let mut expected = [
                r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me\". Does this ask for a refund?"}}}"#.as_bytes().to_vec(),
                r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"Please refund this\". Does this ask for a refund?"}}}"#.as_bytes().to_vec(),
            ];
            expected.sort();
            assert_eq!(actual, expected, "batch one gives quoted singleton bodies");
        } else {
            assert_eq!(requests.len(), 1);
            assert_eq!(
                requests[0].body, eager_body,
                "ordinary lazy packs like eager"
            );
            assert_eq!(tally.facts().input_tokens(), Some(10));
        }
    }
    assert_eq!(
        listener.count(),
        4,
        "eager and ordinary packed; batch one sent singletons"
    );
    assert_eq!(
        (tally.facts().records(), tally.facts().requests_sent()),
        (4, 3)
    );
}

#[test]
fn each_expression_returns_typed_value_and_nullable_probability() {
    let listener = listener();
    let engine = engine(listener.base());
    let source = frame(&[Some("Refund me")]);
    let choose = Question::choose_labels("Which team?")
        .expect("question")
        .label("billing", None)
        .expect("label")
        .label("other", None)
        .expect("label")
        .build()
        .expect("choose");
    let score = Question::score("How urgent?")
        .and_then(|builder| builder.level("low", None))
        .and_then(|builder| builder.level("high", None))
        .and_then(thinkthen::ScoreBuilder::build)
        .expect("score");
    let tag = Question::tag_labels("Which tags?")
        .expect("question")
        .label("billing", None)
        .expect("label")
        .label("other", None)
        .expect("label")
        .build()
        .expect("tag");
    assert_eq!(listener.count(), 0, "invalid construction sends nothing");
    let choose_expr = engine
        .choose_expr(&choose, col("body"), PolarsExprOptions::new())
        .expect("choose expression");
    let score_expr = engine
        .score_expr(&score, col("body"), PolarsExprOptions::new())
        .expect("score expression");
    let tag_expr = engine
        .tag_expr(&tag, col("body"), PolarsExprOptions::new())
        .expect("tag expression");
    let result = source
        .lazy()
        .with_columns([
            choose_expr.alias("chosen"),
            score_expr.alias("scored"),
            tag_expr.alias("tagged"),
        ])
        .collect()
        .expect("all kinds");
    assert_expression_values(&result);
    assert_eq!(listener.count(), 3);
}

fn assert_expression_values(result: &DataFrame) {
    let chosen = result
        .column("chosen")
        .expect("chosen")
        .struct_()
        .expect("Struct");
    assert_eq!(
        chosen
            .field_by_name("value")
            .expect("value")
            .str()
            .expect("String")
            .get(0),
        Some("billing")
    );
    assert_eq!(
        chosen
            .field_by_name("probability")
            .expect("probability")
            .f64()
            .expect("Float64")
            .get(0),
        Some(0.9)
    );
    assert_eq!(
        result
            .column("scored")
            .expect("score")
            .struct_()
            .expect("Struct")
            .field_by_name("value")
            .expect("value")
            .f64()
            .expect("Float64")
            .get(0),
        Some(0.8)
    );
    assert_eq!(
        result
            .column("tagged")
            .expect("tags")
            .struct_()
            .expect("Struct")
            .field_by_name("value")
            .expect("value")
            .list()
            .expect("List")
            .get(0)
            .expect("labels")
            .len(),
        2
    );
}

#[test]
fn a_shared_token_stops_a_later_evaluation_without_a_send() {
    let listener = listener();
    let engine = engine(listener.base());
    let token = CancelToken::new();
    let tally = Tally::new();
    let expression = engine
        .decide_expr(
            &decide(),
            col("body"),
            PolarsExprOptions::new()
                .token(token.clone())
                .tally(tally.clone()),
        )
        .expect("expression");
    let source = frame(&[Some("Refund me")]);
    source
        .clone()
        .lazy()
        .with_columns([expression.clone().alias("judged")])
        .collect()
        .expect("first collect");
    assert_eq!(listener.count(), 1);
    token.cancel();
    let error = source
        .lazy()
        .with_columns([expression.alias("judged")])
        .collect()
        .expect_err("cancelled second collect");
    assert!(error.to_string().contains("cancel"), "{error}");
    assert_eq!(listener.count(), 1, "no later request was admitted");
    assert_eq!(tally.facts().requests_sent(), 1);
}

/// Ticket 0304 slice 3c: a cached lazy frame collected twice sends nothing
/// the second time, and a later slice of one frame sends only its new
/// questions. Counted on the loopback listener.
#[test]
fn a_cached_lazy_frame_collected_twice_sends_nothing_the_second_time() {
    let listener = listener();
    let engine = common::engine(listener.base());
    let texts = common::distinct(30);
    let cells = texts
        .iter()
        .map(|text| Some(text.as_str()))
        .collect::<Vec<_>>();
    let source = frame(&cells);
    let expression = engine
        .decide_expr(&decide(), col("body"), PolarsExprOptions::new())
        .expect("expression");
    let collect = |rows: LazyFrame| {
        rows.with_columns([expression.clone().alias("judged")])
            .collect()
            .expect("lazy collect")
    };
    let first = collect(source.clone().lazy());
    assert_eq!(
        (listener.count(), listener.questions()),
        (1, 30),
        "the first collect asks every row in one request"
    );
    let second = collect(source.clone().lazy());
    assert_eq!(listener.count(), 1, "the second collect sends nothing");
    assert_eq!(first, second, "the cache answers each row alike");

    let head = collect(source.clone().lazy().slice(0, 10));
    assert_eq!(head.height(), 10);
    assert_eq!(listener.count(), 1, "a cached slice sends nothing");
    let wider = frame(
        &common::distinct(40)
            .iter()
            .map(|text| Some(text.as_str()))
            .collect::<Vec<_>>(),
    );
    let tail = collect(wider.lazy().slice(25, 10));
    assert_eq!(tail.height(), 10);
    assert_eq!(
        (listener.count(), listener.questions()),
        (2, 35),
        "the later slice asks only its own five new rows"
    );
}

#[test]
fn annotation_and_recognition_expressions_keep_generated_nested_types_and_null_rows() {
    let corpus: Value =
        serde_json::from_str(include_str!("../../../../conformance/cases.json")).expect("corpus");
    let cases = corpus["cases"].as_array().expect("cases");
    for id in ["17-annotate-mixed", "42-recognize-C01-relations"] {
        let case = cases.iter().find(|case| case["id"] == id).expect("case");
        let responses = case["exchanges"]
            .as_array()
            .expect("exchanges")
            .iter()
            .map(|exchange| Canned::ok(&exchange["response"].to_string()))
            .collect();
        let listener = Listener::serving(responses).expect("saved listener");
        let engine = engine(listener.base());
        let text = case
            .get("text")
            .and_then(Value::as_str)
            .or_else(|| case["exchanges"][0]["evidence"].as_str())
            .expect("text");
        let source = frame(&[Some(text), None]);
        let expression = if id.starts_with("17") {
            let questions =
                thinkthen::QuestionSet::from_json(&case["question_set"].to_string()).expect("set");
            engine
                .annotate_expr(&questions, col("body"), PolarsExprOptions::new())
                .expect("annotation expression")
        } else {
            let question = thinkthen::Recognize::from_json(&case["question"].to_string())
                .expect("recognition");
            engine
                .recognize_expr(&question, col("body"), PolarsExprOptions::new())
                .expect("recognition expression")
        };
        assert_eq!(listener.count(), 0, "building an expression sends nothing");
        let projected = source
            .lazy()
            .select([expression])
            .collect()
            .expect("nested typed results");
        let results = projected
            .column("body")
            .expect("result")
            .struct_()
            .expect("Struct");
        assert_eq!((results.len(), results.null_count()), (2, 1));
        let values = results.field_by_name("value").expect("value");
        let probability = results.field_by_name("probability").expect("probability");
        if id.starts_with("17") {
            let values = values.struct_().expect("named typed annotation");
            for (name, dtype) in [
                ("refund", DataType::Boolean),
                ("team", DataType::String),
                ("severity", DataType::Float64),
                ("topics", DataType::List(Box::new(DataType::String))),
            ] {
                assert_eq!(
                    values.field_by_name(name).expect("typed member").dtype(),
                    &dtype
                );
            }
            assert_eq!(probability.null_count(), 2);
        } else {
            let entities = values
                .struct_()
                .expect("recognition struct")
                .field_by_name("entities")
                .expect("entities");
            assert_eq!(
                entities
                    .list()
                    .expect("List(Struct)")
                    .get_as_series(0)
                    .expect("entity list")
                    .len(),
                3
            );
            assert_eq!(
                probability.dtype(),
                &DataType::List(Box::new(DataType::Float64))
            );
            assert_eq!(
                probability
                    .list()
                    .expect("List(Float64)")
                    .get_as_series(0)
                    .expect("span probabilities")
                    .len(),
                3
            );
            assert_eq!(probability.null_count(), 1);
        }
    }
}
