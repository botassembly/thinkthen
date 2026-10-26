//! Typed probability and span detail for every complex answer shape.
#![allow(
    unsafe_code,
    reason = "detail exports borrow live value handles and exclusive output slots"
)]
use super::{ProbabilityView, RelationView, SpanView, ValueHandle, project, views};
use crate::HostError;
use std::collections::BTreeMap;
use thinkthen::{
    Error, FunctionName, Probability, native, questions,
    results::{KindDetail, PieceDetail},
};
fn labels(value: &ValueHandle) -> Result<Option<&BTreeMap<String, Probability>>, Error> {
    Ok(match value.function()? {
        FunctionName::Choose => value
            .detail::<questions::Choose>()?
            .map(|detail| detail.labels()),
        FunctionName::Tag => value
            .detail::<questions::Tag>()?
            .map(|detail| detail.labels()),
        _ => return Err(native::abi_error()),
    })
}
fn levels(value: &ValueHandle) -> Result<Option<&Vec<Probability>>, Error> {
    Ok(match value.function()? {
        FunctionName::Score => value
            .detail::<questions::Score>()?
            .map(|detail| detail.levels()),
        FunctionName::Rank => value
            .detail::<questions::Rank>()?
            .map(|detail| detail.levels()),
        _ => return Err(native::abi_error()),
    })
}
fn piece(value: &ValueHandle, index: usize) -> Result<&PieceDetail, Error> {
    value
        .detail::<questions::Recognize>()?
        .and_then(|detail| detail.pieces().get(index))
        .ok_or_else(native::abi_error)
}
fn kind(value: &ValueHandle, index: usize) -> Result<&KindDetail, Error> {
    value
        .detail::<questions::Recognize>()?
        .and_then(|detail| detail.names().get(index))
        .ok_or_else(native::abi_error)
}
fn probability(
    values: &BTreeMap<String, Probability>,
    index: usize,
) -> Result<ProbabilityView, Error> {
    let (label, probability) = values.iter().nth(index).ok_or_else(native::abi_error)?;
    Ok(ProbabilityView {
        label: views::text(label),
        probability: probability.get(),
    })
}
/// Count optional choose/tag label probabilities.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_label_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value is borrowed and outputs are exclusive until return. */
    unsafe { project(out, error, || Ok(labels(&*value)?.map_or(0, BTreeMap::len))) }
}
/// Borrow one choose/tag label probability, including unselected labels.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_label_at(
    value: *const ValueHandle,
    index: usize,
    out: *mut ProbabilityView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: label text borrows value and output slots are exclusive. */
    unsafe {
        project(out, error, || {
            probability(labels(&*value)?.ok_or_else(native::abi_error)?, index)
        })
    }
}
/// Count optional score/rank level probabilities in question order.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_level_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value is borrowed and output slots are exclusive. */
    unsafe { project(out, error, || Ok(levels(&*value)?.map_or(0, Vec::len))) }
}
/// Read one score/rank level probability in question order.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_level_at(
    value: *const ValueHandle,
    index: usize,
    out: *mut f64,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        project(out, error, || {
            Ok(levels(&*value)?
                .and_then(|levels| levels.get(index))
                .ok_or_else(native::abi_error)?
                .get())
        })
    }
}
/// Count recognition's original pieces, or zero when detail is absent.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_piece_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        project(out, error, || {
            Ok((*value)
                .detail::<questions::Recognize>()?
                .map_or(0, |detail| detail.pieces().len()))
        })
    }
}
/// Borrow an original piece and its translated span.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_piece_at(
    value: *const ValueHandle,
    index: usize,
    out: *mut SpanView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: returned text borrows value until cleanup. */
    unsafe {
        project(out, error, || {
            let piece = piece(&*value, index)?;
            Ok(SpanView {
                present: 1,
                text: views::text(piece.text()),
                start: *piece.start() as u64,
                end: *piece.end() as u64,
            })
        })
    }
}
/// Count one piece's typed boundary probabilities.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_piece_tag_count(
    value: *const ValueHandle,
    index: usize,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe { project(out, error, || Ok(piece(&*value, index)?.tags().len())) }
}
/// Borrow one original piece's boundary label and probability.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_piece_tag_at(
    value: *const ValueHandle,
    index: usize,
    tag: usize,
    out: *mut ProbabilityView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: returned text borrows value and output slots are exclusive. */
    unsafe {
        project(out, error, || {
            probability(piece(&*value, index)?.tags(), tag)
        })
    }
}
/// Count proposed-name detail entries for recognition.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_kind_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        project(out, error, || {
            Ok((*value)
                .detail::<questions::Recognize>()?
                .map_or(0, |detail| detail.names().len()))
        })
    }
}
/// Read one proposed name's translated span; this detail has no text field.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_kind_span(
    value: *const ValueHandle,
    index: usize,
    out: *mut SpanView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        project(out, error, || {
            let kind = kind(&*value, index)?;
            Ok(SpanView {
                present: 1,
                start: *kind.start() as u64,
                end: *kind.end() as u64,
                ..SpanView::default()
            })
        })
    }
}
/// Count one proposed name's caller-domain kind probabilities.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_kind_probability_count(
    value: *const ValueHandle,
    index: usize,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe { project(out, error, || Ok(kind(&*value, index)?.kinds().len())) }
}
/// Borrow one proposed name's kind label and probability.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_kind_probability_at(
    value: *const ValueHandle,
    index: usize,
    choice: usize,
    out: *mut ProbabilityView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: kind text borrows value and output slots are exclusive. */
    unsafe {
        project(out, error, || {
            probability(kind(&*value, index)?.kinds(), choice)
        })
    }
}
/// Count evaluated relation candidate pairs, including pairs outside the answer.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_pair_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        project(out, error, || {
            Ok((*value)
                .detail::<questions::Relate>()?
                .map_or(0, |detail| detail.pairs().len()))
        })
    }
}
/// Borrow one evaluated candidate pair using the same typed relation shape.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_pair_at(
    value: *const ValueHandle,
    index: usize,
    out: *mut RelationView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: all returned text borrows value until cleanup. */
    unsafe {
        project(out, error, || {
            Ok(views::relation(
                (*value)
                    .detail::<questions::Relate>()?
                    .and_then(|detail| detail.pairs().get(index))
                    .ok_or_else(native::abi_error)?,
            ))
        })
    }
}
