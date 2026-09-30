//! The engine value. `tt.Engine` wraps this class, and the module functions
//! call the same methods on one value over the process engine (amendment
//! change 11). Each method reads its arguments here, then runs its call on
//! the detachable worker. The package names the verb, so one method serves
//! each shape: one text, many texts, an ordering, and the three set calls.

use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict};
use thinkthen::{Annotated, Answer, Batch, Error, FailureCause, Judgment};

use crate::asked::{Edge, Question, QuestionSet, Recognize, Recognized, Relate};
use crate::frame::ask_column;
use crate::input::{controls, entities, is_column, text, texts};
use crate::result::{self, Completed};
use crate::tally::PyTally;
use crate::worker::{Token, run_observed, run_tallied};
use crate::{guard, raised, usage};

mod operations;
mod plan;
mod settings;
pub(crate) use operations::only;
pub(crate) use plan::named as plan_named;

use operations::{Many, labels, many, of_kind, order};

use settings::{Settings, checked_throttle, folder_path, setting, total};
pub(crate) use settings::{batch, context};

const MAX_REQUESTS: &str = "a request limit is a whole number of 1 or more";
const MAX_REQUESTS_TOTAL: &str = "max_requests_total is a whole number of 0 or more";

pub(crate) type Arg<'a, 'py> = Option<&'a Bound<'py, PyAny>>;
pub(crate) type Held<'a, 'py> = Option<&'a Bound<'py, Token>>;

pub(crate) fn answer(py: Python<'_>, answer: Answer) -> Py<PyAny> {
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

fn detail_value(py: Python<'_>, verb: &str, found: thinkthen::Details) -> PyResult<Py<PyAny>> {
    if verb == "details" {
        Ok(found.to_json().into_pyobject(py)?.into_any().unbind())
    } else {
        judgment(py, found.value().clone())
    }
}

/// A failed annotate question's cause, as the shared cases spell it.
pub(crate) const fn cause(cause: FailureCause) -> &'static str {
    match cause {
        FailureCause::MissingAnswer => "missing_answer",
        FailureCause::WrongKind => "wrong_kind",
        FailureCause::MissingProbability => "missing_probability",
        FailureCause::InvalidProbability => "invalid_probability",
        FailureCause::InvalidDistribution => "invalid_distribution",
        FailureCause::UnexpectedProbability => "unexpected_probability",
    }
}

pub(crate) fn annotated(py: Python<'_>, value: Annotated) -> PyResult<Py<PyAny>> {
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

#[expect(
    clippy::expect_used,
    reason = "an exhausted successful batch always fixes its facts"
)]
pub(crate) fn collected<T>(mut batch: Batch<'_, T>) -> Result<Completed<Vec<T>>, Error> {
    let rows = batch.by_ref().collect::<Result<Vec<_>, _>>()?;
    Ok(Completed::new(
        rows,
        batch.facts().expect("successful batch facts"),
    ))
}

/// The engine behind `tt.Engine` and the module functions.
#[pyclass(frozen, name = "_Engine", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct Engine(pub(crate) thinkthen::Engine);

