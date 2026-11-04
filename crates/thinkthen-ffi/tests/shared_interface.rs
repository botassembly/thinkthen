#![cfg(test)]
#![allow(unsafe_code, reason = "exercise the caller's native output ownership")]
//! C failures retain their identity even without an error output.
use std::ptr;
use thinkthen_ffi::*;

#[test]
fn optional_error_slot_preserves_success_and_bad_argument_codes() {
    let mut settings = ptr::null_mut();
    // SAFETY: the output slot is exclusive; NULL discards optional error details.
    unsafe {
        assert_eq!(thinkthen_settings_new(&mut settings, ptr::null_mut()), 0);
        assert!(!settings.is_null());
        assert_eq!(
            thinkthen_settings_set_backend(settings, u32::MAX, ptr::null_mut()),
            -1005
        );
        let mut question = ptr::null_mut();
        assert_eq!(
            thinkthen_question_new(u32::MAX, ptr::null(), 0, &mut question, ptr::null_mut()),
            -1014
        );
        assert!(question.is_null());
        thinkthen_settings_free(settings);
    }
}

#[test]
fn single_record_failure_and_unsure_are_distinct_from_false_and_leave_owned_rows() {
    use std::mem::{size_of, zeroed};
    let server = loopback::Loopback::noul(0.5).unwrap();
    let settings =
        serde_json::json!({"backend":"systemone", "url":server.base(), "api_key":loopback::KEY})
            .to_string();
    let mut client = ptr::null_mut();
    let mut question = ptr::null_mut();
    // SAFETY: all handles and output storage remain live until matching cleanup.
    unsafe {
        assert_eq!(
            thinkthen_client_open_json(
                settings.as_ptr(),
                settings.len(),
                &mut client,
                ptr::null_mut()
            ),
            0
        );
        assert_eq!(
            thinkthen_question_new(1, b"Ready?".as_ptr(), 6, &mut question, ptr::null_mut()),
            0
        );
        assert_eq!(
            thinkthen_question_set_band(question, 0.2, 0.8, ptr::null_mut()),
            0
        );
        let mut row: HostRow = zeroed();
        row.struct_size = size_of::<HostRow>() as u32;
        assert_eq!(
            thinkthen_run_one(
                client,
                question,
                ptr::null(),
                0,
                u32::MAX,
                &mut row,
                ptr::null_mut()
            ),
            -1015
        );
        let mut call = ptr::null_mut();
        assert_eq!(
            thinkthen_call_open_question(
                client,
                question,
                ptr::null(),
                u32::MAX,
                &mut call,
                ptr::null_mut()
            ),
            -1016
        );
        assert!(call.is_null());
        assert_eq!(
            thinkthen_run_one(
                client,
                question,
                b" ".as_ptr(),
                1,
                0,
                &mut row,
                ptr::null_mut()
            ),
            -2005
        );
        assert_eq!(row.error_code, 2005);
        let mut failed = ptr::null_mut();
        assert_eq!(thinkthen_row_values(&row, &mut failed, ptr::null_mut()), 0);
        let mut failed_answer = 97;
        assert_eq!(
            thinkthen_value_bool(failed, &mut failed_answer, ptr::null_mut()),
            -2005
        );
        assert_eq!(failed_answer, 97);
        thinkthen_value_free(failed);
        thinkthen_row_free(&mut row);
        assert_eq!(
            thinkthen_run_one(
                client,
                question,
                b"evidence".as_ptr(),
                8,
                0,
                &mut row,
                ptr::null_mut()
            ),
            4
        );
        let mut value = ptr::null_mut();
        assert_eq!(thinkthen_row_values(&row, &mut value, ptr::null_mut()), 0);
        let mut answer = 97;
        assert_eq!(thinkthen_value_bool(value, &mut answer, ptr::null_mut()), 4);
        assert_eq!(answer, 97, "not sure must leave the answer untouched");
        thinkthen_value_free(value);
        thinkthen_row_free(&mut row);
        thinkthen_question_free(question);
        thinkthen_client_free(client);
    }
    assert_eq!(server.requests().len(), 1);
}

