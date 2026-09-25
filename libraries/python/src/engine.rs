//! The engine value. `tt.Engine` wraps this class, and the module functions
//! call the same methods on one value over the process engine (amendment
//! change 11). Each method reads its arguments here, then runs its call on
//! the detachable worker. The package names the verb, so one method serves
//! each shape: one text, many texts, an ordering, and the three set calls.

use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict};
use thinkthen::{
    Annotated, Answer, CallOptions, EngineBuilder, Error, Evidence, FailureCause, Judgment,
};

use crate::asked::{Asked, Edge, Question, QuestionSet, Recognize, Recognized, Relate};
use crate::input::{controls, entities, text, texts, whole};
use crate::worker::{Token, run};
use crate::{guard, raised, usage};

const THROTTLE: &str = "a throttle is a whole number from 1 through 32";
const MAX_REQUESTS: &str = "a request limit is a whole number of 1 or more";
const CACHE_BYTES: &str = "a cache cap is a whole number of bytes above zero";
const CACHE: &str = "cache is a folder path, False for no cache, or True for the default folder";

type Arg<'a, 'py> = Option<&'a Bound<'py, PyAny>>;
type Held<'a, 'py> = Option<&'a Bound<'py, Token>>;

/// One record and its place in the caller's list.
#[derive(Debug)]
struct Indexed(usize, String);

impl Evidence for Indexed {
    fn evidence(&self) -> &str {
        &self.1
    }
}

fn answer(py: Python<'_>, answer: Answer) -> Py<PyAny> {
    match answer {
        Answer::Yes => PyBool::new(py, true).to_owned().into_any().unbind(),
        Answer::No => PyBool::new(py, false).to_owned().into_any().unbind(),
        Answer::Unsure => py.None(),
    }
}

fn judgment(py: Python<'_>, value: Judgment) -> PyResult<Py<PyAny>> {
    Ok(match value {
        Judgment::Decision(held) => answer(py, held),
        Judgment::Choice(pick) => pick.into_pyobject(py)?.unbind(),
        Judgment::Score(position) => position.into_pyobject(py)?.into_any().unbind(),
        Judgment::Tags(labels) => labels.into_pyobject(py)?.unbind(),
    })
}

/// A failed annotate question's cause, as the shared cases spell it.
const fn cause(cause: FailureCause) -> &'static str {
    match cause {
        FailureCause::MissingAnswer => "missing_answer",
        FailureCause::WrongKind => "wrong_kind",
        FailureCause::MissingProbability => "missing_probability",
        FailureCause::InvalidProbability => "invalid_probability",
        FailureCause::InvalidDistribution => "invalid_distribution",
        FailureCause::UnexpectedProbability => "unexpected_probability",
    }
}

fn annotated(py: Python<'_>, value: Annotated) -> PyResult<Py<PyAny>> {
    judgment(
        py,
        match value {
            Annotated::Decision(held) => Judgment::Decision(held),
            Annotated::Choice(pick) => Judgment::Choice(pick),
            Annotated::Score(position) => Judgment::Score(position),
            Annotated::Tags(labels) => Judgment::Tags(labels),
            Annotated::Failed(failed) => {
                let marker = PyDict::new(py);
                marker.set_item("kind", failed.kind().name())?;
                marker.set_item("cause", cause(failed.cause()))?;
                let outer = PyDict::new(py);
                outer.set_item("failed", marker)?;
                return Ok(outer.into_any().unbind());
            }
        },
    )
}

/// Refuse a question whose kind is not `kind`, naming the verb.
fn of_kind(py: Python<'_>, asked: &Asked, verb: &str, kind: &str) -> PyResult<()> {
    if asked.kind() == kind {
        return Ok(());
    }
    Err(usage(
        py,
        &format!("{verb} does not take a {} question", asked.kind()),
    ))
}

/// The question a verb takes: its own kind, under one cut.
fn only(py: Python<'_>, asked: &Asked, verb: &str, kind: &str) -> PyResult<thinkthen::Question> {
    of_kind(py, asked, verb, kind)?;
    match asked {
        Asked::Plain(question) => Ok(question.clone()),
        Asked::Banded(_) => Err(usage(py, "this call takes one cut, not a band")),
    }
}

