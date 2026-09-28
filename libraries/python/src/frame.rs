//! The column and frame glue (tickets 0106 and 0122). A column verb makes one
//! engine call: `decide` one `decide_many`, and `choose`, `score`, and `tag`
//! one `annotate` over a one-question set. The worker owns the imported
//! column, reads it in place, and releases it (amendment changes 1, 3, and 6).
//! A pandas answer comes back as Python values and a dtype name (ticket 0122).

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::value::RawValue;
use thinkthen::{
    Annotated, Answer, CallOptions, Error, QuestionKind, QuestionSet, RecognizedEntity,
};

use crate::arrow::{self, Arrow, Cells, Imported, Readable};
use crate::asked::{Asked, Recognize};
use crate::engine::{Arg, Engine, Held, annotated, answer};
use crate::input::{Pandas, controls, listed, polars_frame, top};
use crate::worker::{Controls, run};
use crate::{guard, raised, usage};

/// Why a worker's job stopped: the engine's error, or a refusal sentence.
#[derive(Debug)]
enum Stop {
    Engine(Error),
    Said(String),
}

impl From<Error> for Stop {
    fn from(error: Error) -> Self {
        Self::Engine(error)
    }
}

impl From<String> for Stop {
    fn from(sentence: String) -> Self {
        Self::Said(sentence)
    }
}

impl From<&'static str> for Stop {
    fn from(sentence: &'static str) -> Self {
        Self::Said(sentence.to_owned())
    }
}

