//! Pure values for `thinkthen`.
//!
//! This module holds what the tool knows and never what it does. It reaches no
//! file, no environment variable, no socket, no clock, and no process. The
//! repository policy check enforces that boundary across this source tree.

#![forbid(unsafe_code)]
#![forbid(clippy::disallowed_methods, clippy::disallowed_types)]
#![forbid(clippy::disallowed_macros, clippy::indexing_slicing)]
#![forbid(clippy::allow_attributes_without_reason)]

pub(crate) mod adapters;
mod answer;
mod backend;
mod backend_profile;
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
mod recognize;
mod recognize_file;
pub(crate) mod recording;
pub(crate) mod recording_identity;
mod records;
mod relate_file;
pub(crate) mod relation;
mod render;
mod reply;
mod result;
mod text;
mod threshold;

pub(crate) use crate::core::adapters::built_in::DEFAULT_MODEL;
pub(crate) use crate::core::answer::{Answer, Value};
pub(crate) use crate::core::backend::{Backend, BackendError, KEY_VAR};
pub(crate) use crate::core::backend_profile::LimitKind;
pub(crate) use crate::core::backend_profile::{
    BackendProfile, ProfileError, ProfileLimit, ProfileName,
};
#[cfg(test)]
pub(crate) use crate::core::digest::question_sha256;
pub(crate) use crate::core::digest::question_sha256_with_profile;
pub(crate) use crate::core::find::Find;
pub(crate) use crate::core::order::ranking;
pub(crate) use crate::core::plan::Plan;
pub(crate) use crate::core::plan_document::PlanDocument;
pub(crate) use crate::core::pointer::{Pointer, PointerError};
pub(crate) use crate::core::question::{Labels, Question};
pub(crate) use crate::core::question_file::{
    Cutting, QuestionFile, QuestionFileError, Resolved, Source, Sources, Typed, Verb, pointers,
    resolve,
};
pub(crate) use crate::core::question_set::{QuestionSet, QuestionSetError};
pub(crate) use crate::core::recognize::{
    RecognizedName, TokenAnswer, assemble as assemble_names, kind_questions, recognition_questions,
    tokenize,
};
pub(crate) use crate::core::recognize_file::{
    RecognizeConfigError, RecognizeKinds, RecognizeSpec, recognize_sha256,
};
pub(crate) use crate::core::records::{
    Framing, MAX_RECORD_BYTES, Reading, ReadingError, Record, RecordError,
};
pub(crate) use crate::core::relate_file::{
    RelateConfigError, RelateFields, RelateQuestion, RelateSpec,
};
pub(crate) use crate::core::relation::{
    QuestionMap, RelationEdge, RelationEntity, RelationEntityView, RelationPlan, RelationRule,
    assemble_edges, plan as plan_relation, plan_pairs, reaches_cut, relation_evidence,
};
pub(crate) use crate::core::render::{RenderError, json_line};
pub(crate) use crate::core::reply::{AnswerOutcome, BackendFailure, FailedValue, Reply};
pub(crate) use crate::core::result::SCHEMA as RESULT_SCHEMA;
pub(crate) use crate::core::result::{
    AnnotateMeta, AnnotateResult, AnnotatedAnswer, AnnotatedEntry, AnnotatedFailure,
    AnnotatedValue, DecisionResult, Meta, NamedValues, ProfileWarning, RecordValue, RequestMeta,
    Usage,
};
pub(crate) use crate::core::text::{Description, Evidence, ModelName, QuestionText};
pub(crate) use crate::core::threshold::{Outcome, Threshold};

#[cfg(test)]
pub(crate) use crate::core::find::FindAnswer;
#[cfg(test)]
pub(crate) use crate::core::text::Url;

/// The name the tool answers to on the command line and in its own output.
pub(crate) const NAME: &str = "thinkthen";

/// Render the identity line that `--version` prints.
///
/// ```
/// assert_eq!(
///     thinkthen::__internal_doctest::version_line("0.1.0"),
///     "thinkthen 0.1.0"
/// );
/// ```
#[must_use]
pub(crate) fn version_line(version: &str) -> String {
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
