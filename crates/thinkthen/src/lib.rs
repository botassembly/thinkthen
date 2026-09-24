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

mod engine;

#[cfg(feature = "cli")]
mod cli;

#[cfg(feature = "cli")]
pub(crate) use cli::{annotate, args, asking, edge, failure, judge, profile, table};
#[cfg(feature = "cli")]
pub(crate) use cli::{annotate_schedule, schedule};

#[cfg(feature = "cli")]
pub use cli::entry;

/// Build-only access for behavioral doctests over the private core.
#[cfg(thinkthen_internal_doctest)]
#[doc(hidden)]
pub mod __internal_doctest {
    /// Render the command identity line.
    #[must_use]
    pub fn version_line(version: &str) -> String {
        crate::core::version_line(version)
    }

    /// Rank probabilities in descending stable order.
    #[must_use]
    pub fn ranking(of: &[f64], top: Option<usize>) -> Vec<usize> {
        crate::core::ranking(of, top)
    }
}
