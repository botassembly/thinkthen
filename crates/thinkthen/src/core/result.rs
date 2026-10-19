//! The JSON document one judgment prints.

use serde::{Serialize, Serializer};

use crate::core::answer::{Answer, Value};
use crate::core::question::Question;
use crate::core::records::Record;
use crate::core::reply::{BackendFailure, FailedValue};
use crate::core::text::{ModelName, Url};
use crate::core::threshold::Threshold;

mod attempt;
mod reported_usage;
pub(crate) use reported_usage::ReportedSum;
pub use reported_usage::ReportedUsage;
pub(crate) mod complete;
pub use complete::{Observation, Origin, QuestionSource, ResultIdentity};
mod batch_warning;
mod fields;
mod meta;
pub(crate) use fields::Fields;
mod profile_warning;
#[cfg(test)]
mod record_value;

pub use attempt::{AttemptObservation, AttemptOutcome, CompleteAttempt};
pub(crate) use batch_warning::{BatchSetting, BatchWarning};
pub(crate) use meta::Meta;
pub(crate) use profile_warning::ProfileWarning;
#[cfg(test)]
pub(crate) use record_value::RecordValue;

/// What the backend reported it spent on the judgment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "tokenUsage"))]
pub(crate) struct Usage {
    input_tokens: u64,
    output_tokens: u64,
}

impl Usage {
    /// Take the token counts the backend reported.
    #[must_use]
    pub(crate) const fn new(input_tokens: u64, output_tokens: u64) -> Self {
        Self {
            input_tokens,
            output_tokens,
        }
    }

    /// Add the counts from two replies when both totals fit.
    #[must_use]
    pub(crate) const fn checked_plus(self, other: Self) -> Option<Self> {
        let Some(input_tokens) = self.input_tokens.checked_add(other.input_tokens) else {
            return None;
        };
        let Some(output_tokens) = self.output_tokens.checked_add(other.output_tokens) else {
            return None;
        };
        Some(Self {
            input_tokens,
            output_tokens,
        })
    }

    /// The record at `position` of `records`' even share of these counts.
    #[must_use]
    pub(crate) fn share(self, records: usize, position: usize) -> Self {
        Self {
            input_tokens: share(self.input_tokens, records, position),
            output_tokens: share(self.output_tokens, records, position),
        }
    }

    pub(crate) const fn token_counts(self) -> (u64, u64) {
        (self.input_tokens, self.output_tokens)
    }
}

/// The record at `position` of `records`' even share of `total`, with the
/// remainder going to the earliest records, by ADR 0048 item 9.
#[must_use]
pub(crate) fn share(total: u64, records: usize, position: usize) -> u64 {
    let wide = |count: usize| u64::try_from(count).unwrap_or(u64::MAX);
    let records = wide(records).max(1);
    total / records + u64::from(wide(position) < total % records)
}

/// Whether recordings answered and which logical requests made one result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RequestMeta {
    replayed: bool,
    requests_sent: u64,
    requests: Vec<String>,
    failed_questions: usize,
    profile_warning: Option<ProfileWarning>,
    batch_setting: Option<BatchSetting>,
    batch_warning: Option<BatchWarning>,
    context_sha256: Option<String>,
}

impl RequestMeta {
    /// Take the replay fact and ordered recording digests for one result.
    #[must_use]
    pub(crate) const fn new(replayed: bool, requests_sent: u64, requests: Vec<String>) -> Self {
        Self {
            replayed,
            requests_sent,
            requests,
            failed_questions: 0,
            profile_warning: None,
            batch_setting: None,
            batch_warning: None,
            context_sha256: None,
        }
    }

    /// Carry a profile mismatch into detailed metadata.
    pub(crate) fn with_profile_warning(mut self, warning: Option<ProfileWarning>) -> Self {
        self.profile_warning = warning;
        self
    }

    /// Carry the number of failed logical questions in one result.
    #[must_use]
    pub(crate) const fn with_failed_questions(mut self, failed_questions: usize) -> Self {
        self.failed_questions = failed_questions;
        self
    }

    /// Carry the run's resolved batch setting into detailed metadata.
    pub(crate) fn with_batch_setting(mut self, setting: Option<BatchSetting>) -> Self {
        self.batch_setting = setting;
        self
    }

    pub(crate) fn with_batch_warning(mut self, warning: Option<BatchWarning>) -> Self {
        self.batch_warning = warning;
        self
    }

    pub(crate) fn with_context_sha256(mut self, digest: Option<String>) -> Self {
        self.context_sha256 = digest;
        self
    }
}

/// One named answer inside an annotated detailed row.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(test, schemars(rename = "annotateSuccess"))]
pub(crate) struct AnnotatedAnswer {
    value: Value,
    question: Question,
    answer: Answer,
    threshold: Option<Threshold>,
    request: String,
}

