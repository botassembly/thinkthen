#![cfg(test)]
#![allow(unsafe_code, reason = "make the mistakes a C caller can make")]
//! A NULL handle is an ABI error at every export, and a changed length frees the real allocation.
use std::alloc::{GlobalAlloc, Layout, System};
use std::mem::{size_of, zeroed};
use std::ptr::{null as none, null_mut as empty};
use std::sync::atomic::{AtomicUsize, Ordering};
use thinkthen::Code;
use thinkthen_ffi::*;

const TEXT: &[u8] = b"x";
const REQUEST: &str = r#"{"question":{"function":"decide","question":"Ready?"}}"#;
const CODE_POINTS: u32 = 2;

fn error() -> HostError {
    // SAFETY: an all-zero error with its size set is a valid empty error.
    let mut error: HostError = unsafe { zeroed() };
    error.struct_size = size_of::<HostError>() as u32;
    error
}

fn row() -> HostRow {
    // SAFETY: an all-zero row with its size set is a valid empty row.
    let mut row: HostRow = unsafe { zeroed() };
    row.struct_size = size_of::<HostRow>() as u32;
    row
}

macro_rules! refused {
    ($($export:ident($($argument:expr),* $(,)?);)*) => {$({
        let mut error = error();
        // SAFETY: every argument but the NULL handle is what the export documents.
        let status = unsafe { $export($($argument,)* &mut error) };
        let abi = Code::UsageAbi.abi();
        assert_eq!((status, error.code), (-abi, abi), stringify!($export));
        // SAFETY: the error is initialized and holds the message the export wrote.
        unsafe { thinkthen_error_free(&mut error) };
    })*};
}

