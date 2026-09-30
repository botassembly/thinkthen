//! Reading a producer's text column in place, every extent checked first.
//!
//! The interface carries no byte length beside a buffer pointer. It declares
//! extents instead. The array's `offset + length` declares the views and
//! offsets buffers. A string view's trailing sizes buffer declares each data
//! buffer. A Utf8 array's offset at `offset + length` declares its data
//! buffer. Every read sits inside one of those declarations, checked before
//! the read, and every byte read is then checked readable in the call's one
//! snapshot (`Readable`). An extent that runs into another readable
//! allocation cannot be told from a correct one by any reader. That lie
//! stays the producer's (`NOTES.md`, the waivers).

use std::ffi::c_char;
use std::ptr;

use super::ffi::{ArrowArray, ArrowSchema, Imported, bytes, copied, record};
use super::memory::Readable;

/// The most bytes a producer's format, name, or error string may hold.
pub(super) const MAX_TEXT: usize = 64 * 1024;
pub(super) const BAD_TEXT: &str =
    "an Arrow format, name, or error string has no end within 64 KiB of readable memory";
pub(crate) const UNREADABLE: &str = "the column's buffers declare bytes this process cannot read";
/// The longest single row the door borrows: a per-row ceiling, not a bound.
const MAX_ONE_STRING: usize = 4 * 1024 * 1024;
/// The most rows one column may claim, counting its offset.
const MAX_ROWS: usize = u32::MAX as usize;
/// The most bytes one column may declare a data buffer holds. It bounds a
/// residual read past a short allocation to a gibibyte.
const MAX_DATA: usize = 1024 * 1024 * 1024;
/// The most buffers a text array's table may hold.
const MAX_BUFFERS: usize = 64;
const WORD: usize = size_of::<usize>();

/// The three whole-text layouts the door reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Text {
    /// `u`: 32-bit offsets over one UTF-8 buffer.
    Utf8,
    /// `U`: 64-bit offsets over one UTF-8 buffer.
    LargeUtf8,
    /// `vu`: 16-byte views, and data buffers for strings past 12 bytes.
    View,
}

/// A producer's C string without its NUL, read only inside readable memory,
/// or `None` for a null pointer.
pub(super) fn c_text(
    memory: &Readable,
    at: *const c_char,
) -> Result<Option<Vec<u8>>, &'static str> {
    if at.is_null() {
        return Ok(None);
    }
    let reach = memory.reach(at.addr(), MAX_TEXT + 1);
    let mut held = copied(memory, at.cast(), reach).ok_or(BAD_TEXT)?;
    let end = held.iter().position(|byte| *byte == 0).ok_or(BAD_TEXT)?;
    held.truncate(end);
    Ok(Some(held))
}

/// Check a schema is a whole text column, and say why not.
pub(super) fn text_layout(memory: &Readable, schema: &ArrowSchema) -> Result<Text, String> {
    if !schema.dictionary.is_null() {
        return Err("a dictionary-encoded column is not read; cast it to text first".to_owned());
    }
    let format = c_text(memory, schema.format)?.ok_or("an Arrow schema carries no format")?;
    if schema.n_children != 0 && format != b"+s" {
        return Err("a nested column is not a column of text".to_owned());
    }
    match format.as_slice() {
        b"u" => Ok(Text::Utf8),
        b"U" => Ok(Text::LargeUtf8),
        b"vu" => Ok(Text::View),
        b"+s" => {
            Err("a data frame is not a column; pass df[\"name\"], or annotate with on=".to_owned())
        }
        _ => Err(format!(
            "the column's Arrow format is '{}', not text",
            String::from_utf8_lossy(&format)
        )),
    }
}

/// `count` little-endian words of `width` bytes, as addresses or numbers.
fn words(held: &[u8], width: usize) -> impl Iterator<Item = u64> + '_ {
    held.chunks_exact(width).map(move |word| {
        let mut full = [0_u8; 8];
        if let Some(low) = full.get_mut(..width) {
            low.copy_from_slice(word);
        }
        u64::from_le_bytes(full)
    })
}

/// An address read from the producer's memory.
fn address(value: u64) -> *const u8 {
    ptr::with_exposed_provenance(usize::try_from(value).unwrap_or(0))
}

/// The array's buffer pointers, the table checked readable first.
fn buffers(memory: &Readable, array: &ArrowArray) -> Result<Vec<*const u8>, &'static str> {
    let count = usize::try_from(array.n_buffers)
        .ok()
        .filter(|count| *count <= MAX_BUFFERS)
        .ok_or("the column's buffer table names more buffers than a text column carries")?;
    if array.buffers.is_null() {
        return Err("the column carries no buffer table");
    }
    let table = copied(memory, array.buffers.cast(), count * WORD).ok_or(UNREADABLE)?;
    Ok(words(&table, WORD).map(address).collect())
}

