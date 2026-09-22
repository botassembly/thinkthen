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
//!
//! The single-row calls cannot be cancelled that way: the backend thread
//! waits on the wire inside the call, so `pg_cancel_backend` and
//! `statement_timeout` only take effect after the send returns. The
//! enforced tool there is `thinkthen.deadline_ms` (milliseconds; `-1` is
//! none, `0` is a spent deadline, a positive value is the budget), carried
//! by every single-request function; a spent or expired budget returns the
//! deadline kind with nothing sent or with the sent request abandoned.
//!
//! `CREATE EXTENSION` revokes the default PUBLIC grant on every function,
//! so a paid call and an `@path` file read need a role an administrator
//! has granted EXECUTE to; superusers keep access by their own right.

use std::collections::HashMap;
use std::ffi::CString;
use std::ffi::c_int;
use std::path::Path;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use pgrx::datum::{Array, JsonB};
use pgrx::pg_sys::FunctionCallInfo;
use pgrx::prelude::*;
use pgrx::{Aggregate, AggregateName, GucContext, GucFlags, GucRegistry, GucSetting, Spi};
use thinkthen_contract::{
    Annotated, Connector as _, Engine, EngineConfig, Entity, Error, ErrorKind, Kind, Options,
    Question, QuestionKind, QuestionSet, Recognize, Relate, deadline_from_millis,
    relate_checked,
};
use thinkthen_standin::StandinConnector;

pgrx::pg_module_magic!();

/// The API key setting: only a superuser sets it, and no view shows it.
/// The engine itself reads the environment at send time; wiring the setting
/// into the send is the database ADR's preload question, not this lane's.
static API_KEY: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);

/// Why a set `thinkthen.api_key` refuses on this build, or `None` when the
/// setting is absent. Blank reads as absent, the rule the command uses for
/// its key variable. Pure, so the rule is unit-tested without a backend.
///
/// This engine build takes no key from the host: the stand-in reads no key
/// at all, and which channel delivers the setting to the send is the
/// database ADR's open question 4. A configured value therefore refuses
/// loudly, naming the setting and the substitution, instead of being
/// silently ignored. The message never carries a value.
fn unwired_key_refusal(setting: Option<&str>) -> Option<Error> {
    let set = setting.is_some_and(|key| !key.trim().is_empty());
    set.then(|| {
        Error::usage(
            "the thinkthen.api_key setting cannot reach this engine build; unset it and \
             let the engine read THINKTHEN_API_KEY from the server's environment",
        )
    })
}

/// The per-call deadline, in milliseconds: `-1` is no deadline, `0` is a
/// spent deadline, and a positive value is the budget. `Userset`, so any
/// role can bound its own paid call; a superuser can set a database-wide
/// default with `ALTER DATABASE ... SET thinkthen.deadline_ms`.
///
/// This is the enforced tool on the single-row path: the backend thread
/// waits on a blocking send there, so `pg_cancel_backend` and
/// `statement_timeout` cannot reach the wire call — the budget is what
/// bounds it. The batch forms carry the host's cancel instead.
static DEADLINE_MS: GucSetting<i32> = GucSetting::<i32>::new(-1);

/// The engine, lazy in each backend. Built on first use, after the fork.
static ENGINE: OnceLock<Arc<dyn Engine>> = OnceLock::new();

/// The engine value, with the configured key's refusal in front of it.
///
/// Called on the backend's own thread only: the setting read behind it
/// (`GucSetting::get`) checks the active thread and panics elsewhere. The
/// batch closures take the reference before `run_batch` spawns, so no
/// worker thread reads a setting.
fn engine() -> &'static Arc<dyn Engine> {
    if let Some(key) = API_KEY.get() {
        if let Some(error) = unwired_key_refusal(Some(&key.to_string_lossy())) {
            raise(error);
        }
    }
    ENGINE.get_or_init(|| {
        StandinConnector
            .connect(&EngineConfig::from_env())
            .expect("the stand-in connector has no failure path")
    })
}

/// The options every single-request call carries: the deadline setting,
/// converted by the contract's one checked door, so a NaN-like or
/// oversized value refuses instead of panicking inside the backend.
fn call_options() -> Options<'static> {
    let millis = DEADLINE_MS.get();
    let given = if millis == -1 { None } else { Some(f64::from(millis)) };
    Options::new().with_deadline_millis(given).unwrap_or_else(|error| raise(error))
}

