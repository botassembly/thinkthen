//! The Python surface: `import thinkthen as tt`.
//!
//! One engine under ten surfaces, per ADR 0017. This shim binds
//! `thinkthen-contract` and never the engine beneath it: it converts
//! Python arguments into a contract question, calls one engine function,
//! converts the result, and maps the six error kinds to Python's own
//! exceptions. No rule, no retry, and no sending lives here.
//!
//! The ruled shape, from the ADR and the one-shape page:
//!
//! - the same names everywhere, `tt.` prefixed: `decide`, `choose`,
//!   `score`, `tag`, `filter`, `rank`, `find`, `annotate`, plus
//!   `decide_many` as decide's bulk spelling;
//! - `None` is "not sure";
//! - a list crosses into the engine once and runs at the engine's width;
//! - a failure raises Python's own error and never reads as `False`;
//! - every entry point takes `deadline` in seconds from the moment of the
//!   call, and every raised error carries `retryable`.
//!
//! While a bulk call waits, the engine runs this shim's poll on the
//! calling thread; the poll re-takes the interpreter lock and runs
//! `PyErr_CheckSignals`. When a signal fires the poll sets the cancel
//! token, no new request starts, the sent requests finish, and the call
//! raises `Cancelled` (a subclass of `KeyboardInterrupt`, so one
//! `except KeyboardInterrupt` catches both paths). Worst case is one tick
//! plus one in-flight round, per the experiment 211 proof.

mod arrow;
mod generated;

use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use pyo3::create_exception;
use pyo3::exceptions::PyKeyboardInterrupt;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};

use thinkthen_contract::Annotated;
use thinkthen_contract::Answer;
use thinkthen_contract::Cancel;
use thinkthen_contract::Details;
use thinkthen_contract::Edge as ContractEdge;
use thinkthen_contract::Engine;
use thinkthen_contract::Error;
use thinkthen_contract::ErrorKind;
use thinkthen_contract::Kind;
use thinkthen_contract::Options;
use thinkthen_contract::Question as ContractQuestion;
use thinkthen_contract::QuestionSet;
use thinkthen_contract::Recognize;
use thinkthen_contract::Relate;
use thinkthen_contract::RelationRule;
use thinkthen_contract::Scored;
use thinkthen_standin::BlockingEngine;

// The exceptions, one a kind. `Cancelled` subclasses `KeyboardInterrupt`,
// the host's own cancel gesture, so both interrupt paths land in one
// `except KeyboardInterrupt` handler; pyo3 gives an exception one base.
create_exception!(
    thinkthen._thinkthen,
    ThinkThenError,
    pyo3::exceptions::PyException,
    "the base error every thinkthen failure raises"
);
create_exception!(
    thinkthen._thinkthen,
    UsageError,
    ThinkThenError,
    "the request was wrong, and nothing was sent"
);
create_exception!(
    thinkthen._thinkthen,
    BackendError,
    ThinkThenError,
    "the wire failed or refused the request"
);
create_exception!(
    thinkthen._thinkthen,
    DeadlineError,
    ThinkThenError,
    "the caller's own budget ran out; it says nothing about the backend's health"
);
create_exception!(
    thinkthen._thinkthen,
    LocalError,
    ThinkThenError,
    "a local file or recording the caller named failed"
);
create_exception!(
    thinkthen._thinkthen,
    Cancelled,
    PyKeyboardInterrupt,
    "the wait was cancelled; no new request started and the sent requests finished"
);
create_exception!(
    thinkthen._thinkthen,
    DefectError,
    ThinkThenError,
    "the engine broke its own contract"
);

/// One built question, from `tt.question` or a question file.
#[pyclass(from_py_object)]
#[derive(Clone)]
struct Question {
    inner: ContractQuestion,
}

#[pymethods]
impl Question {
    /// The digest of this question with its threshold, 64 hex figures.
    /// Parts and files give the same digest when they give the same
    /// question.
    fn digest(&self) -> String {
        self.inner.digest()
    }
}

/// One loaded question set, from a file this surface's `annotate` reads.
#[pyclass(from_py_object)]
#[derive(Clone)]
struct QuestionSetHolder {
    inner: QuestionSet,
}

/// The engine this process shares, built from the environment on first
/// use so a host can set its variables after the import.
fn engine() -> &'static BlockingEngine {
    static ENGINE: OnceLock<BlockingEngine> = OnceLock::new();
    ENGINE.get_or_init(BlockingEngine::from_env)
}

/// The options a single call takes: a deadline from a budget in seconds.
fn call_options(deadline: Option<f64>) -> Options<'static> {
    match deadline {
        Some(seconds) => Options::new().deadline_in(Duration::from_secs_f64(seconds.max(0.0))),
        None => Options::new(),
    }
}

