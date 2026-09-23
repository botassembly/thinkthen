//! Pure values for `thinkthen`.
//!
//! This crate holds what the tool knows and never what it does. It reaches no
//! file, no environment variable, no socket, no clock, and no process. Its
//! `clippy.toml` bans those and the crate root turns the bans on.

#![forbid(unsafe_code)]
#![forbid(clippy::disallowed_methods, clippy::disallowed_types)]
#![forbid(clippy::disallowed_macros, clippy::indexing_slicing)]
#![forbid(clippy::allow_attributes_without_reason)]

pub mod adapters;
mod answer;
mod backend;
mod digest;
mod find;
mod json;
mod order;
mod plan;
mod plan_document;
mod pointer;
mod probability;
mod question;
mod question_file;
mod question_set;
pub mod recording;
mod records;
mod render;
mod reply;
mod result;
mod text;
mod threshold;

pub use crate::adapters::built_in::DEFAULT_MODEL;
pub use crate::answer::{Answer, Value};
pub use crate::backend::{Backend, BackendError, KEY_VAR};
pub use crate::digest::question_sha256;
pub use crate::find::{Find, FindAnswer, FindError, FindResult};
pub use crate::order::ranking;
pub use crate::plan::{EmptyPlanError, Plan};
pub use crate::plan_document::PlanDocument;
pub use crate::pointer::{Pointer, PointerError};
pub use crate::question::{Labels, LabelsError, Question};
pub use crate::question_file::{
    Cutting, Described, QuestionFile, QuestionFileError, Resolved, Source, Sources, Typed, Verb,
    resolve,
};
pub use crate::question_set::{NamedQuestion, QuestionSet, QuestionSetError};
pub use crate::records::{
    AnnotatedRecord, Framing, MAX_RECORD_BYTES, Reading, ReadingError, Record, RecordError,
};
pub use crate::render::{RenderError, json_line};
pub use crate::reply::Reply;
pub use crate::result::{
    AnnotateMeta, AnnotateResult, AnnotatedAnswer, DecisionResult, Meta, Usage,
};
pub use crate::text::{BlankTextError, Evidence, Meaning, ModelName, QuestionText, Url};
pub use crate::threshold::{Outcome, Threshold, ThresholdError};

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
