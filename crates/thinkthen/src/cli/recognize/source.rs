//! Typed physical span presentation; evidence and answer identity stay unchanged.
use crate::SourceRecord;
use crate::cli::intake::{Position, SourceFields};
use crate::core::{self, RecognizedName, RecognizedValue, RelationEdge};
use crate::failure::Failure;
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub(super) struct Entity<'a> {
    #[serde(flatten)]
    entity: &'a RecognizedName,
    #[serde(flatten)]
    coordinates: SourceFields<'a>,
}
#[derive(Serialize)]
pub(super) struct Relation<'a> {
    relation: &'a str,
    source: Entity<'a>,
    target: Entity<'a>,
    probability: f64,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    either: bool,
}
#[derive(Serialize)]
#[serde(untagged)]
pub(super) enum Located<'a> {
    Whole {
        entities: Vec<Entity<'a>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        relations: Option<Vec<Relation<'a>>>,
    },
    BoundaryOnly {
        mode: core::BoundaryMode,
        proposals: Vec<Proposal<'a>>,
    },
}
#[derive(Serialize)]
pub(super) struct Proposal<'a> {
    #[serde(flatten)]
    proposal: &'a core::BoundaryProposal,
    #[serde(flatten)]
    coordinates: SourceFields<'a>,
}
pub(super) fn located<'a>(
    value: &'a RecognizedValue,
    position: &'a Position,
    text: &str,
) -> Result<Located<'a>, Failure> {
    let RecognizedValue::Whole {
        entities,
        relations,
    } = value
    else {
        let RecognizedValue::BoundaryOnly { proposals, mode } = value else {
            return Err(Failure::Defect("recognition has no value"));
        };
        return Ok(Located::BoundaryOnly {
            mode: *mode,
            proposals: proposals
                .iter()
                .map(|proposal| {
                    Ok(Proposal {
                        proposal,
                        coordinates: coordinates(proposal.start, proposal.end, position, text)?,
                    })
                })
                .collect::<Result<_, Failure>>()?,
        });
    };
    let entities = entities
        .iter()
        .map(|name| entity(name, position, text))
        .collect::<Result<_, _>>()?;
    let relations = relations
        .as_ref()
        .map(|edges| {
            edges
                .iter()
                .map(|edge: &RelationEdge<_>| {
                    Ok(Relation {
                        relation: &edge.relation,
                        source: entity(&edge.source, position, text)?,
                        target: entity(&edge.target, position, text)?,
                        probability: edge.probability,
                        either: edge.either,
                    })
                })
                .collect::<Result<_, Failure>>()
        })
        .transpose()?;
    Ok(Located::Whole {
        entities,
        relations,
    })
}
fn entity<'a>(
    name: &'a RecognizedName,
    position: &'a Position,
    text: &str,
) -> Result<Entity<'a>, Failure> {
    Ok(Entity {
        entity: name,
        coordinates: coordinates(name.start, name.end, position, text)?,
    })
}
fn coordinates<'a>(
    start: usize,
    end: usize,
    position: &'a Position,
    text: &str,
) -> Result<SourceFields<'a>, Failure> {
    let lines = match (position.first, position.last) {
        (Some(first_line), Some(last_line)) => {
            let record = SourceRecord {
                record: text,
                file: String::new(),
                first_line,
                last_line,
            };
            Some(
                record
                    .span_lines(start, end)
                    .map_err(|_| Failure::Defect("recognized span is outside its source"))?,
            )
        }
        (None, None) => None,
        _ => {
            return Err(Failure::Defect(
                "recognized source has incomplete coordinates",
            ));
        }
    };
    Ok(SourceFields {
        file: position.file.as_deref(),
        first_line: lines.map(|(first, _)| first),
        last_line: lines.map(|(_, last)| last),
    })
}
pub(super) struct Complete<'a> {
    pub(super) canonical: &'a core::CompleteRecognition,
    pub(super) value: Located<'a>,
}
impl Serialize for Complete<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.canonical
            .serialize_with_value(self.canonical.input.as_ref(), &self.value, serializer)
    }
}