/// Map a contract error to the Python exception of its kind, carrying the
/// kind's name and the retry signal on the instance.
fn python_error(py: Python<'_>, error: Error) -> PyErr {
    let class: fn(String) -> PyErr = match error.kind {
        ErrorKind::Usage => UsageError::new_err,
        ErrorKind::Backend => BackendError::new_err,
        ErrorKind::Deadline => DeadlineError::new_err,
        ErrorKind::Local => LocalError::new_err,
        ErrorKind::Cancelled => Cancelled::new_err,
        ErrorKind::Defect => DefectError::new_err,
    };
    let raised = class(error.message);
    let value = raised.value(py);
    let _ = value.setattr("kind", format!("{:?}", error.kind).to_lowercase());
    let _ = value.setattr("retryable", error.retryable);
    raised
}

/// A decide question from text alone, under the grammar's default cut.
fn text_question(py: Python<'_>, text: &str) -> PyResult<ContractQuestion> {
    let body = serde_json::json!({ "decide": text });
    ContractQuestion::from_json(&body.to_string()).map_err(|error| python_error(py, error))
}

/// A question argument for the decide family: text alone or a built
/// question.
fn settle_question(py: Python<'_>, object: &Bound<'_, PyAny>) -> PyResult<ContractQuestion> {
    if let Ok(question) = object.extract::<Question>() {
        return Ok(question.inner);
    }
    if let Ok(text) = object.extract::<String>() {
        return text_question(py, &text);
    }
    Err(UsageError::new_err(
        "the question is text or a built question from tt.question",
    ))
}

/// A question argument for a verb with members: text plus this call's
/// members, or a built question.
fn settle_verb_question(
    py: Python<'_>,
    object: &Bound<'_, PyAny>,
    verb: &str,
    key: &str,
    members: Option<Vec<String>>,
) -> PyResult<ContractQuestion> {
    if let Ok(question) = object.extract::<Question>() {
        return Ok(question.inner);
    }
    let text = object.extract::<String>().map_err(|_| {
        UsageError::new_err(format!(
            "a {verb} question is text plus its {key}, or a built question"
        ))
    })?;
    let list = members.ok_or_else(|| {
        UsageError::new_err(format!("a {verb} question needs its {key} beside the text"))
    })?;
    let mut body = serde_json::Map::new();
    body.insert(verb.to_owned(), serde_json::json!(text));
    body.insert(key.to_owned(), serde_json::json!(list));
    let held = serde_json::Value::Object(body).to_string();
    ContractQuestion::from_json(&held).map_err(|error| python_error(py, error))
}

/// A question set argument: a built set or a path to a question file.
fn settle_set(py: Python<'_>, object: &Bound<'_, PyAny>) -> PyResult<QuestionSet> {
    if let Ok(set) = object.extract::<QuestionSetHolder>() {
        return Ok(set.inner);
    }
    if let Ok(path) = object.extract::<String>() {
        QuestionSet::from_file(std::path::Path::new(&path))
            .map_err(|error| python_error(py, error))
    } else {
        Err(UsageError::new_err(
            "the set is a question file's path or a loaded set",
        ))
    }
}

/// The bare value of an answer: `True`, `False`, or `None` when unsure.
fn bare(py: Python<'_>, answer: Answer) -> Py<PyAny> {
    match answer.value() {
        Some(value) => pyo3::types::PyBool::new(py, value).to_owned().unbind().into_any(),
        None => py.None(),
    }
}

/// A plain string or `None`.
fn plain(py: Python<'_>, value: Option<String>) -> Py<PyAny> {
    match value {
        Some(text) => pyo3::types::PyString::new(py, &text).unbind().into_any(),
        None => py.None(),
    }
}

