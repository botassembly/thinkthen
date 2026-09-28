//! Every `unsafe` line of `thinkthen_usage()` and the `thinkthen_warm`
//! aggregate. Each callback runs its body through `guarded`.
#![allow(
    unsafe_code,
    reason = "a table function and an aggregate are DuckDB C API callback sets"
)]

use std::ffi::c_void;

use libduckdb_sys as sys;

use super::{Warm, usage_rows};
use crate::errors::{defect, guarded};
use crate::ffi::{Logical, Type, Value, message, read_column, write};

unsafe extern "C" fn drop_flag(flag: *mut c_void) {
    let _ = guarded("usage state destruction", || {
        if !flag.is_null() {
            // SAFETY: the flag is a box this file leaked.
            drop(unsafe { Box::from_raw(flag.cast::<bool>()) });
        }
        Ok(())
    });
}

unsafe extern "C" fn usage_bind(info: sys::duckdb_bind_info) {
    let bound = guarded("usage bind", || {
        // SAFETY: each type is copied by the bind, then destroyed.
        unsafe {
            let text = Logical::new(&Type::Text);
            sys::duckdb_bind_add_result_column(info, c"metric".as_ptr(), text.0);
            let number = Logical::new(&Type::BigInt);
            sys::duckdb_bind_add_result_column(info, c"value".as_ptr(), number.0);
        }
        Ok(())
    });
    if let Err(text) = bound {
        // SAFETY: a valid bind info.
        unsafe { sys::duckdb_bind_set_error(info, message(&text).as_ptr()) };
    }
}

unsafe extern "C" fn usage_init(info: sys::duckdb_init_info) {
    let made = guarded("usage init", || Ok(Box::new(false)));
    // SAFETY: DuckDB owns the flag and calls `drop_flag` once.
    unsafe {
        match made {
            Ok(flag) => {
                sys::duckdb_init_set_init_data(info, Box::into_raw(flag).cast(), Some(drop_flag))
            }
            Err(text) => sys::duckdb_init_set_error(info, message(&text).as_ptr()),
        }
    }
}

unsafe extern "C" fn usage_scan(info: sys::duckdb_function_info, output: sys::duckdb_data_chunk) {
    let scanned = guarded("usage scan", || {
        // SAFETY: the init data is this scan's flag; the output has two
        // columns with room for four rows.
        unsafe {
            let done = sys::duckdb_function_get_init_data(info)
                .cast::<bool>()
                .as_mut()
                .ok_or_else(|| defect("the usage scan ran with no init state"))?;
            if *done {
                sys::duckdb_data_chunk_set_size(output, 0);
                return Ok(());
            }
            *done = true;
            let rows = usage_rows();
            let (names, values) = (
                sys::duckdb_data_chunk_get_vector(output, 0),
                sys::duckdb_data_chunk_get_vector(output, 1),
            );
            for (row, (name, count)) in rows.iter().enumerate() {
                write(names, &Type::Text, row, &Value::Text((*name).to_owned()));
                write(values, &Type::BigInt, row, &Value::Int(*count));
            }
            sys::duckdb_data_chunk_set_size(output, rows.len() as u64);
        }
        Ok(())
    });
    if let Err(text) = scanned {
        // SAFETY: a valid function info and chunk.
        unsafe {
            sys::duckdb_function_set_error(info, message(&text).as_ptr());
            sys::duckdb_data_chunk_set_size(output, 0);
        }
    }
}

pub(crate) fn register_usage(connection: sys::duckdb_connection) -> Result<(), String> {
    crate::ffi::register_table(
        connection,
        c"thinkthen_usage",
        &[],
        usage_bind,
        usage_init,
        usage_scan,
    )
}

// ---- thinkthen_warm(question, text) ----

/// A warm state block holds one pointer to a boxed [`Warm`]. DuckDB moves
/// state blocks by byte copy, which keeps the pointer.
fn warm_at<'a>(state: sys::duckdb_aggregate_state) -> Result<&'a mut Warm, String> {
    // SAFETY: `warm_init` wrote a leaked box into every block.
    unsafe {
        state
            .cast::<*mut Warm>()
            .as_ref()
            .and_then(|boxed| boxed.as_mut())
    }
    .ok_or_else(|| defect("the warm aggregate ran with no state"))
}

fn warm_error(info: sys::duckdb_function_info, outcome: Result<(), String>) {
    if let Err(text) = outcome {
        // SAFETY: a valid function info.
        unsafe { sys::duckdb_aggregate_function_set_error(info, message(&text).as_ptr()) };
    }
}

unsafe extern "C" fn warm_size(_: sys::duckdb_function_info) -> u64 {
    size_of::<*mut Warm>() as u64
}

