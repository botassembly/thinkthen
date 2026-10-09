//! Borrowed detail events for a caller that needs more than final call facts.

use std::fmt;

use serde::Serialize;

use crate::core::{self, AnswerOutcome, Backend, ModelName, ProfileName, Threshold};
use crate::public::annotated::{FailureCause, NamedAnnotation, cause};
use crate::public::error::Error;
use crate::public::recognize::Recognized;
use crate::public::relate::Edge;

use super::{Answer, Judgment, NamedProbability, Probabilities, Usage, judgment, usage};

/// One completed question or input row, delivered on the caller's thread.
/// The borrowed fields exist only during the callback.
pub enum RecordObservation<'a> {
    /// One logical question, before its row event.
    Question {
        /// Zero-based input row.
        index: usize,
        /// An annotation member, when this question belongs to a set.
        member: Option<&'a str>,
        /// A recognition stage, when this question belongs to recognition.
        stage: Option<&'static str>,
        /// Zero-based question position within the member or stage.
        position: usize,
        /// The question's answer or failure and exact request metadata.
        detail: QuestionDetail<'a>,
    },
    /// One finished input row, including a row omitted by `filter`.
    Row {
        /// Zero-based input row.
        index: usize,
        /// The finished typed value.
        value: ObservedRow<'a>,
    },
}

impl RecordObservation<'_> {
    /// Map a container's retained row back to its original presentation index.
    /// Actual detail, identities, facts and source coordinates stay intact.
    #[must_use]
    pub fn remap_index(self, index: usize) -> Self {
        match self {
            Self::Question {
                member,
                stage,
                position,
                detail,
                ..
            } => Self::Question {
                index,
                member,
                stage,
                position,
                detail,
            },
            Self::Row { value, .. } => Self::Row { index, value },
        }
    }
}

impl fmt::Debug for RecordObservation<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Question {
                index,
                stage,
                position,
                ..
            } => formatter
                .debug_struct("Question")
                .field("index", index)
                .field("stage", stage)
                .field("position", position)
                .finish_non_exhaustive(),
            Self::Row { index, .. } => formatter
                .debug_struct("Row")
                .field("index", index)
                .finish_non_exhaustive(),
        }
    }
}

/// A question event serializes as one object: its `index`, its `member` or
/// `stage` when set, its `position`, and its detail. A row event has no JSON
/// form; serializing one fails, so callers skip row events.
impl Serialize for RecordObservation<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Self::Question {
            index,
            member,
            stage,
            position,
            detail,
        } = self
        else {
            return Err(serde::ser::Error::custom("a row event has no JSON form"));
        };
        let held = detail.0;
        QuestionJson {
            index: *index,
            member: *member,
            stage: *stage,
            position: *position,
            question_sha256: &held.question_sha256,
            model: &held.model,
            url: &held.url,
            requests: &held.requests,
            requests_sent: held.requests_sent,
            cached: held.cached,
            failed_questions: held.failed_questions,
            answer: held.value.as_ref().map(bare),
            failed: held.failure,
            probabilities: held.probabilities.as_ref().map(|shown| match shown {
                Probabilities::YesNo { yes } => ProbabilitiesJson::Yes(*yes),
                Probabilities::Named(rows) => ProbabilitiesJson::Named(
                    rows.iter()
                        .map(|row| (row.name.as_str(), row.probability))
                        .collect(),
                ),
            }),
            confidence: held.confidence,
            usage: held.usage,
        }
        .serialize(serializer)
    }
}

/// One question event as JSON, as [`RecordObservation`] serializes it.
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "questionObservation")
)]
pub(crate) struct QuestionJson<'a> {
    index: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    member: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<&'static str>,
    position: usize,
    question_sha256: &'a str,
    model: &'a str,
    url: &'a str,
    requests: &'a [String],
    requests_sent: u64,
    cached: bool,
    failed_questions: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    answer: Option<core::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failed: Option<core::BackendFailure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    probabilities: Option<ProbabilitiesJson<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, schemars(with = "Option<core::Usage>"))]
    usage: Option<Usage>,
}

