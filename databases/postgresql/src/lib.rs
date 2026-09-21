//! The PostgreSQL surface: twelve SQL functions over the one engine.
//!
//! Rule 1 of the shared database page holds: no rule lives here. This file
//! resolves SQL values into the contract's questions, calls one engine
//! method, maps the answer back, and raises the engine's failures as
//! PostgreSQL errors with the kind and the retry signal in the message. A
//! failure never reads as a `No`, and `NULL` is "not sure", SQL's own
//! three-valued habit.
//!
//! The engine is lazy per backend: the stand-in builds its state on first
//! use in each process, so a forked backend rebuilds rather than inherits
//! a live pool, and `_PG_init` registers the key setting and touches
//! nothing else.
//!
//! A batch runs on one worker thread while the backend thread watches
//! PostgreSQL's interrupt flag: a pending interrupt first cancels the
//! engine's token — no new request starts, and the sent ones finish — and
//! the check then raises the database's own error. Experiment 211 measured
//! this shape; no proof is re-run here.

use std::collections::HashMap;
use std::ffi::CString;
use std::ffi::c_int;
use std::path::Path;
use std::sync::OnceLock;
use std::time::Duration;

use pgrx::datum::{Array, JsonB};
use pgrx::pg_sys::FunctionCallInfo;
use pgrx::prelude::*;
use pgrx::{Aggregate, AggregateName, GucContext, GucFlags, GucRegistry, GucSetting, Spi};
use thinkthen_contract::{
    Annotated, Engine as _, Entity, Error, ErrorKind, Kind, Question, QuestionKind, QuestionSet,
    Recognize, Relate, relate_checked,
};
use thinkthen_standin::BlockingEngine;

pgrx::pg_module_magic!();

/// The API key setting: only a superuser sets it, and no view shows it.
/// The engine itself reads the environment at send time; wiring the setting
/// into the send is the database ADR's preload question, not this lane's.
static API_KEY: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);

/// The engine, lazy in each backend. Built on first use, after the fork.
static ENGINE: OnceLock<BlockingEngine> = OnceLock::new();

fn engine() -> &'static BlockingEngine {
    ENGINE.get_or_init(BlockingEngine::from_env)
}

// PostgreSQL's interrupt flag, read only. The C layer raises the error;
// this read only tells the poll loop to cancel the engine's token first.
unsafe extern "C" {
    #[link_name = "InterruptPending"]
    static INTERRUPT_PENDING: c_int;
}

fn interrupt_pending() -> bool {
    unsafe { std::ptr::read_volatile(std::ptr::addr_of!(INTERRUPT_PENDING)) != 0 }
}