/// How the engine caches: `True` for the default folder, `False` for none,
/// or a folder path.
fn cached(builder: EngineBuilder, cache: &Bound<'_, PyAny>) -> PyResult<EngineBuilder> {
    let py = cache.py();
    if let Ok(on) = cache.cast::<PyBool>() {
        return Ok(if on.is_true() {
            builder.default_cache()
        } else {
            builder.no_cache()
        });
    }
    let folder: std::path::PathBuf = cache.extract().map_err(|_| usage(py, CACHE))?;
    builder.cache_at(folder).map_err(|error| raised(py, &error))
}

/// A whole-number setting in the range its type holds, or its sentence.
fn setting<T: TryFrom<i64>>(
    value: Option<&Bound<'_, PyAny>>,
    sentence: &str,
) -> PyResult<Option<T>> {
    value
        .map(|value| T::try_from(whole(value, sentence)?).map_err(|_| usage(value.py(), sentence)))
        .transpose()
}

/// A record's place, text, and probability.
type Placed = (usize, String, f64);

/// The throttle, checked here so `throttle=300` is a `UsageError`, not an
/// `OverflowError` (amendment change 13).
fn checked_throttle(value: Arg<'_, '_>) -> PyResult<Option<u8>> {
    match (value, setting::<u8>(value, THROTTLE)?) {
        (Some(value), Some(read)) if !(1..=32).contains(&read) => Err(usage(value.py(), THROTTLE)),
        (_, read) => Ok(read),
    }
}

/// The checked settings of `tt.Engine`, each applied over the environment.
struct Settings<'a> {
    base_url: Option<&'a str>,
    model: Option<&'a str>,
    throttle: Option<u8>,
    most: Option<usize>,
    cap: Option<u64>,
}

impl Settings<'_> {
    fn build(self, py: Python<'_>, cache: Arg<'_, '_>) -> PyResult<thinkthen::Engine> {
        let refused = |error: thinkthen::Error| raised(py, &error);
        let mut builder = EngineBuilder::from_env().map_err(refused)?;
        if let Some(address) = self.base_url {
            builder = builder.base_url(address).map_err(refused)?;
        }
        if let Some(name) = self.model {
            builder = builder.model(name).map_err(refused)?;
        }
        if self.most.is_some() {
            builder = builder.max_requests(self.most).map_err(refused)?;
        }
        if let Some(cap) = self.cap {
            builder = builder.cache_bytes(cap).map_err(refused)?;
        }
        if let Some(cache) = cache {
            builder = cached(builder, cache)?;
        }
        if let Some(throttle) = self.throttle {
            builder = builder.throttle(throttle).map_err(refused)?;
        }
        builder.build().map_err(refused)
    }
}

/// What `many` gives back from the worker.
enum Many {
    Kept(Vec<String>),
    Answers(Vec<Answer>),
}

/// The worker's side of `many`: `filter` under `cut`, or `decide_many`.
fn many(
    engine: &thinkthen::Engine,
    cut: Option<&thinkthen::Question>,
    asked: &Asked,
    records: Vec<String>,
    options: CallOptions<'_>,
) -> Result<Many, Error> {
    let value = |row: Result<thinkthen::Row<String, Answer>, Error>| row.map(|row| *row.value());
    match (cut, asked) {
        (Some(cut), _) => engine
            .filter_with(cut, records, options)
            .collect::<Result<_, _>>()
            .map(Many::Kept),
        (None, asked) => engine
            .decide_many_with(asked.decision(), records, options)
            .map(value)
            .collect::<Result<_, _>>()
            .map(Many::Answers),
    }
}

