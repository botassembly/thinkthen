//! A no-send preview through the public core planner.

use pyo3::prelude::*;
use pyo3::types::PyDict;

use super::{Arg, Engine};
use crate::asked::Question;
use crate::input::texts;
use crate::{guard, raised};

pub(super) fn estimate<'py>(
    engine: &Engine,
    py: Python<'py>,
    question: &Bound<'_, Question>,
    records: &Bound<'_, PyAny>,
    batch: Arg<'_, '_>,
    context: Arg<'_, '_>,
) -> PyResult<Bound<'py, PyDict>> {
    guard(py, || {
        let records = texts(records)?;
        let selected = super::batch(batch)?;
        let context = super::context(context)?;
        let options = thinkthen::CallOptions::new();
        let options = selected.map_or(options, |selected| options.batch(selected));
        let options = context
            .as_deref()
            .map_or(options, |text| options.context(text));
        let plan = engine
            .0
            .plan_with(question.get().0.detail(), records, options)
            .map_err(|error| raised(py, &error))?;
        named(py, plan)
    })
}

pub(crate) fn named(py: Python<'_>, plan: thinkthen::PlanEstimate) -> PyResult<Bound<'_, PyDict>> {
    let named = PyDict::new(py);
    named.set_item("records", plan.records())?;
    named.set_item("requests", plan.requests())?;
    named.set_item("estimated_bytes", plan.estimated_bytes())?;
    named.set_item("estimated_input_tokens", plan.estimated_input_tokens())?;
    named.set_item("upper_bound", plan.upper_bound())?;
    named.set_item("first_body", plan.first_body())?;
    Ok(named)
}