#[pymethods]
impl Engine {
    /// Start from what `thinkthen` reads from the environment, then apply
    /// each given setting (amendment changes 11 to 13).
    #[new]
    #[pyo3(signature = (*, base_url=None, model=None, throttle=None, batch=None, max_requests=None, max_requests_total=None, max_request_bytes=None, cache=None, timeout=None, max_retries=None, record=None, replay=None, profile=None))]
    #[expect(
        clippy::too_many_arguments,
        reason = "PyO3's keyword-only constructor exposes the engine settings"
    )]
    fn new(
        base_url: Option<&str>,
        model: Option<&str>,
        throttle: Arg<'_, '_>,
        batch: Arg<'_, '_>,
        max_requests: Arg<'_, '_>,
        max_requests_total: Arg<'_, '_>,
        max_request_bytes: Arg<'_, '_>,
        cache: Arg<'_, '_>,
        timeout: Arg<'_, '_>,
        max_retries: Arg<'_, '_>,
        record: Arg<'_, '_>,
        replay: Arg<'_, '_>,
        profile: Arg<'_, '_>,
    ) -> PyResult<Self> {
        Python::attach(|py| {
            let read = || -> PyResult<Self> {
                let settings = Settings {
                    base_url,
                    model,
                    throttle: checked_throttle(throttle)?,
                    batch: self::batch(batch)?,
                    most: setting(max_requests, MAX_REQUESTS)?,
                    most_total: total(max_requests_total)?,
                    max_request_bytes: setting(
                        max_request_bytes,
                        "max_request_bytes is a whole number",
                    )?,
                    timeout: setting(timeout, "a timeout is a whole number of seconds above zero")?,
                    retries: setting(max_retries, "max_retries is a whole number")?,
                    record: folder_path(py, record, "record is a folder path")?,
                    replay: folder_path(py, replay, "replay is a folder path")?,
                    profile: folder_path(py, profile, "profile is a file path")?,
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
    #[pyo3(signature = (verb, question, evidence, deadline, token, batch=None, context=None, tally=None))]
    #[expect(
        clippy::too_many_arguments,
        reason = "PyO3 accepts column controls beside scalar arguments"
    )]
    fn ask(
        &self,
        verb: &str,
        question: &Bound<'_, Question>,
        evidence: &Bound<'_, PyAny>,
        deadline: Arg<'_, '_>,
        token: Held<'_, '_>,
        batch: Arg<'_, '_>,
        context: Arg<'_, '_>,
        tally: Option<&Bound<'_, PyTally>>,
    ) -> PyResult<Py<PyAny>> {
        let py = question.py();
        guard(py, || {
            let (engine, asked) = (self.0.clone(), question.get().0.clone());
            if verb != "details" {
                of_kind(py, &asked, verb, verb)?;
            }
            if is_column(evidence)? {
                let batch = self::batch(batch)?;
                let context = self::context(context)?;
                return ask_column(
                    &engine,
                    verb,
                    &asked,
                    evidence,
                    batch,
                    context,
                    deadline,
                    token,
                    tally.map(|one| one.get().0.clone()),
                );
            }
            if batch.is_some() || context.is_some() {
                return Err(usage(py, "one text does not take batch or shared context"));
            }
            let (evidence, controls) = (text(evidence)?, controls(py, deadline, token)?);
            let tallied = tally.map(|one| one.get().0.clone());
            let found = run_tallied(py, controls, tallied, move |options| {
                let found = engine.details_with(asked.detail(), &evidence, options)?;
                Ok::<_, Error>(Completed::new(found.value().clone(), found.facts()))
            })?;
            result::converted(py, found, |found| detail_value(py, verb, found))
        })
    }

    /// `decide_many` or `filter` over every record.
    #[pyo3(signature = (verb, question, records, batch, context, deadline, token, tally=None))]
    #[expect(
        clippy::too_many_arguments,
        reason = "PyO3 mirrors the public controls on a many call"
    )]
    fn many(
        &self,
        verb: &str,
        question: &Bound<'_, Question>,
        records: &Bound<'_, PyAny>,
        batch: Arg<'_, '_>,
        context: Arg<'_, '_>,
        deadline: Arg<'_, '_>,
        token: Held<'_, '_>,
        tally: Option<&Bound<'_, PyTally>>,
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
            if cut.is_none() && is_column(records)? {
                let batch = self::batch(batch)?;
                let context = self::context(context)?;
                return ask_column(
                    &engine,
                    "decide",
                    &asked,
                    records,
                    batch,
                    context,
                    deadline,
                    token,
                    tally.map(|one| one.get().0.clone()),
                );
            }
            let (batch, context) = (self::batch(batch)?, self::context(context)?);
            let (records, controls) = (texts(records)?, controls(py, deadline, token)?);
            let tallied = tally.map(|one| one.get().0.clone());
            let done = run_tallied(py, controls, tallied, move |options| {
                let options = batch.map_or(options, |batch| options.batch(batch));
                let options = context
                    .as_deref()
                    .map_or(options, |context| options.context(context));
                many(&engine, cut.as_ref(), &asked, records, options)
            })?;
            result::converted(py, done, |done| match done {
                Many::Kept(passed) => Ok(passed.into_pyobject(py)?.unbind()),
                Many::Answers(answers) => {
                    let answers: Vec<Py<PyAny>> =
                        answers.into_iter().map(|one| answer(py, one)).collect();
                    Ok(answers.into_pyobject(py)?.unbind())
                }
                Many::Judgments(values) => values
                    .into_iter()
                    .map(|value| judgment(py, value))
                    .collect::<PyResult<Vec<_>>>()?
                    .into_pyobject(py)
                    .map(|value| value.unbind()),
            })
        })
    }

    /// Runtime-label choose, score and tag over the shared details planner.
    #[pyo3(signature = (verb, question, records, batch, context, deadline, token, tally=None))]
    #[expect(
        clippy::too_many_arguments,
        reason = "PyO3 mirrors the public controls on a many call"
    )]
    fn label_many(
        &self,
        verb: &str,
        question: &Bound<'_, Question>,
        records: &Bound<'_, PyAny>,
        batch: Arg<'_, '_>,
        context: Arg<'_, '_>,
        deadline: Arg<'_, '_>,
        token: Held<'_, '_>,
        tally: Option<&Bound<'_, PyTally>>,
    ) -> PyResult<Py<PyAny>> {
        let py = question.py();
        guard(py, || {
            let (engine, asked) = (self.0.clone(), question.get().0.clone());
            of_kind(py, &asked, verb, verb)?;
            let (batch, context) = (self::batch(batch)?, self::context(context)?);
            let (records, controls) = (texts(records)?, controls(py, deadline, token)?);
            let tallied = tally.map(|one| one.get().0.clone());
            let done = run_tallied(py, controls, tallied, move |options| {
                let options = batch.map_or(options, |batch| options.batch(batch));
                let options = context
                    .as_deref()
                    .map_or(options, |context| options.context(context));
                labels(&engine, &asked, records, options)
            })?;
            result::converted(py, done, |done| match done {
                Many::Judgments(values) => values
                    .into_iter()
                    .map(|value| judgment(py, value))
                    .collect::<PyResult<Vec<_>>>()?
                    .into_pyobject(py)
                    .map(|value| value.unbind()),
                _ => Err(usage(py, "the question did not produce labels")),
            })
        })
    }

    /// `rank` gives each record's place, text, and probability, most likely
    /// first. `find` gives the selected unit's alone, or nothing.
    #[expect(
        clippy::too_many_arguments,
        reason = "PyO3 mirrors the public controls on an ordering call"
    )]
    fn order(
        &self,
        verb: &str,
        question: &Bound<'_, Question>,
        records: &Bound<'_, PyAny>,
        batch: Arg<'_, '_>,
        context: Arg<'_, '_>,
        deadline: Arg<'_, '_>,
        token: Held<'_, '_>,
    ) -> PyResult<Py<PyAny>> {
        let py = question.py();
        guard(py, || {
            let (engine, asked) = (self.0.clone(), only(py, &question.get().0, verb, verb)?);
            if verb == "find" && context.is_some() {
                return Err(usage(py, "find does not take a shared context"));
            }
            let (batch, context) = (self::batch(batch)?, self::context(context)?);
            let (records, controls) = (texts(records)?, controls(py, deadline, token)?);
            let find = verb == "find";
            let done = run_observed(py, controls, move |options| {
                let options = batch.map_or(options, |batch| options.batch(batch));
                let options = context
                    .as_deref()
                    .map_or(options, |context| options.context(context));
                order(&engine, find, &asked, records, options)
            })?;
            result::converted(py, done, |value| Ok(value.into_pyobject(py)?.unbind()))
        })
    }

    /// One dictionary per record, in the set's order.
    #[expect(
        clippy::too_many_arguments,
        reason = "PyO3 mirrors the public controls on annotation"
    )]
    fn annotate(
        &self,
        py: Python<'_>,
        questions: &Bound<'_, QuestionSet>,
        records: &Bound<'_, PyAny>,
        batch: Arg<'_, '_>,
        deadline: Arg<'_, '_>,
        token: Held<'_, '_>,
    ) -> PyResult<Py<PyAny>> {
        guard(py, || {
            let (engine, set) = (self.0.clone(), questions.get().0.clone());
            let batch = self::batch(batch)?;
            let (records, controls) = (texts(records)?, controls(py, deadline, token)?);
            let rows = run_observed(py, controls, move |options| {
                let options = batch.map_or(options, |batch| options.batch(batch));
                collected(engine.annotate_with(&set, records, options))
                    .map(|done| done.map(|rows| rows.iter().map(|row| row.value_json()).collect()))
            })?;
            result::converted(py, rows, |rows: Vec<String>| {
                let json = py.import("json")?;
                rows.iter()
                    .map(|row| json.call_method1("loads", (row,)).map(Bound::unbind))
                    .collect::<PyResult<Vec<_>>>()?
                    .into_pyobject(py)
                    .map(|value| value.unbind())
            })
        })
    }

    fn recognize(
        &self,
        py: Python<'_>,
        ask: &Bound<'_, Recognize>,
        evidence: &Bound<'_, PyAny>,
        deadline: Arg<'_, '_>,
        token: Held<'_, '_>,
    ) -> PyResult<Py<PyAny>> {
        guard(py, || {
            let (engine, ask) = (self.0.clone(), ask.get().0.clone());
            let (evidence, controls) = (text(evidence)?, controls(py, deadline, token)?);
            let found = run_observed(py, controls, move |options| {
                let found = engine.recognize_with(&ask, &evidence, options)?;
                Ok::<_, Error>(Completed::new(
                    Recognized::from(found.value()),
                    found.facts(),
                ))
            })?;
            result::converted(py, found, |found| Ok(Py::new(py, found)?.into_any()))
        })
    }

    fn relate(
        &self,
        py: Python<'_>,
        ask: &Bound<'_, Relate>,
        given: &Bound<'_, PyAny>,
        deadline: Arg<'_, '_>,
        token: Held<'_, '_>,
    ) -> PyResult<Py<PyAny>> {
        guard(py, || {
            let (engine, ask) = (self.0.clone(), ask.get().0.clone());
            let (given, controls) = (entities(given)?, controls(py, deadline, token)?);
            let edges = run_observed(py, controls, move |options| {
                let edges = engine.relate_with(&ask, given, options)?;
                Ok::<_, Error>(Completed::new(
                    edges.value().iter().map(Edge::from).collect::<Vec<_>>(),
                    edges.facts(),
                ))
            })?;
            result::converted(py, edges, |edges| Ok(edges.into_pyobject(py)?.unbind()))
        })
    }

    /// This engine's totals.
    fn usage<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let counts = self.0.usage();
        let totals = PyDict::new(py);
        totals.set_item("requests_sent", counts.requests_sent())?;
        totals.set_item("retries", counts.retries())?;
        totals.set_item("cache_answers", counts.cache_answers())?;
        totals.set_item("input_tokens", counts.input_tokens())?;
        totals.set_item("output_tokens", counts.output_tokens())?;
        Ok(totals)
    }

    /// The public core planner over a complete, validated input.
    fn plan<'py>(
        &self,
        py: Python<'py>,
        question: &Bound<'_, Question>,
        records: &Bound<'_, PyAny>,
        batch: Arg<'_, '_>,
        context: Arg<'_, '_>,
    ) -> PyResult<Bound<'py, PyDict>> {
        if is_column(records)? {
            crate::frame::plan_column(self, py, question, records, batch, context)
        } else {
            plan::estimate(self, py, question, records, batch, context)
        }
    }

    /// Prepare one caller-fed native core Batch for a Python iterator.
    #[pyo3(signature = (verb, question, source, batch, context, deadline, token, tally=None))]
    #[expect(
        clippy::too_many_arguments,
        reason = "one stream carries its public controls"
    )]
    fn _stream(
        &self,
        py: Python<'_>,
        verb: &str,
        question: &Bound<'_, Question>,
        source: Py<PyAny>,
        batch: Arg<'_, '_>,
        context: Arg<'_, '_>,
        deadline: Arg<'_, '_>,
        token: Held<'_, '_>,
        tally: Option<&Bound<'_, PyTally>>,
    ) -> PyResult<Py<crate::stream::PyStream>> {
        guard(py, || {
            crate::stream::prepare(
                py, self, verb, question, source, batch, context, deadline, token, tally,
            )
        })
    }
}
