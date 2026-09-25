//! Every `unsafe` line of the scalars: their init, invoke, and
//! registration. Each callback runs its body through `guarded`.
#![allow(unsafe_code, reason = "a scalar is a DuckDB C API callback set")]

use std::ffi::c_void;

use libduckdb_sys as sys;

use super::Scalar;
use crate::engines;
use crate::errors::{defect, guarded};
use crate::ffi::{Column, Logical, Type, message, read_column, write};
use crate::questions::{Caller, Files};
use crate::signal::Invoke;

/// The per-thread init: the caller's handles, its settings, the cache
/// folder's check, and the engine, before any row runs.
unsafe extern "C" fn scalar_init(info: sys::duckdb_init_info) {
    let made = guarded("scalar init", || {
        let files = Files::of(info)?;
        let asked = files.settings();
        let engine = engines::engine_for(&asked, |path| files.probe(path))?;
        Ok(Box::new(Caller::new(engine, asked, files)))
    });
    // SAFETY: DuckDB owns the state and calls `drop_caller` once.
    unsafe {
        match made {
            Ok(caller) => sys::duckdb_scalar_function_init_set_state(
                info,
                Box::into_raw(caller).cast(),
                Some(drop_caller),
            ),
            Err(text) => sys::duckdb_scalar_function_init_set_error(info, message(&text).as_ptr()),
        }
    }
}

unsafe extern "C" fn drop_caller(state: *mut c_void) {
    let _ = guarded("scalar state destruction", || {
        if !state.is_null() {
            // SAFETY: the state is the box `scalar_init` leaked.
            drop(unsafe { Box::from_raw(state.cast::<Caller>()) });
        }
        Ok(())
    });
}

unsafe extern "C" fn scalar_invoke(
    info: sys::duckdb_function_info,
    input: sys::duckdb_data_chunk,
    output: sys::duckdb_vector,
) {
    if let Err(text) = guarded("scalar", || answer(info, input, output)) {
        // SAFETY: a valid function info.
        unsafe { sys::duckdb_scalar_function_set_error(info, message(&text).as_ptr()) };
    }
}

fn answer(
    info: sys::duckdb_function_info,
    input: sys::duckdb_data_chunk,
    output: sys::duckdb_vector,
) -> Result<(), String> {
    // SAFETY: extra info is a `&'static Scalar`; the state is this thread's
    // own `Caller`, or null when DuckDB ran no init.
    let (scalar, caller, rows, count) = unsafe {
        (
            sys::duckdb_scalar_function_get_extra_info(info)
                .cast::<Scalar>()
                .as_ref(),
            sys::duckdb_scalar_function_get_state(info)
                .cast::<Caller>()
                .as_mut(),
            sys::duckdb_data_chunk_get_size(input) as usize,
            sys::duckdb_data_chunk_get_column_count(input),
        )
    };
    let scalar = scalar.ok_or_else(|| defect("the scalar ran with no function record"))?;
    let caller = caller.ok_or_else(|| defect("the scalar ran with no init state"))?;
    let columns: Vec<Column> = (0..count)
        .map(|index| {
            let of = scalar
                .arguments
                .get(index as usize)
                .unwrap_or(&Type::BigInt);
            // SAFETY: the index is below the chunk's column count.
            read_column(
                unsafe { sys::duckdb_data_chunk_get_vector(input, index) },
                of,
                rows,
            )
        })
        .collect();
    let invoke = Invoke::begin();
    let values = crate::scalars::run(scalar.verb, caller, &invoke, &columns, rows)?;
    for (row, value) in values.iter().enumerate() {
        write(output, &scalar.result, row, value);
    }
    Ok(())
}

pub(crate) fn register_scalar(
    connection: sys::duckdb_connection,
    scalar: &'static Scalar,
) -> Result<(), String> {
    let name = message(scalar.name);
    // SAFETY: each created object is copied on add or register, then
    // destroyed once.
    unsafe {
        let mut set = sys::duckdb_create_scalar_function_set(name.as_ptr());
        for deadline in [false, true]
            .into_iter()
            .take(if scalar.deadline { 2 } else { 1 })
        {
            let mut function = sys::duckdb_create_scalar_function();
            sys::duckdb_scalar_function_set_name(function, name.as_ptr());
            let deadline_type = deadline.then_some(&Type::BigInt);
            for argument in scalar.arguments.iter().chain(deadline_type) {
                let made = Logical::new(argument);
                sys::duckdb_scalar_function_add_parameter(function, made.0);
            }
            let result = Logical::new(&scalar.result);
            sys::duckdb_scalar_function_set_return_type(function, result.0);
            sys::duckdb_scalar_function_set_function(function, Some(scalar_invoke));
            sys::duckdb_scalar_function_set_init(function, Some(scalar_init));
            sys::duckdb_scalar_function_set_volatile(function);
            sys::duckdb_scalar_function_set_extra_info(
                function,
                std::ptr::from_ref(scalar).cast_mut().cast(),
                None,
            );
            sys::duckdb_add_scalar_function_to_set(set, function);
            sys::duckdb_destroy_scalar_function(&raw mut function);
        }
        let state = sys::duckdb_register_scalar_function_set(connection, set);
        sys::duckdb_destroy_scalar_function_set(&raw mut set);
        if state == sys::duckdb_state_DuckDBSuccess {
            Ok(())
        } else {
            Err(format!("{} did not register", scalar.name))
        }
    }
}
