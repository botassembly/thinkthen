//! Complete calls keep the existing Python cancellation and conversion boundary.
use super::{Arg, Completed, Held, complete, controls, guard, raised, result};
use crate::worker::run_observed;
use pyo3::prelude::*;
use thinkthen::{Engine, Error, Surface};
pub(super) fn stream(
    engine: &Engine,
    py: Python<'_>,
    request: String,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
    surface: Option<&str>,
) -> PyResult<crate::complete_stream::CompleteStream> {
    let surface = native_surface(py, surface)?;
    let controls = controls(py, deadline, token)?;
    let session = complete::stream::Session::start(
        engine.clone(),
        request,
        controls.deadline,
        controls.token,
        None,
        surface,
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
    surface: Option<&str>,
) -> PyResult<Py<PyAny>> {
    guard(py, || {
        let surface = native_surface(py, surface)?;
        let engine = engine.clone();
        let controls = controls(py, deadline, token)?;
        let done = run_observed(py, controls, move |options| {
            let (value, facts) = complete::execute(
                &engine,
                complete::parse(&request)?,
                options.surface(surface),
            )?;
            let output = value;
            Ok::<_, Error>(Completed::new(output, &facts))
        })?;
        result::converted(py, done, |text| {
            Ok(py.import("json")?.call_method1("loads", (text,))?.unbind())
        })
    })
}

fn native_surface(py: Python<'_>, value: Option<&str>) -> PyResult<Surface> {
    let surface = value.unwrap_or("python").parse::<Surface>().ok();
    match surface {
        Some(s @ (Surface::Python | Surface::Pandas | Surface::PythonPolars)) => Ok(s),
        _ => Err(raised(
            py,
            &complete::usage("surface must be python, pandas or python-polars"),
        )),
    }
}
