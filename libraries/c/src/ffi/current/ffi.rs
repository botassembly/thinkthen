//! Counted question admission and owned author metadata.
#![allow(
    unsafe_code,
    reason = "the reviewed C lifetime contract permits foreign pointer access"
)]
#![deny(unsafe_op_in_unsafe_fn)]
#[path = "author/ffi.rs"]
pub(super) mod author;
#[path = "operations_generated.rs"]
mod operations;
#[path = "read/ffi.rs"]
pub(crate) mod read;
use crate::failures::guard;
pub use operations::*;
