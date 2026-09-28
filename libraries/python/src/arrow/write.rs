//! What the door hands back: answer columns, a frame with its new columns,
//! and a long table, as Rust trees `ffi` lays out as C structs.
//!
//! The caller's own columns come back aliased, never copied: each keeps the
//! producer's struct, cut to its batch's rows, and a share of the caller's
//! frame. Their schemas are deep-copied, byte for byte with their metadata,
//! so a Polars `Enum` or a field's own keys ride through (R4-15).

use std::sync::Arc;

use pyo3::prelude::*;
use thinkthen::{Annotated, Answer, QuestionKind};

use super::ffi::{Alias, Imported};
use super::memory::Readable;
use super::out;
use super::read::Frame;

mod schema;
#[cfg(test)]
pub(crate) use schema::BAD_METADATA;
pub(crate) use schema::{SchemaNode, metadata};

const TOO_LONG: &str = "an answer column's text passes the 2 GiB a text column's offsets can name";

/// One array node: this binding's own buffers, or one of the caller's.
#[derive(Debug)]
pub(crate) enum ArrayNode {
    Owned(Owned),
    Alias(Alias),
}

/// An array whose buffers this binding made.
#[derive(Debug)]
pub(crate) struct Owned {
    pub(super) length: i64,
    pub(super) null_count: i64,
    pub(super) buffers: Vec<Option<Vec<u8>>>,
    pub(super) children: Vec<ArrayNode>,
}

/// One answer column's values.
#[derive(Debug)]
pub(crate) enum Cells {
    Bools(Vec<Option<bool>>),
    Numbers(Vec<Option<f64>>),
    Counts(Vec<i64>),
    Texts(Vec<Option<String>>),
    Lists(Vec<Option<Vec<String>>>),
    Failures {
        names: Vec<String>,
        rows: Vec<Vec<Option<FailureMarker>>>,
    },
}

#[derive(Clone, Debug)]
pub(crate) struct FailureMarker {
    pub(crate) kind: String,
    pub(crate) cause: String,
}

fn count(value: usize) -> i64 {
    i64::try_from(value).unwrap_or(i64::MAX)
}

/// A validity bitmap, or none when every cell is present.
fn validity(present: &[bool]) -> (Option<Vec<u8>>, i64) {
    let nulls = present.iter().filter(|one| !**one).count();
    (
        (nulls > 0).then(|| bitmap(present.iter().copied())),
        count(nulls),
    )
}

fn bitmap(bits: impl ExactSizeIterator<Item = bool>) -> Vec<u8> {
    let mut packed = vec![0_u8; bits.len().div_ceil(8)];
    for (place, bit) in bits.enumerate() {
        if let Some(byte) = packed.get_mut(place / 8).filter(|_| bit) {
            *byte |= 1 << (place % 8);
        }
    }
    packed
}

fn struct_node(present: &[bool], children: Vec<ArrayNode>) -> ArrayNode {
    let (validity, null_count) = validity(present);
    ArrayNode::Owned(Owned {
        length: count(present.len()),
        null_count,
        buffers: vec![validity],
        children,
    })
}

fn failure_field(
    values: &[Vec<Option<FailureMarker>>],
    place: usize,
    outer: &mut [bool],
) -> Result<ArrayNode, &'static str> {
    let mut present = Vec::with_capacity(values.len());
    let mut kind = Vec::with_capacity(values.len());
    let mut cause = Vec::with_capacity(values.len());
    for (outer_cell, fields) in outer.iter_mut().zip(values) {
        let marker = fields.get(place).and_then(Option::as_ref);
        present.push(marker.is_some());
        *outer_cell |= marker.is_some();
        kind.push(marker.map(|one| one.kind.clone()));
        cause.push(marker.map(|one| one.cause.clone()));
    }
    let marker = struct_node(
        &present,
        vec![
            Cells::Texts(kind).array(0, values.len())?,
            Cells::Texts(cause).array(0, values.len())?,
        ],
    );
    Ok(struct_node(&present, vec![marker]))
}

/// A `u` array's offsets and values.
type Buffers = (Vec<u8>, Vec<u8>);

/// The offsets and values of a `u` array. Its offsets are i32, so text past
/// 2 GiB is refused (R4-15).
fn utf8<'t>(texts: impl Iterator<Item = &'t str>) -> Result<Buffers, &'static str> {
    let mut offsets = 0_i32.to_le_bytes().to_vec();
    let mut values = Vec::new();
    for text in texts {
        values.extend_from_slice(text.as_bytes());
        offsets.extend_from_slice(&utf8_offset(values.len())?.to_le_bytes());
    }
    Ok((offsets, values))
}

