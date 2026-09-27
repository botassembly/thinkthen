//! Small ownership and panic-boundary proof. This is not the product bridge.
use std::panic::{AssertUnwindSafe, catch_unwind};

#[repr(C)]
pub struct Reply {
    status: i32,
    bytes: *mut u8,
    len: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_probe_row(input: *const u8, len: usize) -> Reply {
    let result = catch_unwind(AssertUnwindSafe(|| {
        if input.is_null() {
            return Err(1);
        }
        // SAFETY: C++ passes a live DuckDB string for this call only.
        let bytes = unsafe { std::slice::from_raw_parts(input, len) };
        let text = std::str::from_utf8(bytes).map_err(|_| 2)?;
        if text == "panic" {
            panic!("bridge proof panic");
        }
        let mut encoded = Vec::new();
        for field in [
            text.as_bytes(),
            b"owned-by-rust".as_slice(),
            b"nested".as_slice(),
        ] {
            let size = u32::try_from(field.len()).map_err(|_| 3)?;
            encoded.extend_from_slice(&size.to_le_bytes());
            encoded.extend_from_slice(field);
        }
        Ok::<_, i32>(encoded)
    }));
    match result {
        Ok(Ok(bytes)) => {
            let mut boxed = bytes.into_boxed_slice();
            let reply = Reply {
                status: 0,
                bytes: boxed.as_mut_ptr(),
                len: boxed.len(),
            };
            std::mem::forget(boxed);
            reply
        }
        Ok(Err(status)) => Reply {
            status,
            bytes: std::ptr::null_mut(),
            len: 0,
        },
        Err(_) => Reply {
            status: 4,
            bytes: std::ptr::null_mut(),
            len: 0,
        },
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_probe_free(bytes: *mut u8, len: usize) {
    if !bytes.is_null() {
        // SAFETY: only a successful thinkthen_probe_row allocation is passed back once.
        unsafe {
            drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                bytes, len,
            )))
        }
    }
}
