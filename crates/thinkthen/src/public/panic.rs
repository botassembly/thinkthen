//! The one panic guard every binding boundary uses.

use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::engine::workers;

/// Run `body` and return `None` if it panics, keeping the payload out of
/// every output.
///
/// A panic inside `body` on this thread skips the panic hook, so its message
/// reaches no output. The payload is forgotten inside the guard without
/// running its destructor, so a payload whose drop panics again stays silent
/// too. A panic on any other thread still reaches the host's hook.
///
/// The first call installs one process hook that wraps the hook in place at
/// that moment. Limits:
///
/// - A host that later replaces the hook displaces this one, and panics in
///   `body` then reach the host's new hook.
/// - A thread past its local-storage teardown cannot be marked. `body` still
///   runs and a panic is still caught, but the hook reports it.
/// - A first call from a thread that is already panicking cannot install
///   the hook, and the process aborts.
pub fn contained<T>(body: impl FnOnce() -> T) -> Option<T> {
    catch_unwind(AssertUnwindSafe(|| {
        workers::with_engine_diagnostics(|| match catch_unwind(AssertUnwindSafe(body)) {
            Ok(value) => Some(value),
            Err(payload) => {
                std::mem::forget(payload);
                None
            }
        })
    }))
    .unwrap_or_else(|payload| {
        std::mem::forget(payload);
        None
    })
}

/// Run host code inside [`contained`] work under the host's own panic hook.
///
/// Use it for a host callback, or to drop a host-owned value, so the host's
/// diagnostics still report the host's own panics.
pub fn uncontained<T>(body: impl FnOnce() -> T) -> T {
    workers::with_host_diagnostics(body)
}
