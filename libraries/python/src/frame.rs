//! The column and frame glue (ticket 0106). A column verb makes one engine
//! call: `decide` one `decide_many`, and `choose`, `score`, and `tag` one
//! `annotate` over a one-question set. The worker owns the imported column,
//! reads it in place, and releases it (amendment changes 1, 3, and 6).

use std::sync::Arc;
use std::time::{Duration, Instant};

use pyo3::prelude::*;
use thinkthen::{
    Annotated, Answer, CallOptions, Error, QuestionKind, QuestionSet, RecognizedEntity,
};

use crate::arrow::{self, Arrow, Cells, Imported, Readable};
use crate::asked::{Asked, Recognize};
use crate::engine::{Arg, Engine, Held, annotated, answer};
use crate::input::{controls, polars_frame, refuse_pandas};
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
fn on_worker<T, F>(py: Python<'_>, controls: Controls, held: Imported, job: F) -> PyResult<T>
where
    T: Send + 'static,
    F: FnOnce(Imported, CallOptions<'_>) -> Result<T, Stop> + Send + 'static,
{
    run(py, controls, move |options| Ok(job(held, options)))?.map_err(|stop| match stop {
        Stop::Engine(error) => raised(py, &error),
        Stop::Said(sentence) => usage(py, &sentence),
    })
}

/// What a column call gives back.
enum Answers {
    Decided(Vec<Answer>),
    Annotated(QuestionKind, Vec<Annotated>),
}

/// `decide`, `choose`, `score`, or `tag` over a column. A Polars `Series`
/// gets an answer column to rebuild. Another producer gets a list.
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
    let polars = refuse_pandas(value)? == "polars";
    let (engine, asked, held) = (engine.clone(), asked.clone(), Imported::column(value)?);
    let done = on_worker(py, controls, held, move |held, options| {
        let memory = Readable::snapshot()?;
        let texts = arrow::series(&held, &memory)?;
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
    if !polars {
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
            let rows = engine
                .annotate_with(&set, read.texts.iter().copied(), options)
                .collect::<Result<Vec<_>, _>>()?;
            let mut columns = Vec::new();
            for (place, (name, kind)) in set.members().enumerate() {
                let values: Option<Vec<&Annotated>> = rows
                    .iter()
                    .map(|row| row.values().get(place).map(|one| one.value()))
                    .collect();
                let values = values.ok_or("the engine answered a record with no value")?;
                columns.push((name.to_owned(), arrow::annotated(kind, &values)));
            }
            Ok(arrow::frame_out(&hold, &memory, &read, &columns)?)
        })
        .map(Arrow::new)
    })
}

/// `recognize(frame, on=)`: one row per name. The texts share one deadline,
/// resolved at call start into an instant. `max_requests` caps each text.
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
        let due = controls
            .deadline
            .filter(|seconds| *seconds >= 0.0)
            .and_then(|seconds| Duration::try_from_secs_f64(seconds).ok())
            .map(|budget| Instant::now().checked_add(budget));
        let (engine, ask) = (engine.get().0.clone(), ask.get().0.clone());
        let held = Imported::frame(records)?;
        on_worker(py, controls, held, move |held, options| {
            let memory = Readable::snapshot()?;
            let read = arrow::frame(&held, &on, &memory)?;
            let options = match due {
                Some(Some(at)) => options.deadline_at(at),
                _ => options,
            };
            let mut found = Vec::new();
            for (place, text) in (1_i64..).zip(&read.texts) {
                let entities = engine.recognize_with(&ask, text, options)?;
                found.extend(entities.entities().iter().map(|one| (place, one.clone())));
            }
            Ok(names(found)?)
        })
        .map(Arrow::new)
    })
}

/// The recognized names as a table: row, text, kind, start, end, strength.
fn names(found: Vec<(i64, RecognizedEntity)>) -> Result<arrow::Output, String> {
    let wide = |at: usize| i64::try_from(at).unwrap_or(i64::MAX);
    let (mut row, mut text, mut kind) = (Vec::new(), Vec::new(), Vec::new());
    let (mut start, mut end, mut strength) = (Vec::new(), Vec::new(), Vec::new());
    for (place, one) in found {
        row.push(place);
        text.push(Some(one.name().to_owned()));
        kind.push(Some(one.kind().to_owned()));
        start.push(wide(one.start()));
        end.push(wide(one.end()));
        strength.push(Some(one.strength()));
    }
    arrow::table(&[
        ("row", Cells::Counts(row)),
        ("text", Cells::Texts(text)),
        ("kind", Cells::Texts(kind)),
        ("start", Cells::Counts(start)),
        ("end", Cells::Counts(end)),
        ("strength", Cells::Numbers(strength)),
    ])
}

/// Where each text the engine reads starts, read on the worker (`probe` only).
#[cfg(feature = "probe")]
#[pyfunction]
pub(crate) fn _arrow_probe(series: &Bound<'_, PyAny>) -> PyResult<Vec<usize>> {
    let py = series.py();
    let held = Imported::column(series)?;
    on_worker(py, Controls::default(), held, |held, _options| {
        let memory = Readable::snapshot()?;
        let texts = arrow::series(&held, &memory)?;
        Ok(texts.iter().map(|text| text.as_ptr().addr()).collect())
    })
}
