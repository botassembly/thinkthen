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
use windows_sys::Win32::System::IO::CancelSynchronousIo;
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

pub(super) fn cancel(thread: &OwnedHandle) -> io::Result<()> {
    // SAFETY: the caller holds its operation registry lock, and this owned
    // handle lives until cancellation returns. No buffer pointers are retained.
    if unsafe { CancelSynchronousIo(thread.as_raw_handle()) } != 0 {
        return Ok(());
    }
    let error = io::Error::last_os_error();
    if error.raw_os_error() == Some(ERROR_NOT_FOUND as i32) {
        return Ok(()); // A read/write may not have entered the kernel yet.
    }
    Err(error)
}
