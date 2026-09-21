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

/// Borrow one already-validated array's strings out of its buffers.
///
/// # Safety
/// `array` must point at a live `ArrowArray` of the given text layout,
/// and the memory must outlive the returned strings.
unsafe fn borrow_strings(array: *const ArrowArray, text: Text) -> PyResult<Vec<&'static str>> {
    if (*array).null_count != 0 {
        return Err(UsageError::new_err(
            "the column holds nulls; the engine needs text, and NA rows are the caller's to drop",
        ));
    }
    let count = (*array).length.max(0) as usize;
    let skip = (*array).offset.max(0) as usize;
    let mut texts = Vec::with_capacity(count);
    if let Text::View = text {
        if (*array).n_buffers < 2 {
            return Err(UsageError::new_err("the string-view array has no views buffer"));
        }
        let buffers = (*array).buffers;
        let views = *buffers.offset(1) as *const u8;
        if views.is_null() {
            return Err(UsageError::new_err("the string-view array has no views buffer"));
        }
        let data_buffers = (*array).n_buffers - 2;
        let mut place = 0usize;
        while place < count {
            let view = views.add((skip + place) * 16);
            let size = u32::from_le_bytes([
                *view,
                *view.add(1),
                *view.add(2),
                *view.add(3),
            ]) as usize;
            let (start, length) = if size <= 12 {
                (view.add(4), size)
            } else {
                let index = u32::from_le_bytes([
                    *view.add(8),
                    *view.add(9),
                    *view.add(10),
                    *view.add(11),
                ]) as i64;
                let offset = u32::from_le_bytes([
                    *view.add(12),
                    *view.add(13),
                    *view.add(14),
                    *view.add(15),
                ]) as usize;
                if index < 0 || index >= data_buffers {
                    return Err(UsageError::new_err(
                        "a string view points past its data buffers",
                    ));
                }
                let base = *buffers.offset(2 + index as isize) as *const u8;
                if base.is_null() {
                    return Err(UsageError::new_err("a string view names an empty buffer"));
                }
                (base.add(offset), size)
            };
            let bytes = std::slice::from_raw_parts(start, length);
            let value = std::str::from_utf8(bytes)
                .map_err(|_| UsageError::new_err("the column's buffer is not valid UTF-8"))?;
            texts.push(value);
            place += 1;
        }
        return Ok(texts);
    }
    if (*array).n_buffers < 3 {
        return Err(UsageError::new_err("the string array has no offsets buffer"));
    }
    let buffers = (*array).buffers;
    let values = *buffers.offset(2) as *const u8;
    if values.is_null() {
        return Err(UsageError::new_err("the string array has no values buffer"));
    }
    let large = matches!(text, Text::LargeUtf8);
    let mut place = 0usize;
    while place < count {
        let (start, end) = if large {
            let offsets = *buffers.offset(1) as *const i64;
            (
                *offsets.add(skip + place) as usize,
                *offsets.add(skip + place + 1) as usize,
            )
        } else {
            let offsets = *buffers.offset(1) as *const i32;
            (
                *offsets.add(skip + place) as usize,
                *offsets.add(skip + place + 1) as usize,
            )
        };
        let bytes = std::slice::from_raw_parts(values.add(start), end - start);
        let text = std::str::from_utf8(bytes)
            .map_err(|_| UsageError::new_err("the column's buffer is not valid UTF-8"))?;
        texts.push(text);
        place += 1;
    }
    Ok(texts)
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
        let mut schema = std::mem::zeroed::<ArrowSchema>();
        let got = (*stream).get_schema.map_or(-1, |get| get(stream, &mut schema));
        if got != 0 {
            return Err(stream_error(stream, "the column stream would not name its schema"));
        }
        let large = require_text(&schema)?;
        column.schemas.push(schema);
        loop {
            let mut array = std::mem::zeroed::<ArrowArray>();
            let got = (*stream).get_next.map_or(-1, |next| next(stream, &mut array));
            if got != 0 {
                return Err(stream_error(stream, "the column stream stopped mid-column"));
            }
            if array.release.is_none() {
                break;
            }
            column.texts.append(&mut borrow_strings(&array, large)?);
            column.batches.push(array);
        }    }
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
/// so `build_frame` can copy names and formats.
pub(crate) struct FrameColumn {
    pub(crate) texts: Vec<&'static str>,
    on: usize,
    names: Vec<String>,
    formats: Vec<Option<Vec<u8>>>,
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

