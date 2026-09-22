//! `thinkthen_relations(body, spec)`: the relations `recognize` found, as
//! a list of structs, so `unnest()` makes rows — the same shape
//! `thinkthen_recognize` takes, because the stable C API gives a table
//! function only literal parameters (no column argument, no lateral
//! join), while a scalar takes a column like any other function.
//!
//! Beta, per the design page: `(name, source, source_kind, target,
//! target_kind, probability)`, and the relation rules come
//! from the question file's `recognize` section (`'@names.json'`). The
//! deck's line is `SELECT * FROM thinkthen_relations(body, '@names.json')`
//! with a bare `body`, which cannot bind without a FROM and is not a
//! table function; the working call is `SELECT t.id,
//! unnest(thinkthen_relations(t.body, '@names.json')) AS r FROM tickets
//! t`. The finding is pinned in NOTES.md.

use std::error::Error;

use duckdb::core::{DataChunkHandle, Inserter, LogicalTypeHandle, LogicalTypeId};
use duckdb::vscalar::{ScalarFunctionSignature, VScalar};
use duckdb::vtab::arrow::WritableVector;
use thinkthen_contract::{Error as EngineError, Recognize};

use crate::{engine_call, failure, options, read_strings};

/// One relation as a row-shaped struct.
type RelationRow = (String, String, String, String, String, f64);

/// `thinkthen_relations(body, spec)`: one list of structs per row.
pub(crate) struct RelationsScalar;

impl VScalar for RelationsScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> Result<(), Box<dyn Error>> {
        let bodies = read_strings(input, 0);
        let specs = read_strings(input, 1);
        let mut rows: Vec<Option<Vec<RelationRow>>> = Vec::with_capacity(bodies.len());
        for (body, spec) in bodies.iter().zip(specs.iter()) {
            let (Some(body), Some(spec)) = (body.as_deref(), spec.as_deref()) else {
                rows.push(None);
                continue;
            };
            let ask = build_ask(spec)?;
            let found = engine_call(|engine| engine.recognize_opts(&ask, body, options()))?;
            rows.push(Some(relation_rows(&found)?));
        }
        write_lists(output, &rows);
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![LogicalTypeId::Varchar.into(), LogicalTypeId::Varchar.into()],
            LogicalTypeHandle::list(&LogicalTypeHandle::struct_type(&[
                ("name", LogicalTypeId::Varchar.into()),
                ("source", LogicalTypeId::Varchar.into()),
                ("source_kind", LogicalTypeId::Varchar.into()),
                ("target", LogicalTypeId::Varchar.into()),
                ("target_kind", LogicalTypeId::Varchar.into()),
                ("probability", LogicalTypeId::Double.into()),
            ])),
        )]
    }

    fn volatile() -> bool {
        true
    }
}

/// The relations as rows, naming each end by the entity's own text and
/// kind.
///
/// The `source` and `target` columns hold the entity's *text*, not its
/// id — the ruled end names, per the one-rule page, over the beta rows'
/// text payload; `source_kind` and `target_kind` hold the kind words.
fn relation_rows(
    found: &thinkthen_contract::Recognized,
) -> Result<Vec<RelationRow>, String> {
    let mut rows = Vec::with_capacity(found.relations.len());
    for relation in &found.relations {
        let end = |id: u64| -> Result<(String, String), String> {
            found
                .entities
                .iter()
                .find(|entity| entity.id == id)
                .map(|entity| (entity.text.clone(), entity.kind.clone()))
                .ok_or_else(|| {
                    failure(EngineError::defect(format!(
                        "the recording's relation {} names entity {id}, and no entity carries it",
                        relation.name
                    )))
                })
        };
        let (source_text, source_kind) = end(relation.source)?;
        let (target_text, target_kind) = end(relation.target)?;
        rows.push((
            relation.name.clone(),
            source_text,
            source_kind,
            target_text,
            target_kind,
            relation.probability,
        ));
    }
    Ok(rows)
}

/// The spec as a `Recognize` ask: `'@file'` reads the question file and
/// takes its `recognize` section, `'{...}'` is the section itself. A file
/// read is checked against every loaded database's own settings first —
/// the same door the scalar functions take, because a scalar cannot name
/// the calling database.
fn build_ask(spec: &str) -> Result<Recognize, String> {
    let only = spec.trim();
    if let Some(path) = only.strip_prefix('@') {
        if let Some(refusal) = crate::connections::file_read_refusal(path) {
            return Err(refusal);
        }
        let text = crate::connections::read_question_file(path, None)?;
        let value: serde_json::Value = serde_json::from_str(&text).map_err(|error| {
            format!("thinkthen usage: the question file {path} is not JSON: {error}")
        })?;
        let section = value.get("recognize").cloned().unwrap_or(value);
        return Recognize::from_json(&section.to_string()).map_err(failure);
    }
    if only.starts_with('{') {
        return Recognize::from_json(only).map_err(failure);
    }
    Err("thinkthen usage: the relations spec is a question file ('@file.json') or its JSON".into())
}

/// Write the per-row relation lists as `LIST(STRUCT(...))`; a NULL row
/// stays NULL and a row with no relations is an empty list.
fn write_lists(output: &mut dyn WritableVector, found: &[Option<Vec<RelationRow>>]) {
    let total: usize = found.iter().flatten().map(Vec::len).sum();
    let mut lists = output.list_vector();
    let capacity = total.max(1);
    let mut child = lists.struct_child(capacity);
    let mut name = child.child(0, capacity);
    let mut source_text = child.child(1, capacity);
    let mut source_kind = child.child(2, capacity);
    let mut target_text = child.child(3, capacity);
    let mut target_kind = child.child(4, capacity);
    let mut probabilities = child.child(5, capacity);
    let mut values: Vec<&RelationRow> = Vec::with_capacity(total);
    let mut entries: Vec<Option<(usize, usize)>> = Vec::with_capacity(found.len());
    for row in found {
        match row {
            Some(row) => {
                let offset = values.len();
                values.extend(row.iter());
                entries.push(Some((offset, row.len())));
            }
            None => entries.push(None),
        }
    }
    for (i, row) in values.iter().enumerate() {
        name.insert(i, row.0.as_str());
        source_text.insert(i, row.1.as_str());
        source_kind.insert(i, row.2.as_str());
        target_text.insert(i, row.3.as_str());
        target_kind.insert(i, row.4.as_str());
    }
    if total > 0 {
        let probabilities_values: Vec<f64> = values.iter().map(|row| row.5).collect();
        unsafe {
            probabilities
                .as_mut_slice_with_len::<f64>(total)
                .copy_from_slice(&probabilities_values);
        }
    }
    lists.set_len(total);
    for (row, entry) in entries.iter().enumerate() {
        match entry {
            Some((offset, length)) => lists.set_entry(row, *offset, *length),
            None => lists.set_null(row),
        }
    }
}
