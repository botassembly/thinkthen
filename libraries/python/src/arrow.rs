//! The Arrow C Interface door: capsules in through `__arrow_c_stream__`,
//! capsules out through the same stream form.
//!
//! Nothing on the Python side needs Polars installed for the door itself:
//! any object answering `__arrow_c_stream__` crosses — a Polars `Series`
//! for the bulk verbs, a Polars `DataFrame` for `annotate`. The strings
//! are never copied on the way in: a text column is offsets over one
//! contiguous UTF-8 buffer, and the engine takes `&[&str]` borrowed
//! straight from the producer's memory. On the way out, `annotate_stream`
//! builds the whole frame — the caller's original columns aliased and the
//! new question columns appended — behind an `ArrowFrame` whose
//! `__arrow_c_stream__` mints a fresh C stream, so `type(records)(frame)`
//! constructs the host's own frame with no import of the host library.
//!
//! Only whole-column text is accepted on the way in (`u` and `U`). Nulls,
//! dictionaries, and nested children are refused with a `UsageError` that
//! names why, because the stand-in engine has no NA row and no
//! distinct-value dedup; those are engine features, not shim features.

use std::ffi::{c_char, c_void, CStr, CString};
use std::os::raw::c_int;
use std::ptr::NonNull;

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyCapsule};

use thinkthen_contract::Annotated;

use crate::UsageError;

/// `ArrowSchema` from the C Data Interface, field order per the spec.
#[repr(C)]
struct ArrowSchema {
    format: *const c_char,
    name: *const c_char,
    metadata: *const c_char,
    flags: i64,
    n_children: i64,
    children: *mut *mut ArrowSchema,
    dictionary: *mut ArrowSchema,
    release: Option<unsafe extern "C" fn(*mut ArrowSchema)>,
    private_data: *mut c_void,
}

/// `ArrowArray` from the C Data Interface, field order per the spec:
/// `n_children` comes before `buffers`, unlike `ArrowSchema`.
#[repr(C)]
struct ArrowArray {
    length: i64,
    null_count: i64,
    offset: i64,
    n_buffers: i64,
    n_children: i64,
    buffers: *mut *const c_void,
    children: *mut *mut ArrowArray,
    dictionary: *mut ArrowArray,
    release: Option<unsafe extern "C" fn(*mut ArrowArray)>,
    private_data: *mut c_void,
}

/// `ArrowArrayStream` from the C Stream Interface, field order per the spec.
#[repr(C)]
struct ArrowArrayStream {
    get_schema: Option<unsafe extern "C" fn(*mut ArrowArrayStream, *mut ArrowSchema) -> c_int>,
    get_next: Option<unsafe extern "C" fn(*mut ArrowArrayStream, *mut ArrowArray) -> c_int>,
    get_last_error: Option<unsafe extern "C" fn(*mut ArrowArrayStream) -> *const c_char>,
    release: Option<unsafe extern "C" fn(*mut ArrowArrayStream)>,
    private_data: *mut c_void,
}

/// The capsule name the stream protocol fixes.
const STREAM_NAME: &CStr = c"arrow_array_stream";

/// Read the pointer a capsule hands out, checking the name it was built with.
unsafe fn capsule_pointer(capsule: &Bound<'_, PyAny>) -> PyResult<*mut c_void> {
    let pointer = pyo3::ffi::PyCapsule_GetPointer(capsule.as_ptr(), STREAM_NAME.as_ptr());
    if pointer.is_null() {
        return Err(PyErr::take(capsule.py())
            .unwrap_or_else(|| UsageError::new_err("the capsule would not open")));
    }
    Ok(pointer)
}

/// The three whole-text layouts the door reads.
#[derive(Clone, Copy)]
pub(crate) enum Text {
    /// `u`: 32-bit offsets over one UTF-8 buffer.
    Utf8,
    /// `U`: 64-bit offsets over one UTF-8 buffer.
    LargeUtf8,
    /// `vu`: the string-view layout, 16-byte views and one or more data
    /// buffers; strings of 12 bytes or less sit inline in the view.
    View,
}

/// A producer's schema, released exactly once when the guard drops.
///
/// Every error path between a stream's `get_schema` and the value that
/// keeps the schema (a `SeriesColumn`, a `FrameColumn`) runs through this
/// guard, so a refused input releases the schema the producer handed over
/// instead of leaking it.
struct SchemaGuard(Option<ArrowSchema>);

impl SchemaGuard {
    fn empty() -> Self {
        Self(Some(unsafe { std::mem::zeroed() }))
    }

    fn held(&self) -> &ArrowSchema {
        self.0.as_ref().expect("the guard holds its schema until take")
    }

    /// The place a producer's `get_schema` fills.
    fn pointer(&mut self) -> *mut ArrowSchema {
        let schema = self.0.as_mut().expect("the guard holds its schema");
        schema as *mut ArrowSchema
    }

    /// The schema, given up to its new owner.
    fn take(&mut self) -> ArrowSchema {
        std::mem::replace(
            self.0.as_mut().expect("the guard holds its schema until take"),
            unsafe { std::mem::zeroed() },
        )
    }
}

impl Drop for SchemaGuard {
    fn drop(&mut self) {
        if let Some(schema) = self.0.as_mut() {
            if let Some(release) = schema.release {
                unsafe { release(schema as *mut ArrowSchema) };
            }
        }
    }
}

/// Check a schema is exactly a whole text column, and say what to do if not.
unsafe fn require_text(schema: &ArrowSchema) -> PyResult<Text> {
    if !schema.dictionary.is_null() {
        return Err(UsageError::new_err(
            "a dictionary-encoded column: the engine judges each distinct value once; this stand-in does not, so it is refused",
        ));
    }
    if schema.n_children != 0 {
        return Err(UsageError::new_err("a nested column is not a column of text"));
    }
    let format = CStr::from_ptr(schema.format).to_bytes();
    match format {
        b"u" => Ok(Text::Utf8),
        b"U" => Ok(Text::LargeUtf8),
        b"vu" => Ok(Text::View),
        _ => Err(UsageError::new_err(format!(
            "the column's Arrow format is '{}', not text",
            String::from_utf8_lossy(format)
        ))),
    }
}

/// The longest single row the door will borrow: far past any text the
/// engine classifies. It caps one row's length, never a position in a
/// buffer, so a column of any total size still borrows.
const MAX_ONE_STRING: usize = 4 * 1024 * 1024;

/// The most rows one borrowed column may claim, counting its offset.
///
/// The C data interface types its length and offset as i64, and
/// `Vec::with_capacity` aborts the process on an allocation it cannot
/// make — a length of 2^40 kills the host before any guard could answer.
/// Four billion rows is past every text column ever carried, so a claim
/// beyond it is refused as the usage kind instead of allocated or walked.
const MAX_ROWS: usize = u32::MAX as usize;

/// The most bytes one text column may declare its data buffer holds: a
/// Utf8 array's final offset, or a string view's per-buffer declared size.
///
/// The interface gives no buffer length, so a producer that declares an
/// extent past what it allocated still reads inside its own declaration.
/// This cap bounds that residual read into a data buffer to at most a
/// gibibyte past it, instead of the 2^63 an offset near `i64::MAX` — or an
/// i32 offset at its own maximum, which no real 2 GiB allocation backs —
/// would otherwise name. The bound sits on the whole declared extent, so
/// the first byte a slice reads (the array's own offset into the buffer)
/// is bounded by it too. A gibibyte is 256 times the per-row cap and past
/// any single chunk of text a classifier is handed; a real column that
/// large arrives in chunks, and one chunk declaring more is refused as
/// usage.
///
/// This governs the data buffers only. The views buffer and the offsets
/// buffer are declared by the row count, not by a byte extent, so their
/// walk is bounded instead by `MAX_ROWS` (16 bytes a view, up to 8 a
/// offset). That is a looser reach past a short buffer, and it is the
/// pre-existing row-count regime, not a promise this cap makes.
const MAX_DATA: usize = 1024 * 1024 * 1024;

/// The most buffers a text array's table may hold. Utf8 carries three and a
/// string view carries a handful of data buffers beside its views and
/// sizes; a table claiming more is a malformed count the reader refuses
/// before it indexes the table's last slot (the sizes buffer).
const MAX_BUFFERS: i64 = 64;

/// The refusal for a metadata blob that is malformed, past the cap, or
/// runs into unreadable memory.
const BAD_METADATA: &str = "a column's Arrow metadata names lengths past its readable bytes or past 16 MiB";

/// The most bytes one metadata blob may hold. Real metadata (an extension
/// type's name and parameters, a pandas schema note) runs to kilobytes;
/// this bounds the pair count and every length a crafted blob declares.
const MAX_METADATA: usize = 16 * 1024 * 1024;

/// The refusal for an extent that names memory this process cannot read.
const UNREADABLE: &str = "the column's buffers declare bytes this process cannot read";

/// The refusal when this process cannot list its own readable memory.
#[cfg(target_os = "linux")]
const NO_MAP: &str = "this process cannot read its memory map (/proc/self/maps), so the Arrow door cannot check a column before it reads it";

/// The memory this process can read, taken once before a column is read.
///
/// The interface gives no allocation length, so a declared extent past a
/// short allocation would read into whatever follows it. When nothing
/// readable follows (an unmapped page, a guard page, a reservation), the
/// read would kill the host. The reader checks each extent against this
/// snapshot first. On Linux the snapshot is the readable mappings in
/// `/proc/self/maps`, adjacent ones merged, so a protected page and an
/// unmapped page both fail the check. The check reads a file and makes no
/// system call a sandbox might forbid or answer with a kill, and it faults
/// in no page. A process that cannot read its own map gets a refusal for
/// every Arrow column. Off Linux, `mincore` finds unmapped pages only.
/// Memory that is mapped and readable but belongs to another allocation
/// cannot be told apart by any reader; that lie stays the producer's.
pub(crate) struct Readable {
    #[cfg(target_os = "linux")]
    ranges: Vec<(usize, usize)>,
}

impl Readable {
    /// Take the snapshot.
    pub(crate) fn snapshot() -> PyResult<Self> {
        #[cfg(target_os = "linux")]
        {
            let map = std::fs::read_to_string("/proc/self/maps")
                .map_err(|_| UsageError::new_err(NO_MAP))?;
            Ok(Self { ranges: readable_ranges(&map) })
        }
        #[cfg(not(target_os = "linux"))]
        Ok(Self {})
    }

    /// True when every byte of `[start, start + length)` can be read.
    fn covers(&self, start: *const u8, length: usize) -> bool {
        if length == 0 {
            return true;
        }
        let first = start as usize;
        let Some(end) = first.checked_add(length) else {
            return false;
        };
        #[cfg(target_os = "linux")]
        {
            let place = self.ranges.partition_point(|&(_, high)| high <= first);
            self.ranges
                .get(place)
                .is_some_and(|&(low, high)| low <= first && end <= high)
        }
        #[cfg(not(target_os = "linux"))]
        mapped(first, end - 1)
    }
}

/// The readable ranges of a `/proc/self/maps` text, in address order, with
/// touching ranges merged so an extent across two mappings still checks.
#[cfg(target_os = "linux")]
fn readable_ranges(map: &str) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for line in map.lines() {
        let mut fields = line.split_whitespace();
        let (Some(span), Some(access)) = (fields.next(), fields.next()) else {
            continue;
        };
        let Some((low, high)) = span.split_once('-') else {
            continue;
        };
        let (Ok(low), Ok(high)) = (usize::from_str_radix(low, 16), usize::from_str_radix(high, 16))
        else {
            continue;
        };
        if !access.starts_with('r') {
            continue;
        }
        match ranges.last_mut() {
            Some(last) if last.1 == low => last.1 = high,
            _ => ranges.push((low, high)),
        }
    }
    ranges
}

