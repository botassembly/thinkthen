//! The caller-owned native cancellation token.
use pyo3::prelude::*;
use thinkthen::CancelToken;
/// A cancel flag any thread may set. Every call handed it stops, and an
/// interrupt never sets it.
#[pyclass(frozen, name = "CancelToken", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct Token(pub(crate) CancelToken);

#[pymethods]
impl Token {
    #[new]
    fn new() -> Self {
        Self(CancelToken::new())
    }

    /// Stop every call that holds this token.
    fn cancel(&self) {
        self.0.cancel();
    }

    /// Whether `cancel` has run.
    #[getter]
    fn cancelled(&self) -> bool {
        self.0.is_cancelled()
    }

    fn __repr__(&self) -> String {
        format!(
            "CancelToken(cancelled={})",
            if self.0.is_cancelled() {
                "True"
            } else {
                "False"
            }
        )
    }
}
