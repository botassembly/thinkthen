//! Engine-independent owned Python results. Schema dispatch is generated.
use pyo3::exceptions::{PyAttributeError, PyKeyError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde_json::Value;

#[pyclass(
    frozen,
    subclass,
    name = "_NativeResult",
    module = "thinkthen._thinkthen"
)]
#[derive(Debug)]
pub(crate) struct NativeResult {
    kind: String,
    value: Value,
}

#[pymethods]
impl NativeResult {
    #[new]
    fn new(kind: String, json: &str) -> PyResult<Self> {
        let value = serde_json::from_str(json)
            .map_err(|_| PyTypeError::new_err("native result requires JSON"))?;
        Ok(Self { kind, value })
    }

    fn __getattr__(&self, py: Python<'_>, member: &str) -> PyResult<Py<PyAny>> {
        if self.value.get(member).is_none() {
            return Err(PyAttributeError::new_err(member.to_owned()));
        }
        self.__getitem__(py, member)
    }

    fn __getitem__(&self, py: Python<'_>, member: &str) -> PyResult<Py<PyAny>> {
        let value = self
            .value
            .get(member)
            .ok_or_else(|| PyKeyError::new_err(member.to_owned()))?;
        crate::results_generated::field(py, &self.kind, member, value)
    }

    fn __contains__(&self, member: &str) -> bool {
        self.value.get(member).is_some()
    }

    fn __len__(&self) -> usize {
        self.value.as_object().map_or(0, serde_json::Map::len)
    }

    fn keys(&self) -> Vec<String> {
        self.value
            .as_object()
            .map(|v| v.keys().cloned().collect())
            .unwrap_or_default()
    }

    fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        plain(py, &self.value)
    }

    fn __eq__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<bool> {
        let other = if other.is_instance_of::<NativeResult>() {
            other.call_method0("to_dict")?
        } else {
            other.clone()
        };
        self.to_dict(py)?.bind(py).eq(other)
    }

    fn __bool__(&self, py: Python<'_>) -> PyResult<bool> {
        if self.value.get("error").is_some() || self.value.get("failure").is_some() {
            return Err(PyTypeError::new_err(
                "a failed native result has no truth value",
            ));
        }
        match self.value.get("value") {
            Some(value) => crate::results_generated::field(py, &self.kind, "value", value)?
                .bind(py)
                .is_truthy(),
            None => Err(PyTypeError::new_err("this native result has no value")),
        }
    }

    fn __repr__(&self) -> String {
        format!("<NativeResult {}>", self.kind)
    }

    fn __reduce__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let restore = py
            .import("thinkthen._thinkthen")?
            .getattr("_restore_native_result")?;
        let json = self.value.to_string();
        Ok((restore, (&self.kind, json))
            .into_pyobject(py)?
            .into_any()
            .unbind())
    }
}

#[pyfunction]
pub(crate) fn _restore_native_result(
    py: Python<'_>,
    kind: &str,
    json: &str,
) -> PyResult<Py<PyAny>> {
    let value = serde_json::from_str(json)
        .map_err(|_| PyTypeError::new_err("invalid native result JSON"))?;
    crate::results_generated::convert(py, kind, &value)
}

pub(crate) fn object(py: Python<'_>, name: &str, kind: &str, value: &Value) -> PyResult<Py<PyAny>> {
    py.import("thinkthen._native_results")?
        .getattr(name)?
        .call1((kind, value.to_string()))
        .map(Bound::unbind)
}

pub(crate) fn plain(py: Python<'_>, value: &Value) -> PyResult<Py<PyAny>> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(v) => Ok(v.into_pyobject(py)?.to_owned().into_any().unbind()),
        Value::String(v) => Ok(v.into_pyobject(py)?.into_any().unbind()),
        Value::Number(v) => {
            if let Some(v) = v.as_i64() {
                return Ok(v.into_pyobject(py)?.into_any().unbind());
            }
            if let Some(v) = v.as_u64() {
                return Ok(v.into_pyobject(py)?.into_any().unbind());
            }
            Ok(v.as_f64()
                .ok_or_else(|| crate::defect(py, "native number cannot be represented"))?
                .into_pyobject(py)?
                .into_any()
                .unbind())
        }
        Value::Array(_) => array(py, value, |v| plain(py, v)),
        Value::Object(_) => mapping(py, value, |v| plain(py, v)),
    }
}

pub(crate) fn array(
    py: Python<'_>,
    value: &Value,
    convert: impl Fn(&Value) -> PyResult<Py<PyAny>>,
) -> PyResult<Py<PyAny>> {
    let items = value
        .as_array()
        .ok_or_else(|| crate::defect(py, "native result is not an array"))?;
    Ok(
        PyList::new(py, items.iter().map(convert).collect::<PyResult<Vec<_>>>()?)?
            .into_any()
            .unbind(),
    )
}

pub(crate) fn mapping(
    py: Python<'_>,
    value: &Value,
    convert: impl Fn(&Value) -> PyResult<Py<PyAny>>,
) -> PyResult<Py<PyAny>> {
    let items = value
        .as_object()
        .ok_or_else(|| crate::defect(py, "native result is not an object"))?;
    let result = PyDict::new(py);
    for (key, value) in items {
        result.set_item(key, convert(value)?)?;
    }
    Ok(result.into_any().unbind())
}