/// Run one bulk verb with the signal poll. `call` receives the armed
/// options and the poll, all inside `detach`, so the interpreter lock is
/// free while the engine works.
fn bulk<T: Send>(
    py: Python<'_>,
    deadline: Option<f64>,
    call: impl FnOnce(Options<'_>, Option<&mut dyn FnMut()>) -> Result<T, Error> + Send,
) -> PyResult<T> {
    let flagged = Arc::new(AtomicBool::new(false));
    let token = Cancel::new();
    let war = token.clone();
    let armed = call_options(deadline).maybe_cancel(Some(&token));
    let result = {
        let flagged = Arc::clone(&flagged);
        py.detach(move || {
            let mut poll = move || {
                if flagged.load(Ordering::Relaxed) {
                    return;
                }
                Python::attach(|py| {
                    if py.check_signals().is_err() {
                        flagged.store(true, Ordering::Relaxed);
                        war.cancel();
                    }
                });
            };
            call(armed, Some(&mut poll as &mut dyn FnMut()))
        })
    };
    match result {
        Ok(value) => Ok(value),
        Err(error) => {
            if error.kind == ErrorKind::Cancelled && flagged.load(Ordering::Relaxed) {
                let raised = Cancelled::new_err(
                    "interrupted: no new request started, and every sent request finished",
                );
                let _ = raised.value(py).setattr("retryable", false);
                Err(raised)
            } else {
                Err(python_error(py, error))
            }
        }
    }
}

/// Build a question from parts, through the one file grammar so parts and
/// files give the same digest.
#[pyfunction(signature = (*, decide = None, choose = None, score = None, tag = None, threshold = None, options = None, labels = None, levels = None, true_ = None, false_ = None, model = None, file = None))]
#[allow(clippy::too_many_arguments)]
fn question(
    py: Python<'_>,
    decide: Option<String>,
    choose: Option<String>,
    score: Option<String>,
    tag: Option<String>,
    threshold: Option<&Bound<'_, PyAny>>,
    options: Option<Vec<String>>,
    labels: Option<Vec<String>>,
    levels: Option<Vec<String>>,
    true_: Option<String>,
    false_: Option<String>,
    model: Option<String>,
    file: Option<String>,
) -> PyResult<Question> {
    if let Some(path) = file {
        let inner = ContractQuestion::from_file(std::path::Path::new(&path))
            .map_err(|error| python_error(py, error))?;
        return Ok(Question { inner });
    }
    let mut body = serde_json::Map::new();
    if let Some(text) = decide {
        body.insert("decide".into(), serde_json::json!(text));
        if let Some(yes) = true_ {
            body.insert("true".into(), serde_json::json!(yes));
        }
        if let Some(no) = false_ {
            body.insert("false".into(), serde_json::json!(no));
        }
    } else if let Some(text) = choose {
        body.insert("choose".into(), serde_json::json!(text));
        body.insert(
            "options".into(),
            serde_json::json!(options.ok_or_else(|| {
                UsageError::new_err("a choose question needs its options beside the text")
            })?),
        );
    } else if let Some(text) = score {
        body.insert("score".into(), serde_json::json!(text));
        body.insert(
            "levels".into(),
            serde_json::json!(levels.ok_or_else(|| {
                UsageError::new_err("a score question needs its levels beside the text")
            })?),
        );
    } else if let Some(text) = tag {
        body.insert("tag".into(), serde_json::json!(text));
        body.insert(
            "labels".into(),
            serde_json::json!(labels.ok_or_else(|| {
                UsageError::new_err("a tag question needs its labels beside the text")
            })?),
        );
    } else {
        return Err(UsageError::new_err(
            "question takes exactly one of decide, choose, score, or tag",
        ));
    }
    if let Some(mark) = threshold {
        if let Ok(cut) = mark.extract::<f64>() {
            body.insert("threshold".into(), serde_json::json!(cut));
        } else if let Ok((low, high)) = mark.extract::<(f64, f64)>() {
            body.insert("threshold".into(), serde_json::json!(format!("{low}:{high}")));
        } else {
            return Err(UsageError::new_err(
                "threshold is a cut (a number) or a band (a low, high pair)",
            ));
        }
    }
    if let Some(name) = model {
        body.insert("model".into(), serde_json::json!(name));
    }
    let text = serde_json::Value::Object(body).to_string();
    let inner = ContractQuestion::from_json(&text).map_err(|error| python_error(py, error))?;
    Ok(Question { inner })
}

/// Ask once. `None` is "not sure".
#[pyfunction(signature = (question, evidence, *, deadline = None))]
fn decide(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    evidence: String,
    deadline: Option<f64>,
) -> PyResult<Py<PyAny>> {
    let asked = settle_question(py, question)?;
    let answer = py
        .detach(move || engine().decide_opts(&asked, &evidence, call_options(deadline)))
        .map_err(|error| python_error(py, error))?;
    Ok(bare(py, answer))
}

/// Ask of every record, once, keeping every judgment in input order.
/// A list or tuple crosses as strings; an object answering
/// `__arrow_c_stream__` (a Polars Series) crosses zero-copy as Arrow.
#[pyfunction(signature = (question, records, *, deadline = None))]
fn decide_many(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    records: &Bound<'_, PyAny>,
    deadline: Option<f64>,
) -> PyResult<Vec<Py<PyAny>>> {
    let asked = settle_question(py, question)?;
    if arrow::is_arrow(records)? {
        let column = arrow::series_column(records)?;
        let references: Vec<&str> = column.texts.clone();
        let judgments = bulk(py, deadline, |armed, poll| {
            engine().decide_many_opts(&asked, &references, armed, poll)
        })?;
        drop(column);
        return judgments
            .into_iter()
            .map(|one| Ok(bare(py, one.answer)))
            .collect();
    }
    let records: Vec<String> = records.extract()?;
    let references: Vec<&str> = records.iter().map(String::as_str).collect();
    let judgments = bulk(py, deadline, |armed, poll| {
        engine().decide_many_opts(&asked, &references, armed, poll)
    })?;
    judgments.into_iter().map(|one| Ok(bare(py, one.answer))).collect()
}

/// Pick the option the evidence fits best. `None` is unresolved.
#[pyfunction(signature = (question, evidence, *, options = None, deadline = None))]
fn choose(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    evidence: String,
    options: Option<Vec<String>>,
    deadline: Option<f64>,
) -> PyResult<Py<PyAny>> {
    let asked = settle_verb_question(py, question, "choose", "options", options)?;
    let picked = py
        .detach(move || engine().choose_opts(&asked, &evidence, call_options(deadline)))
        .map_err(|error| python_error(py, error))?;
    Ok(plain(py, picked))
}

/// Place the evidence on the question's levels: the number, 0 to K-1.
#[pyfunction(signature = (question, evidence, *, levels = None, deadline = None))]
fn score(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    evidence: String,
    levels: Option<Vec<String>>,
    deadline: Option<f64>,
) -> PyResult<f64> {
    let asked = settle_verb_question(py, question, "score", "levels", levels)?;
    py.detach(move || engine().score_opts(&asked, &evidence, call_options(deadline)))
        .map(|Scored { value, .. }| value)
        .map_err(|error| python_error(py, error))
}

/// Name the labels that held, in the question's order.
#[pyfunction(signature = (question, evidence, *, labels = None, deadline = None))]
fn tag(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    evidence: String,
    labels: Option<Vec<String>>,
    deadline: Option<f64>,
) -> PyResult<Vec<String>> {
    let asked = settle_verb_question(py, question, "tag", "labels", labels)?;
    py.detach(move || engine().tag_opts(&asked, &evidence, call_options(deadline)))
        .map_err(|error| python_error(py, error))
}

/// Keep the records whose evidence reached the mark, in order. The caller
/// gets back its own records.
#[pyfunction(signature = (question, records, *, deadline = None))]
#[allow(clippy::needless_pass_by_value)]
fn filter(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    records: Vec<String>,
    deadline: Option<f64>,
) -> PyResult<Vec<String>> {
    let asked = settle_question(py, question)?;
    let references: Vec<&str> = records.iter().map(String::as_str).collect();
    let kept = bulk(py, deadline, |armed, poll| {
        engine().filter_opts(&asked, &references, armed, poll)
    })?;
    Ok(kept.into_iter().map(|place| records[place].clone()).collect())
}

/// Order the records most likely yes first, ties in input order.
#[pyfunction(signature = (question, records, *, top = None, deadline = None))]
#[allow(clippy::needless_pass_by_value)]
fn rank(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    records: Vec<String>,
    top: Option<usize>,
    deadline: Option<f64>,
) -> PyResult<Vec<String>> {
    let asked = settle_question(py, question)?;
    let references: Vec<&str> = records.iter().map(String::as_str).collect();
    let ranked = bulk(py, deadline, |armed, poll| {
        engine().rank_opts(&asked, &references, armed, poll)
    })?;
    let ordered = ranked.into_iter().map(|one| records[one.index].clone());
    Ok(match top {
        Some(count) => ordered.take(count).collect(),
        None => ordered.collect(),
    })
}

/// Pick the unit that best answers the question. `None` is nothing fits.
#[pyfunction(signature = (question, units, *, deadline = None))]
#[allow(clippy::needless_pass_by_value)]
fn find(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    units: Vec<String>,
    deadline: Option<f64>,
) -> PyResult<Py<PyAny>> {
    let asked = settle_question(py, question)?;
    let references: Vec<&str> = units.iter().map(String::as_str).collect();
    let found = py
        .detach(move || engine().find_opts(&asked, &references, call_options(deadline)))
        .map_err(|error| python_error(py, error))?;
    Ok(match found.index {
        Some(place) => pyo3::types::PyString::new(py, &units[place]).unbind().into_any(),
        None => py.None(),
    })
}

/// One record's fields as a plain dictionary, in the set's name order.
fn annotated_row(py: Python<'_>, fields: Vec<(String, Annotated)>) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    for (name, field) in fields {
        let value = match field {
            Annotated::Decision(answer) => bare(py, answer),
            Annotated::Choice(picked) => plain(py, picked),
            Annotated::Score(Scored { value, .. }) => {
                pyo3::types::PyFloat::new(py, value).unbind().into_any()
            }
            Annotated::Tags(held) => held
                .into_pyobject(py)
                .map_err(python_error_of)?
                .unbind()
                .into_any(),
        };
        dict.set_item(name, value)?;
    }
    Ok(dict.into_any().unbind())
}

