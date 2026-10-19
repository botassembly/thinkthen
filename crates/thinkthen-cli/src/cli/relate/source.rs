//! Complete located relation sets and duplicate occurrence expansion.

use std::io::{Read, Write};

use serde::Serialize;

use crate::args::Common;
use crate::cli::intake::{Data, Intake};
use crate::cli::reading::Reading;
use crate::failure::Failure;
use crate::{RawRecord as Record, RequestFraming as Framing};

pub(super) struct Sources {
    occurrences: Vec<Occurrence>,
}

impl Sources {
    pub(super) fn location(&self, ordinal: usize) -> Result<crate::SourceLocation, crate::Error> {
        let source = self.occurrences.get(ordinal).ok_or_else(|| {
            crate::Error::new(
                crate::ErrorKind::Defect,
                "defect: relate lost an occurrence",
            )
        })?;
        crate::SourceLocation::new(
            source.file.clone().unwrap_or_default(),
            source.first_line,
            source.last_line,
        )
    }
}

type Selection = (Option<Sources>, Vec<Record>);

pub(super) fn selection(
    common: &Common,
    input: impl Read + Send + 'static,
    framing: Framing,
    fields: &super::config::Fields,
) -> Result<Selection, Failure> {
    if common.located() || common.input.len() > 1 {
        let sources = read(common, input, framing)?;
        let inputs = sources
            .occurrences
            .iter()
            .map(|occurrence| occurrence.record.clone())
            .collect();
        Ok((Some(sources), inputs))
    } else {
        let source =
            crate::edge::source(common.input.first().map(std::path::PathBuf::as_path), input)?;
        let inputs = super::input::read(source, framing, fields)?;
        Ok((None, inputs))
    }
}

pub(super) fn read(
    common: &Common,
    input: impl Read + Send + 'static,
    framing: Framing,
) -> Result<Sources, Failure> {
    let reading = Reading::new(framing, Vec::new())?;
    let intake = Intake::new(common, &reading, input, false)?;
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
        if occurrences.len() == crate::Relate::max_record_count() {
            return Err(Failure::Usage(
                "source relate takes at most 255 source records",
            ));
        }
        evidence_bytes = evidence_bytes
            .checked_add(original.len())
            .filter(|&bytes| bytes <= crate::RawRecord::max_bytes())
            .ok_or(Failure::Usage("source relate input exceeds 16 MiB"))?;
        let record = reading.record(&bytes).map_err(Failure::from)?;
        let position = item
            .position
            .ok_or(Failure::Defect("relate source has no position"))?;
        occurrences.push(Occurrence {
            ordinal: occurrences.len(),
            record,
            file: position.file,
            first_line: position.first,
            last_line: position.last,
        });
    }
    Ok(Sources { occurrences })
}

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
struct Occurrence {
    ordinal: usize,
    #[cfg_attr(test, schemars(schema_with = "crate::schema_tests::record"))]
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
    result: &crate::CompleteRelated,
    sources: &Sources,
) -> Result<(), Failure> {
    let edges = result
        .source_edges()
        .ok_or(Failure::Defect("relate lost its source edges"))?;
    let mut budget = crate::Relate::max_source_output_bytes() - usize::from(details) * 2;
    let edges = edges
        .iter()
        .enumerate()
        .map(|(at, edge)| {
            let located = Edge {
                relation: edge.edge().relation(),
                source: endpoint(edge.source(), sources)?,
                target: endpoint(edge.target(), sources)?,
                probability: edge.edge().probability(),
                either: edge.edge().either(),
            };
            // The CLI spells an absent filename as null, so measure its rendered shape.
            budget = budget
                .checked_sub(
                    crate::render::json_line(&located)?.len() + usize::from(!details || at > 0),
                )
                .ok_or(Failure::Usage("source relate output exceeds 16 MiB"))?;
            Ok(located)
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    if details {
        super::result::details(writer, result, &sources.occurrences, Some(&edges))?;
    } else {
        for edge in edges {
            if !crate::edge::write_line(&mut *writer, &crate::render::json_line(&edge)?)? {
                break;
            }
        }
    }
    Ok(())
}

fn endpoint<'a>(
    endpoint: &'a crate::SourceRelationEndpoint,
    sources: &'a Sources,
) -> Result<Endpoint<'a>, Failure> {
    Ok(Endpoint {
        name: endpoint.entity().name(),
        kind: endpoint.entity().kind(),
        source: sources
            .occurrences
            .get(endpoint.ordinal())
            .ok_or(Failure::Defect("relate lost an occurrence"))?,
    })
}
