//! Python advances a native complete batch without decoding known output fields.
use crate::engine::complete::stream::Session;
use crate::raised;
use pyo3::prelude::*;

#[pyclass(frozen, name = "_CompleteStream", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct CompleteStream(pub(crate) Session);
#[pymethods]
impl CompleteStream {
    fn _pull(&self, py: Python<'_>) -> PyResult<String> {
        self.0.advance().map_err(|e| raised(py, &e))?;
        loop {
            if let Err(error) = py.check_signals() {
                self.0.cancel();
                return Err(error);
            }
            if let Some(event) = py.detach(|| self.0.poll()).map_err(|e| raised(py, &e))? {
                return Ok(event);
            }
        }
    }
    fn close(&self, py: Python<'_>) {
        py.detach(|| self.0.close());
    }
    fn cancel(&self) {
        self.0.cancel();
    }
    fn __repr__(&self) -> &'static str {
        "<CompleteBatch>"
    }
}
