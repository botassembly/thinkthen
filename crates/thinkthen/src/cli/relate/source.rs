//! Complete located relation sets and duplicate occurrence expansion.

use std::io::{Read, Write};

use serde::Serialize;

use crate::SourceRecord;
use crate::args::Common;
use crate::cli::intake::{Data, Intake};
use crate::core::{Framing, Reading, Record, RelateSpec, RelationEntity, RelationEntityView};
use crate::engine::facade::Execution;
use crate::failure::Failure;

pub(super) struct Sources {
    pub(super) entities: Vec<RelationEntity>,
    occurrences: Vec<(RelationEntity, SourceRecord<Record>)>,
}

type Selection = (Vec<RelationEntity>, Option<Sources>);

pub(super) fn selection(
    common: &Common,
    input: impl Read + Send + 'static,
    framing: Framing,
    spec: &RelateSpec,
) -> Result<Selection, Failure> {
    if common.located() || common.input.len() > 1 {
        let sources = read(common, input, framing, spec)?;
        Ok((sources.entities.clone(), Some(sources)))
    } else {
        let source =
            crate::edge::source(common.input.first().map(std::path::PathBuf::as_path), input)?;
        Ok((super::input::read(source, framing, spec)?, None))
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
    for item in intake {
        let item = item.map_err(|placed| placed.cause)?;
        let Data::Bytes(bytes) = item.data else {
            return Err(Failure::Usage(
                "located relate takes text or explicit JSONL records",
            ));
        };
        let record = reading.record(&bytes).map_err(Failure::from)?;
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
            SourceRecord {
                record,
                file: position.file.unwrap_or_default(),
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
struct Endpoint<'a> {
    name: &'a str,
    kind: &'a str,
    #[serde(flatten)]
    source: &'a SourceRecord<Record>,
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
    output: &super::result::Output<'_>,
    execution: &Execution,
    sources: &Sources,
) -> Result<(), Failure> {
    let mut edges = Vec::new();
    for edge in &execution.edges {
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
                edges.push(Edge {
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
                });
            }
        }
    }
    if output.details {
        let mut details = serde_json::to_value(super::result::details(output, execution)?)
            .map_err(|_| Failure::Defect("relate details could not be written"))?;
        details
            .as_object_mut()
            .ok_or(Failure::Defect("relate details are not an object"))?
            .insert(
                "value".to_owned(),
                serde_json::to_value(&edges)
                    .map_err(|_| Failure::Defect("located edges could not be written"))?,
            );
        crate::edge::write_line(writer, &crate::core::json_line(&details)?)?;
    } else {
        for edge in edges {
            if !crate::edge::write_line(&mut *writer, &crate::core::json_line(&edge)?)? {
                break;
            }
        }
    }
    Ok(())
}
