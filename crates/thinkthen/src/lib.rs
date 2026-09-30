//! The library and command implementation for `thinkthen`.

#![forbid(unsafe_code)]
#![cfg_attr(
    not(feature = "cli"),
    allow(
        dead_code,
        unused_imports,
        reason = "the private engine has no command consumer when default features are disabled"
    )
)]

mod core;

mod config;

mod engine;

mod public;

mod result_json;

pub use public::*;

#[cfg(test)]
mod test_deadline;

#[cfg(feature = "cli")]
mod cli;

#[cfg(feature = "cli")]
pub(crate) use cli::{annotate, args, asking, edge, failure, judge, profile, table};
#[cfg(feature = "cli")]
pub(crate) use cli::{annotate_schedule, schedule};

#[cfg(feature = "cli")]
pub use cli::entry;
