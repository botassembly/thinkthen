//! Pure values for `thinkthen`.
//!
//! This module holds what the tool knows and never what it does. It reaches no
//! file, no environment variable, no socket, no clock, and no process. The
//! repository policy check enforces that boundary across this source tree.

#![forbid(unsafe_code)]
#![forbid(clippy::indexing_slicing)]
// ADR 0112: the unit tests derive the result schema, and the derived code builds
// dynamic JSON. `policy.py` still scans every core source, tests included, for
// file, environment, socket, clock, and process paths.
#![cfg_attr(not(test), forbid(clippy::disallowed_methods))]
#![cfg_attr(not(test), forbid(clippy::disallowed_types))]
#![cfg_attr(not(test), forbid(clippy::disallowed_macros))]
#![forbid(clippy::allow_attributes_without_reason)]

pub(crate) mod adapters;
mod answer;
mod backend;
mod backend_profile;
pub(crate) mod batch;
pub(crate) mod check;
mod digest;
mod find;
mod json;
pub(crate) mod measure;
mod order;
mod plan;
mod plan_document;
mod plan_summary;
mod pointer;
mod price;
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
pub(crate) mod settings;
pub(crate) use settings::engine_settings;
mod text;
mod threshold;

pub(crate) use crate::core::adapters::built_in::DEFAULT_MODEL;
pub(crate) use crate::core::answer::{Answer, Value};
pub(crate) use crate::core::backend::{Backend, BackendError, KEY_IN_ADDRESS, KEY_VAR};
#[cfg(any(test, feature = "cli"))]
pub(crate) use crate::core::backend_profile::LimitKind;
pub(crate) use crate::core::backend_profile::{
    BackendProfile, ProfileError, ProfileLimit, ProfileName,
};
pub(crate) use crate::core::batch::{Batch, BatchError, BatchRecord, Batcher, Setting};
pub(crate) use crate::core::batch::{GroupBatcher, group_halves, quoted_plan};
pub(crate) use crate::core::digest::bytes_sha256;
#[cfg(test)]
pub(crate) use crate::core::digest::question_sha256;
pub(crate) use crate::core::digest::question_sha256_with_profile;
pub(crate) use crate::core::find::Find;
#[cfg(test)]
pub(crate) use crate::core::find::FindResult;
pub(crate) use crate::core::json::Json;
#[cfg(feature = "cli")]
pub(crate) use crate::core::json::JsonError;
pub(crate) use crate::core::order::ranking;
pub(crate) use crate::core::plan::Plan;
pub(crate) use crate::core::plan_document::PlanDocument;
pub(crate) use crate::core::plan_summary::PlanSummary;
pub(crate) use crate::core::pointer::{Pointer, PointerError};
pub(crate) use crate::core::price::Prices;
pub(crate) use crate::core::question::{Labels, LabelsError, Question};
pub(crate) use crate::core::question_file::{
    Cutting, QuestionFile, QuestionFileError, Resolved, Source, Sources, Typed, Verb, pointers,
    resolve, safe_key,
};
pub(crate) use crate::core::question_set::{PartError, QuestionSet, QuestionSetError, check_name};
pub(crate) use crate::core::recognize::{
    Asked, NameOdds, Odds, Piece, PieceOdds, RecognizedName, TAGS, TagRow, decode as found_names,
    evidence as window, kind_question, name_groups, pieces, settle as settle_names,
    step_one_groups, step_one_questions, step_two_questions,
};
pub(crate) use crate::core::recognize_file::{
    RecognizeConfigError, RecognizeKinds, RecognizeSpec, recognize_sha256, rule_side,
};
pub(crate) use crate::core::records::{
    Framing, MAX_RECORD_BYTES, Reading, ReadingError, Record, RecordError,
};
pub(crate) use crate::core::relate_file::{
    EntitySetError, RelateConfigError, RelateFields, RelateQuestion, RelateSpec,
};
pub(crate) use crate::core::relation::{
    Lead, Pair, PairPlan, RelationEdge, RelationEntity, RelationEntityView, RelationRule,
    pair_edges, plan_pairs, reaches_cut,
};
pub(crate) use crate::core::render::{RenderError, json_line};
pub(crate) use crate::core::reply::{
    AnswerOutcome, BackendFailure, BackendFailureCause, FailedValue, Reply,
};
pub(crate) use crate::core::result::SCHEMA as RESULT_SCHEMA;
pub(crate) use crate::core::result::{
    AnnotateBatchMeta, AnnotateMeta, AnnotateResult, AnnotatedAnswer, AnnotatedEntry,
    AnnotatedFailure, AnnotatedValue, BatchMeta, BatchSetting, BatchWarning, DecisionResult, Meta,
    NamedValues, ProfileWarning, RecordValue, RequestMeta, Usage, share,
};
pub(crate) use crate::core::text::{
    BlankTextError, Description, Evidence, Meaning, ModelName, QuestionText, Withheld,
};
pub(crate) use crate::core::threshold::{Outcome, Threshold, ThresholdError};

pub(crate) use crate::core::find::FindAnswer;
#[cfg(test)]
pub(crate) use crate::core::text::Url;

/// The name the tool answers to on the command line and in its own output.
pub(crate) const NAME: &str = "thinkthen";

/// Render the identity line that `--version` prints.
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
