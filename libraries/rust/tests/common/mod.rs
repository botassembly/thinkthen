//! The backend guard every armed test starts with.
//!
//! The suite runs through `./check.sh` with `ENGINE_NULL=1` (the null
//! backend) or a wire address. The crate forbids `unsafe`, so a test
//! cannot arm the backend itself. A test that names no backend fails with
//! the switch that arms it, so a bare `cargo test` never counts a test
//! that did not run as passed (R2-28). The backend question itself is the
//! stand-in's one `testkit` helper, so `THINKTHEN_NULL=1 cargo test` arms
//! the suite exactly like the gate does.

/// Fail the calling test unless the environment names a backend.
pub(crate) fn require_backend() {
    if let Some(note) = thinkthen_standin::testkit::skip_note("the rust surface") {
        panic!("{note}; ./check.sh arms it");
    }
}
