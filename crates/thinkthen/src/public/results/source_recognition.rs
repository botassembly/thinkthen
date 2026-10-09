//! Physical recognition spans use the shared scalar-to-line mapper.
use crate::core;
use crate::public::{Error, RecognizedEntity, SourceLocation, SourceRecord};
use serde::{Serialize, Serializer};
use std::fmt;

/// Complete recognized names and relations with actual physical span coordinates.
#[derive(Clone, PartialEq, Serialize)]
pub struct SourceRecognition {
    #[serde(skip)]
    location: SourceLocation,
    #[serde(flatten)]
    value: SourceValue,
}
#[derive(Clone, PartialEq, Serialize)]
#[serde(untagged)]
enum SourceValue {
    Whole {
        entities: Vec<SourceRecognizedEntity>,
        #[serde(skip_serializing_if = "Option::is_none")]
        relations: Option<Vec<SourceRecognizedRelation>>,
    },
    BoundaryOnly {
        mode: core::BoundaryMode,
        proposals: Vec<SourceBoundaryProposal>,
    },
}
/// An unclassified proposal with its actual physical source coordinates.
#[derive(Clone, PartialEq)]
pub struct SourceBoundaryProposal {
    proposal: crate::BoundaryProposal,
    location: SourceLocation,
}
impl SourceBoundaryProposal {
    /// Original local scalar span and rounded valid-path probability.
    #[must_use]
    pub const fn proposal(&self) -> &crate::BoundaryProposal {
        &self.proposal
    }
    /// Actual physical source span.
    #[must_use]
    pub const fn location(&self) -> &SourceLocation {
        &self.location
    }
}
impl Serialize for SourceBoundaryProposal {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct View<'a> {
            #[serde(flatten)]
            proposal: &'a core::BoundaryProposal,
            #[serde(flatten)]
            location: &'a SourceLocation,
        }
        View {
            proposal: &self.proposal.0,
            location: &self.location,
        }
        .serialize(serializer)
    }
}
impl SourceRecognition {
    /// The selected execution mode.
    #[must_use]
    pub const fn mode(&self) -> crate::RecognitionMode {
        match self.value {
            SourceValue::Whole { .. } => crate::RecognitionMode::Whole,
            SourceValue::BoundaryOnly { .. } => crate::RecognitionMode::BoundaryOnly,
        }
    }
    /// Located proposals, absent for whole recognition.
    #[must_use]
    pub fn proposals(&self) -> Option<&[SourceBoundaryProposal]> {
        match &self.value {
            SourceValue::Whole { .. } => None,
            SourceValue::BoundaryOnly { proposals, .. } => Some(proposals),
        }
    }
    /// Physical location of the complete input unit.
    #[must_use]
    pub const fn location(&self) -> &SourceLocation {
        &self.location
    }
    /// Recognized names in text order.
    #[must_use]
    pub fn entities(&self) -> &[SourceRecognizedEntity] {
        match &self.value {
            SourceValue::Whole { entities, .. } => entities,
            SourceValue::BoundaryOnly { .. } => &[],
        }
    }
    /// Asked relations with both actual span endpoints, absent when no rules were supplied.
    #[must_use]
    pub fn relations(&self) -> Option<&[SourceRecognizedRelation]> {
        match &self.value {
            SourceValue::Whole { relations, .. } => relations.as_deref(),
            SourceValue::BoundaryOnly { .. } => None,
        }
    }

