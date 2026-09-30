//! The public lazy expression family, using real Polars collection and wire sends.

#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

mod common;

use conformance_backend::{Canned, Listener};
use serde_json::{Map, Value, json};
use thinkthen::polars::prelude::{
    DataFrame, DataType, IntoLazy, LazyFrame, NamedFrom, Series, col,
};
use thinkthen::{
    BatchSetting, CancelToken, Engine, PolarsEngine, PolarsExprOptions, Question, QuestionKind,
    Tally,
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

fn probability_refuses_for_score_and_tag(engine: &Engine, score: &Question, tag: &Question) {
    for question in [score, tag] {
        let refused = match question.kind() {
            QuestionKind::Score => engine.score_expr(
                question,
                col("body"),
                PolarsExprOptions::new().probability(true),
            ),
            QuestionKind::Tag => engine.tag_expr(
                question,
                col("body"),
                PolarsExprOptions::new().probability(true),
            ),
            _ => unreachable!("score and tag only"),
        };
        let error = refused.expect_err("probability refusal");
        assert_eq!(error.kind(), thinkthen::ErrorKind::Usage);
        assert!(
            error
                .to_string()
                .contains("probability belongs to decide and choose")
        );
    }
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
        .probability_frame(
            &question,
            source
                .column("body")
                .expect("body")
                .as_materialized_series(),
            thinkthen::CallOptions::new(),
        )
        .expect("eager probability");
    assert_eq!(eager.facts().records(), 2);
    assert_eq!(eager.facts().requests_sent(), 1);
    let eager_body = listener.requests()[0].body.clone();
    let tally = Tally::new();
    let options = PolarsExprOptions::new()
        .probability(true)
        .tally(tally.clone());
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
fn each_expression_keeps_its_type_and_score_tag_refuse_probability() {
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
    probability_refuses_for_score_and_tag(&engine, &score, &tag);
    assert_eq!(listener.count(), 0, "invalid construction sends nothing");
    let choose_expr = engine
        .choose_expr(
            &choose,
            col("body"),
            PolarsExprOptions::new().probability(true),
        )
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
            .f64()
            .expect("Float64")
            .get(0),
        Some(0.8)
    );
    assert_eq!(
        result
            .column("tagged")
            .expect("tags")
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
