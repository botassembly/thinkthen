//! Release the GVL while an owned native session waits; unblock signals cancel.
#![allow(unsafe_code, reason = "Ruby requires C callbacks outside its GVL")]
#![deny(unsafe_op_in_unsafe_fn)]
use magnus::{Error, Ruby};
use std::{ffi::c_void, sync::Arc};
struct Wait {
    session: Arc<thinkthen::RequestSession>,
    input: bool,
}
unsafe extern "C" fn run(data: *mut c_void) -> *mut c_void {
    // SAFETY: wait keeps this stack value and its session owner live until return.
    let wait = unsafe { &*data.cast::<Wait>() };
    let interest = if wait.input {
        thinkthen::RequestSessionWaitInterest::Input
    } else {
        thinkthen::RequestSessionWaitInterest::Settlement
    };
    let _ = wait
        .session
        .wait(interest, std::time::Duration::from_millis(50));
    std::ptr::null_mut()
}
unsafe extern "C" fn unblock(data: *mut c_void) {
    // SAFETY: Ruby calls unblock only while the same stack owner is retained.
    unsafe { &*data.cast::<Wait>() }.session.cancel();
}
pub(super) fn wait(
    _ruby: &Ruby,
    session: Arc<thinkthen::RequestSession>,
    input: bool,
) -> Result<(), Error> {
    let mut wait = Wait { session, input };
    let data = std::ptr::from_mut(&mut wait).cast::<c_void>();
    magnus::rb_sys::protect(|| {
        // SAFETY: run/unblock use only the retained Rust owner, never Ruby values.
        unsafe {
            rb_sys::rb_thread_call_without_gvl(Some(run), data, Some(unblock), data);
        }
        0
    })?;
    Ok(())
}
