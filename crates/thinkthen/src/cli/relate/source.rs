//! Complete located relation sets and duplicate occurrence expansion.

use std::io::{Read, Write};

use serde::Serialize;

use crate::args::Common;
use crate::cli::intake::{Data, Intake};
use crate::core::{Framing, Reading, Record, RelateSpec, RelationEntity, RelationEntityView};
use crate::failure::Failure;

pub(super) struct Sources {
    pub(super) entities: Vec<RelationEntity>,
    occurrences: Vec<(RelationEntity, Occurrence)>,
}

impl Sources {
    pub(super) fn location(&self, ordinal: usize) -> Result<crate::SourceLocation, crate::Error> {
        let (_, source) = self
            .occurrences
            .get(ordinal)
            .ok_or_else(|| crate::Error::defect("relate lost an occurrence"))?;
        crate::SourceLocation::new(
            source.file.clone().unwrap_or_default(),
            source.first_line,
            source.last_line,
        )
    }
}

type Selection = (Vec<RelationEntity>, Option<Sources>, Vec<Record>);

pub(super) fn selection(
    common: &Common,
    input: impl Read + Send + 'static,
    framing: Framing,
    spec: &RelateSpec,
) -> Result<Selection, Failure> {
    if common.located() || common.input.len() > 1 {
        let sources = read(common, input, framing, spec)?;
        let inputs = sources
            .occurrences
            .iter()
            .map(|(_, occurrence)| occurrence.record.clone())
            .collect();
        Ok((sources.entities.clone(), Some(sources), inputs))
    } else {
        let source =
            crate::edge::source(common.input.first().map(std::path::PathBuf::as_path), input)?;
        let (entities, inputs) = super::input::read(source, framing, spec)?;
        Ok((entities, None, inputs))
    }
}

pub(super) fn read(
    common: &Common,
    input: impl Read + Send + 'static,
    framing: Framing,
    spec: &RelateSpec,
) -> Result<Sources, Failure> {
    let text = matches!(framing, Framing::Lines | Framing::Document);
    if text {
        spec.check_lines()
            .map_err(|error| Failure::Relate(crate::failure::relate::Error::Entities(error)))?;
    }
    let reading = Reading::new(framing, Vec::new())?;
    let intake = Intake::new(common, &reading, input, false)?;
    let mut pairs = Vec::new();
    let mut occurrences = Vec::new();
    let mut evidence_bytes = 0usize;
    for item in intake {
        let item = item.map_err(|placed| placed.cause)?;
        let Data::Bytes(bytes) = item.data else {
            return Err(Failure::Usage(
                "located relate takes text or explicit JSONL records",
            ));
        };
        let original = reading.as_it_arrived(&bytes)?;
        if occurrences.len() == 255 {
            return Err(Failure::Usage(
                "source relate takes at most 255 source records",
            ));
        }
        evidence_bytes = evidence_bytes
            .checked_add(original.len())
            .filter(|&bytes| bytes <= crate::core::MAX_RECORD_BYTES)
            .ok_or(Failure::Usage("source relate input exceeds 16 MiB"))?;
        let record = reading.record(&bytes).map_err(Failure::from)?;
        record.validate_item(spec.metadata.item_schema.as_ref())?;
        let (name, kind) = if text {
            (reading.as_it_arrived(&bytes)?, "*")
        } else {
            (
                super::input::name_of(&record, spec)?,
                record.entity_text(spec.kind_field())?,
            )
        };
        let entity = RelationEntity::new(name, kind)
            .map_err(|_| Failure::Usage("relate source names and kinds must be nonblank"))?;
        let pair = (name.to_owned(), kind.to_owned());
        if !pairs.contains(&pair) {
            pairs.push(pair);
            if pairs.len() > 255 {
                super::input::admitted(spec, &pairs)?;
            }
        }
        let position = item
            .position
            .ok_or(Failure::Defect("relate source has no position"))?;
        occurrences.push((
            entity,
            Occurrence {
                ordinal: occurrences.len(),
                record,
                file: position.file,
                first_line: position.first,
                last_line: position.last,
            },
        ));
    }
    Ok(Sources {
        entities: super::input::admitted(spec, &pairs)?,
        occurrences,
    })
}

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
struct Occurrence {
    ordinal: usize,
    record: Record,
    file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_line: Option<usize>,
}

#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "sourceRelationEndpoint")
)]
pub(crate) struct Endpoint<'a> {
    name: &'a str,
    kind: &'a str,
    #[serde(flatten)]
    source: &'a Occurrence,
}

#[derive(Serialize)]
struct Edge<'a> {
    relation: &'a str,
    source: Endpoint<'a>,
    target: Endpoint<'a>,
    probability: f64,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    either: bool,
}

pub(super) fn write(
    writer: &mut dyn Write,
    details: bool,
    canonical: &crate::core::CompleteRelation,
    sources: &Sources,
) -> Result<(), Failure> {
    let mut edges = Vec::new();
    let mut budget = crate::result_json::bounded::OutputBudget(
        crate::core::MAX_RECORD_BYTES - usize::from(details) * 2,
    );
    for edge in &canonical.value {
        for source in sources
            .occurrences
            .iter()
            .filter(|(entity, _)| entity == &edge.source)
        {
            for target in sources
                .occurrences
                .iter()
                .filter(|(entity, _)| entity == &edge.target)
            {
                let located = Edge {
                    relation: &edge.relation,
                    source: Endpoint {
                        name: source.0.name(),
                        kind: source.0.kind(),
                        source: &source.1,
                    },
                    target: Endpoint {
                        name: target.0.name(),
                        kind: target.0.kind(),
                        source: &target.1,
                    },
                    probability: edge.probability,
                    either: edge.either,
                };
                // Count escaped JSON bytes using borrowed evidence before retaining
                // the Cartesian expansion or writing any part of this complete set.
                budget
                    .admit(&located, !details || !edges.is_empty())
                    .map_err(|()| Failure::Usage("source relate output exceeds 16 MiB"))?;
                edges.push(located);
            }
        }
    }
    if details {
        let located = Complete {
            canonical,
            value: &edges,
            originals: sources
                .occurrences
                .iter()
                .map(|(_, occurrence)| occurrence)
                .collect(),
        };
        crate::edge::write_line(writer, &crate::core::json_line(&located)?)?;
    } else {
        for edge in edges {
            if !crate::edge::write_line(&mut *writer, &crate::core::json_line(&edge)?)? {
                break;
            }
        }
    }
    Ok(())
}

struct Complete<'a> {
    canonical: &'a crate::core::CompleteRelation,
    value: &'a Vec<Edge<'a>>,
    originals: Vec<&'a Occurrence>,
}
impl Serialize for Complete<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.canonical
            .serialize_occurrence(Some(&self.originals), self.value, Some(0), serializer)
    }
}
