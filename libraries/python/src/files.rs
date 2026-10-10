//! Explicit file selections through the same located dispatch as the C door.

use crate::{guard, raised};
use pyo3::prelude::*;

use thinkthen_host::source;

#[pyclass(name = "_SourceIterator", module = "thinkthen._thinkthen")]
pub(crate) struct SourceIterator(std::sync::Mutex<thinkthen::SourceRecords>);

impl std::fmt::Debug for SourceIterator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SourceIterator(<withheld>)")
    }
}

#[pymethods]
impl SourceIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self, py: Python<'_>) -> PyResult<Option<String>> {
        guard(py, || {
            let row = py.detach(|| {
                self.0
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .next()
            });
            row.map(|row| {
                let row = row.map_err(|e| raised(py, &e))?;
                serde_json::to_string(&row)
                    .map_err(|_| crate::defect(py, "source record could not be written"))
            })
            .transpose()
        })
    }
}

#[pyfunction]
pub(crate) fn _read_files(py: Python<'_>, selection: &str) -> PyResult<SourceIterator> {
    guard(py, || {
        let selection = source::parse(selection).map_err(|e| raised(py, &e))?;
        let records = py.detach(|| selection.read()).map_err(|e| raised(py, &e))?;
        Ok(SourceIterator(std::sync::Mutex::new(records)))
    })
}