/// Borrow one array's rows `skip .. skip + count` as text.
pub(super) fn borrow<'a, O: ?Sized>(
    owner: &'a O,
    memory: &Readable,
    array: &ArrowArray,
    text: Text,
    skip: usize,
    count: usize,
) -> Result<Vec<Option<&'a str>>, String> {
    let (Ok(length), Ok(offset)) = (usize::try_from(array.length), usize::try_from(array.offset))
    else {
        return Err(
            "the column's length or offset is negative; a malformed array is refused, not read"
                .to_owned(),
        );
    };
    let table = buffers(memory, array)?;
    let carried = offset.saturating_add(length);
    if carried > MAX_ROWS {
        return Err("the column claims more rows than any text column carries".to_owned());
    }
    if skip.saturating_add(count) > carried {
        return Err("the frame's struct root asks for rows its column does not carry".to_owned());
    }
    let present = presence(memory, array, &table, skip, count)?;
    if text == Text::View && !present.iter().any(|one| *one) {
        return Ok(vec![None; count]);
    }
    let rows = match text {
        Text::View => view_rows(owner, memory, &table, skip, count, &present)?,
        Text::Utf8 | Text::LargeUtf8 => offset_rows(
            owner,
            memory,
            &table,
            text,
            (skip..skip + count, &present),
            carried,
        )?,
    };
    rows.into_iter()
        .map(|row| {
            row.map(|bytes| {
                std::str::from_utf8(bytes)
                    .map_err(|_| "the column's buffer is not valid UTF-8".to_owned())
            })
            .transpose()
        })
        .collect()
}

/// One bit per selected row. The bitmap is read inside the same owned memory
/// snapshot as the text buffers, including a sliced array's nonzero offset.
fn presence(
    memory: &Readable,
    array: &ArrowArray,
    table: &[*const u8],
    skip: usize,
    count: usize,
) -> Result<Vec<bool>, &'static str> {
    if array.null_count < -1 || array.null_count > array.length {
        return Err("the column carries an invalid null count");
    }
    if array.null_count == 0 {
        return Ok(vec![true; count]);
    }
    let bitmap = table
        .first()
        .copied()
        .ok_or("the column lacks a validity buffer")?;
    if bitmap.is_null() {
        return if array.null_count == -1 {
            Ok(vec![true; count])
        } else {
            Err("the column lacks a validity buffer")
        };
    }
    let first = skip / 8;
    let last = skip.saturating_add(count).div_ceil(8);
    let bits = copied(memory, bitmap.wrapping_add(first), last - first).ok_or(UNREADABLE)?;
    Ok((skip..skip + count)
        .map(|at| {
            bits.get(at / 8 - first)
                .is_some_and(|byte| byte & (1 << (at % 8)) != 0)
        })
        .collect())
}

fn row_length(length: usize) -> Result<usize, &'static str> {
    if length > MAX_ONE_STRING {
        return Err("a row names a size no text column carries");
    }
    Ok(length)
}

/// One string view: its size, and its inline bytes or its buffer and offset.
struct View {
    size: usize,
    index: usize,
    offset: usize,
}

fn view(one: &[u8]) -> View {
    let word = |from: usize| {
        let mut four = [0_u8; 4];
        if let Some(held) = one.get(from..from + 4) {
            four.copy_from_slice(held);
        }
        u32::from_le_bytes(four) as usize
    };
    View {
        size: word(0),
        index: word(8),
        offset: word(12),
    }
}

