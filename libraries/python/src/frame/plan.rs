//! Preview a nullable column through its owned Arrow or pandas reader.

use pyo3::prelude::*;
use pyo3::types::PyDict;

use super::{Readable, Source, arrow};
use crate::asked::Question;
use crate::engine::{Arg, Engine, batch, context, plan_named};
use crate::{guard, raised, usage};

pub(crate) fn plan_column<'py>(
    engine: &Engine,
    py: Python<'py>,
    question: &Bound<'_, Question>,
    records: &Bound<'_, PyAny>,
    batch_value: Arg<'_, '_>,
    context_value: Arg<'_, '_>,
) -> PyResult<Bound<'py, PyDict>> {
    guard(py, || {
        let source = Source::read(records)?.0;
        let selected = batch(batch_value)?;
        let context = context(context_value)?;
        let options = thinkthen::CallOptions::new();
        let options = selected.map_or(options, |selected| options.batch(selected));
        let options = context
            .as_deref()
            .map_or(options, |text| options.context(text));
        let plan = match &source {
            Source::Door(held) => {
                let memory = Readable::snapshot().map_err(|message| usage(py, message))?;
                let rows = arrow::series(held, &memory).map_err(|message| usage(py, &message))?;
                engine.0.plan_with(
                    question.get().0.detail(),
                    rows.into_iter().flatten(),
                    options,
                )
            }
            Source::List(rows) => engine.0.plan_with(
                question.get().0.detail(),
                rows.iter().filter_map(Option::as_deref),
                options,
            ),
        }
        .map_err(|error| raised(py, &error))?;
        plan_named(py, plan)
    })
}
