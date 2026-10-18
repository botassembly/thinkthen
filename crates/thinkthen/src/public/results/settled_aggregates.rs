//! Aggregate settled values borrow the same canonical value serializers as details.
use super::settled::{
    AnswerResolution, AnswerResolutions, Settled, SettledAnswer, SettledOriginal,
};
use crate::{CompleteRecognized, CompleteRelated, RequestValue};
use serde::{Serialize, Serializer};

/// The bare recognition value with the same physical coordinates as its details.
#[derive(Clone, Debug)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(with = "crate::core::RecognizedValue")
)]
pub struct SettledRecognition(pub(crate) Box<CompleteRecognized>);
impl SettledRecognition {
    /// Native names, relations or boundary proposals.
    #[must_use]
    pub const fn value(&self) -> &crate::Recognized {
        self.0.value()
    }
    /// Located value, when the caller supplied physical sources.
    #[must_use]
    pub const fn source_value(&self) -> Option<&super::SourceRecognition> {
        self.0.source_value()
    }
}
impl Serialize for SettledRecognition {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0.source_value() {
            Some(value) => value.serialize(serializer),
            None => self.0.canonical.value.serialize(serializer),
        }
    }
}
/// The bare selected relation edges, retaining physical endpoints when present.
#[derive(Clone, Debug)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(with = "Vec<crate::core::RelationEdge<crate::core::RelationEntity>>")
)]
pub struct SettledRelations(pub(crate) Box<CompleteRelated>);
impl SettledRelations {
    /// Native selected edges.
    #[must_use]
    pub fn value(&self) -> &[crate::Edge] {
        self.0.value()
    }
    /// Located selected edges, when the caller supplied physical sources.
    #[must_use]
    pub fn source_edges(&self) -> Option<&[super::SourceRelationEdge]> {
        self.0.source_edges()
    }
}
impl Serialize for SettledRelations {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0.source_edges() {
            Some(edges) => edges.serialize(serializer),
            None => self.0.canonical.value.serialize(serializer),
        }
    }
}
impl SettledAnswer {
    pub(crate) fn of(value: &RequestValue) -> Option<Self> {
        let resolved = || AnswerResolutions::State(AnswerResolution::Resolved);
        Some(match value {
            RequestValue::Ranked(rows) => Self::Rank(rank_answer(
                rows.iter()
                    .map(|row| (row.ordinal(), row.result().probability())),
            )),
            RequestValue::SetRanked(rows) => Self::Rank(rank_answer(
                rows.iter()
                    .map(|row| (row.ordinal(), row.result().probability())),
            )),
            RequestValue::Found(found) => Self::Find(find_answer(found)),
            RequestValue::Recognized(rows) => {
                Self::Recognize(Settled::rows(rows.iter().map(|row| {
                    Settled::one(
                        SettledRecognition(Box::new(row.result().clone())),
                        row.result().probability(),
                        None,
                        None,
                        resolved(),
                    )
                })))
            }
            RequestValue::Related(row) => Self::Relate(Settled::one(
                SettledRelations(Box::new(row.result().clone())),
                row.result().probability(),
                None,
                None,
                resolved(),
            )),
            rows @ (RequestValue::Decisions(_)
            | RequestValue::Choices(_)
            | RequestValue::Tags(_)
            | RequestValue::Scores(_)
            | RequestValue::Filtered(_)
            | RequestValue::Annotations(_)) => return Self::rows(rows),
        })
    }
    pub(crate) fn append(&mut self, other: Self) {
        match (self, other) {
            (Self::Decide(left), Self::Decide(right)) => left.append(right),
            (Self::Choose(left), Self::Choose(right)) => left.append(right),
            (Self::Tag(left), Self::Tag(right)) => left.append(right),
            (Self::Score(left), Self::Score(right)) => left.append(right),
            (Self::Annotate(left), Self::Annotate(right)) => left.append(right),
            (Self::Recognize(left), Self::Recognize(right)) => left.append(right),
            (Self::Filter(left), Self::Filter(right)) => {
                append_selected(&mut left.value, right.value);
                append_selected(&mut left.probability, right.probability);
            }
            (left, right) => *left = right,
        }
    }
    pub(crate) fn empty(function: crate::RequestFunction) -> Self {
        use crate::RequestFunction as Function;
        let resolved = || AnswerResolutions::State(AnswerResolution::Resolved);
        match function {
            Function::Decide => Self::Decide(Settled::rows(std::iter::empty())),
            Function::Choose => Self::Choose(Settled::rows(std::iter::empty())),
            Function::Tag => Self::Tag(Settled::rows(std::iter::empty())),
            Function::Score => Self::Score(Settled::rows(std::iter::empty())),
            Function::Annotate => Self::Annotate(Settled::rows(std::iter::empty())),
            Function::Recognize => Self::Recognize(Settled::rows(std::iter::empty())),
            Function::Filter => {
                Self::Filter(Settled::one(Vec::new(), Vec::new(), None, None, resolved()))
            }
            Function::Rank => {
                Self::Rank(Settled::one(Vec::new(), Vec::new(), None, None, resolved()))
            }
            Function::Find | Function::Relate => Self::failed(function),
        }
    }
    pub(crate) fn failed(function: crate::RequestFunction) -> Self {
        use crate::RequestFunction as Function;
        match function {
            Function::Decide => Self::Decide(Settled::rows(std::iter::once(Settled::failed()))),
            Function::Choose => Self::Choose(Settled::rows(std::iter::once(Settled::failed()))),
            Function::Tag => Self::Tag(Settled::rows(std::iter::once(Settled::failed()))),
            Function::Score => Self::Score(Settled::rows(std::iter::once(Settled::failed()))),
            Function::Annotate => Self::Annotate(Settled::rows(std::iter::once(Settled::failed()))),
            Function::Recognize => {
                Self::Recognize(Settled::rows(std::iter::once(Settled::failed())))
            }
            Function::Filter => Self::Filter(Settled::failed()),
            Function::Rank => Self::Rank(Settled::failed()),
            Function::Find => Self::Find(Settled::failed()),
            Function::Relate => Self::Relate(Settled::failed()),
        }
    }
    pub(crate) fn failed_occurrences(
        &mut self,
        failures: std::collections::BTreeMap<usize, AnswerResolution>,
    ) {
        if let Self::Filter(answer) = self {
            answer.resolution =
                super::AnswerShape::Scalar(AnswerResolutions::Occurrences(failures));
        }
    }
    pub(crate) fn scalar(&mut self) {
        match self {
            Self::Decide(answer) => answer.scalar(),
            Self::Choose(answer) => answer.scalar(),
            Self::Tag(answer) => answer.scalar(),
            Self::Score(answer) => answer.scalar(),
            Self::Annotate(answer) => answer.scalar(),
            Self::Recognize(answer) => answer.scalar(),
            Self::Filter(_) | Self::Rank(_) | Self::Find(_) | Self::Relate(_) => {}
        }
    }
}