/// Ask every question in the set of every record, one dictionary a record
/// in the set's name order. The wrapper in `thinkthen/__init__.py` hands a
/// data frame's column over and attaches the new columns on the way back.
#[pyfunction(signature = (set, records, *, deadline = None))]
#[allow(clippy::needless_pass_by_value)]
fn annotate_rows(
    py: Python<'_>,
    set: &Bound<'_, PyAny>,
    records: Vec<String>,
    deadline: Option<f64>,
) -> PyResult<Vec<Py<PyAny>>> {
    let loaded = settle_set(py, set)?;
    let references: Vec<&str> = records.iter().map(String::as_str).collect();
    let rows = bulk(py, deadline, |armed, poll| {
        engine().annotate_opts(&loaded, &references, armed, poll)
    })?;
    rows.into_iter().map(|fields| annotated_row(py, fields)).collect()
}

/// Annotate a frame's `on` column: the caller's own columns come back
/// aliased and one new column a question is appended, all through the
/// Arrow stream form. `type(records)(frame)` builds the host's frame.
#[pyfunction(signature = (set, records, on, *, deadline = None))]
fn annotate_stream(
    py: Python<'_>,
    set: &Bound<'_, PyAny>,
    records: &Bound<'_, PyAny>,
    on: String,
    deadline: Option<f64>,
) -> PyResult<arrow::ArrowFrame> {
    let loaded = settle_set(py, set)?;
    let frame = arrow::frame_column(records, &on)?;
    let references: Vec<&str> = frame.texts.clone();
    let rows = bulk(py, deadline, |armed, poll| {
        engine().annotate_opts(&loaded, &references, armed, poll)
    })?;
    let names: Vec<String> = loaded.names().to_vec();
    let out = arrow::build_frame(frame, &names, &rows)?;
    Ok(arrow::ArrowFrame::new(out))
}

