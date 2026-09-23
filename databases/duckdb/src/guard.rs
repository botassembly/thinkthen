//! Panic containment for the hand-written C callbacks.
//!
//! A Rust panic must not unwind across a C boundary: since Rust 1.71 an
//! unwind out of an `extern "C"` function aborts the process, and the
//! host that dies is the database, not this library. The scalar
//! functions are registered through `duckdb-rs`, whose registration
//! helpers already contain panics; the callbacks this surface hands the
//! raw C API directly — the entrypoint, relate's bind, init, and scan,
//! the warm aggregate's callbacks, usage's table function, and every
//! destructor — do not pass through that machinery, so every one of them
//! runs inside a function here.
//!
//! Where a callback has DuckDB's own error channel, a contained panic
//! becomes the query's error; where it has none (an aggregate update, a
//! destructor), it is contained and logged, because an abort is worse
//! than a missing row.

use std::panic::{AssertUnwindSafe, catch_unwind};
#[cfg(feature = "test-panic")]
use std::sync::OnceLock;

use thinkthen_contract::panic_text;

/// Run a callback body that reports through DuckDB, so a panic becomes
/// that error instead of an abort.
pub(crate) fn contained(
    what: &str,
    body: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    caught(what, body).and_then(|outcome| outcome)
}

/// Run a callback body with no error channel: a panic is contained and
/// logged, never unwound.
pub(crate) fn contained_quiet(what: &str, body: impl FnOnce()) {
    contained_with(what, (), body);
}

/// Run a value-returning callback body with no error channel, answering
/// `fallback` when it panics.
pub(crate) fn contained_with<T>(what: &str, fallback: T, body: impl FnOnce() -> T) -> T {
    caught(what, body).unwrap_or_else(|message| {
        eprintln!("{message}");
        fallback
    })
}

/// The one catch behind the three forms above (surfaces-review-7 R2-31:
/// each form carried its own copy). A panic becomes the message every
/// channel carries, through the contract's one panic-to-text.
fn caught<T>(what: &str, body: impl FnOnce() -> T) -> Result<T, String> {
    catch_unwind(AssertUnwindSafe(|| {
        test_arm(what);
        body()
    }))
    .map_err(|payload| {
        format!("thinkthen defect: the {what} callback panicked: {}", panic_text(payload.as_ref()))
    })
}

/// The test-only arm: `ENGINE_TEST_PANIC` names one boundary to panic
/// in, so the surface check proves a panic inside a callback becomes a
/// query error and a live process. Compile-time gated: a shipped build
/// carries no arm at all, so no environment variable can make a
/// boundary panic (review 4).
#[cfg(feature = "test-panic")]
pub(crate) fn test_arm(what: &str) {
    static ARMED: OnceLock<Option<String>> = OnceLock::new();
    let armed = ARMED.get_or_init(|| std::env::var("ENGINE_TEST_PANIC").ok());
    if armed.as_deref() == Some(what) || armed.as_deref() == Some("*") {
        panic!("the test arm fired for {what}");
    }
}

#[cfg(not(feature = "test-panic"))]
pub(crate) fn test_arm(_what: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    /// The containment contract: a panic becomes the caller's error, not
    /// an unwind. Before the containment every one of these unwound and
    /// the process would have aborted at the C boundary.
    #[test]
    fn a_panic_becomes_the_callbacks_error() {
        let outcome = contained("unit test", || panic!("the fixture panic"));
        let message = outcome.expect_err("a panicking callback returns the defect");
        assert!(message.contains("the unit test callback panicked"), "{message}");
        assert!(message.contains("the fixture panic"), "{message}");
        assert!(message.contains("thinkthen defect:"), "{message}");
    }

    /// A body that reports outright keeps its own message.
    #[test]
    fn a_reported_failure_passes_through() {
        let outcome = contained("unit test", || Err("thinkthen usage: the shape is wrong".into()));
        assert_eq!(outcome.expect_err("the failure passes"), "thinkthen usage: the shape is wrong");
    }

    /// The quiet form contains the panic and returns, and the fallback
    /// form answers the fallback.
    #[test]
    fn the_quiet_and_value_forms_contain() {
        contained_quiet("unit quiet", || panic!("quiet"));
        let answered = contained_with("unit value", 7_u64, || panic!("value"));
        assert_eq!(answered, 7);
        let passed = contained_with("unit value", 7_u64, || 9);
        assert_eq!(passed, 9);
    }
}