    let mut schema = Box::new(unsafe { std::mem::zeroed::<ArrowSchema>() });
    unsafe {
        let got = (*stream).get_schema.map_or(-1, |get| get(stream, schema.as_mut()));
        if got != 0 {
            return Err(stream_error(stream, "the frame stream would not name its schema"));
        }
        let format = CStr::from_ptr(schema.format).to_bytes();
        if format != b"+s" {
            return Err(UsageError::new_err(format!(
                "the stream is '{}', not a frame (a struct); annotate with on= needs a data frame, and a plain column rides the series door",
                String::from_utf8_lossy(format)
            )));
        }
    }
    let count = schema.n_children.max(0) as usize;
    if count == 0 {
        return Err(UsageError::new_err("the frame has no columns"));
    }
    let mut names = Vec::with_capacity(count);
    let mut formats = Vec::with_capacity(count);
    let mut on_index = None;
    for place in 0..count {
        let child = unsafe { *schema.children.add(place) };
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
        let format = if child_ref.format.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(child_ref.format) }.to_bytes().to_vec())
        };
        if name == on {
            on_index = Some(place);
        }
        names.push(name);
        formats.push(format);
    }
    let Some(on_index) = on_index else {
        return Err(UsageError::new_err(format!(
            "the frame has no column named '{on}'"
        )));
    };
    // The `on` column must be whole text, checked on its own schema.
    let large = unsafe {
        let child = *schema.children.add(on_index);
        require_text(&*child)?
    };

    let mut column = FrameColumn {
        texts: Vec::new(),
        on: on_index,
        names,
        formats,
        lengths: Vec::new(),
        hold: Vec::new(),
        schema: std::mem::replace(&mut schema, Box::new(unsafe { std::mem::zeroed() })),
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
            let child_ptrs = array.children;
            let child = std::ptr::read(child_ptrs.add(column.on));
            column
                .texts
                .append(&mut borrow_strings(child, large)?);
            column.lengths.push(array.length.max(0) as usize);
            column.hold.push(BatchHold {
                root: array,
                child_ptrs,
            });
        }
    }
    Ok(column)
}

/// One batch's whole allocation, handed to the consumer with the array.
/// The array's `release` drops this box, so the memory lives exactly as
/// long as the consumer owns the batch, per the C interface.
#[allow(dead_code)] // the fields anchor the pointers handed out in the array
struct BatchKeep {
    root_buffers: Vec<*const c_void>,
    children: Vec<Box<ArrowArray>>,
    child_ptrs: Vec<*mut ArrowArray>,
    buffers: Vec<Vec<*const c_void>>,
    owned: Vec<Vec<u8>>,
    /// The caller's frame, kept alive while any aliased original column
    /// of this batch is alive. `None` for a table this surface built
    /// whole, where every buffer is its own.
    hold: Option<std::sync::Arc<FrameColumn>>,
}

/// The batch release: the consumer is done with the arrays and their
/// buffers, so the whole allocation goes.
unsafe extern "C" fn batch_release(out: *mut ArrowArray) {
    let keep = (*out).private_data as *mut BatchKeep;
    if !keep.is_null() {
        (*out).private_data = std::ptr::null_mut();
        drop(Box::from_raw(keep));
    }
}

/// A schema tree handed to the consumer: its own boxes and strings, freed
/// by the tree's release when the consumer is done with it.
struct SchemaKeep {
    children: Vec<Box<ArrowSchema>>,
    child_ptrs: Vec<*mut ArrowSchema>,
    strings: Vec<CString>,
}

/// The schema release: free the tree the consumer was given.
unsafe extern "C" fn schema_tree_release(root: *mut ArrowSchema) {
    let keep = (*root).private_data as *mut SchemaKeep;
    if !keep.is_null() {
        (*root).private_data = std::ptr::null_mut();
        drop(Box::from_raw(keep));
    }
}

