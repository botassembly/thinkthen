//! Suppress this binding's owned panic text while preserving host callbacks.

use std::cell::Cell;
use std::ops::{Deref, DerefMut};
use std::sync::Once;

use pyo3::{PyErr, PyResult};

thread_local! {
    static DEPTH: Cell<usize> = const { Cell::new(0) };
}

static HOOK: Once = Once::new();

fn install() {
    HOOK.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if !DEPTH.try_with(|depth| depth.get() != 0).unwrap_or(false) {
                previous(info);
            }
        }));
    });
}

struct Restore(usize);

impl Drop for Restore {
    fn drop(&mut self) {
        let _ = DEPTH.try_with(|depth| depth.set(self.0));
    }
}

/// Mark only work this binding owns on the current thread.
pub(crate) fn owned<T>(body: impl FnOnce() -> T) -> T {
    install();
    let prior = DEPTH.with(|depth| {
        let prior = depth.get();
        depth.set(prior.saturating_add(1));
        prior
    });
    let _restore = Restore(prior);
    body()
}

/// Let an interpreter or foreign producer run under the previous hook.
pub(crate) fn host<T>(body: impl FnOnce() -> T) -> T {
    let prior = DEPTH.with(|depth| depth.replace(0));
    let _restore = Restore(prior);
    body()
}

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