/// Each string-view row's bytes. The table is `[validity, views, data 0 ..
/// data n-1, sizes]`, so the data count is the table minus three, and no view
/// can name the sizes buffer.
fn view_rows<'a, O: ?Sized>(
    owner: &'a O,
    memory: &Readable,
    table: &[*const u8],
    skip: usize,
    count: usize,
    present: &[bool],
) -> Result<Vec<Option<&'a [u8]>>, &'static str> {
    const LACKS: &str = "the string-view array lacks its views or its buffer-sizes buffer";
    let data = table.len().checked_sub(3).ok_or(LACKS)?;
    let (Some(&views), Some(&sizes)) = (table.get(1), table.get(data + 2)) else {
        return Err(LACKS);
    };
    if views.is_null() || (data > 0 && sizes.is_null()) {
        return Err(LACKS);
    }
    let views =
        bytes(owner, memory, views.wrapping_add(skip * 16), count * 16).ok_or(UNREADABLE)?;
    let sizes: Vec<u64> =
        words(bytes(owner, memory, sizes, data * 8).ok_or(UNREADABLE)?, 8).collect();
    // The lowest and highest byte the rows read in each data buffer.
    let mut reach: Vec<Option<(usize, usize)>> = vec![None; data];
    for (one, valid) in views.chunks_exact(16).map(view).zip(present) {
        if !valid {
            continue;
        }
        row_length(one.size)?;
        if one.size <= 12 {
            continue;
        }
        let (Some(declared), Some(seen)) = (sizes.get(one.index), reach.get_mut(one.index)) else {
            return Err("a string view points past its data buffers");
        };
        let declared = usize::try_from(i64::try_from(*declared).unwrap_or(-1))
            .ok()
            .filter(|declared| *declared <= MAX_DATA)
            .ok_or("a string view's data buffer declares more bytes than a text column carries")?;
        let end = one.offset + one.size;
        if end > declared || table.get(2 + one.index).is_none_or(|base| base.is_null()) {
            return Err("a string view reaches past the length its data buffer declares");
        }
        *seen = Some(seen.map_or((one.offset, end), |(low, high)| {
            (low.min(one.offset), high.max(end))
        }));
    }
    // Only the bytes the selected rows read are checked, so a short slice of a
    // large buffer costs its own rows.
    let mut held: Vec<(usize, &'a [u8])> = Vec::with_capacity(data);
    for (index, seen) in reach.iter().enumerate() {
        let (low, high) = seen.unwrap_or((0, 0));
        let base = table.get(2 + index).copied().unwrap_or(ptr::null());
        held.push((
            low,
            bytes(owner, memory, base.wrapping_add(low), high - low).ok_or(UNREADABLE)?,
        ));
    }
    let mut rows = Vec::with_capacity(count);
    for (one, valid) in views.chunks_exact(16).zip(present) {
        if !valid {
            rows.push(None);
            continue;
        }
        let at = view(one);
        let row = if at.size <= 12 {
            // The view borrows from `views`, which lives as long as `owner`.
            let whole: &'a [u8] = one;
            whole.get(4..4 + at.size)
        } else {
            held.get(at.index)
                .and_then(|(low, data)| data.get(at.offset - low..at.offset - low + at.size))
        };
        rows.push(Some(row.ok_or(UNREADABLE)?));
    }
    Ok(rows)
}

/// Each Utf8 or LargeUtf8 row's bytes. The offsets are checked rising and
/// nonnegative from the first row read to the last offset, and the last one
/// bounded, before any data byte is touched.
fn offset_rows<'a, O: ?Sized>(
    owner: &'a O,
    memory: &Readable,
    table: &[*const u8],
    text: Text,
    selection: (std::ops::Range<usize>, &[bool]),
    carried: usize,
) -> Result<Vec<Option<&'a [u8]>>, &'static str> {
    let (rows, present) = selection;
    let (skip, count) = (rows.start, rows.len());
    let (Some(&offsets), Some(&values)) = (table.get(1), table.get(2)) else {
        return Err("the string array lacks its offsets or its values buffer");
    };
    if offsets.is_null() || (values.is_null() && present.iter().any(|one| *one)) {
        return Err("the string array lacks its offsets or its values buffer");
    }
    let width = if text == Text::LargeUtf8 { 8 } else { 4 };
    let held = bytes(
        owner,
        memory,
        offsets.wrapping_add(skip * width),
        (carried - skip + 1) * width,
    )
    .ok_or(UNREADABLE)?;
    let signed = |word: u64| {
        if width == 4 {
            i64::from(word as u32 as i32)
        } else {
            word as i64
        }
    };
    let ends: Vec<i64> = words(held, width).map(signed).collect();
    // Utf8 offsets describe every slot, including nulls whose payload is
    // undefined. Validate their structure before borrowing present bytes.
    let mut start = 0;
    for &end in &ends {
        if end < start {
            return Err("the column's offsets are reversed or negative");
        }
        if end > MAX_DATA as i64 {
            return Err("the column's offsets name more bytes than a text column carries");
        }
        start = end;
    }
    if !present.iter().any(|one| *one) {
        return Ok(vec![None; count]);
    }
    if present.iter().any(|one| !one) {
        return ends
            .windows(2)
            .zip(present)
            .map(|(pair, valid)| {
                if !valid {
                    return Ok(None);
                }
                let (low, high) = (
                    pair.first().copied().unwrap_or(0),
                    pair.get(1).copied().unwrap_or(0),
                );
                let (low, high) = (low as usize, high as usize);
                row_length(high - low)?;
                bytes(owner, memory, values.wrapping_add(low), high - low)
                    .map(Some)
                    .ok_or(UNREADABLE)
            })
            .collect();
    }
    let first = usize::try_from(ends.first().copied().unwrap_or(0)).unwrap_or(0);
    let last = usize::try_from(ends.get(count).copied().unwrap_or(0)).unwrap_or(0);
    let mut spans = Vec::with_capacity(count);
    for pair in ends.windows(2).take(count) {
        let (low, high) = (
            pair.first().copied().unwrap_or(0),
            pair.get(1).copied().unwrap_or(0),
        );
        let low = usize::try_from(low).unwrap_or(0);
        spans.push((
            low - first,
            row_length(usize::try_from(high).unwrap_or(0) - low)?,
        ));
    }
    // Only the selected rows' bytes are checked, from their first offset to
    // their last.
    let data = bytes(owner, memory, values.wrapping_add(first), last - first).ok_or(UNREADABLE)?;
    spans
        .into_iter()
        .map(|(from, length)| data.get(from..from + length).map(Some).ok_or(UNREADABLE))
        .collect()
}

