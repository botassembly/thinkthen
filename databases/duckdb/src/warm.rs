//! `thinkthen_warm(question, text)` as a DuckDB aggregate, registered
//! through the raw C API because `duckdb-rs` wires no aggregate path.
//!
//! The aggregate's update collects the distinct texts of the rows the
//! database scans and binds the question the first row carries, combine
//! merges the per-thread partials, and finalize judges them all once
//! through the engine's batch door and returns how many texts were asked.
//! The engine owns the fan, the cache, and the width. The question binds
//! at the first row instead of following the last one seen (review
//! finding: a group whose rows carried different questions judged every
//! text under whichever question came last), and a group carrying more
//! than one question errors. A failure sets the aggregate's own error
//! through `duckdb_aggregate_function_set_error`, so the query that ran
//! the warm hears it — no poison map, and nothing counted as zero.

use std::collections::HashSet;
use std::ffi::CString;

use duckdb::ffi;

use crate::{guard, judge_warm, read_raw_column};

/// The aggregate's state: the question argument the first row bound and
/// the distinct texts seen.
#[derive(Default)]
struct WarmState {
    question: Option<String>,
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
    guard::contained_with(
        "warm state size",
        std::mem::size_of::<*mut WarmState>() as u64,
        || std::mem::size_of::<*mut WarmState>() as u64,
    )
}

unsafe extern "C" fn state_init(_: ffi::duckdb_function_info, state: ffi::duckdb_aggregate_state) {
    guard::contained_quiet("warm state init", || unsafe {
        *(state as *mut *mut WarmState) = Box::into_raw(Box::default());
    });
}

unsafe extern "C" fn state_destroy(states: *mut ffi::duckdb_aggregate_state, count: u64) {
    guard::contained_quiet("warm state destruction", || unsafe {
        for i in 0..count as usize {
            let handle = *states.add(i);
            if !handle.is_null() {
                drop(Box::from_raw(*(handle as *mut *mut WarmState)));
            }
        }
    });
}

unsafe extern "C" fn update(
    info: ffi::duckdb_function_info,
    input: ffi::duckdb_data_chunk,
    states: *mut ffi::duckdb_aggregate_state,
) {
    guard::contained_quiet("warm update", || unsafe {
        let questions = read_raw_column(input, 0);
        let texts = read_raw_column(input, 1);
        for (i, (question, text)) in questions.iter().zip(&texts).enumerate() {
            // A NULL in either column never reaches the state, and so never
            // costs a request.
            let (Some(question), Some(text)) = (question, text) else {
                continue;
            };
            let handle = *states.add(i);
            let warm = warm_state(handle);
            match warm.question.as_deref() {
                None => warm.question = Some(question.clone()),
                Some(bound) if bound == question => {}
                Some(bound) => {
                    set_error(
                        info,
                        &mixed_questions(bound, question),
                    );
                    return;
                }
            }
            warm.seen.insert(text.clone());
        }
    });
}

unsafe extern "C" fn combine(
    info: ffi::duckdb_function_info,
    source: *mut ffi::duckdb_aggregate_state,
    target: *mut ffi::duckdb_aggregate_state,
    count: u64,
) {
    guard::contained_quiet("warm combine", || unsafe {
        for i in 0..count as usize {
            let from = warm_state(*source.add(i));
            let to = warm_state(*target.add(i));
            match (to.question.as_deref(), from.question.as_deref()) {
                (None, Some(question)) => to.question = Some(question.to_owned()),
                (Some(bound), Some(question)) if bound != question => {
                    set_error(info, &mixed_questions(bound, question));
                    return;
                }
                _ => {}
            }
            let moved = std::mem::take(&mut from.seen);
            to.seen.extend(moved);
        }
    });
}

unsafe extern "C" fn finalize(
    info: ffi::duckdb_function_info,
    source: *mut ffi::duckdb_aggregate_state,
    result: ffi::duckdb_vector,
    count: u64,
    offset: u64,
) {
    guard::contained_quiet("warm finalize", || unsafe {
        let out = ffi::duckdb_vector_get_data(result) as *mut i64;
        for i in 0..count as usize {
            let warm = warm_state(*source.add(i));
            let question = warm.question.clone().unwrap_or_default();
            let seen = std::mem::take(&mut warm.seen);
            match judge_warm(&question, seen) {
                Ok(judged) => *out.add(offset as usize + i) = judged as i64,
                Err(message) => {
                    set_error(info, &message);
                    return;
                }
            }
        }
    });
}

/// The error a group carrying more than one question earns.
fn mixed_questions(bound: &str, seen: &str) -> String {
    format!(
        "thinkthen usage: thinkthen_warm judges one question per group, and this group carries more than one: {bound:?} and {seen:?}"
    )
}

/// Set the aggregate's own error, the channel DuckDB raises with the
/// query's own message.
unsafe fn set_error(info: ffi::duckdb_function_info, message: &str) {
    if let Ok(text) = CString::new(message) {
        unsafe { ffi::duckdb_aggregate_function_set_error(info, text.as_ptr()) };
    }
}

/// Register the aggregate on a raw connection.
pub unsafe fn register(con: ffi::duckdb_connection) -> Result<(), String> {
    unsafe {
        let aggregate = ffi::duckdb_create_aggregate_function();
        let name = CString::new("thinkthen_warm").expect("the name holds no NUL byte");
        ffi::duckdb_aggregate_function_set_name(aggregate, name.as_ptr());
        // The function copies each type it is handed, so each one is
        // destroyed after use (review 5: these leaked on every LOAD).
        let mut varchar = ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR);
        ffi::duckdb_aggregate_function_add_parameter(aggregate, varchar);
        ffi::duckdb_aggregate_function_add_parameter(aggregate, varchar);
        ffi::duckdb_destroy_logical_type(&mut varchar);
        let mut bigint = ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_BIGINT);
        ffi::duckdb_aggregate_function_set_return_type(aggregate, bigint);
        ffi::duckdb_destroy_logical_type(&mut bigint);
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
