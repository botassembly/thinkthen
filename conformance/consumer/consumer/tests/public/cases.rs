//! Every applicable shared case through the public Rust API, on the conformance backend.
//!
//! Each success case runs on its own case arm. Expected request digests were
//! recorded against the canonical URL, so each is recomputed for the URL the
//! backend served. One case does not apply to the library:
//! `25-defect-fault` injects an internal invariant failure, which no outside
//! boundary reaches. The crate's own panic-door test covers the defect kind.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::io::Write;
use std::num::NonZeroUsize;

use conformance_backend::Backend;
use serde::Deserialize;
use serde::de::{MapAccess, Visitor};
use serde_json::value::RawValue;
use serde_json::{Value, json};
use thinkthen::{
    Annotated, BatchSetting, CallOptions, Choice, Description, Details, Engine, Entity, Error,
    FailureCause, Judgment, Kind, LoadedQuestion, Probabilities, Question, QuestionSet, Recognize,
    Recognized, Relate, RelationRule,
};

const CASES: &str = include_str!("../../../../cases.json");
const CANONICAL: &str = "https://api.typesafe.ai/v1/systemone";
const SKIPPED: [&str; 1] = ["25-defect-fault"];

/// The saved filter/rank exchanges contain one request body per record.
fn singleton_requests<'a>() -> CallOptions<'a> {
    CallOptions::new().batch(BatchSetting::Records(NonZeroUsize::MIN))
}

pub(crate) type Checked<T = ()> = Result<T, String>;
pub(crate) use crate::values::same;
use crate::values::{digest, selected_ids, swap};

thinkthen::choices! { enum Team { Billing => "billing", Shipping => "shipping", Other => "other" } }
thinkthen::choices! { enum Mark { Billing => "billing", Urgent => "urgent", Security => "security" } }
thinkthen::choices! { enum Word { Refund => "the refund word", Maybe => "the maybe word" } }

#[derive(Deserialize)]
struct Written {
    cases: Vec<Verbatim>,
}

/// The question files as written, so their bytes and key order reach the parser.
#[derive(Deserialize)]
pub(crate) struct Verbatim {
    pub(crate) question: Option<Box<RawValue>>,
    question_set: Option<Box<RawValue>>,
}

#[derive(Deserialize)]
struct Asked {
    recognize: Option<Rules>,
    relate: Option<Rules>,
    threshold: Option<f64>,
    relation_threshold: Option<f64>,
}

#[derive(Deserialize)]
struct Rules {
    #[serde(default)]
    kinds: Ordered,
    #[serde(default)]
    relations: Vec<Rule>,
}

#[derive(Deserialize)]
struct Rule {
    name: String,
    source: String,
    target: String,
    #[serde(default)]
    either: bool,
}

/// A JSON object's members in written order.
#[derive(Default)]
struct Ordered(Vec<(String, String)>);

impl<'de> Deserialize<'de> for Ordered {
    fn deserialize<D: serde::Deserializer<'de>>(from: D) -> Result<Self, D::Error> {
        from.deserialize_map(Pairs)
    }
}

struct Pairs;

impl<'de> Visitor<'de> for Pairs {
    type Value = Ordered;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object of strings")
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Ordered, A::Error> {
        let mut pairs = Vec::new();
        while let Some(pair) = map.next_entry()? {
            pairs.push(pair);
        }
        Ok(Ordered(pairs))
    }
}

#[test]
fn every_applicable_shared_case_passes_through_the_public_api() {
    let document: Value = serde_json::from_str(CASES).expect("the shared cases");
    let written: Written = serde_json::from_str(CASES).expect("the cases as written");
    let cases = document["cases"].as_array().expect("a case list");
    let selected = selected_ids(cases).expect("a valid shared case selector");
    let selected_count = selected.as_ref().map_or(cases.len(), BTreeSet::len);
    let backend = Backend::start().expect("the conformance backend");
    let mut failures = Vec::new();
    let mut ran = 0;
    let mut not_run = 0;
    for (case, verbatim) in cases.iter().zip(&written.cases) {
        let id = case["id"].as_str().expect("an id");
        if selected.as_ref().is_some_and(|ids| !ids.contains(id)) {
            continue;
        }
        if SKIPPED.contains(&id) {
            not_run += 1;
            writeln!(
                std::io::stderr().lock(),
                "{id}: not run by the public API (internal invariant injection)"
            )
            .expect("write skipped case to stderr");
            continue;
        }
        ran += 1;
        if let Err(why) = check(&backend, case, verbatim) {
            failures.push(format!("{id}: {why}"));
        }
    }
    writeln!(
        std::io::stderr().lock(),
        "public Rust API: total={} selected={selected_count} pass={} fail={} not_run={not_run} unselected={}",
        cases.len(),
        ran - failures.len(),
        failures.len(),
        cases.len() - selected_count
    )
    .expect("write case counts to stderr");
    assert_eq!(ran + not_run, selected_count);
    assert!(failures.is_empty(), "{failures:#?}");
}

