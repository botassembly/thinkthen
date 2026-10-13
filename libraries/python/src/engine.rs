//! One owned native engine for canonical sessions and no-send previews.
use crate::{guard, raised, usage};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use thinkthen::{EngineBuilder, RequestEnvironment, Surface};

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

    #[pyo3(signature = (function, values, feed=false, surface=None))]
    fn _request_session(
        &self,
        py: Python<'_>,
        function: &str,
        values: (Bound<'_, PyAny>, Bound<'_, PyAny>, Bound<'_, PyAny>),
        feed: bool,
        surface: Option<&str>,
    ) -> PyResult<crate::request::Session> {
        let (question, source, options) = values;
        let selected = surface.unwrap_or("python").parse::<Surface>().ok();
        let selected = match selected {
            Some(value @ (Surface::Python | Surface::Pandas | Surface::PythonPolars)) => value,
            _ => return Err(usage(py, "surface must be python, pandas or python-polars")),
        };
        let request = crate::inputs::request(py, function, &question, &source, &options, feed)?;
        crate::request::Session::start(py, &self.0, request, selected)
    }

    fn _plan(
        &self,
        py: Python<'_>,
        function: &str,
        question: &Bound<'_, PyAny>,
        source: &Bound<'_, PyAny>,
        options: &Bound<'_, PyAny>,
    ) -> PyResult<Py<PyAny>> {
        guard(py, || {
            let admitted = crate::inputs::request(py, function, question, source, options, false)?
                .admit()
                .map_err(|e| raised(py, &e))?;
            let plan = py
                .detach(|| {
                    self.0
                        .plan_request(&admitted, RequestEnvironment::default())
                })
                .map_err(|e| raised(py, &e))?;
            let value = serde_json::to_value(plan)
                .map_err(|_| crate::defect(py, "native plan could not be serialized"))?;
            crate::results_generated::convert(py, "completeplan", &value)
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