/// True when every page from `first` to `last` is mapped (off Linux).
#[cfg(not(target_os = "linux"))]
fn mapped(first: usize, last: usize) -> bool {
    extern "C" {
        fn mincore(address: *mut c_void, length: usize, pages: *mut u8) -> c_int;
        fn getpagesize() -> c_int;
    }
    // SAFETY: getpagesize takes no argument and cannot fail.
    let page = usize::try_from(unsafe { getpagesize() }).unwrap_or(4096).max(1);
    let base = first - first % page;
    let span = last - base + 1;
    let mut pages = vec![0u8; span.div_ceil(page)];
    // SAFETY: mincore reads no byte of the range; it writes one byte a
    // page into `pages`, which holds a byte for every page asked about.
    unsafe { mincore(base as *mut c_void, span, pages.as_mut_ptr()) == 0 }
}

/// Borrow one already-validated array's strings out of its buffers.
///
/// `skip` is where the caller's rows begin and `count` how many there
/// are. For a series-shaped array they are the array's own offset and
/// length; for a frame's `on` column the struct root's offset and length
/// are added, because the Arrow columnar format lets a sliced struct keep
/// its slice on the root and leave the children whole — the reader
/// applies the parent's offset when indexing.
///
/// What the reader can prove, and what it cannot (surfaces-review-5):
/// the interface carries no byte length beside a buffer pointer. It
/// declares extents instead. The array's `offset + length` declares the
/// views and offsets buffers. A string view's trailing sizes buffer
/// declares each data buffer. A Utf8 array's offset at `offset + length`
/// declares its data buffer. Every read below sits inside one of those
/// declarations, checked before the read. The offsets, views, and sizes
/// the rows rely on, and the data bytes the selected rows read, are then
/// checked readable (`Readable`), so a producer whose declarations agree
/// but whose allocation is shorter gets a refusal when a read would run
/// into unreadable memory. An extent that runs into another readable
/// allocation cannot be told from a correct one by any reader; that lie
/// stays the producer's.
///
/// # Safety
/// `array` must point at a live `ArrowArray` of the given text layout,
/// and the memory must outlive the returned strings.
unsafe fn borrow_strings(
    array: *const ArrowArray,
    text: Text,
    skip: usize,
    count: usize,
) -> PyResult<Vec<&'static str>> {
    if (*array).null_count != 0 {
        return Err(UsageError::new_err(
            "the column holds nulls; the engine needs text, and NA rows are the caller's to drop",
        ));
    }
    if (*array).length < 0 || (*array).offset < 0 {
        return Err(UsageError::new_err(
            "the column's length or offset is negative; a malformed array is refused, not read",
        ));
    }
    if (*array).buffers.is_null() {
        return Err(UsageError::new_err("the column carries no buffer table"));
    }
    if (*array).n_buffers < 0 || (*array).n_buffers > MAX_BUFFERS {
        return Err(UsageError::new_err(
            "the column's buffer table names more buffers than a text column carries",
        ));
    }
    let carried = ((*array).offset as usize).saturating_add((*array).length as usize);
    if carried > MAX_ROWS {
        return Err(UsageError::new_err(
            "the column claims more rows than any text column carries",
        ));
    }
    if skip.saturating_add(count) > carried {
        return Err(UsageError::new_err(
            "the frame's struct root asks for rows its column does not carry",
        ));
    }
    let memory = Readable::snapshot()?;
    let spans = match text {
        Text::View => view_spans(array, &memory, skip, count)?,
        Text::Utf8 | Text::LargeUtf8 => offset_spans(array, &memory, text, skip, count, carried)?,
    };
    let mut texts = Vec::with_capacity(count);
    for (start, length) in spans {
        let bytes = std::slice::from_raw_parts(start, length);
        let value = std::str::from_utf8(bytes)
            .map_err(|_| UsageError::new_err("the column's buffer is not valid UTF-8"))?;
        texts.push(value);
    }
    Ok(texts)
}

/// One row's span, refused when it is longer than any row: a per-row
/// ceiling, not a bound. It keeps `from_raw_parts` far inside its isize
/// contract and refuses a giant row before any probe or read.
fn row_span(start: *const u8, length: usize) -> PyResult<(*const u8, usize)> {
    if length > MAX_ONE_STRING {
        return Err(UsageError::new_err(
            "a row names a size no text column carries",
        ));
    }
    Ok((start, length))
}

/// One buffer pointer out of the table, as bytes.
unsafe fn buffer(array: *const ArrowArray, place: usize) -> *const u8 {
    *(*array).buffers.add(place) as *const u8
}

/// Read one little-endian word of `N` bytes at `at`, unaligned.
unsafe fn word<const N: usize>(at: *const u8) -> [u8; N] {
    std::ptr::read_unaligned(at as *const [u8; N])
}

/// Where each string-view row's bytes are, each span proven inside the
/// data buffer the sizes buffer declares.
///
/// The table is `[validity, views, data 0 .. data n-1, sizes]`: the C data
/// interface appends one buffer of `n` i64 lengths, so the data count is
/// the table minus three, and no view can name the sizes buffer itself.
unsafe fn view_spans(
    array: *const ArrowArray,
    memory: &Readable,
    skip: usize,
    count: usize,
) -> PyResult<Vec<(*const u8, usize)>> {
    if (*array).n_buffers < 3 {
        return Err(UsageError::new_err(
            "the string-view array lacks its views or its buffer-sizes buffer",
        ));
    }
    let data_buffers = ((*array).n_buffers - 3) as usize;
    let views = buffer(array, 1);
    let sizes = buffer(array, data_buffers + 2);
    if views.is_null() || (data_buffers > 0 && sizes.is_null()) {
        return Err(UsageError::new_err(
            "the string-view array lacks its views or its buffer-sizes buffer",
        ));
    }
    // The views the rows read and the whole sizes buffer the count
    // declares must be readable before either is read.
    if !memory.covers(views.wrapping_add(skip * 16), count * 16)
        || !memory.covers(sizes, data_buffers * 8)
    {
        return Err(UsageError::new_err(UNREADABLE));
    }
    // The lowest and highest byte the rows read in each data buffer.
    let mut reach: Vec<Option<(usize, usize)>> = vec![None; data_buffers];
    let mut spans = Vec::with_capacity(count);
    for place in skip..skip + count {
        let view = views.add(place * 16);
        let size = u32::from_le_bytes(word(view)) as usize;
        if size <= 12 {
            spans.push(row_span(view.add(4), size)?);
            continue;
        }
        let index = u32::from_le_bytes(word(view.add(8))) as usize;
        let offset = u32::from_le_bytes(word(view.add(12))) as u64;
        if index >= data_buffers {
            return Err(UsageError::new_err(
                "a string view points past its data buffers",
            ));
        }
        let declared = i64::from_le_bytes(word(sizes.add(index * 8)));
        if declared < 0 || declared as u64 > MAX_DATA as u64 {
            return Err(UsageError::new_err(
                "a string view's data buffer declares more bytes than a text column carries",
            ));
        }
        let base = buffer(array, 2 + index);
        if offset + size as u64 > declared as u64 || base.is_null() {
            return Err(UsageError::new_err(
                "a string view reaches past the length its data buffer declares",
            ));
        }
        let span = row_span(base.add(offset as usize), size)?;
        let (low, high) = (offset as usize, offset as usize + size);
        reach[index] = Some(reach[index].map_or((low, high), |(was_low, was_high)| {
            (was_low.min(low), was_high.max(high))
        }));
        spans.push(span);
    }
    // Only the bytes the selected rows read are checked, so a short slice
    // of a large buffer costs its own rows, and no unread page is touched.
    for (index, reached) in reach.iter().enumerate() {
        if let Some((low, high)) = *reached {
            if !memory.covers(buffer(array, 2 + index).wrapping_add(low), high - low) {
                return Err(UsageError::new_err(UNREADABLE));
            }
        }
    }
    Ok(spans)
}

/// Where each Utf8 or LargeUtf8 row's bytes are, each span proven inside
/// the data extent the array's last offset declares.
///
/// The offsets are checked monotone from the first row read to the last
/// offset, and nonnegative, before any data byte is touched.
unsafe fn offset_spans(
    array: *const ArrowArray,
    memory: &Readable,
    text: Text,
    skip: usize,
    count: usize,
    carried: usize,
) -> PyResult<Vec<(*const u8, usize)>> {
    let (offsets, values) = if (*array).n_buffers < 3 {
        (std::ptr::null(), std::ptr::null())
    } else {
        (buffer(array, 1), buffer(array, 2))
    };
    if offsets.is_null() || values.is_null() {
        return Err(UsageError::new_err(
            "the string array lacks its offsets or its values buffer",
        ));
    }
    // SAFETY: `place` never passes `carried`, and the array's offset plus
    // length declares `carried + 1` offsets, probed readable here first.
    let width = if matches!(text, Text::LargeUtf8) { 8 } else { 4 };
    if !memory.covers(offsets.wrapping_add(skip * width), (carried - skip + 1) * width) {
        return Err(UsageError::new_err(UNREADABLE));
    }
    let at = |place: usize| -> i64 {
        unsafe {
            if matches!(text, Text::LargeUtf8) {
                i64::from_le_bytes(word(offsets.add(place * 8)))
            } else {
                i64::from(i32::from_le_bytes(word(offsets.add(place * 4))))
            }
        }
    };
    let mut spans = Vec::with_capacity(count);
    let first = at(skip);
    let mut start = first;
    let mut read_end = first;
    for place in skip + 1..=carried {
        let end = at(place);
        if start < 0 || end < start {
            return Err(UsageError::new_err(
                "the column's offsets are reversed or negative",
            ));
        }
        // The final offset declares the values buffer's extent; the
        // offsets rise to it, so bounding it bounds every byte read,
        // including a slice's own first offset.
        if end as usize > MAX_DATA {
            return Err(UsageError::new_err(
                "the column's offsets name more bytes than a text column carries",
            ));
        }
        if place <= skip + count {
            spans.push(row_span(values.add(start as usize), (end - start) as usize)?);
            read_end = end;
        }
        start = end;
    }
    // The selected rows read from their first offset to their last, so
    // only those bytes are checked: a short slice of a large column costs
    // its own rows, and the bytes before and after it are never touched.
    if count > 0 && !memory.covers(values.wrapping_add(first as usize), (read_end - first) as usize) {
        return Err(UsageError::new_err(UNREADABLE));
    }
    Ok(spans)
}

/// Read a stream's own error phrase, or fall back to the context.
unsafe fn stream_error(stream: *mut ArrowArrayStream, context: &str) -> PyErr {
    let text = (*stream)
        .get_last_error
        .map_or(std::ptr::null(), |last| last(stream));
    let phrase = if text.is_null() {
        None
    } else {
        CStr::from_ptr(text).to_str().ok().map(str::to_owned)
    };
    UsageError::new_err(phrase.unwrap_or_else(|| context.to_owned()))
}

/// True when the object carries the stream door.
pub(crate) fn is_arrow(records: &Bound<'_, PyAny>) -> PyResult<bool> {
    records.hasattr("__arrow_c_stream__")
}

/// One text column borrowed from the producer's memory, whole.
///
/// The `&str` values claim `'static` through the borrow that the live
/// owners make true: `owners` holds the capsule, and `batches` defers
/// every stream batch's release to `Drop`. The engine call that reads
/// `texts` happens while this value is alive.
pub(crate) struct SeriesColumn {
    pub(crate) texts: Vec<&'static str>,
    owners: Vec<Py<PyAny>>,
    batches: Vec<ArrowArray>,
    schemas: Vec<ArrowSchema>,
}

impl Drop for SeriesColumn {
    fn drop(&mut self) {
        for array in &mut self.batches {
            if let Some(release) = array.release {
                unsafe { release(array as *mut ArrowArray) };
            }
        }
        for schema in &mut self.schemas {
            if let Some(release) = schema.release {
                unsafe { release(schema as *mut ArrowSchema) };
            }
        }
        let _ = &self.owners;
    }
}

