//! Located edge occurrences retain native originals without reparsing result documents.
use crate::public::{Edge, Entity, Error, QuestionInput, RawRecord, SourceLocation};
use serde::{Serialize, Serializer};
use std::{fmt, sync::Arc};

/// One physical occurrence of an admitted relation endpoint.
#[derive(Clone, PartialEq)]
pub struct SourceRelationEndpoint(Arc<Occurrence>);
#[derive(PartialEq)]
struct Occurrence {
    ordinal: usize,
    entity: Entity,
    record: RawRecord,
    location: SourceLocation,
}
impl SourceRelationEndpoint {
    /// Zero-based position in the complete original input set.
    #[must_use]
    pub fn ordinal(&self) -> usize {
        self.0.ordinal
    }
    /// Actual selected name and kind used by the relation plan.
    #[must_use]
    pub fn entity(&self) -> &Entity {
        &self.0.entity
    }
    /// Complete original native content, including unselected fields.
    #[must_use]
    pub fn record(&self) -> &RawRecord {
        &self.0.record
    }
    /// Caller-supplied physical source, outside request and answer identity.
    #[must_use]
    pub fn location(&self) -> &SourceLocation {
        &self.0.location
    }
}
impl Serialize for SourceRelationEndpoint {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct View<'a> {
            ordinal: usize,
            name: &'a str,
            kind: &'a str,
            record: &'a RawRecord,
            #[serde(flatten)]
            location: &'a SourceLocation,
        }
        View {
            ordinal: self.ordinal(),
            name: self.entity().name(),
            kind: self.entity().kind(),
            record: self.record(),
            location: self.location(),
        }
        .serialize(serializer)
    }
}
/// One accepted edge expanded to a pair of physical input occurrences.
#[derive(Clone, PartialEq)]
pub struct SourceRelationEdge {
    edge: Edge,
    source: SourceRelationEndpoint,
    target: SourceRelationEndpoint,
}
impl SourceRelationEdge {
    /// The accepted semantic edge shared by these occurrences.
    #[must_use]
    pub const fn edge(&self) -> &Edge {
        &self.edge
    }
    /// Actual source occurrence.
    #[must_use]
    pub const fn source(&self) -> &SourceRelationEndpoint {
        &self.source
    }
    /// Actual target occurrence.
    #[must_use]
    pub const fn target(&self) -> &SourceRelationEndpoint {
        &self.target
    }
}
impl Serialize for SourceRelationEdge {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct View<'a> {
            relation: &'a str,
            source: &'a SourceRelationEndpoint,
            target: &'a SourceRelationEndpoint,
            probability: f64,
            #[serde(skip_serializing_if = "std::ops::Not::not")]
            either: bool,
        }
        View {
            relation: self.edge.relation(),
            source: &self.source,
            target: &self.target,
            probability: self.edge.probability(),
            either: self.edge.either(),
        }
        .serialize(serializer)
    }
}
impl fmt::Debug for SourceRelationEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceRelationEndpoint")
            .field("ordinal", &self.ordinal())
            .finish_non_exhaustive()
    }
}
impl fmt::Debug for SourceRelationEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceRelationEdge")
            .field("source", &self.source)
            .field("target", &self.target)
            .finish_non_exhaustive()
    }
}

pub(crate) fn expand(
    edges: &[Edge],
    inputs: &[Arc<QuestionInput>],
    pairs: &[(String, String)],
) -> Result<Vec<SourceRelationEdge>, Error> {
    let occurrences = inputs
        .iter()
        .zip(pairs)
        .enumerate()
        .map(|(ordinal, (input, pair))| {
            let QuestionInput::Record(input) = input.as_ref() else {
                return Err(Error::defect(
                    "a located relation input has no native record",
                ));
            };
            Ok(SourceRelationEndpoint(Arc::new(Occurrence {
                ordinal,
                entity: Entity::new(&pair.0, &pair.1)?,
                record: input.original().clone(),
                location: input
                    .location()
                    .cloned()
                    .ok_or_else(|| Error::defect("a located relation input has no source"))?,
            })))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let mut expanded = Vec::new();
    let mut budget = crate::result_json::bounded::OutputBudget(crate::core::MAX_RECORD_BYTES - 2);
    for edge in edges {
        for source in occurrences
            .iter()
            .filter(|source| source.entity() == edge.source())
        {
            for target in occurrences
                .iter()
                .filter(|target| target.entity() == edge.target())
            {
                let located = SourceRelationEdge {
                    edge: edge.clone(),
                    source: source.clone(),
                    target: target.clone(),
                };
                budget
                    .admit(&located, !expanded.is_empty())
                    .map_err(|()| Error::usage("source relate output exceeds 16 MiB"))?;
                expanded.push(located);
            }
        }
    }
    Ok(expanded)
}
