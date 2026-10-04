//! Owned native file handles and creation-time protected descriptors.
#![cfg(windows)]
#![allow(
    unsafe_code,
    reason = "Windows no-follow handles and protected creation require native file APIs"
)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fs::File;
use std::io;
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt as _;
use std::os::windows::io::{AsRawHandle as _, FromRawHandle as _};
use std::path::Path;
use std::ptr;
use windows_sys::Win32::Foundation::{ERROR_ALREADY_EXISTS, GetLastError, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::{
    BY_HANDLE_FILE_INFORMATION, CREATE_NEW, CreateDirectoryW, CreateFileW,
    FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_REPARSE_POINT, FILE_FLAG_BACKUP_SEMANTICS,
    FILE_FLAG_OPEN_REPARSE_POINT, FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_ID_INFO,
    FILE_SHARE_READ, FILE_SHARE_WRITE, FileIdInfo, GetFileInformationByHandle,
    GetFileInformationByHandleEx, OPEN_ALWAYS, OPEN_EXISTING,
};

use super::super::security::ffi::Creation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Identity {
    volume: u64,
    id: [u8; 16],
}

fn name(path: &Path) -> io::Result<Vec<u16>> {
    let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    if wide.contains(&0) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "native path"));
    }
    wide.push(0);
    Ok(wide)
}

pub(super) fn open(
    path: &Path,
    writable: bool,
    create: bool,
    exclusive: bool,
) -> io::Result<(File, bool)> {
    let wide = name(path)?;
    let descriptor = if create {
        Some(Creation::new(false)?)
    } else {
        None
    };
    let access = FILE_GENERIC_READ | if writable { FILE_GENERIC_WRITE } else { 0 };
    let disposition = if exclusive {
        CREATE_NEW
    } else if create {
        OPEN_ALWAYS
    } else {
        OPEN_EXISTING
    };
    let flags = FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS;
    let call = |attributes: *const windows_sys::Win32::Security::SECURITY_ATTRIBUTES| {
        // SAFETY: terminated wide path and optional descriptor remain live for
        // the call. attributes is null or borrowed inside with_attributes.
        // The returned HANDLE transfers once below; no deletion sharing is granted.
        let handle = unsafe {
            CreateFileW(
                wide.as_ptr(),
                access,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                attributes,
                disposition,
                flags,
                ptr::null_mut(),
            )
        };
        // SAFETY: last-error is captured immediately after the native call.
        let error = unsafe { GetLastError() };
        (handle, error)
    };
    let (handle, error) = match &descriptor {
        Some(descriptor) => descriptor.with_attributes(|attributes| call(attributes)),
        None => call(ptr::null()),
    };
    if handle.is_null() || handle == INVALID_HANDLE_VALUE {
        return Err(io::Error::from_raw_os_error(error as i32));
    }
    // SAFETY: successful CreateFileW returned a unique owned valid HANDLE.
    // File now closes it exactly once, including every later error path.
    let file = unsafe { File::from_raw_handle(handle) };
    Ok((file, create && error != ERROR_ALREADY_EXISTS))
}

pub(super) fn create_directory(path: &Path) -> io::Result<()> {
    let wide = name(path)?;
    let descriptor = Creation::new(true)?;
    let (created, error) = descriptor.with_attributes(|attributes| {
        // SAFETY: path and protected descriptor live throughout creation; the
        // API borrows both and returns no owned handle or borrowed pointer.
        let created = unsafe { CreateDirectoryW(wide.as_ptr(), attributes) };
        // SAFETY: capture this call's thread-local native error immediately.
        (created, unsafe { GetLastError() })
    });
    if created != 0 || error == ERROR_ALREADY_EXISTS {
        Ok(())
    } else {
        Err(io::Error::from_raw_os_error(error as i32))
    }
}

pub(super) fn kind(file: &File, directory: bool) -> io::Result<()> {
    let mut information = BY_HANDLE_FILE_INFORMATION::default();
    // SAFETY: file owns the borrowed native handle; the output is a properly
    // aligned, writable instance with the API's exact required size.
    if unsafe { GetFileInformationByHandle(file.as_raw_handle(), &mut information) } == 0 {
        return Err(io::Error::last_os_error());
    }
    if information.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        return Err(super::super::permission());
    }
    if (information.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY != 0) != directory {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unsafe usage object",
        ));
    }
    Ok(())
}

pub(super) fn identity(file: &File) -> io::Result<Identity> {
    let mut information = FILE_ID_INFO::default();
    let size = u32::try_from(size_of::<FILE_ID_INFO>()).map_err(io::Error::other)?;
    // SAFETY: file retains its handle; output has native structure alignment,
    // exactly size bytes, and lives through the complete information call.
    if unsafe {
        GetFileInformationByHandleEx(
            file.as_raw_handle(),
            FileIdInfo,
            (&mut information as *mut FILE_ID_INFO).cast(),
            size,
        )
    } == 0
    {
        return Err(io::Error::last_os_error());
    }
    Ok(Identity {
        volume: information.VolumeSerialNumber,
        id: information.FileId.Identifier,
    })
}
