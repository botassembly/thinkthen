//! Copy a file's access list on Windows, where the standard library has no call
//! for it. This is the one module of this crate allowed `unsafe`.

use std::fs::{File, OpenOptions};
use std::io;
use std::os::windows::fs::OpenOptionsExt;
use std::os::windows::io::AsRawHandle;
use std::path::Path;
use std::ptr;
#[cfg(feature = "command")]
use std::slice;
#[cfg(feature = "command")]
use windows_sys::Win32::Foundation::{ERROR_INVALID_FUNCTION, ERROR_NOT_SUPPORTED};
use windows_sys::Win32::Foundation::{ERROR_SUCCESS, LocalFree};
use windows_sys::Win32::Security::Authorization::{
    GetSecurityInfo, SE_FILE_OBJECT, SetSecurityInfo,
};
use windows_sys::Win32::Security::{
    ACE_HEADER, ACL, DACL_SECURITY_INFORMATION, GetAce, PROTECTED_DACL_SECURITY_INFORMATION,
};
#[cfg(feature = "command")]
use windows_sys::Win32::Security::{
    GetSecurityDescriptorControl, INHERITED_ACE, OBJECT_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR,
    SE_DACL_PROTECTED, UNPROTECTED_DACL_SECURITY_INFORMATION,
};
#[cfg(feature = "command")]
use windows_sys::Win32::Storage::FileSystem::{DELETE, FILE_WRITE_ATTRIBUTES};
use windows_sys::Win32::Storage::FileSystem::{READ_CONTROL, WRITE_DAC};

/// One file's access list, held until every file made from it has it.
#[derive(Debug)]
pub(crate) struct AccessList {
    #[cfg(feature = "command")]
    descriptor: PSECURITY_DESCRIPTOR,
    #[cfg(feature = "command")]
    list: *mut ACL,
    #[cfg(feature = "command")]
    inheritance: OBJECT_SECURITY_INFORMATION,
}

impl AccessList {
    /// Restrict a newly created empty file to its owner before any content is written.
    pub(crate) fn private(original: &File, path: &Path) -> io::Result<()> {
        let file = OpenOptions::new()
            .access_mode(READ_CONTROL | WRITE_DAC)
            .custom_flags(
                windows_sys::Win32::Storage::FileSystem::FILE_FLAG_BACKUP_SEMANTICS
                    | windows_sys::Win32::Storage::FileSystem::FILE_FLAG_OPEN_REPARSE_POINT,
            )
            .open(path)?;
        if !Self::same_file(original, &file)? {
            return Err(io::Error::other("private file path was replaced"));
        }
        use windows_sys::Win32::Security::Authorization::{
            EXPLICIT_ACCESS_W, NO_MULTIPLE_TRUSTEE, SET_ACCESS, SetEntriesInAclW, TRUSTEE_IS_SID,
            TRUSTEE_IS_USER, TRUSTEE_W,
        };
        use windows_sys::Win32::Security::{NO_INHERITANCE, OWNER_SECURITY_INFORMATION};
        use windows_sys::Win32::Storage::FileSystem::FILE_ALL_ACCESS;
        let mut owner = ptr::null_mut();
        let mut descriptor = ptr::null_mut();
        // SAFETY: the open file and writable output pointers outlive this call.
        let status = unsafe {
            GetSecurityInfo(
                file.as_raw_handle(),
                SE_FILE_OBJECT,
                OWNER_SECURITY_INFORMATION,
                &raw mut owner,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                &raw mut descriptor,
            )
        };
        checked(status)?;
        if owner.is_null() {
            // SAFETY: GetSecurityInfo allocated this descriptor for LocalFree.
            unsafe {
                LocalFree(descriptor);
            }
            return Err(io::Error::other(
                "file owner is unavailable for private recovery access",
            ));
        }
        let entry = EXPLICIT_ACCESS_W {
            grfAccessPermissions: FILE_ALL_ACCESS,
            grfAccessMode: SET_ACCESS,
            grfInheritance: if file.metadata()?.is_dir() {
                windows_sys::Win32::Security::OBJECT_INHERIT_ACE
                    | windows_sys::Win32::Security::CONTAINER_INHERIT_ACE
            } else {
                NO_INHERITANCE
            },
            Trustee: TRUSTEE_W {
                pMultipleTrustee: ptr::null_mut(),
                MultipleTrusteeOperation: NO_MULTIPLE_TRUSTEE,
                TrusteeForm: TRUSTEE_IS_SID,
                TrusteeType: TRUSTEE_IS_USER,
                ptstrName: owner.cast(),
            },
        };
        let mut list = ptr::null_mut();
        // SAFETY: entry contains the SID in the live descriptor; list is a writable output.
        let status = unsafe { SetEntriesInAclW(1, &entry, ptr::null(), &raw mut list) };
        let result = checked(status).and_then(|()| {
            // SAFETY: list is the allocated ACL, and file remains open throughout.
            checked(unsafe {
                SetSecurityInfo(
                    file.as_raw_handle(),
                    SE_FILE_OBJECT,
                    DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    list,
                    ptr::null(),
                )
            })
        });
        // SAFETY: both allocations were returned by Windows and are caller-owned.
        unsafe {
            LocalFree(list.cast());
            LocalFree(descriptor);
        }
        result
    }

    /// Compare open file identities without following a new path.
    pub(crate) fn same_file(left: &File, right: &File) -> io::Result<bool> {
        use windows_sys::Win32::Storage::FileSystem::{
            BY_HANDLE_FILE_INFORMATION, GetFileInformationByHandle,
        };
        fn identity(file: &File) -> io::Result<(u32, u32, u32)> {
            let mut info = BY_HANDLE_FILE_INFORMATION::default();
            // SAFETY: file remains open and info is a writable complete output object.
            if unsafe { GetFileInformationByHandle(file.as_raw_handle(), &raw mut info) } == 0 {
                return Err(io::Error::last_os_error());
            }
            Ok((
                info.dwVolumeSerialNumber,
                info.nFileIndexHigh,
                info.nFileIndexLow,
            ))
        }
        Ok(identity(left)? == identity(right)?)
    }

    /// Read the access list of an open file. A file on a drive with no access
    /// lists has none, so there is nothing to carry and nothing to widen.
    #[cfg(feature = "command")]
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
    #[cfg(feature = "command")]
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

    #[cfg(feature = "command")]
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

    #[cfg(feature = "command")]
    fn held_by(&self, file: &File) -> io::Result<bool> {
        match Self::read(file)? {
            Some(current) => Ok(entries(self.list)? == entries(current.list)?),
            None => Ok(false),
        }
    }
}

/// Each entry's bytes, without the mark saying it was inherited: a protected
/// list may clear that mark, and it grants nothing.
#[cfg(feature = "command")]
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

#[cfg(feature = "command")]
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