pub(crate) fn said(error: Error) -> String {
    format!("{:?}: {error}", error.kind())
}

pub(crate) fn engine(base: &str) -> Checked<Engine> {
    Engine::builder()
        .base_url(base)
        .and_then(|builder| builder.api_key("sk-consumer-loopback"))
        .map(thinkthen::EngineBuilder::no_cache)
        .and_then(thinkthen::EngineBuilder::build)
        .map_err(said)
}

fn check(backend: &Backend, case: &Value, verbatim: &Verbatim) -> Checked {
    let id = case["id"].as_str().unwrap_or_default();
    if let Some(kind) = case["expect"]["error"]["kind"].as_str() {
        return crate::paths::refused(backend, case, verbatim, kind);
    }
    let base = format!("{}/case/{id}/v1", backend.origin());
    let served = format!("{base}/systemone");
    let exchanges = case["exchanges"].as_array().cloned().unwrap_or_default();
    let renamed: BTreeMap<String, String> = exchanges
        .iter()
        .map(|exchange| {
            let request = exchange["request"].as_str().unwrap_or_default().as_bytes();
            (digest(CANONICAL, request), digest(&served, request))
        })
        .collect();
    let success = swap(&case["expect"]["success"], &renamed);
    let texts: Vec<&str> = exchanges
        .iter()
        .map(|exchange| exchange["evidence"].as_str().unwrap_or_default())
        .collect();
    let engine = engine(&base)?;
    let raw = |held: &Option<Box<RawValue>>| held.as_ref().map_or("", |raw| raw.get()).to_owned();
    let question = raw(&verbatim.question);
    match (
        case["verb"].as_str().unwrap_or_default(),
        success["kind"].as_str(),
    ) {
        ("recognize", _) => {
            let found = engine.recognize(
                &recognize(&question)?,
                case["text"].as_str().unwrap_or_default(),
            );
            same(
                "result",
                &recognized(&found.map_err(said)?.into_value()),
                &success["answers"][0]["bare"],
            )
        }
        ("relate", _) => related(&engine, &question, case, &success),
        ("annotate", _) => {
            let record = case.get("record").map(Value::to_string);
            let records = record.as_deref().map_or(texts, |whole| vec![whole]);
            let set = raw(&verbatim.question_set);
            annotated(&engine, &set, &records, &success, record.is_some())
        }
        ("find", _) => crate::parts::found(&engine, &case["question"], &success),
        ("rank", _) => {
            let asked = Question::rank(case["question"]["decide"].as_str().unwrap_or_default());
            let ranked = engine
                .rank_with(&asked.map_err(said)?, texts.clone(), singleton_requests())
                .map_err(said)?;
            let rows = ranked.value().iter().map(
                |row| json!({"index": at(&texts, row.input()), "probability": row.probability()}),
            );
            same("ranking", &rows.collect(), &success["operation"]["ranking"])
        }
        (_, kind) => loaded(&engine, &question, kind, &texts, &success, &base),
    }
}

/// A question file: one judgment, `filter`, or `decide_many`.
fn loaded(
    engine: &Engine,
    question: &str,
    kind: Option<&str>,
    texts: &[&str],
    success: &Value,
    base: &str,
) -> Checked {
    let asked = match Question::from_json(question).map_err(said)? {
        LoadedQuestion::Question(asked) => asked,
        LoadedQuestion::Banded(banded) => {
            let details = engine.details(&banded, texts[0]).map_err(said)?;
            let answer = engine.decide(&banded, texts[0]).map_err(said)?;
            let bare = &success["answers"][0]["bare"];
            same("decide", &json!(decision(answer.into_value())), bare)?;
            return detailed(details.value(), &success["answers"][0], base);
        }
    };
    match kind {
        Some("filter") => {
            let kept: Result<Vec<_>, _> = engine
                .filter_with(&asked, texts.to_vec(), singleton_requests())
                .collect();
            let kept = kept
                .map_err(said)?
                .iter()
                .map(|text| at(texts, text))
                .collect();
            same("indexes", &kept, &success["operation"]["indexes"])
        }
        Some("decide_many") => {
            let rows: Result<Vec<_>, _> = engine.decide_many(&asked, texts.to_vec()).collect();
            let rows = rows.map_err(said)?;
            let bare = rows.iter().map(|row| json!(decision(*row.value())));
            let expected = success["answers"].as_array().into_iter().flatten();
            same(
                "bare",
                &bare.collect(),
                &expected.map(|answer| answer["bare"].clone()).collect(),
            )
        }
        _ => single(engine, &asked, texts[0], success, base),
    }
}

