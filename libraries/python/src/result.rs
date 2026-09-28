//! Owned call observations and their Python result carrier.

use std::sync::{Arc, Mutex};

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use serde_json::{Value, json};
use thinkthen::{ErrorKind, Facts, Judgment, Probabilities, RecordObservation};

use crate::engine::cause;

#[derive(Default, Clone)]
pub(crate) struct Observations(Arc<Mutex<Vec<Value>>>);

fn put(row: &mut Value, name: &str, value: Value) {
    if let Some(fields) = row.as_object_mut() {
        fields.insert(name.to_owned(), value);
    }
}

impl Observations {
    pub(crate) fn push(&self, event: RecordObservation<'_>) {
        let RecordObservation::Question {
            index,
            member,
            stage,
            position,
            detail,
        } = event
        else {
            return;
        };
        let mut row = json!({
            "index": index,
            "question_sha256": detail.question_sha256(),
            "requests": detail.requests(),
            "request_digests": detail.requests(),
            "requests_sent": detail.requests_sent(),
            "cached": detail.cached(),
            "failed_questions": detail.failed_questions(),
            "url": detail.url(),
        });
        if let Some(member) = member {
            put(&mut row, "member", json!(member));
        }
        if let Some(stage) = stage {
            put(&mut row, "stage", json!(stage));
        }
        put(&mut row, "position", json!(position));
        if let Some(answer) = detail.value() {
            put(
                &mut row,
                "answer",
                match answer {
                    Judgment::Decision(value) => json!(match value {
                        thinkthen::Answer::Yes => Some(true),
                        thinkthen::Answer::No => Some(false),
                        thinkthen::Answer::Unsure => None,
                    }),
                    Judgment::Choice(value) => json!(value),
                    Judgment::Score(value) => json!(value),
                    Judgment::Tags(value) => json!(value),
                },
            );
        }
        if let Some(failure) = detail.failure() {
            put(
                &mut row,
                "failed",
                json!({"kind": "backend", "cause": cause(failure)}),
            );
        }
        if let Some(probabilities) = detail.probabilities() {
            put(&mut row, "probabilities", match probabilities {
                Probabilities::YesNo { yes } => json!({"yes": yes}),
                Probabilities::Named(values) => json!(values.iter().map(|value|
                    json!({"name": value.name(), "probability": value.probability()})).collect::<Vec<_>>()),
            });
        }
        if let Some(confidence) = detail.confidence() {
            put(&mut row, "confidence", json!(confidence));
        }
        if let Some(usage) = detail.usage() {
            put(
                &mut row,
                "usage",
                json!({"input_tokens": usage.input_tokens(), "output_tokens": usage.output_tokens()}),
            );
        }
        if !detail.model().is_empty() {
            put(&mut row, "model", json!(detail.model()));
        }
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

#[derive(Clone, Debug)]
pub(crate) struct OwnedFacts {
    pub(crate) records: u64,
    pub(crate) requests_sent: u64,
    pub(crate) cache_answers: u64,
    pub(crate) input_tokens: Option<u64>,
    pub(crate) output_tokens: Option<u64>,
    pub(crate) seconds: f64,
    pub(crate) model: Option<String>,
    model_conflict: bool,
}

impl From<&Facts> for OwnedFacts {
    fn from(facts: &Facts) -> Self {
        Self {
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
    facts: Py<PyAny>,
    details: Py<PyAny>,
}

#[pyclass(frozen, name = "Facts", module = "thinkthen._thinkthen")]
pub(crate) struct PyFacts(OwnedFacts);

#[pymethods]
impl PyFacts {
    #[getter]
    fn records(&self) -> u64 {
        self.0.records
    }
    #[getter]
    fn requests_sent(&self) -> u64 {
        self.0.requests_sent
    }
    #[getter]
    fn cache_answers(&self) -> u64 {
        self.0.cache_answers
    }
    #[getter]
    fn input_tokens(&self) -> Option<u64> {
        self.0.input_tokens
    }
    #[getter]
    fn output_tokens(&self) -> Option<u64> {
        self.0.output_tokens
    }
    #[getter]
    fn seconds(&self) -> f64 {
        self.0.seconds
    }
    #[getter]
    fn model(&self) -> Option<&str> {
        self.0.model.as_deref()
    }
}

#[pymethods]
impl PyCall {
    #[getter]
    fn value(&self, py: Python<'_>) -> Py<PyAny> {
        self.value.clone_ref(py)
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

pub(crate) fn python_owned_facts(py: Python<'_>, facts: &OwnedFacts) -> PyResult<Py<PyAny>> {
    Ok(Py::new(py, PyFacts(facts.clone()))?.into_any())
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
