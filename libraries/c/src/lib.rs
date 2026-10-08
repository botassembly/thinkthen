//! The C door to the thinkthen engine (ticket 0094, ADR 0037).
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

mod call;
mod current;
mod door;
mod failures;
mod plan;
mod settings;

use failures::Held;

/// The header's `thinkthen_answer`: the outcome code, then the probability
/// of yes. Two fixed fields, never a third.
#[repr(C)]
#[derive(Debug)]
pub struct Judgment {
    /// `THINKTHEN_YES`, `THINKTHEN_NO`, or `THINKTHEN_UNSURE`.
    pub outcome: std::ffi::c_int,
    /// The probability the backend gave the yes side.
    pub probability: f64,
}

/// thinkthen.h is the single C header for thinkthen, version 0.2.0.
/// Plain calls equal their _opts twin with THINKTHEN_NO_DEADLINE and a NULL token.
/// Engines serve concurrent callers and rebuild state after a fork; free them
/// only after all calls return. Free owned strings with thinkthen_free_string.
/// Error messages and failure facts are borrowed; never free their pointers.
/// See thinkthen_error_message for engine and NULL-engine pointer lifetimes.
/// Nonzero returns leave all outputs unchanged. Eager calls have no partial
/// rows; lazy batches retain completed prefixes. Prefer *_with_facts forms.
/// Version 0.1.0 names, layouts, argument types and return codes stay frozen.
/// Reference: https://github.com/botassembly/thinkthen/blob/main/libraries/c/DESIGN.md
/// DESIGN.md references below name this online reference; archives retain
/// their header/library contents.
/// Argument refusals send nothing. NULL engine returns EUSAGE or NULL.
/// question_json/request_json/spec_json must be NUL-terminated UTF-8.
/// text reads exactly text_len bytes, never a terminator. NULL requires length
/// zero; empty evidence still follows the engine's blank-evidence refusal.
/// texts/lengths/bulk out have count entries; NULL is allowed only at count=0.
/// Decide out and recognize/relate out/out_len must be nonnull.
/// A NULL _opts token means no cancellation token.
/// Opaque engine owner. Free after every concurrent call finishes.
#[derive(Debug)]
pub struct Door(Held);

mod complete;
