//! Run host-owned work under the host's own panic hook inside the binding's
//! guard, which is `thinkthen::contained`.

use std::ops::{Deref, DerefMut};

use pyo3::{PyErr, PyResult};

/// Let an interpreter or foreign producer run under the previous hook.
pub(crate) use thinkthen::uncontained as host;

/// Discard a host exception under the host hook before building our refusal.
pub(crate) fn host_error<T>(result: PyResult<T>, refusal: impl FnOnce() -> PyErr) -> PyResult<T> {
    result.map_err(|error| {
        host(|| drop(error));
        refusal()
    })
}

/// Keep a Python-owned result of a host callout unmarked through its final
/// reference release, including an early return from the binding reader.
pub(crate) struct HostOwned<T>(Option<T>);

pub(crate) const fn host_owned<T>(value: T) -> HostOwned<T> {
    HostOwned(Some(value))
}

impl<T> Deref for HostOwned<T> {
    type Target = T;

    #[allow(
        clippy::expect_used,
        reason = "the value is taken only in Drop, after all borrows end"
    )]
    fn deref(&self) -> &T {
        self.0.as_ref().expect("host value remains owned")
    }
}

impl<T> DerefMut for HostOwned<T> {
    #[allow(
        clippy::expect_used,
        reason = "the value is taken only in Drop, after all borrows end"
    )]
    fn deref_mut(&mut self) -> &mut T {
        self.0.as_mut().expect("host value remains owned")
    }
}

impl<T> Drop for HostOwned<T> {
    fn drop(&mut self) {
        if let Some(value) = self.0.take() {
            host(|| drop(value));
        }
    }
}
