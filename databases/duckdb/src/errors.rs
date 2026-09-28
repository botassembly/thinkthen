//! The one error-kind table and the one panic guard at this binding's edge
//! (ADR 0047 item 7, error-index rows R1-31 and R2-31).

use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Once;

use thinkthen::{Error, ErrorKind};

thread_local! {
    static DUCKDB_DEPTH: Cell<usize> = const { Cell::new(0) };
}

static DUCKDB_HOOK: Once = Once::new();

fn install_hook() {
    DUCKDB_HOOK.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if !DUCKDB_DEPTH
                .try_with(|depth| depth.get() != 0)
                .unwrap_or(false)
            {
                previous(info);
            }
        }));
    });
}

struct DuckdbDepth(usize);

impl Drop for DuckdbDepth {
    fn drop(&mut self) {
        DUCKDB_DEPTH.with(|depth| depth.set(self.0));
    }
}

fn in_duckdb<T>(body: impl FnOnce() -> T) -> T {
    install_hook();
    let prior = DUCKDB_DEPTH.with(|depth| {
        let prior = depth.get();
        depth.set(prior.saturating_add(1));
        prior
    });
    let _restore = DuckdbDepth(prior);
    body()
}

/// An engine error as the SQL error a reader sees: `thinkthen <kind>: `,
/// then the engine's own message, with the retry signal carried.
pub(crate) fn failure(error: &Error) -> String {
    let retry = if error.retryable() {
        " (a second try could help)"
    } else {
        ""
    };
    format!(
        "{}{retry}{}",
        prefix(error.kind()),
        error.detail().message()
    )
}

/// The prefix every failure of one kind carries.
pub(crate) const fn prefix(kind: ErrorKind) -> &'static str {
    match kind {
        ErrorKind::Usage => "thinkthen usage: ",
        ErrorKind::Backend => "thinkthen backend: ",
        ErrorKind::Local => "thinkthen local: ",
        ErrorKind::Cancelled => "thinkthen cancelled: ",
        ErrorKind::Deadline => "thinkthen deadline: ",
        ErrorKind::Defect => "thinkthen defect: ",
    }
}

/// A usage failure this binding raises itself.
pub(crate) fn usage(message: &str) -> String {
    format!("{}{message}", prefix(ErrorKind::Usage))
}

/// A defect this binding raises itself.
pub(crate) fn defect(message: &str) -> String {
    format!("{}{message}", prefix(ErrorKind::Defect))
}

/// Typed scalar failure. Its diagnostic stays on the raising path only.
#[derive(Debug)]
pub(crate) struct RowError {
    pub(crate) kind: ErrorKind,
    pub(crate) retryable: bool,
    pub(crate) text: String,
}

impl RowError {
    pub(crate) fn of(kind: ErrorKind, message: &str) -> Self {
        Self {
            kind,
            retryable: false,
            text: format!("{}{message}", prefix(kind)),
        }
    }

    pub(crate) fn usage(message: &str) -> Self {
        Self::of(ErrorKind::Usage, message)
    }
    pub(crate) fn local(message: &str) -> Self {
        Self::of(ErrorKind::Local, message)
    }
    pub(crate) fn defect(message: &str) -> Self {
        Self::of(ErrorKind::Defect, message)
    }

    /// A fixed defect already formatted by this binding's callback guard.
    pub(crate) fn caught_defect(text: String) -> Self {
        Self {
            kind: ErrorKind::Defect,
            retryable: false,
            text,
        }
    }

    pub(crate) fn recoverable(&self) -> bool {
        matches!(
            self.kind,
            ErrorKind::Usage | ErrorKind::Local | ErrorKind::Backend
        )
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

impl From<Error> for RowError {
    fn from(error: Error) -> Self {
        Self {
            kind: error.kind(),
            retryable: error.retryable(),
            text: failure(&error),
        }
    }
}

/// Run one callback body so a panic becomes that callback's error and never
/// unwinds into DuckDB. Every callback the C API calls runs through here.
pub(crate) fn guarded<T>(
    what: &str,
    body: impl FnOnce() -> Result<T, String>,
) -> Result<T, String> {
    in_duckdb(|| {
        catch_unwind(AssertUnwindSafe(|| {
            test_panic(what);
            body()
        }))
        .unwrap_or_else(|payload| {
            std::mem::forget(payload);
            Err(defect(&format!("the {what} callback panicked")))
        })
    })
}

/// The R1-10 door: in a `test-hooks` build, the environment variable
/// `thinkthen_test_hook_panic` names one boundary to panic in.
#[cfg(feature = "test-hooks")]
#[expect(
    clippy::panic,
    reason = "the test hook exists to panic inside a boundary"
)]
fn test_panic(what: &str) {
    if std::env::var("thinkthen_test_hook_panic").is_ok_and(|named| named == what) {
        panic!("thinkthen_test_hook_panic fired in {what}");
    }
}

