//! Span coordinates keep saved characters independent of the host's string units.
use super::{AnnotationEntry, AnyRow, Outcome, values::*};
use crate::core::OffsetUnit;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Offset {
    canonical: usize,
    host: usize,
}
impl Offset {
    pub(crate) const fn canonical(self) -> usize {
        self.canonical
    }
    pub(crate) const fn host(self) -> usize {
        self.host
    }
}
impl From<usize> for Offset {
    fn from(value: usize) -> Self {
        Self {
            canonical: value,
            host: value,
        }
    }
}
impl PartialEq for Offset {
    fn eq(&self, other: &Self) -> bool {
        self.canonical == other.canonical
    }
}
impl Serialize for Offset {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.canonical.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for Offset {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        usize::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for Offset {
    fn inline_schema() -> bool {
        usize::inline_schema()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        usize::schema_id()
    }
    fn schema_name() -> std::borrow::Cow<'static, str> {
        usize::schema_name()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        usize::json_schema(generator)
    }
}
trait Coordinates {
    fn coordinates(&mut self, visit: &mut impl FnMut(&mut Offset, &mut Offset));
}
macro_rules! leaf {
    ($($ty:ty),+ $(,)?) => { $(impl Coordinates for $ty {
        fn coordinates(&mut self, visit: &mut impl FnMut(&mut Offset, &mut Offset)) {
            visit(&mut self.start, &mut self.end);
        }
    })+ };
}
leaf!(Span, Name, RelationName, PieceDetail, KindDetail);
impl<T: Coordinates> Coordinates for Vec<T> {
    fn coordinates(&mut self, visit: &mut impl FnMut(&mut Offset, &mut Offset)) {
        for value in self {
            value.coordinates(visit);
        }
    }
}
impl<T: Coordinates> Coordinates for Option<T> {
    fn coordinates(&mut self, visit: &mut impl FnMut(&mut Offset, &mut Offset)) {
        if let Some(value) = self {
            value.coordinates(visit);
        }
    }
}
impl Coordinates for Relation {
    fn coordinates(&mut self, visit: &mut impl FnMut(&mut Offset, &mut Offset)) {
        self.source.coordinates(visit);
        self.target.coordinates(visit);
    }
}
impl Coordinates for RecognizeDetail {
    fn coordinates(&mut self, visit: &mut impl FnMut(&mut Offset, &mut Offset)) {
        self.pieces.coordinates(visit);
        self.names.coordinates(visit);
    }
}
impl Coordinates for PairDetail {
    fn coordinates(&mut self, visit: &mut impl FnMut(&mut Offset, &mut Offset)) {
        self.names.coordinates(visit);
        self.pairs.coordinates(visit);
    }
}
fn outcome<A: Coordinates, D: Coordinates>(
    value: &mut Outcome<A, D>,
    visit: &mut impl FnMut(&mut Offset, &mut Offset),
) {
    match value {
        Outcome::Answered { answer, detail, .. } => {
            answer.coordinates(visit);
            detail.coordinates(visit);
        }
        Outcome::Unsure { detail, .. } => detail.coordinates(visit),
        Outcome::Failed { .. } => {}
    }
}
// Find has no detail, so its outcome does not need a generic empty detail visitor.
fn found(
    value: &mut Outcome<Option<Span>, super::NoDetail>,
    visit: &mut impl FnMut(&mut Offset, &mut Offset),
) {
    if let Outcome::Answered { answer, .. } = value {
        answer.coordinates(visit);
    }
}
fn entry(value: &mut AnnotationEntry, visit: &mut impl FnMut(&mut Offset, &mut Offset)) {
    match value {
        AnnotationEntry::Find(value) => found(value, visit),
        AnnotationEntry::Recognize(value) => outcome(value, visit),
        AnnotationEntry::Relate(value) => outcome(value, visit),
        _ => {}
    }
}
fn row(value: &mut AnyRow, visit: &mut impl FnMut(&mut Offset, &mut Offset)) {
    match value {
        AnyRow::Find(value) => found(&mut value.outcome, visit),
        AnyRow::Recognize(value) => outcome(&mut value.outcome, visit),
        AnyRow::Relate(value) => outcome(&mut value.outcome, visit),
        AnyRow::Annotate(value) => {
            if let Outcome::Answered { answer, .. } = &mut value.outcome {
                for value in answer.entries_mut().values_mut() {
                    entry(value, visit);
                }
            }
        }
        _ => {}
    }
}
fn projection(text: &str, unit: OffsetUnit) -> impl FnMut(&mut Offset, &mut Offset) + '_ {
    move |start, end| {
        (start.host, end.host) =
            crate::core::spans::span(text, start.canonical, end.canonical, unit);
    }
}
pub(crate) fn project(value: &mut AnyRow, text: &str, unit: OffsetUnit) {
    row(value, &mut projection(text, unit));
}
pub(crate) fn project_entry(value: &mut AnnotationEntry, text: &str, unit: OffsetUnit) {
    entry(value, &mut projection(text, unit));
}
#[cfg(feature = "native")]
pub(crate) fn host(value: &mut AnyRow) -> Vec<(usize, usize)> {
    let mut canonical = Vec::new();
    row(value, &mut |start, end| {
        canonical.push((start.canonical, end.canonical));
        start.canonical = start.host;
        end.canonical = end.host;
    });
    canonical
}
#[cfg(feature = "native")]
pub(crate) fn host_entry(value: &mut AnnotationEntry) {
    entry(value, &mut |start, end| {
        start.canonical = start.host;
        end.canonical = end.host;
    });
}
#[cfg(feature = "native")]
pub(crate) fn restore(
    value: &mut AnyRow,
    canonical: &[(usize, usize)],
) -> Result<(), super::Error> {
    let mut positions = canonical.iter();
    let mut complete = true;
    row(value, &mut |start, end| {
        if let Some(&(a, b)) = positions.next() {
            start.canonical = a;
            end.canonical = b;
        } else {
            complete = false;
        }
    });
    if complete && positions.next().is_none() {
        Ok(())
    } else {
        Err(super::mismatch())
    }
}

pub(crate) fn project_form(
    value: &mut AnyRow,
    mut text: impl FnMut(&str) -> Option<String>,
    unit: OffsetUnit,
) {
    if let AnyRow::Annotate(value) = value
        && let Outcome::Answered { answer, .. } = &mut value.outcome
    {
        for (name, value) in answer.entries_mut() {
            if let Some(text) = text(name) {
                project_entry(value, &text, unit);
            }
        }
    }
}
