//! Lossless row metadata projection for foreign JSON libraries.
#![allow(
    unsafe_code,
    reason = "the row projection borrows counted C inputs and publishes an owned buffer"
)]

use crate::exports::operation;
use crate::memory::text;
use crate::{Buffer, HostRow};
use serde_json::{Value, value::RawValue};
use std::collections::BTreeMap;
use thinkthen::{Error, ErrorKind};

/// Project row metadata without interpreting arbitrary answer or identifier keys.
/// Identifier, answer and error are JSON strings containing their original JSON bytes;
/// absent fields stay absent. Status and probability remain ordinary scalars.
/// # Safety
/// Row is a live owned row. Output and error are distinct writable empty slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_columns(
    row: *const HostRow,
    out: *mut Buffer,
    error: *mut Buffer,
) -> i32 {
    // SAFETY: the required output slot is exclusive and owns no buffer.
    unsafe { out.write(Buffer::default()) };
    // SAFETY: the row and its counted bytes remain live through this operation.
    unsafe {
        operation(error, || {
            let fields: BTreeMap<String, &RawValue> =
                serde_json::from_str(text((*row).fields.data, (*row).fields.len)?)
                    .map_err(column_error)?;
            let mut columns = serde_json::Map::new();
            for (name, value) in fields {
                let value = match name.as_str() {
                    "id" | "answer" | "error" => Value::String(value.get().to_owned()),
                    "status" | "p" => serde_json::from_str(value.get()).map_err(column_error)?,
                    _ => continue,
                };
                columns.insert(name, value);
            }
            out.write(Buffer::new(Value::Object(columns).to_string()));
            Ok(0)
        })
    }
}

fn column_error(error: serde_json::Error) -> Error {
    Error::new(ErrorKind::Io, error.to_string())
}
