//! Event waits retain no pointer into a call's mutable driver.
#![allow(
    unsafe_code,
    reason = "the ABI retains and releases opaque event handles"
)]
use crate::memory::CallHandle;

/// A call's wake source, from thinkthen_call_events; it may outlive the call. Free it with
/// thinkthen_events_free.
#[derive(Debug)]
pub struct CallEventsHandle(thinkthen::native::Events);

/// Get a wake source for a streaming call, for a host that waits on input and output at once.
///
/// Read thinkthen_events_version before thinkthen_call_next. When it returns
/// THINKTHEN_NEEDS_INPUT, wait for your own input or thinkthen_events_wait(version), whichever
/// comes first. If input comes first, call thinkthen_events_wake, join the waiter, then send.
/// If an event comes first, call thinkthen_call_next again. Cancelling the call wakes the
/// source. Returns a new handle the caller owns; free it with thinkthen_events_free after every
/// wait and wake has returned. It may outlive the call.
/// # Safety
/// The thread driving call calls this. call is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_events(call: *const CallHandle) -> *mut CallEventsHandle {
    // SAFETY: the caller lends a live call or NULL.
    let Some(call) = (unsafe { call.as_ref() }) else {
        return std::ptr::null_mut();
    };
    // SAFETY: the sole driver grants shared access before concurrent waits start.
    let events = unsafe { (&*call.call.get()).events() };
    Box::into_raw(Box::new(CallEventsHandle(events)))
}
/// Return the wake source's current version.
///
/// Read it before you check the call, then wait on it.
/// # Safety
/// Any thread may call this while events is live; event functions may run at the same time.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_events_version(events: *const CallEventsHandle) -> u64 {
    // SAFETY: only an immutable retained signal is accessed.
    unsafe { events.as_ref() }.map_or(0, |events| events.0.version())
}
/// Wait until the version is no longer version; return at once if it already changed.
///
/// A wake asks you to check the call again. It does not promise a row.
/// # Safety
/// Any thread may call this while events is live; event functions may run at the same time.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_events_wait(events: *const CallEventsHandle, version: u64) {
    // SAFETY: only the retained signal's mutex and condition variable are used.
    if let Some(events) = unsafe { events.as_ref() } {
        events.0.wait(version);
    }
}
/// Wake every wait on an earlier version, including a wait not yet entered.
///
/// It does not cancel the call or change its input.
/// # Safety
/// Any thread may call this while events is live; event functions may run at the same time.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_events_wake(events: *const CallEventsHandle) {
    // SAFETY: the retained signal provides its own synchronization.
    if let Some(events) = unsafe { events.as_ref() } {
        events.0.wake();
    }
}
/// Free a wake source.
///
/// NULL is ignored.
/// # Safety
/// Call this once, after every version, wait and wake call on events has returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_events_free(events: *mut CallEventsHandle) {
    // SAFETY: the caller retires all uses before releasing this unique allocation.
    if !events.is_null() {
        drop(unsafe { Box::from_raw(events) });
    }
}
