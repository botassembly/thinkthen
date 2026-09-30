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

// ADR 0111: the question store's SQLite is a choice every build makes.
#[cfg(not(any(feature = "bundled-sqlite", feature = "host-sqlite")))]
compile_error!(
    "enable thinkthen's bundled-sqlite feature, or host-sqlite in a SQLite extension that initializes rusqlite's loadable API"
);

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
