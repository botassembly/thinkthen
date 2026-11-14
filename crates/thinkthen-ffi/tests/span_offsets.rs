#![cfg(test)]
#![allow(
    unsafe_code,
    reason = "exercise independently owned native span projections"
)]
//! Host buffers and their decoded value handles preserve native string coordinates.
use std::{
    mem::{size_of, zeroed},
    ptr,
};
use thinkthen_ffi::*;

#[test]
fn streamed_and_direct_spans_keep_native_units_after_calls_are_freed() {
    let server = loopback::Loopback::case(&serde_json::json!({"replies":[
        {"distribution":{"u1":0.1,"u2":0.9,"none":0}}
    ]}))
    .unwrap();
    let settings =
        serde_json::json!({"backend":"systemone","url":server.base(),"api_key":loopback::KEY})
            .to_string();
    let text = "é🦀\r\n E\u{301}lise";
    let request = serde_json::json!({"question":{"function":"find","question":"Which line?"},"records":[text]}).to_string();
    // SAFETY: every handle and exclusive output is initialized, retained and freed once.
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
        let mut direct = SpanView::default();
        assert_eq!(
            thinkthen_find(
                client,
                c"Which line?".as_ptr(),
                text.as_ptr(),
                text.len(),
                &mut direct,
                ptr::null_mut()
            ),
            0
        );
        assert_eq!((direct.start, direct.end), (8, 16));
        for (unit, start, end) in [(0, 8, 16), (1, 5, 12), (2, 4, 11), (3, 5, 11)] {
            let mut call = ptr::null_mut();
            assert_eq!(
                thinkthen_call_open_json(
                    client,
                    request.as_ptr(),
                    request.len(),
                    unit,
                    &mut call,
                    ptr::null_mut()
                ),
                0
            );
            thinkthen_call_end_input(call);
            let mut row: HostRow = zeroed();
            row.struct_size = size_of::<HostRow>() as u32;
            assert_eq!(thinkthen_call_next(call, &mut row, ptr::null_mut()), 0);
            thinkthen_call_free(call);
            let mut value = ptr::null_mut();
            assert_eq!(thinkthen_row_values(&row, &mut value, ptr::null_mut()), 0);
            thinkthen_row_free(&mut row);
            let mut span = SpanView::default();
            assert_eq!(thinkthen_value_span(value, &mut span, ptr::null_mut()), 0);
            assert_eq!((span.start, span.end), (start, end));
            assert_eq!(
                std::slice::from_raw_parts(span.text.data, span.text.len),
                " E\u{301}lise".as_bytes()
            );
            thinkthen_value_free(value);
        }
        thinkthen_client_free(client);
    }
    assert_eq!(server.count(), 5);
}
