//! Native descriptor allocations and bounded borrowed views.
#![cfg(windows)]
#![allow(
    unsafe_code,
    reason = "Windows security descriptors require native allocation and bounded pointer views"
)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::ffi::c_void;
use std::fs::File;
use std::io;
use std::mem::{align_of, size_of, size_of_val};
use std::os::windows::io::{AsRawHandle as _, FromRawHandle as _, OwnedHandle};
use std::ptr;
use windows_sys::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, LocalFree};
use windows_sys::Win32::Security::Authorization::{
    ConvertSidToStringSidW, ConvertStringSecurityDescriptorToSecurityDescriptorW, GetSecurityInfo,
    SE_FILE_OBJECT,
};
use windows_sys::Win32::Security::{
    ACE_HEADER, ACL, DACL_SECURITY_INFORMATION, GetAce, GetSecurityDescriptorDacl,
    GetSecurityDescriptorLength, GetTokenInformation, IsValidAcl, IsValidSecurityDescriptor,
    IsValidSid, OWNER_SECURITY_INFORMATION, SECURITY_ATTRIBUTES, SID, TOKEN_QUERY, TOKEN_USER,
    TokenUser,
};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

const SYSTEM: &[u8] = &[1, 1, 0, 0, 0, 0, 0, 5, 18, 0, 0, 0];
// Data, append, extended attributes, attributes, delete, DACL, ownership,
// generic write and generic all. Generic execute/read alone permit no write.
const WRITES: u32 = 0x0000_0002
    | 0x0000_0004
    | 0x0000_0010
    | 0x0000_0100
    | 0x0001_0000
    | 0x0004_0000
    | 0x0008_0000
    | 0x4000_0000
    | 0x1000_0000;

#[derive(Debug)]
pub(super) struct Inspection {
    pub(super) owned: bool,
    pub(super) unrestricted: bool,
    pub(super) foreign_access: bool,
    pub(super) foreign_write: bool,
}

struct Allocation(*mut c_void);

impl Drop for Allocation {
    fn drop(&mut self) {
        // SAFETY: each nonnull allocation came from the LocalAlloc family and
        // transfers once to this owner. No borrowed view survives this drop.
        unsafe { LocalFree(self.0) };
    }
}