// PostgreSQL's interrupt flags, read only. The C layer raises the error;
// these reads only tell the poll loop which interrupts are a cancel.
unsafe extern "C" {
    #[link_name = "QueryCancelPending"]
    static QUERY_CANCEL_PENDING: c_int;
    #[link_name = "ProcDiePending"]
    static PROC_DIE_PENDING: c_int;
}

/// Whether the backend has a real cancel pending: SIGINT (a
/// `pg_cancel_backend` or a statement timeout) sets `QueryCancelPending`,
/// and SIGTERM (`pg_terminate_backend`) sets `ProcDiePending`. Any other
/// interrupt — a procsignal barrier, a memory-contexts request, a notify
/// — sets `InterruptPending` alone and is serviced by
/// `check_for_interrupts!` without ending anything.
fn cancel_requested() -> bool {
    let query = unsafe { std::ptr::read_volatile(std::ptr::addr_of!(QUERY_CANCEL_PENDING)) };
    let die = unsafe { std::ptr::read_volatile(std::ptr::addr_of!(PROC_DIE_PENDING)) };
    query != 0 || die != 0
}

/// The configured per-call deadline as the budget a batch carries: `-1`
/// is no deadline, `0` a spent one, a positive value the budget. Pure, so
/// the rule is unit-tested without a backend.
fn budget_of(millis: i32) -> Result<Option<Duration>, Error> {
    deadline_from_millis(f64::from(millis))
}

/// The deadline the setting names, resolved on the backend thread (the
/// setting read panics off it). A refused value raises the usage kind
/// before any worker spawns.
fn batch_budget() -> Option<Duration> {
    budget_of(DEADLINE_MS.get()).unwrap_or_else(|error| raise(error))
}

/// The options a batch carries: the host's cancel token and the resolved
/// deadline budget.
fn batch_options<'a>(
    cancel: Option<&'a thinkthen_contract::Cancel>,
    budget: Option<Duration>,
) -> thinkthen_contract::Options<'a> {
    let options = thinkthen_contract::Options::new().maybe_cancel(cancel);
    match budget {
        Some(budget) => options.deadline_in(budget),
        None => options,
    }
}