/// One judgment plus the audit trail, with the sends that produced it.
#[pyfunction(signature = (question, evidence, *, deadline = None))]
fn details(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    evidence: String,
    deadline: Option<f64>,
) -> PyResult<Py<PyAny>> {
    let asked = settle_question(py, question)?;
    let Details { probability, answer, model, digest, sends } = py
        .detach(move || engine().details_opts(&asked, &evidence, call_options(deadline)))
        .map_err(|error| python_error(py, error))?;
    let dict = PyDict::new(py);
    dict.set_item("probability", probability)?;
    dict.set_item("answer", bare(py, answer))?;
    dict.set_item("model", model)?;
    dict.set_item("digest", digest)?;
    dict.set_item("sends", sends)?;
    Ok(dict.into_any().unbind())
}

/// One name `recognize` found: the user's own kind word and where the
/// name sits in the text the user gave. `number` is the interim field
/// name for the number on a name; the open comparison behind it is the
/// recognize team's, and the marketing vocabulary page
/// (`repos/mktg/products/thinkthen/vocabulary.md`, "The words for
/// numbers") restricts `confidence` to the vendor's own literal field,
/// so this number never carries that word here.
#[pyclass(frozen, skip_from_py_object)]
#[derive(Clone)]
struct Entity {
    #[pyo3(get)]
    id: u64,
    #[pyo3(get)]
    text: String,
    #[pyo3(get)]
    kind: String,
    #[pyo3(get)]
    start: u64,
    #[pyo3(get)]
    end: u64,
    #[pyo3(get)]
    number: f64,
}

#[pymethods]
impl Entity {
    fn __repr__(&self) -> String {
        format!(
            "Entity(id={}, text={:?}, kind={:?}, start={}, end={}, number={})",
            self.id, self.text, self.kind, self.start, self.end, self.number
        )
    }
}

/// One relation between two names, by entity id. The ends are spelled
/// `source` and `target` on every surface; the number is the model's own
/// probability for the picked relation.
#[pyclass(frozen, skip_from_py_object)]
#[derive(Clone)]
struct Relation {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    source: u64,
    #[pyo3(get)]
    target: u64,
    #[pyo3(get)]
    probability: f64,
}

#[pymethods]
impl Relation {
    fn __repr__(&self) -> String {
        format!(
            "Relation(name={:?}, source={}, target={}, probability={})",
            self.name, self.source, self.target, self.probability
        )
    }
}

/// What `recognize` returned: the names, and the relations when a rule
/// was given.
#[pyclass(frozen, skip_from_py_object)]
#[derive(Clone)]
struct Recognized {
    entities: Vec<Entity>,
    relations: Vec<Relation>,
}

#[pymethods]
impl Recognized {
    #[getter]
    fn entities(&self) -> Vec<Entity> {
        self.entities.clone()
    }

    #[getter]
    fn relations(&self) -> Vec<Relation> {
        self.relations.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "Recognized(entities={}, relations={})",
            self.entities.len(),
            self.relations.len()
        )
    }
}

/// One edge `relate` found, by record numbers counted from 1 in input
/// order. An `either` edge prints once, with the lower number in
/// `source`.
#[pyclass(frozen, skip_from_py_object)]
#[derive(Clone)]
struct Edge {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    source: u64,
    #[pyo3(get)]
    target: u64,
    #[pyo3(get)]
    probability: f64,
    #[pyo3(get)]
    source_kind: Option<String>,
    #[pyo3(get)]
    target_kind: Option<String>,
}

#[pymethods]
impl Edge {
    fn __repr__(&self) -> String {
        format!(
            "Edge(name={:?}, source={}, target={}, probability={})",
            self.name, self.source, self.target, self.probability
        )
    }
}

/// The ruled question-file section for one function: the named section
/// when the file nests one, with the file's top-level thresholds carried
/// into the section when it lacks them, else the whole object. Reading
/// the file is this surface's own; every grammar check is the core's.
fn spec_from_file(path: &str, section: &str) -> Result<String, Error> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| Error::local(format!("{path}: {error}")))?;
    let mut value: serde_json::Value = serde_json::from_str(&text)
        .map_err(|error| Error::usage(format!("the question file {path} is not JSON: {error}")))?;
    let object = value
        .as_object_mut()
        .ok_or_else(|| Error::usage(format!("the question file {path} is not a JSON object")))?;
    let spec = match object.get(section).and_then(serde_json::Value::as_object) {
        Some(section) => {
            let mut spec = section.clone();
            for key in ["threshold", "relation_threshold"] {
                if !spec.contains_key(key) {
                    if let Some(carried) = object.get(key) {
                        spec.insert(key.to_owned(), carried.clone());
                    }
                }
            }
            serde_json::Value::Object(spec)
        }
        None => value,
    };
    Ok(serde_json::to_string(&spec).expect("a JSON value serializes"))
}