    pub(crate) fn of(
        value: &core::RecognizedValue,
        text: &str,
        location: &SourceLocation,
    ) -> Result<Self, Error> {
        let span_location = |start, end| {
            let lines = match (location.first_line(), location.last_line()) {
                (Some(first_line), Some(last_line)) => Some(
                    SourceRecord {
                        record: text,
                        file: String::new(),
                        first_line,
                        last_line,
                    }
                    .span_lines(start, end)?,
                ),
                (None, None) => None,
                _ => {
                    return Err(Error::defect(
                        "a recognition source has incomplete coordinates",
                    ));
                }
            };
            SourceLocation::new(
                location.file().to_owned(),
                lines.map(|lines| lines.0),
                lines.map(|lines| lines.1),
            )
        };
        let core::RecognizedValue::Whole {
            entities,
            relations,
        } = value
        else {
            let core::RecognizedValue::BoundaryOnly { mode, proposals } = value else {
                return Err(Error::defect("recognition has no value"));
            };
            return Ok(Self {
                location: location.clone(),
                value: SourceValue::BoundaryOnly {
                    mode: *mode,
                    proposals: proposals
                        .iter()
                        .map(|proposal| {
                            Ok(SourceBoundaryProposal {
                                proposal: crate::BoundaryProposal(proposal.clone()),
                                location: span_location(proposal.start, proposal.end)?,
                            })
                        })
                        .collect::<Result<_, Error>>()?,
                },
            });
        };
        let name = |name: &core::RecognizedName| {
            Ok(SourceRecognizedEntity {
                entity: RecognizedEntity(name.clone()),
                location: span_location(name.start, name.end)?,
            })
        };
        let entities = entities.iter().map(name).collect::<Result<_, Error>>()?;
        let relations = relations
            .as_ref()
            .map(|edges| {
                edges
                    .iter()
                    .map(|edge| {
                        Ok(SourceRecognizedRelation(core::RelationEdge {
                            relation: edge.relation.clone(),
                            source: name(&edge.source)?,
                            target: name(&edge.target)?,
                            probability: edge.probability,
                            either: edge.either,
                        }))
                    })
                    .collect::<Result<_, Error>>()
            })
            .transpose()?;
        Ok(Self {
            location: location.clone(),
            value: SourceValue::Whole {
                entities,
                relations,
            },
        })
    }
}
/// A recognized name with local scalar offsets and actual physical source range.
#[derive(Clone, PartialEq)]
pub struct SourceRecognizedEntity {
    entity: RecognizedEntity,
    location: SourceLocation,
}
impl SourceRecognizedEntity {
    /// Existing recognized entity, with unchanged local scalar offsets.
    #[must_use]
    pub const fn entity(&self) -> &RecognizedEntity {
        &self.entity
    }
    /// Actual containing physical span, omitting lines for an unlocated document range.
    #[must_use]
    pub const fn location(&self) -> &SourceLocation {
        &self.location
    }
}
impl Serialize for SourceRecognizedEntity {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct View<'a> {
            #[serde(flatten)]
            entity: &'a core::RecognizedName,
            #[serde(flatten)]
            location: &'a SourceLocation,
        }
        View {
            entity: &self.entity.0,
            location: &self.location,
        }
        .serialize(serializer)
    }
}
/// One relation between physically located recognized spans.
#[derive(Clone, PartialEq, Serialize)]
#[serde(transparent)]
pub struct SourceRecognizedRelation(core::RelationEdge<SourceRecognizedEntity>);
impl SourceRecognizedRelation {
    /// Authored relation name.
    #[must_use]
    pub fn relation(&self) -> &str {
        &self.0.relation
    }
    /// Actual source span.
    #[must_use]
    pub const fn source(&self) -> &SourceRecognizedEntity {
        &self.0.source
    }
    /// Actual target span.
    #[must_use]
    pub const fn target(&self) -> &SourceRecognizedEntity {
        &self.0.target
    }
    /// Observed relation probability.
    #[must_use]
    pub const fn probability(&self) -> f64 {
        self.0.probability
    }
    /// Whether the accepted edge holds both ways.
    #[must_use]
    pub const fn either(&self) -> bool {
        self.0.either
    }
}
macro_rules! withheld {
    ($($name:ident),+) => { $(impl fmt::Debug for $name {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct(stringify!($name)).finish_non_exhaustive()
        }
    })+ };
}
withheld!(
    SourceBoundaryProposal,
    SourceRecognition,
    SourceRecognizedEntity,
    SourceRecognizedRelation
);
