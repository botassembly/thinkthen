#![cfg(test)]
//! Memory-safety contract: a larger engine layout accepts old output buffers.
#![allow(
    unsafe_code,
    reason = "exercise foreign allocations at the public C boundary"
)]
use crate::memory::{HostError, HostRow};
use crate::*;
use std::{
    alloc::{Layout, alloc_zeroed, dealloc},
    mem::{align_of, size_of},
    ptr,
};

struct Slot {
    layout: Layout,
    data: *mut u8,
}
impl Slot {
    fn new<T>(prefix: usize, guard: u64) -> Self {
        let layout = Layout::from_size_align(prefix + size_of::<u64>(), align_of::<T>()).unwrap();
        assert!(layout.size() < size_of::<T>());
        // SAFETY: the allocator accepts this nonzero validated layout.
        let data = unsafe { alloc_zeroed(layout) };
        assert!(!data.is_null());
        // SAFETY: the guard occupies the final eight bytes of this allocation.
        unsafe { data.add(prefix).cast::<u64>().write_unaligned(guard) };
        Self { layout, data }
    }
    fn guard(&self) -> u64 {
        // SAFETY: this reads only the live allocation's final eight bytes.
        unsafe {
            self.data
                .add(self.layout.size() - size_of::<u64>())
                .cast::<u64>()
                .read_unaligned()
        }
    }
}
impl Drop for Slot {
    fn drop(&mut self) {
        // SAFETY: this uniquely owned allocation was created with the same layout.
        unsafe { dealloc(self.data, self.layout) };
    }
}

#[test]
fn old_prefixes_work_when_engine_structs_are_larger() {
    const GUARD: u64 = 0x1234_5678_9abc_def0;
    assert!(size_of::<HostError>() > HostError::PREFIX_SIZE as usize);
    assert!(size_of::<HostRow>() > HostRow::PREFIX_SIZE as usize);
    let errors = Slot::new::<HostError>(HostError::PREFIX_SIZE as usize, GUARD);
    let rows = Slot::new::<HostRow>(HostRow::PREFIX_SIZE as usize, GUARD);
    let error = errors.data.cast::<HostError>();
    let row = rows.data.cast::<HostRow>();
    // SAFETY: aligned allocations expose exactly the old prefix plus a guard;
    // never create a reference to the enlarged engine layout at either pointer.
    unsafe {
        ptr::addr_of_mut!((*error).struct_size).write(HostError::PREFIX_SIZE);
        ptr::addr_of_mut!((*row).struct_size).write(HostRow::PREFIX_SIZE);
        assert_eq!(thinkthen_settings_conversion_error(error), THINKTHEN_ERROR);
        assert!((*error).code != 0 && (*error).message.len > 0);
        thinkthen_error_free(error);
        let mut client = ptr::null_mut();
        let settings = br#"{"backend":"stub"}"#;
        assert_eq!(
            thinkthen_client_open_json(settings.as_ptr(), settings.len(), &mut client, error),
            THINKTHEN_OK
        );
        let mut call = ptr::null_mut();
        let request = br#"{"question":{"function":"decide","question":"Ready?"}}"#;
        assert_eq!(
            thinkthen_call_open_json(
                client,
                request.as_ptr(),
                request.len(),
                thinkthen::OffsetUnit::CodePoints.abi(),
                &mut call,
                error
            ),
            THINKTHEN_OK
        );
        assert_eq!(thinkthen_call_next(call, row, error), THINKTHEN_NEEDS_INPUT);
        assert_eq!(
            thinkthen_call_send_text(call, b"record".as_ptr(), 6, error),
            THINKTHEN_OK
        );
        thinkthen_call_end_input(call);
        assert_eq!(thinkthen_call_next(call, row, error), THINKTHEN_OK);
        assert_eq!((*row).position, 1);
        assert!((*row).source.len > 0);
        let mut value = ptr::null_mut();
        assert_eq!(thinkthen_row_values(row, &mut value, error), THINKTHEN_OK);
        thinkthen_value_free(value);
        thinkthen_call_free(call);
        thinkthen_client_free(client);
        thinkthen_row_free(row);
        thinkthen_error_free(error);
        assert_eq!((*row).struct_size, HostRow::PREFIX_SIZE);
        assert_eq!((*error).struct_size, HostError::PREFIX_SIZE);
    }
    assert_eq!(rows.guard(), GUARD);
    assert_eq!(errors.guard(), GUARD);
}
