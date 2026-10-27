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
/// Compare required and engine ABI versions using the shared acceptance rule.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_abi_compatible(required: u32, actual: u32) -> u8 {
    u8::from(native::check_version(required, actual).is_ok())
}
/// Number of codes in the authoritative engine registry.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_code_count() -> usize {
    Code::ALL.len()
}
/// Registry code at index, or zero outside the registry. Zero is not an error code.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_code_at(index: usize) -> i32 {
    Code::ALL.get(index).map_or(0, |code| code.abi())
}
/// Copy typed row values into an independent owner; every view borrows that owner.
/// # Safety
/// Row is a live sized prefix; out/error are distinct empty writable slots.
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
/// Release a typed snapshot after all of its borrowed field views are finished.
/// # Safety
/// NULL or the unique live value owner with no active readers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_free(value: *mut ValueHandle) {
    if !value.is_null() {
        /* SAFETY: the caller transfers this unique owner. */
        drop(unsafe { Box::from_raw(value) });
    }
}
/// Read a typed function discriminator without interpreting JSON.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_function(
    value: *const ValueHandle,
    out: *mut u32,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the value is borrowed and outputs are exclusive until return. */
    unsafe { project(out, error, || Ok((*value).function()?.abi())) }
}
/// Read a typed status discriminator.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_status(
    value: *const ValueHandle,
    out: *mut u32,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the value is borrowed and outputs are exclusive until return. */
    unsafe { project(out, error, || Ok((*value).status()?.abi())) }
}
/// Borrow source fields until value cleanup. Annotation entries have no source.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
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
/// Borrow an exact identifier; integer variants never use a double.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
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
/// Copy a failed-record error into a sized owner; an ordinary result gives no code.
/// # Safety
/// Value is live; result/error are sized, distinct empty writable slots.
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
            (match (*value).error()? {
                Some(error) => HostError::from_error(size, error),
                None => HostError::empty(size),
            })
            .write(result);
            Ok(OK)
        })
    }
}
/// Read an aggregate probability; NaN means the function has no defined aggregate.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
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
/// Distinguish absent detail from a present empty typed detail collection.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
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

/// Borrow the actual execution name; a null text view means unnamed execution.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
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

/// Borrow original filter bytes owned by this independent typed snapshot.
/// # Safety
/// Value is live; out/error are distinct empty writable slots; readers may run concurrently.
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