/// Take a whole column from a `Series`-shaped stream.
pub(crate) fn series_column(records: &Bound<'_, PyAny>) -> PyResult<SeriesColumn> {
    if !is_arrow(records)? {
        return Err(UsageError::new_err("not an Arrow column"));
    }
    let capsule = records.call_method0("__arrow_c_stream__")?;
    let stream = unsafe { capsule_pointer(&capsule)? as *mut ArrowArrayStream };
    let mut column = SeriesColumn {
        texts: Vec::new(),
        owners: vec![capsule.unbind()],
        batches: Vec::new(),
        schemas: Vec::new(),
    };
    unsafe {
        let mut schema = SchemaGuard::empty();
        let got = (*stream).get_schema.map_or(-1, |get| get(stream, schema.pointer()));
        if got != 0 {
            return Err(stream_error(stream, "the column stream would not name its schema"));
        }
        let large = require_text(schema.held())?;
        column.schemas.push(schema.take());
        loop {
            let mut array = std::mem::zeroed::<ArrowArray>();
            let got = (*stream).get_next.map_or(-1, |next| next(stream, &mut array));
            if got != 0 {
                return Err(stream_error(stream, "the column stream stopped mid-column"));
            }
            if array.release.is_none() {
                break;
            }
            // Own the batch before reading it, so a refusal below releases
            // the producer's buffers together with the column instead of
            // leaking them.
            let skip = array.offset.max(0) as usize;
            let count = array.length.max(0) as usize;
            column.batches.push(array);
            let held = column.batches.last().expect("just pushed");
            column.texts.append(&mut borrow_strings(held, large, skip, count)?);
        }
    }
    Ok(column)
}

/// One batch of the caller's frame, kept whole so its other columns can
/// be aliased on the way out.
struct BatchHold {
    root: ArrowArray,
    child_ptrs: *mut *mut ArrowArray,
}

impl BatchHold {
    /// The child array at `place`, by value (a shallow copy that aliases
    /// the producer's buffers while the hold lives).
    ///
    /// # Safety
    /// `place` must be below `n_children`; the hold keeps the producer's
    /// child structs and buffers alive.
    unsafe fn child(&self, place: usize) -> ArrowArray {
        std::ptr::read(*self.child_ptrs.add(place))
    }
}

/// A frame-shaped column: the `on` column's texts borrowed, every other
/// column kept as a shallow batch hold for aliasing, and the schema held
/// so the output can deep-copy each column's own tree.
pub(crate) struct FrameColumn {
    pub(crate) texts: Vec<&'static str>,
    on: usize,
    names: Vec<String>,
    lengths: Vec<usize>,
    hold: Vec<BatchHold>,
    schema: Box<ArrowSchema>,
    owners: Vec<Py<PyAny>>,
}

impl Drop for FrameColumn {
    fn drop(&mut self) {
        for batch in &mut self.hold {
            if let Some(release) = batch.root.release {
                unsafe { release(&mut batch.root as *mut ArrowArray) };
            }
        }
        if let Some(release) = self.schema.release {
            unsafe { release(self.schema.as_mut() as *mut ArrowSchema) };
        }
        let _ = &self.owners;
    }
}

/// Take the whole frame from a `DataFrame`-shaped stream, borrowing the
/// `on` column and holding everything else for the output aliases.
pub(crate) fn frame_column(records: &Bound<'_, PyAny>, on: &str) -> PyResult<FrameColumn> {
    if !is_arrow(records)? {
        return Err(UsageError::new_err(
            "annotate with on= takes a data frame whose column crosses as Arrow",
        ));
    }
    let capsule = records.call_method0("__arrow_c_stream__")?;
    let stream = unsafe { capsule_pointer(&capsule)? as *mut ArrowArrayStream };

    let mut schema = SchemaGuard::empty();
    unsafe {
        let got = (*stream).get_schema.map_or(-1, |get| get(stream, schema.pointer()));
        if got != 0 {
            return Err(stream_error(stream, "the frame stream would not name its schema"));
        }
        let format = CStr::from_ptr(schema.held().format).to_bytes();
        if format != b"+s" {
            return Err(UsageError::new_err(format!(
                "the stream is '{}', not a frame (a struct); annotate with on= needs a data frame, and a plain column rides the series door",
                String::from_utf8_lossy(format)
            )));
        }
    }
    let count = schema.held().n_children.max(0) as usize;
    if count == 0 {
        return Err(UsageError::new_err("the frame has no columns"));
    }
    if schema.held().children.is_null() {
        // A struct root that names children but carries no child table is
        // malformed, and reading the null table would fault; refuse it as
        // usage instead of walking the null pointer (review-4, the
        // null-children frame root).
        return Err(UsageError::new_err(
            "the frame names columns but carries no column arrays",
        ));
    }
    let mut names = Vec::with_capacity(count);
    let mut on_index = None;
    for place in 0..count {
        let child = unsafe { *schema.held().children.add(place) };
        if child.is_null() {
            return Err(UsageError::new_err("the frame named a column it did not carry"));
        }
        let child_ref = unsafe { &*child };
        let name = if child_ref.name.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(child_ref.name) }
                .to_string_lossy()
                .into_owned()
        };
        if name == on {
            on_index = Some(place);
        }
        names.push(name);
    }
    let Some(on_index) = on_index else {
        return Err(UsageError::new_err(format!(
            "the frame has no column named '{on}'"
        )));
    };
    // The `on` column must be whole text, checked on its own schema.
    let large = unsafe {
        let child = *schema.held().children.add(on_index);
        require_text(&*child)?
    };

    let mut column = FrameColumn {
        texts: Vec::new(),
        on: on_index,
        names,
        lengths: Vec::new(),
        hold: Vec::new(),
        schema: Box::new(schema.take()),
        owners: vec![capsule.unbind()],
    };

    unsafe {
        loop {
            let mut array = std::mem::zeroed::<ArrowArray>();
            let got = (*stream).get_next.map_or(-1, |next| next(stream, &mut array));
            if got != 0 {
                return Err(stream_error(stream, " the frame stream stopped mid-frame"));
            }
            if array.release.is_none() {
                break;
            }
            if array.n_children as usize != column.names.len() {
                return Err(UsageError::new_err(
                    "a frame batch carried a different column count than its schema",
                ));
            }
            if array.children.is_null() {
                // The batch side of the null-children shape: the root
                // claims columns it does not carry, and the hold would
                // walk a null child table when it reads the `on` column.
                return Err(UsageError::new_err(
                    "a frame batch names columns but carries no column arrays",
                ));
            }
            let child_ptrs = array.children;
            let root_offset = array.offset.max(0) as usize;
            let root_length = array.length.max(0) as usize;
            // Own the batch before reading it, so a refusal below releases
            // the producer's buffers together with the hold instead of
            // leaking them.
            column.lengths.push(root_length);
            column.hold.push(BatchHold {
                root: array,
                child_ptrs,
            });
            let held = column.hold.last().expect("just pushed");
            let child = held.child(column.on);
            let child_offset = child.offset.max(0) as usize;
            column.texts.append(&mut borrow_strings(
                &child,
                large,
                root_offset + child_offset,
                root_length,
            )?);
        }
    }
    Ok(column)
}

/// One batch's whole allocation, handed to the consumer with the array.
/// The root and each child hold a share of it (`BatchShare`), so the
/// memory lives until the consumer releases the last of them.
///
/// Each batch carries its own root, because one stream may hand out
/// several batches and every one must describe its own children: a shared
/// root template would make the second batch alias the first batch's
/// arrays, and the first batch's box is freed as soon as the consumer
/// releases it.
#[allow(dead_code)] // the fields anchor the pointers handed out in the array
struct BatchKeep {
    /// This batch's own root struct, read out on emission. The box is
    /// load-bearing: the struct holds pointers into `root_buffers`, and
    /// an unboxed value could move when the keeping Vec grows.
    #[allow(clippy::vec_box, reason = "pointer stability for the C tree")]
    root: Box<ArrowArray>,
    root_buffers: Vec<*const c_void>,
    /// Boxed for pointer stability, like `root`.
    #[allow(clippy::vec_box, reason = "pointer stability for the C tree")]
    children: Vec<Box<ArrowArray>>,
    child_ptrs: Vec<*mut ArrowArray>,
    buffers: Vec<Vec<*const c_void>>,
    owned: Vec<Vec<u8>>,
    /// The caller's frame, kept alive while any aliased original column
    /// of this batch is alive. `None` for a table this surface built
    /// whole, where every buffer is its own.
    hold: Option<std::sync::Arc<FrameColumn>>,
}

/// One share of an emitted batch. The root and every child each carry one
/// in `private_data`, so a consumer that moves a child out and releases the
/// parent still owns the child's buffers, as the C data interface's rules
/// for moving child arrays require. The batch is freed with its last share.
type BatchShare = std::sync::Arc<BatchKeep>;

/// Hand one batch to the consumer: every child gets its own share, then
/// the root gets one.
///
/// # Safety
/// `out` must be writable; the batch's child pointers point into its own
/// boxes, which the shares keep alive.
unsafe fn emit_batch(keep: BatchKeep, out: *mut ArrowArray) {
    // The C data interface lets a consumer release the root and each moved
    // child from any thread, so the shares count atomically (Arc).
    #[allow(clippy::arc_with_non_send_sync, reason = "raw Arrow pointers; the atomic count is what crosses threads")]
    let share = std::sync::Arc::new(keep);
    for &child in &share.child_ptrs {
        (*child).private_data = Box::into_raw(Box::new(BatchShare::clone(&share))).cast();
        (*child).release = Some(child_release);
    }
    // The emitted root is this batch's own, so the second batch describes
    // its own children rather than the first's.
    *out = std::ptr::read(share.root.as_ref());
    (*out).release = Some(root_release);
    (*out).private_data = Box::into_raw(Box::new(share)).cast();
}

/// Drop the share a node carries and mark it released, as the C data
/// interface requires, so a second call cannot free again.
unsafe fn drop_share(node: *mut ArrowArray) {
    (*node).release = None;
    let share = (*node).private_data as *mut BatchShare;
    if !share.is_null() {
        (*node).private_data = std::ptr::null_mut();
        drop(Box::from_raw(share));
    }
}

/// A child's release: its share goes. The caller's aliased columns keep
/// their producer's own children, which the frame hold releases when the
/// last share drops, so this never walks below the child.
unsafe extern "C" fn child_release(child: *mut ArrowArray) {
    drop_share(child);
}

/// The root's release: release every child the consumer has not moved out
/// (a moved child's slot reads released), then drop the root's share.
unsafe extern "C" fn root_release(root: *mut ArrowArray) {
    let count = (*root).n_children.max(0) as usize;
    for place in 0..count {
        let child = *(*root).children.add(place);
        if let Some(release) = (*child).release {
            release(child);
        }
    }
    drop_share(root);
}

/// The owned pieces of one schema tree: every struct boxed, the
/// child-pointer arrays, and the strings the structs point at. Every
/// buffer lives on the heap, so pointers into them stay put when the tree
/// moves.
struct SchemaTree {
    /// Boxed for the same reason as `BatchKeep::root`: the schema's
    /// pointer arrays must not move when this Vec grows.
    #[allow(clippy::vec_box, reason = "pointer stability for the C tree")]
    boxes: Vec<Box<ArrowSchema>>,
    child_ptrs: Vec<Vec<*mut ArrowSchema>>,
    strings: Vec<CString>,
    /// The metadata blobs the structs point at, copied byte for byte.
    blobs: Vec<Vec<u8>>,
    /// The tree's root struct, the last node built; every build path ends
    /// with the root.
    root: *mut ArrowSchema,
}

