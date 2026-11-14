//! Complete status-bearing host and canonical saved projections share typed row ownership.
#![allow(
    unsafe_code,
    reason = "serialization borrows a live row and exclusive output slots"
)]
use crate::{
    Buffer, HostError, HostRow,
    exports::{THINKTHEN_OK, operation},
    row,
    values::Owned,
};
use thinkthen::native;

unsafe fn encode(row: *const HostRow, out: *mut Buffer, error: *mut HostError, host: bool) -> i32 {
    // SAFETY: the caller lends a live row and exclusive output slots.
    unsafe {
        operation(error, || {
            out.write(Buffer::default());
            let data = row::data(row)?;
            let mut json = match &data.value.0 {
                Owned::Row(row) if host => serde_json::to_string(&native::host_row_json(row)?.0),
                Owned::Row(row) => serde_json::to_string(row),
                Owned::Entry(entry) if host => {
                    serde_json::to_string(&native::host_entry_json(entry)?)
                }
                Owned::Entry(entry) => serde_json::to_string(entry),
            }
            .map_err(|_| native::internal_error())?;
            // SQL-selected arbitrary JSON IDs belong only to the host transport.
            // Saved rows retain the admitted typed ID and always reload under their schema.
            if host && let Some(id) = &data.id_override {
                // Preserve the selected JSON bytes, including null and integers beyond u64.
                let mut value: serde_json::Map<String, serde_json::Value> =
                    serde_json::from_str(&json).map_err(|_| native::internal_error())?;
                value.remove("id");
                json = serde_json::to_string(&value).map_err(|_| native::internal_error())?;
                json.pop();
                json.push_str(",\"id\":");
                json.push_str(id.get());
                json.push('}');
            }
            out.write(Buffer::new(json));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Write the complete row as saved JSON, with character offsets and no original record.
/// Works for answered, unsure and failed rows and form entries. The independent buffer is yours.
/// # Safety
/// row is live; out is writable and empty; error is NULL or initialized. Free out with
/// thinkthen_buffer_free before filling it again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_to_json(
    row: *const HostRow,
    out: *mut Buffer,
    error: *mut HostError,
) -> i32 {
    // SAFETY: encode only borrows the typed row.
    unsafe { encode(row, out, error, false) }
}
/// Write the complete status-bearing row JSON in the call's host offset units.
/// This projection serves native adapters; saved rows use thinkthen_row_to_json instead.
/// The independent output buffer belongs to the caller, including for failed rows.
/// # Safety
/// row is live; out is writable and empty; error is NULL or initialized. Free out with
/// thinkthen_buffer_free before filling it again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_host_json(
    row: *const HostRow,
    out: *mut Buffer,
    error: *mut HostError,
) -> i32 {
    // SAFETY: encode only borrows the typed row.
    unsafe { encode(row, out, error, true) }
}
