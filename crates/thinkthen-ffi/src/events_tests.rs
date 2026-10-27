#![cfg(test)]
//! ABI concurrency: retained waits preserve reservations and cannot lose wakes.
#![allow(
    unsafe_code,
    reason = "exercise opaque handles through their C exports"
)]
use crate::*;
use std::{mem::size_of, ptr, sync::mpsc, thread};

unsafe fn open() -> (*mut ClientHandle, *mut CallHandle) {
    let settings = br#"{"backend":"stub","batch":1}"#;
    let request = br#"{"question":{"function":"decide","question":"Ready?"},"records":[]}"#;
    let mut error = HostError::empty(size_of::<HostError>() as u32);
    let mut client = ptr::null_mut();
    let mut call = ptr::null_mut();
    // SAFETY: counted JSON and exclusive output slots remain live for each call.
    unsafe {
        assert_eq!(
            thinkthen_client_open_json(settings.as_ptr(), settings.len(), &mut client, &mut error),
            0
        );
        assert_eq!(
            thinkthen_call_open_json(
                client,
                request.as_ptr(),
                request.len(),
                1,
                &mut call,
                &mut error
            ),
            0
        );
    }
    (client, call)
}

#[test]
fn event_wake_before_wait_preserves_input_reservation_and_outlives_call() {
    // SAFETY: one test owns the driver and retires every event operation before free.
    unsafe {
        let (client, call) = open();
        let events = thinkthen_call_events(call);
        let version = thinkthen_events_version(events);
        let mut error = HostError::empty(size_of::<HostError>() as u32);
        let mut row = HostRow::empty(size_of::<HostRow>() as u32);
        assert_eq!(thinkthen_call_next(call, &mut row, &mut error), 2);
        thinkthen_events_wake(events);
        thinkthen_events_wait(events, version);
        assert_eq!(thinkthen_call_next(call, &mut row, &mut error), 2);
        assert_eq!(thinkthen_call_send_missing(call, &mut error), 0);
        thinkthen_call_end_input(call);
        assert_eq!(thinkthen_call_next(call, &mut row, &mut error), 0);
        thinkthen_row_free(&mut row);
        assert_eq!(thinkthen_call_next(call, &mut row, &mut error), 1);
        let version = thinkthen_events_version(events);
        thinkthen_call_free(call);
        thinkthen_client_free(client);
        // Call destruction closes its output queue; the standalone signal remains live.
        thinkthen_events_wait(events, version);
        let version = thinkthen_events_version(events);
        thinkthen_events_wake(events);
        thinkthen_events_wait(events, version);
        thinkthen_events_free(events);
    }
}

#[test]
fn event_wait_wakes_on_call_cancellation() {
    // SAFETY: call remains owned by this thread; only the independent event source
    // crosses to the waiter, and that thread is joined before its handle is freed.
    unsafe {
        let (client, call) = open();
        let events = thinkthen_call_events(call);
        let version = thinkthen_events_version(events);
        let address = events as usize;
        let (started, entered) = mpsc::channel();
        let waiter = thread::spawn(move || {
            started.send(()).unwrap();
            thinkthen_events_wait(address as *const CallEventsHandle, version);
        });
        entered.recv().unwrap();
        thinkthen_call_cancel(call);
        waiter.join().unwrap();
        thinkthen_call_free(call);
        thinkthen_client_free(client);
        thinkthen_events_free(events);
    }
}
