#![cfg(test)]
#![allow(
    unsafe_code,
    reason = "exercise native handle ownership through public exports"
)]
//! Snapshots remain owned while calls outlive clients, and cancelled calls settle.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
use std::{
    mem::size_of,
    ptr,
    sync::{Mutex, mpsc},
};
use thinkthen_ffi::*;

#[test]
fn copied_native_usage_survives_client_free_during_an_active_call() {
    let (arrived, observed) = mpsc::channel();
    let (release, blocked) = mpsc::channel();
    let blocked = Mutex::new(blocked);
    let server = loopback::Loopback::start(move |_| {
        arrived.send(()).unwrap();
        blocked.lock().unwrap().recv().unwrap();
        loopback::Reply::json(200, &serde_json::json!({"answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":9,"output_tokens":0}}))
    }).unwrap();
    let settings =
        serde_json::json!({"backend":"systemone","url":server.base(),"api_key":loopback::KEY})
            .to_string();
    let request =
        br#"{"question":{"function":"decide","question":"Ready?"},"records":["evidence"]}"#;
    // SAFETY: handles remain live until their sole free; output buffers own their bytes.
    unsafe {
        let mut client = ptr::null_mut();
        assert_eq!(
            thinkthen_client_open_json(
                settings.as_ptr(),
                settings.len(),
                &mut client,
                ptr::null_mut()
            ),
            0
        );
        let mut call = ptr::null_mut();
        assert_eq!(
            thinkthen_call_open_json(
                client,
                request.as_ptr(),
                request.len(),
                2,
                &mut call,
                ptr::null_mut()
            ),
            0
        );
        thinkthen_call_end_input(call);
        observed.recv().unwrap();
        let mut snapshot = Buffer::default();
        assert_eq!(
            thinkthen_client_usage(client, &mut snapshot, ptr::null_mut()),
            0
        );
        let mut typed = HostUsage {
            struct_size: size_of::<HostUsage>() as u32,
            ..HostUsage::default()
        };
        assert_eq!(
            thinkthen_client_usage_values(client, &mut typed, ptr::null_mut()),
            0
        );
        assert_eq!(typed.attempts, 1);
        assert_eq!(typed.active_calls, 1);
        assert_eq!(typed.active_attempts, 1);
        thinkthen_client_free(client);
        release.send(()).unwrap();
        let mut row: HostRow = std::mem::zeroed();
        row.struct_size = size_of::<HostRow>() as u32;
        assert_eq!(thinkthen_call_next(call, &mut row, ptr::null_mut()), 0);
        thinkthen_row_free(&mut row);
        assert_eq!(thinkthen_call_next(call, &mut row, ptr::null_mut()), 1);
        thinkthen_row_free(&mut row);
        thinkthen_call_free(call);
        let copied: serde_json::Value =
            serde_json::from_slice(std::slice::from_raw_parts(snapshot.data, snapshot.len))
                .unwrap();
        assert_eq!(copied["attempts"], 1);
        assert_eq!(copied["active_calls"], 1);
        assert_eq!(copied["active_attempts"], 1);
        thinkthen_buffer_free(&mut snapshot);
    }
}

#[test]
fn cancelled_dropped_calls_settle_without_waiting_for_server_reply() {
    let (arrived, observed) = mpsc::channel();
    let (release, blocked) = mpsc::channel();
    let blocked = Mutex::new(blocked);
    let server = loopback::Loopback::start(move |_| {
        arrived.send(()).unwrap();
        blocked.lock().unwrap().recv().unwrap();
        loopback::Reply::json(200, &serde_json::json!({}))
    })
    .unwrap();
    let settings =
        serde_json::json!({"backend":"systemone","url":server.base(),"api_key":loopback::KEY})
            .to_string();
    let request =
        br#"{"question":{"function":"decide","question":"Ready?"},"records":["evidence"]}"#;
    // SAFETY: exclusive live handles and sized output; no dead handle is queried.
    unsafe {
        let mut client = ptr::null_mut();
        assert_eq!(
            thinkthen_client_open_json(
                settings.as_ptr(),
                settings.len(),
                &mut client,
                ptr::null_mut()
            ),
            0
        );
        let mut call = ptr::null_mut();
        assert_eq!(
            thinkthen_call_open_json(
                client,
                request.as_ptr(),
                request.len(),
                2,
                &mut call,
                ptr::null_mut()
            ),
            0
        );
        thinkthen_call_end_input(call);
        observed.recv().unwrap();
        thinkthen_call_free(call);
        let mut snapshot = HostUsage {
            struct_size: size_of::<HostUsage>() as u32,
            ..HostUsage::default()
        };
        assert_eq!(
            thinkthen_client_usage_values_settled(client, &mut snapshot, ptr::null_mut()),
            0
        );
        assert_eq!(snapshot.active_calls, 0);
        assert_eq!(snapshot.active_attempts, 0);
        assert_eq!(snapshot.attempts, 1);
        assert_eq!(snapshot.input_tokens.unknown_attempts, 1);
        assert!(snapshot.input_tokens.reported_known);
        thinkthen_client_free(client);
        release.send(()).unwrap();
    }
}