/// Every row of a column, batch by batch.
pub(crate) fn series<'a>(
    column: &'a Imported,
    memory: &Readable,
) -> Result<Vec<Option<&'a str>>, String> {
    let text = text_layout(memory, &column.schema)?;
    let mut texts = Vec::new();
    for batch in &column.batches {
        let skip = usize::try_from(batch.offset).unwrap_or(0);
        let count = usize::try_from(batch.length).unwrap_or(0);
        texts.extend(borrow(column, memory, batch, text, skip, count)?);
    }
    Ok(texts)
}

/// One batch of the caller's frame: its root's rows, and a copy of each
/// column's struct for the output to alias.
#[derive(Debug)]
pub(crate) struct Batch {
    pub(super) offset: i64,
    pub(super) length: i64,
    pub(super) columns: Vec<ArrowArray>,
}

/// A frame read for `annotate` or `recognize` with `on=`.
#[derive(Debug)]
pub(crate) struct Frame<'a> {
    pub(crate) texts: Vec<Option<&'a str>>,
    pub(super) schemas: Vec<*const ArrowSchema>,
    pub(super) batches: Vec<Batch>,
}

/// A pointer table of `count` slots, checked readable, each slot nonnull and
/// naming a readable struct.
fn children<T: super::ffi::Plain>(
    memory: &Readable,
    at: *mut *mut T,
    count: usize,
) -> Result<Vec<(*const T, T)>, &'static str> {
    if at.is_null() {
        return Err("the frame names columns but carries no column arrays");
    }
    let table = copied(memory, at.cast_const().cast(), count * WORD).ok_or(UNREADABLE)?;
    words(&table, WORD)
        .map(|word| {
            let child = address(word).cast::<T>();
            record(memory, child)
                .map(|held| (child, held))
                .ok_or(UNREADABLE)
        })
        .collect()
}

/// Read the whole frame, borrowing the `on` column's rows.
pub(crate) fn frame<'a>(
    held: &'a Imported,
    on: &str,
    memory: &Readable,
) -> Result<Frame<'a>, String> {
    let schema = &held.schema;
    let format = c_text(memory, schema.format)?.ok_or("an Arrow schema carries no format")?;
    if format != b"+s" {
        return Err("the stream is not a data frame".to_owned());
    }
    let count = usize::try_from(schema.n_children).unwrap_or(0);
    if count == 0 {
        return Err("the frame has no columns".to_owned());
    }
    let fields = children(memory, schema.children, count)?;
    let mut place = None;
    for (index, (_, field)) in fields.iter().enumerate() {
        if c_text(memory, field.name)?.as_deref() == Some(on.as_bytes()) {
            place = Some(index);
        }
    }
    let Some((on_place, (_, field))) = place.and_then(|at| fields.get(at).map(|one| (at, one)))
    else {
        return Err(format!("the frame has no column named '{on}'"));
    };
    let text = text_layout(memory, field)?;
    let mut read = Frame {
        texts: Vec::new(),
        schemas: fields.iter().map(|(at, _)| *at).collect(),
        batches: Vec::new(),
    };
    for root in &held.batches {
        if usize::try_from(root.n_children).ok() != Some(count) {
            return Err(
                "a frame batch carried a different column count than its schema".to_owned(),
            );
        }
        let columns: Vec<ArrowArray> = children(memory, root.children, count)?
            .into_iter()
            .map(|(_, column)| column)
            .collect();
        let (Ok(offset), Ok(length)) = (usize::try_from(root.offset), usize::try_from(root.length))
        else {
            return Err(
                "the column's length or offset is negative; a malformed array is refused, not read"
                    .to_owned(),
            );
        };
        let column = columns.get(on_place).ok_or(UNREADABLE)?;
        let skip = offset.saturating_add(usize::try_from(column.offset).unwrap_or(0));
        read.texts
            .extend(borrow(held, memory, column, text, skip, length)?);
        read.batches.push(Batch {
            offset: root.offset,
            length: root.length,
            columns,
        });
    }
    Ok(read)
}

#[cfg(test)]
#[path = "read/tests.rs"]
mod tests;
