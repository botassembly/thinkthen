//! The binding's one raw-memory module (ADR 0047 item 3, ticket 0106
//! decision 7). Every `unsafe` block of the binding sits here, each beside
//! the rule it keeps. The rest of `arrow` reads a producer's memory only
//! through `bytes` and `record`, which read nothing the call's snapshot has
//! not vouched for.
//!
//! In: a producer's stream or array is moved out of its capsule, as the C
//! stream interface's move rule allows, and every batch is taken on the
//! calling thread. The `Imported` value then crosses to the worker, which
//! reads the strings in place and releases the batches (`gate`).
//!
//! Out: `out/ffi.rs` turns answer trees into C structs, and `probe/ffi.rs`
//! holds the probe build's lock-bound producer. The policy check admits
//! `unsafe` only in files named `ffi.rs`, so each part keeps that name.

use std::ffi::{CStr, c_char, c_int, c_void};
use std::ptr;
use std::sync::Arc;

use crate::diagnostics::{host, host_owned};
use pyo3::prelude::*;
use pyo3::types::PyCapsule;

use super::memory::Readable;
use super::{gate, read};
use crate::usage;

pub(super) const STREAM: &CStr = c"arrow_array_stream";
pub(super) const SCHEMA: &CStr = c"arrow_schema";
pub(super) const ARRAY: &CStr = c"arrow_array";

/// `ArrowSchema` from the C data interface, fields in the interface's order.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct ArrowSchema {
    pub(super) format: *const c_char,
    pub(super) name: *const c_char,
    pub(super) metadata: *const c_char,
    pub(super) flags: i64,
    pub(super) n_children: i64,
    pub(super) children: *mut *mut ArrowSchema,
    pub(super) dictionary: *mut ArrowSchema,
    pub(super) release: Option<unsafe extern "C" fn(*mut ArrowSchema)>,
    pub(super) private_data: *mut c_void,
}

/// `ArrowArray` from the C data interface: `n_children` comes before
/// `buffers`, unlike `ArrowSchema`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct ArrowArray {
    pub(super) length: i64,
    pub(super) null_count: i64,
    pub(super) offset: i64,
    pub(super) n_buffers: i64,
    pub(super) n_children: i64,
    pub(super) buffers: *mut *const c_void,
    pub(super) children: *mut *mut ArrowArray,
    pub(super) dictionary: *mut ArrowArray,
    pub(super) release: Option<unsafe extern "C" fn(*mut ArrowArray)>,
    pub(super) private_data: *mut c_void,
}

/// `ArrowArrayStream` from the C stream interface.
#[repr(C)]
#[derive(Debug)]
pub(super) struct ArrowArrayStream {
    pub(super) get_schema:
        Option<unsafe extern "C" fn(*mut ArrowArrayStream, *mut ArrowSchema) -> c_int>,
    pub(super) get_next:
        Option<unsafe extern "C" fn(*mut ArrowArrayStream, *mut ArrowArray) -> c_int>,
    pub(super) get_last_error: Option<unsafe extern "C" fn(*mut ArrowArrayStream) -> *const c_char>,
    pub(super) release: Option<unsafe extern "C" fn(*mut ArrowArrayStream)>,
    pub(super) private_data: *mut c_void,
}

pub(super) const EMPTY_SCHEMA: ArrowSchema = ArrowSchema {
    format: ptr::null(),
    name: ptr::null(),
    metadata: ptr::null(),
    flags: 0,
    n_children: 0,
    children: ptr::null_mut(),
    dictionary: ptr::null_mut(),
    release: None,
    private_data: ptr::null_mut(),
};

pub(super) const EMPTY_ARRAY: ArrowArray = ArrowArray {
    length: 0,
    null_count: 0,
    offset: 0,
    n_buffers: 0,
    n_children: 0,
    buffers: ptr::null_mut(),
    children: ptr::null_mut(),
    dictionary: ptr::null_mut(),
    release: None,
    private_data: ptr::null_mut(),
};

/// A type whose every bit pattern is a valid value: plain integers, raw
/// pointers, and optional function pointers only.
pub(super) trait Plain: Copy {}
impl Plain for ArrowSchema {}
impl Plain for ArrowArray {}

/// `length` bytes at `at`, borrowed for as long as `_owner` lives, or `None`
/// when the snapshot does not vouch for every byte.
pub(super) fn bytes<'a, O: ?Sized>(
    _owner: &'a O,
    memory: &Readable,
    at: *const u8,
    length: usize,
) -> Option<&'a [u8]> {
    if length == 0 {
        return Some(&[]);
    }
    if at.is_null() || isize::try_from(length).is_err() || !memory.covers(at.addr(), length) {
        return None;
    }
    // SAFETY: the snapshot lists every byte as readable, and `_owner` keeps
    // the producer's memory alive (its batches are released only on drop).
    Some(unsafe { std::slice::from_raw_parts(at, length) })
}

