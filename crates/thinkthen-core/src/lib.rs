//! Pure values for `thinkthen`.
//!
//! This crate holds what the tool knows and never what it does. It reaches no
//! file, no environment variable, no socket, no clock, and no process. Its
//! `clippy.toml` bans those and the crate root turns the bans on.

#![forbid(unsafe_code)]
#![forbid(clippy::disallowed_methods, clippy::disallowed_types)]
#![forbid(clippy::disallowed_macros, clippy::indexing_slicing)]
#![forbid(clippy::allow_attributes_without_reason)]

mod adapter;
mod answer;
mod assessment;
mod pass_mark;
mod policy;
mod probability;
mod question;
mod result;
mod text;

pub use crate::adapter::{Adapter, UnknownAdapterError};
pub use crate::answer::{Answer, AnswerKind};
pub use crate::assessment::{Assessment, AssessmentStatus, assess};
pub use crate::pass_mark::{PassMark, PassMarkError};
pub use crate::policy::Policy;
pub use crate::probability::{Probability, ProbabilityError};
pub use crate::question::{Question, Verb};
pub use crate::result::{DecisionResult, Meta, SCHEMA, Usage};
pub use crate::text::{BackendName, BlankTextError, Condition, Evidence, ModelName};

/// The name the tool answers to on the command line and in its own output.
pub const NAME: &str = "thinkthen";

/// Render the identity line that `--version` prints.
///
/// ```
/// assert_eq!(thinkthen_core::version_line("0.1.0"), "thinkthen 0.1.0");
/// ```
#[must_use]
pub fn version_line(version: &str) -> String {
    format!("{NAME} {version}")
}

#[cfg(test)]
mod tests {
    use super::version_line;

    #[test]
    fn version_line_joins_the_name_and_the_version() {
        let cases = [
            ("0.0.1", "thinkthen 0.0.1"),
            ("1.2.3", "thinkthen 1.2.3"),
            ("0.1.0-rc.1", "thinkthen 0.1.0-rc.1"),
            ("", "thinkthen "),
        ];

        for (version, expected) in cases {
            assert_eq!(version_line(version), expected, "version {version}");
        }
    }
}