/// One end of a relation rule: `"*"` is the any kind, any other string a
/// kind of the user's own.
fn end_kind(end: &str) -> Kind {
    if end == "*" {
        Kind::Any
    } else {
        Kind::named(end)
    }
}

/// The pair a relation value must be: from, then to.
fn ends_pair(name: &str, ends: &Bound<'_, PyAny>) -> PyResult<(Kind, Kind)> {
    let pair: Vec<String> = ends.extract().map_err(|_| {
        UsageError::new_err(format!(
            "the relation {name} takes a pair of ends, from then to; each is a kind or \"*\""
        ))
    })?;
    if pair.len() != 2 {
        return Err(UsageError::new_err(format!(
            "the relation {name} takes a pair of ends, from then to, and {} came",
            pair.len()
        )));
    }
    Ok((end_kind(&pair[0]), end_kind(&pair[1])))
}

/// Build a recognize ask from the ruled keyword shape: kinds as a list or
/// a question file path, relations as a name-to-pair mapping, and the two
/// bars. One parser in the core checks every rule; nothing is checked
/// again here.
fn build_recognize(
    py: Python<'_>,
    kinds: Option<&Bound<'_, PyAny>>,
    relations: Option<&Bound<'_, PyAny>>,
    threshold: Option<f64>,
    relation_threshold: Option<f64>,
) -> PyResult<Recognize> {
    let mut ask = match kinds {
        None => Recognize::new(),
        Some(bound) => {
            if let Ok(path) = bound.extract::<String>() {
                let spec = spec_from_file(&path, "recognize").map_err(|error| python_error(py, error))?;
                Recognize::from_json(&spec).map_err(|error| python_error(py, error))?
            } else {
                let names: Vec<String> = bound.extract().map_err(|_| {
                    UsageError::new_err("kinds takes a list of kind names or a question file path")
                })?;
                if names.is_empty() {
                    Recognize::new()
                } else {
                    Recognize::new().kinds(names)
                }
            }
        }
    };
    if let Some(bound) = relations {
        let dict = bound.cast::<PyDict>().map_err(|_| {
            UsageError::new_err(
                "relations takes a dictionary, one entry per rule: the name to a (from, to) pair",
            )
        })?;
        for (name, ends) in dict.iter() {
            let name: String = name
                .extract()
                .map_err(|_| UsageError::new_err("a relation name is a string"))?;
            let (from, to) = ends_pair(&name, &ends)?;
            let rule = RelationRule::new(&name, from, to).map_err(|error| python_error(py, error))?;
            ask.relations.push(rule);
        }
    }
    if let Some(value) = threshold {
        ask = ask.threshold(value).map_err(|error| python_error(py, error))?;
    }
    if let Some(value) = relation_threshold {
        ask = ask.relation_threshold(value).map_err(|error| python_error(py, error))?;
    }
    Ok(ask)
}

/// Build a relate ask from the ruled keyword shape: rules as bare names,
/// as name-to-pair mappings, or as a question file path; `either` names
/// the both-ways rules; one bar.
fn build_relate(
    py: Python<'_>,
    relations: Option<&Bound<'_, PyAny>>,
    either: Option<&Bound<'_, PyAny>>,
    threshold: Option<f64>,
) -> PyResult<Relate> {
    let mut ask = Relate::new();
    if let Some(bound) = relations {
        if let Ok(path) = bound.extract::<String>() {
            let spec = spec_from_file(&path, "relate").map_err(|error| python_error(py, error))?;
            ask = Relate::from_json(&spec).map_err(|error| python_error(py, error))?;
        } else {
            let entries = bound.cast::<pyo3::types::PyList>().map_err(|_| {
                UsageError::new_err(
                    "relations takes a list of names or name-to-pair mappings, or a question file path",
                )
            })?;
            for entry in entries.iter() {
                if let Ok(name) = entry.extract::<String>() {
                    let rule = RelationRule::new(&name, Kind::Any, Kind::Any)
                        .map_err(|error| python_error(py, error))?;
                    ask.relations.push(rule);
                } else if let Ok(dict) = entry.cast::<PyDict>() {
                    let held = |key: &str| -> PyResult<String> {
                        dict.get_item(key)?
                            .and_then(|value| value.extract::<String>().ok())
                            .ok_or_else(|| {
                                UsageError::new_err(format!(
                                    "a relation mapping needs its {key} as a string"
                                ))
                            })
                    };
                    let name = held("name")?;
                    let from = held("from")?;
                    let to = held("to")?;
                    let either = dict
                        .get_item("either")?
                        .and_then(|value| value.extract::<bool>().ok())
                        .unwrap_or(false);
                    let rule = RelationRule::new(&name, end_kind(&from), end_kind(&to))
                        .map_err(|error| python_error(py, error))?
                        .either(either);
                    ask.relations.push(rule);
                } else {
                    return Err(UsageError::new_err(
                        "a relation entry is a name or a mapping with name, from, to",
                    ));
                }
            }
        }
    }
    if let Some(bound) = either {
        let names: Vec<String> = bound.extract().map_err(|_| {
            UsageError::new_err("either takes a list of relation names")
        })?;
        for name in names {
            let rule = RelationRule::new(&name, Kind::Any, Kind::Any)
                .map_err(|error| python_error(py, error))?
                .either(true);
            ask.relations.push(rule);
        }
    }
    if let Some(value) = threshold {
        ask = ask.threshold(value).map_err(|error| python_error(py, error))?;
    }
    Ok(ask)
}

