//! Shared native test helpers for the Windows library unit tests.

pub(crate) use crate::test_deadline::child;

#[path = "../../tests/windows/ffi.rs"]
pub(crate) mod ffi;
#[path = "../../tests/windows/process.rs"]
#[allow(
    clippy::expect_used,
    clippy::panic,
    reason = "owned subprocess fixtures stop the proof on a broken test precondition"
)]
pub(crate) mod process;