#[test]
fn a_null_handle_is_an_abi_error_at_every_export_that_can_report_one() {
    let t = TEXT.as_ptr();
    let (mut client, mut question, mut settings, mut toolbox) =
        (empty(), empty(), empty(), empty());
    // SAFETY: each output is exclusive and each handle is freed below.
    unsafe {
        assert_eq!(thinkthen_client_open_backend(3, &mut client, empty()), 0);
        assert_eq!(
            thinkthen_question_new(1, b"Ready?".as_ptr(), 6, &mut question, empty()),
            0
        );
        assert_eq!(thinkthen_settings_new(&mut settings, empty()), 0);
        assert_eq!(thinkthen_toolbox_new(&mut toolbox, empty()), 0);
    }
    let (request, len) = (REQUEST.as_ptr(), REQUEST.len());
    refused! {
        thinkthen_question_validate_table_columns(none(), none(), 0, none(), 0);
        thinkthen_client_usage(none(), &mut Buffer::default());
        thinkthen_client_usage_settled(none(), &mut Buffer::default());
        thinkthen_client_usage_values(none(), &mut HostUsage::default());
        thinkthen_client_usage_values_settled(none(), &mut HostUsage::default());
        thinkthen_call_open_question(none(), question, none(), CODE_POINTS, &mut empty());
        thinkthen_call_open_question(client, none(), none(), CODE_POINTS, &mut empty());
        thinkthen_call_open_json(none(), request, len, CODE_POINTS, &mut empty());
        thinkthen_call_open_records_json(
            none(), request, len, none(), 0, 0, CODE_POINTS, &mut empty(),
        );
        thinkthen_call_open_records_question(
            none(), question, none(), none(), 0, 0, CODE_POINTS, &mut empty(),
        );
        thinkthen_call_open_records_question(
            client, none(), none(), none(), 0, 0, CODE_POINTS, &mut empty(),
        );
        thinkthen_call_send_text(empty(), t, 1);
        thinkthen_call_send_json(empty(), b"1".as_ptr(), 1);
        thinkthen_call_send_utf16(empty(), [120_u16].as_ptr(), 1);
        thinkthen_call_send_missing(empty());
        thinkthen_call_send_invalid(empty());
        thinkthen_call_next(empty(), &mut row());
        thinkthen_call_next_kept(empty(), &mut row());
        thinkthen_run_one(none(), question, t, 1, 0, &mut row());
        thinkthen_run_one(client, none(), t, 1, 0, &mut row());
        thinkthen_sql_call_open_question(none(), question, none(), &mut empty());
        thinkthen_sql_call_open_question(client, none(), none(), &mut empty());
        thinkthen_sql_call_open_json(none(), request, len, &mut empty());
        thinkthen_sql_question_set_threshold(empty(), &0.5, none(), none());
        thinkthen_sql_call_send_chunk(empty(), none(), 0);
        thinkthen_sql_call_send_chunk_postgres(empty(), none(), 0);
        thinkthen_sql_call_poll(empty(), 0, None, empty(), &mut row());
        thinkthen_sql_call_poll_postgres(empty(), 0, None, empty(), &mut row());
        thinkthen_question_set_yes_means(empty(), t, 1);
        thinkthen_question_set_no_means(empty(), t, 1);
        thinkthen_question_set_description(empty(), t, 1, t, 1);
        thinkthen_question_add_choice(empty(), t, 1);
        thinkthen_question_add_label(empty(), t, 1);
        thinkthen_question_add_level(empty(), t, 1);
        thinkthen_question_add_kind(empty(), t, 1);
        thinkthen_question_add_rule(empty(), none());
        thinkthen_question_set_cut(empty(), 0.5);
        thinkthen_question_set_band(empty(), 0.2, 0.8);
        thinkthen_question_set_field(empty(), t, 1);
        thinkthen_question_set_top(empty(), 1);
        thinkthen_question_add_entry(empty(), t, 1, question);
        thinkthen_question_add_entry(question, t, 1, none());
        thinkthen_question_clone(none(), &mut empty());
        thinkthen_question_to_json(none(), &mut Buffer::default());
        thinkthen_question_function(none(), &mut 0);
        thinkthen_settings_set_backend(empty(), 3);
        thinkthen_settings_set_api_key(empty(), t, 1);
        thinkthen_settings_set_model(empty(), t, 1);
        thinkthen_settings_set_url(empty(), t, 1);
        thinkthen_settings_set_batch(empty(), 1);
        thinkthen_settings_set_retry(empty(), 0, 1, 1000);
        thinkthen_settings_clone(none(), &mut empty());
        thinkthen_settings_copy_api_key(empty(), settings);
        thinkthen_settings_copy_api_key(settings, none());
        thinkthen_toolbox_insert(empty(), t, 1, question);
        thinkthen_toolbox_insert(toolbox, t, 1, none());
        thinkthen_toolbox_select(none(), t, 1, &mut empty());
        thinkthen_toolbox_select_as(none(), t, 1, 1, &mut empty());
        thinkthen_toolbox_to_json(none(), &mut Buffer::default());
        thinkthen_toolbox_definitions_json(none(), &mut Buffer::default());
        thinkthen_value_choice(none(), empty());
        thinkthen_value_bool(none(), empty());
        thinkthen_value_number(none(), empty());
        thinkthen_value_rank(none(), empty());
        thinkthen_value_tag_count(none(), empty());
        thinkthen_value_tag_at(none(), 0, empty());
        thinkthen_value_span(none(), empty());
        thinkthen_value_name_count(none(), empty());
        thinkthen_value_name_at(none(), 0, empty());
        thinkthen_value_relation_count(none(), empty());
        thinkthen_value_relation_at(none(), 0, empty());
        thinkthen_value_annotation_count(none(), empty());
        thinkthen_value_annotation_at(none(), 0, empty(), &mut empty());
        thinkthen_value_annotation_get(none(), t, 1, &mut empty());
        thinkthen_value_label_count(none(), empty());
        thinkthen_value_label_at(none(), 0, empty());
        thinkthen_value_level_count(none(), empty());
        thinkthen_value_level_at(none(), 0, empty());
        thinkthen_value_piece_count(none(), empty());
        thinkthen_value_piece_at(none(), 0, empty());
        thinkthen_value_piece_tag_count(none(), 0, empty());
        thinkthen_value_piece_tag_at(none(), 0, 0, empty());
        thinkthen_value_kind_count(none(), empty());
        thinkthen_value_kind_span(none(), 0, empty());
        thinkthen_value_kind_probability_count(none(), 0, empty());
        thinkthen_value_kind_probability_at(none(), 0, 0, empty());
        thinkthen_value_pair_count(none(), empty());
        thinkthen_value_pair_at(none(), 0, empty());
        thinkthen_value_function(none(), empty());
        thinkthen_value_status(none(), empty());
        thinkthen_value_source(none(), empty());
        thinkthen_value_id(none(), empty());
        thinkthen_value_error(none(), &mut error());
        thinkthen_value_probability(none(), empty());
        thinkthen_value_has_detail(none(), empty());
        thinkthen_value_source_name(none(), empty());
        thinkthen_value_original(none(), empty());
    }
    // SAFETY: each handle came from its constructor above and is freed once.
    unsafe {
        thinkthen_toolbox_free(toolbox);
        thinkthen_settings_free(settings);
        thinkthen_question_free(question);
        thinkthen_client_free(client);
    }
}

