#![cfg(test)]
//! Reusing one row or error variable frees what it held. Only the library sees a
//! free, so this counts the engine buffers the calling thread still owns.
#![allow(unsafe_code, reason = "drive the C exports as a C caller does")]
use crate::memory::LIVE;
use crate::*;
use std::{mem::size_of, ptr};

fn live() -> isize {
    LIVE.with(std::cell::Cell::get)
}

#[test]
fn a_reused_error_or_row_holds_only_its_latest_result() {
    let mut error = HostError::empty(size_of::<HostError>() as u32);
    let mut row = HostRow::empty(size_of::<HostRow>() as u32);
    let mut client = ptr::null_mut();
    let mut question = ptr::null_mut();
    // SAFETY: every handle and output slot stays live until its matching free.
    unsafe {
        assert_eq!(
            thinkthen_client_open_backend(3, &mut client, ptr::null_mut()),
            0
        );
        assert_eq!(
            thinkthen_question_new(1, b"Ready?".as_ptr(), 6, &mut question, ptr::null_mut()),
            0
        );
        let start = live();
        for _ in 0..3 {
            let mut unknown = ptr::null_mut();
            assert_eq!(
                thinkthen_question_new(99, ptr::null(), 0, &mut unknown, &mut error),
                -1014
            );
        }
        assert_eq!(live() - start, 1, "the error holds one message");
        for _ in 0..3 {
            assert_eq!(
                thinkthen_run_one(client, question, b"ok".as_ptr(), 2, 0, &mut row, &mut error),
                0
            );
        }
        let held = live() - start;
        for _ in 0..3 {
            assert_eq!(
                thinkthen_run_one(client, question, b" ".as_ptr(), 1, 0, &mut row, &mut error),
                -2005
            );
            assert!(
                row.private_data.is_null(),
                "a failed scalar call leaves no row"
            );
            assert_eq!(error.code, 2005);
            assert_eq!(live() - start, 1, "only the error message remains owned");
            assert_eq!(
                thinkthen_run_one(client, question, b"ok".as_ptr(), 2, 0, &mut row, &mut error),
                0
            );
            assert_eq!(live() - start, held, "the client and row can be reused");
        }
        thinkthen_row_free(&mut row);
        thinkthen_error_free(&mut error);
        assert!(held > 0, "the row holds its latest answer");
        assert_eq!(live(), start, "freeing the row and error leaves nothing");
        let one = {
            let before = live();
            assert_eq!(
                thinkthen_run_one(client, question, b"ok".as_ptr(), 2, 0, &mut row, &mut error),
                0
            );
            let one = live() - before;
            thinkthen_row_free(&mut row);
            one
        };
        assert_eq!(held, one, "three reuses hold no more than one call");
        thinkthen_question_free(question);
        thinkthen_client_free(client);
    }
}
