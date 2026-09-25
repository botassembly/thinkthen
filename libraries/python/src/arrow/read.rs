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

use super::ffi::{ArrowArray, ArrowSchema, Imported, bytes, record};
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
pub(super) fn c_text<'a, O: ?Sized>(
    owner: &'a O,
    memory: &Readable,
    at: *const c_char,
) -> Result<Option<&'a [u8]>, &'static str> {
    if at.is_null() {
        return Ok(None);
    }
    let reach = memory.reach(at.addr(), MAX_TEXT + 1);
    let held = bytes(owner, memory, at.cast(), reach).ok_or(BAD_TEXT)?;
    let end = held.iter().position(|byte| *byte == 0).ok_or(BAD_TEXT)?;
    Ok(held.get(..end))
}

/// Check a schema is a whole text column, and say why not.
pub(super) fn text_layout(memory: &Readable, schema: &ArrowSchema) -> Result<Text, String> {
    if !schema.dictionary.is_null() {
        return Err("a dictionary-encoded column is not read; cast it to text first".to_owned());
    }
    let format = c_text(&(), memory, schema.format)?.ok_or("an Arrow schema carries no format")?;
    if schema.n_children != 0 && format != b"+s" {
        return Err("a nested column is not a column of text".to_owned());
    }
    match format {
        b"u" => Ok(Text::Utf8),
        b"U" => Ok(Text::LargeUtf8),
        b"vu" => Ok(Text::View),
        b"+s" => {
            Err("a data frame is not a column; pass df[\"name\"], or annotate with on=".to_owned())
        }
        _ => Err(format!(
            "the column's Arrow format is '{}', not text",
            String::from_utf8_lossy(format)
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
    let table = bytes(&(), memory, array.buffers.cast(), count * WORD).ok_or(UNREADABLE)?;
    Ok(words(table, WORD).map(address).collect())
}

/// Borrow one array's rows `skip .. skip + count` as text.
pub(super) fn borrow<'a, O: ?Sized>(
    owner: &'a O,
    memory: &Readable,
    array: &ArrowArray,
    text: Text,
    skip: usize,
    count: usize,
) -> Result<Vec<&'a str>, String> {
    if array.null_count != 0 {
        return Err(
            "the column holds nulls; the engine needs text, so drop or fill them first".to_owned(),
        );
    }
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
    let rows = match text {
        Text::View => view_rows(owner, memory, &table, skip, count)?,
        Text::Utf8 | Text::LargeUtf8 => {
            offset_rows(owner, memory, &table, text, skip..skip + count, carried)?
        }
    };
    rows.into_iter()
        .map(|row| {
            std::str::from_utf8(row)
                .map_err(|_| "the column's buffer is not valid UTF-8".to_owned())
        })
        .collect()
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
) -> Result<Vec<&'a [u8]>, &'static str> {
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
    for one in views.chunks_exact(16).map(view) {
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
    for one in views.chunks_exact(16) {
        let at = view(one);
        let row = if at.size <= 12 {
            // The view borrows from `views`, which lives as long as `owner`.
            let whole: &'a [u8] = one;
            whole.get(4..4 + at.size)
        } else {
            held.get(at.index)
                .and_then(|(low, data)| data.get(at.offset - low..at.offset - low + at.size))
        };
        rows.push(row.ok_or(UNREADABLE)?);
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
    rows: std::ops::Range<usize>,
    carried: usize,
) -> Result<Vec<&'a [u8]>, &'static str> {
    let (skip, count) = (rows.start, rows.len());
    let (Some(&offsets), Some(&values)) = (table.get(1), table.get(2)) else {
        return Err("the string array lacks its offsets or its values buffer");
    };
    if offsets.is_null() || values.is_null() {
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
    let mut start = ends.first().copied().unwrap_or(0);
    for &end in ends.iter().skip(1) {
        if start < 0 || end < start {
            return Err("the column's offsets are reversed or negative");
        }
        if usize::try_from(end).is_ok_and(|end| end > MAX_DATA) {
            return Err("the column's offsets name more bytes than a text column carries");
        }
        start = end;
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
        .map(|(from, length)| data.get(from..from + length).ok_or(UNREADABLE))
        .collect()
}

/// Every row of a column, batch by batch.
pub(crate) fn series<'a>(column: &'a Imported, memory: &Readable) -> Result<Vec<&'a str>, String> {
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
    pub(crate) texts: Vec<&'a str>,
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
    let table = bytes(&(), memory, at.cast_const().cast(), count * WORD).ok_or(UNREADABLE)?;
    words(table, WORD)
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
    let format = c_text(&(), memory, schema.format)?.ok_or("an Arrow schema carries no format")?;
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
        if c_text(&(), memory, field.name)? == Some(on.as_bytes()) {
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

#[cfg(feature = "probe")]
/// The first batch's data and views (or offsets) buffer addresses, and its
/// length, for the zero-copy proof.
pub(crate) fn addresses(
    column: &Imported,
    memory: &Readable,
) -> Result<(usize, usize, usize), String> {
    let batch = column.batches.first().ok_or("the probe found no batch")?;
    let table = buffers(memory, batch)?;
    let at = |place: usize| table.get(place).map_or(0, |one| one.addr());
    Ok((at(2), at(1), usize::try_from(batch.length).unwrap_or(0)))
}

#[cfg(test)]
mod tests {
    //! The reviews' exit-139 probes, pinned: a malformed view or offsets
    //! buffer comes back as a refusal, never as a read past a buffer. The
    //! arrays are hand-built over real, small buffers in the interface's
    //! layouts, and each refusal pins its sentence, so a loosened check that
    //! refuses for another reason fails.

    use std::ffi::c_void;

    use super::super::ffi::{ArrowArray, ArrowSchema, EMPTY_ARRAY, EMPTY_SCHEMA};
    use super::super::memory::Readable;
    use super::{BAD_TEXT, MAX_TEXT, Text, UNREADABLE, borrow, c_text, text_layout};

    fn array_of(held: &[*const c_void], length: i64) -> ArrowArray {
        ArrowArray {
            length,
            n_buffers: held.len() as i64,
            buffers: held.as_ptr().cast_mut(),
            ..EMPTY_ARRAY
        }
    }

    fn pointers(buffers: &[&[u8]]) -> Vec<*const c_void> {
        buffers.iter().map(|one| one.as_ptr().cast()).collect()
    }

    fn one_view(size: u32, index: u32, offset: u32) -> Vec<u8> {
        let mut one = [0_u8; 16];
        one[0..4].copy_from_slice(&size.to_le_bytes());
        one[8..12].copy_from_slice(&index.to_le_bytes());
        one[12..16].copy_from_slice(&offset.to_le_bytes());
        one.to_vec()
    }

    fn snapshot() -> Readable {
        Readable::snapshot().expect("the memory map reads")
    }

    fn read(
        array: &ArrowArray,
        text: Text,
        skip: usize,
        rows: usize,
    ) -> Result<Vec<String>, String> {
        borrow(&(), &snapshot(), array, text, skip, rows)
            .map(|texts| texts.into_iter().map(str::to_owned).collect())
    }

    /// A view column `[validity, views, data.., sizes]` with its sizes set by hand.
    fn views_raw(
        views: &[u8],
        data: &[&[u8]],
        sizes: &[i64],
        rows: usize,
    ) -> Result<Vec<String>, String> {
        let sizes: Vec<u8> = sizes.iter().flat_map(|one| one.to_le_bytes()).collect();
        let mut table = vec![b"".as_slice(), views];
        table.extend_from_slice(data);
        table.push(&sizes);
        let held = pointers(&table);
        read(&array_of(&held, rows as i64), Text::View, 0, rows)
    }

    fn views_over(views: &[u8], data: &[&[u8]], rows: usize) -> Result<Vec<String>, String> {
        let sizes: Vec<i64> = data.iter().map(|one| one.len() as i64).collect();
        views_raw(views, data, &sizes, rows)
    }

    fn utf8_at(offsets: &[i64], values: *const u8, text: Text) -> Result<Vec<String>, String> {
        let bytes: Vec<u8> = match text {
            Text::LargeUtf8 => offsets.iter().flat_map(|one| one.to_le_bytes()).collect(),
            _ => offsets
                .iter()
                .flat_map(|one| (*one as i32).to_le_bytes())
                .collect(),
        };
        let held = [std::ptr::null(), bytes.as_ptr().cast(), values.cast()];
        let rows = offsets.len() - 1;
        read(&array_of(&held, rows as i64), text, 0, rows)
    }

    fn utf8_over(offsets: &[i64], values: &[u8]) -> Result<Vec<String>, String> {
        utf8_at(offsets, values.as_ptr(), Text::Utf8)
    }

    fn refusal<T: std::fmt::Debug>(outcome: Result<T, String>) -> String {
        outcome.expect_err("the shape must be refused, not read")
    }

    #[test]
    fn a_view_past_its_declared_length_is_refused() {
        let data = vec![b'x'; 100];
        for (size, offset) in [(100, 60_000_000), (13, 200), (13, 90)] {
            assert_eq!(
                refusal(views_over(&one_view(size, 0, offset), &[&data], 1)),
                "a string view reaches past the length its data buffer declares",
                "{size} at {offset}"
            );
        }
        assert_eq!(
            refusal(views_over(&one_view(0x4000_0000, 0, 0), &[&data], 1)),
            "a row names a size no text column carries"
        );
    }

    #[test]
    fn a_view_cannot_name_the_sizes_buffer() {
        let data = vec![b'x'; 100];
        for index in [1, 5] {
            assert_eq!(
                refusal(views_over(&one_view(13, index, 0), &[&data], 1)),
                "a string view points past its data buffers"
            );
        }
    }

    #[test]
    fn a_utf8_row_past_the_final_offset_is_refused() {
        for offsets in [[0, 200, 3].as_slice(), &[3, 1], &[-5, 3]] {
            assert_eq!(
                refusal(utf8_over(offsets, b"abc")),
                "the column's offsets are reversed or negative"
            );
        }
        assert_eq!(
            refusal(utf8_over(&[0, 60_000_000], b"abc")),
            "a row names a size no text column carries"
        );
    }

    #[test]
    fn a_declared_data_extent_past_the_cap_is_refused_and_a_big_slice_borrows() {
        assert_eq!(
            refusal(views_raw(
                &one_view(13, 0, u32::MAX),
                &[b"abc".as_slice()],
                &[i64::MAX],
                1
            )),
            "a string view's data buffer declares more bytes than a text column carries"
        );
        let past = "the column's offsets name more bytes than a text column carries";
        assert_eq!(
            refusal(utf8_at(
                &[i64::MAX - 13, i64::MAX],
                b"abc".as_ptr(),
                Text::LargeUtf8
            )),
            past
        );
        assert_eq!(
            refusal(utf8_over(
                &[i64::from(i32::MAX) - 13, i64::from(i32::MAX)],
                b"abc"
            )),
            past
        );
        assert_eq!(
            refusal(utf8_at(
                &[1 << 40, (1 << 40) + 13],
                b"abc".as_ptr(),
                Text::LargeUtf8
            )),
            past
        );
        // A slice's first row may start partway into its buffer, and a column
        // past the 4 MiB row cap still borrows.
        assert_eq!(
            utf8_over(&[5, 9], b"-----SEEN"),
            Ok(vec!["SEEN".to_owned()])
        );
        let values = vec![b'a'; 5 << 20];
        let rows: Vec<i64> = (0..=5).map(|one| one << 20).collect();
        assert_eq!(
            utf8_at(&rows, values.as_ptr(), Text::LargeUtf8).map(|texts| texts.len()),
            Ok(5)
        );
        assert_eq!(
            views_over(&one_view(13, 0, (5 << 20) - 13), &[&values], 1).map(|texts| texts.len()),
            Ok(1)
        );
    }

    #[test]
    fn a_buffer_table_far_longer_than_a_text_column_is_refused() {
        let offsets = [0_i32, 3].map(i32::to_le_bytes).concat();
        let held = pointers(&[b"".as_slice(), &offsets, b"abc"]);
        let mut outer = array_of(&held, 1);
        outer.n_buffers = 1 << 40;
        assert_eq!(
            refusal(read(&outer, Text::Utf8, 0, 1)),
            "the column's buffer table names more buffers than a text column carries"
        );
    }

    #[test]
    fn null_tables_and_row_claims_are_refused() {
        let outer = ArrowArray {
            length: 1,
            n_buffers: 3,
            ..EMPTY_ARRAY
        };
        assert_eq!(
            refusal(read(&outer, Text::View, 0, 1)),
            "the column carries no buffer table"
        );
        let values = b"abc".to_vec();
        let held = [std::ptr::null(), std::ptr::null(), values.as_ptr().cast()];
        assert_eq!(
            refusal(read(&array_of(&held, 1), Text::Utf8, 0, 1)),
            "the string array lacks its offsets or its values buffer"
        );
        let held = pointers(&[b"".as_slice(), &values]);
        assert_eq!(
            refusal(read(&array_of(&held, 1), Text::View, 0, 1)),
            "the string-view array lacks its views or its buffer-sizes buffer"
        );
        let offsets = [0_i32, 3].map(i32::to_le_bytes).concat();
        let held = pointers(&[b"".as_slice(), &offsets, &values]);
        let claims = [
            (
                1_i64 << 40,
                0,
                "the column claims more rows than any text column carries",
            ),
            (
                1,
                1 << 40,
                "the column claims more rows than any text column carries",
            ),
            (
                -1,
                0,
                "the column's length or offset is negative; a malformed array is refused, not read",
            ),
        ];
        for (length, offset, sentence) in claims {
            let outer = ArrowArray {
                offset,
                ..array_of(&held, length)
            };
            let skip = offset.max(0) as usize;
            assert_eq!(
                refusal(read(&outer, Text::Utf8, skip, length.max(0) as usize)),
                sentence
            );
        }
    }

    #[test]
    fn a_conformant_view_column_round_trips() {
        let first = b"alpha runs longer than twentyfour".to_vec();
        let second = b"beta also runs long past twelve".to_vec();
        let mut inline = one_view(5, 0, 0);
        inline[4..9].copy_from_slice(b"gamma");
        let views = [one_view(24, 0, 3), inline, one_view(21, 1, 4)].concat();
        assert_eq!(
            views_over(&views, &[&first, &second], 3),
            Ok(vec![
                "ha runs longer than twen".to_owned(),
                "gamma".to_owned(),
                " also runs long past ".to_owned()
            ])
        );
    }

    #[cfg(target_os = "linux")]
    mod guarded {
        use super::super::super::ffi::testing::{Guarded, PastEnd};
        use super::super::super::write::{BAD_METADATA, SchemaNode, metadata};
        use super::*;

        /// R3-4, R5-6 (a), R7-2: offsets that clear the row cap and the
        /// extent cap still land in unreadable memory, and the readability
        /// check refuses them first.
        #[test]
        fn an_extent_past_readable_memory_is_refused() {
            let (_region, abc) = Guarded::ending_with(b"abc");
            for text in [Text::Utf8, Text::LargeUtf8] {
                let past = [[60_000_000, 60_000_001], [0, 200]]
                    .map(|offsets| utf8_at(&offsets, abc, text));
                assert_eq!(
                    past.map(refusal),
                    [UNREADABLE, UNREADABLE].map(str::to_owned)
                );
                assert_eq!(utf8_at(&[0, 3], abc, text), Ok(vec!["abc".to_owned()]));
            }
        }

        /// R5-6 (b): the view names data buffer 1, whose size sits past a
        /// guard page, and a view whose 13 bytes run past the readable page.
        #[test]
        fn a_sizes_buffer_shorter_than_its_count_is_refused() {
            let data = [b'x'; 100];
            let (_region, sizes) = Guarded::ending_with(&100_i64.to_le_bytes());
            let views = one_view(13, 1, 0);
            let held = [
                std::ptr::null(),
                views.as_ptr().cast(),
                data.as_ptr().cast(),
                data.as_ptr().cast(),
                sizes.cast(),
            ];
            assert_eq!(
                refusal(read(&array_of(&held, 1), Text::View, 0, 1)),
                UNREADABLE
            );
            let (_short, tail) = Guarded::ending_with(&[b'y'; 20]);
            let sizes = [200_i64, 200].map(i64::to_le_bytes).concat();
            let views = one_view(13, 1, 10);
            let held = [
                std::ptr::null(),
                views.as_ptr().cast(),
                tail.cast(),
                tail.cast(),
                sizes.as_ptr().cast(),
            ];
            assert_eq!(
                refusal(read(&array_of(&held, 1), Text::View, 0, 1)),
                UNREADABLE
            );
        }

        /// R5-6 (c): a mapping longer than its file is listed readable, and a
        /// row, a string, or a metadata blob past the file's end raised SIGBUS.
        #[test]
        fn a_file_mapping_past_the_end_of_its_file_is_unreadable() {
            let words = |parts: &[i32]| {
                parts
                    .iter()
                    .flat_map(|one| one.to_le_bytes())
                    .collect::<Vec<u8>>()
            };
            for (backing, file) in PastEnd::each(b"abc") {
                let memory = snapshot();
                assert!(
                    memory.covers(file.tail(3).addr(), 3),
                    "{backing}: the file's bytes read"
                );
                assert!(
                    !memory.covers(file.tail(3).addr(), 4),
                    "{backing}: one byte past the file"
                );
                assert_eq!(memory.reach(file.tail(3).addr(), 100), 3, "{backing}");
                assert_eq!(
                    utf8_at(&[0, 3], file.tail(3), Text::Utf8),
                    Ok(vec!["abc".to_owned()]),
                    "{backing}"
                );
                assert_eq!(
                    refusal(utf8_at(&[0, 200], file.tail(3), Text::Utf8)),
                    UNREADABLE,
                    "{backing}"
                );
                assert_eq!(
                    c_text(&(), &memory, file.past().cast()),
                    Err(BAD_TEXT),
                    "{backing}"
                );
                assert_eq!(
                    metadata(&memory, file.past().cast()),
                    Err(BAD_METADATA),
                    "{backing}"
                );
                let blob = file.put(&words(&[1, 64]));
                assert_eq!(
                    metadata(&memory, blob.cast()),
                    Err(BAD_METADATA),
                    "{backing}"
                );
            }
        }

        /// R5-6 (d): a buffer table and a schema's child table each shorter
        /// than their counts, against a guard page.
        #[test]
        fn a_pointer_table_shorter_than_its_count_is_refused() {
            let offsets = [0_i32, 3].map(i32::to_le_bytes).concat();
            let values = b"abc".to_vec();
            let slots = [0, offsets.as_ptr().addr()]
                .map(usize::to_le_bytes)
                .concat();
            let (_region, table) = Guarded::ending_with(&slots);
            let outer = ArrowArray {
                length: 1,
                n_buffers: 3,
                buffers: table.cast_mut().cast(),
                ..EMPTY_ARRAY
            };
            assert_eq!(refusal(read(&outer, Text::Utf8, 0, 1)), UNREADABLE);
            let whole = [0, offsets.as_ptr().addr(), values.as_ptr().addr()]
                .map(usize::to_le_bytes)
                .concat();
            let (_region, table) = Guarded::ending_with(&whole);
            let outer = ArrowArray {
                buffers: table.cast_mut().cast(),
                ..outer
            };
            assert_eq!(read(&outer, Text::Utf8, 0, 1), Ok(vec!["abc".to_owned()]));
            let mut leaf = ArrowSchema {
                format: c"u".as_ptr(),
                ..EMPTY_SCHEMA
            };
            let one = (&raw mut leaf).addr().to_le_bytes();
            let (_region, children) = Guarded::ending_with(&one);
            let root = ArrowSchema {
                format: c"+s".as_ptr(),
                n_children: 2,
                children: children.cast_mut().cast(),
                ..EMPTY_SCHEMA
            };
            assert_eq!(
                SchemaNode::copy(&snapshot(), &root, 0).map(|_| ()),
                Err(UNREADABLE.to_owned())
            );
            let root = ArrowSchema {
                n_children: 1,
                ..root
            };
            assert!(SchemaNode::copy(&snapshot(), &root, 0).is_ok());
            let (_region, cut) = Guarded::ending_with(&vec![0_u8; size_of::<ArrowSchema>() - 8]);
            assert_eq!(
                SchemaNode::copy(&snapshot(), cut.cast(), 0).map(|_| ()),
                Err(UNREADABLE.to_owned())
            );
        }

        /// A 10-row slice of a large column is checked for its own rows only:
        /// every byte outside the rows read here is unreadable.
        #[test]
        fn only_the_bytes_the_selected_rows_read_are_checked() {
            let region = Guarded::new();
            let page = region.place(0, b"alpha runs past twelve");
            let values = page.wrapping_sub(Guarded::PAGE);
            let at = Guarded::PAGE as i64;
            assert_eq!(
                utf8_at(&[at, at + 5], values, Text::Utf8),
                Ok(vec!["alpha".to_owned()])
            );
            let offsets = [0, at, at + 5, at + 10, at + 9000]
                .map(|one| (one as i32).to_le_bytes())
                .concat();
            let held = [std::ptr::null(), offsets.as_ptr().cast(), values.cast()];
            let outer = ArrowArray {
                offset: 1,
                ..array_of(&held, 2)
            };
            assert_eq!(
                read(&outer, Text::Utf8, 1, 2),
                Ok(vec!["alpha".to_owned(), " runs".to_owned()])
            );
            let views = one_view(22, 0, Guarded::PAGE as u32);
            let sizes = (64_i64 << 20).to_le_bytes();
            let held = [
                std::ptr::null(),
                views.as_ptr().cast(),
                values.cast(),
                sizes.as_ptr().cast(),
            ];
            assert_eq!(
                read(&array_of(&held, 1), Text::View, 0, 1),
                Ok(vec!["alpha runs past twelve".to_owned()])
            );
        }

        /// A format or name that runs into a guard page, or has no NUL within
        /// the cap, is refused. One that ends at the page's edge reads.
        #[test]
        fn a_producer_string_is_read_only_inside_readable_memory() {
            let (_region, open) = Guarded::ending_with(b"abc");
            assert_eq!(c_text(&(), &snapshot(), open.cast()), Err(BAD_TEXT));
            let schema = ArrowSchema {
                format: open.cast(),
                ..EMPTY_SCHEMA
            };
            assert_eq!(text_layout(&snapshot(), &schema), Err(BAD_TEXT.to_owned()));
            let (_region, closed) = Guarded::ending_with(b"u\0");
            let schema = ArrowSchema {
                format: closed.cast(),
                ..EMPTY_SCHEMA
            };
            assert_eq!(text_layout(&snapshot(), &schema), Ok(Text::Utf8));
            let long = vec![b'x'; MAX_TEXT + 8];
            assert_eq!(
                c_text(&(), &snapshot(), long.as_ptr().cast()),
                Err(BAD_TEXT)
            );
        }
    }
}
