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

/// Borrow one already-validated array's strings out of its buffers.
///
/// `skip` is where the caller's rows begin and `count` how many there
/// are. For a series-shaped array they are the array's own offset and
/// length; for a frame's `on` column the struct root's offset and length
/// are added, because the Arrow columnar format lets a sliced struct keep
/// its slice on the root and leave the children whole — the reader
/// applies the parent's offset when indexing.
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
    let carried = (*array).offset.max(0) as usize + (*array).length.max(0) as usize;
    if skip.saturating_add(count) > carried {
        return Err(UsageError::new_err(
            "the frame's struct root asks for rows its column does not carry",
        ));
    }
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
/// The array's `release` drops this box, so the memory lives exactly as
/// long as the consumer owns the batch, per the C interface.
///
/// Each batch carries its own root, because one stream may hand out
/// several batches and every one must describe its own children: a shared
/// root template would make the second batch alias the first batch's
/// arrays, and the first batch's box is freed as soon as the consumer
/// releases it.
#[allow(dead_code)] // the fields anchor the pointers handed out in the array
struct BatchKeep {
    /// This batch's own root struct, read out on emission.
    root: Box<ArrowArray>,
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
/// buffers, so the whole allocation goes. The release pointer is cleared
/// as the C data interface requires, so a second call cannot free again.
unsafe extern "C" fn batch_release(out: *mut ArrowArray) {
    (*out).release = None;
    let keep = (*out).private_data as *mut BatchKeep;
    if !keep.is_null() {
        (*out).private_data = std::ptr::null_mut();
        drop(Box::from_raw(keep));
    }
}

/// The owned pieces of one schema tree: every struct boxed, the
/// child-pointer arrays, and the strings the structs point at. Every
/// buffer lives on the heap, so pointers into them stay put when the tree
/// moves.
struct SchemaTree {
    boxes: Vec<Box<ArrowSchema>>,
    child_ptrs: Vec<Vec<*mut ArrowSchema>>,
    strings: Vec<CString>,
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

    /// One node over already-owned pieces; the child array is kept here so
    /// its pointer stays valid.
    fn node(
        &mut self,
        format: *const c_char,
        name: *const c_char,
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
            metadata: std::ptr::null(),
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
        Ok(self.node(format_ptr, name_ptr, flags, Vec::new(), std::ptr::null_mut()))
    }

    /// The struct node every frame hands out: the `+s` format, no name.
    fn branch(&mut self, children: Vec<*mut ArrowSchema>) -> PyResult<*mut ArrowSchema> {
        let format =
            CString::new("+s").map_err(|_| UsageError::new_err("a column format holds a NUL"))?;
        let format_ptr = format.as_ptr();
        self.strings.push(format);
        Ok(self.node(format_ptr, std::ptr::null(), 0, children, std::ptr::null_mut()))
    }

    /// Deep-copy one schema struct and its whole tree: names, formats,
    /// children, and a dictionary when one is attached. Every node is this
    /// tree's own, so the consumer's release frees exactly what it was
    /// handed.
    ///
    /// # Safety
    /// `source` must point at a live schema struct.
    unsafe fn copy(&mut self, source: *const ArrowSchema) -> *mut ArrowSchema {
        let format = self.text(unsafe { (*source).format });
        let name = self.text(unsafe { (*source).name });
        let count = unsafe { (*source).n_children }.max(0) as usize;
        let mut children = Vec::with_capacity(count);
        for place in 0..count {
            let child = unsafe { *(*source).children.add(place) };
            children.push(unsafe { self.copy(child) });
        }
        let dictionary = unsafe { (*source).dictionary };
        let dictionary = if dictionary.is_null() {
            std::ptr::null_mut()
        } else {
            unsafe { self.copy(dictionary) }
        };
        let flags = unsafe { (*source).flags };
        self.node(format, name, flags, children, dictionary)
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
    let hold = std::sync::Arc::new(frame);
    // The output schema: the caller's columns keep their own whole schema
    // trees — a dictionary, a struct's fields, a list's element — deep
    // copied so the output describes exactly the aliased arrays, and the
    // new columns get one leaf each.
    let mut schema = SchemaTree::default();
    let mut schema_children: Vec<*mut ArrowSchema> = Vec::with_capacity(total);
    for place in 0..originals {
        let source = unsafe { *hold.schema.children.add(place) };
        schema_children.push(unsafe { schema.copy(source) });
    }
    for (place, name) in names.iter().enumerate() {
        schema_children.push(schema.leaf(Some(name), formats_for_new[place], 2)?);
    }
    let schema_root = schema.branch(schema_children)?;
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
    let schema_root = schema.branch(schema_children)?;
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
/// releases through `schema_tree_release`.
unsafe fn hand_schema(state: &OutFrame, out: *mut ArrowSchema) {
    let mut tree = SchemaTree::default();
    let root = unsafe { tree.copy(state.schema.root) };
    let keep = Box::into_raw(Box::new(tree));
    *out = std::ptr::read(root);
    (*out).private_data = keep as *mut c_void;
    (*out).release = Some(schema_tree_release);
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
            // The emitted root is this batch's own, so the second batch
            // describes its own children rather than the first's.
            *out = std::ptr::read(keep.root.as_ref());
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
        Ok((
            *(*batch).buffers.offset(2) as usize,
            *(*batch).buffers.offset(1) as usize,
            (*batch).length.max(0) as usize,
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
