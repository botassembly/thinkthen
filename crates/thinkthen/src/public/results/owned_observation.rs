//! Owned observer snapshots retain actual accepted sources and normalized readings.
use super::{
    Judgment, ObservedQuestion, ObservedRow, QuestionDetail, RecordObservation, ResolvedQuestion,
    ResolvedThreshold,
};
use crate::core::{self, MemberIdentity, Observation};
use crate::engine::facade;
use crate::public::{
    AnswerId, Error, FailureId, InputFunction, NamedAnnotation, QuestionInput, QuestionSource,
    Recognized, ReportedUsage,
};
use serde::Serialize;
use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub(super) struct ActualQuestion {
    pub(super) question: core::Question,
    pub(super) threshold: Option<core::Threshold>,
    pub(super) raw_pick: Option<String>,
    pub(super) sources: Vec<QuestionSource>,
    pub(super) observations: Vec<Observation>,
    pub(super) reported_usage: Option<ReportedUsage>,
    pub(super) identity: Option<MemberIdentity>,
    pub(super) input: Option<Arc<QuestionInput>>,
    pub(super) inputs: Vec<Arc<QuestionInput>>,
}
impl ObservedQuestion {
    pub(crate) fn with_receipt(mut self, receipt: &facade::Answered) -> Self {
        self.actual.sources.clone_from(&receipt.sources);
        self.actual.observations.clone_from(&receipt.observations);
        self.actual.reported_usage = receipt.reply.reported_usage();
        self
    }
    pub(crate) fn with_input(mut self, input: Arc<QuestionInput>) -> Self {
        self.actual.input = Some(input);
        self
    }
    pub(crate) fn with_inputs(mut self, inputs: &[Arc<QuestionInput>]) -> Self {
        self.actual.inputs = inputs.to_vec();
        self
    }
    pub(crate) fn with_threshold(mut self, threshold: Option<core::Threshold>) -> Self {
        self.actual.threshold = threshold;
        self
    }
    pub(crate) fn qualified(
        mut self,
        function: InputFunction,
        index: usize,
        member: Option<&str>,
        stage: Option<&'static str>,
        position: usize,
    ) -> Result<Self, Error> {
        if self.failure.is_some() {
            let failure = self
                .actual
                .observations
                .iter()
                .find_map(|value| match value {
                    Observation::Failed { failure_id } => Some(failure_id.clone()),
                    Observation::Answered { .. } => None,
                })
                .ok_or_else(|| {
                    Error::defect("an observed failed question lost its failure identity")
                })?;
            self.actual.identity = Some(MemberIdentity::Failed(failure));
        } else {
            let reading = core::AtomicReading {
                question: &self.actual.question,
                threshold: self.actual.threshold,
                rank_position: None,
            };
            let sources = self.actual.sources.clone();
            let observations = self.actual.observations.clone();
            let identity = match (member, stage) {
                (None, None) => core::ResultIdentity::of(
                    function,
                    &core::RecordScope { record: index },
                    sources,
                    observations,
                    &reading,
                    &[],
                ),
                (Some(member), _) => core::ResultIdentity::of(
                    function,
                    &MemberScope {
                        record: index,
                        member,
                        position,
                    },
                    sources,
                    observations,
                    &reading,
                    &[],
                ),
                (None, Some(stage)) => core::ResultIdentity::of(
                    function,
                    &StageScope {
                        record: index,
                        stage,
                        position,
                    },
                    sources,
                    observations,
                    &reading,
                    &[],
                ),
            }
            .map_err(|_| Error::defect("an observed question identity could not be constructed"))?;
            self.actual.identity = Some(MemberIdentity::Answered(identity.answer_id().clone()));
        }
        Ok(self)
    }
}
#[derive(Serialize)]
struct MemberScope<'a> {
    record: usize,
    member: &'a str,
    position: usize,
}
#[derive(Serialize)]
struct StageScope<'a> {
    record: usize,
    stage: &'a str,
    position: usize,
}

