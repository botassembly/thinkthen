//! Every applicable shared case through the public Rust API, on the conformance backend.
//!
//! Each success case runs on its own case arm. Expected request digests were
//! recorded against the canonical URL, so each is recomputed for the URL the
//! backend served. One case does not apply to the library:
//! `25-defect-fault` injects an internal invariant failure, which no outside
//! boundary reaches. The crate's own panic-door test covers the defect kind.

use std::collections::BTreeMap;
use std::fmt;

use conformance_backend::Backend;
use serde::Deserialize;
use serde::de::{MapAccess, Visitor};
use serde_json::value::RawValue;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use thinkthen::{
    Annotated, Choice, Description, Details, Engine, Entity, Error, FailureCause, Judgment, Kind,
    LoadedQuestion, Probabilities, Question, QuestionSet, Recognize, Recognized, Relate,
    RelationRule,
};

const CASES: &str = include_str!("../../../../cases.json");
const CANONICAL: &str = "https://api.typesafe.ai/v1/systemone";
const SKIPPED: [&str; 1] = ["25-defect-fault"];

pub(crate) type Checked<T = ()> = Result<T, String>;

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
    let backend = Backend::start().expect("the conformance backend");
    let mut failures = Vec::new();
    let mut ran = 0;
    for (case, verbatim) in cases.iter().zip(&written.cases) {
        let id = case["id"].as_str().expect("an id");
        if SKIPPED.contains(&id) {
            continue;
        }
        ran += 1;
        if let Err(why) = check(&backend, case, verbatim) {
            failures.push(format!("{id}: {why}"));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
    assert_eq!(ran, cases.len() - SKIPPED.len());
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
                &recognized(&found.map_err(said)?),
                &success["answers"][0]["bare"],
            )
        }
        ("relate", _) => related(&engine, &question, case, &success),
        ("annotate", _) => {
            let record = case.get("record").map(Value::to_string);
            let records = record.as_deref().map_or(texts, |whole| vec![whole]);
            annotated(&engine, &raw(&verbatim.question_set), &records, &success)
        }
        ("find", _) => found(&engine, &case["question"], &success),
        ("rank", _) => {
            let asked = Question::rank(case["question"]["decide"].as_str().unwrap_or_default());
            let ranked = engine
                .rank(&asked.map_err(said)?, texts.clone())
                .map_err(said)?;
            let rows = ranked.iter().map(
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
            same("decide", &json!(decision(answer)), bare)?;
            return detailed(&details, &success["answers"][0], base);
        }
    };
    match kind {
        Some("filter") => {
            let kept: Result<Vec<_>, _> = engine.filter(&asked, texts.to_vec()).collect();
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
    same("bare", &judgment(details.value()), &expected["bare"])?;
    detailed(&details, expected, base)?;
    let typed = match details.value() {
        Judgment::Decision(_) => json!(decision(engine.decide(asked, text).map_err(said)?)),
        Judgment::Score(_) => json!(engine.score(asked, text).map_err(said)?),
        Judgment::Choice(_) => {
            let typed = asked.clone().into_choose::<Team>().map_err(said)?;
            let pick = engine.choose(&typed, text).map_err(said)?;
            json!(pick.map(|team| team.label()))
        }
        Judgment::Tags(_) => match asked.clone().into_tag::<Mark>() {
            Ok(typed) => tags(engine.tag(&typed, text).map_err(said)?),
            Err(_) => {
                let typed = asked.clone().into_tag::<Word>().map_err(said)?;
                tags(engine.tag(&typed, text).map_err(said)?)
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

fn annotated(engine: &Engine, set: &str, texts: &[&str], success: &Value) -> Checked {
    let set = QuestionSet::from_json(set).map_err(said)?;
    let records: Result<Vec<_>, _> = engine.annotate(&set, texts.to_vec()).collect();
    let records = records.map_err(said)?;
    let mut failed = 0;
    for expected in success["answers"].as_array().into_iter().flatten() {
        let exchange = usize::try_from(expected["exchange"].as_u64().unwrap_or_default());
        let record = &records[exchange.unwrap_or_default().min(records.len().saturating_sub(1))];
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

/// A find over the case's units, with its none candidate when asked.
fn found(engine: &Engine, asked: &Value, success: &Value) -> Checked {
    let mut question = Question::find(asked["find"].as_str().unwrap_or_default()).map_err(said)?;
    if asked["none"] == json!(true) {
        question = question.offering_none().map_err(said)?;
    }
    let units: Vec<&str> = asked["units"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    let found = engine.find(&question, units.clone()).map_err(said)?;
    let rows = found.candidates().iter().map(|candidate| {
        let index = candidate.input().and_then(|unit| at(&units, unit));
        json!({"index": index, "probability": candidate.probability()})
    });
    let operation = &success["operation"];
    same("candidates", &rows.collect(), &operation["probabilities"])?;
    let selected = found.selected().and_then(|unit| at(&units, unit));
    same("selected", &json!(selected), &operation["selected"])
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
    let edges = edges.iter().map(|edge| {
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
    let entity = |one: &thinkthen::RecognizedEntity| json!({"name": one.name(), "kind": one.kind(), "start": one.start(), "end": one.end(), "strength": one.strength()});
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
fn at(texts: &[&str], text: &str) -> Option<usize> {
    texts
        .iter()
        .position(|one| std::ptr::eq(one.as_ptr(), text.as_ptr()))
}

fn digest(url: &str, request: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"systemone\n");
    hasher.update(url.as_bytes());
    hasher.update(b"\n");
    hasher.update(request);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn swap(value: &Value, renamed: &BTreeMap<String, String>) -> Value {
    match value {
        Value::String(held) => json!(renamed.get(held).unwrap_or(held)),
        Value::Array(items) => items.iter().map(|item| swap(item, renamed)).collect(),
        Value::Object(fields) => fields
            .iter()
            .map(|(name, field)| (name.clone(), swap(field, renamed)))
            .collect(),
        other => other.clone(),
    }
}

/// Compare two values, holding numbers to a rounding tolerance.
pub(crate) fn same(what: &str, actual: &Value, expected: &Value) -> Checked {
    fn close(one: &Value, other: &Value) -> bool {
        match (one, other) {
            (Value::Number(one), Value::Number(other)) => {
                (one.as_f64().unwrap_or(f64::NAN) - other.as_f64().unwrap_or(f64::NAN)).abs() < 1e-9
            }
            (Value::Array(one), Value::Array(other)) => {
                one.len() == other.len()
                    && one.iter().zip(other).all(|(one, other)| close(one, other))
            }
            (Value::Object(one), Value::Object(other)) => {
                one.len() == other.len()
                    && one
                        .iter()
                        .all(|(name, value)| other.get(name).is_some_and(|held| close(value, held)))
            }
            (one, other) => one == other,
        }
    }
    if close(actual, expected) {
        Ok(())
    } else {
        Err(format!("{what}: got {actual}, expected {expected}"))
    }
}
