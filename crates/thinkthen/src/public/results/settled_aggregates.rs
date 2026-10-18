//! Aggregate settled values borrow the same canonical value serializers as details.
use super::settled::{AnswerResolution, AnswerResolutions, Settled, SettledAnswer, SettledOriginal};
use crate::{CompleteRecognized, CompleteRelated, RequestValue};
use serde::{Serialize, Serializer};

/// The bare recognition value with the same physical coordinates as its details.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(with = "crate::core::RecognizedValue"))]
pub struct SettledRecognition(pub(crate) CompleteRecognized);
impl SettledRecognition {
    /// Native names, relations or boundary proposals.
    #[must_use]
    pub const fn value(&self) -> &crate::Recognized { self.0.value() }
    /// Located value, when the caller supplied physical sources.
    #[must_use]
    pub const fn source_value(&self) -> Option<&super::SourceRecognition> { self.0.source_value() }
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
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(with = "Vec<crate::core::RelationEdge<crate::core::RelationEntity>>"))]
pub struct SettledRelations(pub(crate) CompleteRelated);
impl SettledRelations {
    /// Native selected edges.
    #[must_use]
    pub fn value(&self) -> &[crate::Edge] { self.0.value() }
    /// Located selected edges, when the caller supplied physical sources.
    #[must_use]
    pub fn source_edges(&self) -> Option<&[super::SourceRelationEdge]> { self.0.source_edges() }
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
            RequestValue::Ranked(rows) => Self::Rank(Settled::one(
                rows.iter().map(|row| row.ordinal()).collect(),
                rows.iter().map(|row| row.result().probability()).collect(),
                None, None, resolved(),
            )),
            RequestValue::SetRanked(rows) => Self::Rank(Settled::one(
                rows.iter().map(|row| row.ordinal()).collect(),
                rows.iter().map(|row| row.result().probability()).collect(),
                None, None, resolved(),
            )),
            RequestValue::Found(found) => Self::Find(Settled::one(
                found.selected().cloned().map(SettledOriginal), found.probability(),
                Some(found.raw_pick().to_owned()),
                match found.selection() { crate::FindSelection::Unit(index) => Some(index), crate::FindSelection::None => None }, resolved(),
            )),
            RequestValue::Recognized(rows) => Self::Recognize(Settled::rows(rows.iter().map(|row| {
                Settled::one(SettledRecognition(row.result().clone()), row.result().probability(), None, None, resolved())
            }))),
            RequestValue::Related(row) => Self::Relate(Settled::one(
                SettledRelations(row.result().clone()), row.result().probability(), None, None, resolved(),
            )),
            rows => return Self::rows(rows),
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
                if let (super::AnswerShape::Scalar(values), super::AnswerShape::Scalar(more)) = (&mut left.value, right.value) { values.extend(more); }
                if let (super::AnswerShape::Scalar(values), super::AnswerShape::Scalar(more)) = (&mut left.probability, right.probability) { values.extend(more); }
            }
            (left, right) => *left = right,
        }
    }
    pub(crate) fn scalar(&mut self) {
        match self {
            Self::Decide(answer) => answer.scalar(), Self::Choose(answer) => answer.scalar(),
            Self::Tag(answer) => answer.scalar(), Self::Score(answer) => answer.scalar(),
            Self::Annotate(answer) => answer.scalar(), Self::Recognize(answer) => answer.scalar(),
            Self::Filter(_) | Self::Rank(_) | Self::Find(_) | Self::Relate(_) => {}
        }
    }
}