impl Default for SchemaTree {
    fn default() -> Self {
        Self {
            boxes: Vec::new(),
            child_ptrs: Vec::new(),
            strings: Vec::new(),
            blobs: Vec::new(),
            root: std::ptr::null_mut(),
        }
    }
}

impl SchemaTree {
    /// Copy one C string into the tree; a null stays null.
    fn text(&mut self, value: *const c_char) -> *const c_char {
        if value.is_null() {
            return std::ptr::null();
        }
        let owned = CString::new(unsafe { CStr::from_ptr(value) }.to_bytes())
            .unwrap_or_default();
        let pointer = owned.as_ptr();
        self.strings.push(owned);
        pointer
    }

    /// Copy one metadata blob into the tree; a null stays null.
    ///
    /// The C data interface lays metadata out as an i32 pair count, then
    /// each key and value as an i32 length and its bytes. Field metadata
    /// carries a column's extension type (a Polars `Enum`, a timezone
    /// rule), so the output keeps it. A producer's blob is walked against
    /// `memory`: each length word is checked readable before it is read,
    /// the blob is capped at `MAX_METADATA`, and its whole extent is
    /// checked before the copy. A blob that fails is refused as usage.
    /// `None` walks a blob this surface built itself.
    fn metadata(&mut self, value: *const c_char, memory: Option<&Readable>) -> PyResult<*const c_char> {
        if value.is_null() {
            return Ok(std::ptr::null());
        }
        let at = value as *const u8;
        let readable = |from: usize, length: usize| {
            memory.is_none_or(|memory| memory.covers(at.wrapping_add(from), length))
        };
        let refused = || UsageError::new_err(BAD_METADATA);
        // SAFETY: each word is read only after its four bytes check readable.
        let length = |place: usize| -> PyResult<usize> {
            if !readable(place, 4) {
                return Err(refused());
            }
            usize::try_from(i32::from_le_bytes(unsafe { word(at.add(place)) })).map_err(|_| refused())
        };
        let pairs = length(0)?;
        if pairs > MAX_METADATA / 8 {
            return Err(refused());
        }
        let mut end = 4usize;
        for _ in 0..pairs * 2 {
            end = end + 4 + length(end)?;
            if end > MAX_METADATA {
                return Err(refused());
            }
        }
        if !readable(0, end) {
            return Err(refused());
        }
        // SAFETY: `end` is the capped extent the blob's own lengths declare,
        // checked readable above.
        let blob = unsafe { std::slice::from_raw_parts(at, end) }.to_vec();
        let pointer = blob.as_ptr() as *const c_char;
        self.blobs.push(blob);
        Ok(pointer)
    }

    /// One node over already-owned pieces; the child array is kept here so
    /// its pointer stays valid.
    fn node(
        &mut self,
        format: *const c_char,
        name: *const c_char,
        metadata: *const c_char,
        flags: i64,
        children: Vec<*mut ArrowSchema>,
        dictionary: *mut ArrowSchema,
    ) -> *mut ArrowSchema {
        let count = children.len();
        let child_ptrs = if children.is_empty() {
            std::ptr::null_mut()
        } else {
            self.child_ptrs.push(children);
            self.child_ptrs.last_mut().expect("just pushed").as_mut_ptr()
        };
        self.boxes.push(Box::new(ArrowSchema {
            format,
            name,
            metadata,
            flags,
            n_children: count as i64,
            children: child_ptrs,
            dictionary,
            release: Some(schema_tree_release),
            private_data: std::ptr::null_mut(),
        }));
        let pointer = self.boxes.last_mut().expect("just pushed").as_mut() as *mut ArrowSchema;
        self.root = pointer;
        pointer
    }

    /// A leaf: one name and one format, no children.
    fn leaf(&mut self, name: Option<&str>, format: &str, flags: i64) -> PyResult<*mut ArrowSchema> {
        let name = match name {
            Some(text) => Some(
                CString::new(text)
                    .map_err(|_| UsageError::new_err("a column name holds a NUL byte"))?,
            ),
            None => None,
        };
        let format =
            CString::new(format).map_err(|_| UsageError::new_err("a column format holds a NUL"))?;
        let name_ptr = match &name {
            Some(text) => text.as_ptr(),
            None => std::ptr::null(),
        };
        let format_ptr = format.as_ptr();
        if let Some(text) = name {
            self.strings.push(text);
        }
        self.strings.push(format);
        Ok(self.node(format_ptr, name_ptr, std::ptr::null(), flags, Vec::new(), std::ptr::null_mut()))
    }

    /// The struct node every frame hands out: the `+s` format, no name,
    /// and the caller's own schema metadata when it has some.
    fn branch(
        &mut self,
        children: Vec<*mut ArrowSchema>,
        metadata: *const c_char,
        memory: &Readable,
    ) -> PyResult<*mut ArrowSchema> {
        let metadata = self.metadata(metadata, Some(memory))?;
        let format =
            CString::new("+s").map_err(|_| UsageError::new_err("a column format holds a NUL"))?;
        let format_ptr = format.as_ptr();
        self.strings.push(format);
        Ok(self.node(format_ptr, std::ptr::null(), metadata, 0, children, std::ptr::null_mut()))
    }

    /// Deep-copy one schema struct and its whole tree: names, formats,
    /// children, and a dictionary when one is attached. Every node is this
    /// tree's own, so the consumer's release frees exactly what it was
    /// handed.
    ///
    /// `memory` checks a producer's metadata; `None` copies this surface's
    /// own template.
    ///
    /// # Safety
    /// `source` must point at a live schema struct.
    unsafe fn copy(&mut self, source: *const ArrowSchema, memory: Option<&Readable>) -> PyResult<*mut ArrowSchema> {
        let format = self.text(unsafe { (*source).format });
        let name = self.text(unsafe { (*source).name });
        let metadata = self.metadata(unsafe { (*source).metadata }, memory)?;
        let count = unsafe { (*source).n_children }.max(0) as usize;
        let mut children = Vec::with_capacity(count);
        for place in 0..count {
            let child = unsafe { *(*source).children.add(place) };
            children.push(unsafe { self.copy(child, memory) }?);
        }
        let dictionary = unsafe { (*source).dictionary };
        let dictionary = if dictionary.is_null() {
            std::ptr::null_mut()
        } else {
            unsafe { self.copy(dictionary, memory) }?
        };
        let flags = unsafe { (*source).flags };
        Ok(self.node(format, name, metadata, flags, children, dictionary))
    }
}

/// The schema release: free the tree the consumer was given, and clear the
/// release pointer the C data interface requires to be cleared.
unsafe extern "C" fn schema_tree_release(root: *mut ArrowSchema) {
    (*root).release = None;
    let keep = (*root).private_data as *mut SchemaTree;
    if !keep.is_null() {
        (*root).private_data = std::ptr::null_mut();
        drop(Box::from_raw(keep));
    }
}

/// The whole frame behind the output stream: the schema template to deep
/// copy from, the batches not yet handed out, and the error phrase.
pub(crate) struct OutFrame {
    /// The output schema template: the caller's columns' whole trees,
    /// deep-copied, and the new columns' leaves. `hand_schema` clones it
    /// for every take, starting from its root.
    schema: SchemaTree,
    batches: Vec<Option<BatchKeep>>,
    emitted: usize,
    error: Option<CString>,
}

/// Bit-pack a validity bitmap and a value bitmap for `col` rows starting
/// at `base`, LSB first per the Arrow spec.
fn bitmaps(flags: &[bool], nulls: &[bool], base: usize, count: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; count.div_ceil(8)];
    for place in 0..count {
        let flag = flags[base + place];
        if flag && !nulls[base + place] {
            bytes[place / 8] |= 1 << (place % 8);
        }
    }
    bytes
}

fn nulls_of(flags: &[bool], nulls: &[bool], base: usize, count: usize) -> i64 {
    (0..count).filter(|place| !flags[base + place] || nulls[base + place]).count() as i64
}

