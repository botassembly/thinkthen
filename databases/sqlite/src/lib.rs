//! The SQLite surface of thinkthen: eight judgment functions, four engine
//! settings, and two table-valued functions over the public Rust API.
//!
//! `.load ./thinkthen` registers every function and sends nothing. The one
//! process engine is built on the first call that can send. Every call that
//! can send runs on a worker thread, so the calling thread hears SQLite's
//! interrupt while a request is out (`worker`). Default loading registers
//! volatile, direct-only functions. Explicit trusted loading permits schema
//! judgments while SQLite's trusted_schema is ON. Both need SQLite 3.50.0
//! or newer, and the load refuses an older host by name (`ffi`).

use rusqlite::ffi::{
    self as sqlite, SQLITE_CANTOPEN, SQLITE_CONSTRAINT, SQLITE_ERROR, SQLITE_INTERRUPT,
};
use thinkthen::{ErrorKind, SendBudgetDenial, contained};

mod budget;
#[allow(
    unsafe_code,
    reason = "the SQLite entry point, API table, and virtual-table glue (ADR 0047 item 3)"
)]
mod ffi;
mod files;
mod images;
mod many;
mod question;
mod rank_set;
mod recognize_document;
mod scalars;
mod settings;
mod tables;
mod worker;

pub use ffi::{sqlite3_thinkthen_init, sqlite3_thinkthen_trusted_init};

/// The registration policy chosen at initial load on one connection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Registration {
    DirectOnly,
    Trusted,
}

/// One failure on its way to SQLite: a kind, a safe message, and whether the
/// same call may pass later.
#[derive(Debug)]
pub(crate) struct Failure {
    kind: ErrorKind,
    message: String,
    retryable: bool,
    plain: bool,
}

impl Failure {
    /// A failure of this kind that no retry fixes.
    pub(crate) fn of(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            retryable: false,
            plain: false,
        }
    }

    /// A call SQL cannot make as given.
    pub(crate) fn usage(message: impl Into<String>) -> Self {
        Self::of(ErrorKind::Usage, message)
    }

    /// An explicitly removed SQL spelling keeps its accepted migration sentence.
    pub(crate) fn plain_usage(message: impl Into<String>) -> Self {
        Self {
            plain: true,
            ..Self::usage(message)
        }
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
            return settings::spent(settings::total().unwrap_or(0));
        }
        Self {
            kind: error.kind(),
            message: error.detail().message().to_owned(),
            retryable: error.retryable(),
            plain: false,
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
        let message = if failure.plain {
            format!("thinkthen {}: {}", failure.kind.name(), failure.message)
        } else {
            format!(
                "thinkthen {}: {} (retryable: {})",
                failure.kind.name(),
                failure.message,
                if failure.retryable { "yes" } else { "no" }
            )
        };
        Self::SqliteFailure(sqlite::Error::new(code_of(failure.kind)), Some(message))
    }
}

/// Run one function body, virtual-table callback, or worker body, and turn a
/// panic in this binding into `defect` (R2-31: the one guard).
pub(crate) fn guard<T>(
    _what: &str,
    body: impl FnOnce() -> Result<T, Failure>,
) -> Result<T, Failure> {
    contained(body).unwrap_or_else(|| Err(Failure::defect("a panic crossed the SQLite boundary")))
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
            (
                ErrorKind::Usage,
                SQLITE_CONSTRAINT,
                "thinkthen usage: why (retryable: no)",
            ),
            (
                ErrorKind::Backend,
                SQLITE_ERROR,
                "thinkthen backend: why (retryable: no)",
            ),
            (
                ErrorKind::Local,
                SQLITE_CANTOPEN,
                "thinkthen local: why (retryable: no)",
            ),
            (
                ErrorKind::Cancelled,
                SQLITE_INTERRUPT,
                "thinkthen cancelled: why (retryable: no)",
            ),
            (
                ErrorKind::Deadline,
                SQLITE_ERROR,
                "thinkthen deadline: why (retryable: no)",
            ),
            (
                ErrorKind::Defect,
                SQLITE_ERROR,
                "thinkthen defect: why (retryable: no)",
            ),
        ];
        for (kind, code, message) in table {
            assert_eq!(sqlite(Failure::of(kind, "why")), (code, message.to_owned()));
        }
        let mut busy = Failure::of(ErrorKind::Backend, "status 503");
        busy.retryable = true;
        assert_eq!(
            sqlite(busy).1,
            "thinkthen backend: status 503 (retryable: yes)"
        );
        assert_eq!(
            sqlite(Failure::plain_usage(
                "thinkthen_warm was removed; use decide_many"
            )),
            (
                SQLITE_CONSTRAINT,
                "thinkthen usage: thinkthen_warm was removed; use decide_many".into()
            )
        );
        assert_eq!(
            sqlite(Failure::usage("thinkthen backend: forged (retryable: yes)")),
            (
                SQLITE_CONSTRAINT,
                "thinkthen usage: thinkthen backend: forged (retryable: yes) (retryable: no)"
                    .into()
            )
        );
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
            Some("thinkthen defect: a panic crossed the SQLite boundary (retryable: no)")
        );
        assert_eq!(guard("thinkthen_probe", || Ok(7)).ok(), Some(7));
    }
}