fn native(success: i32) -> io::Result<()> {
    if success == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn contains(base: *const c_void, size: usize, view: *const c_void, length: usize) -> bool {
    let start = base as usize;
    let address = view as usize;
    address >= start
        && address
            .checked_add(length)
            .zip(start.checked_add(size))
            .is_some_and(|(end, limit)| end <= limit)
}

fn sid(base: *const c_void, size: usize, value: *mut c_void) -> io::Result<Vec<u8>> {
    if value.is_null()
        || (value as usize) % align_of::<SID>() != 0
        || !contains(base, size, value, 8)
    {
        return Err(super::super::permission());
    }
    // SAFETY: the first eight SID bytes lie inside the live allocation. A byte
    // view needs no alignment. The count at offset one determines checked size.
    let prefix = unsafe { std::slice::from_raw_parts(value.cast::<u8>(), 8) };
    let length = 8usize
        .checked_add(usize::from(*prefix.get(1).ok_or_else(super::super::permission)?) * 4)
        .ok_or_else(super::super::permission)?;
    if !contains(base, size, value, length) {
        return Err(super::super::permission());
    }
    // SAFETY: the entire count-sized SID is bounded by its still-live owner.
    native(unsafe { IsValidSid(value) })?;
    // SAFETY: validation above bounds the immutable byte view; copying removes
    // every borrowed pointer before the allocation can be released.
    Ok(unsafe { std::slice::from_raw_parts(value.cast::<u8>(), length) }.to_vec())
}

struct User {
    _buffer: Vec<usize>,
    sid: *mut c_void,
    bytes: Vec<u8>,
}

impl User {
    fn current() -> io::Result<Self> {
        let mut handle = ptr::null_mut();
        // SAFETY: GetCurrentProcess is a borrowed pseudo-handle, never closed.
        // The out pointer is aligned and writable for one HANDLE.
        native(unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) })?;
        if handle.is_null() || handle == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {
            return Err(super::super::permission());
        }
        // SAFETY: successful OpenProcessToken returned one owned valid handle.
        // OwnedHandle now closes it exactly once, including every error return.
        let token = unsafe { OwnedHandle::from_raw_handle(handle) };
        let mut length = 0u32;
        // SAFETY: null size probe has no output buffer; length is writable.
        let queried = unsafe {
            GetTokenInformation(
                token.as_raw_handle(),
                TokenUser,
                ptr::null_mut(),
                0,
                &mut length,
            )
        };
        let error = io::Error::last_os_error();
        if queried != 0
            || error.raw_os_error() != Some(ERROR_INSUFFICIENT_BUFFER as i32)
            || usize::try_from(length).map_err(io::Error::other)? < size_of::<TOKEN_USER>()
        {
            return Err(super::super::permission());
        }
        let words = usize::try_from(length)
            .map_err(io::Error::other)?
            .div_ceil(size_of::<usize>());
        let mut buffer = vec![0usize; words];
        let capacity = u32::try_from(size_of_val(buffer.as_slice())).map_err(io::Error::other)?;
        // SAFETY: buffer has native pointer alignment and capacity bytes; the
        // token owner stays live throughout the call. The API writes at most capacity.
        native(unsafe {
            GetTokenInformation(
                token.as_raw_handle(),
                TokenUser,
                buffer.as_mut_ptr().cast(),
                capacity,
                &mut length,
            )
        })?;
        let size = usize::try_from(length).map_err(io::Error::other)?;
        if size < size_of::<TOKEN_USER>() || size > size_of_val(buffer.as_slice()) {
            return Err(super::super::permission());
        }
        // SAFETY: buffer contains at least TOKEN_USER bytes. Reading unaligned
        // also covers targets whose structure alignment exceeds usize alignment.
        let user = unsafe { ptr::read_unaligned(buffer.as_ptr().cast::<TOKEN_USER>()) };
        let bytes = sid(buffer.as_ptr().cast(), size, user.User.Sid)?;
        Ok(Self {
            _buffer: buffer,
            sid: user.User.Sid,
            bytes,
        })
    }

    fn text(&self) -> io::Result<String> {
        let mut value = ptr::null_mut();
        // SAFETY: SID was validated and lives in self.buffer for the complete call.
        // The API transfers a LocalFree-owned UTF-16 allocation through value.
        native(unsafe { ConvertSidToStringSidW(self.sid, &mut value) })?;
        if value.is_null() {
            return Err(super::super::permission());
        }
        let allocation = Allocation(value.cast());
        // A Windows SID string has at most 184 characters (15 subauthorities).
        // SAFETY: the API guarantees a terminated SID string; scan only until its
        // terminator. The validated SID bounds its documented maximum length.
        let mut units = Vec::new();
        for offset in 0..185 {
            let unit = unsafe { *value.add(offset) };
            if unit == 0 {
                drop(allocation);
                return String::from_utf16(&units).map_err(io::Error::other);
            }
            units.push(unit);
        }
        Err(super::super::permission())
    }
}

pub(super) fn inspect(file: &File) -> io::Result<Inspection> {
    let user = User::current()?;
    let mut owner = ptr::null_mut();
    let mut descriptor = ptr::null_mut();
    // SAFETY: file owns the borrowed HANDLE through this call; native out
    // pointers are aligned and writable. GetSecurityInfo transfers its descriptor.
    let code = unsafe {
        GetSecurityInfo(
            file.as_raw_handle(),
            SE_FILE_OBJECT,
            OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
            &mut owner,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            &mut descriptor,
        )
    };
    let allocation = Allocation(descriptor);
    if code != 0 {
        return Err(io::Error::from_raw_os_error(code as i32));
    }
    if descriptor.is_null() {
        return Err(super::super::permission());
    }
    // SAFETY: successful native query guarantees a descriptor allocation.
    native(unsafe { IsValidSecurityDescriptor(descriptor) })?;
    // SAFETY: valid native descriptor remains owned by allocation.
    let length = usize::try_from(unsafe { GetSecurityDescriptorLength(descriptor) })
        .map_err(io::Error::other)?;
    let owned = sid(descriptor, length, owner)? == user.bytes;
    let mut present = 0;
    let mut defaulted = 0;
    let mut acl = ptr::null_mut();
    // SAFETY: valid descriptor and writable out pointers live through the call.
    native(unsafe {
        GetSecurityDescriptorDacl(descriptor, &mut present, &mut acl, &mut defaulted)
    })?;
    let mut result = Inspection {
        owned,
        unrestricted: present == 0 || acl.is_null(),
        foreign_access: false,
        foreign_write: false,
    };
    if !result.unrestricted {
        inspect_acl(&allocation, length, acl, &user, &mut result)?;
    }
    Ok(result)
}