/// Run one engine batch on a worker thread while the backend thread polls
/// the interrupt flag. A pending interrupt cancels the token, the batch
/// stops between requests, and the proper error then raises through the
/// check.
fn run_batch<T, F>(work: F) -> Result<T, Error>
where
    F: FnOnce(Option<&thinkthen_contract::Cancel>) -> Result<T, Error> + Send + 'static,
    T: Send + 'static,
{
    let token = thinkthen_contract::Cancel::new();
    let worker = token.clone();
    let handle = std::thread::spawn(move || work(Some(&worker)));
    while !handle.is_finished() {
        if interrupt_pending() {
            token.cancel();
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    let out = handle
        .join()
        .unwrap_or_else(|_| Err(Error::defect("the batch thread stopped")));
    check_for_interrupts!();
    out
}

/// Raise the engine's failure as the database's own error, with the kind,
/// the retry signal, and a SQLSTATE that matches the kind. A failure never
/// reads as a NULL.
fn raise(error: Error) -> ! {
    let code = match error.kind {
        ErrorKind::Usage => PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE,
        ErrorKind::Backend => PgSqlErrorCode::ERRCODE_EXTERNAL_ROUTINE_EXCEPTION,
        ErrorKind::Deadline | ErrorKind::Cancelled => PgSqlErrorCode::ERRCODE_QUERY_CANCELED,
        ErrorKind::Local => PgSqlErrorCode::ERRCODE_IO_ERROR,
        ErrorKind::Defect => PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
    };
    let retry = if error.retryable { "yes" } else { "no" };
    let text = format!("thinkthen {}: {error} (retryable: {retry})", error.kind);
    ereport!(ERROR, code, text.as_str());
}

/// Resolve the question argument: `'@name'` names a file (the command's
/// spelling), JSON text carries the grammar, and a bare sentence is a
/// decide question with the default cut.
fn question_of(arg: Option<&str>) -> Question {
    let text = arg.unwrap_or_default();
    if text.trim().is_empty() {
        raise(Error::usage("the question is empty"));
    }
    match text.strip_prefix('@') {
        Some(path) => Question::from_file(Path::new(path)).unwrap_or_else(|error| raise(error)),
        None => Question::from_json(text).unwrap_or_else(|error| raise(error)),
    }
}

/// Resolve the question-set argument of `annotate`: `'@name'` or a bare
/// path names a set file, and JSON text is the set itself.
fn set_of(arg: Option<&str>) -> QuestionSet {
    let text = arg.unwrap_or_default();
    if text.trim().is_empty() {
        raise(Error::usage("the question set is empty"));
    }
    match text.strip_prefix('@') {
        Some(path) => QuestionSet::from_file(Path::new(path)).unwrap_or_else(|error| raise(error)),
        None if text.trim_start().starts_with('{') => {
            QuestionSet::from_json(text).unwrap_or_else(|error| raise(error))
        }
        None => QuestionSet::from_file(Path::new(text)).unwrap_or_else(|error| raise(error)),
    }
}

/// The members the function names — options, levels, labels — rebuilt onto
/// the question the caller gave. A question whose JSON already carries its
/// members passes straight through.
fn with_members(question: &Question, members: Option<Array<'_, &str>>) -> Question {
    let Some(members) = members else { return question.clone() };
    let members: Vec<&str> = members.iter().flatten().collect();
    if members.is_empty() {
        raise(Error::usage("the members array is empty"));
    }
    let built = match question.kind() {
        QuestionKind::Choose | QuestionKind::Decide => Question::choose(question.text(), &members)
            .and_then(|built| built.build()),
        QuestionKind::Score => Question::score(question.text(), &members),
        QuestionKind::Tag => Question::tag(question.text(), &members),
    };
    built.unwrap_or_else(|error| raise(error))
}

/// The distinct texts of an iterator, in first-seen order.
fn distinct_of<'a>(texts: impl Iterator<Item = Option<&'a str>>) -> Vec<String> {
    let mut seen: HashMap<&str, ()> = HashMap::new();
    let mut out = Vec::new();
    for text in texts.flatten() {
        if seen.insert(text, ()).is_none() {
            out.push(text.to_owned());
        }
    }
    out
}

/// Read a spec file named by the caller with the command's `@name`
/// spelling, or a bare path; a failure is the local kind.
fn read_spec(path: &str) -> String {
    std::fs::read_to_string(Path::new(path)).unwrap_or_else(|error| {
        raise(Error::local(format!("the spec file {path} did not read: {error}")))
    })
}

/// Resolve the `recognize` spec argument: `'@name'` or a bare path names a
/// file (the question file's `recognize` section is taken when present),
/// and JSON text carries the spec itself.
///
/// The ruled question file spells a relation's ends `source` and `target`
/// (`sdlc/planning/recognize-design.md`, the ruling of 2026-09-21); the
/// contract's spec parser reads `from` and `to` today, so this door accepts
/// both spellings and normalizes to the parser's pair before the one core
/// parser runs. Recorded for the contract's owner.
fn recognizer_of(arg: Option<&str>) -> Recognize {
    let text = arg.unwrap_or_default();
    if text.trim().is_empty() {
        raise(Error::usage("the recognize spec is empty"));
    }
    let json = match text.strip_prefix('@') {
        Some(path) => read_spec(path),
        None if text.trim_start().starts_with('{') => text.to_owned(),
        None => read_spec(text),
    };
    let mut value: serde_json::Value = serde_json::from_str(&json).unwrap_or_else(|error| {
        raise(Error::usage(format!("the recognize spec is not JSON: {error}")))
    });
    if let Some(section) = value.get_mut("recognize").map(std::mem::take) {
        // The question file keeps both thresholds at the top level, beside
        // the section; the spec parser reads them inside it.
        let held: Vec<(&str, serde_json::Value)> = ["threshold", "relation_threshold"]
            .into_iter()
            .filter_map(|key| value.get(key).cloned().map(|number| (key, number)))
            .collect();
        let mut section = section;
        if let Some(object) = section.as_object_mut() {
            for (key, number) in held {
                object.entry(key.to_owned()).or_insert(number);
            }
        }
        value = section;
    }
    if let Some(relations) = value.get_mut("relations").and_then(serde_json::Value::as_array_mut) {
        for rule in relations {
            if let Some(object) = rule.as_object_mut() {
                for (ruled, parser) in [("source", "from"), ("target", "to")] {
                    if let Some(held) = object.remove(ruled) {
                        object.entry(parser.to_owned()).or_insert(held);
                    }
                }
            }
        }
    }
    Recognize::from_json(&value.to_string()).unwrap_or_else(|error| raise(error))
}

/// One annotate answer as JSON: a decision is true, false, or null; a
/// choice is the label or null; a score is its number; tags are a list.
fn annotated_as_json(held: &Annotated) -> serde_json::Value {
    match held {
        Annotated::Decision(answer) => serde_json::json!(answer.value()),
        Annotated::Choice(Some(won)) => serde_json::json!(won),
        Annotated::Choice(None) => serde_json::Value::Null,
        Annotated::Score(scored) => serde_json::json!(scored.value),
        Annotated::Tags(labels) => serde_json::json!(labels),
    }
}

#[pg_extern(parallel_restricted)]
fn thinkthen_decide(question: Option<&str>, evidence: Option<&str>) -> Option<bool> {
    let question = question_of(question);
    let Some(evidence) = evidence else { return None };
    match engine().decide_with(&question, evidence, None) {
        Ok(answer) => answer.value(),
        Err(error) => raise(error),
    }
}

#[pg_extern(parallel_restricted)]
fn thinkthen_probability(question: Option<&str>, evidence: Option<&str>) -> Option<f64> {
    let question = question_of(question);
    let Some(evidence) = evidence else { return None };
    let options = thinkthen_contract::Options::new();
    match engine().decide_many_opts(&question, &[evidence], options, None) {
        Ok(mut judged) => judged.pop().map(|judgment| judgment.probability),
        Err(error) => raise(error),
    }
}

#[pg_extern(parallel_restricted)]
fn thinkthen_choose(
    question: Option<&str>,
    evidence: Option<&str>,
    options: Option<Array<'_, &str>>,
) -> Option<String> {
    let question = with_members(&question_of(question), options);
    let Some(evidence) = evidence else { return None };
    match engine().choose(&question, evidence) {
        Ok(choice) => choice,
        Err(error) => raise(error),
    }
}

#[pg_extern(parallel_restricted)]
fn thinkthen_score(
    question: Option<&str>,
    evidence: Option<&str>,
    levels: Option<Array<'_, &str>>,
) -> Option<f64> {
    let question = with_members(&question_of(question), levels);
    let Some(evidence) = evidence else { return None };
    match engine().score(&question, evidence) {
        Ok(scored) => Some(scored.value),
        Err(error) => raise(error),
    }
}

#[pg_extern(parallel_restricted)]
fn thinkthen_tag(
    question: Option<&str>,
    evidence: Option<&str>,
    labels: Option<Array<'_, &str>>,
) -> Option<Vec<String>> {
    let question = with_members(&question_of(question), labels);
    let Some(evidence) = evidence else { return None };
    match engine().tag(&question, evidence) {
        Ok(held) => Some(held),
        Err(error) => raise(error),
    }
}

#[pg_extern(parallel_restricted)]
fn thinkthen_annotate(set: Option<&str>, evidence: Option<&str>) -> Option<JsonB> {
    let set = set_of(set);
    let Some(evidence) = evidence else { return None };
    let options = thinkthen_contract::Options::new();
    match engine().annotate_opts(&set, &[evidence], options, None) {
        Ok(mut answers) => {
            let mut object = serde_json::Map::new();
            let Some(fields) = answers.pop() else { return None };
            for (name, held) in fields {
                object.insert(name, annotated_as_json(&held));
            }
            Some(JsonB(serde_json::Value::Object(object)))
        }
        Err(error) => raise(error),
    }
}

#[pg_extern(parallel_restricted)]
fn thinkthen_details(question: Option<&str>, evidence: Option<&str>) -> Option<JsonB> {
    let question = question_of(question);
    let Some(evidence) = evidence else { return None };
    match engine().details(&question, evidence) {
        Ok(details) => Some(JsonB(serde_json::to_value(details).expect("details serializes"))),
        Err(error) => raise(error),
    }
}

/// The session counters the engine keeps: requests count sends, retried
/// sends included, so a bill showing two requests never meets a tool
/// showing one.
#[pg_extern(parallel_restricted)]
fn thinkthen_usage(
) -> TableIterator<'static, (name!(requests, i64), name!(cache_answers, i64), name!(tokens, i64))> {
    let usage = engine().usage();
    let row = (
        i64::try_from(usage.requests).unwrap_or(i64::MAX),
        i64::try_from(usage.cache_answers).unwrap_or(i64::MAX),
        i64::try_from(usage.tokens).unwrap_or(i64::MAX),
    );
    TableIterator::new(vec![row].into_iter())
}

