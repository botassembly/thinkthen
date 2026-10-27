//! Event waits retain no pointer into a call's mutable driver.
#![allow(
    unsafe_code,
    reason = "the ABI retains and releases opaque event handles"
)]
use crate::memory::CallHandle;

/// An independently retained event source, safe after the call is freed.
#[derive(Debug)]
pub struct CallEventsHandle(thinkthen::native::Events);

/// Retain the call's wake source without changing its input reservation.
///
/// Host sequence: retain once; read events_version BEFORE call_next; on
/// NEEDS_INPUT select caller input against events_wait(version). If input wins,
/// call events_wake and join the waiter before sending through the sole driver.
/// If an event wins, recheck call_next without issuing another input demand.
/// Output may become ready while the reserved input remains outstanding.
///
/// Cancellation wakes this source through the call's queues. A host also wakes
/// and joins its waiters on close; it frees the event handle only after all
/// version/wait/wake operations return. The event handle can outlive call_free.
/// # Safety
/// The sole driver owns a live call during this retain operation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_events(call: *const CallHandle) -> *mut CallEventsHandle {
    // SAFETY: the sole driver grants shared access before concurrent waits start.
    let events = unsafe { (&*(*call).call.get()).events() };
    Box::into_raw(Box::new(CallEventsHandle(events)))
}
/// Read the version BEFORE the sole driver checks call readiness.
/// # Safety
/// Events stays live until this call returns. Concurrent event operations are safe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_events_version(events: *const CallEventsHandle) -> u64 {
    // SAFETY: only an immutable retained signal is accessed.
    unsafe { (*events).0.version() }
}
/// Wait for a change from version, returning immediately if it already changed.
/// A wake requests a recheck; it does not promise a row or consume input capacity.
/// # Safety
/// Events stays live until this wait returns. Concurrent waits and wakes are safe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_events_wait(events: *const CallEventsHandle, version: u64) {
    // SAFETY: only the retained signal's mutex and condition variable are used.
    unsafe {
        (*events).0.wait(version);
    }
}
/// Wake waits on earlier versions, including waits not yet entered.
/// This does not cancel the call or alter its reserved input.
/// # Safety
/// Events stays live until this call returns. Concurrent event operations are safe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_events_wake(events: *const CallEventsHandle) {
    // SAFETY: the retained signal provides its own synchronization.
    unsafe {
        (*events).0.wake();
    }
}
/// Free an event source after all its operations have returned.
/// # Safety
/// Events is uniquely owned and no version, wait or wake operation remains active.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_events_free(events: *mut CallEventsHandle) {
    // SAFETY: the caller retires all uses before releasing this unique allocation.
    unsafe {
        drop(Box::from_raw(events));
    }
}
