//! Owned call observations and their Python result carrier.

use std::sync::{Arc, Mutex};

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use serde_json::Value;
use thinkthen::{ErrorKind, Facts, RecordObservation};

/// The sentence for a question event serde could not write.
pub(crate) const UNWRITTEN: &str = "a question event could not be written";

#[derive(Default)]
struct Held {
    rows: Vec<Value>,
    unwritten: bool,
}

#[derive(Default, Clone)]
pub(crate) struct Observations(Arc<Mutex<Held>>);

impl Observations {
    pub(crate) fn push(&self, event: RecordObservation<'_>) {
        if matches!(event, RecordObservation::Row { .. }) {
            return;
        }
        let written = serde_json::to_value(&event);
        let mut held = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match written {
            Ok(row) => held.rows.push(row),
            Err(_) => held.unwritten = true,
        }
    }

    /// The question events so far, or [`UNWRITTEN`] when one failed to write.
    pub(crate) fn snapshot(&self) -> Result<Vec<Value>, &'static str> {
        let held = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if held.unwritten {
            return Err(UNWRITTEN);
        }
        Ok(held.rows.clone())
    }
}

pub(crate) struct Completed<T> {
    pub(crate) value: T,
    pub(crate) facts: Facts,
    pub(crate) details: Vec<Value>,
}

impl<T> Completed<T> {
    pub(crate) fn new(value: T, facts: &Facts) -> Self {
        Self {
            value,
            facts: facts.clone(),
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

/// Facts as the same read-only mapping every other result is.
pub(crate) fn python_facts(py: Python<'_>, facts: &Facts) -> PyResult<Py<PyAny>> {
    let json = serde_json::to_value(facts).map_err(|_| {
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
            facts: python_facts(py, &done.facts)?,
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
                .setattr("facts", python_facts(py, &facts)?)?;
            error
                .value(py)
                .setattr("details", python_details(py, &details)?)?;
            Err(error)
        }
    }
}
