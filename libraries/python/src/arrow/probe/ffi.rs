//! A producer whose releases assume the interpreter lock, as a C producer
//! may, and abort when it is not held (change 6). Built only under `probe`.

use std::ffi::{c_int, c_void};
use std::ptr;

use pyo3::ffi;
use pyo3::prelude::*;

use super::ffi::{ArrowArray, ArrowArrayStream, ArrowSchema, EMPTY_ARRAY, EMPTY_SCHEMA, STREAM};
use super::out::{capsule, stream_destructor};

unsafe extern "C" {
    // In CPython's full API, not in the limited API an abi3 build sees.
    fn PyGILState_Check() -> c_int;
}

#[derive(Debug)]
struct State {
    remaining: usize,
    token: *mut ffi::PyObject,
    table: [*const c_void; 3],
    #[cfg(test)]
    diagnostic: bool,
}

unsafe extern "C" fn schema_release(schema: *mut ArrowSchema) {
    // SAFETY: the consumer's schema, which this producer filled.
    unsafe { (*schema).release = None };
}

unsafe extern "C" fn get_schema(_stream: *mut ArrowArrayStream, out: *mut ArrowSchema) -> c_int {
    #[cfg(test)]
    unsafe {
        let state = &*(*_stream).private_data.cast::<State>();
        if state.diagnostic {
            let stopped = std::panic::catch_unwind(|| -> () { panic!("arrow-ingress-marker") });
            if let Err(payload) = stopped {
                std::mem::forget(payload);
            }
        }
    }
    // SAFETY: `out` is the consumer's writable struct.
    unsafe {
        *out = ArrowSchema {
            format: c"u".as_ptr(),
            release: Some(schema_release),
            ..EMPTY_SCHEMA
        };
    }
    0
}

/// Abort unless the lock is held, then drop one reference to the token.
unsafe fn locked_drop(token: *mut c_void) {
    // SAFETY: every CPython this module loads into exports the symbol,
    // and `token` holds one strong reference taken in `get_next`.
    unsafe {
        if PyGILState_Check() == 0 {
            std::process::abort();
        }
        ffi::Py_DecRef(token.cast());
    }
}

unsafe extern "C" fn array_release(array: *mut ArrowArray) {
    // SAFETY: a batch this producer made.
    unsafe {
        locked_drop((*array).private_data);
        (*array).release = None;
    }
}

#[cfg(test)]
unsafe extern "C" fn diagnostic_array_release(array: *mut ArrowArray) {
    let stopped = std::panic::catch_unwind(|| -> () { panic!("arrow-release-marker") });
    if let Err(payload) = stopped {
        std::mem::forget(payload);
    }
    // SAFETY: the batch is the one this producer issued with one token ref.
    unsafe { array_release(array) };
}

/// Each batch is one row, the text "x".
static OFFSETS: [i32; 2] = [0, 1];
static VALUES: [u8; 1] = *b"x";

unsafe extern "C" fn get_next(stream: *mut ArrowArrayStream, out: *mut ArrowArray) -> c_int {
    // SAFETY: the stream's private data is this producer's `State`, and
    // `get_next` runs on the consumer's attached thread.
    unsafe {
        let state = &mut *(*stream).private_data.cast::<State>();
        let mut array = EMPTY_ARRAY;
        if state.remaining > 0 {
            state.remaining -= 1;
            ffi::Py_IncRef(state.token);
            array = ArrowArray {
                length: 1,
                n_buffers: 3,
                buffers: state.table.as_mut_ptr(),
                private_data: state.token.cast(),
                release: Some({
                    #[cfg(test)]
                    if state.diagnostic {
                        diagnostic_array_release
                    } else {
                        array_release
                    }
                    #[cfg(not(test))]
                    array_release
                }),
                ..EMPTY_ARRAY
            };
        }
        *out = array;
    }
    0
}

unsafe extern "C" fn stream_release(stream: *mut ArrowArrayStream) {
    // SAFETY: the same lock rule as a batch. The state is taken once.
    unsafe {
        let state = Box::from_raw((*stream).private_data.cast::<State>());
        locked_drop(state.token.cast());
        (*stream).release = None;
    }
}

/// A stream of `batches` one-row batches whose releases need the lock.
#[pyfunction]
pub(crate) fn _raw_producer(
    py: Python<'_>,
    batches: usize,
    token: Py<PyAny>,
) -> PyResult<Py<PyAny>> {
    let state = Box::new(State {
        remaining: batches,
        token: token.into_ptr(),
        table: [ptr::null(), OFFSETS.as_ptr().cast(), VALUES.as_ptr().cast()],
        #[cfg(test)]
        diagnostic: false,
    });
    let stream = Box::new(ArrowArrayStream {
        get_schema: Some(get_schema),
        get_next: Some(get_next),
        get_last_error: None,
        release: Some(stream_release),
        private_data: Box::into_raw(state).cast(),
    });
    capsule(py, Box::into_raw(stream).cast(), STREAM, stream_destructor)
}

#[cfg(test)]
#[path = "diagnostics_tests.rs"]
mod diagnostics_tests;
