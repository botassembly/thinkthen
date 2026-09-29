//! The SQLite surface of thinkthen: eight judgment functions, four engine
//! settings, and two table-valued functions over the public Rust API.
//!
//! `.load ./thinkthen` registers every function and sends nothing. The one
//! process engine is built on the first call that can send. Every call that
//! can send runs on a worker thread, so the calling thread hears SQLite's
//! interrupt while a request is out (`worker`). Every function is volatile
//! and direct-only, so a schema in a database the host has not vouched for
//! cannot spend money or read files. That promise needs SQLite 3.50.0 or
//! newer, and the load refuses an older host by name (`ffi`).

use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Once;

use rusqlite::ffi::{
    self as sqlite, SQLITE_CANTOPEN, SQLITE_CONSTRAINT, SQLITE_ERROR, SQLITE_INTERRUPT,
};
use thinkthen::{ErrorKind, SendBudgetDenial};

mod budget;
#[allow(
    unsafe_code,
    reason = "the SQLite entry point, API table, and virtual-table glue (ADR 0047 item 3)"
)]
mod ffi;
mod question;
mod recognize_document;
mod scalars;
mod settings;
mod tables;
mod worker;

pub use ffi::sqlite3_thinkthen_init;

thread_local! {
    static SQLITE_DEPTH: Cell<usize> = const { Cell::new(0) };
}

static SQLITE_HOOK: Once = Once::new();

fn install_hook() {
    SQLITE_HOOK.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if !SQLITE_DEPTH
                .try_with(|depth| depth.get() != 0)
                .unwrap_or(false)
            {
                previous(info);
            }
        }));
    });
}

struct SqliteDepth(usize);

impl Drop for SqliteDepth {
    fn drop(&mut self) {
        SQLITE_DEPTH.with(|depth| depth.set(self.0));
    }
}

pub(crate) fn in_sqlite_diagnostics<T>(body: impl FnOnce() -> T) -> T {
    install_hook();
    let prior = SQLITE_DEPTH.with(|depth| {
        let prior = depth.get();
        depth.set(prior.saturating_add(1));
        prior
    });
    let _restore = SqliteDepth(prior);
    body()
}

/// One failure on its way to SQLite: a kind, a safe message, and whether the
/// same call may pass later.
#[derive(Debug)]
pub(crate) struct Failure {
    kind: ErrorKind,
    message: String,
    retryable: bool,
}

impl Failure {
    /// A failure of this kind that no retry fixes.
    pub(crate) fn of(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            retryable: false,
        }
    }

    /// A call SQL cannot make as given.
    pub(crate) fn usage(message: impl Into<String>) -> Self {
        Self::of(ErrorKind::Usage, message)
    }

    /// A fault inside this binding.
    pub(crate) fn defect(message: impl Into<String>) -> Self {
        Self::of(ErrorKind::Defect, message)
    }

    pub(crate) fn value(&self) -> Option<serde_json::Value> {
        let message = match self.kind {
            ErrorKind::Usage => {
                "check the row's question and arguments, or raise the process request total when it is spent"
            }
            ErrorKind::Local => "check the named file and its permissions",
            ErrorKind::Backend => "the backend did not answer; retry if allowed",
            ErrorKind::Cancelled | ErrorKind::Deadline | ErrorKind::Defect => return None,
        };
        Some(
            serde_json::json!({"status":"failed","error":{"kind":self.kind.name(),"message":message,"retryable":self.retryable}}),
        )
    }
}

impl From<thinkthen::Error> for Failure {
    fn from(error: thinkthen::Error) -> Self {
        if matches!(
            error.send_budget_denial(),
            Some(
                SendBudgetDenial::BeforeFirstSend
                    | SendBudgetDenial::BeforeAdditionalSend
                    | SendBudgetDenial::BeforeRetry { .. }
            )
        ) {
            return settings::spent(settings::send_budget().1.unwrap_or(0));
        }
        Self {
            kind: error.kind(),
            message: error.detail().message().to_owned(),
            retryable: error.retryable(),
        }
    }
}

/// The one kind table (R1-31): the message prefix and SQLite's result code.
fn code_of(kind: ErrorKind) -> i32 {
    match kind {
        ErrorKind::Cancelled => SQLITE_INTERRUPT,
        ErrorKind::Usage => SQLITE_CONSTRAINT,
        ErrorKind::Local => SQLITE_CANTOPEN,
        ErrorKind::Backend | ErrorKind::Deadline | ErrorKind::Defect => SQLITE_ERROR,
    }
}

impl From<Failure> for rusqlite::Error {
    fn from(failure: Failure) -> Self {
        let retry = if failure.retryable {
            " (retryable)"
        } else {
            ""
        };
        let message = format!(
            "thinkthen {}{retry}: {}",
            failure.kind.name(),
            failure.message
        );
        Self::SqliteFailure(sqlite::Error::new(code_of(failure.kind)), Some(message))
    }
}

/// Run one function body, virtual-table callback, or worker body, and turn a
/// panic in this binding into `defect` (R2-31: the one guard).
pub(crate) fn guard<T>(
    _what: &str,
    body: impl FnOnce() -> Result<T, Failure>,
) -> Result<T, Failure> {
    in_sqlite_diagnostics(|| {
        catch_unwind(AssertUnwindSafe(body)).unwrap_or_else(|payload| {
            std::mem::forget(payload);
            Err(Failure::defect("a panic crossed the SQLite boundary"))
        })
    })
}

#[cfg(test)]
mod tests {
    use super::{Failure, guard};
    use rusqlite::ffi::{SQLITE_CANTOPEN, SQLITE_CONSTRAINT, SQLITE_ERROR, SQLITE_INTERRUPT};
    use thinkthen::ErrorKind;

    fn sqlite(failure: Failure) -> (i32, String) {
        match rusqlite::Error::from(failure) {
            rusqlite::Error::SqliteFailure(error, Some(message)) => (error.extended_code, message),
            other => (0, other.to_string()),
        }
    }

    /// R1-31: each kind keeps its prefix and SQLite result code.
    #[test]
    fn each_kind_maps_to_its_prefix_and_code() {
        let table = [
            (ErrorKind::Usage, SQLITE_CONSTRAINT, "thinkthen usage: why"),
            (ErrorKind::Backend, SQLITE_ERROR, "thinkthen backend: why"),
            (ErrorKind::Local, SQLITE_CANTOPEN, "thinkthen local: why"),
            (
                ErrorKind::Cancelled,
                SQLITE_INTERRUPT,
                "thinkthen cancelled: why",
            ),
            (ErrorKind::Deadline, SQLITE_ERROR, "thinkthen deadline: why"),
            (ErrorKind::Defect, SQLITE_ERROR, "thinkthen defect: why"),
        ];
        for (kind, code, message) in table {
            assert_eq!(sqlite(Failure::of(kind, "why")), (code, message.to_owned()));
        }
        let mut busy = Failure::of(ErrorKind::Backend, "status 503");
        busy.retryable = true;
        assert_eq!(sqlite(busy).1, "thinkthen backend (retryable): status 503");
    }

    /// R1-10 host half: a panic is a fixed defect, and the next call answers.
    #[test]
    fn a_panic_is_a_defect_and_the_next_call_answers() {
        let held = guard("thinkthen_probe", || -> Result<(), Failure> {
            panic!("the probe blew up")
        });
        let message = held.err().map(|failure| sqlite(failure).1);
        assert_eq!(
            message.as_deref(),
            Some("thinkthen defect: a panic crossed the SQLite boundary")
        );
        assert_eq!(guard("thinkthen_probe", || Ok(7)).ok(), Some(7));
    }
}
