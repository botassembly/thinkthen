//! The pgrx embed shim, which `cargo pgrx package` builds to write the SQL.
#![allow(missing_docs, reason = "the macro emits the shim's items")]
#![allow(
    unsafe_code,
    reason = "cargo pgrx package injects unsafe extern blocks here"
)]
::pgrx::pgrx_embed!();
