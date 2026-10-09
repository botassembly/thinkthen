//! Immediate failures use fixed safe messages in a separate calling-thread slot.
use crate::ffi::values as abi;
use std::cell::Cell;
use std::ffi::{CStr, c_char};
use thinkthen::{ErrorKind, contained};

thread_local! {
    static LAST: Cell<&'static CStr> = const { Cell::new(c"no session failure yet") };
}

pub(crate) fn fail(kind: ErrorKind) -> i32 {
    let (code, message) = match kind {
        ErrorKind::Usage => (abi::THINKTHEN_EUSAGE, c"invalid session arguments or input"),
        ErrorKind::Backend => (
            abi::THINKTHEN_EBACKEND,
            c"session backend refused the operation",
        ),
        ErrorKind::Deadline => (abi::THINKTHEN_EDEADLINE, c"session deadline expired"),
        ErrorKind::Local => (abi::THINKTHEN_ELOCAL, c"session local operation failed"),
        ErrorKind::Cancelled => (abi::THINKTHEN_ECANCELLED, c"session operation cancelled"),
        ErrorKind::Defect => (
            abi::THINKTHEN_EDEFECT,
            c"defect: a session operation failed",
        ),
    };
    let _ = LAST.try_with(|slot| slot.set(message));
    code
}

pub(crate) fn call(body: impl FnOnce() -> Result<(), ErrorKind>) -> i32 {
    guard(abi::THINKTHEN_EDEFECT, || match body() {
        Ok(()) => abi::THINKTHEN_OK,
        Err(kind) => fail(kind),
    })
}

pub(crate) fn guard<T>(fallback: T, body: impl FnOnce() -> T) -> T {
    contained(body).unwrap_or_else(|| {
        fail(ErrorKind::Defect);
        fallback
    })
}

pub(crate) fn message() -> *const c_char {
    LAST.try_with(|slot| slot.get().as_ptr())
        .unwrap_or(c"no session failure yet".as_ptr())
}
