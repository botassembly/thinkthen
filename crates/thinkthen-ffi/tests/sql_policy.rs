#![cfg(test)]
#![allow(
    unsafe_code,
    reason = "exercise the public SQL row conversion and owned serialization boundary"
)]
//! A failed PostgreSQL identifier cannot leave stale span metadata that breaks storage.
use std::{
    mem::{size_of, zeroed},
    ptr,
    time::{Duration, Instant},
};
use thinkthen_ffi::*;
#[test]
fn converted_span_rows_remain_serializable_and_count_as_one_failed_input() {
    let server = loopback::Loopback::guide_scenario("yes").unwrap();
    let settings = serde_json::json!({"backend":"systemone","url":server.base(),"api_key":loopback::KEY,"batch":1}).to_string();
    let request = br#"{"question":{"function":"find","question":"Which line?"}}"#;
    let text = "é🦀\nsecond";
    // SAFETY: all pointers and exclusive outputs remain live until their matching frees.
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
            thinkthen_sql_call_open_json(
                client,
                request.as_ptr(),
                request.len(),
                &mut call,
                ptr::null_mut()
            ),
            0
        );
        assert_eq!(
            thinkthen_sql_call_set_raise_if_failed(call, 1, ptr::null_mut()),
            0
        );
        let mut row: HostRow = zeroed();
        row.struct_size = size_of::<HostRow>() as u32;
        let mut sent = false;
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            assert!(Instant::now() < deadline);
            match thinkthen_sql_call_poll_postgres(
                call,
                20,
                None,
                ptr::null_mut(),
                &mut row,
                ptr::null_mut(),
            ) {
                2 => {
                    assert!(!sent);
                    sent = true;
                    let input = SqlInput {
                        kind: 0,
                        data: text.as_ptr(),
                        len: text.len(),
                    };
                    assert_eq!(
                        thinkthen_sql_call_send_chunk(call, &input, 1, ptr::null_mut()),
                        0
                    );
                    thinkthen_sql_call_end_input(call);
                }
                3 => {}
                0 => break,
                other => panic!("unexpected SQL event {other}"),
            }
        }
        assert_eq!(row.status, thinkthen::results::Status::Answered.abi());
        let id = TextView {
            data: b"NaN".as_ptr(),
            len: 3,
        };
        assert_eq!(
            thinkthen_sql_row_postgres_id(call, &mut row, &id, ptr::null_mut()),
            0
        );
        assert_eq!(row.status, thinkthen::results::Status::Failed.abi());
        let mut saved: Buffer = zeroed();
        assert_eq!(thinkthen_row_to_json(&row, &mut saved, ptr::null_mut()), 0);
        let value: serde_json::Value =
            serde_json::from_slice(std::slice::from_raw_parts(saved.data, saved.len)).unwrap();
        assert_eq!(value["status"], "failed");
        assert_eq!(value["error"]["code"], "record.json");
        assert!(value.get("answer").is_none());
        thinkthen_buffer_free(&mut saved);
        thinkthen_row_free(&mut row);
        let mut error: HostError = zeroed();
        error.struct_size = size_of::<HostError>() as u32;
        loop {
            assert!(Instant::now() < deadline);
            let result = thinkthen_sql_call_poll_postgres(
                call,
                20,
                None,
                ptr::null_mut(),
                &mut row,
                &mut error,
            );
            if result == 3 {
                continue;
            }
            assert_eq!(result, -thinkthen::Code::RecordJson.abi());
            let message = std::str::from_utf8(std::slice::from_raw_parts(
                error.message.data,
                error.message.len,
            ))
            .unwrap();
            assert!(message.starts_with("1 of 1 records failed; the first, at position 1"));
            break;
        }
        thinkthen_error_free(&mut error);
        thinkthen_sql_call_free(call);
        thinkthen_client_free(client);
    }
    assert_eq!(server.count(), 1);
}
