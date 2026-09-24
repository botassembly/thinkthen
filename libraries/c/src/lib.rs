//! The C door to the thinkthen engine (ticket 0094, ADR 0037).
//!
//! This crate builds `libthinkthen.so` and `libthinkthen.a` against
//! `include/thinkthen.h` and owns every exported symbol; `thinkthen` exports
//! none. It reaches the engine only through the public Rust API.
//!
//! The door follows the header's lifetime rules exactly. An engine lives
//! until `thinkthen_engine_free`, a returned string until
//! `thinkthen_free_string`, and the message from `thinkthen_error_message`
//! until the calling thread records its next failure on that engine. Each
//! engine carries its own failure table keyed by the recording thread.
//!
//! No panic crosses into the host: every exported symbol runs behind one
//! guard that records the defect kind and returns a code or a null.
//! `DESIGN.md` records the options and ownership design.

pub mod ffi;

mod call;
mod door;
mod failures;