/// One judgment by `details`, by its typed method, and, for a counters case, twice through a cache.
fn single(engine: &Engine, asked: &Question, text: &str, success: &Value, base: &str) -> Checked {
    let expected = &success["answers"][0];
    let details = engine.details(asked, text).map_err(said)?;
    same(
        "bare",
        &judgment(details.value().value()),
        &expected["bare"],
    )?;
    detailed(details.value(), expected, base)?;
    let typed = match details.value().value() {
        Judgment::Decision(_) => json!(decision(
            engine.decide(asked, text).map_err(said)?.into_value()
        )),
        Judgment::Score(_) => json!(engine.score(asked, text).map_err(said)?.into_value()),
        Judgment::Choice(_) => {
            let typed = asked.clone().into_choose::<Team>().map_err(said)?;
            let pick = engine.choose(&typed, text).map_err(said)?;
            json!(pick.into_value().map(|team| team.label()))
        }
        Judgment::Tags(_) => match asked.clone().into_tag::<Mark>() {
            Ok(typed) => tags(engine.tag(&typed, text).map_err(said)?.into_value()),
            Err(_) => {
                let typed = asked.clone().into_tag::<Word>().map_err(said)?;
                tags(engine.tag(&typed, text).map_err(said)?.into_value())
            }
        },
    };
    same("typed", &typed, &expected["bare"])?;
    let Some(counters) = success.get("counters") else {
        return Ok(());
    };
    let folder = std::env::temp_dir().join(format!("consumer-counters-{}", std::process::id()));
    let _absent = std::fs::remove_dir_all(&folder);
    let cached = Engine::builder()
        .base_url(base)
        .and_then(|builder| builder.api_key("sk-consumer-loopback"))
        .and_then(|builder| builder.cache_at(&folder))
        .and_then(thinkthen::EngineBuilder::build)
        .map_err(said)?;
    let before = cached.usage();
    for _ in 0..counters["calls"].as_u64().unwrap_or_default() {
        cached.details(asked, text).map_err(said)?;
    }
    let after = cached.usage();
    let moved = json!({
        "calls": counters["calls"],
        "requests": after.requests_sent() - before.requests_sent(),
        "cache_answers": after.cache_answers() - before.cache_answers(),
    });
    same("counters", &moved, counters)
}

fn detailed(details: &Details, expected: &Value, base: &str) -> Checked {
    let wanted = &expected["details"];
    let answer = &wanted["answer"];
    let probabilities = match details.probabilities() {
        Probabilities::YesNo { yes } => ("probability", json!(yes)),
        Probabilities::Named(named) => (
            "probabilities",
            named
                .iter()
                .map(|one| (one.name().to_owned(), json!(one.probability())))
                .collect(),
        ),
    };
    same(probabilities.0, &probabilities.1, &answer[probabilities.0])?;
    if let Some(level) = answer.get("level") {
        same("level", &json!(details.nearest()), level)?;
    }
    same("model", &json!(details.model()), &wanted["model"])?;
    same(
        "question_sha256",
        &json!(details.question_sha256()),
        &wanted["question_sha256"],
    )?;
    same("requests", &json!(details.requests()), &wanted["requests"])?;
    same(
        "confidence",
        &json!(details.confidence()),
        &answer["confidence"],
    )?;
    let usage = details.usage().map(|usage| {
        json!({"input_tokens": usage.input_tokens(), "output_tokens": usage.output_tokens()})
    });
    same("usage", &json!(usage), &wanted["usage"])?;
    same(
        "requests_sent",
        &json!(details.requests_sent()),
        &wanted["requests_sent"],
    )?;
    same("cached", &json!(details.cached()), &wanted["cached"])?;
    let served = json!(format!("{base}/systemone"));
    same("url", &json!(details.url()), &served)?;
    let line: Value =
        serde_json::from_str(&details.to_json()).map_err(|error| error.to_string())?;
    same("line url", &line["meta"]["url"], &served)
}

/// With `one`, the case names one record, and every answer reads it.
fn annotated(engine: &Engine, set: &str, texts: &[&str], success: &Value, one: bool) -> Checked {
    let set = QuestionSet::from_json(set).map_err(said)?;
    let records: Result<Vec<_>, _> = engine.annotate(&set, texts.to_vec()).collect();
    let records = records.map_err(said)?;
    let mut failed = 0;
    for expected in success["answers"].as_array().into_iter().flatten() {
        let exchange = usize::try_from(expected["exchange"].as_u64().unwrap_or_default());
        let record = &records[if one { 0 } else { exchange.unwrap_or_default() }];
        let named = record
            .values()
            .iter()
            .find(|value| value.name() == expected["name"]);
        let value = named.ok_or("a named value is missing")?.value();
        let bare = match value {
            Annotated::Decision(answer) => json!(decision(*answer)),
            Annotated::Choice(pick) => json!(pick),
            Annotated::Score(score) => json!(score),
            Annotated::Tags(tags) => json!(tags),
            Annotated::Failed(why) => {
                failed += 1;
                json!({"failed": {"kind": format!("{:?}", why.kind()).to_lowercase(), "cause": cause(why.cause())}})
            }
        };
        same("bare", &bare, &expected["bare"])?;
    }
    same(
        "failed",
        &json!(failed),
        success.get("failed_questions").unwrap_or(&json!(0)),
    )
}

