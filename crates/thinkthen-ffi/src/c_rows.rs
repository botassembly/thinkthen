//! The C row is the single loud read; views borrow its independently owned payload.
#![allow(
    unsafe_code,
    reason = "C lends initialized rows and exclusive output slots"
)]
use crate::{
    HostError, HostRow,
    exports::{THINKTHEN_OK, operation},
    memory::{self},
    row,
    values::{self, Owned, ValueHandle},
};
use std::{ffi::c_char, ptr};
use thinkthen::{Code, Error, FunctionName, native, questions};

/// Borrow the row's NUL-terminated display text, valid until the row is freed or filled again.
/// # Safety
/// row is NULL or an initialized live row, with no concurrent mutation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_string(row: *const HostRow) -> *const c_char {
    // SAFETY: the row is borrowed until return; the allocation owns this C string.
    unsafe { row::data(row).map_or(c"".as_ptr(), |data| data.display.as_ptr()) }
}
/// Borrow the error's NUL-terminated code-and-message display until it is freed or filled again.
/// # Safety
/// error is NULL or an initialized live error, with no concurrent mutation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_string(error: *const HostError) -> *const c_char {
    // SAFETY: an initialized error owns its message buffer and its trailing NUL.
    unsafe {
        if memory::error_size(error.cast_mut()).is_some() && !(*error).message.data.is_null() {
            (*error).message.data.add((*error).message.len + 1).cast()
        } else {
            c"".as_ptr()
        }
    }
}
/// Return a failed row's code, including the first failed form entry; uncertainty is success.
/// # Safety
/// row is live and initialized; error is NULL or an initialized exclusive error slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_err_if_failed(
    row: *const HostRow,
    error: *mut HostError,
) -> i32 {
    // SAFETY: this call only borrows the payload and fills the error.
    unsafe {
        operation(error, || {
            let value = &row::data(row)?.value;
            let failure = match &value.0 {
                Owned::Row(row) => row.failure(),
                Owned::Entry(_) => value.error()?,
            };
            if let Some(error) = failure {
                return Err(error.clone());
            }
            Ok(THINKTHEN_OK)
        })
    }
}
unsafe fn delegate<T>(
    row: *const HostRow,
    out: *mut T,
    error: *mut HostError,
    read: unsafe extern "C" fn(*const ValueHandle, *mut T, *mut HostError) -> i32,
) -> i32 {
    // SAFETY: the row owns the borrowed value; the output slots remain exclusive.
    unsafe {
        match row::data(row) {
            Ok(data) => read(&data.value, out, error),
            Err(e) => operation(error, || Err(e)),
        }
    }
}
/// Read a typed row field. Missing answers leave the output unchanged.
/// # Safety
/// row is live; out is writable and error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_answer_score(
    row: *const HostRow,
    out: *mut f64,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the existing typed value accessor borrows this row owner.
    unsafe { delegate(row, out, error, crate::thinkthen_value_number) }
}
/// Read a typed row field. Missing answers leave the output unchanged.
/// # Safety
/// row is live; out is writable and error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_answer_rank(
    row: *const HostRow,
    out: *mut u64,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the existing typed value accessor borrows this row owner.
    unsafe { delegate(row, out, error, crate::thinkthen_value_rank) }
}
/// Read a typed row field. Missing answers leave the output unchanged.
/// # Safety
/// row is live; out is writable and error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_answer_span(
    row: *const HostRow,
    out: *mut values::SpanView,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the existing typed value accessor borrows this row owner.
    unsafe { delegate(row, out, error, crate::thinkthen_value_span) }
}
/// Read a typed row field. Missing answers leave the output unchanged.
/// # Safety
/// row is live; out is writable and error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_id(
    row: *const HostRow,
    out: *mut values::IdView,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the existing typed value accessor borrows this row owner.
    unsafe { delegate(row, out, error, crate::thinkthen_value_id) }
}
/// Read a decide answer into a bool, leaving it unchanged when there is no answer.
/// # Safety
/// row is initialized and live; out is writable and error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_answer_bool(
    row: *const HostRow,
    out: *mut bool,
    error: *mut HostError,
) -> i32 {
    // SAFETY: payload is live and out is exclusive until return.
    unsafe {
        read_answer(row, out, error, FunctionName::Decide, |data| {
            Ok(*data.value.answer::<questions::Decide>()?)
        })
    }
}