/// The whole frame behind the output stream: the schema template to deep
/// copy from, the batches not yet handed out, and the error phrase.
#[allow(dead_code)] // the schema template anchors the pointers deep-copied out
pub(crate) struct OutFrame {
    schema_children: Vec<Box<ArrowSchema>>,
    schema_strings: Vec<CString>,
    batches: Vec<Option<BatchKeep>>,
    root_template: Box<ArrowArray>,
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
    let mut schema_strings: Vec<CString> = Vec::with_capacity(total * 2 + 1);
    schema_strings.push(CString::new("+s").expect("static format"));
    let mut schema_children: Vec<Box<ArrowSchema>> = Vec::with_capacity(total);
    let mut formats_for_new: Vec<&'static str> = Vec::with_capacity(names.len());
    for name in names {
        let format = match rows.first().and_then(|row| {
            row.iter()
                .find(|(held, _)| held == name)
                .map(|(_, field)| field)
        }) {
            Some(Annotated::Decision(_)) => "b",
            Some(Annotated::Choice(_)) => "u",
            Some(Annotated::Score(_)) => "g",
            Some(Annotated::Tags(_)) => "u",
            None => "u",
        };
        formats_for_new.push(format);
    }
    for (place, name) in frame.names.iter().enumerate() {
        let owned = CString::new(name.as_str())
            .map_err(|_| UsageError::new_err("a column name holds a NUL byte"))?;
        let name_ptr = owned.as_ptr();
        schema_strings.push(owned);
        let format = match &frame.formats[place] {
            Some(bytes) => CString::new(bytes.clone())
                .map_err(|_| UsageError::new_err("a column format holds a NUL"))?,
            None => CString::new("u").expect("static format"),
        };
        let format_ptr = format.as_ptr();
        schema_strings.push(format);
        schema_children.push(Box::new(ArrowSchema {
            format: format_ptr,
            name: name_ptr,
            metadata: std::ptr::null(),
            flags: 2,
            n_children: 0,
            children: std::ptr::null_mut(),
            dictionary: std::ptr::null_mut(),
            release: Some(schema_tree_release),
            private_data: std::ptr::null_mut(),
        }));
    }
    for (place, name) in names.iter().enumerate() {
        let owned = CString::new(name.as_str())
            .map_err(|_| UsageError::new_err("a question name holds a NUL byte"))?;
        let name_ptr = owned.as_ptr();
        schema_strings.push(owned);
        let format = CString::new(formats_for_new[place]).expect("static format");
        let format_ptr = format.as_ptr();
        schema_strings.push(format);
        schema_children.push(Box::new(ArrowSchema {
            format: format_ptr,
            name: name_ptr,
            metadata: std::ptr::null(),
            flags: 2,
            n_children: 0,
            children: std::ptr::null_mut(),
            dictionary: std::ptr::null_mut(),
            release: Some(schema_tree_release),
            private_data: std::ptr::null_mut(),
        }));
    }