/// A copy of `length` bytes at `at`, for a read with no owner to borrow
/// from: the borrow ends inside this call, so no caller can keep it.
pub(super) fn copied(memory: &Readable, at: *const u8, length: usize) -> Option<Vec<u8>> {
    let here = ();
    bytes(&here, memory, at, length).map(<[u8]>::to_vec)
}

/// A copy of the struct at `at`, or `None` when its bytes are not readable.
pub(super) fn record<T: Plain>(memory: &Readable, at: *const T) -> Option<T> {
    let whole = copied(memory, at.cast(), size_of::<T>())?;
    // SAFETY: `whole` holds `size_of::<T>()` readable bytes, and every bit
    // pattern is a valid `T` (`Plain`).
    Some(unsafe { ptr::read_unaligned(whole.as_ptr().cast::<T>()) })
}

/// A producer's stream or array, moved out of its capsule, with every batch.
#[derive(Debug)]
pub(crate) struct Imported {
    stream: Option<Box<ArrowArrayStream>>,
    pub(super) schema: ArrowSchema,
    pub(super) batches: Vec<ArrowArray>,
    /// Any producer's release may drop a Python reference, so a column's
    /// release waits for the interpreter behind the exit gate. A frame is
    /// Polars only (decision 4), whose releases are plain Rust.
    gated: bool,
}

// SAFETY: the C data interface lets a consumer move and release its structs
// on any thread. Which lock a release needs is the producer's rule, and
// `gate::release` answers it for a column. Nothing mutates an `Imported`
// through a shared reference, so sharing it for reads is sound.
unsafe impl Send for Imported {}
// SAFETY: as above.
unsafe impl Sync for Imported {}

impl Imported {
    /// A column: any object with `__arrow_c_stream__` or `__arrow_c_array__`.
    pub(crate) fn column(value: &Bound<'_, PyAny>) -> PyResult<Self> {
        if host(|| value.hasattr("__arrow_c_stream__"))? {
            return Self::stream(value, true);
        }
        let pair = host_owned(host(|| value.call_method0("__arrow_c_array__"))?);
        let (schema, array): (Bound<'_, PyCapsule>, Bound<'_, PyCapsule>) =
            host(|| pair.extract())?;
        let schema = host_owned(schema);
        let array = host_owned(array);
        let schema = schema
            .pointer_checked(Some(SCHEMA))?
            .as_ptr()
            .cast::<ArrowSchema>();
        let array = array
            .pointer_checked(Some(ARRAY))?
            .as_ptr()
            .cast::<ArrowArray>();
        // SAFETY: each capsule holds a live struct of its name. Moving it out
        // and clearing the capsule's release is the interface's move rule.
        let (schema, array) = unsafe {
            let moved = (ptr::read(schema), ptr::read(array));
            (*schema).release = None;
            (*array).release = None;
            moved
        };
        Ok(Self {
            stream: None,
            schema,
            batches: if array.release.is_some() {
                vec![array]
            } else {
                Vec::new()
            },
            gated: true,
        })
    }

    /// A Polars frame's stream.
    pub(crate) fn frame(value: &Bound<'_, PyAny>) -> PyResult<Self> {
        Self::stream(value, false)
    }

    fn stream(value: &Bound<'_, PyAny>, gated: bool) -> PyResult<Self> {
        let py = value.py();
        let capsule = host_owned(host(|| value.call_method0("__arrow_c_stream__"))?);
        let capsule = capsule.cast::<PyCapsule>()?;
        let source = capsule
            .pointer_checked(Some(STREAM))?
            .as_ptr()
            .cast::<ArrowArrayStream>();
        // SAFETY: the capsule holds a live stream. Moving it out and clearing
        // the capsule's release is the interface's move rule.
        let moved = unsafe {
            let moved = ptr::read(source);
            (*source).release = None;
            moved
        };
        let mut taken = Self {
            stream: Some(Box::new(moved)),
            schema: EMPTY_SCHEMA,
            batches: Vec::new(),
            gated,
        };
        let Some(stream) = taken.stream.as_deref_mut() else {
            return Err(usage(py, "the Arrow stream would not open"));
        };
        let stream: *mut ArrowArrayStream = stream;
        // SAFETY: `stream` is the live, boxed stream this value owns, and each
        // out-pointer is a valid, empty struct.
        unsafe {
            let got = (*stream)
                .get_schema
                .map_or(-1, |get| host(|| get(stream, &raw mut taken.schema)));
            if got != 0 || taken.schema.release.is_none() {
                return Err(usage(
                    py,
                    &last_error(stream, "the Arrow stream would not name its schema"),
                ));
            }
            loop {
                let mut batch = EMPTY_ARRAY;
                let got = (*stream)
                    .get_next
                    .map_or(-1, |next| host(|| next(stream, &raw mut batch)));
                if got != 0 {
                    return Err(usage(
                        py,
                        &last_error(stream, "the Arrow stream stopped mid-column"),
                    ));
                }
                if batch.release.is_none() {
                    break;
                }
                taken.batches.push(batch);
            }
        }
        Ok(taken)
    }