/// The worker's side of `order`: every ranked record, or the found unit.
fn order(
    engine: &thinkthen::Engine,
    find: bool,
    asked: &thinkthen::Question,
    records: Vec<String>,
    options: CallOptions<'_>,
) -> Result<Vec<Placed>, Error> {
    let records = records
        .into_iter()
        .enumerate()
        .map(|(at, text)| Indexed(at, text));
    if find {
        let found = engine.find_with(asked, records, options)?;
        let probability = |at: usize| {
            found
                .candidates()
                .get(at)
                .map_or(0.0, thinkthen::Candidate::probability)
        };
        return Ok(found
            .selected()
            .map(|Indexed(at, text)| (*at, text.clone(), probability(*at)))
            .into_iter()
            .collect());
    }
    let ranked = engine.rank_with(asked, records, options)?;
    Ok(ranked
        .into_iter()
        .map(|row| (row.probability(), row.into_input()))
        .map(|(probability, Indexed(at, text))| (at, text, probability))
        .collect())
}

/// One record's values, owned so they leave the worker.
fn named(record: thinkthen::AnnotatedRecord<String>) -> Vec<(String, Annotated)> {
    record
        .values()
        .iter()
        .map(|one| (one.name().to_owned(), one.value().clone()))
        .collect()
}

/// One record's dictionary, in the set's order.
fn row(py: Python<'_>, values: Vec<(String, Annotated)>) -> PyResult<Py<PyAny>> {
    let row = PyDict::new(py);
    for (name, value) in values {
        row.set_item(name, annotated(py, value)?)?;
    }
    Ok(row.into_any().unbind())
}

/// The engine behind `tt.Engine` and the module functions.
#[pyclass(frozen, name = "_Engine", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct Engine(thinkthen::Engine);

#[pymethods]
impl Engine {
    /// Start from what `thinkthen` reads from the environment, then apply
    /// each given setting (amendment changes 11 to 13).
    #[new]
    #[pyo3(signature = (*, base_url=None, model=None, throttle=None, max_requests=None, cache=None, cache_bytes=None))]
    fn new(
        base_url: Option<&str>,
        model: Option<&str>,
        throttle: Arg<'_, '_>,
        max_requests: Arg<'_, '_>,
        cache: Arg<'_, '_>,
        cache_bytes: Arg<'_, '_>,
    ) -> PyResult<Self> {
        Python::attach(|py| {
            let read = || -> PyResult<Self> {
                let settings = Settings {
                    base_url,
                    model,
                    throttle: checked_throttle(throttle)?,
                    most: setting(max_requests, MAX_REQUESTS)?,
                    cap: setting(cache_bytes, CACHE_BYTES)?,
                };
                settings.build(py, cache).map(Self)
            };
            guard(py, read)
        })
    }

    /// The process engine, built once from the environment.
    #[staticmethod]
    fn _process(py: Python<'_>) -> PyResult<Self> {
        guard(py, || {
            thinkthen::default_engine()
                .map(|engine| Self(engine.clone()))
                .map_err(|error| raised(py, &error))
        })
    }

    /// `decide`, `choose`, `score`, `tag`, or `details` over one text. Each
    /// reads the one `details` call, so a question with runtime labels adds
    /// no send (decision 8). `details` gives the document as JSON text.
    fn ask(
        &self,
        verb: &str,
        question: &Bound<'_, Question>,
        evidence: &Bound<'_, PyAny>,
        deadline: Arg<'_, '_>,
        token: Held<'_, '_>,
    ) -> PyResult<Py<PyAny>> {
        let py = question.py();
        guard(py, || {
            let (engine, asked) = (self.0.clone(), question.get().0.clone());
            if verb != "details" {
                of_kind(py, &asked, verb, verb)?;
            }
            let (evidence, controls) = (text(evidence)?, controls(py, deadline, token)?);
            let found = run(py, controls, move |options| {
                engine.details_with(asked.detail(), &evidence, options)
            })?;
            if verb == "details" {
                return Ok(found.to_json().into_pyobject(py)?.into_any().unbind());
            }
            judgment(py, found.value().clone())
        })
    }