/// `recognize(text, kinds)`: every name in the text as a row, the five
/// ruled columns — `text`, `kind`, `start`, `end`, `strength`. `start`
/// and `end` count characters, PostgreSQL's own string indexing, so
/// `substring(text from start + 1 for end - start)` is the name. With no
/// kinds the three defaults apply. `PARALLEL RESTRICTED`; used with
/// `LATERAL`.
#[pg_extern(parallel_restricted)]
fn thinkthen_recognize(
    body: Option<&str>,
    kinds: Option<Array<'_, &str>>,
) -> TableIterator<
    'static,
    (
        name!(text, Option<String>),
        name!(kind, Option<String>),
        name!(start, Option<i32>),
        name!(end, Option<i32>),
        name!(strength, Option<f64>),
    ),
> {
    let Some(body) = body else {
        return TableIterator::new(std::iter::empty());
    };
    let mut ask = Recognize::new();
    if let Some(kinds) = kinds {
        let names: Vec<&str> = kinds
            .iter()
            .flatten()
            .map(str::trim)
            .filter(|kind| !kind.is_empty())
            .collect();
        if names.len() > 20 {
            raise(Error::usage(format!(
                "the recognize call carries {} kinds, and 20 is the limit",
                names.len()
            )));
        }
        if !names.is_empty() {
            ask = ask.kinds(names);
        }
    }
    let found = engine().recognize(&ask, body).unwrap_or_else(|error| raise(error));
    let rows: Vec<_> = found
        .entities
        .into_iter()
        .map(|entity| {
            (
                Some(entity.text),
                Some(entity.kind),
                Some(i32::try_from(entity.start).unwrap_or(i32::MAX)),
                Some(i32::try_from(entity.end).unwrap_or(i32::MAX)),
                Some(entity.strength),
            )
        })
        .collect();
    TableIterator::new(rows.into_iter())
}

