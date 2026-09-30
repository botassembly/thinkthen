//! Owned call observations and their Python result carrier.

use std::sync::{Arc, Mutex};

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use serde::Serialize;
use serde_json::Value;
use thinkthen::{ErrorKind, Facts, RecordObservation};

#[derive(Default, Clone)]
pub(crate) struct Observations(Arc<Mutex<Vec<Value>>>);

impl Observations {
    pub(crate) fn push(&self, event: RecordObservation<'_>) {
        let Some(row) = event
            .to_json()
            .and_then(|json| serde_json::from_str(&json).ok())
        else {
            return;
        };
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(row);
    }

    pub(crate) fn snapshot(&self) -> Vec<Value> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }
}

pub(crate) struct Completed<T> {
    pub(crate) value: T,
    pub(crate) facts: OwnedFacts,
    pub(crate) details: Vec<Value>,
}

/// One call's facts, or several calls' summed facts, which the core's
/// `Facts` cannot hold. A single call's facts serialize as the core wrote them.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct OwnedFacts {
    #[serde(skip)]
    pub(crate) core: Option<Facts>,
    pub(crate) cache_answers: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) output_tokens: Option<u64>,
    pub(crate) records: u64,
    pub(crate) requests_sent: u64,
    pub(crate) seconds: f64,
    #[serde(skip)]
    model_conflict: bool,
}

impl From<&Facts> for OwnedFacts {
    fn from(facts: &Facts) -> Self {
        Self {
            core: Some(facts.clone()),
            records: facts.records(),
            requests_sent: facts.requests_sent(),
            cache_answers: facts.cache_answers(),
            input_tokens: facts.input_tokens(),
            output_tokens: facts.output_tokens(),
            seconds: facts.seconds(),
            model: facts.model().map(str::to_owned),
            model_conflict: false,
        }
    }
}

impl OwnedFacts {
    pub(crate) fn empty() -> Self {
        Self {
            core: None,
            records: 0,
            requests_sent: 0,
            cache_answers: 0,
            input_tokens: None,
            output_tokens: None,
            seconds: 0.0,
            model: None,
            model_conflict: false,
        }
    }

    pub(crate) fn combine(&mut self, next: &Facts) {
        self.core = None;
        let prior_sends = self.requests_sent;
        self.records += next.records();
        self.requests_sent += next.requests_sent();
        self.cache_answers += next.cache_answers();
        if next.requests_sent() > 0 {
            self.input_tokens = if prior_sends == 0 {
                next.input_tokens()
            } else {
                self.input_tokens
                    .zip(next.input_tokens())
                    .map(|(a, b)| a + b)
            };
            self.output_tokens = if prior_sends == 0 {
                next.output_tokens()
            } else {
                self.output_tokens
                    .zip(next.output_tokens())
                    .map(|(a, b)| a + b)
            };
        }
        if let Some(model) = next.model() {
            if self.model.as_deref().is_some_and(|old| old != model) {
                self.model = None;
                self.model_conflict = true;
            } else if self.model.is_none() && !self.model_conflict {
                self.model = Some(model.to_owned());
            }
        }
    }
}

impl<T> Completed<T> {
    pub(crate) fn new(value: T, facts: &Facts) -> Self {
        Self {
            value,
            facts: facts.into(),
            details: Vec::new(),
        }
    }

    pub(crate) fn map<U>(self, convert: impl FnOnce(T) -> U) -> Completed<U> {
        Completed {
            value: convert(self.value),
            facts: self.facts,
            details: self.details,
        }
    }
}

#[pyclass(frozen, name = "Call", module = "thinkthen._thinkthen")]
pub(crate) struct PyCall {
    value: Py<PyAny>,
    probability: Py<PyAny>,
    facts: Py<PyAny>,
    details: Py<PyAny>,
}