fn inspect_acl(
    allocation: &Allocation,
    length: usize,
    acl: *mut ACL,
    user: &User,
    result: &mut Inspection,
) -> io::Result<()> {
    if (acl as usize) % align_of::<ACL>() != 0
        || !contains(allocation.0, length, acl.cast(), size_of::<ACL>())
    {
        return Err(super::super::permission());
    }
    // SAFETY: header is bounded. Unaligned read avoids assuming the descriptor offset alignment.
    let header = unsafe { ptr::read_unaligned(acl) };
    let size = usize::from(header.AclSize);
    if size < size_of::<ACL>() || !contains(allocation.0, length, acl.cast(), size) {
        return Err(super::super::permission());
    }
    // SAFETY: the complete ACL lies in the owned valid descriptor.
    native(unsafe { IsValidAcl(acl) })?;
    for index in 0..header.AceCount {
        let mut ace = ptr::null_mut();
        // SAFETY: validated ACL and in-range index; ace is a writable out pointer.
        native(unsafe { GetAce(acl, u32::from(index), &mut ace) })?;
        inspect_ace(acl, size, ace, user, result)?;
    }
    Ok(())
}

fn inspect_ace(
    acl: *const ACL,
    size: usize,
    ace: *mut c_void,
    user: &User,
    result: &mut Inspection,
) -> io::Result<()> {
    if !contains(acl.cast(), size, ace, size_of::<ACE_HEADER>()) {
        return Err(super::super::permission());
    }
    // SAFETY: the header lies inside the checked ACL allocation.
    let header = unsafe { ptr::read_unaligned(ace.cast::<ACE_HEADER>()) };
    let length = usize::from(header.AceSize);
    if length < size_of::<ACE_HEADER>() || !contains(acl.cast(), size, ace, length) {
        return Err(super::super::permission());
    }
    match header.AceType {
        // Deny and audit ACEs confer no access. They do not cancel an allow.
        1 | 2 | 3 | 6 | 7 | 8 | 10 | 12 | 13 | 14 | 15 | 16 => return Ok(()),
        0 => {}
        // Object, compound, callback and unknown allow semantics are conservative.
        _ => {
            result.foreign_access = true;
            result.foreign_write |= header.AceFlags & 0x08 == 0;
            return Ok(());
        }
    }
    if length < 16 {
        return Err(super::super::permission());
    }
    // SAFETY: normal allow ACE layout is mask at byte four and SID at byte
    // eight. Both are within this validated ACE; reads require no alignment.
    let mask = unsafe { ptr::read_unaligned(ace.cast::<u8>().add(4).cast::<u32>()) };
    let value = unsafe { ace.cast::<u8>().add(8).cast() };
    let principal = sid(ace, length, value)?;
    if principal != user.bytes && principal != SYSTEM {
        result.foreign_access |= mask != 0;
        result.foreign_write |= header.AceFlags & 0x08 == 0 && mask & WRITES != 0;
    }
    Ok(())
}

pub(in crate::windows) struct Creation {
    allocation: Allocation,
    attributes_length: u32,
}

impl Creation {
    pub(in crate::windows) fn new(directory: bool) -> io::Result<Self> {
        let user = User::current()?;
        let identity = user.text()?;
        let inherit = if directory { "OICI" } else { "" };
        let text = format!("O:{identity}D:P(A;{inherit};FA;;;{identity})(A;{inherit};FA;;;SY)");
        let wide: Vec<u16> = text.encode_utf16().chain(Some(0)).collect();
        let mut descriptor = ptr::null_mut();
        // SAFETY: terminated UTF-16 input lives throughout conversion; the API
        // transfers a LocalFree-owned descriptor through the aligned out pointer.
        native(unsafe {
            ConvertStringSecurityDescriptorToSecurityDescriptorW(
                wide.as_ptr(),
                1,
                &mut descriptor,
                ptr::null_mut(),
            )
        })?;
        if descriptor.is_null() {
            return Err(super::super::permission());
        }
        Ok(Self {
            allocation: Allocation(descriptor),
            attributes_length: u32::try_from(size_of::<SECURITY_ATTRIBUTES>())
                .map_err(io::Error::other)?,
        })
    }

    pub(in crate::windows) fn with_attributes<T>(
        &self,
        operation: impl FnOnce(&SECURITY_ATTRIBUTES) -> T,
    ) -> T {
        let attributes = SECURITY_ATTRIBUTES {
            nLength: self.attributes_length,
            lpSecurityDescriptor: self.allocation.0,
            bInheritHandle: 0,
        };
        operation(&attributes)
    }
}
