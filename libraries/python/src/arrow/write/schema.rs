//! Schema nodes and bounded metadata copies for the Arrow output door.

use std::ffi::{CString, c_char};

use super::super::ffi::{ArrowSchema, copied, record};
use super::super::memory::Readable;
use super::super::read::{self, UNREADABLE};

pub(crate) const BAD_METADATA: &str =
    "a column's Arrow metadata names lengths past its readable bytes or past 16 MiB";
const MAX_METADATA: usize = 16 * 1024 * 1024;
/// The deepest schema tree the copy follows, so a cycle is refused.
const MAX_DEPTH: usize = 64;
/// The nullable flag.
const NULLABLE: i64 = 2;

/// One schema node, owned.
#[derive(Debug)]
pub(crate) struct SchemaNode {
    pub(crate) format: CString,
    pub(crate) name: Option<CString>,
    pub(crate) metadata: Option<Vec<u8>>,
    pub(crate) flags: i64,
    pub(crate) children: Vec<SchemaNode>,
    pub(crate) dictionary: Option<Box<SchemaNode>>,
}

fn c_string(text: &str) -> Result<CString, String> {
    CString::new(text).map_err(|_| format!("the name '{text}' holds a NUL byte"))
}

impl SchemaNode {
    pub(super) fn leaf(name: &str, format: &str, children: Vec<Self>) -> Result<Self, String> {
        Ok(Self {
            format: c_string(format)?,
            name: Some(c_string(name)?),
            metadata: None,
            flags: NULLABLE,
            children,
            dictionary: None,
        })
    }

    pub(super) fn frame(children: Vec<Self>, metadata: Option<Vec<u8>>) -> Result<Self, String> {
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
    pub(crate) fn copy(
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
pub(crate) fn metadata(
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