/// `relate(query, rules)`: every legal pair in the query's records judged
/// at once, as rows `(name, source, target, probability)`. The query
/// selects two columns — the record's id first, its text second — and
/// `source` and `target` carry those id values, so the edges join back to
/// the query's table. More than 255 records is a usage error. Rules are
/// bare relation names (any kind to any kind); richer rules ride the
/// question file. `PARALLEL RESTRICTED`.
#[pg_extern(parallel_restricted)]
fn thinkthen_relate(
    query: Option<&str>,
    rules: Option<Array<'_, &str>>,
) -> TableIterator<
    'static,
    (
        name!(name, Option<String>),
        name!(source, Option<i64>),
        name!(target, Option<i64>),
        name!(probability, Option<f64>),
    ),
> {
    let query = query.unwrap_or_default().trim().trim_end_matches(';').to_owned();
    if query.is_empty() {
        raise(Error::usage("the relate query is empty"));
    }
    let mut ask = Relate::new();
    let mut any_rule = false;
    if let Some(rules) = rules {
        for rule in rules.iter().flatten() {
            let rule = rule.trim();
            if rule.is_empty() {
                raise(Error::usage("a relate rule is empty"));
            }
            ask = ask.relation(rule, Kind::Any, Kind::Any).unwrap_or_else(|error| raise(error));
            any_rule = true;
        }
    }
    if !any_rule {
        raise(Error::usage("relate needs at least one relation rule"));
    }
    let rows = Spi::connect(|client| -> Result<Vec<(i64, String)>, Error> {
        let wrapped = format!(
            "SELECT ask.id::bigint AS id, ask.body::text AS body FROM ({query}) AS ask(id, body)"
        );
        let table = client.select(wrapped.as_str(), Some(256), &[]).map_err(|error| {
            Error::usage(format!("the relate query did not run: {error}"))
        })?;
        let mut out = Vec::with_capacity(table.len());
        for row in table {
            let id = row
                .get::<i64>(1)
                .map_err(|error| {
                    Error::usage(format!("the relate query's id column did not read: {error}"))
                })?
                .ok_or_else(|| Error::usage("the relate query returned a null id"))?;
            let body = row
                .get::<String>(2)
                .map_err(|error| {
                    Error::usage(format!("the relate query's text column did not read: {error}"))
                })?
                .ok_or_else(|| Error::usage("the relate query returned a null text"))?;
            out.push((id, body));
        }
        Ok(out)
    });
    let rows = rows.unwrap_or_else(|error| raise(error));
    let owned: Vec<String> = rows.iter().map(|(_, body)| body.clone()).collect();
    let edges = run_batch(move |cancel| {
        let texts: Vec<&str> = owned.iter().map(String::as_str).collect();
        let options = thinkthen_contract::Options::new().maybe_cancel(cancel);
        relate_checked(engine(), &ask, &texts, options)
    })
    .unwrap_or_else(|error| raise(error));
    let out: Vec<_> = edges
        .into_iter()
        .map(|edge| {
            let id_of = |number: u64| -> Option<i64> {
                rows.get(number.saturating_sub(1) as usize).map(|row| row.0)
            };
            (
                Some(edge.name),
                id_of(edge.source),
                id_of(edge.target),
                Some(edge.probability),
            )
        })
        .collect();
    TableIterator::new(out.into_iter())
}

