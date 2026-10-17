//! The C interface to the thinkthen engine (ticket 0094, ADR 0037).
//!
//! This crate builds `libthinkthen.so` and `libthinkthen.a` against
//! `include/thinkthen.h` and owns every exported symbol; `thinkthen` exports
//! none. It reaches the engine only through the public Rust API.
//!
//! Rust owns the C lifetime rules and the generated header carries them. An engine lives
//! until `thinkthen_engine_free`, a returned string until
//! `thinkthen_free_string`, and the message from `thinkthen_error_message`
//! until the calling thread records its next failure on that engine. Each
//! engine carries its own failure table keyed by the recording thread.
//!
//! No panic crosses into the host: every exported symbol runs behind one
//! guard that records the defect kind and returns a code or a null.
//! `DESIGN.md` records the options and ownership design.

pub mod ffi;

mod current;
mod failures;
mod session;
/// Primitive immutable C JSON and UTF-8 views.
pub use session::views as complete_json_views;
/// Immutable generated complete C packet declarations.
pub use session::views_generated as complete_session_views;
mod settings;

use failures::Held;

/// The current C interface owns engines, parsed questions, sessions and packet views.
/// Free each owner after its last use. Request preview returns an owned string
/// released with thinkthen_free_string. Diagnostics borrow per-thread storage.
/// Session cancellation uses thinkthen_session_cancel.
/// Opaque engine owner. Free after every concurrent call finishes.
#[derive(Debug)]
pub struct EngineHandle(Held);
