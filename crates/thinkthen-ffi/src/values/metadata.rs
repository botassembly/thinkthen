//! Typed metadata and independently owned complex-value snapshots.
#![allow(
    unsafe_code,
    reason = "projection exports borrow live value handles and exclusive output slots"
)]
use super::{IdView, Owned, SourceView, ValueHandle, project, views};
use crate::{
    HostError, HostRow,
    exports::{OK, operation},
    memory::error_size,
};
use thinkthen::{
    Code, native,
    records::{IdKind, RecordId},
};
/// Return 1 when a library with binary interface actual serves a caller built for required, and
/// 0 when not.
///
/// The major versions must match, and actual's minor must be at least required's. Any thread
/// may call this.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_abi_compatible(required: u32, actual: u32) -> u8 {
    u8::from(native::check_version(required, actual).is_ok())
}
/// Return how many codes thinkthen_code_at lists.
///
/// Any thread may call this.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_code_count() -> usize {
    Code::ALL.len()
}
/// Return the code at index, or 0 when index is out of range; 0 is never a code.
///
/// Any thread may call this.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_code_at(index: usize) -> i32 {
    Code::ALL.get(index).map_or(0, |code| code.abi())
}
/// Copy a row's answer into a new value, which the thinkthen_value_* functions read.
///
/// Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL:
/// -THINKTHEN_CODE_USAGE_ABI when the row is empty or did not come from this library. The
/// caller owns *out, which outlives the row, and frees it with thinkthen_value_free; an old
/// handle in *out is not freed.
/// # Safety
/// Any thread may call this while no thread changes or frees row. out is writable. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_values(
    row: *const HostRow,
    out: *mut *mut ValueHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends live buffers and exclusive outputs until their copy completes. */
    unsafe {
        operation(error, || {
            out.write(std::ptr::null_mut());
            let value = ValueHandle::from_row(row)?;
            out.write(Box::into_raw(Box::new(value)));
            Ok(OK)
        })
    }
}
/// Free a value.
///
/// Free a value. NULL is ignored. Views borrowed from it end here.
/// # Safety
/// Call this once, after every other use of value and its views has returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_free(value: *mut ValueHandle) {
    if !value.is_null() {
        /* SAFETY: the caller transfers this unique owner. */
        drop(unsafe { Box::from_raw(value) });
    }
}
/// Write the value's thinkthen_function to *out.
///
/// Returns THINKTHEN_OK with *out written.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_function(
    value: *const ValueHandle,
    out: *mut u32,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the value is borrowed and outputs are exclusive until return. */
    unsafe { project(out, error, || Ok((*value).function()?.abi())) }
}
/// Write the value's thinkthen_status to *out.
///
/// Returns THINKTHEN_OK with *out written.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_status(
    value: *const ValueHandle,
    out: *mut u32,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the value is borrowed and outputs are exclusive until return. */
    unsafe { project(out, error, || Ok((*value).status()?.abi())) }
}
/// Write which question, backend and model gave the value to *out.
///
/// Returns THINKTHEN_OK with *out written, or -THINKTHEN_CODE_USAGE_ABI with *out unchanged for
/// an annotation entry, which has no source. *out borrows from value and lasts until
/// thinkthen_value_free(value); never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_source(
    value: *const ValueHandle,
    out: *mut SourceView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value owns the returned strings; the output slots are exclusive. */
    unsafe {
        project(out, error, || {
            let Owned::Row(row) = &(*value).0 else {
                return Err(native::abi_error());
            };
            let source = row.source();
            Ok(SourceView {
                question_identity: views::text(source.question_identity()),
                backend: source.backend().abi(),
                model: views::text(source.model()),
            })
        })
    }
}
/// Write the record's id to *out, exactly: integers never pass through a double.
///
/// out->kind is one of the THINKTHEN_ID_* kinds. Returns THINKTHEN_OK with *out written, or
/// -THINKTHEN_CODE_USAGE_ABI with *out unchanged for an annotation entry. *out borrows from
/// value and lasts until thinkthen_value_free(value); never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_id(
    value: *const ValueHandle,
    out: *mut IdView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: string views borrow value; output slots are exclusive until return. */
    unsafe {
        project(out, error, || {
            let Owned::Row(row) = &(*value).0 else {
                return Err(native::abi_error());
            };
            let mut view = IdView {
                kind: IdKind::Absent.abi(),
                ..IdView::default()
            };
            match row.id() {
                None => {}
                Some(RecordId::String(text)) => {
                    view.kind = IdKind::String.abi();
                    view.text = views::text(text);
                }
                Some(RecordId::Integer(integer)) => {
                    view.kind = IdKind::Signed.abi();
                    view.signed_value = *integer;
                }
                Some(RecordId::Unsigned(integer)) => {
                    view.kind = IdKind::Unsigned.abi();
                    view.unsigned_value = *integer;
                }
                _ => return Err(native::abi_error()),
            }
            Ok(view)
        })
    }
}
/// Copy a failed value's code and message into *result; it stays empty for a value that did not
/// fail.
///
/// result is initialized with THINKTHEN_ERROR_INIT. Passing it again frees what it held, so a
/// reused result leaks nothing; free it with thinkthen_error_free. Returns THINKTHEN_OK, or
/// -THINKTHEN_CODE_USAGE_ABI when result is not initialized.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is live. result and
/// error are different variables. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
/// negative return it holds the code and message; passing it again frees what it held, so a
/// reused error leaks nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_error(
    value: *const ValueHandle,
    result: *mut HostError,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: output prefixes are checked before writes; value remains borrowed. */
    unsafe {
        operation(error, || {
            let size = error_size(result).ok_or_else(native::abi_error)?;
            HostError::clear(result, size);
            (match (*value).error()? {
                Some(error) => HostError::from_error(size, error),
                None => HostError::empty(size),
            })
            .write(result);
            Ok(OK)
        })
    }
}
/// Write the value's probability to *out; NaN when its function has none.
///
/// Returns THINKTHEN_OK with *out written.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_probability(
    value: *const ValueHandle,
    out: *mut f64,
    error: *mut HostError,
) -> i32 {
    fn probability<A, D>(outcome: &thinkthen::results::Outcome<A, D>) -> f64 {
        match outcome {
            thinkthen::results::Outcome::Answered { probability, .. } => {
                probability.map_or(f64::NAN, thinkthen::Probability::get)
            }
            thinkthen::results::Outcome::Unsure { probability, .. } => probability.get(),
            _ => f64::NAN,
        }
    }
    /* SAFETY: value is borrowed and outputs are exclusively writable until return. */
    unsafe {
        project(out, error, || {
            Ok(match &(*value).0 {
                Owned::Row(row) => row
                    .probability()
                    .map_or(f64::NAN, thinkthen::Probability::get),
                Owned::Entry(entry) => {
                    macro_rules! read{($($variant:ident),*)=>{match entry{$(thinkthen::results::AnnotationEntry::$variant(value)=>probability(value),)*_=>return Err(native::abi_error())}};}
                    read!(Decide, Choose, Tag, Score, Filter, Find, Recognize, Relate)
                }
            })
        })
    }
}
/// Write 1 to *out when the value carries detail, even an empty list, and 0 when it does not.
///
/// Returns THINKTHEN_OK with *out written.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_has_detail(
    value: *const ValueHandle,
    out: *mut u8,
    error: *mut HostError,
) -> i32 {
    fn present<A, D>(outcome: &thinkthen::results::Outcome<A, D>) -> u8 {
        match outcome {
            thinkthen::results::Outcome::Answered { detail, .. }
            | thinkthen::results::Outcome::Unsure { detail, .. } => u8::from(detail.is_some()),
            _ => 0,
        }
    }
    /* SAFETY: value is borrowed and output slots are exclusively writable. */
    unsafe {
        project(out, error, || {
            Ok(match &(*value).0 {
                Owned::Row(row) => {
                    macro_rules! read{($($variant:ident),*)=>{match row{$(thinkthen::results::AnyRow::$variant(row)=>present(row.outcome()),)*_=>return Err(native::abi_error())}};}
                    read!(
                        Decide, Choose, Tag, Score, Filter, Rank, Find, Recognize, Relate, Annotate
                    )
                }
                Owned::Entry(entry) => {
                    macro_rules! read{($($variant:ident),*)=>{match entry{$(thinkthen::results::AnnotationEntry::$variant(outcome)=>present(outcome),)*_=>return Err(native::abi_error())}};}
                    read!(Decide, Choose, Tag, Score, Filter, Find, Recognize, Relate)
                }
            })
        })
    }
}

/// Write the run's name to *out; out->data is NULL when the run has none.
///
/// Returns THINKTHEN_OK with *out written, or -THINKTHEN_CODE_USAGE_ABI with *out unchanged for
/// an annotation entry. *out borrows from value and lasts until thinkthen_value_free(value);
/// never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_source_name(
    value: *const ValueHandle,
    out: *mut crate::builders::TextView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the returned view borrows the live value owner. */
    unsafe {
        project(out, error, || {
            let Owned::Row(row) = &(*value).0 else {
                return Err(native::abi_error());
            };
            Ok(row
                .source()
                .name()
                .map_or_else(crate::builders::TextView::default, views::text))
        })
    }
}

/// Write the input of a filter or rank value to *out.
///
/// Returns THINKTHEN_OK with *out written, or -THINKTHEN_CODE_USAGE_ABI with *out unchanged
/// when the value has no input. *out borrows from value and lasts until
/// thinkthen_value_free(value); never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_original(
    value: *const ValueHandle,
    out: *mut crate::RecordView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the value owns the original input and output slots are exclusive. */
    unsafe {
        project(out, error, || {
            (*value)
                .1
                .as_ref()
                .map(crate::original::Original::view)
                .ok_or_else(native::abi_error)
        })
    }
}