#[pymethods]
impl PyCall {
    fn __bool__(&self) -> PyResult<bool> {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "a ThinkThen Call is not truthy; use tt.filter(q)(xs) to batch the records",
        ))
    }
    #[getter]
    fn value(&self, py: Python<'_>) -> Py<PyAny> {
        self.value.clone_ref(py)
    }
    #[getter]
    fn probability(&self, py: Python<'_>) -> Py<PyAny> {
        self.probability.clone_ref(py)
    }
    #[getter]
    fn facts(&self, py: Python<'_>) -> Py<PyAny> {
        self.facts.clone_ref(py)
    }
    #[getter]
    fn details(&self, py: Python<'_>) -> Py<PyAny> {
        self.details.clone_ref(py)
    }

    fn _with_value(&self, py: Python<'_>, value: Py<PyAny>) -> PyResult<Py<PyCall>> {
        Py::new(
            py,
            PyCall {
                value,
                probability: self.probability.clone_ref(py),
                facts: self.facts.clone_ref(py),
                details: self.details.clone_ref(py),
            },
        )
    }

    fn _with_probability(&self, py: Python<'_>, probability: Py<PyAny>) -> PyResult<Py<PyCall>> {
        Py::new(
            py,
            PyCall {
                value: self.value.clone_ref(py),
                probability,
                facts: self.facts.clone_ref(py),
                details: self.details.clone_ref(py),
            },
        )
    }
}

fn frozen(py: Python<'_>, value: &Value) -> PyResult<Py<PyAny>> {
    Ok(match value {
        Value::Null => py.None(),
        Value::Bool(value) => value.into_pyobject(py)?.to_owned().into_any().unbind(),
        Value::Number(value) => {
            let json = py.import("json")?;
            json.call_method1("loads", (value.to_string(),))?.unbind()
        }
        Value::String(value) => value.into_pyobject(py)?.into_any().unbind(),
        Value::Array(values) => PyTuple::new(
            py,
            values
                .iter()
                .map(|value| frozen(py, value))
                .collect::<PyResult<Vec<_>>>()?,
        )?
        .into_any()
        .unbind(),
        Value::Object(values) => {
            let map = PyDict::new(py);
            for (key, value) in values {
                map.set_item(key, frozen(py, value)?)?;
            }
            py.import("types")?
                .getattr("MappingProxyType")?
                .call1((map,))?
                .unbind()
        }
    })
}

pub(crate) fn python_facts(py: Python<'_>, facts: &Facts) -> PyResult<Py<PyAny>> {
    python_owned_facts(py, &OwnedFacts::from(facts))
}

/// Facts as the same read-only mapping every other result is.
pub(crate) fn python_owned_facts(py: Python<'_>, facts: &OwnedFacts) -> PyResult<Py<PyAny>> {
    let json = match &facts.core {
        Some(core) => serde_json::to_value(core),
        None => serde_json::to_value(facts),
    }
    .map_err(|_| {
        crate::raise(
            py,
            ErrorKind::Defect,
            "the facts could not be written",
            false,
        )
    })?;
    frozen(py, &json)
}

pub(crate) fn python_details(py: Python<'_>, details: &[Value]) -> PyResult<Py<PyAny>> {
    Ok(PyTuple::new(
        py,
        details
            .iter()
            .map(|detail| frozen(py, detail))
            .collect::<PyResult<Vec<_>>>()?,
    )?
    .into_any()
    .unbind())
}

pub(crate) fn call(py: Python<'_>, done: Completed<Py<PyAny>>) -> PyResult<Py<PyAny>> {
    Ok(Py::new(
        py,
        PyCall {
            value: done.value,
            probability: py.None(),
            facts: python_owned_facts(py, &done.facts)?,
            details: python_details(py, &done.details)?,
        },
    )?
    .into_any())
}

pub(crate) fn converted<T>(
    py: Python<'_>,
    done: Completed<T>,
    convert: impl FnOnce(T) -> PyResult<Py<PyAny>>,
) -> PyResult<Py<PyAny>> {
    let Completed {
        value,
        facts,
        details,
    } = done;
    match convert(value) {
        Ok(value) => call(
            py,
            Completed {
                value,
                facts,
                details,
            },
        ),
        Err(source) => {
            let error = if source.value(py).getattr("kind").is_ok() {
                source
            } else {
                let error = crate::raise(
                    py,
                    ErrorKind::Local,
                    "the completed result could not be rebuilt",
                    false,
                );
                error.set_cause(py, Some(source));
                error
            };
            error
                .value(py)
                .setattr("facts", python_owned_facts(py, &facts)?)?;
            error
                .value(py)
                .setattr("details", python_details(py, &details)?)?;
            Err(error)
        }
    }
}