    /// `decide_many` or `filter` over every record.
    fn many(
        &self,
        verb: &str,
        question: &Bound<'_, Question>,
        records: &Bound<'_, PyAny>,
        deadline: Arg<'_, '_>,
        token: Held<'_, '_>,
    ) -> PyResult<Py<PyAny>> {
        let py = question.py();
        guard(py, || {
            let (engine, asked) = (self.0.clone(), question.get().0.clone());
            let cut = (verb == "filter")
                .then(|| only(py, &asked, verb, "decide"))
                .transpose()?;
            if asked.kind() != "decide" {
                return Err(usage(
                    py,
                    &format!("{verb} does not take a {} question", asked.kind()),
                ));
            }
            let (records, controls) = (texts(records)?, controls(py, deadline, token)?);
            let done = run(py, controls, move |options| {
                many(&engine, cut.as_ref(), &asked, records, options)
            })?;
            match done {
                Many::Kept(passed) => Ok(passed.into_pyobject(py)?.unbind()),
                Many::Answers(answers) => {
                    let answers: Vec<Py<PyAny>> =
                        answers.into_iter().map(|one| answer(py, one)).collect();
                    Ok(answers.into_pyobject(py)?.unbind())
                }
            }
        })
    }

    /// `rank` gives each record's place, text, and probability, most likely
    /// first. `find` gives the selected unit's alone, or nothing.
    fn order(
        &self,
        verb: &str,
        question: &Bound<'_, Question>,
        records: &Bound<'_, PyAny>,
        deadline: Arg<'_, '_>,
        token: Held<'_, '_>,
    ) -> PyResult<Vec<Placed>> {
        let py = question.py();
        guard(py, || {
            let (engine, asked) = (self.0.clone(), only(py, &question.get().0, verb, verb)?);
            let (records, controls) = (texts(records)?, controls(py, deadline, token)?);
            let find = verb == "find";
            run(py, controls, move |options| {
                order(&engine, find, &asked, records, options)
            })
        })
    }

    /// One dictionary per record, in the set's order.
    fn annotate(
        &self,
        py: Python<'_>,
        questions: &Bound<'_, QuestionSet>,
        records: &Bound<'_, PyAny>,
        deadline: Arg<'_, '_>,
        token: Held<'_, '_>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        guard(py, || {
            let (engine, set) = (self.0.clone(), questions.get().0.clone());
            let (records, controls) = (texts(records)?, controls(py, deadline, token)?);
            let rows = run(py, controls, move |options| {
                engine
                    .annotate_with(&set, records, options)
                    .map(|record| record.map(named))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rows.into_iter().map(|values| row(py, values)).collect()
        })
    }

    fn recognize(
        &self,
        py: Python<'_>,
        ask: &Bound<'_, Recognize>,
        evidence: &Bound<'_, PyAny>,
        deadline: Arg<'_, '_>,
        token: Held<'_, '_>,
    ) -> PyResult<Recognized> {
        guard(py, || {
            let (engine, ask) = (self.0.clone(), ask.get().0.clone());
            let (evidence, controls) = (text(evidence)?, controls(py, deadline, token)?);
            let found = run(py, controls, move |options| {
                engine.recognize_with(&ask, &evidence, options)
            })?;
            Ok(Recognized::from(&found))
        })
    }

    fn relate(
        &self,
        py: Python<'_>,
        ask: &Bound<'_, Relate>,
        given: &Bound<'_, PyAny>,
        deadline: Arg<'_, '_>,
        token: Held<'_, '_>,
    ) -> PyResult<Vec<Edge>> {
        guard(py, || {
            let (engine, ask) = (self.0.clone(), ask.get().0.clone());
            let (given, controls) = (entities(given)?, controls(py, deadline, token)?);
            let edges = run(py, controls, move |options| {
                engine.relate_with(&ask, given, options)
            })?;
            Ok(edges.iter().map(Edge::from).collect())
        })
    }

    /// This engine's totals.
    fn usage<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let counts = self.0.usage();
        let totals = PyDict::new(py);
        totals.set_item("requests_sent", counts.requests_sent())?;
        totals.set_item("cache_answers", counts.cache_answers())?;
        totals.set_item("input_tokens", counts.input_tokens())?;
        totals.set_item("output_tokens", counts.output_tokens())?;
        Ok(totals)
    }
}
