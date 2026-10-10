//! One owned native engine for canonical sessions and no-send previews.
use crate::{guard, raised, usage};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use thinkthen::{EngineBuilder, Request, RequestEnvironment, Surface};

#[pyclass(frozen, name = "_Engine", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct Engine(thinkthen::Engine);

#[pymethods]
impl Engine {
    #[new]
    fn new(py: Python<'_>, settings: &str) -> PyResult<Self> {
        guard(py, || {
            let builder =
                EngineBuilder::from_settings_json(settings).map_err(|e| raised(py, &e))?;
            py.detach(|| builder.build())
                .map(Self)
                .map_err(|e| raised(py, &e))
        })
    }

    #[pyo3(signature = (request, surface=None))]
    fn _request_session(
        &self,
        py: Python<'_>,
        request: &str,
        surface: Option<&str>,
    ) -> PyResult<crate::request::Session> {
        let selected = surface.unwrap_or("python").parse::<Surface>().ok();
        let selected = match selected {
            Some(value @ (Surface::Python | Surface::Pandas | Surface::PythonPolars)) => value,
            _ => return Err(usage(py, "surface must be python, pandas or python-polars")),
        };
        crate::request::Session::start(py, &self.0, request, selected)
    }

    fn _plan<'py>(&self, py: Python<'py>, request: &str) -> PyResult<Bound<'py, PyDict>> {
        guard(py, || {
            let admitted = Request::from_json(request)
                .and_then(Request::admit)
                .map_err(|e| raised(py, &e))?;
            let plan = py
                .detach(|| {
                    self.0
                        .plan_request(&admitted, RequestEnvironment::default())
                })
                .map_err(|e| raised(py, &e))?;
            let value = PyDict::new(py);
            value.set_item("records", plan.records())?;
            value.set_item("requests", plan.requests())?;
            value.set_item("estimated_bytes", plan.estimated_bytes())?;
            value.set_item("estimated_input_tokens", plan.estimated_input_tokens())?;
            value.set_item("upper_bound", plan.upper_bound())?;
            value.set_item("first_body", plan.first_body())?;
            Ok(value)
        })
    }

    fn usage_persistence(&self, py: Python<'_>) -> PyResult<(String, Option<String>)> {
        persistence(py, self.0.usage_persistence())
    }

    fn finish_usage_status(&self, py: Python<'_>) -> PyResult<(String, Option<String>)> {
        persistence(py, py.detach(|| self.0.finish_usage_status()))
    }

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
}

fn persistence(
    py: Python<'_>,
    state: thinkthen::UsagePersistence,
) -> PyResult<(String, Option<String>)> {
    let value = serde_json::to_value(state)
        .map_err(|_| crate::defect(py, "native usage state could not be serialized"))?;
    let name = value
        .as_str()
        .ok_or_else(|| crate::defect(py, "native usage state is not text"))?;
    Ok((name.to_owned(), state.advice().map(str::to_owned)))
}