    /// Release every batch, the schema, and the stream, each once.
    fn release_all(&mut self) {
        // SAFETY: each struct came from its producer, and each release runs
        // once: the pointer is taken out before the call.
        unsafe {
            for batch in &mut self.batches {
                if let Some(release) = batch.release {
                    host(|| release(batch));
                }
            }
            if let Some(release) = self.schema.release {
                host(|| release(&raw mut self.schema));
            }
            if let Some(stream) = self.stream.as_deref_mut()
                && let Some(release) = stream.release
            {
                host(|| release(stream));
            }
        }
    }
}

impl Drop for Imported {
    fn drop(&mut self) {
        if self.gated {
            gate::release(|| self.release_all());
        } else {
            self.release_all();
        }
    }
}

/// The stream's own error phrase, read inside readable memory, or `context`.
///
/// # Safety
/// `stream` is a live stream.
unsafe fn last_error(stream: *mut ArrowArrayStream, context: &str) -> String {
    // SAFETY: the caller's live stream.
    let text = unsafe {
        (*stream)
            .get_last_error
            .map_or(ptr::null(), |last| host(|| last(stream)))
    };
    Readable::snapshot()
        .ok()
        .and_then(|memory| read::c_text(&memory, text).ok().flatten())
        .and_then(|found| String::from_utf8(found).ok())
        .unwrap_or_else(|| context.to_owned())
}

/// One of the caller's columns, aliased into an output batch: the producer's
/// child struct with this batch's rows, and the value that keeps it alive.
#[derive(Debug)]
pub(crate) struct Alias {
    pub(super) array: ArrowArray,
    pub(super) hold: Arc<Imported>,
}

impl Alias {
    /// The producer's column struct, cut to `length` rows from `offset`.
    pub(super) fn new(column: ArrowArray, offset: i64, length: i64, hold: &Arc<Imported>) -> Self {
        Self {
            array: ArrowArray {
                offset,
                length,
                release: None,
                private_data: ptr::null_mut(),
                ..column
            },
            hold: Arc::clone(hold),
        }
    }
}

// SAFETY: the struct's pointers name the producer's memory, which `hold`
// keeps alive and `Imported`'s own rule lets any thread hold.
unsafe impl Send for Alias {}

/// True when every page from `first` to `last` is mapped (off Linux).
#[cfg(all(unix, not(target_os = "linux")))]
pub(super) fn mapped(first: usize, last: usize) -> bool {
    unsafe extern "C" {
        fn mincore(address: *mut c_void, length: usize, pages: *mut u8) -> c_int;
        fn getpagesize() -> c_int;
    }
    // SAFETY: getpagesize takes no argument and cannot fail.
    let page = usize::try_from(unsafe { getpagesize() })
        .unwrap_or(4096)
        .max(1);
    let base = first - first % page;
    let span = last - base + 1;
    let mut pages = vec![0_u8; span.div_ceil(page)];
    // SAFETY: mincore reads no byte of the range. It writes one byte a page
    // into `pages`, which holds a byte for every page asked about.
    unsafe {
        mincore(
            ptr::with_exposed_provenance_mut(base),
            span,
            pages.as_mut_ptr(),
        ) == 0
    }
}

#[cfg(windows)]
#[path = "windows/ffi.rs"]
mod windows;
#[cfg(windows)]
pub(super) use windows::mapped;

#[cfg(test)]
mod tests;

/// Memory with unreadable neighbors, for the reader's unit tests.
#[cfg(test)]
#[cfg(target_os = "linux")]
pub(super) mod testing {
    use std::ffi::{c_char, c_int, c_void};
    use std::os::fd::{AsRawFd, FromRawFd};

    unsafe extern "C" {
        fn mmap(
            address: *mut c_void,
            length: usize,
            protect: c_int,
            flags: c_int,
            fd: c_int,
            offset: i64,
        ) -> *mut c_void;
        fn mprotect(address: *mut c_void, length: usize, protect: c_int) -> c_int;
        fn munmap(address: *mut c_void, length: usize) -> c_int;
        fn memfd_create(name: *const c_char, flags: u32) -> c_int;
    }

    const PAGE: usize = 4096;

