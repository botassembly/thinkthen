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
struct Relation<'a> {
    relation: &'a str,
    source: Entity<'a>,
    target: Entity<'a>,
    probability: f64,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    either: bool,
}
#[derive(Serialize)]
pub(super) struct Located<'a> {
    entities: Vec<Entity<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relations: Option<Vec<Relation<'a>>>,
}
pub(super) fn located<'a>(
    value: &'a RecognizedValue,
    position: &'a Position,
    text: &str,
) -> Result<Located<'a>, Failure> {
    let entities = value
        .entities
        .iter()
        .map(|name| entity(name, position, text))
        .collect::<Result<_, _>>()?;
    let relations = value
        .relations
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
    Ok(Located {
        entities,
        relations,
    })
}
fn entity<'a>(
    name: &'a RecognizedName,
    position: &'a Position,
    text: &str,
) -> Result<Entity<'a>, Failure> {
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
                    .span_lines(name.start, name.end)
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
    Ok(Entity {
        entity: name,
        coordinates: SourceFields {
            file: position.file.as_deref(),
            first_line: lines.map(|(first, _)| first),
            last_line: lines.map(|(_, last)| last),
        },
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