/// The beta companion: `thinkthen_relations(body, '@names.json')` runs the
/// spec file's rules over one text and returns the relations as rows
/// `(name, source_text, source_kind, target_text, target_kind,
/// probability)`. Relations need the question file. `PARALLEL RESTRICTED`.
#[pg_extern(parallel_restricted)]
fn thinkthen_relations(
    body: Option<&str>,
    spec: Option<&str>,
) -> TableIterator<
    'static,
    (
        name!(name, Option<String>),
        name!(source_text, Option<String>),
        name!(source_kind, Option<String>),
        name!(target_text, Option<String>),
        name!(target_kind, Option<String>),
        name!(probability, Option<f64>),
    ),
> {
    let Some(body) = body else {
        return TableIterator::new(std::iter::empty());
    };
    let ask = recognizer_of(spec);
    let found = engine().recognize(&ask, body).unwrap_or_else(|error| raise(error));
    let by_id: HashMap<u64, Entity> =
        found.entities.into_iter().map(|entity| (entity.id, entity)).collect();
    let rows: Vec<_> = found
        .relations
        .into_iter()
        .filter_map(|relation| {
            let source = by_id.get(&relation.source)?;
            let target = by_id.get(&relation.target)?;
            Some((
                Some(relation.name),
                Some(source.text.clone()),
                Some(source.kind.clone()),
                Some(target.text.clone()),
                Some(target.kind.clone()),
                Some(relation.probability),
            ))
        })
        .collect();
    TableIterator::new(rows.into_iter())
}

