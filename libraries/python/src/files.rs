//! Explicit file selections through the same located dispatch as the C interface.

use crate::{guard, raised};
use pyo3::prelude::*;

use std::path::PathBuf;

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
pub(crate) fn _read_files(
    py: Python<'_>,
    paths: Vec<PathBuf>,
    unit: &str,
    window: Option<usize>,
) -> PyResult<SourceIterator> {
    guard(py, || {
        let reading = crate::inputs::reading(py, unit, window)?;
        let records = py
            .detach(|| thinkthen::read_files(paths, reading))
            .map_err(|e| raised(py, &e))?;
        Ok(SourceIterator(std::sync::Mutex::new(records)))
    })
}

#[pyfunction]
pub(crate) fn _validate_reading(
    py: Python<'_>,
    unit: &Bound<'_, PyAny>,
    window: &Bound<'_, PyAny>,
) -> PyResult<()> {
    let invalid = || {
        crate::usage(
            py,
            "reader window requires unit window and a positive whole number",
        )
    };
    let unit: String = unit.extract().map_err(|_| invalid())?;
    let window = if window.is_none() {
        None
    } else {
        crate::controls::whole_number(py, window).map_err(|_| invalid())?;
        Some(window.extract::<usize>().map_err(|_| invalid())?)
    };
    crate::inputs::reading(py, &unit, window)?
        .validate()
        .map(|_| ())
        .map_err(|e| raised(py, &e))
}
