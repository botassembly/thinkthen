//! Windows readable-memory inspection for the Arrow door.

use std::ptr;

/// True when every Windows region in the range is committed and readable.
pub(in crate::arrow) fn mapped(first: usize, last: usize) -> bool {
    use windows_sys::Win32::System::Memory::{
        MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE,
        PAGE_EXECUTE_WRITECOPY, PAGE_GUARD, PAGE_READONLY, PAGE_READWRITE, PAGE_WRITECOPY,
        VirtualQuery,
    };
    let mut at = first;
    loop {
        let mut info = MEMORY_BASIC_INFORMATION::default();
        // SAFETY: VirtualQuery inspects the address without dereferencing it and
        // writes only the complete information struct supplied here.
        let size = unsafe {
            VirtualQuery(
                ptr::with_exposed_provenance(at),
                &raw mut info,
                size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        };
        if size != size_of::<MEMORY_BASIC_INFORMATION>()
            || info.State != MEM_COMMIT
            || info.Protect & PAGE_GUARD != 0
            || !matches!(
                info.Protect & 0xff,
                PAGE_READONLY
                    | PAGE_READWRITE
                    | PAGE_WRITECOPY
                    | PAGE_EXECUTE_READ
                    | PAGE_EXECUTE_READWRITE
                    | PAGE_EXECUTE_WRITECOPY
            )
        {
            return false;
        }
        let Some(end) = (info.BaseAddress as usize).checked_add(info.RegionSize) else {
            return false;
        };
        if end <= at {
            return false;
        }
        if end > last {
            return true;
        }
        at = end;
    }
}