/// Run `job` on the worker, which owns `held` and drops it before it answers.
fn on_worker<H, T, F>(py: Python<'_>, controls: Controls, held: H, job: F) -> PyResult<T>
where
    H: Send + 'static,
    T: Send + 'static,
    F: FnOnce(H, CallOptions<'_>) -> Result<T, Stop> + Send + 'static,
{
    run(py, controls, move |options| Ok(job(held, options)))?.map_err(|stop| match stop {
        Stop::Engine(error) => raised(py, &error),
        Stop::Said(sentence) => usage(py, &sentence),
    })
}

/// Where a column's texts come from: the Arrow door, or a pandas Series the
/// package marks for 0105's list reader (ticket 0122). `read` says if marked.
enum Source {
    Door(Imported),
    List(Vec<String>),
}

impl Source {
    fn read(value: &Bound<'_, PyAny>) -> PyResult<(Self, bool)> {
        let Ok(marked) = value.cast::<Pandas>() else {
            return Ok((Self::Door(Imported::column(value)?), false));
        };
        let source = match marked.get() {
            Pandas(inner, true) => Self::List(listed(inner.bind(value.py()))?),
            Pandas(inner, false) => Self::Door(Imported::column(inner.bind(value.py()))?),
        };
        Ok((source, true))
    }
}

/// Run `job` over a column's texts on the worker, which owns the source.
fn over_texts<T, F>(py: Python<'_>, controls: Controls, source: Source, job: F) -> PyResult<T>
where
    T: Send + 'static,
    F: FnOnce(&[&str], CallOptions<'_>) -> Result<T, Stop> + Send + 'static,
{
    on_worker(py, controls, source, move |source, options| match &source {
        Source::Door(held) => {
            let memory = Readable::snapshot()?;
            job(&arrow::series(held, &memory)?, options)
        }
        Source::List(all) => job(&all.iter().map(String::as_str).collect::<Vec<_>>(), options),
    })
}

/// What a column call gives back.
enum Answers {
    Decided(Vec<Answer>),
    Annotated(QuestionKind, Vec<Annotated>),
}

/// `decide`, `choose`, `score`, or `tag` over a column. A Polars `Series`
/// gets an answer column to rebuild, and a pandas Series its values and
/// dtype name. Another producer gets a list.
pub(crate) fn ask_column(
    engine: &thinkthen::Engine,
    verb: &str,
    asked: &Asked,
    value: &Bound<'_, PyAny>,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
) -> PyResult<Py<PyAny>> {
    let py = value.py();
    if verb == "details" {
        return Err(usage(py, "details reads one str, not a column"));
    }
    let controls = controls(py, deadline, token)?;
    let set = match asked {
        Asked::Plain(question) if asked.kind() != "decide" => Some(
            QuestionSet::builder()
                .question(verb, question.clone())
                .and_then(thinkthen::QuestionSetBuilder::build)
                .map_err(|error| raised(py, &error))?,
        ),
        _ => None,
    };
    let polars = top(value)? == "polars";
    let (engine, asked) = (engine.clone(), asked.clone());
    let (source, pandas) = Source::read(value)?;
    let done = over_texts(py, controls, source, move |texts, options| {
        let texts = texts.iter().copied();
        if let (Some(set), Asked::Plain(question)) = (&set, &asked) {
            let values = engine
                .annotate_with(set, texts, options)
                .map(|record| {
                    record.map(|one| one.values().first().map(|named| named.value().clone()))
                })
                .collect::<Result<Option<Vec<_>>, _>>()?
                .ok_or("the engine answered a record with no value")?;
            return Ok(Answers::Annotated(question.kind(), values));
        }
        let rows = engine.decide_many_with(asked.decision(), texts, options);
        Ok(Answers::Decided(
            rows.map(|row| row.map(|row| *row.value()))
                .collect::<Result<_, _>>()?,
        ))
    })?;
    if !polars && !pandas {
        let values: Vec<Py<PyAny>> = match done {
            Answers::Decided(values) => values.into_iter().map(|one| answer(py, one)).collect(),
            Answers::Annotated(_, values) => values
                .into_iter()
                .map(|one| annotated(py, one))
                .collect::<PyResult<_>>()?,
        };
        return Ok(values.into_pyobject(py)?.unbind());
    }
    let cells = match done {
        Answers::Decided(values) => arrow::decided(&values),
        Answers::Annotated(kind, values) => {
            arrow::annotated(kind, &values.iter().collect::<Vec<_>>())
        }
    };
    if pandas {
        return arrow::pandas(py, cells);
    }
    let output = arrow::column(verb, &cells).map_err(|sentence| usage(py, &sentence))?;
    Ok(Py::new(py, Arrow::new(output))?.into_any())
}

/// `annotate(set, frame, on=)`: the frame with one new column per question.
#[pyfunction]
#[pyo3(signature = (engine, questions, records, on, deadline, token))]
pub(crate) fn _annotate_frame(
    engine: &Bound<'_, Engine>,
    questions: &Bound<'_, crate::asked::QuestionSet>,
    records: &Bound<'_, PyAny>,
    on: String,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
) -> PyResult<Arrow> {
    let py = engine.py();
    guard(py, || {
        let controls = controls(py, deadline, token)?;
        polars_frame(records, "annotate")?;
        let (engine, set) = (engine.get().0.clone(), questions.get().0.clone());
        let held = Imported::frame(records)?;
        on_worker(py, controls, held, move |held, options| {
            let (hold, memory) = (Arc::new(held), Readable::snapshot()?);
            let read = arrow::frame(&hold, &on, &memory)?;
            reserved(&set)?;
            let kept = arrow::kept(
                &hold,
                &memory,
                &read,
                set.members()
                    .map(|(name, _)| name)
                    .chain(std::iter::once("failed")),
            )?;
            let columns = answered(&engine, &set, &read.texts, options)?;
            Ok(arrow::frame_out(&hold, kept, &read, &columns)?)
        })
        .map(Arrow::new)
    })
}

/// Typed answer columns and a separate fixed-schema failure column.
fn answered(
    engine: &thinkthen::Engine,
    set: &QuestionSet,
    texts: &[&str],
    options: CallOptions<'_>,
) -> Result<Vec<(String, Cells)>, Stop> {
    let rows = engine
        .annotate_with(set, texts.iter().copied(), options)
        .collect::<Result<Vec<_>, _>>()?;
    let json = rows
        .iter()
        .map(|row| serde_json::from_str::<Members>(&row.value_json()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("the engine wrote a record the door cannot read: {error}"))?;
    let mut columns = Vec::new();
    let mut failed = vec![Vec::new(); rows.len()];
    let mut names = Vec::new();
    for (place, (name, kind)) in set.members().enumerate() {
        names.push(name.to_owned());
        let values: Option<Vec<&Annotated>> = rows
            .iter()
            .map(|row| row.values().get(place).map(|one| one.value()))
            .collect();
        let values = values.ok_or("the engine answered a record with no value")?;
        for (failures, (value, members)) in failed.iter_mut().zip(values.iter().zip(&json)) {
            failures.push(failure_cell(value, members, name)?);
        }
        let cells = arrow::annotated(kind, &values);
        columns.push((name.to_owned(), cells));
    }
    columns.push((
        "failed".to_owned(),
        Cells::Failures {
            names,
            rows: failed,
        },
    ));
    Ok(columns)
}

fn failure_cell(
    value: &Annotated,
    members: &Members,
    name: &str,
) -> Result<Option<arrow::FailureMarker>, Stop> {
    if !matches!(value, Annotated::Failed(_)) {
        return Ok(None);
    }
    let raw = members
        .get(name)
        .ok_or(format!("the engine's record has no member {name}"))?;
    let marker: serde_json::Value = serde_json::from_str(raw.get())
        .map_err(|error| format!("the engine's failure marker did not parse: {error}"))?;
    let marker = marker
        .get("failed")
        .ok_or("the engine's failure marker is missing")?;
    let field = |key| {
        marker
            .get(key)
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
            .ok_or(format!("the engine's failure marker has no {key}"))
    };
    Ok(Some(arrow::FailureMarker {
        kind: field("kind")?,
        cause: field("cause")?,
    }))
}

fn reserved(set: &QuestionSet) -> Result<(), Stop> {
    if set.members().any(|(name, _)| name == "failed") {
        return Err("the question name failed is reserved for frame failures".into());
    }
    Ok(())
}

/// One record's members as the engine wrote them, raw JSON text by name.
type Members = BTreeMap<String, Box<RawValue>>;

/// `annotate(set, df, on=)` over a pandas frame's marked `on` column: each
/// question's name to its values and dtype name, in set order (ticket 0122).
#[pyfunction]
#[pyo3(signature = (engine, questions, series, deadline, token))]
pub(crate) fn _annotate_column<'py>(
    engine: &Bound<'py, Engine>,
    questions: &Bound<'_, crate::asked::QuestionSet>,
    series: &Bound<'_, PyAny>,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
) -> PyResult<Bound<'py, PyDict>> {
    let py = engine.py();
    guard(py, || {
        let controls = controls(py, deadline, token)?;
        let (engine, set) = (engine.get().0.clone(), questions.get().0.clone());
        reserved(&set).map_err(|stop| match stop {
            Stop::Said(sentence) => usage(py, &sentence),
            Stop::Engine(error) => raised(py, &error),
        })?;
        let source = Source::read(series)?.0;
        let columns = over_texts(py, controls, source, move |texts, options| {
            answered(&engine, &set, texts, options)
        })?;
        let named = PyDict::new(py);
        for (name, cells) in columns {
            named.set_item(name, arrow::pandas(py, cells)?)?;
        }
        Ok(named)
    })
}

/// `recognize(frame, on=)`: one row per name, over `each_named`.
#[pyfunction]
#[pyo3(signature = (engine, ask, records, on, deadline, token))]
pub(crate) fn _recognize_frame(
    engine: &Bound<'_, Engine>,
    ask: &Bound<'_, Recognize>,
    records: &Bound<'_, PyAny>,
    on: String,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
) -> PyResult<Arrow> {
    let py = engine.py();
    guard(py, || {
        let controls = controls(py, deadline, token)?;
        polars_frame(records, "recognize")?;
        let due = due(&controls);
        let (engine, ask) = (engine.get().0.clone(), ask.get().0.clone());
        let held = Imported::frame(records)?;
        on_worker(py, controls, held, move |held, options| {
            let memory = Readable::snapshot()?;
            let read = arrow::frame(&held, &on, &memory)?;
            let found = each_named(&engine, &ask, &read.texts, options, due)?;
            Ok(names(found)?)
        })
        .map(Arrow::new)
    })
}

/// `recognize(df, on=)` over a pandas frame's `on` column: one list of
/// names per row, each a `dict` of 0106's fields but `row` (ticket 0122).
#[pyfunction]
#[pyo3(signature = (engine, ask, series, deadline, token))]
pub(crate) fn _recognize_column(
    engine: &Bound<'_, Engine>,
    ask: &Bound<'_, Recognize>,
    series: &Bound<'_, PyAny>,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
) -> PyResult<Vec<Vec<Py<PyDict>>>> {
    let py = engine.py();
    guard(py, || {
        let controls = controls(py, deadline, token)?;
        let due = due(&controls);
        let (engine, ask) = (engine.get().0.clone(), ask.get().0.clone());
        let source = Source::read(series)?.0;
        let found = over_texts(py, controls, source, move |texts, options| {
            Ok(each_named(&engine, &ask, texts, options, due)?)
        })?;
        let fields = |one: &RecognizedEntity| -> PyResult<Py<PyDict>> {
            let named = PyDict::new(py);
            named.set_item("text", one.text())?;
            named.set_item("start", one.start())?;
            named.set_item("end", one.end())?;
            named.set_item("length", one.length())?;
            named.set_item("kind", one.kind())?;
            named.set_item("strength", one.strength())?;
            Ok(named.unbind())
        };
        found
            .iter()
            .map(|row| row.iter().map(fields).collect())
            .collect()
    })
}

/// The call's deadline, resolved once at call start into an instant.
fn due(controls: &Controls) -> Option<Instant> {
    controls
        .deadline
        .filter(|seconds| *seconds >= 0.0)
        .and_then(|seconds| Duration::try_from_secs_f64(seconds).ok())
        .and_then(|budget| Instant::now().checked_add(budget))
}

/// Each text's names: one `recognize_with` call per text, all under one
/// deadline. `max_requests` caps each text's call.
fn each_named(
    engine: &thinkthen::Engine,
    ask: &thinkthen::Recognize,
    texts: &[&str],
    options: CallOptions<'_>,
    due: Option<Instant>,
) -> Result<Vec<Vec<RecognizedEntity>>, Error> {
    let options = due.map_or(options, |at| options.deadline_at(at));
    let named = |text: &&str| engine.recognize_with(ask, text, options);
    texts
        .iter()
        .map(|text| Ok(named(text)?.entities().to_vec()))
        .collect()
}

/// The recognized names as a table: row, text, start, end, length, kind,
/// strength.
fn names(found: Vec<Vec<RecognizedEntity>>) -> Result<arrow::Output, String> {
    let wide = |at: usize| i64::try_from(at).unwrap_or(i64::MAX);
    let (mut row, mut text, mut kind) = (Vec::new(), Vec::new(), Vec::new());
    let (mut start, mut end, mut length) = (Vec::new(), Vec::new(), Vec::new());
    let mut strength = Vec::new();
    let found = (1_i64..)
        .zip(found)
        .flat_map(|(place, row)| row.into_iter().map(move |one| (place, one)));
    for (place, one) in found {
        row.push(place);
        text.push(Some(one.text().to_owned()));
        start.push(wide(one.start()));
        end.push(wide(one.end()));
        length.push(wide(one.length()));
        kind.push(Some(one.kind().to_owned()));
        strength.push(Some(one.strength()));
    }
    arrow::table(&[
        ("row", Cells::Counts(row)),
        ("text", Cells::Texts(text)),
        ("start", Cells::Counts(start)),
        ("end", Cells::Counts(end)),
        ("length", Cells::Counts(length)),
        ("kind", Cells::Texts(kind)),
        ("strength", Cells::Numbers(strength)),
    ])
}

/// Where each text the engine reads starts, read on the worker (`probe` only).
#[cfg(feature = "probe")]
#[pyfunction]
pub(crate) fn _arrow_probe(series: &Bound<'_, PyAny>) -> PyResult<Vec<usize>> {
    let (source, controls) = (Source::Door(Imported::column(series)?), Controls::default());
    over_texts(series.py(), controls, source, |texts, _options| {
        Ok(texts.iter().map(|text| text.as_ptr().addr()).collect())
    })
}
