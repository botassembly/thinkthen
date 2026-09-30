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

#[cfg(test)]
mod schema_forms;

#[cfg(test)]
#[cfg(feature = "cli")]
mod schema_tests;

#[cfg(feature = "cli")]
mod cli;

#[cfg(feature = "cli")]
pub(crate) use cli::schedule;
#[cfg(feature = "cli")]
pub(crate) use cli::{annotate, args, asking, edge, failure, judge, profile, table};

#[cfg(feature = "cli")]
pub use cli::entry;
