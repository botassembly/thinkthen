//! Safe defaults for a bare `cargo test`.
//!
//! Every binary here is run by `check.sh` with `ENGINE_NULL=1` (the null
//! backend) or with `ENGINE_BASE_URL` (the wire stub). A bare
//! `cargo test` names neither, and the crate forbids `unsafe`, so a test
//! cannot set the environment itself; it reports the missing environment
//! once and passes, leaving the null and wire runs to `check.sh`.

use std::sync::Once;

static NOTED: Once = Once::new();

/// Whether an engine can be built: the null backend or a wire address.
#[must_use]
pub(crate) fn engine_env_is_set() -> bool {
    std::env::var_os("ENGINE_NULL").is_some()
        || std::env::var_os("ENGINE_BASE_URL").is_some()
        || std::env::var_os("THINKTHEN_BASE_URL").is_some()
}

/// The guard a test starts with. False means the environment names no
/// backend, so the test reports the skip and returns; the reason prints
/// once a process and the skip per test, so nothing is silent.
#[must_use]
pub(crate) fn note_missing_env(test: &str) -> bool {
    if engine_env_is_set() {
        return true;
    }
    NOTED.call_once(|| {
        eprintln!(
            "note: neither ENGINE_NULL nor a wire address is set; \
             run ./check.sh for the null and wire suites"
        );
    });
    eprintln!("skip {test}: set ENGINE_NULL=1 (null backend) or ENGINE_BASE_URL (wire stub)");
    false
}
