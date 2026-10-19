//! The public `thinkthen` library.

#![cfg_attr(not(windows), forbid(unsafe_code))]
#![cfg_attr(windows, deny(unsafe_code))]

// ADR 0111: the question store's SQLite is a choice every build makes.
#[cfg(not(any(feature = "bundled-sqlite", feature = "host-sqlite")))]
compile_error!(
    "enable thinkthen's bundled-sqlite feature, or host-sqlite in a SQLite extension that initializes rusqlite's loadable API"
);
#[cfg(all(feature = "bundled-sqlite", feature = "host-sqlite"))]
compile_error!(
    "enable only one of thinkthen's bundled-sqlite and host-sqlite features, because the question store runs on one SQLite"
);

mod core;

mod chunks;

mod fallible;

mod table;

mod config;

#[cfg(windows)]
mod windows;

mod engine;

mod public;

mod result_json;

pub use public::*;

#[cfg(test)]
mod test_deadline;

#[cfg(test)]
mod schema_forms;

#[cfg(test)]
mod schema_tests;

#[cfg(test)]
extern crate self as thinkthen;
#[cfg(test)]
#[path = "../tests/support/api.rs"]
mod api;
#[cfg(test)]
mod ownership_tests;

#[cfg(test)]
mod command_carrier_secrecy_tests;
