//! Convert explicit caller controls; native admission owns their meaning.
use crate::{inputs::decode, usage};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyInt};
use thinkthen::RequestOptions;
pub(crate) fn whole_number(py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<()> {
    if value.is_instance_of::<PyBool>() || !value.is_instance_of::<PyInt>() {
        return Err(usage(py, "invalid canonical request"));
    }
    Ok(())
}
pub(crate) fn options(py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<RequestOptions> {
    let values = value.cast::<PyDict>()?;
    for name in ["instructions", "entity_definition", "descriptions"] {
        if values.contains(name)? {
            return Err(usage(
                py,
                "recognize task keywords take an inline declaration",
            ));
        }
    }
    decode(py, value)
}
