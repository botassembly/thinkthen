//! Native collection calls over the existing owned nullable Arrow reader.
use super::{Source, nullable, over_texts, recognition::each_complete};
use crate::asked::{Question, Recognize, Recognized};
use crate::engine::only;
use crate::engine::{Arg, Engine, Held, collected};
use crate::input::controls;
use crate::result::Completed;
use crate::tally::PyTally;
use crate::{guard, result};
use pyo3::prelude::*;
use thinkthen::Evidence;

struct Indexed(usize, String);
impl Evidence for Indexed {
    fn evidence(&self) -> &str {
        &self.1
    }
}
fn inputs(rows: &[Option<&str>]) -> Vec<Indexed> {
    rows.iter()
        .enumerate()
        .filter_map(|(at, row)| row.map(|row| Indexed(at, row.to_owned())))
        .collect()
}

#[pyfunction]
#[pyo3(signature = (engine, verb, asked, series, batch, context, deadline, token, tally=None))]
#[expect(
    clippy::too_many_arguments,
    reason = "the column carries the existing call controls"
)]
pub(crate) fn _collection_column(
    engine: &Bound<'_, Engine>,
    verb: &str,
    asked: &Bound<'_, Question>,
    series: &Bound<'_, PyAny>,
    batch: Arg<'_, '_>,
    context: Arg<'_, '_>,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
    tally: Option<&Bound<'_, PyTally>>,
) -> PyResult<Py<PyAny>> {
    let py = engine.py();
    guard(py, || {
        let question = only(
            py,
            &asked.get().0,
            verb,
            if verb == "filter" { "decide" } else { verb },
        )?;
        let controls = controls(py, deadline, token)?;
        let batch = crate::engine::batch(batch)?;
        let context = crate::engine::context(context)?;
        let source = Source::read(series)?.0;
        let engine = engine.get().0.clone();
        let verb = verb.to_owned();
        let done = over_texts(
            py,
            controls,
            tally.map(|tally| tally.get().0.clone()),
            source,
            move |_, rows, options| {
                let options = batch.map_or(options, |batch| options.batch(batch));
                let options = context
                    .as_deref()
                    .map_or(options, |context| options.context(context));
                collection(&engine, &verb, &question, rows, options)
            },
        )?;
        result::converted(py, done, |rows| Ok(rows.into_pyobject(py)?.unbind()))
    })
}

#[pyfunction]
pub(crate) fn _recognize_series(
    engine: &Bound<'_, Engine>,
    ask: &Bound<'_, Recognize>,
    series: &Bound<'_, PyAny>,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
) -> PyResult<Py<PyAny>> {
    let py = engine.py();
    guard(py, || {
        let controls = controls(py, deadline, token)?;
        let due = super::recognition::due(&controls);
        let source = Source::read(series)?.0;
        let (engine, ask) = (engine.get().0.clone(), ask.get().0.clone());
        let done = over_texts(py, controls, None, source, move |texts, rows, options| {
            nullable::aligned(rows, each_complete(&engine, &ask, texts, options, due)?)
        })?;
        result::converted(py, done, |rows| {
            let rows = rows
                .iter()
                .map(|row| {
                    row.as_ref()
                        .map(|found| Py::new(py, Recognized::from(found)))
                        .transpose()
                })
                .collect::<PyResult<Vec<_>>>()?;
            Ok(rows.into_pyobject(py)?.unbind())
        })
    })
}

type Placed = (usize, String, Option<f64>);
fn collection(
    engine: &thinkthen::Engine,
    verb: &str,
    question: &thinkthen::Question,
    rows: &[Option<&str>],
    options: thinkthen::CallOptions<'_>,
) -> Result<Completed<Vec<Placed>>, super::Stop> {
    let records = inputs(rows);
    if verb == "filter" {
        return Ok(
            collected(engine.filter_with(question, records, options))?.map(|rows| {
                rows.into_iter()
                    .map(|row| (row.0, row.1, None))
                    .collect::<Vec<_>>()
            }),
        );
    }
    if verb == "rank" {
        let call = engine.rank_with(question, records, options)?;
        return Ok(Completed::new(
            call.value()
                .iter()
                .map(|row| {
                    (
                        row.input().0,
                        row.input().1.clone(),
                        Some(row.probability()),
                    )
                })
                .collect(),
            call.facts(),
        ));
    }
    let call = engine.find_with(question, records, options)?;
    let selected = call.value().selected();
    let rows = call
        .value()
        .candidates()
        .iter()
        .filter_map(|candidate| {
            let input = candidate.input()?;
            (selected.is_some_and(|selected| selected.0 == input.0))
                .then(|| (input.0, input.1.clone(), Some(candidate.probability())))
        })
        .collect::<Vec<_>>();
    Ok(Completed::new(rows, call.facts()))
}