#[test]
fn single_c_spans_use_bytes_and_retain_the_original_match() {
    use std::mem::{size_of, zeroed};
    let server = loopback::Loopback::case(
        &serde_json::json!({"replies":[{"distribution":{"u1":0.0,"u2":1.0,"none":0.0}}]}),
    )
    .unwrap();
    let settings =
        serde_json::json!({"backend":"systemone","url":server.base(),"api_key":loopback::KEY})
            .to_string();
    let mut client = ptr::null_mut();
    let mut question = ptr::null_mut();
    // SAFETY: the handles and result owner remain live through every borrowed span read.
    unsafe {
        assert_eq!(
            thinkthen_client_open_json(
                settings.as_ptr(),
                settings.len(),
                &mut client,
                ptr::null_mut()
            ),
            0
        );
        assert_eq!(
            thinkthen_question_new(
                thinkthen::FunctionName::Find.abi(),
                b"Which line?".as_ptr(),
                11,
                &mut question,
                ptr::null_mut()
            ),
            0
        );
        let mut row: HostRow = zeroed();
        row.struct_size = size_of::<HostRow>() as u32;
        let field = "😀\nAda\n";
        assert_eq!(
            thinkthen_run_one(
                client,
                question,
                field.as_ptr(),
                field.len(),
                0,
                &mut row,
                ptr::null_mut()
            ),
            0
        );
        let mut value = ptr::null_mut();
        assert_eq!(thinkthen_row_values(&row, &mut value, ptr::null_mut()), 0);
        let mut found: SpanView = zeroed();
        assert_eq!(thinkthen_value_found(value, &mut found, ptr::null_mut()), 0);
        assert_eq!((found.start, found.end), (5, 8));
        assert_eq!(
            std::slice::from_raw_parts(found.text.data, found.text.len),
            b"Ada"
        );
        thinkthen_value_free(value);
        thinkthen_row_free(&mut row);
        thinkthen_question_free(question);
        thinkthen_client_free(client);
    }
    assert_eq!(server.requests().len(), 1);
}