unsafe fn read_answer<T>(
    row: *const HostRow,
    out: *mut T,
    error: *mut HostError,
    function: FunctionName,
    read: impl FnOnce(&row::RowData) -> Result<T, Error>,
) -> i32 {
    // SAFETY: the caller lends a live row and exclusive output.
    unsafe {
        operation(error, || {
            let data = row::data(row)?;
            if data.value.function()? != function {
                return Err(native::argument_error(Code::UsageFunction));
            }
            if data.value.status()? == thinkthen::Status::Unsure {
                return Ok(crate::THINKTHEN_UNSURE);
            }
            if let Some(error) = data.value.error()? {
                return Err(error.clone());
            }
            out.write(read(data)?);
            Ok(THINKTHEN_OK)
        })
    }
}
/// Read the chosen word's zero-based position in the admitted word array.
/// # Safety
/// row is initialized and live; out is writable and error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_answer_choice(
    row: *const HostRow,
    out: *mut i32,
    error: *mut HostError,
) -> i32 {
    // SAFETY: read_answer preserves the output when an answer is absent.
    unsafe {
        read_answer(row, out, error, FunctionName::Choose, |data| {
            let answer = data.value.answer::<questions::Choose>()?;
            data.domain
                .words
                .iter()
                .position(|word| word == answer)
                .and_then(|i| i32::try_from(i).ok())
                .ok_or_else(native::abi_error)
        })
    }
}
/// Read one bool for every admitted tag label, in its original order.
/// # Safety
/// row is initialized and live; out holds count writable bools; error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_answer_tags(
    row: *const HostRow,
    out: *mut bool,
    count: usize,
    error: *mut HostError,
) -> i32 {
    // SAFETY: all validation precedes writing the caller's complete bool array.
    unsafe {
        operation(error, || {
            let data = row::data(row)?;
            if data.value.function()? != FunctionName::Tag {
                return Err(native::argument_error(Code::UsageFunction));
            }
            if let Some(error) = data.value.error()? {
                return Err(error.clone());
            }
            let answer = data.value.answer::<questions::Tag>()?;
            if count != data.domain.words.len() || (count > 0 && out.is_null()) {
                return Err(native::argument_error(Code::UsageArgument));
            }
            for (i, label) in data.domain.words.iter().enumerate() {
                out.add(i).write(answer.contains(label));
            }
            Ok(THINKTHEN_OK)
        })
    }
}
pub(crate) unsafe fn project_list<T>(
    row: *const HostRow,
    out: *mut *const T,
    count: *mut usize,
    error: *mut HostError,
    read: impl FnOnce(&row::RowData) -> Result<&[T], Error>,
) -> i32 {
    // SAFETY: the row owns the borrowed list and callers lend exclusive output slots.
    unsafe {
        operation(error, || {
            let data = row::data(row)?;
            let list = read(data)?;
            out.write(list.as_ptr());
            count.write(list.len());
            Ok(THINKTHEN_OK)
        })
    }
}
/// Borrow the contiguous answer list until the row is freed or filled again.
/// # Safety
/// row is live; out and count are writable; error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_answer_names(
    row: *const HostRow,
    out: *mut *const values::NameView,
    count: *mut usize,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the row owns the list and its strings.
    unsafe {
        project_list(row, out, count, error, |d| {
            d.value.answer::<questions::Recognize>()?;
            Ok(&d.names)
        })
    }
}
/// Borrow the contiguous answer list until the row is freed or filled again.
/// # Safety
/// row is live; out and count are writable; error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_answer_relations(
    row: *const HostRow,
    out: *mut *const values::RelationView,
    count: *mut usize,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the row owns the list and its strings.
    unsafe {
        project_list(row, out, count, error, |d| {
            d.value.answer::<questions::Relate>()?;
            Ok(&d.relations)
        })
    }
}
/// Borrow every name in a relate row's detail, including names without a relation.
/// # Safety
/// row is live; out and count are writable, error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_detail_names(
    row: *const HostRow,
    out: *mut *const values::NameView,
    count: *mut usize,
    error: *mut HostError,
) -> i32 {
    // SAFETY: detail remains inspectable even for a failed row and borrows its owner.
    unsafe {
        operation(error, || {
            let data = row::data(row)?;
            data.value.detail::<questions::Relate>()?;
            out.write(data.names.as_ptr());
            count.write(data.names.len());
            Ok(THINKTHEN_OK)
        })
    }
}
/// Copy a named form entry into an independently owned row.
/// # Safety
/// row is live; name is readable; out and error are initialized exclusive output slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_answer_entry(
    row: *const HostRow,
    name: *const u8,
    len: usize,
    out: *mut HostRow,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the parent remains live until its typed entry and domain have been copied.
    unsafe {
        operation(error, || {
            let size = memory::row_size(out)?;
            if ptr::eq(row, out) {
                return Err(native::argument_error(Code::UsageArgument));
            }
            let data = row::data(row)?;
            let name = memory::text(name, len)?;
            let entry = data
                .value
                .answer::<questions::Annotate>()?
                .get(name)
                .ok_or_else(|| native::argument_error(Code::UsageName))?;
            let Owned::Row(thinkthen::results::AnyRow::Annotate(parent)) = &data.value.0 else {
                return Err(native::abi_error());
            };
            let domain = data.domain.entries.get(name).cloned().unwrap_or_default();
            let result = HostRow::from_value(
                size,
                ValueHandle::from_any(native::entry_row(parent, entry.clone())),
                domain,
            )?;
            if let Some(selected) = data.selected_snapshot(name) {
                (*result.private_data)
                    .selected
                    .insert(String::new(), selected);
            }
            HostRow::clear(out);
            result.write(out);
            Ok(THINKTHEN_OK)
        })
    }
}
