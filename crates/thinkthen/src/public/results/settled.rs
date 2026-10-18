//! Settled answers are computed from native typed readings before publication.
use serde::Serialize;
use std::collections::BTreeMap;

/// A single input keeps a scalar; records, sources and feeds keep collections.
#[derive(Clone, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum AnswerShape<T> {
    /// One explicit text or JSON input, or one whole-set aggregate.
    Scalar(T),
    /// Ordered answers to a collection, including a one-element collection.
    Collection(Vec<T>),
}
impl<T> std::fmt::Debug for AnswerShape<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnswerShape").finish_non_exhaustive()
    }
}

/// Whether an answer exists, independently of its nullable value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum AnswerResolution {
    /// A successful reading, including an authored null or find's synthetic none.
    Resolved,
    /// A successful decision or choice abstained under its effective threshold.
    Unresolved,
    /// A logical or invocation failure supplied no answer.
    Failed,
}

/// Resolution follows input rows and named annotation members without losing failures.
#[derive(Clone, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum AnswerResolutions {
    /// One primitive reading or aggregate.
    State(AnswerResolution),
    /// Every named member, including members omitted from the bare value on failure.
    Members(BTreeMap<String, AnswerResolution>),
    /// Recoverable failures at original input indexes in a whole-call aggregate.
    Occurrences(BTreeMap<usize, AnswerResolution>),
}

impl std::fmt::Debug for AnswerResolutions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::State(state) => state.fmt(f),
            Self::Occurrences(rows) => f
                .debug_struct("AnswerResolutions")
                .field("occurrences", &rows.len())
                .finish(),
            Self::Members(members) => f
                .debug_struct("AnswerResolutions")
                .field("members", &members.len())
                .finish(),
        }
    }
}

/// Rust-owned answer fields. Probability is answer odds, never model confidence.
#[derive(Clone, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Settled<T, P> {
    /// The bare value in the caller's input shape. None means no answer after a
    /// failure; Some(None) on nullable values retains an unresolved or authored null.
    pub value: AnswerShape<Option<T>>,
    /// Yes, selected-option or aligned item odds according to the function's schema.
    /// The outer None retains absence after a failure.
    pub probability: AnswerShape<Option<P>>,
    /// Raw accepted leading option; absent when the function has no choice.
    pub pick: AnswerShape<Option<String>>,
    /// Selected zero-based original input index, when the function selects one unit.
    pub index: AnswerShape<Option<usize>>,
    /// Successful null, unresolved readings and failures remain distinguishable.
    pub resolution: AnswerShape<AnswerResolutions>,
}
impl<T, P> std::fmt::Debug for Settled<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Settled").finish_non_exhaustive()
    }
}

/// One annotation's primitive bare value. Failed members have no entry.
#[derive(Clone, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum SettledJudgment {
    /// A yes, no or unresolved decision.
    Decision(Option<bool>),
    /// A selected label or unresolved choice.
    Choice(Option<String>),
    /// A weighted position.
    Score(f64),
    /// Selected labels in declared order.
    Tags(Vec<String>),
}
impl std::fmt::Debug for SettledJudgment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettledJudgment").finish_non_exhaustive()
    }
}

/// The function fixes the answer and probability types in the native schema.
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "settledAnswer"))]
#[serde(tag = "function", rename_all = "lowercase")]
pub enum SettledAnswer {
    /// Decision values carry yes odds when resolved.
    #[cfg_attr(test, schemars(extend("x-probability-mode" = "yes")))]
    Decide(Settled<SettledDecision, Option<f64>>),
    /// Choice values carry the selected option's odds when resolved.
    #[cfg_attr(test, schemars(extend("x-probability-mode" = "picked")))]
    Choose(Settled<Option<String>, Option<f64>>),
    /// Tag has no scalar probability; each label retains its detailed odds.
    #[cfg_attr(test, schemars(extend("x-probability-mode" = "null")))]
    Tag(Settled<Vec<String>, Option<f64>>),
    /// Score has no scalar probability; its detailed distribution remains available.
    #[cfg_attr(test, schemars(extend("x-probability-mode" = "null")))]
    Score(Settled<f64, Option<f64>>),
    /// Filtering keeps selected originals and their yes odds in matching order.
    #[cfg_attr(test, schemars(extend("x-probability-mode" = "yes")))]
    Filter(Settled<Vec<SettledOriginal>, Vec<Option<f64>>>),
    /// Ranked zero-based originals carry aligned item odds.
    #[cfg_attr(test, schemars(extend("x-probability-mode" = "items")))]
    Rank(Settled<Vec<usize>, Vec<Option<f64>>>),
    /// Find's successful synthetic none is resolved null with no selected index.
    #[cfg_attr(test, schemars(extend("x-probability-mode" = "picked")))]
    Find(Settled<Option<SettledOriginal>, Option<f64>>),
    /// Recognition carries probabilities aligned with its names or proposals.
    #[cfg_attr(test, schemars(extend("x-probability-mode" = "items")))]
    Recognize(Settled<super::settled_aggregates::SettledRecognition, Vec<Option<f64>>>),
    /// Relations carry probabilities aligned with their selected edges.
    #[cfg_attr(test, schemars(extend("x-probability-mode" = "items")))]
    Relate(Settled<super::settled_aggregates::SettledRelations, Vec<Option<f64>>>),
    /// Named answers retain successful nulls and omit failed values.
    #[cfg_attr(test, schemars(extend("x-probability-mode" = "null")))]
    Annotate(Settled<BTreeMap<String, SettledJudgment>, Option<f64>>),
}

/// An original input value, including authored JSON null, without interpretation.
#[derive(Clone, Serialize)]
#[serde(transparent)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(with = "serde_json::Value")
)]
pub struct SettledOriginal(pub(crate) crate::QuestionInput);
impl SettledOriginal {
    /// Borrow the exact native original.
    #[must_use]
    pub const fn original(&self) -> &crate::QuestionInput {
        &self.0
    }
}
impl std::fmt::Debug for SettledOriginal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettledOriginal").finish_non_exhaustive()
    }
}

/// A primitive decision and its optional authored presentation, including null.
#[derive(Clone)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(with = "crate::core::DecideValue<'static>")
)]
pub struct SettledDecision {
    pub(crate) value: Option<bool>,
    pub(crate) meaning: Option<crate::core::Meaning>,
}
impl SettledDecision {
    /// The primitive reading remains available when its authored value is null.
    #[must_use]
    pub const fn primitive(&self) -> Option<bool> {
        self.value
    }
}
impl std::fmt::Debug for SettledDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettledDecision").finish_non_exhaustive()
    }
}
impl Serialize for SettledDecision {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use crate::core::DecideValue;
        self.meaning
            .as_ref()
            .map_or(DecideValue::Primitive(self.value), DecideValue::Authored)
            .serialize(serializer)
    }
}
