#![cfg(test)]
#![allow(
    unsafe_code,
    reason = "exercise the public C table-admission interface"
)]
//! Native hosts supply their actual column shape without duplicating engine rules.
use std::{mem::size_of, ptr};
use thinkthen_ffi::*;

fn text(value: &str) -> TextView {
    TextView {
        data: value.as_ptr(),
        len: value.len(),
    }
}

#[test]
fn table_column_admission_names_the_clash_before_any_send() {
    let mut client = ptr::null_mut();
    let mut question = ptr::null_mut();
    let input = r#"{"function":"annotate","questions":{"refund":{"function":"decide","question":"Refund?"},"refund_status":{"function":"decide","question":"Refund?"}}}"#;
    // SAFETY: all handles and output slots are initialized and uniquely used in this test.
    unsafe {
        let mut error: HostError = std::mem::zeroed();
        error.struct_size = size_of::<HostError>() as u32;
        assert_eq!(thinkthen_client_open_backend(3, &mut client, &mut error), 0);
        assert_eq!(
            thinkthen_question_from_json(input.as_ptr(), input.len(), &mut question, &mut error),
            0
        );
        let columns = [text("status"), text("probability")];
        let suffixes = [text("_status")];
        assert_eq!(
            thinkthen_question_validate_table_columns(
                question,
                columns.as_ptr(),
                columns.len(),
                suffixes.as_ptr(),
                suffixes.len(),
                &mut error
            ),
            -thinkthen::Code::UsageColumn.abi()
        );
        let message = std::slice::from_raw_parts(error.message.data, error.message.len);
        assert!(
            std::str::from_utf8(message)
                .unwrap()
                .contains("refund_status")
        );
        let mut usage = HostUsage {
            struct_size: size_of::<HostUsage>() as u32,
            ..HostUsage::default()
        };
        assert_eq!(
            thinkthen_client_usage_values(client, &mut usage, &mut error),
            0
        );
        assert_eq!(usage.attempts, 0);
        assert_eq!(
            thinkthen_question_validate_table_columns(
                question,
                columns.as_ptr(),
                columns.len(),
                ptr::null(),
                0,
                &mut error
            ),
            0
        );
        thinkthen_question_free(question);
        thinkthen_client_free(client);
        thinkthen_error_free(&mut error);
    }
}
