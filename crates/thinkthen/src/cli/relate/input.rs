use std::io::BufRead;

use crate::core::{
    EntitySetError, Framing, Pointer, Reading, Record, RecordError, RelateSpec, RelationEntity,
};
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
            name_of(&record, spec)?.to_owned(),
            record.entity_text(spec.kind_field())?.to_owned(),
        ));
    }
    admitted(spec, &pairs)
}

/// The entity's name. A name `recognize` found carries `text` in place of
/// `name`, so a record with no `name` at the default field reads its `text`.
pub(super) fn name_of<'a>(record: &'a Record, spec: &RelateSpec) -> Result<&'a str, RecordError> {
    let field = spec.name_field();
    match record.entity_text(field) {
        Err(missed @ RecordError::Missed(_)) if field.as_str() == "/name" => Pointer::new("/text")
            .ok()
            .and_then(|text| record.entity_text(&text).ok())
            .ok_or(missed),
        read => read,
    }
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

pub(super) fn admitted(
    spec: &RelateSpec,
    pairs: &[(String, String)],
) -> Result<Vec<RelationEntity>, Failure> {
    spec.admit(pairs).map_err(|error| {
        let error = match error {
            EntitySetError::TooMany => {
                crate::failure::relate::Error::TooMany { count: pairs.len() }
            }
            other => crate::failure::relate::Error::Entities(other),
        };
        Failure::Relate(error)
    })
}