fn utf8_offset(end: usize) -> Result<i32, &'static str> {
    i32::try_from(end).map_err(|_| TOO_LONG)
}

impl Cells {
    fn len(&self) -> usize {
        match self {
            Self::Bools(values) => values.len(),
            Self::Numbers(values) => values.len(),
            Self::Counts(values) => values.len(),
            Self::Texts(values) => values.len(),
            Self::Lists(values) => values.len(),
            Self::Failures { rows, .. } => rows.len(),
        }
    }

    fn schema(&self, name: &str) -> Result<SchemaNode, String> {
        match self {
            Self::Bools(_) => SchemaNode::leaf(name, "b", Vec::new()),
            Self::Numbers(_) => SchemaNode::leaf(name, "g", Vec::new()),
            Self::Counts(_) => SchemaNode::leaf(name, "l", Vec::new()),
            Self::Texts(_) => SchemaNode::leaf(name, "u", Vec::new()),
            Self::Lists(_) => {
                SchemaNode::leaf(name, "+l", vec![SchemaNode::leaf("item", "u", Vec::new())?])
            }
            Self::Failures { names, .. } => SchemaNode::leaf(
                name,
                "+s",
                names
                    .iter()
                    .map(|question| {
                        SchemaNode::leaf(
                            question,
                            "+s",
                            vec![SchemaNode::leaf(
                                "failed",
                                "+s",
                                vec![
                                    SchemaNode::leaf("kind", "u", Vec::new())?,
                                    SchemaNode::leaf("cause", "u", Vec::new())?,
                                ],
                            )?],
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        }
    }

    /// The array of rows `from .. from + rows`.
    fn array(&self, from: usize, rows: usize) -> Result<ArrayNode, &'static str> {
        let cut = |length: usize| from.min(length)..(from + rows).min(length);
        let owned = |present: &[bool], mut data: Vec<Option<Vec<u8>>>, children| {
            let (validity, null_count) = validity(present);
            data.insert(0, validity);
            ArrayNode::Owned(Owned {
                length: count(present.len()),
                null_count,
                buffers: data,
                children,
            })
        };
        Ok(match self {
            Self::Bools(values) => {
                let values = values.get(cut(values.len())).unwrap_or_default();
                let present: Vec<bool> = values.iter().map(Option::is_some).collect();
                let bits = bitmap(values.iter().map(|one| one.unwrap_or(false)));
                owned(&present, vec![Some(bits)], Vec::new())
            }
            Self::Numbers(values) => {
                let values = values.get(cut(values.len())).unwrap_or_default();
                let present: Vec<bool> = values.iter().map(Option::is_some).collect();
                let data = values
                    .iter()
                    .flat_map(|one| one.unwrap_or(0.0).to_le_bytes())
                    .collect();
                owned(&present, vec![Some(data)], Vec::new())
            }
            Self::Counts(values) => {
                let values = values.get(cut(values.len())).unwrap_or_default();
                let data = values.iter().flat_map(|one| one.to_le_bytes()).collect();
                owned(&vec![true; values.len()], vec![Some(data)], Vec::new())
            }
            Self::Texts(values) => {
                let values = values.get(cut(values.len())).unwrap_or_default();
                let present: Vec<bool> = values.iter().map(Option::is_some).collect();
                let (offsets, data) =
                    utf8(values.iter().map(|one| one.as_deref().unwrap_or_default()))?;
                owned(&present, vec![Some(offsets), Some(data)], Vec::new())
            }
            Self::Lists(values) => {
                let values = values.get(cut(values.len())).unwrap_or_default();
                let present: Vec<bool> = values.iter().map(Option::is_some).collect();
                let items: Vec<&str> = values
                    .iter()
                    .flatten()
                    .flatten()
                    .map(String::as_str)
                    .collect();
                let mut ends = vec![0_i32];
                let mut seen = 0;
                for one in values {
                    seen += one.as_ref().map_or(0, Vec::len);
                    ends.push(utf8_offset(seen)?);
                }
                let (offsets, data) = utf8(items.iter().copied())?;
                let item = owned(
                    &vec![true; items.len()],
                    vec![Some(offsets), Some(data)],
                    Vec::new(),
                );
                let ends = ends.iter().flat_map(|one| one.to_le_bytes()).collect();
                owned(&present, vec![Some(ends)], vec![item])
            }
            Self::Failures {
                names,
                rows: values,
            } => {
                let values = values.get(cut(values.len())).unwrap_or_default();
                let mut outer = vec![false; values.len()];
                let children = (0..names.len())
                    .map(|place| failure_field(values, place, &mut outer))
                    .collect::<Result<Vec<_>, _>>()?;
                struct_node(&outer, children)
            }
        })
    }
}

const fn answer(value: Answer) -> Option<bool> {
    match value {
        Answer::Yes => Some(true),
        Answer::No => Some(false),
        Answer::Unsure => None,
    }
}

/// A `decide` column's answers: `None` is "not sure".
pub(crate) fn decided(values: &[Answer]) -> Cells {
    Cells::Bools(values.iter().map(|one| answer(*one)).collect())
}

/// One question's answers across the records, in the kind's own column. A
/// failed frame answer becomes null here; the companion column carries its
/// marker. A one-question Series call still ends with the engine's error.
pub(crate) fn annotated(kind: QuestionKind, values: &[&Annotated]) -> Cells {
    match kind {
        QuestionKind::Decide => Cells::Bools(
            values
                .iter()
                .map(|one| match one {
                    Annotated::Decision(held) => answer(*held),
                    _ => None,
                })
                .collect(),
        ),
        QuestionKind::Score => Cells::Numbers(
            values
                .iter()
                .map(|one| match one {
                    Annotated::Score(position) => Some(*position),
                    _ => None,
                })
                .collect(),
        ),
        QuestionKind::Tag => Cells::Lists(
            values
                .iter()
                .map(|one| match one {
                    Annotated::Tags(labels) => Some(labels.clone()),
                    _ => None,
                })
                .collect(),
        ),
        _ => Cells::Texts(
            values
                .iter()
                .map(|one| match one {
                    Annotated::Choice(pick) => pick.clone(),
                    _ => None,
                })
                .collect(),
        ),
    }
}

/// One answer column, handed to the host through `__arrow_c_array__`.
pub(crate) fn column(name: &str, cells: &Cells) -> Result<Output, String> {
    Ok(Output::Column(
        cells.schema(name)?,
        cells.array(0, cells.len())?,
    ))
}

/// The caller's column schemas and the frame's metadata.
pub(crate) type Kept = (Vec<SchemaNode>, Option<Vec<u8>>);

/// The caller's column schemas and frame metadata, copied before any send,
/// so a schema the copy refuses or a question named as a column sends
/// nothing (ticket 0136). Question names are `[a-z0-9_]`, so `'{name}'` reads
/// as Python's `repr`, as in the pandas sentence.
pub(crate) fn kept<'a>(
    hold: &Imported,
    memory: &Readable,
    read: &Frame<'_>,
    mut names: impl Iterator<Item = &'a str>,
) -> Result<Kept, String> {
    let fields: Vec<SchemaNode> = read
        .schemas
        .iter()
        .map(|at| SchemaNode::copy(memory, *at, 0))
        .collect::<Result<_, _>>()?;
    let held = |name: &str| {
        (fields.iter()).any(|one| {
            one.name
                .as_ref()
                .is_some_and(|at| at.as_bytes() == name.as_bytes())
        })
    };
    if let Some(name) = names.find(|name| held(name)) {
        return Err(format!(
            "the frame already has a column named '{name}'; rename it first"
        ));
    }
    Ok((fields, metadata(memory, hold.schema.metadata)?))
}

/// The caller's frame with one new column per question: the caller's
/// columns aliased batch by batch, and the new columns cut to match.
pub(crate) fn frame(
    hold: &Arc<Imported>,
    (mut fields, metadata): Kept,
    read: &Frame<'_>,
    columns: &[(String, Cells)],
) -> Result<Output, String> {
    for (name, cells) in columns {
        fields.push(cells.schema(name)?);
    }
    let schema = SchemaNode::frame(fields, metadata)?;
    let mut batches = Vec::with_capacity(read.batches.len());
    let mut base = 0;
    for batch in &read.batches {
        let rows = usize::try_from(batch.length).unwrap_or(0);
        let mut children: Vec<ArrayNode> = batch
            .columns
            .iter()
            .map(|one| {
                ArrayNode::Alias(Alias::new(
                    *one,
                    batch.offset + one.offset,
                    batch.length,
                    hold,
                ))
            })
            .collect();
        for (_, cells) in columns {
            children.push(cells.array(base, rows)?);
        }
        batches.push(root(batch.length, children));
        base += rows;
    }
    Ok(Output::Frame(schema, batches))
}

fn root(length: i64, children: Vec<ArrayNode>) -> ArrayNode {
    ArrayNode::Owned(Owned {
        length,
        null_count: 0,
        buffers: vec![None],
        children,
    })
}

/// A long table of this binding's own columns, in one batch.
pub(crate) fn table(columns: &[(&str, Cells)]) -> Result<Output, String> {
    let rows = columns.first().map_or(0, |(_, cells)| cells.len());
    let mut fields = Vec::with_capacity(columns.len());
    let mut children = Vec::with_capacity(columns.len());
    for (name, cells) in columns {
        fields.push(cells.schema(name)?);
        children.push(cells.array(0, rows)?);
    }
    Ok(Output::Frame(
        SchemaNode::frame(fields, None)?,
        vec![root(count(rows), children)],
    ))
}

/// What the worker hands back for the host to rebuild.
#[derive(Debug)]
pub(crate) enum Output {
    Column(SchemaNode, ArrayNode),
    Frame(SchemaNode, Vec<ArrayNode>),
}

/// An answer column or a frame. `pl.Series(value)` or `pl.DataFrame(value)`
/// takes it once, through the Arrow PyCapsule interface.
#[pyclass(unsendable, name = "_Arrow", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct Arrow(Option<Output>);

impl Arrow {
    pub(crate) const fn new(output: Output) -> Self {
        Self(Some(output))
    }
}

const TAKEN: &str = "this Arrow result was already taken";

#[pymethods]
impl Arrow {
    #[pyo3(signature = (requested_schema = None))]
    fn __arrow_c_array__(
        &mut self,
        py: Python<'_>,
        requested_schema: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
        let _ = requested_schema;
        match self.0.take() {
            Some(Output::Column(schema, array)) => out::export_array(py, &schema, array),
            other => {
                self.0 = other;
                Err(crate::usage(py, TAKEN))
            }
        }
    }

    #[pyo3(signature = (requested_schema = None))]
    fn __arrow_c_stream__(
        &mut self,
        py: Python<'_>,
        requested_schema: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Py<PyAny>> {
        let _ = requested_schema;
        match self.0.take() {
            Some(Output::Frame(schema, batches)) => out::export_stream(py, schema, batches),
            Some(Output::Column(schema, array)) => out::export_stream(py, schema, vec![array]),
            None => Err(crate::usage(py, TAKEN)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::ffi::{ArrowSchema, EMPTY_SCHEMA};
    use super::super::memory::Readable;
    use super::schema::{BAD_METADATA, metadata};
    use super::{SchemaNode, TOO_LONG, utf8_offset};

    fn words(parts: &[i32]) -> Vec<u8> {
        parts.iter().flat_map(|one| one.to_le_bytes()).collect()
    }

    /// R4-15 (c): the answer offsets were `len as i32`, which wraps past 2 GiB.
    #[test]
    fn an_answer_column_past_i32_offsets_is_refused() {
        assert_eq!(utf8_offset(i32::MAX as usize), Ok(i32::MAX));
        assert_eq!(utf8_offset(i32::MAX as usize + 1), Err(TOO_LONG));
    }

    /// R4-15 (b): metadata is copied byte for byte, and a negative length is
    /// refused.
    #[test]
    fn schema_metadata_is_copied_byte_for_byte() {
        let memory = Readable::snapshot().expect("the memory map reads");
        let blob = [
            words(&[1, 4]),
            b"unit".to_vec(),
            words(&[2]),
            b"cm".to_vec(),
        ]
        .concat();
        assert_eq!(
            metadata(&memory, blob.as_ptr().cast()),
            Ok(Some(blob.clone()))
        );
        assert_eq!(
            metadata(&memory, words(&[-1]).as_ptr().cast()),
            Err(BAD_METADATA)
        );
        let huge = words(&[1, 0x7fff_fff0]);
        assert_eq!(metadata(&memory, huge.as_ptr().cast()), Err(BAD_METADATA));
        assert_eq!(
            metadata(&memory, words(&[i32::MAX]).as_ptr().cast()),
            Err(BAD_METADATA)
        );
    }

    /// A schema whose child names itself is refused, not followed until the
    /// stack runs out.
    #[test]
    fn a_schema_cycle_is_refused() {
        let memory = Readable::snapshot().expect("the memory map reads");
        let mut node = ArrowSchema {
            format: c"+s".as_ptr(),
            n_children: 1,
            ..EMPTY_SCHEMA
        };
        let mut table = [&raw mut node];
        node.children = table.as_mut_ptr();
        assert_eq!(
            SchemaNode::copy(&memory, &raw const node, 0).map(|_| ()),
            Err("a column's schema nests deeper than 64 levels".to_owned())
        );
    }
}