impl QuestionDetail<'_> {
    /// Own the actual question detail beyond the observer callback.
    #[must_use]
    pub fn to_owned(&self) -> OwnedQuestionDetail {
        OwnedQuestionDetail(Arc::new(self.0.clone()))
    }
    /// The normalized logical primitive actually asked, including declared order.
    #[must_use]
    pub const fn question(&self) -> ResolvedQuestion<'_> {
        ResolvedQuestion(&self.0.actual.question)
    }
    /// Actual admitted reading rule, including failed members.
    #[must_use]
    pub fn threshold(&self) -> Option<ResolvedThreshold> {
        self.0.actual.threshold.map(ResolvedThreshold::of)
    }
    /// Unthresholded accepted option or level, absent for ties or failed answers.
    #[must_use]
    pub fn raw_pick(&self) -> Option<&str> {
        self.0.actual.raw_pick.as_deref()
    }
    /// Aligned actual sources, with missing historical counts absent.
    #[must_use]
    pub fn question_sources(&self) -> &[QuestionSource] {
        &self.0.actual.sources
    }
    /// Actual accepted answer or failure observations in logical request order.
    #[must_use]
    pub fn observations(&self) -> &[Observation] {
        &self.0.actual.observations
    }
    /// Stable identity of this qualified logical question's answer.
    /// Aggregate and ranked result identities are supplied separately by complete results.
    #[must_use]
    pub fn answer_id(&self) -> Option<&AnswerId> {
        match &self.0.actual.identity {
            Some(MemberIdentity::Answered(id)) => Some(id),
            _ => None,
        }
    }
    /// Actual failure identity, absent for successful answers including null.
    #[must_use]
    pub fn failure_id(&self) -> Option<&FailureId> {
        match &self.0.actual.identity {
            Some(MemberIdentity::Failed(id)) => Some(id),
            _ => None,
        }
    }
    /// Reported partial input/output shares, with unknown counts absent.
    #[must_use]
    pub const fn reported_usage(&self) -> Option<ReportedUsage> {
        self.0.actual.reported_usage
    }
    /// Admitted evidence with original native records, images and physical location when supplied.
    #[must_use]
    pub fn input(&self) -> Option<&QuestionInput> {
        self.0.actual.input.as_deref()
    }
    /// Every original whole-set input, in order, with physical locations outside identity.
    /// Atomic and per-record events expose their single input through the same iterator.
    pub fn inputs(&self) -> impl Iterator<Item = &QuestionInput> {
        self.0
            .actual
            .input
            .iter()
            .chain(self.0.actual.inputs.iter())
            .map(AsRef::as_ref)
    }
}

/// Immutable owned actual question detail; borrowed views live as long as this owner.
#[derive(Clone)]
pub struct OwnedQuestionDetail(Arc<ObservedQuestion>);
impl OwnedQuestionDetail {
    /// Borrow the same concrete typed fields as the original event.
    #[must_use]
    pub fn detail(&self) -> QuestionDetail<'_> {
        QuestionDetail::of(self.0.as_ref())
    }
}
impl fmt::Debug for OwnedQuestionDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.detail().fmt(f)
    }
}

/// Owned finished-row values, retaining order and successful null values.
#[derive(Clone)]
pub enum OwnedObservedRow {
    /// Concrete atomic judgment.
    Judgment(Judgment),
    /// Named annotation members in authored order.
    Annotated(Vec<NamedAnnotation>),
    /// Actual recognized spans and relations.
    Recognized(Recognized),
    /// Selected zero-based original ordinal, or no selection.
    Find(Option<usize>),
    /// Actual accepted relation edges in declared order.
    Relations(Vec<crate::public::Edge>),
}
impl fmt::Debug for OwnedObservedRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OwnedObservedRow(<withheld>)")
    }
}
/// An owned event snapshot with no borrowed callback or engine lifetime.
#[derive(Clone)]
pub enum OwnedRecordObservation {
    /// One actual qualified logical question before the finished row.
    Question {
        /// Zero-based original occurrence.
        index: usize,
        /// Exact annotation member name when present.
        member: Option<String>,
        /// Actual recognition/relation stage when present.
        stage: Option<&'static str>,
        /// Zero-based stage/member question position.
        position: usize,
        /// Immutable owned detail.
        detail: OwnedQuestionDetail,
    },
    /// One completed typed row.
    Row {
        /// Zero-based original occurrence.
        index: usize,
        /// Immutable owned actual value.
        value: OwnedObservedRow,
    },
}
impl RecordObservation<'_> {
    /// Own this event without interpreting known protocol JSON or cloning an engine.
    #[must_use]
    pub fn to_owned(&self) -> OwnedRecordObservation {
        match self {
            Self::Question {
                index,
                member,
                stage,
                position,
                detail,
            } => OwnedRecordObservation::Question {
                index: *index,
                member: member.map(str::to_owned),
                stage: *stage,
                position: *position,
                detail: detail.to_owned(),
            },
            Self::Row { index, value } => OwnedRecordObservation::Row {
                index: *index,
                value: match value {
                    ObservedRow::Judgment(value) => OwnedObservedRow::Judgment((*value).clone()),
                    ObservedRow::Annotated(value) => OwnedObservedRow::Annotated(value.to_vec()),
                    ObservedRow::Recognized(value) => {
                        OwnedObservedRow::Recognized((*value).clone())
                    }
                    ObservedRow::Find(value) => OwnedObservedRow::Find(*value),
                    ObservedRow::Relations(value) => OwnedObservedRow::Relations(value.to_vec()),
                },
            },
        }
    }
}
impl fmt::Debug for OwnedRecordObservation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Question {
                index,
                stage,
                position,
                detail,
                ..
            } => f
                .debug_struct("Question")
                .field("index", index)
                .field("stage", stage)
                .field("position", position)
                .field("detail", detail)
                .finish_non_exhaustive(),
            Self::Row { index, .. } => f
                .debug_struct("Row")
                .field("index", index)
                .finish_non_exhaustive(),
        }
    }
}