#[test]
fn selected_empty_field_never_reaches_the_backend_through_c() {
    use std::mem::{size_of, zeroed};
    let server = loopback::Loopback::noul(0.9).unwrap();
    let settings =
        serde_json::json!({"backend":"systemone","url":server.base(),"api_key":loopback::KEY})
            .to_string();
    let mut client = ptr::null_mut();
    let mut question = ptr::null_mut();
    // SAFETY: counted inputs and all uniquely owned outputs remain live through cleanup.
    unsafe {
        assert_eq!(
            thinkthen_client_open_json(
                settings.as_ptr(),
                settings.len(),
                &mut client,
                ptr::null_mut()
            ),
            0
        );
        assert_eq!(
            thinkthen_question_new(1, b"Ready?".as_ptr(), 6, &mut question, ptr::null_mut()),
            0
        );
        for (input, kind) in [
            ("", 0),
            ("   ", 0),
            ("\n", 0),
            ("{}", 1),
            (r#"{"id":"only"}"#, 1),
        ] {
            let mut row: HostRow = zeroed();
            row.struct_size = size_of::<HostRow>() as u32;
            assert_eq!(
                thinkthen_run_one(
                    client,
                    question,
                    input.as_ptr(),
                    input.len(),
                    kind,
                    &mut row,
                    ptr::null_mut()
                ),
                -2005
            );
            assert_eq!(row.error_code, 2005);
            thinkthen_row_free(&mut row);
        }
        assert_eq!(
            thinkthen_question_set_field(question, b"/text".as_ptr(), 5, ptr::null_mut()),
            0
        );
        for (input, code) in [
            (r#"{"text":""}"#, 2005),
            (r#"{"text":"  "}"#, 2005),
            (r#"{"text":{}}"#, 2005),
            (r#"{"text":[]}"#, 2005),
            (r#"{"text":null}"#, 2002),
        ] {
            let mut row: HostRow = zeroed();
            row.struct_size = size_of::<HostRow>() as u32;
            assert_eq!(
                thinkthen_run_one(
                    client,
                    question,
                    input.as_ptr(),
                    input.len(),
                    1,
                    &mut row,
                    ptr::null_mut()
                ),
                -code
            );
            assert_eq!(row.error_code, code);
            thinkthen_row_free(&mut row);
        }
        thinkthen_question_free(question);
        thinkthen_client_free(client);
    }
    assert!(server.requests().is_empty());
}

#[test]
fn c_stream_publishes_a_failed_blank_row_between_two_answered_records() {
    use std::mem::{size_of, zeroed};
    let server = loopback::Loopback::noul(0.9).unwrap();
    let settings =
        serde_json::json!({"backend":"systemone","url":server.base(),"api_key":loopback::KEY})
            .to_string();
    let request = br#"{"question":{"function":"decide","question":"Ready?","field":"/text"}}"#;
    let inputs = [
        r#"{"text":"first"}"#,
        r#"{"text":[]}"#,
        r#"{"text":"second"}"#,
    ];
    let records: Vec<_> = inputs
        .iter()
        .map(|text| RecordInput {
            struct_size: size_of::<RecordInput>() as u32,
            kind: 1,
            data: text.as_ptr(),
            len: text.len(),
        })
        .collect();
    let mut client = ptr::null_mut();
    let mut call = ptr::null_mut();
    // SAFETY: finite descriptors are readable and copied; each published row has one owner.
    unsafe {
        assert_eq!(
            thinkthen_client_open_json(
                settings.as_ptr(),
                settings.len(),
                &mut client,
                ptr::null_mut()
            ),
            0
        );
        assert_eq!(
            thinkthen_call_open_records_json(
                client,
                request.as_ptr(),
                request.len(),
                records.as_ptr(),
                records.len(),
                size_of::<RecordInput>(),
                0,
                &mut call,
                ptr::null_mut()
            ),
            0
        );
        for position in 1..=3 {
            let mut row: HostRow = zeroed();
            row.struct_size = size_of::<HostRow>() as u32;
            assert_eq!(thinkthen_call_next(call, &mut row, ptr::null_mut()), 0);
            assert_eq!(row.position, position);
            assert_eq!(row.error_code, if position == 2 { 2005 } else { 0 });
            assert_eq!(
                row.status,
                if position == 2 {
                    thinkthen::results::Status::Failed.abi()
                } else {
                    thinkthen::results::Status::Answered.abi()
                }
            );
            thinkthen_row_free(&mut row);
        }
        let mut row: HostRow = zeroed();
        row.struct_size = size_of::<HostRow>() as u32;
        assert_eq!(thinkthen_call_next(call, &mut row, ptr::null_mut()), 1);
        thinkthen_call_free(call);
        thinkthen_client_free(client);
    }
    assert_eq!(server.requests().len(), 1);
}

#[test]
fn ranked_originals_follow_positions_and_outlive_calls_and_rows() {
    use std::mem::{size_of, zeroed};
    let request = br#"{"question":{"function":"rank","question":"Priority?","levels":["low","high"],"top":1}}"#;
    let inputs: [&[u8]; 4] = [
        br#"{ "id":18446744073709551615, "body":"first" }"#,
        b"second",
        b"null",
        &[0xff],
    ];
    let records: Vec<_> = inputs
        .iter()
        .enumerate()
        .map(|(index, bytes)| RecordInput {
            struct_size: size_of::<RecordInput>() as u32,
            kind: if index == 0 || index == 2 { 1 } else { 0 },
            data: bytes.as_ptr(),
            len: bytes.len(),
        })
        .collect();
    let mut client = ptr::null_mut();
    let mut call = ptr::null_mut();
    // SAFETY: input descriptors remain readable; every row and snapshot is freed once.
    unsafe {
        let settings = br#"{"backend":"stub"}"#;
        assert_eq!(
            thinkthen_client_open_json(
                settings.as_ptr(),
                settings.len(),
                &mut client,
                ptr::null_mut()
            ),
            0
        );
        assert_eq!(
            thinkthen_call_open_records_json(
                client,
                request.as_ptr(),
                request.len(),
                records.as_ptr(),
                records.len(),
                size_of::<RecordInput>(),
                0,
                &mut call,
                ptr::null_mut()
            ),
            0
        );
        let mut rows = Vec::new();
        loop {
            let mut row: HostRow = zeroed();
            row.struct_size = size_of::<HostRow>() as u32;
            let status = thinkthen_call_next(call, &mut row, ptr::null_mut());
            if status == 1 {
                break;
            }
            assert_eq!(status, 0);
            rows.push(row);
        }
        thinkthen_call_free(call);
        thinkthen_client_free(client);
        assert_eq!(rows.len(), 3); // top one successful row, plus the two admission failures.
        assert_eq!(rows.iter().filter(|row| row.error_code == 0).count(), 1);
        for mut row in rows {
            let index = row.position as usize - 1;
            let mut original: RecordView = zeroed();
            assert_eq!(
                thinkthen_row_original(&row, &mut original, ptr::null_mut()),
                0
            );
            assert_eq!(original.kind, records[index].kind);
            assert_eq!(
                std::slice::from_raw_parts(original.data, original.len),
                inputs[index]
            );
            let mut snapshot = ptr::null_mut();
            assert_eq!(
                thinkthen_row_values(&row, &mut snapshot, ptr::null_mut()),
                0
            );
            thinkthen_row_free(&mut row);
            assert_eq!(
                thinkthen_value_original(snapshot, &mut original, ptr::null_mut()),
                0
            );
            assert_eq!(
                std::slice::from_raw_parts(original.data, original.len),
                inputs[index]
            );
            thinkthen_value_free(snapshot);
        }
    }
}

#[test]
fn streaming_rank_retains_invalid_original_bytes_after_owners_are_freed() {
    use std::mem::{size_of, zeroed};
    let request = br#"{"question":{"function":"rank","question":"Priority?","levels":["low","high"],"top":1}}"#;
    let settings = br#"{"backend":"stub"}"#;
    let bytes = [0xff, 0, 0xfe];
    let mut client = ptr::null_mut();
    let mut call = ptr::null_mut();
    // SAFETY: input bytes remain live through admission; each output has one owner.
    unsafe {
        assert_eq!(
            thinkthen_client_open_json(
                settings.as_ptr(),
                settings.len(),
                &mut client,
                ptr::null_mut()
            ),
            0
        );
        assert_eq!(
            thinkthen_call_open_json(
                client,
                request.as_ptr(),
                request.len(),
                0,
                &mut call,
                ptr::null_mut()
            ),
            0
        );
        for kind in 0..2 {
            let mut row: HostRow = zeroed();
            row.struct_size = size_of::<HostRow>() as u32;
            assert_eq!(thinkthen_call_next(call, &mut row, ptr::null_mut()), 2);
            let status = if kind == 0 {
                thinkthen_call_send_text(call, bytes.as_ptr(), bytes.len(), ptr::null_mut())
            } else {
                thinkthen_call_send_json(call, bytes.as_ptr(), bytes.len(), ptr::null_mut())
            };
            assert_eq!(status, 0);
        }
        thinkthen_call_end_input(call);
        let mut rows = Vec::new();
        for _ in 0..2 {
            let mut row: HostRow = zeroed();
            row.struct_size = size_of::<HostRow>() as u32;
            assert_eq!(thinkthen_call_next(call, &mut row, ptr::null_mut()), 0);
            rows.push(row);
        }
        thinkthen_call_free(call);
        thinkthen_client_free(client);
        for (kind, mut row) in rows.into_iter().enumerate() {
            assert_eq!(row.position, kind as u64 + 1);
            assert_eq!(row.error_code, thinkthen::Code::RecordInvalid.abi());
            let mut original: RecordView = zeroed();
            assert_eq!(
                thinkthen_row_original(&row, &mut original, ptr::null_mut()),
                0
            );
            assert_eq!(original.kind, kind as u32);
            assert_eq!(
                std::slice::from_raw_parts(original.data, original.len),
                bytes
            );
            let mut snapshot = ptr::null_mut();
            assert_eq!(
                thinkthen_row_values(&row, &mut snapshot, ptr::null_mut()),
                0
            );
            thinkthen_row_free(&mut row);
            assert_eq!(
                thinkthen_value_original(snapshot, &mut original, ptr::null_mut()),
                0
            );
            assert_eq!(
                std::slice::from_raw_parts(original.data, original.len),
                bytes
            );
            thinkthen_value_free(snapshot);
        }
    }
}