/// Carry the contract's recognized answer into this surface's records.
fn recognized_record(found: thinkthen_contract::Recognized) -> Recognized {
    Recognized {
        entities: found
            .entities
            .into_iter()
            .map(|entity| Entity {
                id: entity.id,
                text: entity.text,
                kind: entity.kind,
                start: entity.start as u64,
                end: entity.end as u64,
                number: entity.number,
            })
            .collect(),
        relations: found
            .relations
            .into_iter()
            .map(|relation| Relation {
                name: relation.name,
                source: relation.source,
                target: relation.target,
                probability: relation.probability,
            })
            .collect(),
    }
}

/// Carry one contract edge into this surface's record.
fn edge_record(edge: ContractEdge) -> Edge {
    Edge {
        name: edge.name,
        source: edge.source,
        target: edge.target,
        probability: edge.probability,
        source_kind: edge.source_kind,
        target_kind: edge.target_kind,
    }
}

/// Find every name in one text and say what kind it is. Relations are the
/// second answer, on when a rule is given. `kinds` is a list of the user's
/// own kind words or a path to a question file; `relations` is a mapping
/// of rule name to a (from, to) pair, each end a kind or the one-character
/// string `"*"`. Offsets count Python string positions, so
/// `text[start:end]` is the name.
#[pyfunction(signature = (text, *, kinds = None, relations = None, threshold = None, relation_threshold = None, deadline = None))]
#[allow(clippy::too_many_arguments)]
fn recognize(
    py: Python<'_>,
    text: String,
    kinds: Option<&Bound<'_, PyAny>>,
    relations: Option<&Bound<'_, PyAny>>,
    threshold: Option<f64>,
    relation_threshold: Option<f64>,
    deadline: Option<f64>,
) -> PyResult<Recognized> {
    let asked = build_recognize(py, kinds, relations, threshold, relation_threshold)?;
    let found = py
        .detach(move || engine().recognize_opts(&asked, &text, call_options(deadline)))
        .map_err(|error| python_error(py, error))?;
    Ok(recognized_record(found))
}

/// `recognize` over a frame's `on` column: one row per name, with the
/// source row's number counted from 1, in a long frame this surface
/// builds whole. No relation rules here; ask them of the text form.
#[pyfunction(signature = (records, on, *, kinds = None, threshold = None, relation_threshold = None, deadline = None))]
fn recognize_stream(
    py: Python<'_>,
    records: &Bound<'_, PyAny>,
    on: String,
    kinds: Option<&Bound<'_, PyAny>>,
    threshold: Option<f64>,
    relation_threshold: Option<f64>,
    deadline: Option<f64>,
) -> PyResult<arrow::ArrowFrame> {
    let asked = build_recognize(py, kinds, None, threshold, relation_threshold)?;
    let frame = arrow::frame_column(records, &on)?;
    let references: Vec<&str> = frame.texts.clone();
    let rows = bulk(py, deadline, |armed, mut poll| {
        let mut row_col: Vec<i64> = Vec::new();
        let mut text_col: Vec<String> = Vec::new();
        let mut kind_col: Vec<String> = Vec::new();
        let mut start_col: Vec<i64> = Vec::new();
        let mut end_col: Vec<i64> = Vec::new();
        let mut number_col: Vec<f64> = Vec::new();
        for (place, text) in references.iter().enumerate() {
            if let Some(poll) = poll.as_mut() {
                poll();
            }
            let found = engine().recognize_opts(&asked, text, armed)?;
            for entity in found.entities {
                row_col.push(place as i64 + 1);
                text_col.push(entity.text);
                kind_col.push(entity.kind);
                start_col.push(entity.start as i64);
                end_col.push(entity.end as i64);
                number_col.push(entity.number);
            }
        }
        Ok((row_col, text_col, kind_col, start_col, end_col, number_col))
    })?;
    drop(frame);
    let (row_col, text_col, kind_col, start_col, end_col, number_col) = rows;
    let table = arrow::build_table(&[
        ("row", arrow::TableValue::Counts(row_col)),
        ("text", arrow::TableValue::Texts(text_col)),
        ("kind", arrow::TableValue::Texts(kind_col)),
        ("start", arrow::TableValue::Counts(start_col)),
        ("end", arrow::TableValue::Counts(end_col)),
        ("number", arrow::TableValue::Numbers(number_col)),
    ])?;
    Ok(arrow::ArrowFrame::new(table))
}