/// The yes probability, or each name and its probability in declared order.
#[derive(Serialize)]
#[serde(untagged)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(inline))]
enum ProbabilitiesJson<'a> {
    Yes(f64),
    Named(Vec<(&'a str, f64)>),
}

pub(super) fn bare(value: &Judgment) -> core::Value {
    match value {
        Judgment::Decision(Answer::Yes) => core::Value::YesNo(Some(true)),
        Judgment::Decision(Answer::No) => core::Value::YesNo(Some(false)),
        Judgment::Decision(Answer::Unsure) => core::Value::YesNo(None),
        Judgment::Choice(pick) => core::Value::Choice(pick.clone()),
        Judgment::Score(position) => core::Value::Score(*position),
        Judgment::Tags(labels) => core::Value::Tag(labels.clone()),
    }
}

/// A row's typed answer without its raw evidence.
pub enum ObservedRow<'a> {
    /// A decide, choose, score, or tag judgment.
    Judgment(&'a Judgment),
    /// The named values of an annotation.
    Annotated(&'a [NamedAnnotation]),
    /// A completed recognition result.
    Recognized(&'a Recognized),
    /// The selected candidate's zero-based place, if any.
    Find(Option<usize>),
    /// The completed relation edges.
    Relations(&'a [Edge]),
}

impl fmt::Debug for ObservedRow<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Judgment(_) => "Judgment(<withheld>)",
            Self::Annotated(_) => "Annotated(<withheld>)",
            Self::Recognized(_) => "Recognized(<withheld>)",
            Self::Find(_) => "Find(<withheld>)",
            Self::Relations(_) => "Relations(<withheld>)",
        })
    }
}

/// One question's complete answer and per-row request share.
#[derive(Clone, Copy)]
pub struct QuestionDetail<'a>(pub(crate) &'a ObservedQuestion);

impl fmt::Debug for QuestionDetail<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("QuestionDetail")
            .field("answered", &self.0.value.is_some())
            .field("failure", &self.0.failure)
            .field("requests_sent", &self.0.requests_sent)
            .finish_non_exhaustive()
    }
}

impl<'a> QuestionDetail<'a> {
    pub(crate) const fn of(value: &'a ObservedQuestion) -> Self {
        Self(value)
    }

    /// The complete question and rule digest.
    #[must_use]
    pub fn question_sha256(&self) -> &str {
        &self.0.question_sha256
    }
    /// The answered typed value, absent for a failed question.
    #[must_use]
    pub fn value(&self) -> Option<&Judgment> {
        self.0.value.as_ref()
    }
    /// The typed backend failure, absent for an answered question.
    #[must_use]
    pub const fn failure(&self) -> Option<FailureCause> {
        match self.0.failure {
            Some(failed) => Some(cause(core::FailedValue::new(failed).cause())),
            None => None,
        }
    }
    /// Declared-order probabilities, absent for a failed question.
    #[must_use]
    pub fn probabilities(&self) -> Option<&Probabilities> {
        self.0.probabilities.as_ref()
    }
    /// Backend confidence when supplied.
    #[must_use]
    pub const fn confidence(&self) -> Option<f64> {
        self.0.confidence
    }
    /// The answer model.
    #[must_use]
    pub fn model(&self) -> &str {
        &self.0.model
    }
    /// The backend URL.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.0.url
    }
    /// The exact recording digests of requests behind this question.
    #[must_use]
    pub fn requests(&self) -> &[String] {
        &self.0.requests
    }
    /// This question's even share of live attempts, retries included.
    #[must_use]
    pub const fn requests_sent(&self) -> u64 {
        self.0.requests_sent
    }
    /// This question's even share of reported provider tokens.
    #[must_use]
    pub const fn usage(&self) -> Option<Usage> {
        self.0.usage
    }
    /// Whether a cache or replay answered the question.
    #[must_use]
    pub const fn cached(&self) -> bool {
        self.0.cached
    }
    /// Failed logical questions represented by this detail.
    #[must_use]
    pub const fn failed_questions(&self) -> usize {
        self.0.failed_questions
    }
}