    /// One readable page between an unreadable page and a 64 MiB unreadable
    /// reservation, so a read before or past the page faults on every run.
    #[derive(Debug)]
    pub(in super::super) struct Guarded {
        base: *mut u8,
        size: usize,
    }

    impl Guarded {
        pub(in super::super) const PAGE: usize = PAGE;

        pub(in super::super) fn new() -> Self {
            let size = 2 * PAGE + (64 << 20);
            // SAFETY: a fresh PROT_NONE private anonymous reservation, then
            // PROT_READ | PROT_WRITE on page one.
            let base =
                unsafe { mmap(std::ptr::null_mut(), size, 0, 0x02 | 0x20, -1, 0) }.cast::<u8>();
            assert!(
                !base.is_null() && base.addr() != usize::MAX,
                "the reservation maps"
            );
            assert_eq!(
                unsafe { mprotect(base.wrapping_add(PAGE).cast(), PAGE, 0x1 | 0x2) },
                0
            );
            Self { base, size }
        }

        /// `bytes` copied into the readable page at `offset`.
        pub(in super::super) fn place(&self, offset: usize, bytes: &[u8]) -> *const u8 {
            assert!(offset + bytes.len() <= PAGE);
            let at = self.base.wrapping_add(PAGE + offset);
            // SAFETY: inside the readable, writable page.
            unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), at, bytes.len()) };
            at
        }

        /// `bytes` ending exactly at the readable page's end.
        pub(in super::super) fn ending_with(bytes: &[u8]) -> (Self, *const u8) {
            let region = Self::new();
            let at = region.place(PAGE - bytes.len(), bytes);
            (region, at)
        }
    }

    impl Drop for Guarded {
        fn drop(&mut self) {
            // SAFETY: the reservation this value mapped.
            unsafe { munmap(self.base.cast(), self.size) };
        }
    }

    /// Two pages mapped shared over a one-page file, with `tail` at the end
    /// of the first page: the second page is listed readable, and touching
    /// it raises SIGBUS.
    #[derive(Debug)]
    pub(in super::super) struct PastEnd {
        base: *mut u8,
        _file: std::fs::File,
        path: Option<std::path::PathBuf>,
    }

    impl PastEnd {
        fn new(file: std::fs::File, tail: &[u8]) -> Self {
            file.set_len(PAGE as u64).expect("the file sizes");
            // SAFETY: PROT_READ | PROT_WRITE, MAP_SHARED over an open file.
            let base = unsafe {
                mmap(
                    std::ptr::null_mut(),
                    2 * PAGE,
                    0x1 | 0x2,
                    0x01,
                    file.as_raw_fd(),
                    0,
                )
            }
            .cast::<u8>();
            assert!(
                !base.is_null() && base.addr() != usize::MAX,
                "the file maps"
            );
            let made = Self {
                base,
                _file: file,
                path: None,
            };
            made.put(tail);
            made
        }

        /// The three backings the check tells apart: a named file, a file
        /// deleted after it mapped, and a memfd.
        pub(in super::super) fn each(tail: &[u8]) -> Vec<(&'static str, Self)> {
            let path =
                std::env::temp_dir().join(format!("thinkthen-past-end-{}", std::process::id()));
            let open = |path: &std::path::Path| {
                std::fs::File::options()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                    .expect("the file opens")
            };
            let mut named = Self::new(open(&path), tail);
            named.path = Some(path.clone());
            let gone_path = path.with_extension("gone");
            let gone = Self::new(open(&gone_path), tail);
            std::fs::remove_file(&gone_path).expect("the file deletes");
            // SAFETY: a fresh memfd, owned by the File from here.
            let fd = unsafe { memfd_create(c"thinkthen-past-end".as_ptr(), 0) };
            assert!(fd >= 0, "the memfd opens");
            let memfd = Self::new(unsafe { std::fs::File::from_raw_fd(fd) }, tail);
            vec![("named", named), ("deleted", gone), ("memfd", memfd)]
        }

        /// Write `bytes` so they end at the file's end, and give their start.
        pub(in super::super) fn put(&self, bytes: &[u8]) -> *const u8 {
            let at = self.base.wrapping_add(PAGE - bytes.len());
            // SAFETY: inside the file's own page.
            unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), at, bytes.len()) };
            at
        }

        pub(in super::super) fn tail(&self, length: usize) -> *const u8 {
            self.base.wrapping_add(PAGE - length)
        }

        pub(in super::super) fn past(&self) -> *const u8 {
            self.base.wrapping_add(PAGE)
        }
    }

    impl Drop for PastEnd {
        fn drop(&mut self) {
            // SAFETY: the mapping this value made.
            unsafe { munmap(self.base.cast(), 2 * PAGE) };
            if let Some(path) = &self.path {
                let _ = std::fs::remove_file(path);
            }
        }
    }
}
