//! The explicit Python carrier for the core's shared call tally.

use pyo3::prelude::*;
use thinkthen::Tally;

use crate::result::python_facts;

#[pyclass(frozen, name = "Tally", module = "thinkthen._thinkthen")]
#[derive(Debug, Default)]
pub(crate) struct PyTally(pub(crate) Tally);

#[pymethods]
impl PyTally {
    #[new]
    fn new() -> Self {
        Self(Tally::new())
    }

    #[getter]
    fn facts(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        python_facts(py, &self.0.facts())
    }
}