/// One failed question inside an annotated detailed row.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(test, schemars(rename = "annotateFailure", deny_unknown_fields))]
pub(crate) struct AnnotatedFailure {
    question: Question,
    failure: BackendFailure,
    request: String,
}

impl AnnotatedFailure {
    /// Gather the question, backend failure, and request that produced it.
    #[must_use]
    pub(crate) const fn new(question: Question, failure: BackendFailure, request: String) -> Self {
        Self {
            question,
            failure,
            request,
        }
    }
}

/// A successful or failed named entry inside detailed `annotate` output.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "annotateEntry"))]
pub(crate) enum AnnotatedEntry {
    /// A successful answer keeps the established detailed shape.
    Answered(AnnotatedAnswer),
    /// A failed answer omits value, answer, and threshold.
    Failed(AnnotatedFailure),
}

impl AnnotatedEntry {
    /// Borrow the complete question and answer without changing the written shape.
    #[must_use]
    pub(crate) const fn answered(&self) -> Option<AnnotatedAnswered<'_>> {
        match self {
            Self::Answered(entry) => Some((
                &entry.question,
                &entry.answer,
                entry.threshold,
                entry.request.as_str(),
            )),
            Self::Failed(_) => None,
        }
    }

    /// Borrow a failed question and its typed backend cause.
    #[must_use]
    pub(crate) const fn failed(&self) -> Option<(&Question, BackendFailure, &str)> {
        match self {
            Self::Answered(_) => None,
            Self::Failed(entry) => Some((&entry.question, entry.failure, entry.request.as_str())),
        }
    }
}

type AnnotatedAnswered<'a> = (&'a Question, &'a Answer, Option<Threshold>, &'a str);

/// A successful or failed named value in bare `annotate` output.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(test, schemars(rename = "annotatedField"))]
pub(crate) enum AnnotatedValue {
    /// A value read from a usable backend answer.
    Answered(Value),
    /// The exact marker for one failed backend answer.
    Failed(FailedValue),
}

impl AnnotatedAnswer {
    /// Gather the complete answer and the request that produced it.
    #[must_use]
    pub(crate) const fn new(
        value: Value,
        question: Question,
        answer: Answer,
        threshold: Option<Threshold>,
        request: String,
    ) -> Self {
        Self {
            value,
            question,
            answer,
            threshold,
            request,
        }
    }
}

/// Aggregate metadata for one annotated record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "annotateMeta"))]
pub(crate) struct AnnotateMeta {
    tool: String,
    questions_sha256: String,
    url: Url,
    model: ModelName,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<Usage>,
    #[serde(skip)]
    #[cfg_attr(test, schemars(skip))]
    reported_usage: Option<ReportedUsage>,
    requests_sent: u64,
    cached: bool,
    requests: Vec<String>,
    failed_questions: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile_warning: Option<ProfileWarning>,
}

impl AnnotateMeta {
    /// Gather the shared facts and ordered request identities behind one row.
    #[must_use]
    pub(crate) fn new(
        version: &str,
        questions_sha256: String,
        url: Url,
        model: ModelName,
        usage: Option<Usage>,
        request_meta: RequestMeta,
    ) -> Self {
        let RequestMeta {
            replayed,
            requests_sent,
            requests,
            failed_questions,
            profile_warning,
            batch_setting: _,
            batch_warning: _,
            context_sha256: _,
        } = request_meta;
        Self {
            tool: crate::core::version_line(version),
            questions_sha256,
            url,
            model,
            usage,
            reported_usage: usage.map(ReportedUsage::from_complete),
            requests_sent,
            cached: replayed,
            requests,
            failed_questions,
            profile_warning,
        }
    }
    pub(crate) fn with_reported_usage(mut self, usage: Option<ReportedUsage>) -> Self {
        self.reported_usage = usage;
        self
    }
}

/// The detailed result from applying a question set to one record.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AnnotateResult {
    input: Record,
    value: NamedValues,
    meta: AnnotateMeta,
}

impl AnnotateResult {
    /// Primitive named values through the existing annotation serializer.
    pub(crate) const fn value(&self) -> &NamedValues {
        &self.value
    }

    /// Gather one complete annotation row.
    #[must_use]
    pub(crate) const fn new(
        input: Record,
        values: Vec<(String, AnnotatedValue)>,
        meta: AnnotateMeta,
    ) -> Self {
        Self {
            input,
            value: NamedValues(values),
            meta,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "annotatedRow"))]
pub(crate) struct NamedValues(
    #[cfg_attr(
        test,
        schemars(with = "std::collections::BTreeMap<String, AnnotatedValue>")
    )]
    Vec<(String, AnnotatedValue)>,
);

impl NamedValues {
    pub(crate) fn into_values(self) -> Vec<(String, AnnotatedValue)> {
        self.0
    }
}
impl Serialize for NamedValues {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|(name, value)| (name, value)))
    }
}

#[cfg(test)]
mod tests;