unsafe extern "C" fn warm_init(
    info: sys::duckdb_function_info,
    state: sys::duckdb_aggregate_state,
) {
    warm_error(
        info,
        guarded("warm init", || {
            // SAFETY: DuckDB allocated `warm_size` bytes for this block.
            unsafe { *state.cast::<*mut Warm>() = Box::into_raw(Box::default()) };
            Ok(())
        }),
    );
}

unsafe extern "C" fn warm_destroy(states: *mut sys::duckdb_aggregate_state, count: u64) {
    let _ = guarded("warm destruction", || {
        for index in 0..count as usize {
            // SAFETY: each block holds a box `warm_init` leaked, freed once.
            unsafe {
                let boxed = *(*states.add(index)).cast::<*mut Warm>();
                if !boxed.is_null() {
                    drop(Box::from_raw(boxed));
                }
            }
        }
        Ok(())
    });
}

unsafe extern "C" fn warm_update(
    info: sys::duckdb_function_info,
    input: sys::duckdb_data_chunk,
    states: *mut sys::duckdb_aggregate_state,
) {
    warm_error(
        info,
        guarded("warm update", || {
            // SAFETY: two VARCHAR columns and one state per row.
            let rows = unsafe { sys::duckdb_data_chunk_get_size(input) } as usize;
            let read = |index| {
                read_column(
                    unsafe { sys::duckdb_data_chunk_get_vector(input, index) },
                    &Type::Text,
                    rows,
                )
            };
            let (questions, texts) = (read(0), read(1));
            for row in 0..rows {
                if let (Some(question), Some(text)) = (questions.text(row), texts.text(row)) {
                    // SAFETY: as above.
                    warm_at(unsafe { *states.add(row) })?.add(question, text)?;
                }
            }
            Ok(())
        }),
    );
}

unsafe extern "C" fn warm_combine(
    info: sys::duckdb_function_info,
    source: *mut sys::duckdb_aggregate_state,
    target: *mut sys::duckdb_aggregate_state,
    count: u64,
) {
    warm_error(
        info,
        guarded("warm combine", || {
            for index in 0..count as usize {
                // SAFETY: both arrays hold `count` states.
                let (from, to) = unsafe { (*source.add(index), *target.add(index)) };
                warm_at(to)?.merge(warm_at(from)?)?;
            }
            Ok(())
        }),
    );
}

unsafe extern "C" fn warm_finalize(
    info: sys::duckdb_function_info,
    source: *mut sys::duckdb_aggregate_state,
    result: sys::duckdb_vector,
    count: u64,
    offset: u64,
) {
    warm_error(
        info,
        guarded("warm finalize", || {
            // SAFETY: `register_warm` set a boxed serial that lives as long
            // as the function.
            let serial = unsafe {
                sys::duckdb_aggregate_function_get_extra_info(info)
                    .cast::<u64>()
                    .as_ref()
                    .copied()
            }
            .ok_or_else(|| defect("the warm aggregate carries no serial"))?;
            for index in 0..count as usize {
                // SAFETY: the array holds `count` states.
                let asked = warm_at(unsafe { *source.add(index) })?.finish(serial)?;
                write(
                    result,
                    &Type::BigInt,
                    offset as usize + index,
                    &Value::Int(asked),
                );
            }
            Ok(())
        }),
    );
}

unsafe extern "C" fn drop_serial(serial: *mut std::ffi::c_void) {
    if !serial.is_null() {
        // SAFETY: `register_warm` leaked this box, and DuckDB frees it once.
        drop(unsafe { Box::from_raw(serial.cast::<u64>()) });
    }
}

pub(crate) fn register_warm(connection: sys::duckdb_connection, serial: u64) -> Result<(), String> {
    // SAFETY: as in `register_scalar`.
    unsafe {
        let mut aggregate = sys::duckdb_create_aggregate_function();
        sys::duckdb_aggregate_function_set_name(aggregate, c"thinkthen_warm".as_ptr());
        for of in [&Type::Text, &Type::Text] {
            let made = Logical::new(of);
            sys::duckdb_aggregate_function_add_parameter(aggregate, made.0);
        }
        let result = Logical::new(&Type::BigInt);
        sys::duckdb_aggregate_function_set_return_type(aggregate, result.0);
        sys::duckdb_aggregate_function_set_functions(
            aggregate,
            Some(warm_size),
            Some(warm_init),
            Some(warm_update),
            Some(warm_combine),
            Some(warm_finalize),
        );
        sys::duckdb_aggregate_function_set_destructor(aggregate, Some(warm_destroy));
        sys::duckdb_aggregate_function_set_extra_info(
            aggregate,
            Box::into_raw(Box::new(serial)).cast(),
            Some(drop_serial),
        );
        let state = sys::duckdb_register_aggregate_function(connection, aggregate);
        sys::duckdb_destroy_aggregate_function(&raw mut aggregate);
        if state == sys::duckdb_state_DuckDBSuccess {
            Ok(())
        } else {
            Err("thinkthen_warm did not register".to_owned())
        }
    }
}
