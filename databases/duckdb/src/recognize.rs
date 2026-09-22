//! `thinkthen_recognize(body, kinds)`: the recorded names as a list of
//! structs, so `unnest()` makes rows and every name joins like any row.
//!
//! The ruled shape (`sdlc/planning/recognize-design.md`): `(text, kind,
//! start, end, strength)`. `start` and `end` are code points — the
//! recordings' own unit and DuckDB's own string indexing — so
//! `body[start + 1 : end]` slices the name back out (DuckDB slices are
//! one-based, both ends included). The accent-and-emoji case in the
//! conformance file proves it.

use std::collections::HashMap;
use std::error::Error;

use duckdb::core::{DataChunkHandle, Inserter, LogicalTypeHandle, LogicalTypeId};
use duckdb::vscalar::{ScalarFunctionSignature, VScalar};
use duckdb::vtab::arrow::WritableVector;
use thinkthen_contract::{Recognize, Recognized};

use crate::{engine_call, options, read_list_strings, read_strings};

/// `thinkthen_recognize(body, kinds)`: one list of structs per row; a
/// NULL row stays NULL, and a text with no names gives an empty list.
pub(crate) struct RecognizeScalar;

impl VScalar for RecognizeScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> Result<(), Box<dyn Error>> {
        let bodies = read_strings(input, 0);
        let kinds = read_list_strings(input, 1);
        let mut memo: HashMap<(String, String), Recognized> = HashMap::new();
        let mut found: Vec<Option<Recognized>> = Vec::with_capacity(bodies.len());
        for (body, kinds) in bodies.iter().zip(kinds.iter()) {
            let (Some(body), Some(kinds)) = (body.as_deref(), kinds.as_deref()) else {
                found.push(None);
                continue;
            };
            let key = (kinds.join("\u{1f}"), body.to_owned());
            if let Some(answer) = memo.get(&key) {
                found.push(Some(answer.clone()));
                continue;
            }
            let ask = Recognize::new().kinds(kinds.to_vec());
            let answer = engine_call(|engine| engine.recognize_opts(&ask, body, options()))?;
            memo.insert(key, answer.clone());
            found.push(Some(answer));
        }
        write_lists(output, &found);
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![
                LogicalTypeId::Varchar.into(),
                LogicalTypeHandle::list(&LogicalTypeId::Varchar.into()),
            ],
            LogicalTypeHandle::list(&LogicalTypeHandle::struct_type(&[
                ("text", LogicalTypeId::Varchar.into()),
                ("kind", LogicalTypeId::Varchar.into()),
                ("start", LogicalTypeId::Bigint.into()),
                ("end", LogicalTypeId::Bigint.into()),
                ("strength", LogicalTypeId::Double.into()),
            ])),
        )]
    }

    fn volatile() -> bool {
        true
    }
}

/// Write the per-row answers as `LIST(STRUCT(...))`; a NULL row stays
/// NULL and a row with no names is an empty list.
fn write_lists(output: &mut dyn WritableVector, found: &[Option<Recognized>]) {
    let total: usize = found
        .iter()
        .flatten()
        .map(|answer| answer.entities.len())
        .sum();
    let mut lists = output.list_vector();
    let capacity = total.max(1);
    let mut child = lists.struct_child(capacity);
    let mut text = child.child(0, capacity);
    let mut kind = child.child(1, capacity);
    let mut starts = child.child(2, capacity);
    let mut ends = child.child(3, capacity);
    let mut strengths = child.child(4, capacity);
    let mut texts: Vec<&str> = Vec::with_capacity(total);
    let mut kinds: Vec<&str> = Vec::with_capacity(total);
    let mut start_values: Vec<i64> = Vec::with_capacity(total);
    let mut end_values: Vec<i64> = Vec::with_capacity(total);
    let mut strength_values: Vec<f64> = Vec::with_capacity(total);
    let mut entries: Vec<Option<(usize, usize)>> = Vec::with_capacity(found.len());
    for answer in found {
        match answer {
            Some(answer) => {
                let offset = texts.len();
                for entity in &answer.entities {
                    texts.push(entity.text.as_str());
                    kinds.push(entity.kind.as_str());
                    start_values.push(entity.start as i64);
                    end_values.push(entity.end as i64);
                    strength_values.push(entity.strength);
                }
                entries.push(Some((offset, answer.entities.len())));
            }
            None => entries.push(None),
        }
    }
    for (i, value) in texts.iter().enumerate() {
        text.insert(i, *value);
    }
    for (i, value) in kinds.iter().enumerate() {
        kind.insert(i, *value);
    }
    if total > 0 {
        unsafe {
            starts
                .as_mut_slice_with_len::<i64>(total)
                .copy_from_slice(&start_values);
            ends.as_mut_slice_with_len::<i64>(total)
                .copy_from_slice(&end_values);
            strengths
                .as_mut_slice_with_len::<f64>(total)
                .copy_from_slice(&strength_values);
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
