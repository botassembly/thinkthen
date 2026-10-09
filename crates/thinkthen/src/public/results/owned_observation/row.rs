//! Serialization projections borrow native typed observer values.
use super::{Judgment, OwnedObservedRow};
use crate::core;
use crate::public::{Annotated, Answer, FailureCause, NamedAnnotation, Probabilities};
use serde::Serialize;

#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "sessionJudgment")
)]
#[serde(tag = "kind", content = "value", rename_all = "lowercase")]
pub(crate) enum JudgmentDocument<'a> {
    Decision(Option<bool>),
    Choice(Option<&'a str>),
    Score(f64),
    Tags(&'a [String]),
}
impl<'a> JudgmentDocument<'a> {
    pub(super) fn of(value: &'a Judgment) -> Self {
        match value {
            Judgment::Decision(value) => Self::Decision(decision(*value)),
            Judgment::Choice(value) => Self::Choice(value.as_deref()),
            Judgment::Score(value) => Self::Score(*value),
            Judgment::Tags(value) => Self::Tags(value),
        }
    }
}
fn decision(value: Answer) -> Option<bool> {
    match value {
        Answer::Yes => Some(true),
        Answer::No => Some(false),
        Answer::Unsure => None,
    }
}
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "sessionProbabilities")
)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub(super) enum ProbabilitiesDocument<'a> {
    YesNo(f64),
    Named(Vec<NamedProbabilityDocument<'a>>),
}
impl<'a> ProbabilitiesDocument<'a> {
    pub(super) fn of(value: &'a Probabilities) -> Self {
        match value {
            Probabilities::YesNo { yes } => Self::YesNo(*yes),
            Probabilities::Named(values) => Self::Named(
                values
                    .iter()
                    .map(|v| NamedProbabilityDocument {
                        name: v.name(),
                        probability: v.probability(),
                    })
                    .collect(),
            ),
        }
    }
}
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "sessionNamedProbability")
)]
pub(crate) struct NamedProbabilityDocument<'a> {
    name: &'a str,
    probability: f64,
}
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "sessionObservedRow")
)]
#[serde(tag = "kind", content = "value", rename_all = "lowercase")]
pub(crate) enum SessionObservedRowDocument<'a> {
    Judgment(JudgmentDocument<'a>),
    Annotated(Vec<AnnotationDocument<'a>>),
    Recognized(RecognitionDocument<'a>),
    Find(Option<usize>),
    Relations(Vec<EdgeDocument<'a>>),
}
impl<'a> SessionObservedRowDocument<'a> {
    pub(super) fn of(value: &'a OwnedObservedRow) -> Self {
        match value {
            OwnedObservedRow::Judgment(value) => Self::Judgment(JudgmentDocument::of(value)),
            OwnedObservedRow::Annotated(values) => {
                Self::Annotated(values.iter().map(AnnotationDocument::of).collect())
            }
            OwnedObservedRow::Recognized(value) => Self::Recognized(RecognitionDocument::of(value)),
            OwnedObservedRow::Find(value) => Self::Find(*value),
            OwnedObservedRow::Relations(values) => {
                Self::Relations(values.iter().map(EdgeDocument::of).collect())
            }
        }
    }
}
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "sessionAnnotation")
)]
pub(crate) struct AnnotationDocument<'a> {
    name: &'a str,
    value: AnnotationValue<'a>,
}
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "kind", content = "value", rename_all = "lowercase")]
enum AnnotationValue<'a> {
    Decision(Option<bool>),
    Choice(Option<&'a str>),
    Score(f64),
    Tags(&'a [String]),
    Failed(core::BackendFailure),
}
impl<'a> AnnotationDocument<'a> {
    fn of(value: &'a NamedAnnotation) -> Self {
        let answer = match value.value() {
            Annotated::Decision(value) => AnnotationValue::Decision(decision(*value)),
            Annotated::Choice(value) => AnnotationValue::Choice(value.as_deref()),
            Annotated::Score(value) => AnnotationValue::Score(*value),
            Annotated::Tags(value) => AnnotationValue::Tags(value),
            Annotated::Failed(failed) => {
                return Self {
                    name: value.name(),
                    value: AnnotationValue::Failed(core::BackendFailure::new(cause(
                        failed.cause(),
                    ))),
                };
            }
        };
        Self {
            name: value.name(),
            value: answer,
        }
    }
}
fn cause(value: FailureCause) -> core::BackendFailureCause {
    match value {
        FailureCause::MissingAnswer => core::BackendFailureCause::MissingAnswer,
        FailureCause::WrongKind => core::BackendFailureCause::WrongKind,
        FailureCause::MissingProbability => core::BackendFailureCause::MissingProbability,
        FailureCause::InvalidProbability => core::BackendFailureCause::InvalidProbability,
        FailureCause::InvalidDistribution => core::BackendFailureCause::InvalidDistribution,
        FailureCause::UnexpectedProbability => core::BackendFailureCause::UnexpectedProbability,
    }
}
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "sessionRecognition")
)]
pub(crate) struct RecognitionDocument<'a> {
    entities: Vec<&'a core::RecognizedName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relations: Option<Vec<RecognitionEdgeDocument<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proposals: Option<Vec<&'a core::BoundaryProposal>>,
}
impl<'a> RecognitionDocument<'a> {
    fn of(value: &'a crate::Recognized) -> Self {
        Self {
            entities: value.entities().iter().map(|v| &v.0).collect(),
            relations: value.relations().map(|rows| {
                rows.iter()
                    .map(|v| RecognitionEdgeDocument {
                        relation: v.relation(),
                        source: &v.source().0,
                        target: &v.target().0,
                        probability: v.probability(),
                        either: v.either(),
                    })
                    .collect()
            }),
            proposals: value
                .proposals()
                .map(|rows| rows.iter().map(|v| &v.0).collect()),
        }
    }
}
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
struct RecognitionEdgeDocument<'a> {
    relation: &'a str,
    source: &'a core::RecognizedName,
    target: &'a core::RecognizedName,
    probability: f64,
    either: bool,
}
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "sessionRelationEdge")
)]
pub(crate) struct EdgeDocument<'a> {
    relation: &'a str,
    source: EntityDocument<'a>,
    target: EntityDocument<'a>,
    probability: f64,
    either: bool,
}
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
struct EntityDocument<'a> {
    name: &'a str,
    kind: &'a str,
}
impl<'a> EdgeDocument<'a> {
    fn of(value: &'a crate::Edge) -> Self {
        Self {
            relation: value.relation(),
            source: EntityDocument {
                name: value.source().name(),
                kind: value.source().kind(),
            },
            target: EntityDocument {
                name: value.target().name(),
                kind: value.target().kind(),
            },
            probability: value.probability(),
            either: value.either(),
        }
    }
}