    let hold = std::sync::Arc::new(frame);
    // One batch out per batch in, so aliased originals stay per-batch.
    let mut batches: Vec<Option<BatchKeep>> = Vec::with_capacity(hold.hold.len().max(1));
    let mut root_template: Option<Box<ArrowArray>> = None;
    let mut base = 0usize;
    for (batch_place, length) in hold.lengths.iter().copied().enumerate() {
        let mut children: Vec<Box<ArrowArray>> = Vec::with_capacity(total);
        let mut buffers: Vec<Vec<*const c_void>> = Vec::with_capacity(total);
        let mut owned: Vec<Vec<u8>> = Vec::new();
        // Originals, aliased from the hold.
        for place in 0..originals {
            let child = unsafe { hold.hold[batch_place].child(place) };
            let buffers_ptr = child.buffers;
            buffers.push(Vec::new());
            children.push(Box::new(ArrowArray {
                length: child.length,
                null_count: child.null_count,
                offset: child.offset,
                n_buffers: child.n_buffers,
                n_children: child.n_children,
                buffers: buffers_ptr,
                children: child.children,
                dictionary: std::ptr::null_mut(),
                release: Some(batch_release),
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
                        values_str.push(None);
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
                        values_str.push(None);
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
                    let mut offsets = Vec::with_capacity((length + 1) * 4);
                    let mut values = Vec::new();
                    offsets.extend_from_slice(&0i32.to_le_bytes());
                    for text in &values_str {
                        if let Some(text) = text {
                            values.extend_from_slice(text.as_bytes());
                        }
                        offsets.extend_from_slice(&(values.len() as i32).to_le_bytes());
                    }
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
                release: Some(batch_release),
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
            release: Some(batch_release),
            private_data: std::ptr::null_mut(),
        });
        if root_template.is_none() {
            root_template = Some(Box::new(unsafe { std::ptr::read(root.as_ref()) }));
        }
        batches.push(Some(BatchKeep {
            root_buffers,
            children,
            child_ptrs,
            buffers,
            owned,
            hold: Some(std::sync::Arc::clone(&hold)),
        }));
        base += length;
    }
    let root_template = root_template.unwrap_or_else(|| {
        Box::new(ArrowArray {
            length: 0,
            null_count: 0,
            offset: 0,
            n_buffers: 1,
            n_children: total as i64,
            buffers: std::ptr::null_mut(),
            children: std::ptr::null_mut(),
            dictionary: std::ptr::null_mut(),
            release: Some(batch_release),
            private_data: std::ptr::null_mut(),
        })
    });

    Ok(OutFrame {
        schema_children,
        schema_strings,
        batches,
        root_template,
        emitted: 0,
        error: None,
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
    let mut schema_strings: Vec<CString> = Vec::with_capacity(total * 2 + 1);
    schema_strings.push(CString::new("+s").expect("static format"));
    let mut schema_children: Vec<Box<ArrowSchema>> = Vec::with_capacity(total);
    for (name, value) in columns {
        let owned = CString::new(*name)
            .map_err(|_| UsageError::new_err("a column name holds a NUL byte"))?;
        let name_ptr = owned.as_ptr();
        schema_strings.push(owned);
        let format = CString::new(value.format()).expect("static format");
        let format_ptr = format.as_ptr();
        schema_strings.push(format);
        schema_children.push(Box::new(ArrowSchema {
            format: format_ptr,
            name: name_ptr,
            metadata: std::ptr::null(),
            flags: 2,
            n_children: 0,
            children: std::ptr::null_mut(),
            dictionary: std::ptr::null_mut(),
            release: Some(schema_tree_release),
            private_data: std::ptr::null_mut(),
        }));
    }

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
                let mut offsets = Vec::with_capacity((length + 1) * 4);
                let mut values = Vec::new();
                offsets.extend_from_slice(&0i32.to_le_bytes());
                for text in texts {
                    values.extend_from_slice(text.as_bytes());
                    offsets.extend_from_slice(&(values.len() as i32).to_le_bytes());
                }
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
            release: Some(batch_release),
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
        release: Some(batch_release),
        private_data: std::ptr::null_mut(),
    });
    let root_template = Box::new(unsafe { std::ptr::read(root.as_ref()) });
    let batches = vec![Some(BatchKeep {
        root_buffers,
        children,
        child_ptrs,
        buffers,
        owned,
        hold: None,
    })];
    Ok(OutFrame {
        schema_children,
        schema_strings,
        batches,
        root_template,
        emitted: 0,
        error: None,
    })
}

/// Deep-copy the schema template into a tree the consumer owns and
/// releases through `schema_tree_release`.
unsafe fn hand_schema(state: &OutFrame, out: *mut ArrowSchema) {
    let mut strings: Vec<CString> = Vec::with_capacity(state.schema_strings.len());
    for text in &state.schema_strings {
        strings.push(text.clone());
    }
    let mut children: Vec<Box<ArrowSchema>> = Vec::with_capacity(state.schema_children.len());
    for child in &state.schema_children {
        let name = if child.name.is_null() {
            std::ptr::null()
        } else {
            let bytes = CStr::from_ptr(child.name).to_bytes();
            let owned = CString::new(bytes).unwrap_or_default();
            let pointer = owned.as_ptr();
            strings.push(owned);
            pointer
        };
        let format = if child.format.is_null() {
            std::ptr::null()
        } else {
            let bytes = CStr::from_ptr(child.format).to_bytes();
            let owned = CString::new(bytes).unwrap_or_default();
            let pointer = owned.as_ptr();
            strings.push(owned);
            pointer
        };
        children.push(Box::new(ArrowSchema {
            format,
            name,
            metadata: std::ptr::null(),
            flags: child.flags,
            n_children: 0,
            children: std::ptr::null_mut(),
            dictionary: std::ptr::null_mut(),
            release: Some(schema_tree_release),
            private_data: std::ptr::null_mut(),
        }));
    }
    let child_ptrs: Vec<*mut ArrowSchema> = children
        .iter_mut()
        .map(|child| child.as_mut() as *mut ArrowSchema)
        .collect();
    let keep = Box::new(SchemaKeep {
        children,
        child_ptrs,
        strings,
    });
    let keep_ptr = Box::into_raw(keep);
    let keep = &*keep_ptr;
    *out = ArrowSchema {
        format: keep.strings[0].as_ptr(),
        name: std::ptr::null(),
        metadata: std::ptr::null(),
        flags: 0,
        n_children: keep.children.len() as i64,
        children: keep.child_ptrs.as_ptr() as *mut *mut ArrowSchema,
        dictionary: std::ptr::null_mut(),
        release: Some(schema_tree_release),
        private_data: keep_ptr as *mut c_void,
    };
}

unsafe extern "C" fn frame_get_schema(
    stream: *mut ArrowArrayStream,
    out: *mut ArrowSchema,
) -> c_int {
    let state = &mut *((*stream).private_data as *mut OutFrame);
    hand_schema(state, out);
    0
}

unsafe extern "C" fn frame_get_next(stream: *mut ArrowArrayStream, out: *mut ArrowArray) -> c_int {
    let state = &mut *((*stream).private_data as *mut OutFrame);
    if state.emitted < state.batches.len() {
        let keep = state.batches[state.emitted].take();
        state.emitted += 1;
        if let Some(keep) = keep {
            *out = std::ptr::read(state.root_template.as_ref());
            let keep_ptr = Box::into_raw(Box::new(keep));
            (*out).private_data = keep_ptr as *mut c_void;
            (*out).release = Some(batch_release);
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
        Ok((
            *(*batch).buffers.offset(2) as usize,
            *(*batch).buffers.offset(1) as usize,
            (*batch).length.max(0) as usize,
        ))
    }
}
