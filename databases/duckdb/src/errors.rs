//! The one error-kind table and the one panic guard at this binding's edge
//! (ADR 0047 item 7, error-index rows R1-31 and R2-31).

use std::panic::{AssertUnwindSafe, catch_unwind};

use thinkthen::{Error, ErrorKind};

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

/// Run one callback body so a panic becomes that callback's error and never
/// unwinds into DuckDB. Every callback the C API calls runs through here.
pub(crate) fn guarded<T>(
    what: &str,
    body: impl FnOnce() -> Result<T, String>,
) -> Result<T, String> {
    catch_unwind(AssertUnwindSafe(|| {
        test_panic(what);
        body()
    }))
    .unwrap_or_else(|payload| {
        let said = payload
            .downcast_ref::<&str>()
            .map(|text| (*text).to_owned())
            .or_else(|| payload.downcast_ref::<String>().cloned())
            .unwrap_or_default();
        Err(defect(&format!("the {what} callback panicked: {said}")))
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
            Err("thinkthen defect: the unit callback panicked: planted".to_owned())
        );
    }
}
