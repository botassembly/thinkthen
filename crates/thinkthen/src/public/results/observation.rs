//! Borrowed detail events for a caller that needs more than final call facts.

use std::fmt;

use crate::core::{self, AnswerOutcome, Backend, ModelName, ProfileName, Threshold};
use crate::engine::error::Error as EngineError;
use crate::engine::facade::Answered;
use crate::public::annotated::{FailureCause, NamedAnnotation};
use crate::public::error::Error;
use crate::public::options::Stop;
use crate::public::recognize::Recognized;
use crate::public::relate::Edge;

use super::{Details, Judgment, NamedProbability, Probabilities, Usage, judgment, usage};

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
        self.0.failure
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
pub(crate) struct ObservedQuestion {
    pub(crate) question_sha256: String,
    pub(crate) value: Option<Judgment>,
    pub(crate) failure: Option<FailureCause>,
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
            AnswerOutcome::Failed(failed) => (
                None,
                Some(crate::public::annotated::cause(
                    core::FailedValue::new(*failed).cause(),
                )),
                None,
                None,
            ),
        };
        Ok(Self {
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

    pub(crate) fn from_details(details: &Details) -> Self {
        Self {
            question_sha256: details.question_sha256.clone(),
            value: Some(details.value.clone()),
            failure: None,
            probabilities: Some(details.probabilities.clone()),
            confidence: details.confidence,
            model: details.model.clone(),
            url: details.url.clone(),
            requests: details.requests.clone(),
            requests_sent: details.requests_sent,
            usage: details.usage,
            cached: details.cached,
            failed_questions: 0,
        }
    }
}

fn stage_slot(stage: &str) -> Option<usize> {
    match stage {
        "boundary" => Some(0),
        "kind" => Some(1),
        "edge" => Some(2),
        "relation" => Some(3),
        _ => None,
    }
}

/// Name each logical question in one actual ordered request chunk.
pub(crate) fn observe_chunk(
    stop: &Stop<'_>,
    backend: &Backend,
    plan: &core::Plan,
    answered: &Answered,
    stages: impl ExactSizeIterator<Item = &'static str>,
    positions: &mut [usize; 4],
) -> Result<(), EngineError> {
    if !stop.observing() {
        return Ok(());
    }
    let rows = plan.questions().len();
    if stages.len() != rows || answered.reply.outcomes().len() != rows {
        return Err(EngineError::Defect(
            "an observed chunk has unequal questions",
        ));
    }
    for (within_chunk, ((stage, question), outcome)) in stages
        .zip(plan.questions())
        .zip(answered.reply.outcomes())
        .enumerate()
    {
        let place = stage_slot(stage).ok_or(EngineError::Defect("an observed stage is unknown"))?;
        let current = positions
            .get_mut(place)
            .ok_or(EngineError::Defect("an observed stage has no counter"))?;
        let position = *current;
        *current += 1;
        let detail = ObservedQuestion::from_reply(
            question,
            None,
            None,
            backend,
            outcome,
            &answered.reply,
            answered.request.as_str(),
            answered.requests_sent,
            answered.replayed,
            rows,
            within_chunk,
        )
        .map_err(|_| EngineError::Defect("an observed question digest could not be written"))?;
        stop.observe(RecordObservation::Question {
            index: 0,
            member: None,
            stage: Some(stage),
            position,
            detail: QuestionDetail::of(&detail),
        });
        if stop.observer_panicked() {
            return Err(EngineError::Defect("the question observer panicked"));
        }
    }
    Ok(())
}
