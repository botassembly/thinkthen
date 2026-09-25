//! The exit gate for a column's input batches (ticket 0106 change 6).
//!
//! Any producer's release may drop a Python reference, so the worker releases
//! a column's batches while attached to the interpreter. Attaching alone
//! cannot stop a freeze at exit: under `abi3-py310`, pyo3 compiles out its
//! finalizing check, so a worker could pass `try_attach` just before the
//! interpreter starts to exit and then wait on the lock for good (spike 255).
//!
//! At import the module registers an `atexit` hook, `_exit_gate`. The hook
//! sets `EXITING`, then waits, detached, for the write side of `GATE`, so it
//! waits for every attached release already under way. It drops the write
//! side at once. Each release tries the read side before it attaches, then
//! checks `EXITING`, and it attaches and releases while it still holds the
//! read side. It never waits for the read side: a release on a thread that
//! is attached could otherwise wait on the hook while the hook waits on it.
//! Only the hook takes the write side, so a busy gate means the interpreter
//! is exiting. A release that finds the gate busy or `EXITING` set, or whose
//! `try_attach` gives nothing, leaks the batches on purpose. Nothing here
//! panics.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{PoisonError, RwLock, TryLockError};

use pyo3::prelude::*;

static GATE: RwLock<()> = RwLock::new(());
static EXITING: AtomicBool = AtomicBool::new(false);

/// The `atexit` hook: close the gate, and wait for releases under way.
#[pyfunction]
pub(crate) fn _exit_gate(py: Python<'_>) {
    EXITING.store(true, Ordering::SeqCst);
    py.detach(|| drop(GATE.write().unwrap_or_else(PoisonError::into_inner)));
}

/// Run `release` attached to the interpreter behind the gate, or leak.
pub(super) fn release(release: impl FnOnce()) {
    trace::line("wake");
    let open = match GATE.try_read() {
        Ok(open) => Some(open),
        Err(TryLockError::Poisoned(open)) => Some(open.into_inner()),
        Err(TryLockError::WouldBlock) => None,
    };
    let outcome = if open.is_none() || EXITING.load(Ordering::SeqCst) {
        "leaked"
    } else if Python::try_attach(|_py| release()).is_some() {
        "released"
    } else {
        "leaked"
    };
    drop(open);
    trace::line(outcome);
    trace::line("done");
}

#[cfg(feature = "probe")]
pub(crate) use trace::_probe_trace;

/// The exit test's trace: one line per event, in a file `_probe_trace`
/// names. Each line goes out in one `write` call on a file opened for
/// append, so lines from two threads never run together. Built only under
/// `probe`, and nothing in it touches Python.
#[cfg(feature = "probe")]
mod trace {
    use std::io::Write as _;
    use std::path::PathBuf;
    use std::sync::{Mutex, PoisonError};

    use pyo3::prelude::*;

    static PATH: Mutex<Option<PathBuf>> = Mutex::new(None);

    /// Write each later release event to `path`.
    #[pyfunction]
    pub(crate) fn _probe_trace(path: PathBuf) {
        *PATH.lock().unwrap_or_else(PoisonError::into_inner) = Some(path);
    }

    pub(super) fn line(word: &str) {
        let path = PATH.lock().unwrap_or_else(PoisonError::into_inner).clone();
        if let Some(path) = path
            && let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
        {
            let _unwritten = file.write_all(format!("{word}\n").as_bytes());
        }
    }
}

#[cfg(not(feature = "probe"))]
mod trace {
    pub(super) const fn line(_word: &str) {}
}