/// Build the whole output frame: the caller's original columns aliased
/// from the holds, then one column per question in the set's name order.
pub(crate) fn build_frame(
    frame: FrameColumn,
    names: &[String],
    rows: &[Vec<(String, Annotated)>],
) -> PyResult<OutFrame> {
    if rows.len() != frame.texts.len() {
        return Err(UsageError::new_err(
            "the engine answered a different record count than the frame carried",
        ));
    }
    let originals = frame.names.len();
    let total = originals + names.len();
    let mut formats_for_new: Vec<&'static str> = Vec::with_capacity(names.len());
    for name in names {
        // A failed member widens the question's whole column to text: a
        // marker is neither a bool nor a number, so the string layout
        // carries the good answers and the ruled marker beside them, and
        // no failed cell can read as `false` or `null`.
        let any_failed = rows.iter().any(|row| {
            row.iter()
                .any(|(held, field)| held == name && matches!(field, Annotated::Failed(_)))
        });
        let format = if any_failed {
            "u"
        } else {
            match rows.first().and_then(|row| {
                row.iter()
                    .find(|(held, _)| held == name)
                    .map(|(_, field)| field)
            }) {
                Some(Annotated::Decision(_)) => "b",
                Some(Annotated::Choice(_)) => "u",
                Some(Annotated::Score(_)) => "g",
                Some(Annotated::Tags(_)) => "u",
                Some(Annotated::Failed(_)) => "u",
                None => "u",
            }
        };
        formats_for_new.push(format);
    }
    // The Arc holds the caller's frame so aliased columns outlive the
    // call; it is deliberately not Send+Sync because the frame carries
    // raw Arrow pointers single-threaded by contract.
    #[allow(clippy::arc_with_non_send_sync, reason = "raw Arrow pointers, single-threaded by contract")]
    let hold = std::sync::Arc::new(frame);
    // The output schema: the caller's columns keep their own whole schema
    // trees — a dictionary, a struct's fields, a list's element — deep
    // copied so the output describes exactly the aliased arrays, and the
    // new columns get one leaf each.
    let mut schema = SchemaTree::default();
    let mut schema_children: Vec<*mut ArrowSchema> = Vec::with_capacity(total);
    let memory = Readable::snapshot()?;
    for place in 0..originals {
        let source = unsafe { *hold.schema.children.add(place) };
        schema_children.push(unsafe { schema.copy(source, Some(&memory)) }?);
    }
    for (place, name) in names.iter().enumerate() {
        schema_children.push(schema.leaf(Some(name), formats_for_new[place], 2)?);
    }
    let schema_root = schema.branch(schema_children, hold.schema.metadata, &memory)?;
    let _ = schema_root;
    // One batch out per batch in, so aliased originals stay per-batch and
    // every batch keeps its own root.
    let mut batches: Vec<Option<BatchKeep>> = Vec::with_capacity(hold.hold.len().max(1));
    let mut base = 0usize;
    for (batch_place, length) in hold.lengths.iter().copied().enumerate() {
        let mut children: Vec<Box<ArrowArray>> = Vec::with_capacity(total);
        let mut buffers: Vec<Vec<*const c_void>> = Vec::with_capacity(total);
        let mut owned: Vec<Vec<u8>> = Vec::new();
        // Originals, aliased from the hold: the whole struct comes across,
        // including a dictionary's values and a nested column's children,
        // because those pointers are part of the column's shape.
        for place in 0..originals {
            let held = &hold.hold[batch_place];
            let child = unsafe { held.child(place) };
            let buffers_ptr = child.buffers;
            buffers.push(Vec::new());
            children.push(Box::new(ArrowArray {
                // The caller's own rows, cut to this batch's length: a
                // sliced struct may keep its slice on the root with whole
                // children, so the root's offset is added here and the
                // output's own root carries none.
                length: length as i64,
                null_count: child.null_count,
                offset: held.root.offset + child.offset,
                n_buffers: child.n_buffers,
                n_children: child.n_children,
                buffers: buffers_ptr,
                children: child.children,
                dictionary: child.dictionary,
                release: None,
                private_data: std::ptr::null_mut(),
            }));
        }
        // New columns, computed.
        for (question, format) in formats_for_new.iter().enumerate() {
            let mut flags = Vec::with_capacity(length);
            let mut nulls = Vec::with_capacity(length);
            let mut values_bool = Vec::with_capacity(length);
            let mut values_num = Vec::with_capacity(length);
            let mut values_str: Vec<Option<String>> = Vec::with_capacity(length);
            for place in 0..length {
                let row = &rows[base + place];
                let field = row.iter().find(|(held, _)| held == &names[question]);
                match field.map(|(_, field)| field) {
                    Some(Annotated::Decision(answer)) => {
                        let value = answer.value();
                        flags.push(value.is_some());
                        nulls.push(false);
                        values_bool.push(value.unwrap_or(false));
                        values_num.push(0.0);
                        // Only the widened text layout reads this; the
                        // bool layout keeps its bits.
                        values_str.push(match (*format, value) {
                            ("u", Some(held)) => Some(held.to_string()),
                            _ => None,
                        });
                    }
                    Some(Annotated::Choice(picked)) => {
                        flags.push(picked.is_some());
                        nulls.push(false);
                        values_bool.push(false);
                        values_num.push(0.0);
                        values_str.push(picked.clone());
                    }
                    Some(Annotated::Score(scored)) => {
                        flags.push(true);
                        nulls.push(false);
                        values_bool.push(false);
                        values_num.push(scored.value);
                        values_str.push(if *format == "u" {
                            Some(scored.value.to_string())
                        } else {
                            None
                        });
                    }
                    Some(Annotated::Tags(tags)) => {
                        flags.push(true);
                        nulls.push(false);
                        values_bool.push(false);
                        values_num.push(0.0);
                        values_str.push(Some(
                            serde_json::to_string(tags).unwrap_or_else(|_| "[]".into()),
                        ));
                    }
                    Some(Annotated::Failed(failed)) => {
                        // The ruled marker as its JSON text (0054): never
                        // `false`, never a `null`.
                        flags.push(true);
                        nulls.push(false);
                        values_bool.push(false);
                        values_num.push(0.0);
                        values_str.push(Some(
                            serde_json::to_string(&Annotated::Failed(*failed))
                                .expect("a failed marker is JSON-clean"),
                        ));
                    }
                    None => {
                        flags.push(false);
                        nulls.push(true);
                        values_bool.push(false);
                        values_num.push(0.0);
                        values_str.push(None);
                    }
                }
            }
            let null_count = nulls_of(&flags, &nulls, 0, length);
            let validity = bitmaps(&flags, &nulls, 0, length);
            let mut local: Vec<Vec<u8>> = Vec::new();
            let n_buffers = match *format {
                "b" => {
                    let mut values = vec![0u8; length.div_ceil(8)];
                    for (place, value) in values_bool.iter().enumerate() {
                        if *value {
                            values[place / 8] |= 1 << (place % 8);
                        }
                    }
                    local.push(validity);
                    local.push(values);
                    2i64
                }
                "g" => {
                    let mut values = Vec::with_capacity(length * 8);
                    for value in &values_num {
                        values.extend_from_slice(&value.to_le_bytes());
                    }
                    local.push(validity);
                    local.push(values);
                    2i64
                }
                _ => {
                    let (offsets, values) = utf8_buffers(
                        values_str.iter().map(|text| text.as_deref().unwrap_or("")),
                        length,
                    )?;
                    local.push(validity);
                    local.push(offsets);
                    local.push(values);
                    3i64
                }
            };
            let pointers: Vec<*const c_void> = local
                .iter()
                .enumerate()
                .map(|(place, bytes)| {
                    if place == 0 && null_count == 0 {
                        std::ptr::null()
                    } else {
                        bytes.as_ptr() as *const c_void
                    }
                })
                .collect();
            owned.extend(local);
            buffers.push(pointers);
            children.push(Box::new(ArrowArray {
                length: length as i64,
                null_count,
                offset: 0,
                n_buffers,
                n_children: 0,
                buffers: std::ptr::null_mut(),
                children: std::ptr::null_mut(),
                dictionary: std::ptr::null_mut(),
                release: None,
                private_data: std::ptr::null_mut(),
            }));
        }
        let child_ptrs: Vec<*mut ArrowArray> = children
            .iter_mut()
            .map(|child| child.as_mut() as *mut ArrowArray)
            .collect();
        // Wire the new columns' buffers after every Vec has its final
        // shape; the originals keep the producer's aliased pointer.
        for (place, pointers) in buffers.iter().enumerate().skip(originals) {
            children[place].buffers = pointers.as_ptr() as *mut *const c_void;
        }
        let root_buffers: Vec<*const c_void> = vec![std::ptr::null()];
        let root = Box::new(ArrowArray {
            length: length as i64,
            null_count: 0,
            offset: 0,
            n_buffers: 1,
            n_children: total as i64,
            buffers: root_buffers.as_ptr() as *mut *const c_void,
            children: child_ptrs.as_ptr() as *mut *mut ArrowArray,
            dictionary: std::ptr::null_mut(),
            release: None,
            private_data: std::ptr::null_mut(),
        });
        batches.push(Some(BatchKeep {
            root,
            root_buffers,
            children,
            child_ptrs,
            buffers,
            owned,
            hold: Some(std::sync::Arc::clone(&hold)),
        }));
        base += length;
    }

    Ok(OutFrame {
        schema,
        batches,
        emitted: 0,
        error: None,
    })
}

/// The offsets and values buffers of one `u` column this surface builds.
///
/// The `u` layout's offsets are i32, so a column whose text passes 2 GiB
/// cannot be described by it; that column is refused instead of handed
/// out with offsets that wrapped negative.
fn utf8_buffers<'a>(
    texts: impl Iterator<Item = &'a str>,
    length: usize,
) -> PyResult<(Vec<u8>, Vec<u8>)> {
    let mut offsets = Vec::with_capacity((length + 1) * 4);
    let mut values = Vec::new();
    offsets.extend_from_slice(&0i32.to_le_bytes());
    for text in texts {
        values.extend_from_slice(text.as_bytes());
        offsets.extend_from_slice(&utf8_offset(values.len())?.to_le_bytes());
    }
    Ok((offsets, values))
}

/// One `u` offset, refused when the text before it passes i32.
fn utf8_offset(end: usize) -> PyResult<i32> {
    i32::try_from(end).map_err(|_| {
        UsageError::new_err("an answer column's text passes the 2 GiB a text column's offsets can name")
    })
}

/// One column of a table this surface builds whole: the ruled field
/// names over owned buffers, because `recognize` and `relate` produce
/// results with no fixed size and no caller frame to alias.
pub(crate) enum TableValue {
    /// A text column, the `u` layout: offsets over one UTF-8 buffer.
    Texts(Vec<String>),
    /// A whole-number column, the `l` layout.
    Counts(Vec<i64>),
    /// A number column, the `g` layout.
    Numbers(Vec<f64>),
}

impl TableValue {
    fn len(&self) -> usize {
        match self {
            Self::Texts(values) => values.len(),
            Self::Counts(values) => values.len(),
            Self::Numbers(values) => values.len(),
        }
    }

    fn format(&self) -> &'static str {
        match self {
            Self::Texts(_) => "u",
            Self::Counts(_) => "l",
            Self::Numbers(_) => "g",
        }
    }
}

/// Build a whole frame from owned columns: the long frame `recognize`
/// returns over a frame, and the edge frame `relate` returns. One batch,
/// every buffer this surface's own, behind the same stream form Polars
/// consumes through `__arrow_c_stream__`.
pub(crate) fn build_table(columns: &[(&str, TableValue)]) -> PyResult<OutFrame> {
    let length = columns.first().map_or(0, |(_, value)| value.len());
    for (name, value) in columns {
        if value.len() != length {
            return Err(UsageError::new_err(format!(
                "the column {name} carries {} rows beside a table of {length}",
                value.len()
            )));
        }
    }
    let total = columns.len();
    let mut schema = SchemaTree::default();
    let mut schema_children: Vec<*mut ArrowSchema> = Vec::with_capacity(total);
    for (name, value) in columns {
        schema_children.push(schema.leaf(Some(name), value.format(), 2)?);
    }
    let schema_root = schema.branch(schema_children, std::ptr::null(), &Readable::snapshot()?)?;
    let _ = schema_root;

    let mut children: Vec<Box<ArrowArray>> = Vec::with_capacity(total);
    let mut buffers: Vec<Vec<*const c_void>> = Vec::with_capacity(total);
    let mut owned: Vec<Vec<u8>> = Vec::new();
    for (_, value) in columns {
        // Every value here is present, so the validity bitmap is all
        // ones and the null count is zero; the array still carries the
        // bitmap, the shape every Arrow reader accepts.
        let mut validity = vec![0u8; length.div_ceil(8)];
        for place in 0..length {
            validity[place / 8] |= 1 << (place % 8);
        }
        let mut local: Vec<Vec<u8>> = Vec::new();
        let n_buffers = match value {
            TableValue::Texts(texts) => {
                let (offsets, values) = utf8_buffers(texts.iter().map(String::as_str), length)?;
                local.push(validity);
                local.push(offsets);
                local.push(values);
                3i64
            }
            TableValue::Counts(counts) => {
                let mut values = Vec::with_capacity(length * 8);
                for count in counts {
                    values.extend_from_slice(&count.to_le_bytes());
                }
                local.push(validity);
                local.push(values);
                2i64
            }
            TableValue::Numbers(numbers) => {
                let mut values = Vec::with_capacity(length * 8);
                for number in numbers {
                    values.extend_from_slice(&number.to_le_bytes());
                }
                local.push(validity);
                local.push(values);
                2i64
            }
        };
        let pointers: Vec<*const c_void> = local
            .iter()
            .enumerate()
            .map(|(place, bytes)| {
                if place == 0 {
                    std::ptr::null()
                } else {
                    bytes.as_ptr() as *const c_void
                }
            })
            .collect();
        owned.extend(local);
        buffers.push(pointers);
        children.push(Box::new(ArrowArray {
            length: length as i64,
            null_count: 0,
            offset: 0,
            n_buffers,
            n_children: 0,
            buffers: std::ptr::null_mut(),
            children: std::ptr::null_mut(),
            dictionary: std::ptr::null_mut(),
            release: None,
            private_data: std::ptr::null_mut(),
        }));
    }
    let child_ptrs: Vec<*mut ArrowArray> = children
        .iter_mut()
        .map(|child| child.as_mut() as *mut ArrowArray)
        .collect();
    for (place, pointers) in buffers.iter().enumerate() {
        children[place].buffers = pointers.as_ptr() as *mut *const c_void;
    }
    let root_buffers: Vec<*const c_void> = vec![std::ptr::null()];
    let root = Box::new(ArrowArray {
        length: length as i64,
        null_count: 0,
        offset: 0,
        n_buffers: 1,
        n_children: total as i64,
        buffers: root_buffers.as_ptr() as *mut *const c_void,
        children: child_ptrs.as_ptr() as *mut *mut ArrowArray,
        dictionary: std::ptr::null_mut(),
        release: None,
        private_data: std::ptr::null_mut(),
    });
    let batches = vec![Some(BatchKeep {
        root,
        root_buffers,
        children,
        child_ptrs,
        buffers,
        owned,
        hold: None,
    })];
    Ok(OutFrame {
        schema,
        batches,
        emitted: 0,
        error: None,
    })
}

