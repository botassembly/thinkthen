//! Copy a file's access list on Windows, where the standard library has no call
//! for it. This is the one module of this crate allowed `unsafe`.

use std::fs::{File, OpenOptions};
use std::io;
use std::os::windows::fs::OpenOptionsExt;
use std::os::windows::io::AsRawHandle;
use std::path::Path;
use std::{ptr, slice};
use windows_sys::Win32::Foundation::{
    ERROR_INVALID_FUNCTION, ERROR_NOT_SUPPORTED, ERROR_SUCCESS, LocalFree,
};
use windows_sys::Win32::Security::Authorization::{
    GetSecurityInfo, SE_FILE_OBJECT, SetSecurityInfo,
};
use windows_sys::Win32::Security::{
    ACE_HEADER, ACL, DACL_SECURITY_INFORMATION, GetAce, GetSecurityDescriptorControl,
    INHERITED_ACE, OBJECT_SECURITY_INFORMATION, PROTECTED_DACL_SECURITY_INFORMATION,
    PSECURITY_DESCRIPTOR, SE_DACL_PROTECTED, UNPROTECTED_DACL_SECURITY_INFORMATION,
};
use windows_sys::Win32::Storage::FileSystem::{
    DELETE, FILE_WRITE_ATTRIBUTES, READ_CONTROL, WRITE_DAC,
};

/// One file's access list, held until every file made from it has it.
#[derive(Debug)]
pub(crate) struct AccessList {
    descriptor: PSECURITY_DESCRIPTOR,
    list: *mut ACL,
    inheritance: OBJECT_SECURITY_INFORMATION,
}

impl AccessList {
    /// Read the access list of an open file. A file on a drive with no access
    /// lists has none, so there is nothing to carry and nothing to widen.
    pub(crate) fn read(file: &File) -> io::Result<Option<Self>> {
        let mut list = ptr::null_mut();
        let mut descriptor = ptr::null_mut();
        // SAFETY: `file` keeps the handle open for the call. `list` and
        // `descriptor` are live locals the call writes, and the null pointers
        // decline the parts of the descriptor this does not ask for.
        let status = unsafe {
            GetSecurityInfo(
                file.as_raw_handle(),
                SE_FILE_OBJECT,
                DACL_SECURITY_INFORMATION,
                ptr::null_mut(),
                ptr::null_mut(),
                &raw mut list,
                ptr::null_mut(),
                &raw mut descriptor,
            )
        };
        if matches!(status, ERROR_NOT_SUPPORTED | ERROR_INVALID_FUNCTION) {
            return Ok(None);
        }
        checked(status)?;
        let mut held = Self {
            descriptor,
            list,
            inheritance: UNPROTECTED_DACL_SECURITY_INFORMATION,
        };
        if held.list.is_null() {
            return Ok(None);
        }
        let mut control = 0;
        let mut revision = 0;
        // SAFETY: `held.descriptor` is the live descriptor read above, and
        // `control` and `revision` are live locals the call writes.
        let read = unsafe {
            GetSecurityDescriptorControl(held.descriptor, &raw mut control, &raw mut revision)
        };
        if read == 0 {
            return Err(io::Error::last_os_error());
        }
        if control & SE_DACL_PROTECTED != 0 {
            held.inheritance = PROTECTED_DACL_SECURITY_INFORMATION;
        }
        Ok(Some(held))
    }

    /// Give the file at `path` this list, protected only if the source's is,
    /// so an ordinary file goes on following its folder. The owner stays as it is.
    ///
    /// A source moved in from another folder holds entries marked inherited
    /// that this folder would replace, so a list that doesn't read back the
    /// same is set again as protected. A file that still differs, or whose new
    /// list would block publishing it, is an error.
    pub(crate) fn apply(&self, path: &Path) -> io::Result<()> {
        let file = OpenOptions::new()
            .access_mode(READ_CONTROL | WRITE_DAC)
            .open(path)?;
        self.set(&file, self.inheritance)?;
        if !self.held_by(&file)? {
            self.set(&file, PROTECTED_DACL_SECURITY_INFORMATION)?;
            if !self.held_by(&file)? {
                return Err(io::Error::other(
                    "the source's access list could not be copied exactly",
                ));
            }
        }
        // Publishing sets attributes by path and renames, under the new list.
        OpenOptions::new()
            .access_mode(FILE_WRITE_ATTRIBUTES | DELETE)
            .open(path)
            .map(drop)
            .map_err(|error| {
                io::Error::new(
                    error.kind(),
                    format!("the source's access list would block replacing it: {error}"),
                )
            })
    }

    fn set(&self, file: &File, inheritance: OBJECT_SECURITY_INFORMATION) -> io::Result<()> {
        // SAFETY: `file` keeps the handle open for the call. `self.list` points
        // into `self.descriptor`, which lives until `self` drops, and the call
        // only reads it. The null pointers leave the owner, group and audit
        // list alone.
        let status = unsafe {
            SetSecurityInfo(
                file.as_raw_handle(),
                SE_FILE_OBJECT,
                DACL_SECURITY_INFORMATION | inheritance,
                ptr::null_mut(),
                ptr::null_mut(),
                self.list,
                ptr::null(),
            )
        };
        checked(status)
    }

    fn held_by(&self, file: &File) -> io::Result<bool> {
        match Self::read(file)? {
            Some(current) => Ok(entries(self.list)? == entries(current.list)?),
            None => Ok(false),
        }
    }
}

/// Each entry's bytes, without the mark saying it was inherited: a protected
/// list may clear that mark, and it grants nothing.
fn entries(list: *const ACL) -> io::Result<Vec<Vec<u8>>> {
    // SAFETY: both callers pass a non-null list inside a live descriptor.
    let count = unsafe { (*list).AceCount };
    (0..u32::from(count))
        .map(|index| {
            let mut entry = ptr::null_mut();
            // SAFETY: `list` is live, `index` is below its entry count, and
            // `entry` is a live local the call writes.
            if unsafe { GetAce(list, index, &raw mut entry) } == 0 {
                return Err(io::Error::last_os_error());
            }
            // SAFETY: `entry` points at a whole entry inside `list`, which
            // starts with its header, and that header's size counts the entry.
            let mut bytes = unsafe {
                let size = usize::from((*entry.cast::<ACE_HEADER>()).AceSize);
                slice::from_raw_parts(entry.cast::<u8>(), size).to_vec()
            };
            // The flags byte follows the type byte in every entry's header.
            if let Some(flags) = bytes.get_mut(1) {
                *flags &= !(INHERITED_ACE as u8);
            }
            Ok(bytes)
        })
        .collect()
}

impl Drop for AccessList {
    fn drop(&mut self) {
        // SAFETY: `GetSecurityInfo` allocated `descriptor` for the caller to
        // free with `LocalFree`, and only this drop frees it.
        unsafe {
            LocalFree(self.descriptor);
        }
    }
}

fn checked(status: u32) -> io::Result<()> {
    if status == ERROR_SUCCESS {
        Ok(())
    } else {
        Err(io::Error::from_raw_os_error(status.cast_signed()))
    }
}
