//! The column and frame glue (tickets 0106 and 0122). A column verb makes one
//! engine call: `decide` one `decide_many`, and `choose`, `score`, and `tag`
//! one `annotate` over a one-question set. The worker owns the imported
//! column, reads it in place, and releases it (amendment changes 1, 3, and 6).
//! A pandas answer comes back as Python values and a dtype name (ticket 0122).

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::value::RawValue;
use thinkthen::{
    Annotated, Answer, BatchSetting, CallOptions, QuestionKind, QuestionSet, RecognizedEntity,
};

use crate::arrow::{self, Arrow, Cells, Imported, Readable};
use crate::asked::{Asked, Recognize};
use crate::engine::{Arg, Engine, Held, annotated, answer, batch, collected};
use crate::input::{Pandas, controls, listed_nullable, polars_frame, top};
use crate::result::{self, Completed, OwnedFacts};
use crate::worker::{Controls, run_tallied};
use crate::{guard, raised, usage};

mod names;
mod nullable;
mod plan;
mod recognition;
mod stop;
use names::names;
pub(crate) use plan::plan_column;
use recognition::{due, each_named};
use stop::{AccountedFailure, Stop};

/// Run `job` on the worker, which owns `held` and drops it before it answers.
fn on_worker<H, T, F>(py: Python<'_>, controls: Controls, held: H, job: F) -> PyResult<Completed<T>>
where
    H: Send + 'static,
    T: Send + 'static,
    F: FnOnce(H, CallOptions<'_>) -> Result<Completed<T>, Stop> + Send + 'static,
{
    on_worker_tallied(py, controls, None, held, job)
}

fn on_worker_tallied<H, T, F>(
    py: Python<'_>,
    controls: Controls,
    tally: Option<thinkthen::Tally>,
    held: H,
    job: F,
) -> PyResult<Completed<T>>
where
    H: Send + 'static,
    T: Send + 'static,
    F: FnOnce(H, CallOptions<'_>) -> Result<Completed<T>, Stop> + Send + 'static,
{
    run_tallied(py, controls, tally, move |options| {
        let began = Instant::now();
        job(held, options).map_err(|stop| match stop {
            Stop::Said(message, None) => {
                let mut facts = OwnedFacts::empty();
                facts.seconds = began.elapsed().as_secs_f64();
                Stop::Said(message, Some(Box::new(facts)))
            }
            other => other,
        })
    })
}

/// Where a column's texts come from: the Arrow door, or a pandas Series the
/// package marks for 0105's list reader (ticket 0122). `read` says if marked.
enum Source {
    Door(Imported),
    List(Vec<Option<String>>),
}

impl Source {
    fn read(value: &Bound<'_, PyAny>) -> PyResult<(Self, bool)> {
        let Ok(marked) = value.cast::<Pandas>() else {
            return Ok((Self::Door(Imported::column(value)?), false));
        };
        let source = match marked.get() {
            Pandas(inner, true) => Self::List(listed_nullable(inner.bind(value.py()))?),
            Pandas(inner, false) => Self::Door(Imported::column(inner.bind(value.py()))?),
        };
        Ok((source, true))
    }
}

/// Run `job` over a column's texts on the worker, which owns the source.
fn over_texts<T, F>(
    py: Python<'_>,
    controls: Controls,
    tally: Option<thinkthen::Tally>,
    source: Source,
    job: F,
) -> PyResult<Completed<T>>
where
    T: Send + 'static,
    F: FnOnce(&[&str], &[Option<&str>], CallOptions<'_>) -> Result<Completed<T>, Stop>
        + Send
        + 'static,
{
    on_worker_tallied(
        py,
        controls,
        tally,
        source,
        move |source, options| match &source {
            Source::Door(held) => {
                let memory = Readable::snapshot()?;
                nullable::present(&arrow::series(held, &memory)?, options, job)
            }
            Source::List(all) => nullable::present(
                &all.iter().map(|one| one.as_deref()).collect::<Vec<_>>(),
                options,
                job,
            ),
        },
    )
}

/// What a column call gives back.
enum Answers {
    Decided(Vec<Option<Answer>>),
    Annotated(QuestionKind, Vec<Option<Annotated>>),
}

type AnswerColumns = Vec<(String, Cells)>;

/// `decide`, `choose`, `score`, or `tag` over a column. A Polars `Series`
/// gets an answer column to rebuild, and a pandas Series its values and
/// dtype name. Another producer gets a list.
#[expect(
    clippy::too_many_arguments,
    reason = "the column route carries the public call controls"
)]
pub(crate) fn ask_column(
    engine: &thinkthen::Engine,
    verb: &str,
    asked: &Asked,
    value: &Bound<'_, PyAny>,
    batch: Option<BatchSetting>,
    context: Option<String>,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
    tally: Option<thinkthen::Tally>,
) -> PyResult<Py<PyAny>> {
    let py = value.py();
    if verb == "details" {
        return Err(usage(py, "details reads one str, not a column"));
    }
    if verb != "decide" && context.is_some() {
        return Err(usage(py, "this column call does not take a shared context"));
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
    let done = over_texts(py, controls, tally, source, move |texts, rows, options| {
        let options = batch.map_or(options, |batch| options.batch(batch));
        let options = context
            .as_deref()
            .map_or(options, |context| options.context(context));
        let texts = texts.iter().copied();
        if let (Some(set), Asked::Plain(question)) = (&set, &asked) {
            let done = collected(engine.annotate_with(set, texts, options))?;
            let facts = done.facts.clone();
            let values = done
                .value
                .iter()
                .map(|one| one.values().first().map(|named| named.value().clone()))
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| Stop::after("the engine answered a record with no value", facts))?;
            return nullable::aligned(
                rows,
                Completed {
                    value: values,
                    facts: done.facts,
                    details: done.details,
                },
            )
            .map(|done| done.map(|values| Answers::Annotated(question.kind(), values)));
        }
        let done = collected(engine.decide_many_with(asked.decision(), texts, options))?
            .map(|rows| rows.iter().map(|row| *row.value()).collect());
        nullable::aligned(rows, done).map(|done| done.map(Answers::Decided))
    })?;
    result::converted(py, done, |done| {
        if !polars && !pandas {
            let values: Vec<Py<PyAny>> = match done {
                Answers::Decided(values) => values
                    .into_iter()
                    .map(|one| one.map_or_else(|| py.None(), |one| answer(py, one)))
                    .collect(),
                Answers::Annotated(_, values) => values
                    .into_iter()
                    .map(|one| one.map_or_else(|| Ok(py.None()), |one| annotated(py, one)))
                    .collect::<PyResult<_>>()?,
            };
            return Ok(values.into_pyobject(py)?.unbind());
        }
        let cells = match done {
            Answers::Decided(values) => arrow::decided(&values),
            Answers::Annotated(kind, values) => {
                arrow::annotated(kind, &values.iter().map(Option::as_ref).collect::<Vec<_>>())
            }
        };
        if pandas {
            return arrow::pandas(py, cells);
        }
        let output = arrow::column(verb, &cells).map_err(|sentence| usage(py, &sentence))?;
        Ok(Py::new(py, Arrow::new(output))?.into_any())
    })
}

/// `annotate(set, frame, on=)`: the frame with one new column per question.
#[pyfunction]
#[pyo3(signature = (engine, questions, records, on, batch_value, deadline, token))]
#[expect(
    clippy::too_many_arguments,
    reason = "PyO3 mirrors the public frame controls"
)]
pub(crate) fn _annotate_frame(
    engine: &Bound<'_, Engine>,
    questions: &Bound<'_, crate::asked::QuestionSet>,
    records: &Bound<'_, PyAny>,
    on: String,
    batch_value: Arg<'_, '_>,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
) -> PyResult<Py<PyAny>> {
    let py = engine.py();
    guard(py, || {
        let controls = controls(py, deadline, token)?;
        let batch = batch(batch_value)?;
        polars_frame(records, "annotate")?;
        let (engine, set) = (engine.get().0.clone(), questions.get().0.clone());
        let held = Imported::frame(records)?;
        on_worker(py, controls, held, move |held, options| {
            let options = batch.map_or(options, |batch| options.batch(batch));
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
            let columns = nullable::present(&read.texts, options, |texts, rows, options| {
                answered(&engine, &set, texts, rows, options)
            })?;
            let output = arrow::frame_out(&hold, kept, &read, &columns.value)
                .map_err(|sentence| Stop::after(sentence, columns.facts.clone()))?;
            Ok(columns.map(|_| output))
        })
        .and_then(|done| {
            result::converted(py, done, |out| Ok(Py::new(py, Arrow::new(out))?.into_any()))
        })
    })
}