#[cfg(not(feature = "test-hooks"))]
const fn test_panic(_: &str) {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    const CHILD: &str = "THINKTHEN_TEST_DUCKDB_PANIC_CHILD";
    const STRING_MARKER: &str = "duckdb-owned-string-payload-marker";
    const DROP_MARKER: &str = "duckdb-owned-drop-payload-marker";

    struct Exploding;

    impl Drop for Exploding {
        fn drop(&mut self) {
            panic!("{DROP_MARKER}");
        }
    }

    /// R1-31: every kind maps to its own word, and the table matches the
    /// engine's names.
    #[test]
    fn every_kind_maps_to_its_word() {
        for kind in [
            ErrorKind::Usage,
            ErrorKind::Backend,
            ErrorKind::Local,
            ErrorKind::Cancelled,
            ErrorKind::Deadline,
            ErrorKind::Defect,
        ] {
            assert_eq!(prefix(kind), format!("thinkthen {}: ", kind.name()));
        }
    }

    /// The guard turns a panic into this callback's defect.
    #[test]
    fn a_panic_becomes_the_callbacks_defect() {
        let caught: Result<(), String> = guarded("unit", || panic!("planted"));
        assert_eq!(
            caught,
            Err("thinkthen defect: the unit callback panicked".to_owned())
        );
    }

    #[test]
    fn callback_and_worker_payloads_stay_out_of_diagnostics() {
        if std::env::var_os(CHILD).is_some() {
            native_panic_child();
        } else {
            let output = std::process::Command::new(std::env::current_exe().expect("test binary"))
                .env_clear()
                .args([
                    "--exact",
                    "errors::tests::callback_and_worker_payloads_stay_out_of_diagnostics",
                    "--nocapture",
                ])
                .env(CHILD, "1")
                .output()
                .expect("isolated DuckDB proof");
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stdout)
            );
            for stream in [&output.stdout, &output.stderr] {
                let text = String::from_utf8_lossy(stream);
                assert!(!text.contains(STRING_MARKER), "{text}");
                assert!(!text.contains(DROP_MARKER), "{text}");
            }
            assert_eq!(
                String::from_utf8_lossy(&output.stderr),
                "host-thread-marker\n"
            );
        }
    }

    fn native_panic_child() {
        std::panic::set_hook(Box::new(|info| {
            let text = info
                .payload()
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| info.payload().downcast_ref::<String>().map(String::as_str))
                .unwrap_or(if info.payload().is::<Exploding>() {
                    DROP_MARKER
                } else {
                    "other panic"
                });
            let _ = std::io::stderr().write_all(format!("{text}\n").as_bytes());
        }));
        let callback: Result<(), String> = guarded("scalar", || panic!("{STRING_MARKER}"));
        assert_eq!(
            callback,
            Err("thinkthen defect: the scalar callback panicked".to_owned())
        );
        let invoke = crate::signal::Invoke::begin();
        let failure = crate::worker::run_typed(&invoke, |_| -> Result<(), Error> {
            std::panic::panic_any(Exploding)
        })
        .expect_err("caught worker");
        assert_eq!(
            (failure.kind, failure.retryable),
            (ErrorKind::Defect, false)
        );
        assert_eq!(
            failure.text,
            "thinkthen defect: the engine worker callback panicked"
        );
        let later = crate::worker::run_typed(&invoke, |_| Ok::<_, Error>(7));
        assert_eq!(later.ok(), Some(7));
        let _ = std::thread::spawn(|| panic!("host-thread-marker")).join();
    }
}
