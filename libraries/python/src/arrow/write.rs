//! What the door hands back: answer columns, a frame with its new columns,
//! and a long table, as Rust trees `ffi` lays out as C structs.
//!
//! The caller's own columns come back aliased, never copied: each keeps the
//! producer's struct, cut to its batch's rows, and a share of the caller's
//! frame. Their schemas are deep-copied, byte for byte with their metadata,
//! so a Polars `Enum` or a field's own keys ride through (R4-15).

use std::ffi::{CString, c_char};
use std::sync::Arc;

use pyo3::prelude::*;
use thinkthen::{Annotated, Answer, QuestionKind};

use super::ffi::{Alias, ArrowSchema, Imported, copied, record};
use super::memory::Readable;
use super::out;
use super::read::{self, Frame, UNREADABLE};

pub(super) const BAD_METADATA: &str =
    "a column's Arrow metadata names lengths past its readable bytes or past 16 MiB";
const MAX_METADATA: usize = 16 * 1024 * 1024;
/// The deepest schema tree the copy follows, so a cycle is refused.
const MAX_DEPTH: usize = 64;
const TOO_LONG: &str = "an answer column's text passes the 2 GiB a text column's offsets can name";
/// The nullable flag.
const NULLABLE: i64 = 2;

/// One schema node, owned.
#[derive(Debug)]
pub(crate) struct SchemaNode {
    pub(super) format: CString,
    pub(super) name: Option<CString>,
    pub(super) metadata: Option<Vec<u8>>,
    pub(super) flags: i64,
    pub(super) children: Vec<SchemaNode>,
    pub(super) dictionary: Option<Box<SchemaNode>>,
}

fn c_string(text: &str) -> Result<CString, String> {
    CString::new(text).map_err(|_| format!("the name '{text}' holds a NUL byte"))
}

impl SchemaNode {
    fn leaf(name: &str, format: &str, children: Vec<Self>) -> Result<Self, String> {
        Ok(Self {
            format: c_string(format)?,
            name: Some(c_string(name)?),
            metadata: None,
            flags: NULLABLE,
            children,
            dictionary: None,
        })
    }

    fn frame(children: Vec<Self>, metadata: Option<Vec<u8>>) -> Result<Self, String> {
        Ok(Self {
            format: c_string("+s")?,
            name: None,
            metadata,
            flags: 0,
            children,
            dictionary: None,
        })
    }

    /// Deep-copy a producer's schema tree: names, formats, metadata,
    /// children, and a dictionary when one is attached.
    pub(super) fn copy(
        memory: &Readable,
        at: *const ArrowSchema,
        depth: usize,
    ) -> Result<Self, String> {
        if depth > MAX_DEPTH {
            return Err("a column's schema nests deeper than 64 levels".to_owned());
        }
        let source = record(memory, at).ok_or(UNREADABLE)?;
        let text = |at| -> Result<Option<CString>, String> {
            Ok(read::c_text(memory, at)?.map(|held| CString::new(held).unwrap_or_default()))
        };
        let count = usize::try_from(source.n_children).unwrap_or(0);
        let children = if count == 0 {
            Vec::new()
        } else {
            let table = copied(
                memory,
                source.children.cast_const().cast(),
                count * size_of::<usize>(),
            )
            .ok_or(UNREADABLE)?;
            let mut children = Vec::with_capacity(count);
            for word in table.chunks_exact(size_of::<usize>()) {
                let mut held = [0_u8; size_of::<usize>()];
                held.copy_from_slice(word);
                let child = std::ptr::with_exposed_provenance(usize::from_le_bytes(held));
                children.push(Self::copy(memory, child, depth + 1)?);
            }
            children
        };
        let dictionary = if source.dictionary.is_null() {
            None
        } else {
            Some(Box::new(Self::copy(memory, source.dictionary, depth + 1)?))
        };
        Ok(Self {
            format: text(source.format)?.ok_or("an Arrow schema carries no format")?,
            name: text(source.name)?,
            metadata: metadata(memory, source.metadata)?,
            flags: source.flags,
            children,
            dictionary,
        })
    }
}

/// Copy one metadata blob: an i32 pair count, then each key and value as an
/// i32 length and its bytes. Each length word is checked readable before it
/// is read, and the blob is capped.
pub(super) fn metadata(
    memory: &Readable,
    at: *const c_char,
) -> Result<Option<Vec<u8>>, &'static str> {
    if at.is_null() {
        return Ok(None);
    }
    let at = at.cast::<u8>();
    let length = |place: usize| -> Result<usize, &'static str> {
        let word = copied(memory, at.wrapping_add(place), 4).ok_or(BAD_METADATA)?;
        let mut four = [0_u8; 4];
        four.copy_from_slice(&word);
        usize::try_from(i32::from_le_bytes(four)).map_err(|_| BAD_METADATA)
    };
    let pairs = length(0)?;
    if pairs > MAX_METADATA / 8 {
        return Err(BAD_METADATA);
    }
    let mut end = 4_usize;
    for _ in 0..pairs * 2 {
        end = end + 4 + length(end)?;
        if end > MAX_METADATA {
            return Err(BAD_METADATA);
        }
    }
    Ok(Some(copied(memory, at, end).ok_or(BAD_METADATA)?))
}

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
/// failed answer never reaches here: a one-question call ends with the
/// engine's error, and `frame::answered` widens a failed question's column.
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
    use super::{BAD_METADATA, SchemaNode, TOO_LONG, metadata, utf8_offset};

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