/// Deep-copy the schema template into a fresh tree the consumer owns and
/// releases through `schema_tree_release`. The template's blobs are this
/// surface's own and passed their checks when it was built, so the copy
/// cannot fail; a failure still answers the interface's EINVAL.
unsafe fn hand_schema(state: &OutFrame, out: *mut ArrowSchema) -> c_int {
    const EINVAL: c_int = 22;
    let mut tree = SchemaTree::default();
    let Ok(root) = (unsafe { tree.copy(state.schema.root, None) }) else {
        return EINVAL;
    };
    let keep = Box::into_raw(Box::new(tree));
    *out = std::ptr::read(root);
    (*out).private_data = keep as *mut c_void;
    (*out).release = Some(schema_tree_release);
    0
}

unsafe extern "C" fn frame_get_schema(
    stream: *mut ArrowArrayStream,
    out: *mut ArrowSchema,
) -> c_int {
    let state = &mut *((*stream).private_data as *mut OutFrame);
    hand_schema(state, out)
}

unsafe extern "C" fn frame_get_next(stream: *mut ArrowArrayStream, out: *mut ArrowArray) -> c_int {
    let state = &mut *((*stream).private_data as *mut OutFrame);
    if state.emitted < state.batches.len() {
        let keep = state.batches[state.emitted].take();
        state.emitted += 1;
        if let Some(keep) = keep {
            emit_batch(keep, out);
            return 0;
        }
    }
    *out = std::mem::zeroed();
    0
}

unsafe extern "C" fn frame_last_error(stream: *mut ArrowArrayStream) -> *const c_char {
    let state = &mut *((*stream).private_data as *mut OutFrame);
    match &state.error {
        Some(text) => text.as_ptr(),
        None => std::ptr::null(),
    }
}

unsafe extern "C" fn frame_release(stream: *mut ArrowArrayStream) {
    let data = (*stream).private_data;
    (*stream).release = None;
    if !data.is_null() {
        (*stream).private_data = std::ptr::null_mut();
        drop(Box::from_raw(data as *mut OutFrame));
    }
}

/// The capsule's destructor: release the stream once, then free its box.
unsafe extern "C" fn frame_capsule_destructor(capsule: *mut pyo3::ffi::PyObject) {
    let pointer = pyo3::ffi::PyCapsule_GetPointer(capsule, STREAM_NAME.as_ptr());
    if !pointer.is_null() {
        let stream = pointer as *mut ArrowArrayStream;
        if let Some(release) = (*stream).release {
            release(stream);
        }
        drop(Box::from_raw(stream));
    }
}

/// The built frame, handed to the host. One stream is taken per value.
#[pyclass(unsendable)]
pub(crate) struct ArrowFrame {
    state: Option<OutFrame>,
}

impl ArrowFrame {
    pub(crate) fn new(state: OutFrame) -> Self {
        Self { state: Some(state) }
    }
}

#[pymethods]
impl ArrowFrame {
    /// Mint a fresh Arrow C stream over the built frame. Polars takes one
    /// per `type(records)(frame)` construction.
    #[pyo3(signature = (requested_schema = None))]
    fn __arrow_c_stream__(
        &mut self,
        py: Python<'_>,
        requested_schema: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Py<PyAny>> {
        let _ = requested_schema;
        let state = self.state.take().ok_or_else(|| {
            UsageError::new_err("the frame's Arrow stream was already taken")
        })?;
        let stream = Box::into_raw(Box::new(ArrowArrayStream {
            get_schema: Some(frame_get_schema),
            get_next: Some(frame_get_next),
            get_last_error: Some(frame_last_error),
            release: Some(frame_release),
            private_data: Box::into_raw(Box::new(state)) as *mut c_void,
        }));
        let capsule = unsafe {
            PyCapsule::new_with_pointer_and_destructor(
                py,
                NonNull::new_unchecked(stream as *mut c_void),
                STREAM_NAME,
                Some(frame_capsule_destructor),
            )
        }?;
        Ok(capsule.into_any().unbind())
    }
}

/// The addresses of the values and offsets buffers of a series-shaped
/// stream's first batch, for the zero-copy proof.
#[pyfunction]
pub(crate) fn _arrow_probe(records: &Bound<'_, PyAny>) -> PyResult<(usize, usize, usize)> {
    let column = series_column(records)?;
    let Some(batch) = column.batches.first() else {
        return Err(UsageError::new_err("the probe found no batch"));
    };
    unsafe {
        let buffers = batch.buffers;
        Ok((
            buffers.offset(2).read() as usize,
            buffers.offset(1).read() as usize,
            usize::try_from(batch.length).unwrap_or(0),
        ))
    }
}

/// Probe whether a host can rebuild its own frame from the stream form the
/// frame doors return, before any request runs.
///
/// An empty frame of the same shape (a struct stream) is handed to the
/// host's own constructor. A host whose constructor cannot consume the
/// stream form — pandas' `DataFrame`, pyarrow's `Table` — raises here, so
/// the caller refuses before the batch runs and before the host's own
/// stream is ever taken. The refusal keeps the constructor's error as the
/// cause, so the wrapper can chain the host's own words.
#[pyfunction]
pub(crate) fn _probe_frame_rebuild(host: &Bound<'_, PyAny>) -> PyResult<()> {
    let probe = build_table(&[("probe", TableValue::Texts(Vec::new()))])?;
    let frame = ArrowFrame::new(probe);
    host.get_type().call1((frame,)).map(|_| ())
}

/// The capsule names the Arrow PyCapsule interface fixes for a single
/// array: a schema capsule and an array capsule, handed out as one pair.
const ARRAY_SCHEMA_NAME: &CStr = c"arrow_schema";
const ARRAY_NAME: &CStr = c"arrow_array";

/// The format and name strings a column's schema keeps alive.
struct ColumnStrings {
    format: CString,
    name: CString,
}

/// One column of answers on the way out: a boolean column with `None`
/// rows ("not sure"), or a number column, both this surface's own
/// buffers because the answers are new values.
struct SeriesOut {
    name: String,
    format: &'static str,
    length: usize,
    null_count: i64,
    validity: Vec<u8>,
    values: Vec<u8>,
}

/// The buffers and the pointer table a column's array keeps alive: the
/// `buffers` vector itself must not move while the consumer reads it.
struct ColumnKeep {
    state: SeriesOut,
    buffers: Vec<*const c_void>,
}

/// A column of answers, handed to the host as one Arrow array.
///
/// The bulk verbs over a Polars column produce new values, so the buffers
/// are this surface's own, minted once as one schema capsule and one
/// array capsule through `__arrow_c_array__`; `type(series)(column)`
/// rebuilds the host's own column with no import of the host library.
#[pyclass(unsendable)]
pub(crate) struct ArrowSeries {
    state: Option<SeriesOut>,
}

impl ArrowSeries {
    /// A boolean column: `None` is "not sure", one null bit a row.
    pub(crate) fn bools(name: &str, values: &[Option<bool>]) -> Self {
        let mut validity = vec![0u8; values.len().div_ceil(8)];
        let mut bits = vec![0u8; values.len().div_ceil(8)];
        let mut nulls = 0i64;
        for (place, value) in values.iter().enumerate() {
            match value {
                Some(true) => {
                    validity[place / 8] |= 1 << (place % 8);
                    bits[place / 8] |= 1 << (place % 8);
                }
                Some(false) => validity[place / 8] |= 1 << (place % 8),
                None => nulls += 1,
            }
        }
        let validity = if nulls == 0 { Vec::new() } else { validity };
        Self {
            state: Some(SeriesOut {
                name: name.to_owned(),
                format: "b",
                length: values.len(),
                null_count: nulls,
                validity,
                values: bits,
            }),
        }
    }

    /// A number column, every cell present.
    pub(crate) fn numbers(name: &str, values: &[f64]) -> Self {
        let mut bytes = Vec::with_capacity(values.len() * 8);
        for value in values {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        Self {
            state: Some(SeriesOut {
                name: name.to_owned(),
                format: "g",
                length: values.len(),
                null_count: 0,
                validity: Vec::new(),
                values: bytes,
            }),
        }
    }
}

#[pymethods]
impl ArrowSeries {
    /// The answers as a plain Python list, read without taking the Arrow
    /// capsules: booleans with `None` for "not sure", or numbers. The
    /// wrapper uses this for a host whose constructors do not consume the
    /// array capsule (a pandas Series), so the caller still gets one
    /// answer a row as the plain list the list door returns.
    fn to_list(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let state = self.state.as_ref().ok_or_else(|| {
            UsageError::new_err("the column's Arrow array was already taken")
        })?;
        let list = pyo3::types::PyList::empty(py);
        match state.format {
            "b" => {
                for place in 0..state.length {
                    let present = state.null_count == 0
                        || state.validity[place / 8] & (1 << (place % 8)) != 0;
                    if !present {
                        list.append(py.None())?;
                    } else {
                        let set = state.values[place / 8] & (1 << (place % 8)) != 0;
                        list.append(pyo3::types::PyBool::new(py, set))?;
                    }
                }
            }
            _ => {
                for place in 0..state.length {
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(&state.values[place * 8..place * 8 + 8]);
                    list.append(pyo3::types::PyFloat::new(py, f64::from_le_bytes(bytes)))?;
                }
            }
        }
        Ok(list.into_any().unbind())
    }

    /// Mint the schema and array capsules the Arrow PyCapsule interface
    /// names; the host takes one pair per column construction.
    #[pyo3(signature = (requested_schema = None))]
    fn __arrow_c_array__(
        &mut self,
        py: Python<'_>,
        requested_schema: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
        let _ = requested_schema;
        let state = self.state.take().ok_or_else(|| {
            UsageError::new_err("the column's Arrow array was already taken")
        })?;
        // The schema's format and name strings live in one keep box the
        // schema's release frees.
        let strings = Box::into_raw(Box::new(ColumnStrings {
            format: CString::new(state.format).expect("static format"),
            name: CString::new(state.name.as_str()).unwrap_or_default(),
        }));
        let schema = Box::into_raw(Box::new(ArrowSchema {
            format: unsafe { (*strings).format.as_ptr() },
            name: unsafe { (*strings).name.as_ptr() },
            metadata: std::ptr::null(),
            flags: 2,
            n_children: 0,
            children: std::ptr::null_mut(),
            dictionary: std::ptr::null_mut(),
            release: Some(column_schema_release),
            private_data: strings as *mut c_void,
        }));
        // The buffers live in the array's keep; the pointer table points
        // into that box, so it is built after the box is on the heap.
        let mut keep = Box::new(ColumnKeep { state, buffers: Vec::new() });
        let validity = if keep.state.null_count == 0 {
            std::ptr::null()
        } else {
            keep.state.validity.as_ptr() as *const c_void
        };
        let values = keep.state.values.as_ptr() as *const c_void;
        keep.buffers = vec![validity, values];
        let keep_ptr = Box::into_raw(keep);
        let array = Box::into_raw(Box::new(ArrowArray {
            length: unsafe { (*keep_ptr).state.length } as i64,
            null_count: unsafe { (*keep_ptr).state.null_count },
            offset: 0,
            n_buffers: 2,
            n_children: 0,
            buffers: unsafe { (*keep_ptr).buffers.as_ptr() as *mut *const c_void },
            children: std::ptr::null_mut(),
            dictionary: std::ptr::null_mut(),
            release: Some(column_array_release),
            private_data: keep_ptr as *mut c_void,
        }));
        let schema_capsule = unsafe {
            PyCapsule::new_with_pointer_and_destructor(
                py,
                NonNull::new_unchecked(schema as *mut c_void),
                ARRAY_SCHEMA_NAME,
                Some(column_schema_capsule_destructor),
            )
        }?;
        let array_capsule = unsafe {
            PyCapsule::new_with_pointer_and_destructor(
                py,
                NonNull::new_unchecked(array as *mut c_void),
                ARRAY_NAME,
                Some(column_array_capsule_destructor),
            )
        }?;
        Ok((schema_capsule.into_any().unbind(), array_capsule.into_any().unbind()))
    }
}

unsafe extern "C" fn column_schema_release(schema: *mut ArrowSchema) {
    (*schema).release = None;
    let data = (*schema).private_data;
    if !data.is_null() {
        (*schema).private_data = std::ptr::null_mut();
        drop(Box::from_raw(data as *mut ColumnStrings));
    }
}

unsafe extern "C" fn column_array_release(array: *mut ArrowArray) {
    (*array).release = None;
    let data = (*array).private_data;
    if !data.is_null() {
        (*array).private_data = std::ptr::null_mut();
        drop(Box::from_raw(data as *mut ColumnKeep));
    }
}

unsafe extern "C" fn column_schema_capsule_destructor(capsule: *mut pyo3::ffi::PyObject) {
    let pointer = pyo3::ffi::PyCapsule_GetPointer(capsule, ARRAY_SCHEMA_NAME.as_ptr());
    if !pointer.is_null() {
        let schema = pointer as *mut ArrowSchema;
        if let Some(release) = (*schema).release {
            release(schema);
        }
        drop(Box::from_raw(schema));
    }
}

unsafe extern "C" fn column_array_capsule_destructor(capsule: *mut pyo3::ffi::PyObject) {
    let pointer = pyo3::ffi::PyCapsule_GetPointer(capsule, ARRAY_NAME.as_ptr());
    if !pointer.is_null() {
        let array = pointer as *mut ArrowArray;
        if let Some(release) = (*array).release {
            release(array);
        }
        drop(Box::from_raw(array));
    }
}

#[cfg(test)]
mod malformed_tests {
    //! The reviews' exit-139 probes, pinned as tests: a malformed string
    //! view or offsets buffer must come back as a refusal, never as a read
    //! past a buffer. The arrays are hand-built C structs over real, small
    //! buffers in the layouts the C data interface names, so every
    //! refusal below is a claim the reader could check before reading.