#[test]
fn a_null_handle_does_nothing_at_an_export_with_no_error_to_report() {
    // SAFETY: these exports accept NULL as a handle that does nothing.
    unsafe {
        thinkthen_call_end_input(empty());
        thinkthen_call_cancel(empty());
        thinkthen_sql_call_end_input(empty());
        thinkthen_sql_call_cancel(empty());
        thinkthen_events_wait(none(), 0);
        thinkthen_events_wake(none());
        thinkthen_events_free(empty());
        assert_eq!(thinkthen_events_version(none()), 0);
        assert!(thinkthen_call_events(none()).is_null());
    }
}

/// The system allocator, reporting how many bytes past a watched address it frees.
struct Watching;
#[global_allocator]
static ALLOCATOR: Watching = Watching;
static WATCHED: AtomicUsize = AtomicUsize::new(0);
static FREED_PAST_WATCHED: AtomicUsize = AtomicUsize::new(0);
// SAFETY: every request passes straight to the system allocator.
unsafe impl GlobalAlloc for Watching {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the caller's layout passes through unchanged.
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, data: *mut u8, layout: Layout) {
        let (start, watched) = (data as usize, WATCHED.load(Ordering::Relaxed));
        if (start..=start + layout.size()).contains(&watched) {
            FREED_PAST_WATCHED.store(start + layout.size() - watched, Ordering::Relaxed);
        }
        // SAFETY: the caller's allocation and layout pass through unchanged.
        unsafe { System.dealloc(data, layout) }
    }
}

#[test]
fn a_buffer_frees_its_real_allocation_after_the_caller_changes_len() {
    let mut question = empty();
    let mut buffer = Buffer::default();
    // SAFETY: the outputs are exclusive and the buffer and question are freed once.
    unsafe {
        assert_eq!(
            thinkthen_question_new(1, b"Ready?".as_ptr(), 6, &mut question, empty()),
            0
        );
        assert_eq!(
            thinkthen_question_to_json(question, &mut buffer, empty()),
            0
        );
        let len = buffer.len;
        assert!(len > 0);
        WATCHED.store(buffer.data as usize, Ordering::Relaxed);
        buffer.len = 0;
        thinkthen_buffer_free(&mut buffer);
        // The owned allocation includes the trailing C-string sentinel, even
        // when the caller changes the public counted length before freeing it.
        assert_eq!(FREED_PAST_WATCHED.load(Ordering::Relaxed), len + 1);
        assert!(buffer.data.is_null());
        thinkthen_question_free(question);
    }
}
