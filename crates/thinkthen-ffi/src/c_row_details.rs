//! Safe typed detail lists borrow the row's owned caches, including in failed states.
#![allow(unsafe_code, reason = "C lends live rows and writable output slots")]
use crate::{HostError, HostRow, row, values};
use thinkthen::{Code, native};
/// Borrow detail values. Missing detail writes a count of zero in every state.
/// # Safety
/// row is live; out and count are writable; error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_detail_labels(
    row: *const HostRow,
    out: *mut *const values::ProbabilityView,
    count: *mut usize,
    error: *mut HostError,
) -> i32 {
    // SAFETY: project_list keeps the payload borrowed until its views are written.
    unsafe {
        crate::c_rows::project_list(row, out, count, error, |d: &row::RowData| {
            if !matches!(
                d.value.function()?,
                thinkthen::FunctionName::Choose | thinkthen::FunctionName::Tag
            ) {
                return Err(native::argument_error(Code::UsageFunction));
            }
            Ok(&d.labels)
        })
    }
}
/// Borrow detail values. Missing detail writes a count of zero in every state.
/// # Safety
/// row is live; out and count are writable; error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_detail_levels(
    row: *const HostRow,
    out: *mut *const f64,
    count: *mut usize,
    error: *mut HostError,
) -> i32 {
    // SAFETY: project_list keeps the payload borrowed until its views are written.
    unsafe {
        crate::c_rows::project_list(row, out, count, error, |d: &row::RowData| {
            if !matches!(
                d.value.function()?,
                thinkthen::FunctionName::Score | thinkthen::FunctionName::Rank
            ) {
                return Err(native::argument_error(Code::UsageFunction));
            }
            Ok(&d.levels)
        })
    }
}
/// Borrow complete recognition detail. Missing detail has both counts zero in every state.
/// # Safety
/// row is live; out is writable; error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_detail_pieces(
    row: *const HostRow,
    out: *mut crate::RecognitionDetailView,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the output's two arrays and nested probabilities borrow this row owner.
    unsafe {
        crate::values::project(out, error, || {
            let data = row::data(row)?;
            data.value.detail::<thinkthen::questions::Recognize>()?;
            Ok(crate::RecognitionDetailView {
                pieces: data.pieces.as_ptr(),
                piece_count: data.pieces.len(),
                names: data.kinds.as_ptr(),
                name_count: data.kinds.len(),
            })
        })
    }
}
/// Borrow detail values. Missing detail writes a count of zero in every state.
/// # Safety
/// row is live; out and count are writable; error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_detail_pairs(
    row: *const HostRow,
    out: *mut *const values::RelationView,
    count: *mut usize,
    error: *mut HostError,
) -> i32 {
    // SAFETY: project_list keeps the payload borrowed until its views are written.
    unsafe {
        crate::c_rows::project_list(row, out, count, error, |d: &row::RowData| {
            if !matches!(d.value.function()?, thinkthen::FunctionName::Relate) {
                return Err(native::argument_error(Code::UsageFunction));
            }
            Ok(&d.pairs)
        })
    }
}
