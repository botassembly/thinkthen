//! Typed physical span presentation; evidence and answer identity stay unchanged.
use crate::SourceRecord;
use crate::cli::intake::{Position, SourceFields};
use crate::failure::Failure;
use crate::{RecognitionValue, Recognized, RecognizedEntity};
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub(super) struct Entity<'a> {
    text: &'a str,
    start: usize,
    end: usize,
    length: usize,
    kind: &'a str,
    strength: f64,
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
        mode: crate::RecognitionMode,
        proposals: Vec<Proposal<'a>>,
    },
}
#[derive(Serialize)]
pub(super) struct Proposal<'a> {
    text: &'a str,
    start: usize,
    end: usize,
    length: usize,
    probability: f64,
    #[serde(flatten)]
    coordinates: SourceFields<'a>,
}
pub(super) fn located<'a>(
    value: &'a Recognized,
    position: &'a Position,
    text: &str,
) -> Result<Located<'a>, Failure> {
    let RecognitionValue::Whole {
        entities,
        relations,
    } = value.value()
    else {
        let RecognitionValue::BoundaryOnly(proposals) = value.value() else {
            return Err(Failure::Defect("recognition has no value"));
        };
        return Ok(Located::BoundaryOnly {
            mode: value.mode(),
            proposals: proposals
                .iter()
                .map(|proposal| {
                    Ok(Proposal {
                        text: proposal.text(),
                        start: proposal.start(),
                        end: proposal.end(),
                        length: proposal.length(),
                        probability: proposal.probability(),
                        coordinates: coordinates(proposal.start(), proposal.end(), position, text)?,
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
                .map(|edge| {
                    Ok(Relation {
                        relation: edge.relation(),
                        source: entity(edge.source(), position, text)?,
                        target: entity(edge.target(), position, text)?,
                        probability: edge.probability(),
                        either: edge.either(),
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
    name: &'a RecognizedEntity,
    position: &'a Position,
    text: &str,
) -> Result<Entity<'a>, Failure> {
    Ok(Entity {
        text: name.text(),
        start: name.start(),
        end: name.end(),
        length: name.length(),
        kind: name.kind(),
        strength: name.strength(),
        coordinates: coordinates(name.start(), name.end(), position, text)?,
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
    pub(super) canonical: &'a crate::CompleteRecognized,
    pub(super) value: Option<Located<'a>>,
    pub(super) input: Option<&'a crate::RawRecord>,
}
impl Complete<'_> {
    fn write_value<M: serde::ser::SerializeMap>(
        &self,
        map: &mut M,
        name: &str,
        value: &crate::saved_json::Json,
    ) -> Result<(), M::Error> {
        if let Some(located) = &self.value {
            map.serialize_entry(name, located)?;
        } else {
            map.serialize_entry(name, value)?;
        }
        if let Some(input) = self.input {
            map.serialize_entry("input", input)?;
        }
        Ok(())
    }
}

impl Serialize for Complete<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::{Error as _, SerializeMap as _};
        let text = self.canonical.to_json().map_err(S::Error::custom)?;
        let document = crate::saved_json::Json::parse(&text).map_err(S::Error::custom)?;
        let crate::saved_json::Kind::Object(fields) = document.kind() else {
            return Err(S::Error::custom("a complete recognition is not an object"));
        };
        let mut map = serializer.serialize_map(None)?;
        for (name, value) in fields {
            if name == "input" || name == "index" {
                continue;
            }
            if name == "value" {
                self.write_value(&mut map, name, value)?;
            } else {
                map.serialize_entry(name, value)?;
            }
        }
        map.end()
    }
}
