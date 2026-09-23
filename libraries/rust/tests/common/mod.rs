//! Safe defaults for a bare `cargo test`.
//!
//! The suite runs through `./check.sh` with `ENGINE_NULL=1` (the null
//! backend) or a wire address. A bare `cargo test` names no backend, the
//! crate forbids `unsafe`, so a test cannot set the environment itself;
//! it prints the shared skip note and returns, leaving the armed runs to
//! `check.sh`. The backend question itself is the stand-in's one
//! `testkit` helper — the settled `THINKTHEN_` spellings and the
//! deprecated `ENGINE_` ones — so `THINKTHEN_NULL=1 cargo test` arms the
//! suite exactly like the gate does (third review: the private helper
//! knew only `ENGINE_`, and fifteen verb tests skipped silently inside a
//! green summary).

/// Whether an engine can be built: the shared testkit's answer.
#[must_use]
pub(crate) fn engine_env_is_set() -> bool {
    thinkthen_standin::testkit::backend_kind().is_some()
}

/// The guard a test starts with. False means the environment names no
/// backend, so the test prints the shared skip note and returns; the note
/// prints once a process and the skip per test, so nothing is silent.
#[must_use]
pub(crate) fn note_missing_env(test: &str) -> bool {
    if engine_env_is_set() {
        return true;
    }
    if let Some(note) = thinkthen_standin::testkit::skip_note(test) {
        eprintln!("skip {note}");
    }
    false
}
