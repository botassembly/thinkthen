use std::io::BufRead;

use crate::core::{Framing, Reading, Record, RelateSpec};
use crate::edge::Chunks;
use crate::failure::Failure;
use crate::table::{Kind as TableKind, Rows as TableRows};

pub(super) fn read(
    source: Box<dyn BufRead + Send>,
    framing: Framing,
    spec: &RelateSpec,
) -> Result<Vec<Record>, Failure> {
    let records = match framing {
        Framing::Document => document(source, spec)?,
        Framing::Lines => stream(source, Framing::Lines)?,
        Framing::Jsonl => stream(source, Framing::Jsonl)?,
        Framing::Csv => table(source, TableKind::Csv)?,
        Framing::Tsv => table(source, TableKind::Tsv)?,
    };
    Ok(records)
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

fn stream(source: Box<dyn BufRead + Send>, framing: Framing) -> Result<Vec<Record>, Failure> {
    let reading = Reading::new(framing, Vec::new())?;
    Chunks::new(source, true)
        .map(|bytes| reading.record(&bytes?).map_err(Failure::from))
        .collect()
}

fn table(source: Box<dyn BufRead + Send>, kind: TableKind) -> Result<Vec<Record>, Failure> {
    TableRows::new(source, kind)?
        .map(|row| row.map_err(Failure::from))
        .collect()
}
