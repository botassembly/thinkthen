//! Only native thread ownership, pipe admission and synchronous cancellation.
#![cfg(windows)]
#![allow(
    unsafe_code,
    reason = "0455 owned stdio pipes require native synchronous I/O cancellation"
)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::io;
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use windows_sys::Win32::Foundation::{DUPLICATE_SAME_ACCESS, DuplicateHandle, ERROR_NOT_FOUND};
use windows_sys::Win32::Storage::FileSystem::{FILE_TYPE_PIPE, GetFileType};
use windows_sys::Win32::System::IO::{CancelIoEx, CancelSynchronousIo};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, GetCurrentThread};

pub(super) fn pipe(file: &std::fs::File) -> io::Result<()> {
    // SAFETY: the borrowed file keeps this handle valid throughout the query.
    if unsafe { GetFileType(file.as_raw_handle()) } != FILE_TYPE_PIPE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "MCP stdio requires pipes",
        ));
    }
    Ok(())
}

pub(super) fn current_thread() -> io::Result<OwnedHandle> {
    let mut handle = std::ptr::null_mut();
    // SAFETY: process/thread pseudo-handles belong to the calling thread. The
    // output points to initialized storage; the duplicate is not inheritable.
    let success = unsafe {
        let process = GetCurrentProcess();
        DuplicateHandle(
            process,
            GetCurrentThread(),
            process,
            &mut handle,
            0,
            0,
            DUPLICATE_SAME_ACCESS,
        )
    };
    if success == 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: successful duplication transferred one real owned handle here.
    Ok(unsafe { OwnedHandle::from_raw_handle(handle) })
}

pub(super) fn cancel(thread: &OwnedHandle, pipe: &std::fs::File) -> io::Result<()> {
    // SAFETY: the registry lock keeps the owned thread and exact pipe handle alive and confines
    // cancellation to the registered MCP operation. Rust subprocess pipes use
    // overlapped handles; inherited stdio can use synchronous handles. Cancel
    // both forms while the read/write still owns and waits for its buffer.
    let synchronous = unsafe { CancelSynchronousIo(thread.as_raw_handle()) };
    let synchronous_error = (synchronous == 0).then(io::Error::last_os_error);
    let overlapped = unsafe { CancelIoEx(pipe.as_raw_handle(), std::ptr::null()) };
    let overlapped_error = (overlapped == 0).then(io::Error::last_os_error);
    for error in [synchronous_error, overlapped_error].into_iter().flatten() {
        if error.raw_os_error() != Some(ERROR_NOT_FOUND as i32) {
            return Err(error);
        }
    }
    Ok(())
}