    use super::*;

    fn array_of(held: &[*const c_void], length: i64) -> ArrowArray {
        // SAFETY: an all-zero ArrowArray is the C interface's empty
        // producer state; the fields set below are the ones read.
        let mut outer = unsafe { std::mem::zeroed::<ArrowArray>() };
        outer.length = length;
        outer.n_buffers = held.len() as i64;
        outer.buffers = held.as_ptr() as *mut *const c_void;
        outer
    }

    fn pointers(buffers: &[&[u8]]) -> Vec<*const c_void> {
        buffers.iter().map(|one| one.as_ptr().cast()).collect()
    }

    fn one_view(size: u32, index: u32, offset: u32) -> Vec<u8> {
        let mut one = [0u8; 16];
        one[0..4].copy_from_slice(&size.to_le_bytes());
        one[8..12].copy_from_slice(&index.to_le_bytes());
        one[12..16].copy_from_slice(&offset.to_le_bytes());
        one.to_vec()
    }

    /// The trailing buffer the interface appends to a view array: one
    /// i64 length a data buffer.
    /// Borrow one view column laid out as `[validity, views, data.., sizes]`,
    /// each data buffer's size declared as its real length.
    fn views_over(views: &[u8], data: &[&[u8]], rows: usize) -> PyResult<Vec<&'static str>> {
        let sizes: Vec<i64> = data.iter().map(|one| one.len() as i64).collect();
        views_raw(views, data, &sizes, rows)
    }

    fn utf8_over(offsets: &[i32], values: &[u8]) -> PyResult<Vec<&'static str>> {
        let offsets: Vec<u8> = offsets.iter().flat_map(|one| one.to_le_bytes()).collect();
        let held = pointers(&[b"".as_slice(), &offsets, values]);
        let rows = offsets.len() / 4 - 1;
        let outer = array_of(&held, rows as i64);
        unsafe { borrow_strings(&outer, Text::Utf8, 0, rows) }
    }

