use std::io::BufRead;

use crate::core::{Framing, Reading, Record, RelateSpec, RelationEntity, RelationEntityView};
use crate::edge::Chunks;
use crate::failure::Failure;
use crate::table::{Kind as TableKind, Rows as TableRows};

const MAX_ENTITIES: usize = 255;

#[rustfmt::skip]
pub(super) fn read(
    source: Box<dyn BufRead + Send>, framing: Framing, spec: &RelateSpec,
) -> Result<Vec<RelationEntity>, Failure> {
    let records = match framing {
        Framing::Document => document(source, spec)?,
        Framing::Lines => return lines(source, spec),
        Framing::Jsonl => stream(source, spec)?,
        Framing::Csv => table(source, TableKind::Csv)?,
        Framing::Tsv => table(source, TableKind::Tsv)?,
    };
    let mut entities = Vec::new();
    for record in records {
        push(
            &mut entities,
            record.entity_text(spec.name_field())?,
            record.entity_text(spec.kind_field())?,
        )?;
    }
    validate_kinds(&entities, spec)?;
    Ok(entities)
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
    let mut entities = Vec::new();
    for bytes in Chunks::new(source, true) {
        let bytes = bytes?;
        push(&mut entities, reading.as_it_arrived(&bytes)?, "*")?;
    }
    validate_kinds(&entities, spec)?;
    Ok(entities)
}

fn push(entities: &mut Vec<RelationEntity>, name: &str, kind: &str) -> Result<(), Failure> {
    if entities.len() == MAX_ENTITIES {
        return Err(Failure::Usage("relate takes at most 255 entities"));
    }
    let entity = RelationEntity::new(name, kind)
        .map_err(|_| Failure::Usage("an entity name and kind are nonempty strings"))?;
    if entities
        .iter()
        .any(|held| held.name() == entity.name() && held.kind() == entity.kind())
    {
        return Err(Failure::Usage(
            "the complete entity set contains no duplicate name-and-kind identity",
        ));
    }
    entities.push(entity);
    Ok(())
}

fn validate_kinds(entities: &[RelationEntity], spec: &RelateSpec) -> Result<(), Failure> {
    if entities.is_empty() {
        return Ok(());
    }
    for rule in &spec.relations {
        for kind in [&rule.source, &rule.target] {
            if kind != "*" && !entities.iter().any(|entity| entity.kind() == kind) {
                return Err(Failure::Usage(
                    "a concrete relation kind is absent from the complete entity set",
                ));
            }
        }
    }
    Ok(())
}