/// Run one engine batch on a worker thread while the backend thread polls
/// PostgreSQL's cancel flags. A real cancel first sets the engine's token
/// — no new request starts, the sent ones finish — and the database's own
/// error then raises through the check. Any other interrupt is left to
/// `check_for_interrupts!`, which services it without ending the batch:
/// before this gate every interrupt was treated as a cancel, so a benign
/// one failed a paid batch (review 2).
///
/// The configured deadline rides the batch: the engine's guard stops it at
/// the deadline between requests, and a spent budget sends nothing.
///
/// A worker that panics — a defect, since every error a worker is meant to
/// report comes back as a value — is contained by the contract's shared
/// boundary and reported as the defect kind carrying the panic's own
/// words, so a bad question in a batch never reads as "the batch thread
/// stopped" with the cause lost.
fn run_batch<T, F>(work: F) -> Result<T, Error>
where
    F: FnOnce(Option<&thinkthen_contract::Cancel>, Option<Duration>) -> Result<T, Error>
        + Send
        + 'static,
    T: Send + 'static,
{
    let token = thinkthen_contract::Cancel::new();
    let worker = token.clone();
    let budget = batch_budget();
    let handle = std::thread::spawn(move || {
        thinkthen_contract::catch_panic("the batch thread", || work(Some(&worker), budget))
    });
    while !handle.is_finished() {
        if cancel_requested() {
            token.cancel();
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    let out = match handle.join() {
        Ok(out) => out,
        Err(payload) => Err(Error::defect(format!(
            "the batch thread stopped: {}",
            worker_panic_text(payload.as_ref())
        ))),
    };
    check_for_interrupts!();
    out
}

/// The text a stopped worker thread carried: a pgrx error report keeps
/// its PostgreSQL message, and every other payload takes the contract's
/// own formatter, so the spelling is the one every surface's boundary
/// uses.
fn worker_panic_text(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(report) = payload.downcast_ref::<pgrx::pg_sys::panic::ErrorReportWithLevel>() {
        return report.message().to_string();
    }
    thinkthen_contract::panic_text(payload)
}

/// Raise the engine's failure as the database's own error, with the kind,
/// the retry signal, and a SQLSTATE that matches the kind. A failure never
/// reads as a NULL.
fn raise(error: Error) -> ! {
    let (code, text) = surface(&error);
    ereport!(ERROR, code, text.as_str());
}

/// The PostgreSQL surface of a contract failure: the SQLSTATE a caller
/// sees and the message a reader sees. One `thinkthen {kind}:` prefix,
/// the contract's own message, and the retry signal — the shape the other
/// two surfaces render. Pure, so the mapping is unit-tested without a
/// running backend.
fn surface(error: &Error) -> (PgSqlErrorCode, String) {
    let code = match error.kind {
        ErrorKind::Usage => PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE,
        ErrorKind::Backend => PgSqlErrorCode::ERRCODE_EXTERNAL_ROUTINE_EXCEPTION,
        ErrorKind::Deadline | ErrorKind::Cancelled => PgSqlErrorCode::ERRCODE_QUERY_CANCELED,
        ErrorKind::Local => PgSqlErrorCode::ERRCODE_IO_ERROR,
        ErrorKind::Defect => PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
    };
    let retry = if error.retryable { "yes" } else { "no" };
    let text = format!("thinkthen {}: {} (retryable: {retry})", error.kind, error.message);
    (code, text)
}

/// The form a named argument takes: JSON text, or a file named with the
/// command's `@name` spelling.
#[derive(Debug, PartialEq, Eq)]
enum ArgForm<'a> {
    /// The path after `@`.
    File(&'a str),
    /// JSON text, the section or the whole grammar.
    Json(&'a str),
}

/// Decide what a question, question-set, or spec argument names. Pure, so
/// the rule is unit-tested without a backend.
///
/// A bare string that is not JSON never names a file: before this rule
/// any text without `@` was read as a path, so a role holding EXECUTE
/// could read server files with no `@` required (review 2, item 4). The
/// refusal names the required form.
fn arg_form<'a>(text: &'a str, what: &str) -> Result<ArgForm<'a>, Error> {
    if let Some(path) = text.strip_prefix('@') {
        return Ok(ArgForm::File(path));
    }
    if text.trim_start().starts_with('{') {
        return Ok(ArgForm::Json(text));
    }
    Err(Error::usage(format!(
        "a {what} file is named with the @ spelling: '@{text}'; bare text is never a path"
    )))
}

/// Resolve the question argument: `'@name'` names a file (the command's
/// spelling), and JSON text carries the grammar. Bare text that is not
/// JSON is a usage error naming the `@` form.
fn question_of(arg: Option<&str>) -> Question {
    let text = arg.unwrap_or_default();
    if text.trim().is_empty() {
        raise(Error::usage("the question is empty"));
    }
    match arg_form(text, "question").unwrap_or_else(|error| raise(error)) {
        ArgForm::File(path) => {
            Question::from_file(Path::new(path)).unwrap_or_else(|error| raise(error))
        }
        ArgForm::Json(text) => {
            Question::from_json(text).unwrap_or_else(|error| raise(error))
        }
    }
}

/// Resolve the question-set argument of `annotate`: `'@name'` names a set
/// file, and JSON text is the set itself. Bare text that is not JSON is a
/// usage error naming the `@` form.
fn set_of(arg: Option<&str>) -> QuestionSet {
    let text = arg.unwrap_or_default();
    if text.trim().is_empty() {
        raise(Error::usage("the question set is empty"));
    }
    match arg_form(text, "question set").unwrap_or_else(|error| raise(error)) {
        ArgForm::File(path) => {
            QuestionSet::from_file(Path::new(path)).unwrap_or_else(|error| raise(error))
        }
        ArgForm::Json(text) => {
            QuestionSet::from_json(text).unwrap_or_else(|error| raise(error))
        }
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

/// Resolve the `recognize` spec argument: `'@name'` names a file (the
/// question file's `recognize` section is taken when present), and JSON
/// text carries the spec itself. Bare text that is not JSON is a usage
/// error naming the `@` form (review 2, item 4).
///
/// The ruled spelling of a relation's ends is `source` and `target`
/// (`sdlc/planning/recognize-design.md`, the ruling of 2026-09-21), and
/// the spec crosses to the one core parser unchanged: this door converts
/// nothing, so `from`/`to` is refused here the way every other door
/// refuses it, with the ruled spelling named.
fn recognizer_of(arg: Option<&str>) -> Recognize {
    let text = arg.unwrap_or_default();
    if text.trim().is_empty() {
        raise(Error::usage("the recognize spec is empty"));
    }
    let json = match arg_form(text, "recognize spec").unwrap_or_else(|error| raise(error)) {
        ArgForm::File(path) => read_spec(path),
        ArgForm::Json(text) => text.to_owned(),
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
    Recognize::from_json(&value.to_string()).unwrap_or_else(|error| raise(error))
}

/// One annotate answer as JSON: a decision is true, false, or null; a
/// choice is the label or null; a score is its number; tags are a list; a
/// failed logical question carries the ruled marker in its place (0054).
fn annotated_as_json(held: &Annotated) -> serde_json::Value {
    match held {
        Annotated::Decision(answer) => serde_json::json!(answer.value()),
        Annotated::Choice(Some(won)) => serde_json::json!(won),
        Annotated::Choice(None) => serde_json::Value::Null,
        Annotated::Score(scored) => serde_json::json!(scored.value),
        Annotated::Tags(labels) => serde_json::json!(labels),
        // The ruled failed marker, in the same place a value would sit:
        // `{"failed":{"kind":"backend","cause":CAUSE}}` (0054).
        Annotated::Failed(failed) => serde_json::json!({ "failed": failed }),
    }
}

#[pg_extern(parallel_restricted)]
fn thinkthen_decide(question: Option<&str>, evidence: Option<&str>) -> Option<bool> {
    let question = question_of(question);
    let Some(evidence) = evidence else { return None };
    match engine().decide_opts(&question, evidence, call_options()) {
        Ok(answer) => answer.value(),
        Err(error) => raise(error),
    }
}

#[pg_extern(parallel_restricted)]
fn thinkthen_probability(question: Option<&str>, evidence: Option<&str>) -> Option<f64> {
    let question = question_of(question);
    let Some(evidence) = evidence else { return None };
    match engine().decide_many_opts(&question, &[evidence], call_options(), None) {
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
    match engine().choose_opts(&question, evidence, call_options()) {
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
    match engine().score_opts(&question, evidence, call_options()) {
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
    match engine().tag_opts(&question, evidence, call_options()) {
        Ok(held) => Some(held),
        Err(error) => raise(error),
    }
}

#[pg_extern(parallel_restricted)]
fn thinkthen_annotate(set: Option<&str>, evidence: Option<&str>) -> Option<JsonB> {
    let set = set_of(set);
    let Some(evidence) = evidence else { return None };
    match engine().annotate_opts(&set, &[evidence], call_options(), None) {
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
    match engine().details_opts(&question, evidence, call_options()) {
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
    let found = engine().recognize_opts(&ask, body, call_options()).unwrap_or_else(|error| raise(error));
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
    let engine = engine();
    let edges = run_batch(move |cancel, budget| {
        let texts: Vec<&str> = owned.iter().map(String::as_str).collect();
        let options = batch_options(cancel, budget);
        relate_checked(engine.as_ref(), &ask, &texts, options)
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
    let found = engine().recognize_opts(&ask, body, call_options()).unwrap_or_else(|error| raise(error));
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
    // The question resolves on the backend thread, before any worker
    // spawns: a bad question file raises its own error (naming the file)
    // instead of dying inside the worker and reading as a stopped thread.
    let question = question_of(Some(&question_text));
    let rows: Vec<Option<String>> = evidences
        .iter()
        .flat_map(|array| array.iter().map(|text| text.map(str::to_owned)))
        .collect();
    let distinct = distinct_of(rows.iter().map(|text| text.as_deref()));
    let owned = distinct.clone();
    let engine = engine();
    let judged = run_batch(move |cancel, budget| {
        let records: Vec<&str> = owned.iter().map(String::as_str).collect();
        let options = batch_options(cancel, budget);
        engine.decide_many_opts(&question, &records, options, None)
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
            // The question resolves on the backend thread, before any
            // worker spawns: a bad question file raises its own error
            // (naming the file) instead of dying inside the worker.
            let question = question_of(Some(&question_text));
            let engine = engine();
            let outcome = run_batch(move |cancel, budget| {
                let records: Vec<&str> = distinct.iter().map(String::as_str).collect();
                let options = batch_options(cancel, budget);
                engine.decide_many_opts(&question, &records, options, None)
            });
            match outcome {
                Ok(held) => judged += i64::try_from(held.len()).unwrap_or(i64::MAX),
                Err(error) => raise(error),
            }
        }
        judged
    }
}

/// Register the key setting and the deadline setting. Nothing else
/// happens here: the engine builds lazily in each backend, after the
/// fork, and `_PG_init` never touches the wire.
#[pg_guard]
extern "C-unwind" fn _PG_init() {
    GucRegistry::define_int_guc(
        c"thinkthen.deadline_ms",
        c"the per-call deadline in milliseconds: -1 none, 0 spent, positive a budget",
        c"",
        &DEADLINE_MS,
        -1,
        i32::MAX,
        GucContext::Userset,
        GucFlags::default(),
    );
    GucRegistry::define_string_guc(
        c"thinkthen.api_key",
        c"the API key for the ThinkThen engine",
        c"",
        &API_KEY,
        GucContext::Suset,
        GucFlags::NO_SHOW_ALL | GucFlags::SUPERUSER_ONLY | GucFlags::DISALLOW_IN_AUTO_FILE,
    );
}

// Revoke the default PUBLIC grant on every function this extension
// installs, and on nothing else.
//
// PostgreSQL grants EXECUTE on a new function to PUBLIC by default, so
// without this every role could make paid calls and read server files
// through `@path`. The loop reads the functions the extension owns from
// `pg_depend`, so it covers both `thinkthen_decide` overloads, the
// `thinkthen_warm` aggregate, and the aggregate's own support functions
// (`REVOKE ... ON FUNCTION` accepts an aggregate's signature).
//
// The narrowed grant an administrator runs to let one application role
// call the surface (which also grants the `@path` reads, so a role with
// it is trusted). It names the extension's own functions through
// `pg_depend`, never `ALL FUNCTIONS IN SCHEMA public`, which would also
// grant every other extension's functions (review 2, item 4):
//
//     DO $thinkthen_grant$
//     DECLARE signature text;
//     BEGIN
//         FOR signature IN
//             SELECT p.oid::regprocedure::text
//             FROM pg_proc p
//             JOIN pg_depend d
//               ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass
//             JOIN pg_extension e
//               ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass
//             WHERE e.extname = 'thinkthen'
//         LOOP
//             EXECUTE format('GRANT EXECUTE ON FUNCTION %s TO %I', signature, 'the_app_role');
//         END LOOP;
//     END
//     $thinkthen_grant$;
extension_sql!(
    r#"
DO $thinkthen_revoke$
DECLARE
    signature text;
BEGIN
    FOR signature IN
        SELECT p.oid::regprocedure::text
        FROM pg_proc p
        JOIN pg_depend d
          ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass
        JOIN pg_extension e
          ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass
        WHERE e.extname = 'thinkthen'
    LOOP
        EXECUTE format('REVOKE ALL ON FUNCTION %s FROM PUBLIC', signature);
    END LOOP;
END
$thinkthen_revoke$;
"#,
    name = "revoke_public",
    finalize,
);

// Keep every function this extension owns out of PUBLIC's hands, including
// the ones a future `ALTER EXTENSION UPDATE` creates.
//
// The revoke above runs once, at CREATE EXTENSION. An update script that
// creates a new function would hand it EXECUTE by PostgreSQL's default
// grant, and an unchanged revoke block never appears in a diff-based
// update script (review 2, item 4). This event trigger closes that path
// mechanically: whenever a function, procedure, or aggregate is created
// in this database, it revokes PUBLIC on every function the extension
// owns. It is SECURITY DEFINER — the DDL may be run by a role that does
// not own the functions — with its search path pinned, and it grants
// nothing: the narrowed grant above is the administrator's act.
extension_sql!(
    r#"
CREATE FUNCTION thinkthen_guard_public() RETURNS event_trigger
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog AS $thinkthen_guard$
DECLARE
    signature text;
BEGIN
    FOR signature IN
        SELECT p.oid::regprocedure::text
        FROM pg_proc p
        JOIN pg_depend d
          ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass
        JOIN pg_extension e
          ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass
        WHERE e.extname = 'thinkthen'
    LOOP
        EXECUTE format('REVOKE ALL ON FUNCTION %s FROM PUBLIC', signature);
    END LOOP;
END
$thinkthen_guard$;

CREATE EVENT TRIGGER thinkthen_guard_public
    ON ddl_command_end
    WHEN TAG IN ('CREATE FUNCTION', 'CREATE PROCEDURE', 'CREATE AGGREGATE')
    EXECUTE FUNCTION thinkthen_guard_public();
"#,
    name = "guard_public",
    finalize,
);

#[cfg(test)]
mod mapping_tests {
    use super::*;

    /// Punch-list item 5: a configured key must never be silently ignored.
    /// A set, non-blank value refuses with the usage kind and a message
    /// naming the setting and the substitute channel; an absent or blank
    /// value passes, and no value ever rides the message.
    #[test]
    fn a_configured_key_refuses_but_a_blank_one_passes() {
        assert!(unwired_key_refusal(None).is_none());
        assert!(unwired_key_refusal(Some("   ")).is_none());
        let error = unwired_key_refusal(Some("made-up-not-a-key")).expect("a set key refuses");
        assert_eq!(error.kind, ErrorKind::Usage);
        assert!(error.message.contains("thinkthen.api_key"), "{}", error.message);
        assert!(error.message.contains("THINKTHEN_API_KEY"), "{}", error.message);
        assert!(!error.message.contains("made-up-not-a-key"), "the value never rides the message");
    }

    /// Ruling 2 of the product rulings: the defect kind maps to this
    /// engine's own error surface, with the internal-error SQLSTATE and
    /// the kind named in the message. No public door carries a fault hook;
    /// this proves the mapping at the shim level.
    #[test]
    fn the_defect_kind_maps_to_the_engines_error() {        let (code, text) = surface(&Error::defect("the relate plan lost its bind data"));
        assert_eq!(code, PgSqlErrorCode::ERRCODE_INTERNAL_ERROR);
        assert!(text.contains("thinkthen defect:"), "{text}");
        assert!(text.contains("the relate plan lost its bind data"), "{text}");
    }

    /// The message shape the other two surfaces render: one
    /// `thinkthen {kind}:` prefix, the contract's own message, and the
    /// retry signal. The kind word appeared twice before this lane
    /// touched the function (`thinkthen usage: usage: ...`), because the
    /// contract's Display already carries it.
    #[test]
    fn the_kind_word_appears_once() {
        let (code, text) = surface(&Error::usage("the question is empty"));
        assert_eq!(code, PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE);
        assert_eq!(text, "thinkthen usage: the question is empty (retryable: no)");
        let (code, text) = surface(&Error::backend_retryable("the backend is busy"));
        assert_eq!(code, PgSqlErrorCode::ERRCODE_EXTERNAL_ROUTINE_EXCEPTION);
        assert_eq!(text, "thinkthen backend: the backend is busy (retryable: yes)");
    }

    /// A worker that panics keeps its own message: `run_batch` reports the
    /// defect kind with the payload's text, so a bad question in a batch
    /// never reads as "the batch thread stopped" with the cause lost.
    /// Before the fix the payload was dropped on the floor; a plain
    /// payload now takes the contract's shared spelling.
    #[test]
    fn a_stopped_worker_keeps_its_message() {
        let payload = std::panic::catch_unwind(|| panic!("the relate plan lost its bind data"))
            .expect_err("the probe panicked");
        let text = worker_panic_text(payload.as_ref());
        assert!(
            text.contains("the relate plan lost its bind data"),
            "{text}"
        );
    }

    /// A named argument is JSON text or the `@name` file spelling; bare
    /// text is never a path. Before the rule (review 2, item 4) any text
    /// without `@` was read as a server file, and the refusal now names
    /// the required form.
    #[test]
    fn a_bare_path_is_refused_and_the_at_form_passes() {
        assert_eq!(arg_form("@refund.json", "question"), Ok(ArgForm::File("refund.json")));
        assert_eq!(
            arg_form("  {\"decide\": \"Is this a complaint?\"}", "question"),
            Ok(ArgForm::Json("  {\"decide\": \"Is this a complaint?\"}"))
        );
        let refusal = arg_form("/etc/passwd", "question").expect_err("a bare path refuses");
        assert_eq!(refusal.kind, ErrorKind::Usage, "{refusal:?}");
        assert!(refusal.message.contains("@/etc/passwd"), "{}", refusal.message);
        assert!(refusal.message.contains("never a path"), "{}", refusal.message);
        let refusal = arg_form("names.json", "recognize spec").expect_err("a bare name refuses");
        assert!(refusal.message.contains("@names.json"), "{}", refusal.message);
    }

    /// The deadline setting's conversion: `-1` is no deadline, `0` is a
    /// spent deadline, a positive value is the budget, and any other
    /// negative is refused by the contract's one checked door.
    #[test]
    fn the_deadline_setting_converts_by_the_ruled_rule() {
        assert_eq!(budget_of(-1).expect("the sentinel passes"), None);
        assert_eq!(budget_of(0).expect("a spent budget is legal"), Some(Duration::ZERO));
        assert_eq!(
            budget_of(250).expect("a budget passes"),
            Some(Duration::from_millis(250))
        );
        let refusal = budget_of(-2).expect_err("a negative below the sentinel refuses");
        assert_eq!(refusal.kind, ErrorKind::Usage, "{refusal:?}");
    }
}
