use std::io::BufRead;

use crate::core::{
    EntitySetError, Framing, Reading, Record, RecordError, RelateSpec, RelationEntity,
};
use crate::edge::Chunks;
use crate::failure::Failure;
use crate::table::{Kind as TableKind, Rows as TableRows};

pub(super) fn read(
    source: Box<dyn BufRead + Send>,
    framing: Framing,
    spec: &RelateSpec,
) -> Result<Entities, Failure> {
    let records = match framing {
        Framing::Document => document(source, spec)?,
        Framing::Lines => return lines(source, spec),
        Framing::Jsonl => stream(source, spec)?,
        Framing::Csv => table(source, TableKind::Csv)?,
        Framing::Tsv => table(source, TableKind::Tsv)?,
    };
    let mut pairs = Vec::new();
    let mut retained = 0usize;
    for record in &records {
        retain(record, &mut retained)?;
        record.validate_item(spec.metadata.item_schema.as_ref())?;
        pairs.push((
            name_of(record, spec)?.to_owned(),
            record.entity_text(spec.kind_field())?.to_owned(),
        ));
    }
    Ok((admitted(spec, &pairs)?, records))
}

/// The entity's name. A name `recognize` found carries `text` in place of
/// `name`, so a record with no `name` at the default field reads its `text`.
pub(super) fn name_of<'a>(record: &'a Record, spec: &RelateSpec) -> Result<&'a str, RecordError> {
    spec.record_name(record)
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

fn lines(source: Box<dyn BufRead + Send>, spec: &RelateSpec) -> Result<Entities, Failure> {
    let reading = Reading::new(Framing::Lines, Vec::new())?;
    let mut pairs = Vec::new();
    let mut records = Vec::new();
    let mut retained = 0usize;
    for bytes in Chunks::new(source, true) {
        let bytes = bytes?;
        let record = reading.record(&bytes)?;
        retain(&record, &mut retained)?;
        record.validate_item(spec.metadata.item_schema.as_ref())?;
        pairs.push((reading.as_it_arrived(&bytes)?.to_owned(), "*".to_owned()));
        records.push(record);
    }
    Ok((admitted(spec, &pairs)?, records))
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

fn retain(record: &Record, total: &mut usize) -> Result<(), Failure> {
    let original = crate::public::RawRecord(std::sync::Arc::new(record.clone()));
    let bytes = original
        .retained_bytes()
        .map_err(|_| Failure::Defect("an original record could not be measured"))?;
    *total = total
        .checked_add(bytes)
        .filter(|bytes| *bytes <= crate::core::MAX_RECORD_BYTES)
        .ok_or(Failure::Usage("relate input exceeds 16 MiB"))?;
    Ok(())
}

pub(super) type Entities = (Vec<RelationEntity>, Vec<Record>);