fn append_selected<T>(
    left: &mut super::AnswerShape<Option<Vec<T>>>,
    right: super::AnswerShape<Option<Vec<T>>>,
) {
    match (left, right) {
        (super::AnswerShape::Scalar(Some(values)), super::AnswerShape::Scalar(Some(more))) => {
            values.extend(more)
        }
        (left @ super::AnswerShape::Scalar(None), right) => *left = right,
        _ => {}
    }
}

pub(crate) fn rank_answer(
    rows: impl Iterator<Item = (usize, Option<f64>)>,
) -> Settled<Vec<usize>, Vec<Option<f64>>> {
    let (indexes, probabilities) = rows.unzip();
    Settled::one(
        indexes,
        probabilities,
        None,
        None,
        AnswerResolutions::State(AnswerResolution::Resolved),
    )
}
pub(crate) fn find_answer<T>(
    found: &crate::CompleteFound<T>,
) -> Settled<Option<SettledOriginal>, Option<f64>> {
    let index = match found.selection() {
        crate::FindSelection::Unit(index) => Some(index),
        crate::FindSelection::None => None,
    };
    let value = index
        .and_then(|index| found.inputs.get(index))
        .map(|input| SettledOriginal(Box::new(input.as_ref().clone())));
    Settled::one(
        value,
        found.probability(),
        Some(found.raw_pick().to_owned()),
        index,
        AnswerResolutions::State(AnswerResolution::Resolved),
    )
}