fn related(engine: &Engine, question: &str, case: &Value, success: &Value) -> Checked {
    let asked: Asked = serde_json::from_str(question).map_err(|error| error.to_string())?;
    let mut builder = Relate::builder();
    for rule in asked.relate.ok_or("no relate block")?.relations {
        builder = builder.relation(rule_of(&rule)?).map_err(said)?;
    }
    if let Some(threshold) = asked.threshold {
        builder = builder.threshold(threshold).map_err(said)?;
    }
    let entities = case["entities"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|entity| {
            Entity::new(
                entity["name"].as_str().unwrap_or_default(),
                entity["kind"].as_str().unwrap_or_default(),
            )
        });
    let entities: Result<Vec<_>, _> = entities.collect();
    let edges = engine
        .relate(&builder.build().map_err(said)?, entities.map_err(said)?)
        .map_err(said)?;
    let pair = |entity: &Entity| json!({"name": entity.name(), "kind": entity.kind()});
    let edges = edges.value().iter().map(|edge| {
        json!({"relation": edge.relation(), "source": pair(edge.source()), "target": pair(edge.target()), "probability": edge.probability()})
    });
    same("result", &edges.collect(), &success["answers"][0]["bare"])
}

fn recognize(question: &str) -> Checked<Recognize> {
    let asked: Asked = serde_json::from_str(question).map_err(|error| error.to_string())?;
    let rules = asked.recognize.ok_or("no recognize block")?;
    let mut builder = Recognize::builder();
    for (name, meaning) in rules.kinds.0 {
        let described = Description::text(&meaning).map_err(said)?;
        builder = builder
            .kind(Kind::new(&name, Some(described)).map_err(said)?)
            .map_err(said)?;
    }
    for rule in &rules.relations {
        builder = builder.relation(rule_of(rule)?).map_err(said)?;
    }
    if let Some(threshold) = asked.threshold {
        builder = builder.threshold(threshold).map_err(said)?;
    }
    if let Some(threshold) = asked.relation_threshold {
        builder = builder.relation_threshold(threshold).map_err(said)?;
    }
    builder.build().map_err(said)
}

fn rule_of(rule: &Rule) -> Checked<RelationRule> {
    let made = if rule.either {
        RelationRule::both_ways
    } else {
        RelationRule::one_way
    };
    made(&rule.name, &rule.source, &rule.target).map_err(said)
}

fn recognized(found: &Recognized) -> Value {
    let entity = |one: &thinkthen::RecognizedEntity| json!({"text": one.text(), "start": one.start(), "end": one.end(), "length": one.length(), "kind": one.kind(), "strength": one.strength()});
    let mut value = json!({"entities": found.entities().iter().map(entity).collect::<Vec<_>>()});
    if let Some(relations) = found.relations() {
        value["relations"] = relations
            .iter()
            .map(|one| json!({"relation": one.relation(), "source": entity(one.source()), "target": entity(one.target()), "probability": one.probability()}))
            .collect();
    }
    value
}

fn decision(answer: thinkthen::Answer) -> Option<bool> {
    match answer {
        thinkthen::Answer::Yes => Some(true),
        thinkthen::Answer::No => Some(false),
        thinkthen::Answer::Unsure => None,
    }
}

fn judgment(value: &Judgment) -> Value {
    match value {
        Judgment::Decision(answer) => json!(decision(*answer)),
        Judgment::Choice(pick) => json!(pick),
        Judgment::Score(score) => json!(score),
        Judgment::Tags(tags) => json!(tags),
    }
}

/// Typed tags as the shared cases spell them.
fn tags<C: Choice>(picked: Vec<C>) -> Value {
    json!(picked.iter().map(Choice::label).collect::<Vec<_>>())
}

/// A failure cause as the shared cases spell it.
fn cause(cause: FailureCause) -> String {
    let named = format!("{cause:?}");
    let mut snake = String::new();
    for letter in named.chars() {
        if letter.is_uppercase() && !snake.is_empty() {
            snake.push('_');
        }
        snake.push(letter.to_ascii_lowercase());
    }
    snake
}

/// The input index a returned borrowed text came from.
pub(crate) fn at(texts: &[&str], text: &str) -> Option<usize> {
    texts
        .iter()
        .position(|one| std::ptr::eq(one.as_ptr(), text.as_ptr()))
}