/// Say how the records relate to each other: one pick-one question per
/// legal pair, every record crossing at once. More than 255 records is a
/// usage error before anything happens. `relations` is a list of names or
/// name-to-pair mappings, or a question file path; `either` names the
/// both-ways rules.
#[pyfunction(signature = (records, *, relations = None, either = None, threshold = None, deadline = None))]
#[allow(clippy::needless_pass_by_value)]
fn relate(
    py: Python<'_>,
    records: Vec<String>,
    relations: Option<&Bound<'_, PyAny>>,
    either: Option<&Bound<'_, PyAny>>,
    threshold: Option<f64>,
    deadline: Option<f64>,
) -> PyResult<Vec<Edge>> {
    let asked = build_relate(py, relations, either, threshold)?;
    let references: Vec<&str> = records.iter().map(String::as_str).collect();
    let edges = bulk(py, deadline, |armed, mut poll| {
        if let Some(poll) = poll.as_mut() {
            poll();
        }
        thinkthen_contract::relate_checked(engine(), &asked, &references, armed)
    })?;
    Ok(edges.into_iter().map(edge_record).collect())
}

/// `relate` over a frame's `on` column: a frame of edges, one row per
/// edge, the record numbers counted from 1 in input order.
#[pyfunction(signature = (records, on, *, relations = None, either = None, threshold = None, deadline = None))]
fn relate_stream(
    py: Python<'_>,
    records: &Bound<'_, PyAny>,
    on: String,
    relations: Option<&Bound<'_, PyAny>>,
    either: Option<&Bound<'_, PyAny>>,
    threshold: Option<f64>,
    deadline: Option<f64>,
) -> PyResult<arrow::ArrowFrame> {
    let asked = build_relate(py, relations, either, threshold)?;
    let frame = arrow::frame_column(records, &on)?;
    let references: Vec<&str> = frame.texts.clone();
    let edges = bulk(py, deadline, |armed, mut poll| {
        if let Some(poll) = poll.as_mut() {
            poll();
        }
        thinkthen_contract::relate_checked(engine(), &asked, &references, armed)
    })?;
    drop(frame);
    let mut name_col: Vec<String> = Vec::with_capacity(edges.len());
    let mut source_col: Vec<i64> = Vec::with_capacity(edges.len());
    let mut target_col: Vec<i64> = Vec::with_capacity(edges.len());
    let mut probability_col: Vec<f64> = Vec::with_capacity(edges.len());
    for edge in edges {
        name_col.push(edge.name);
        source_col.push(edge.source as i64);
        target_col.push(edge.target as i64);
        probability_col.push(edge.probability);
    }
    let table = arrow::build_table(&[
        ("name", arrow::TableValue::Texts(name_col)),
        ("source", arrow::TableValue::Counts(source_col)),
        ("target", arrow::TableValue::Counts(target_col)),
        ("probability", arrow::TableValue::Numbers(probability_col)),
    ])?;
    Ok(arrow::ArrowFrame::new(table))
}

/// The counters since the last reset. `requests` counts sends, so a
/// retried send shows twice, the same as the bill.
#[pyfunction]
fn usage(py: Python<'_>) -> Py<PyAny> {
    let counted = engine().usage();
    let dict = PyDict::new(py);
    let _ = dict.set_item("requests", counted.requests);
    let _ = dict.set_item("cache_answers", counted.cache_answers);
    let _ = dict.set_item("tokens", counted.tokens);
    dict.into_any().unbind()
}

/// Zero the counters.
#[pyfunction]
fn reset_usage() {
    thinkthen_standin::reset_usage();
}

#[pymodule]
fn _thinkthen(module: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = module.py();
    module.add_class::<Question>()?;
    module.add_class::<QuestionSetHolder>()?;
    module.add_class::<Entity>()?;
    module.add_class::<Relation>()?;
    module.add_class::<Recognized>()?;
    module.add_class::<Edge>()?;
    module.add_class::<arrow::ArrowFrame>()?;
    module.add_function(wrap_pyfunction!(arrow::_arrow_probe, module)?)?;
    module.add_function(wrap_pyfunction!(annotate_stream, module)?)?;
    module.add_function(wrap_pyfunction!(recognize_stream, module)?)?;
    module.add_function(wrap_pyfunction!(relate_stream, module)?)?;
    generated::register(module)?;
    module.add("ThinkThenError", py.get_type::<ThinkThenError>())?;
    module.add("UsageError", py.get_type::<UsageError>())?;
    module.add("BackendError", py.get_type::<BackendError>())?;
    module.add("DeadlineError", py.get_type::<DeadlineError>())?;
    module.add("LocalError", py.get_type::<LocalError>())?;
    module.add("Cancelled", py.get_type::<Cancelled>())?;
    module.add("DefectError", py.get_type::<DefectError>())?;
    Ok(())
}

/// A conversion failure pyo3 itself reports, as a defect of this shim.
fn python_error_of(error: pyo3::PyErr) -> PyErr {
    error
}