    fn large_over(offsets: &[i64], values: &[u8]) -> PyResult<Vec<&'static str>> {
        let bytes: Vec<u8> = offsets.iter().flat_map(|one| one.to_le_bytes()).collect();
        let held = pointers(&[b"".as_slice(), &bytes, values]);
        let rows = offsets.len() - 1;
        let outer = array_of(&held, rows as i64);
        unsafe { borrow_strings(&outer, Text::LargeUtf8, 0, rows) }
    }

    /// A view column whose sizes buffer is set by hand, so a test can
    /// declare an extent the data buffer does not carry.
    fn views_raw(views: &[u8], data: &[&[u8]], sizes: &[i64], rows: usize) -> PyResult<Vec<&'static str>> {
        let sizes_bytes: Vec<u8> = sizes.iter().flat_map(|one| one.to_le_bytes()).collect();
        let mut table = vec![b"".as_slice(), views];
        table.extend_from_slice(data);
        table.push(&sizes_bytes);
        let held = pointers(&table);
        let outer = array_of(&held, rows as i64);
        unsafe { borrow_strings(&outer, Text::View, 0, rows) }
    }

    /// The exact sentence a refused shape carries. Pinning the words, not
    /// just `is_err`, is what makes a test fail when a check is loosened
    /// and the reader refuses the shape for a different reason (or reads
    /// garbage that is then refused) instead of the reason under test.
    fn refusal<T>(outcome: PyResult<T>) -> String {
        Python::initialize();
        Python::attach(|py| {
            outcome
                .err()
                .expect("the shape must be refused, not read")
                .value(py)
                .to_string()
        })
    }

    #[test]
    fn a_view_past_its_declared_length_is_refused() {
        // Review-4's probe (offset 60,000,000) and review-5's (offset 200,
        // and a view straddling the end): a 100-byte buffer declared 100.
        let data = vec![b'x'; 100];
        for (size, offset) in [(100, 60_000_000), (13, 200), (13, 90), (0x4000_0000, 0)] {
            let views = one_view(size, 0, offset);
            assert!(views_over(&views, &[&data], 1).is_err(), "{size} at {offset}");
        }
    }

    #[test]
    fn a_view_cannot_name_the_sizes_buffer() {
        // Review-5: the data count is the table minus three, so index 1
        // over one data buffer is the sizes buffer. Review-6: the exact
        // refusal is pinned, so loosening the index check by one (`>`
        // instead of `>=`) reads the sizes buffer as data and refuses for
        // a different reason, which this test then catches.
        let data = vec![b'x'; 100];
        assert_eq!(
            refusal(views_over(&one_view(13, 1, 0), &[&data], 1)),
            "a string view points past its data buffers"
        );
        assert_eq!(
            refusal(views_over(&one_view(32, 5, 0), &[&data], 1)),
            "a string view points past its data buffers"
        );
    }

    #[test]
    fn a_utf8_row_past_the_final_offset_is_refused() {
        // Review-5: the final offset declares the data extent; a row that
        // ends past it is non-monotone. Review-6: the exact refusal is
        // pinned, so removing the order check reads `200` bytes of a
        // three-byte buffer and refuses for a different reason, which
        // this test then catches.
        assert_eq!(
            refusal(utf8_over(&[0, 200, 3], b"abc")),
            "the column's offsets are reversed or negative"
        );
        assert_eq!(
            refusal(utf8_over(&[3, 1], b"abc")),
            "the column's offsets are reversed or negative"
        );
        assert_eq!(
            refusal(utf8_over(&[-5, 3], b"abc")),
            "the column's offsets are reversed or negative"
        );
        // A row longer than the per-row ceiling is refused before the read.
        assert_eq!(
            refusal(utf8_over(&[0, 60_000_000], b"abc")),
            "a row names a size no text column carries"
        );
    }

    #[test]
    fn a_declared_data_extent_past_the_cap_is_refused_and_a_big_slice_borrows() {
        // Review-6: the per-row cap bounds a row's length, not where it
        // starts, so a view offset near 2^32, a large_utf8 offset near
        // i64::MAX, a utf8 offset near i32::MAX, and offsets at 2^40 over a
        // tiny buffer all read gigabytes past it. Bounding the declared
        // extent refuses each one before the read.
        assert_eq!(
            refusal(views_raw(&one_view(13, 0, u32::MAX), &[b"abc".as_slice()], &[i64::MAX], 1)),
            "a string view's data buffer declares more bytes than a text column carries"
        );
        assert_eq!(
            refusal(large_over(&[i64::MAX - 13, i64::MAX], b"abc")),
            "the column's offsets name more bytes than a text column carries"
        );
        assert_eq!(
            refusal(utf8_over(&[i32::MAX - 13, i32::MAX], b"abc")),
            "the column's offsets name more bytes than a text column carries"
        );
        assert_eq!(
            refusal(large_over(&[1i64 << 40, (1i64 << 40) + 13], b"abc")),
            "the column's offsets name more bytes than a text column carries"
        );
        // The trade-off, tested: a legitimate column whose first row starts
        // partway into its values buffer (a slice's own offset) still
        // borrows, as long as its declared extent stays under the cap.
        assert_eq!(utf8_over(&[5, 9], b"-----SEEN").expect("a big slice borrows"), ["SEEN"]);
        let values = vec![b'a'; 5 << 20];
        let rows: Vec<i64> = (0..=5).map(|one| one << 20).collect();
        assert_eq!(large_over(&rows, &values).expect("a 5 MiB large column borrows").len(), 5);
    }

    #[test]
    fn a_buffer_table_far_longer_than_a_text_column_is_refused() {
        // Review-6: the reader indexes the table's last slot for the sizes
        // buffer, so a count near 2^40 walks the table far past its end.
        // The count is bounded before any slot is read.
        let offsets = [0i32, 3].map(i32::to_le_bytes).concat();
        let values = b"abc".to_vec();
        let held = pointers(&[b"".as_slice(), &offsets, &values]);
        let mut outer = array_of(&held, 1);
        outer.n_buffers = 1i64 << 40;
        assert_eq!(
            refusal(unsafe { borrow_strings(&outer, Text::Utf8, 0, 1) }),
            "the column's buffer table names more buffers than a text column carries"
        );
        let views = one_view(3, 0, 0);
        let big = vec![b'x'; 100];
        let held = pointers(&[b"".as_slice(), &views, &big, &(100i64).to_le_bytes()]);
        let mut outer = array_of(&held, 1);
        outer.n_buffers = 1i64 << 40;
        assert_eq!(
            refusal(unsafe { borrow_strings(&outer, Text::View, 0, 1) }),
            "the column's buffer table names more buffers than a text column carries"
        );
    }

    #[test]
    fn a_column_past_four_mebibytes_borrows() {
        // Review-5: the wave-5 cap compared absolute positions and refused
        // any ordinary column whose text passed 4 MiB.
        let values = vec![b'a'; 5 << 20];
        let rows: Vec<i32> = (0..=5).map(|one| one << 20).collect();
        assert!(utf8_over(&rows, &values).is_ok());
        let views = one_view(13, 0, (5 << 20) - 13);
        assert!(views_over(&views, &[&values], 1).is_ok());
    }

    #[test]
    fn null_tables_and_row_claims_are_refused() {
        // Review-4: a null table, a null offsets slot, a 2^40 row claim
        // (length or offset), and a negative length never reach a read.
        let mut outer = unsafe { std::mem::zeroed::<ArrowArray>() };
        outer.length = 1;
        outer.n_buffers = 3;
        assert!(unsafe { borrow_strings(&outer, Text::View, 0, 1) }.is_err());
        let values = b"abc".to_vec();
        let held = [std::ptr::null(), std::ptr::null(), values.as_ptr().cast()];
        let outer = array_of(&held, 1);
        assert!(unsafe { borrow_strings(&outer, Text::Utf8, 0, 1) }.is_err());
        let offsets = [0i32, 3].map(i32::to_le_bytes).concat();
        let held = pointers(&[b"".as_slice(), &offsets, &values]);
        for (length, offset) in [(1i64 << 40, 0), (1, 1i64 << 40), (-1, 0)] {
            let mut outer = array_of(&held, length);
            outer.offset = offset;
            let skip = offset.max(0) as usize;
            let rows = length.max(0) as usize;
            assert!(unsafe { borrow_strings(&outer, Text::Utf8, skip, rows) }.is_err());
        }
    }

    #[test]
    fn a_conformant_view_column_round_trips() {
        // The shape real producers export: one inline view and two long
        // views naming two data buffers, then the sizes buffer, per the
        // interface's table `[validity, views, data, data, sizes]`.
        let first = b"alpha runs longer than twentyfour".to_vec();
        let second = b"beta also runs long past twelve".to_vec();
        let mut inline = one_view(5, 0, 0);
        inline[4..9].copy_from_slice(b"gamma");
        let views = [one_view(24, 0, 3), inline, one_view(21, 1, 4)].concat();
        let Ok(texts) = views_over(&views, &[&first, &second], 3) else {
            // Formatting a live PyErr needs the interpreter, which a bare
            // test thread cannot start, so the failure is said in words.
            panic!("a conformant view column borrows");
        };
        assert_eq!(texts, ["ha runs longer than twen", "gamma", " also runs long past "]);
    }

    /// A region with one readable page between an unreadable page and a
    /// 64 MiB unreadable reservation, so a read before or past the page
    /// faults on every run.
    #[cfg(target_os = "linux")]
    struct Guarded {
        base: *mut u8,
        size: usize,
    }

    #[cfg(target_os = "linux")]
    extern "C" {
        fn mmap(address: *mut c_void, length: usize, protect: c_int, flags: c_int, fd: c_int, offset: i64) -> *mut c_void;
        fn mprotect(address: *mut c_void, length: usize, protect: c_int) -> c_int;
        fn munmap(address: *mut c_void, length: usize) -> c_int;
    }

    #[cfg(target_os = "linux")]
    impl Guarded {
        const PAGE: usize = 4096;

        /// The reservation with its one readable page, page one.
        fn new() -> Self {
            let size = 2 * Self::PAGE + (64 << 20);
            // PROT_NONE, MAP_PRIVATE | MAP_ANONYMOUS, then PROT_READ | PROT_WRITE on page one.
            let base = unsafe { mmap(std::ptr::null_mut(), size, 0, 0x02 | 0x20, -1, 0) } as *mut u8;
            assert!(!base.is_null() && base as isize != -1, "the reservation maps");
            assert_eq!(unsafe { mprotect(base.add(Self::PAGE).cast(), Self::PAGE, 0x1 | 0x2) }, 0);
            Self { base, size }
        }

        /// `bytes` copied into the readable page at `offset`.
        fn place(&self, offset: usize, bytes: &[u8]) -> *const u8 {
            assert!(offset + bytes.len() <= Self::PAGE);
            let at = unsafe { self.base.add(Self::PAGE + offset) };
            unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), at, bytes.len()) };
            at
        }

        /// `bytes` placed so they end exactly at the readable page's end,
        /// with 64 MiB of unreadable reservation after them.
        fn ending_with(bytes: &[u8]) -> (Self, *const u8) {
            let region = Self::new();
            let at = region.place(Self::PAGE - bytes.len(), bytes);
            (region, at)
        }
    }

    #[cfg(target_os = "linux")]
    impl Drop for Guarded {
        fn drop(&mut self) {
            unsafe { munmap(self.base.cast(), self.size) };
        }
    }

    #[cfg(target_os = "linux")]
    fn utf8_at(offsets: &[i32], values: *const u8, text: Text) -> PyResult<Vec<&'static str>> {
        let bytes: Vec<u8> = match text {
            Text::LargeUtf8 => offsets.iter().flat_map(|one| i64::from(*one).to_le_bytes()).collect(),
            _ => offsets.iter().flat_map(|one| one.to_le_bytes()).collect(),
        };
        let held = [std::ptr::null(), bytes.as_ptr().cast(), values.cast()];
        let rows = offsets.len() - 1;
        let outer = array_of(&held, rows as i64);
        unsafe { borrow_strings(&outer, text, 0, rows) }
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn an_extent_past_readable_memory_is_refused() {
        // Review 7 (R3-4, R5-6): offsets 60,000,000 .. 60,000,001 over a
        // three-byte buffer clear the per-row cap and the 1 GiB extent cap,
        // and so do offsets 0 .. 200. Each read lands in unreadable memory
        // and killed the host; the readability probe refuses them first.
        let (_region, abc) = Guarded::ending_with(b"abc");
        for text in [Text::Utf8, Text::LargeUtf8] {
            for offsets in [[60_000_000, 60_000_001], [0, 200]] {
                assert_eq!(refusal(utf8_at(&offsets, abc, text)), UNREADABLE);
            }
            assert_eq!(utf8_at(&[0, 3], abc, text).expect("the real bytes borrow"), ["abc"]);
        }
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn a_sizes_buffer_shorter_than_its_count_is_refused() {
        // Review 7 (R5-6): a table of two data buffers whose sizes buffer
        // holds one i64 against a guard page. The view names data buffer
        // 1, so the reader needs the second size, past the guard.
        let data = [b'x'; 100];
        let (_region, sizes) = Guarded::ending_with(&100i64.to_le_bytes());
        let views = one_view(13, 1, 0);
        let held = [std::ptr::null(), views.as_ptr().cast(), data.as_ptr().cast(), data.as_ptr().cast(), sizes.cast()];
        let outer = array_of(&held, 1);
        assert_eq!(refusal(unsafe { borrow_strings(&outer, Text::View, 0, 1) }), UNREADABLE);
        // A view whose own 13 bytes, at offset 10 of a 20-byte tail, run
        // past the readable page into what its buffer declares.
        let (_short, abc) = Guarded::ending_with(&[b'y'; 20]);
        let sizes = [200i64, 200].map(i64::to_le_bytes).concat();
        let views = one_view(13, 1, 10);
        let held = [std::ptr::null(), views.as_ptr().cast(), abc.cast(), abc.cast(), sizes.as_ptr().cast()];
        let outer = array_of(&held, 1);
        assert_eq!(refusal(unsafe { borrow_strings(&outer, Text::View, 0, 1) }), UNREADABLE);
    }

    #[test]
    fn an_answer_column_past_i32_offsets_is_refused() {
        // Review 7 (R4-15): the output offsets were `len as i32`, which
        // wraps negative past 2 GiB of text.
        assert_eq!(utf8_offset(i32::MAX as usize).ok(), Some(i32::MAX));
        assert_eq!(
            refusal(utf8_offset(i32::MAX as usize + 1)),
            "an answer column's text passes the 2 GiB a text column's offsets can name"
        );
    }

    #[test]
    fn schema_metadata_is_copied_byte_for_byte() {
        // Review 7 (R4-15): the output schema set every metadata pointer
        // to null, so a Polars Enum came back Categorical and a field's
        // own keys were lost.
        let mut blob = 1i32.to_le_bytes().to_vec();
        for part in [b"unit".as_slice(), b"cm"] {
            blob.extend_from_slice(&(part.len() as i32).to_le_bytes());
            blob.extend_from_slice(part);
        }
        let mut tree = SchemaTree::default();
        let Ok(memory) = Readable::snapshot() else { panic!("the memory map reads") };
        let Ok(copied) = tree.metadata(blob.as_ptr().cast(), Some(&memory)) else { panic!("a sound blob copies") };
        assert_ne!(copied, blob.as_ptr().cast());
        assert_eq!(unsafe { std::slice::from_raw_parts(copied as *const u8, blob.len()) }, blob.as_slice());
        let negative = (-1i32).to_le_bytes();
        assert_eq!(refusal(tree.metadata(negative.as_ptr().cast(), Some(&memory))), BAD_METADATA);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn crafted_metadata_is_refused_before_it_is_read() {
        // Review 7, second pass: the metadata walk trusted every length.
        // A key length past a guard page, a value length past it, a pair
        // count of 2^31 - 1, and a 2 GiB key length in a readable blob
        // each read past the blob and killed the host.
        let Ok(memory) = Readable::snapshot() else { panic!("the memory map reads") };
        let words = |parts: &[i32]| parts.iter().flat_map(|one| one.to_le_bytes()).collect::<Vec<u8>>();
        let mut value_past = words(&[1, 4]);
        value_past.extend_from_slice(b"unit");
        value_past.extend_from_slice(&5000i32.to_le_bytes());
        for blob in [words(&[1, 100]), value_past, words(&[i32::MAX])] {
            let (_region, at) = Guarded::ending_with(&blob);
            assert_eq!(refusal(SchemaTree::default().metadata(at.cast(), Some(&memory))), BAD_METADATA);
        }
        let huge = words(&[1, 0x7fff_fff0]);
        assert_eq!(refusal(SchemaTree::default().metadata(huge.as_ptr().cast(), Some(&memory))), BAD_METADATA);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn the_memory_map_keeps_readable_ranges_and_merges_touching_ones() {
        // Review 7, second pass: a protected page (`---p`) is mapped but
        // unreadable, so it must leave a gap the check refuses.
        let map = "1000-2000 r--p 0 00:00 0 /lib\n2000-3000 rw-p 0 00:00 0\n3000-4000 ---p 0 00:00 0\n5000-6000 r-xp 0 00:00 0\nbad line\n";
        assert_eq!(readable_ranges(map), [(0x1000, 0x3000), (0x5000, 0x6000)]);
        let memory = Readable { ranges: readable_ranges(map) };
        let at = |place: usize| place as *const u8;
        assert!(memory.covers(at(0x1800), 0x1000));
        assert!(!memory.covers(at(0x2800), 0x1000));
        assert!(!memory.covers(at(0x4800), 0x10));
        assert!(memory.covers(at(0x5000), 0x1000));
        assert!(!memory.covers(at(0x5000), 0x1001));
        assert!(memory.covers(at(0x4800), 0));
    }

    /// The strings a shape borrows, or a panic naming the refusal: a live
    /// PyErr cannot be formatted without the interpreter.
    fn borrowed(outcome: PyResult<Vec<&'static str>>) -> Vec<&'static str> {
        match outcome {
            Ok(texts) => texts,
            Err(error) => panic!("refused: {}", refusal::<()>(Err(error))),
        }
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn only_the_bytes_the_selected_rows_read_are_checked() {
        // Review 7, second pass: the check covered a Utf8 buffer from byte 0
        // to the last offset and a view buffer's whole declared size, so a
        // 10-row tail slice of a 1 GiB column paid for the whole gibibyte.
        // Here every byte outside the rows read is unreadable: the rows sit
        // at the start of the readable page, the buffer pointer one page
        // before it, and the declared extents run past the page's end.
        let region = Guarded::new();
        let page = region.place(0, b"alpha runs past twelve");
        let values = page.wrapping_sub(Guarded::PAGE);
        let at = Guarded::PAGE as i32;
        assert_eq!(borrowed(utf8_at(&[at, at + 5], values, Text::Utf8)), ["alpha"]);
        // A slice: rows 1 and 2 of a column whose row 0 and row 3 lie in
        // unreadable memory on either side of the page.
        let offsets = [0, at, at + 5, at + 10, at + 9000].map(i32::to_le_bytes).concat();
        let held = [std::ptr::null(), offsets.as_ptr().cast(), values.cast()];
        let mut outer = array_of(&held, 2);
        outer.offset = 1;
        assert_eq!(borrowed(unsafe { borrow_strings(&outer, Text::Utf8, 1, 2) }), ["alpha", " runs"]);
        // A view reading 22 bytes one page into a buffer declared far past
        // the readable page.
        let views = one_view(22, 0, Guarded::PAGE as u32);
        let sizes = (64i64 << 20).to_le_bytes();
        let held = [std::ptr::null(), views.as_ptr().cast(), values.cast(), sizes.as_ptr().cast()];
        let outer = array_of(&held, 1);
        assert_eq!(borrowed(unsafe { borrow_strings(&outer, Text::View, 0, 1) }), ["alpha runs past twelve"]);
    }
}