/// The array overload, the second bulk form `postgres.md` names: one array
/// in, one row per element out, position kept from 0, one crossing.
#[pg_extern(name = "thinkthen_decide", parallel_restricted)]
fn thinkthen_decide_array(
    question: Option<&str>,
    evidences: Option<Array<'_, &str>>,
) -> TableIterator<'static, (name!(i, Option<i32>), name!(decided, Option<bool>))> {
    let question_text = question.unwrap_or_default().to_owned();
    let rows: Vec<Option<String>> = evidences
        .iter()
        .flat_map(|array| array.iter().map(|text| text.map(str::to_owned)))
        .collect();
    let distinct = distinct_of(rows.iter().map(|text| text.as_deref()));
    let owned = distinct.clone();
    let judged = run_batch(move |cancel| {
        let question = question_of(Some(&question_text));
        let records: Vec<&str> = owned.iter().map(String::as_str).collect();
        let options = thinkthen_contract::Options::new().maybe_cancel(cancel);
        engine().decide_many_opts(&question, &records, options, None)
    });
    let by_text: HashMap<String, Option<bool>> = match judged {
        Ok(judged) => distinct
            .iter()
            .cloned()
            .zip(judged)
            .map(|(text, judgment)| (text, judgment.value()))
            .collect(),
        Err(error) => raise(error),
    };
    let out = rows
        .iter()
        .enumerate()
        .map(|(place, text)| {
            (
                Some(i32::try_from(place).unwrap_or(i32::MAX)),
                text.as_ref().and_then(|text| by_text.get(text).copied().flatten()),
            )
        })
        .collect::<Vec<_>>();
    TableIterator::new(out.into_iter())
}

/// The warm aggregate's name carrier. The state travels as JSON text
/// because pgrx 0.17 offers no internal state type.
#[derive(AggregateName)]
#[aggregate_name = "thinkthen_warm"]
struct Warm;

/// The warm state as data: pairs of question and evidence, arrival order.
#[derive(Default, serde::Serialize, serde::Deserialize)]
struct WarmRows {
    rows: Vec<(String, String)>,
}

#[pg_aggregate]
impl Aggregate<Warm> for Warm {
    type State = String;
    type Args = (Option<String>, Option<String>);
    type Finalize = i64;
    const INITIAL_CONDITION: Option<&'static str> = Some(r#"{"rows":[]}"#);

    fn state(current: String, args: Self::Args, _fcinfo: FunctionCallInfo) -> String {
        let mut rows: WarmRows = serde_json::from_str(&current).unwrap_or_default();
        if let (Some(question), Some(evidence)) = args {
            rows.rows.push((question, evidence));
        }
        serde_json::to_string(&rows).expect("the warm state serializes")
    }

    fn combine(one: String, two: String, _fcinfo: FunctionCallInfo) -> String {
        let mut one: WarmRows = serde_json::from_str(&one).unwrap_or_default();
        let two: WarmRows = serde_json::from_str(&two).unwrap_or_default();
        one.rows.extend(two.rows);
        serde_json::to_string(&one).expect("the warm state serializes")
    }

    fn finalize(current: String, _direct: Self::OrderedSetArgs, _fcinfo: FunctionCallInfo) -> i64 {
        let rows: WarmRows = serde_json::from_str(&current).unwrap_or_default();
        let mut by_question: HashMap<String, Vec<String>> = HashMap::new();
        for (question, evidence) in rows.rows {
            by_question.entry(question).or_default().push(evidence);
        }
        let mut judged: i64 = 0;
        for (question_text, evidences) in by_question {
            let distinct = distinct_of(evidences.iter().map(|text| Some(text.as_str())));
            let outcome = run_batch(move |cancel| {
                let question = question_of(Some(&question_text));
                let records: Vec<&str> = distinct.iter().map(String::as_str).collect();
                let options = thinkthen_contract::Options::new().maybe_cancel(cancel);
                engine().decide_many_opts(&question, &records, options, None)
            });
            match outcome {
                Ok(held) => judged += i64::try_from(held.len()).unwrap_or(i64::MAX),
                Err(error) => raise(error),
            }
        }
        judged
    }
}

/// Register the key setting. Nothing else happens here: the engine builds
/// lazily in each backend, after the fork, and `_PG_init` never touches
/// the wire.
#[pg_guard]
extern "C-unwind" fn _PG_init() {
    GucRegistry::define_string_guc(
        c"thinkthen.api_key",
        c"the API key for the ThinkThen engine",
        c"",
        &API_KEY,
        GucContext::Suset,
        GucFlags::NO_SHOW_ALL | GucFlags::SUPERUSER_ONLY | GucFlags::DISALLOW_IN_AUTO_FILE,
    );
}