/// Owned only within the current bounded worker result.
#[derive(Clone)]
pub(crate) struct ObservedQuestion {
    pub(super) actual: super::owned_observation::ActualQuestion,
    pub(crate) question_sha256: String,
    pub(crate) value: Option<Judgment>,
    pub(crate) failure: Option<core::BackendFailure>,
    pub(crate) probabilities: Option<Probabilities>,
    pub(crate) confidence: Option<f64>,
    pub(crate) model: String,
    pub(crate) url: String,
    pub(crate) requests: Vec<String>,
    pub(crate) requests_sent: u64,
    pub(crate) usage: Option<Usage>,
    pub(crate) cached: bool,
    pub(crate) failed_questions: usize,
}

impl ObservedQuestion {
    pub(crate) fn from_annotated(
        entry: &core::AnnotatedEntry,
        profile: Option<&ProfileName>,
        backend: &Backend,
        model: &ModelName,
        receipt: (Option<core::Usage>, u64, bool),
    ) -> Result<Self, Error> {
        let (usage, sent, cached) = receipt;
        let (question, threshold, outcome, request) =
            if let Some((question, answer, threshold, request)) = entry.answered() {
                (
                    question,
                    threshold,
                    AnswerOutcome::Answered(answer.clone()),
                    request,
                )
            } else if let Some((question, failure, request)) = entry.failed() {
                (question, None, AnswerOutcome::Failed(failure), request)
            } else {
                return Err(Error::defect("an annotated entry held no answer"));
            };
        let reply = core::Reply::new(model.clone(), vec![outcome.clone()], usage);
        Self::from_reply(
            question, threshold, profile, backend, &outcome, &reply, request, sent, cached, 1, 0,
        )
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "one bounded reply supplies the question and its request share"
    )]
    pub(crate) fn from_reply(
        question: &core::Question,
        threshold: Option<Threshold>,
        tuned_for: Option<&ProfileName>,
        backend: &Backend,
        answer: &AnswerOutcome,
        reply: &core::Reply,
        request: &str,
        sent: u64,
        cached: bool,
        rows: usize,
        position: usize,
    ) -> Result<Self, Error> {
        let question_sha256 = core::question_sha256_with_profile(question, threshold, tuned_for)
            .map_err(|_| Error::defect("a question digest could not be written"))?;
        let (value, failure, probabilities, confidence) = match answer {
            AnswerOutcome::Answered(answer) => {
                let (value, _) = answer.read(threshold);
                let probabilities = match (answer.yes(), answer.named()) {
                    (Some(yes), _) => Some(Probabilities::YesNo { yes }),
                    (None, Some(named)) => Some(Probabilities::Named(
                        named
                            .into_iter()
                            .map(|(name, probability)| NamedProbability {
                                name: name.to_owned(),
                                probability,
                            })
                            .collect(),
                    )),
                    (None, None) => None,
                };
                (
                    Some(judgment(&value)),
                    None,
                    probabilities,
                    answer.confidence().map(|one| one.as_f64()),
                )
            }
            AnswerOutcome::Failed(failed) => (None, Some(*failed), None, None),
        };
        Ok(Self {
            actual: super::owned_observation::ActualQuestion {
                declarations: core::declaration::QuestionMetadata::default(),
                question: question.clone(),
                threshold,
                raw_pick: match answer {
                    AnswerOutcome::Answered(answer) => answer.leader().map(str::to_owned),
                    _ => None,
                },
                sources: Vec::new(),
                observations: Vec::new(),
                reported_usage: reply
                    .reported_usage()
                    .map(|whole| whole.share(rows, position)),
                identity: None,
                input: None,
                inputs: Vec::new(),
            },
            question_sha256,
            value,
            failure,
            probabilities,
            confidence,
            model: reply.model().as_str().to_owned(),
            url: backend.url().as_str().to_owned(),
            requests: vec![request.to_owned()],
            requests_sent: core::share(sent, rows, position),
            usage: reply
                .usage()
                .map(|whole| usage(whole.share(rows, position))),
            cached,
            failed_questions: usize::from(failure.is_some()),
        })
    }
}
