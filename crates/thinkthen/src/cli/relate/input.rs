use std::io::BufRead;

use crate::core::{Framing, Reading, Record, RelateSpec, RelationEntity};
use crate::edge::Chunks;
use crate::failure::Failure;
use crate::table::{Kind as TableKind, Rows as TableRows};

pub(super) fn read(
    source: Box<dyn BufRead + Send>,
    framing: Framing,
    spec: &RelateSpec,
) -> Result<Vec<RelationEntity>, Failure> {
    let records = match framing {
        Framing::Document => document(source, spec)?,
        Framing::Lines => return lines(source, spec),
        Framing::Jsonl => stream(source, spec)?,
        Framing::Csv => table(source, TableKind::Csv)?,
        Framing::Tsv => table(source, TableKind::Tsv)?,
    };
    let mut pairs = Vec::new();
    for record in records {
        pairs.push((
            record.entity_text(spec.name_field())?.to_owned(),
            record.entity_text(spec.kind_field())?.to_owned(),
        ));
    }
    admitted(spec, &pairs)
}

fn document(source: Box<dyn BufRead + Send>, spec: &RelateSpec) -> Result<Vec<Record>, Failure> {
    let reading = Reading::new(
        Framing::Document,
        vec![spec.name_field().clone(), spec.kind_field().clone()],
    )?;
    let bytes = Chunks::new(source, false)
        .next()
        .transpose()?
        .unwrap_or_default();
    reading.entity_document(&bytes).map_err(Failure::from)
}

fn stream(source: Box<dyn BufRead + Send>, spec: &RelateSpec) -> Result<Vec<Record>, Failure> {
    let reading = Reading::new(
        Framing::Jsonl,
        vec![spec.name_field().clone(), spec.kind_field().clone()],
    )?;
    Chunks::new(source, true)
        .map(|bytes| reading.record(&bytes?).map_err(Failure::from))
        .collect()
}

fn table(source: Box<dyn BufRead + Send>, kind: TableKind) -> Result<Vec<Record>, Failure> {
    TableRows::new(source, kind)?.collect()
}

fn lines(
    source: Box<dyn BufRead + Send>,
    spec: &RelateSpec,
) -> Result<Vec<RelationEntity>, Failure> {
    let reading = Reading::new(Framing::Lines, Vec::new())?;
    let mut pairs = Vec::new();
    for bytes in Chunks::new(source, true) {
        pairs.push((reading.as_it_arrived(&bytes?)?.to_owned(), "*".to_owned()));
    }
    admitted(spec, &pairs)
}

fn admitted(spec: &RelateSpec, pairs: &[(String, String)]) -> Result<Vec<RelationEntity>, Failure> {
    spec.admit(pairs)
        .map_err(|error| Failure::Relate(crate::failure::relate::Error::Entities(error)))
}
