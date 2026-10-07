//! Complete calls keep the existing Python cancellation and conversion boundary.
use super::{Arg, Completed, Held, complete, controls, guard, raised, result};
use crate::worker::run_observed;
use pyo3::prelude::*;
use thinkthen::{Engine, Error};
pub(super) fn stream(
    engine: &Engine,
    py: Python<'_>,
    request: String,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
) -> PyResult<crate::complete_stream::CompleteStream> {
    let controls = controls(py, deadline, token)?;
    let session = complete::stream::Session::start(
        engine.clone(),
        request,
        controls.deadline,
        controls.token,
        None,
    )
    .map_err(|e| raised(py, &e))?;
    Ok(crate::complete_stream::CompleteStream(session))
}
pub(super) fn call(
    engine: &Engine,
    py: Python<'_>,
    request: String,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
) -> PyResult<Py<PyAny>> {
    guard(py, || {
        let engine = engine.clone();
        let controls = controls(py, deadline, token)?;
        let done = run_observed(py, controls, move |options| {
            let (value, facts) = complete::execute(&engine, complete::parse(&request)?, options)?;
            let output = value;
            Ok::<_, Error>(Completed::new(output, &facts))
        })?;
        result::converted(py, done, |text| {
            Ok(py.import("json")?.call_method1("loads", (text,))?.unbind())
        })
    })
}
