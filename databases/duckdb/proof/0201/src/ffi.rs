//! Small ownership and panic-boundary proof. This is not the product bridge.
use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Once;

static INSTALL_HOOK: Once = Once::new();
thread_local! {
    static BRIDGE_DEPTH: Cell<usize> = const { Cell::new(0) };
}

// Install once, never swap hooks around individual calls. A panic on another
// thread still reaches the host's pre-existing hook.
fn install_hook() {
    INSTALL_HOOK.call_once(|| {
        let prior = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let inside = BRIDGE_DEPTH
                .try_with(|depth| depth.get() != 0)
                .unwrap_or(false);
            if !inside {
                prior(info);
            }
        }));
    });
}

struct BridgeCall;

impl BridgeCall {
    fn enter() -> Self {
        BRIDGE_DEPTH.with(|depth| depth.set(depth.get().saturating_add(1)));
        Self
    }
}

impl Drop for BridgeCall {
    fn drop(&mut self) {
        let _ = BRIDGE_DEPTH.try_with(|depth| depth.set(depth.get().saturating_sub(1)));
    }
}

#[repr(C)]
pub struct Reply {
    status: i32,
    bytes: *mut u8,
    len: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_probe_init() -> i32 {
    catch_unwind(AssertUnwindSafe(install_hook)).map_or(4, |_| 0)
}

#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_probe_row(input: *const u8, len: usize) -> Reply {
    let result = catch_unwind(AssertUnwindSafe(|| {
        install_hook();
        let _bridge = BridgeCall::enter();
        make_row(input, len)
    }));
    match result {
        Ok(Ok(reply)) => reply,
        Ok(Err(status)) => empty_reply(status),
        Err(_) => empty_reply(4),
    }
}

fn empty_reply(status: i32) -> Reply {
    Reply {
        status,
        bytes: std::ptr::null_mut(),
        len: 0,
    }
}

fn make_row(input: *const u8, len: usize) -> Result<Reply, i32> {
    if input.is_null() {
        return Err(1);
    }
    // SAFETY: C++ passes a live DuckDB string for this call only.
    let bytes = unsafe { std::slice::from_raw_parts(input, len) };
    let text = std::str::from_utf8(bytes).map_err(|_| 2)?;
    if let Some(marker) = text.strip_prefix("panic:") {
        panic!("bridge proof panic: {marker}");
    }
    if text == "other-thread" {
        let _ = std::thread::spawn(|| panic!("unrelated host panic marker")).join();
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
    let mut boxed = encoded.into_boxed_slice();
    let reply = Reply {
        status: 0,
        bytes: boxed.as_mut_ptr(),
        len: boxed.len(),
    };
    std::mem::forget(boxed);
    Ok(reply)
}

#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_probe_free(bytes: *mut u8, len: usize) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        install_hook();
        let _bridge = BridgeCall::enter();
        if !bytes.is_null() {
            // SAFETY: only a successful thinkthen_probe_row allocation is passed back once.
            unsafe {
                drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    bytes, len,
                )))
            }
        }
    }));
}
