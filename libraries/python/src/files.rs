//! Explicit file selections through the same located dispatch as the C door.

use crate::engine::{Arg, Engine, Held, batch, context};
use crate::{guard, input, raised, result, worker};
use pyo3::prelude::*;

#[path = "../../shared/source.rs"]
mod source;

#[pyfunction]
pub(crate) fn _read_files(py: Python<'_>, selection: &str) -> PyResult<String> {
    guard(py, || {
        let selection = source::parse(selection).map_err(|e| raised(py, &e))?;
        let records = py
            .detach(|| {
                selection
                    .read()?
                    .collect::<Result<Vec<_>, thinkthen::Error>>()
            })
            .map_err(|e| raised(py, &e))?;
        serde_json::to_string(&records)
            .map_err(|_| crate::defect(py, "source records could not be written"))
    })
}

#[pyfunction]
pub(crate) fn _spec_source(py: Python<'_>, path: &str) -> PyResult<String> {
    thinkthen::read_question_file(path).map_err(|_| {
        crate::raise(
            py,
            thinkthen::ErrorKind::Local,
            "the question file could not be read",
            false,
        )
    })
}

#[expect(
    clippy::too_many_arguments,
    reason = "the source call retains the public call controls"
)]
pub(crate) fn execute(
    engine: &Engine,
    py: Python<'_>,
    verb: &str,
    question: &str,
    selection: &str,
    selected: Arg<'_, '_>,
    shared: Arg<'_, '_>,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
) -> PyResult<Py<PyAny>> {
    guard(py, || {
        let (engine, verb, question) = (engine.0.clone(), verb.to_owned(), question.to_owned());
        let selection = selection.to_owned();
        let question = if verb == "annotate" {
            format!("{{\"annotate\":{question}}}")
        } else if verb == "filter" {
            let mut body: serde_json::Map<String, serde_json::Value> =
                serde_json::from_str(&question)
                    .map_err(|_| crate::usage(py, "filter requires a question object"))?;
            if let Some(text) = body.remove("decide") {
                body.insert("filter".into(), text);
            }
            serde_json::Value::Object(body).to_string()
        } else {
            question
        };
        let selected = batch(selected)?;
        let shared = context(shared)?;
        let controls = input::controls(py, deadline, token)?;
        let done = worker::run_observed(py, controls, move |options| {
            let options = selected.map_or(options, |b| options.batch(b));
            let options = shared.as_deref().map_or(options, |c| options.context(c));
            let (value, facts) = source::dispatch(&engine, &question, &selection, options)?;
            Ok::<_, thinkthen::Error>(result::Completed::new(value, &facts))
        })?;
        result::converted(py, done, |value| {
            Ok(value.into_pyobject(py)?.into_any().unbind())
        })
    })
}
