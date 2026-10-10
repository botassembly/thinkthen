//! Private shared host translations over the public native Request API.
//! Family migrations remove legacy entry points; frozen C translation remains.
#![forbid(unsafe_code)]
#![allow(
    missing_docs,
    missing_debug_implementations,
    reason = "private compatibility transports retain their existing withheld diagnostics"
)]
pub mod complete_native;
pub mod source;
pub mod sql_request;