/// Typed answer columns and a separate fixed-schema failure column.
fn answered(
    engine: &thinkthen::Engine,
    set: &QuestionSet,
    texts: &[&str],
    input: &[Option<&str>],
    options: CallOptions<'_>,
) -> Result<Completed<AnswerColumns>, Stop> {
    let done = nullable::aligned(
        input,
        collected(engine.annotate_with(set, texts.iter().copied(), options))?,
    )?;
    let facts = done.facts.clone();
    let rows = done.value;
    let converted = (|| -> Result<AnswerColumns, Stop> {
        let json = rows
            .iter()
            .map(|row| {
                row.as_ref()
                    .map(|row| serde_json::from_str::<Members>(&row.value_json()))
                    .transpose()
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("the engine wrote a record the door cannot read: {error}"))?;
        let mut columns = Vec::new();
        let mut failed = vec![Vec::new(); rows.len()];
        let mut names = Vec::new();
        for (place, (name, kind)) in set.members().enumerate() {
            names.push(name.to_owned());
            let values: Option<Vec<Option<&Annotated>>> = rows
                .iter()
                .map(|row| match row {
                    Some(row) => row.values().get(place).map(|one| Some(one.value())),
                    None => Some(None),
                })
                .collect();
            let values = values.ok_or("the engine answered a record with no value")?;
            for (failures, (value, members)) in failed.iter_mut().zip(values.iter().zip(&json)) {
                failures.push(match (value, members) {
                    (Some(value), Some(members)) => failure_cell(value, members, name)?,
                    _ => None,
                });
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
    })()
    .map_err(|error| error.with_facts(facts))?;
    Ok(Completed {
        value: converted,
        facts: done.facts,
        details: done.details,
    })
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
#[pyo3(signature = (engine, questions, series, batch_value, deadline, token))]
pub(crate) fn _annotate_column<'py>(
    engine: &Bound<'py, Engine>,
    questions: &Bound<'_, crate::asked::QuestionSet>,
    series: &Bound<'_, PyAny>,
    batch_value: Arg<'_, '_>,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
) -> PyResult<Py<PyAny>> {
    let py = engine.py();
    guard(py, || {
        let controls = controls(py, deadline, token)?;
        let batch = batch(batch_value)?;
        let (engine, set) = (engine.get().0.clone(), questions.get().0.clone());
        reserved(&set).map_err(|stop| match stop {
            Stop::Said(sentence, _) => usage(py, &sentence),
            Stop::Engine(error) => raised(py, &error),
            Stop::Accounted(account) => raised(py, &account.error),
        })?;
        let source = Source::read(series)?.0;
        let columns = over_texts(py, controls, None, source, move |texts, rows, options| {
            let options = batch.map_or(options, |batch| options.batch(batch));
            answered(&engine, &set, texts, rows, options)
        })?;
        result::converted(py, columns, |columns| {
            let named = PyDict::new(py);
            for (name, cells) in columns {
                named.set_item(name, arrow::pandas(py, cells)?)?;
            }
            Ok(named.into_any().unbind())
        })
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
) -> PyResult<Py<PyAny>> {
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
            let found = nullable::present(&read.texts, options, |texts, rows, options| {
                nullable::aligned(rows, each_named(&engine, &ask, texts, options, due)?)
            })?;
            let table = names(found.value)
                .map_err(|sentence| Stop::after(sentence, found.facts.clone()))?;
            Ok(Completed {
                value: table,
                facts: found.facts,
                details: found.details,
            })
        })
        .and_then(|done| {
            result::converted(py, done, |out| Ok(Py::new(py, Arrow::new(out))?.into_any()))
        })
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
) -> PyResult<Py<PyAny>> {
    let py = engine.py();
    guard(py, || {
        let controls = controls(py, deadline, token)?;
        let due = due(&controls);
        let (engine, ask) = (engine.get().0.clone(), ask.get().0.clone());
        let source = Source::read(series)?.0;
        let found = over_texts(py, controls, None, source, move |texts, rows, options| {
            nullable::aligned(rows, each_named(&engine, &ask, texts, options, due)?)
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
        result::converted(py, found, |found| {
            let rows = found
                .iter()
                .map(|row| {
                    row.as_ref()
                        .map(|row| row.iter().map(fields).collect::<PyResult<Vec<_>>>())
                        .transpose()
                })
                .collect::<PyResult<Vec<Option<Vec<_>>>>>()?;
            Ok(rows.into_pyobject(py)?.unbind())
        })
    })
}

/// Where each text the engine reads starts, read on the worker (`probe` only).
#[cfg(feature = "probe")]
#[pyfunction]
pub(crate) fn _arrow_probe(series: &Bound<'_, PyAny>) -> PyResult<Vec<usize>> {
    let (py, source) = (series.py(), Source::Door(Imported::column(series)?));
    over_texts(py, Controls::default(), None, source, |texts, rows, _| {
        nullable::aligned(
            rows,
            Completed {
                value: texts.iter().map(|text| text.as_ptr().addr()).collect(),
                facts: OwnedFacts::empty(),
                details: Vec::new(),
            },
        )
    })
    .map(|done| {
        done.value
            .into_iter()
            .map(Option::unwrap_or_default)
            .collect()
    })
}
