//! `thinkthen_warm(question, text)` as a DuckDB aggregate, registered
//! through the raw C API because `duckdb-rs` wires no aggregate path.
//!
//! The aggregate's update collects the distinct texts of the rows the
//! database scans, combine merges the per-thread partials, and finalize
//! judges them all once through the engine's batch door and returns how
//! many texts were asked. The engine owns the fan, the cache, and the
//! width; what stays here is the aggregate's own shape and the poison
//! map, because no error-reporting call works from an aggregate callback
//! on this C API version.

use std::collections::HashSet;
use std::ffi::CString;

use duckdb::ffi;

use crate::{judge_warm, read_raw_column};

/// The aggregate's state: one question argument and the distinct texts
/// seen.
#[derive(Default)]
struct WarmState {
    question: String,
    seen: HashSet<String>,
}

/// The state slot points at the `state_size` bytes DuckDB allocated, and
/// the first eight of them hold the boxed `WarmState`. DuckDB moves state
/// blocks during hash-table growth; a byte copy preserves the payload.
unsafe fn warm_state(handle: ffi::duckdb_aggregate_state) -> &'static mut WarmState {
    let boxed = unsafe { *(handle as *mut *mut WarmState) };
    unsafe { &mut *boxed }
}

unsafe extern "C" fn state_size(_: ffi::duckdb_function_info) -> u64 {
    std::mem::size_of::<*mut WarmState>() as u64
}

unsafe extern "C" fn state_init(_: ffi::duckdb_function_info, state: ffi::duckdb_aggregate_state) {
    unsafe { *(state as *mut *mut WarmState) = Box::into_raw(Box::default()) };
}

unsafe extern "C" fn state_destroy(states: *mut ffi::duckdb_aggregate_state, count: u64) {
    for i in 0..count as usize {
        let handle = unsafe { *states.add(i) };
        if !handle.is_null() {
            drop(unsafe { Box::from_raw(*(handle as *mut *mut WarmState)) });
        }
    }
}

unsafe extern "C" fn update(
    _: ffi::duckdb_function_info,
    input: ffi::duckdb_data_chunk,
    states: *mut ffi::duckdb_aggregate_state,
) {
    let questions = unsafe { read_raw_column(input, 0) };
    let texts = unsafe { read_raw_column(input, 1) };
    for (i, (question, text)) in questions.iter().zip(&texts).enumerate() {
        // A NULL in either column never reaches the state, and so never
        // costs a request.
        let (Some(question), Some(text)) = (question, text) else {
            continue;
        };
        let handle = unsafe { *states.add(i) };
        let warm = unsafe { warm_state(handle) };
        warm.question = question.clone();
        warm.seen.insert(text.clone());
    }
}

unsafe extern "C" fn combine(
    _: ffi::duckdb_function_info,
    source: *mut ffi::duckdb_aggregate_state,
    target: *mut ffi::duckdb_aggregate_state,
    count: u64,
) {
    for i in 0..count as usize {
        let from = unsafe { warm_state(*source.add(i)) };
        let to = unsafe { warm_state(*target.add(i)) };
        if to.question.is_empty() {
            to.question = from.question.clone();
        }
        let moved = std::mem::take(&mut from.seen);
        to.seen.extend(moved);
    }
}

unsafe extern "C" fn finalize(
    _: ffi::duckdb_function_info,
    source: *mut ffi::duckdb_aggregate_state,
    result: ffi::duckdb_vector,
    count: u64,
    offset: u64,
) {
    let out = unsafe { ffi::duckdb_vector_get_data(result) } as *mut i64;
    for i in 0..count as usize {
        let warm = unsafe { warm_state(*source.add(i)) };
        let judged = judge_warm(&warm.question, std::mem::take(&mut warm.seen)).unwrap_or(0);
        unsafe { *out.add(offset as usize + i) = judged as i64 };
    }
}

/// Register the aggregate on a raw connection.
pub unsafe fn register(con: ffi::duckdb_connection) -> Result<(), String> {
    unsafe {
        let aggregate = ffi::duckdb_create_aggregate_function();
        let name = CString::new("thinkthen_warm").expect("the name holds no NUL byte");
        ffi::duckdb_aggregate_function_set_name(aggregate, name.as_ptr());
        let varchar = ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR);
        ffi::duckdb_aggregate_function_add_parameter(aggregate, varchar);
        let varchar = ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR);
        ffi::duckdb_aggregate_function_add_parameter(aggregate, varchar);
        let bigint = ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_BIGINT);
        ffi::duckdb_aggregate_function_set_return_type(aggregate, bigint);
        ffi::duckdb_aggregate_function_set_functions(
            aggregate,
            Some(state_size),
            Some(state_init),
            Some(update),
            Some(combine),
            Some(finalize),
        );
        ffi::duckdb_aggregate_function_set_destructor(aggregate, Some(state_destroy));
        if ffi::duckdb_register_aggregate_function(con, aggregate) != ffi::DuckDBSuccess {
            let mut aggregate = aggregate;
            ffi::duckdb_destroy_aggregate_function(&mut aggregate);
            return Err("thinkthen_warm did not register".into());
        }
        let mut aggregate = aggregate;
        ffi::duckdb_destroy_aggregate_function(&mut aggregate);
        Ok(())
    }
}
