//! Immediate failures own safe messages in a separate calling-thread slot.
use crate::ffi::values as abi;
use std::cell::RefCell;
use std::ffi::{CStr, CString, c_char};
use thinkthen::{Error, ErrorKind, contained};

enum Message {
    Fixed(&'static CStr),
    Native(CString),
}
impl Message {
    fn as_ptr(&self) -> *const c_char {
        match self {
            Self::Fixed(message) => message.as_ptr(),
            Self::Native(message) => message.as_ptr(),
        }
    }
}
thread_local! {
    static LAST: RefCell<Message> = const { RefCell::new(Message::Fixed(c"no session failure yet")) };
}

pub(crate) enum Failure {
    Kind(ErrorKind),
    Native(Error),
}
impl From<ErrorKind> for Failure {
    fn from(kind: ErrorKind) -> Self {
        Self::Kind(kind)
    }
}

fn fail(failure: Failure) -> i32 {
    let kind = match &failure {
        Failure::Kind(kind) => *kind,
        Failure::Native(error) => error.kind(),
    };
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
    let message = match failure {
        Failure::Kind(_) => Message::Fixed(message),
        Failure::Native(error) => {
            CString::new(error.to_string()).map_or(Message::Fixed(message), Message::Native)
        }
    };
    let _ = LAST.try_with(|slot| {
        if let Ok(mut slot) = slot.try_borrow_mut() {
            *slot = message;
        }
    });
    code
}

pub(crate) fn call(body: impl FnOnce() -> Result<(), ErrorKind>) -> i32 {
    native_call(|| body().map_err(Failure::Kind))
}

pub(crate) fn native_call(body: impl FnOnce() -> Result<(), Failure>) -> i32 {
    guard(abi::THINKTHEN_EDEFECT, || match body() {
        Ok(()) => abi::THINKTHEN_OK,
        Err(kind) => fail(kind),
    })
}

pub(crate) fn guard<T>(fallback: T, body: impl FnOnce() -> T) -> T {
    contained(body).unwrap_or_else(|| {
        fail(Failure::Kind(ErrorKind::Defect));
        fallback
    })
}

pub(crate) fn message() -> *const c_char {
    LAST.try_with(|slot| {
        slot.try_borrow()
            .map_or(c"defect: a session operation failed".as_ptr(), |message| {
                message.as_ptr()
            })
    })
    .unwrap_or(c"no session failure yet".as_ptr())
}
