//! Typed probability and span detail for every complex answer shape.
#![allow(
    unsafe_code,
    reason = "detail exports borrow live value handles and exclusive output slots"
)]
use super::{ProbabilityView, RelationView, SpanView, ValueHandle, project, views};
use crate::{HostError, memory::live};
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
        _ => return Err(native::argument_error(thinkthen::Code::UsageFunction)),
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
        _ => return Err(native::argument_error(thinkthen::Code::UsageFunction)),
    })
}
fn piece(value: &ValueHandle, index: usize) -> Result<&PieceDetail, Error> {
    value
        .detail::<questions::Recognize>()?
        .and_then(|detail| detail.pieces().get(index))
        .ok_or_else(|| native::argument_error(thinkthen::Code::UsageArgument))
}
fn kind(value: &ValueHandle, index: usize) -> Result<&KindDetail, Error> {
    value
        .detail::<questions::Recognize>()?
        .and_then(|detail| detail.names().get(index))
        .ok_or_else(|| native::argument_error(thinkthen::Code::UsageArgument))
}
fn probability(
    values: &BTreeMap<String, Probability>,
    index: usize,
) -> Result<ProbabilityView, Error> {
    let (label, probability) = values
        .iter()
        .nth(index)
        .ok_or_else(|| native::argument_error(thinkthen::Code::UsageArgument))?;
    Ok(ProbabilityView {
        label: views::text(label),
        probability: probability.get(),
    })
}
/// Write how many label probabilities a choose or tag value's detail holds to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
/// -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's. A value without detail
/// writes 0.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_label_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value is borrowed and outputs are exclusive until return. */
    unsafe {
        project(out, error, || {
            Ok(labels(live(value)?)?.map_or(0, BTreeMap::len))
        })
    }
}
/// Write the label probability at index, chosen or not, to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
/// -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
/// -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
/// *out borrows from value and lasts until thinkthen_value_free(value); never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
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
            probability(
                labels(live(value)?)?
                    .ok_or_else(|| native::argument_error(thinkthen::Code::UsageArgument))?,
                index,
            )
        })
    }
}
/// Write how many level probabilities a score or rank value's detail holds to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
/// -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's. A value without detail
/// writes 0.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_level_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value is borrowed and output slots are exclusive. */
    unsafe { project(out, error, || Ok(levels(live(value)?)?.map_or(0, Vec::len))) }
}
/// Write the probability of the level at index, in question order, to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
/// -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
/// -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
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
            Ok(levels(live(value)?)?
                .and_then(|levels| levels.get(index))
                .ok_or_else(|| native::argument_error(thinkthen::Code::UsageArgument))?
                .get())
        })
    }
}
/// Write how many pieces a recognize value's detail split the text into to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
/// -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's. A value without detail
/// writes 0.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_piece_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        project(out, error, || {
            Ok(live(value)?
                .detail::<questions::Recognize>()?
                .map_or(0, |detail| detail.pieces().len()))
        })
    }
}
/// Write the piece at index, with its span, to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
/// -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
/// -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
/// *out borrows from value and lasts until thinkthen_value_free(value); never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
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
            let piece = piece(live(value)?, index)?;
            Ok(SpanView {
                present: 1,
                text: views::text(piece.text()),
                start: piece.start() as u64,
                end: piece.end() as u64,
            })
        })
    }
}
/// Write how many boundary probabilities the piece at index holds to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
/// -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
/// -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_piece_tag_count(
    value: *const ValueHandle,
    index: usize,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe { project(out, error, || Ok(piece(live(value)?, index)?.tags().len())) }
}
/// Write boundary probability tag of the piece at index to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
/// -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
/// -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
/// *out borrows from value and lasts until thinkthen_value_free(value); never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
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
            probability(piece(live(value)?, index)?.tags(), tag)
        })
    }
}
/// Write how many proposed names a recognize value's detail holds to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
/// -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's. A value without detail
/// writes 0.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_kind_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        project(out, error, || {
            Ok(live(value)?
                .detail::<questions::Recognize>()?
                .map_or(0, |detail| detail.names().len()))
        })
    }
}
/// Write the text and span of the proposed name at index to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
/// -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
/// -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
/// *out borrows from value and lasts until thinkthen_value_free(value); never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
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
            let kind = kind(live(value)?, index)?;
            Ok(SpanView {
                present: 1,
                text: views::text(kind.text()),
                start: kind.start() as u64,
                end: kind.end() as u64,
            })
        })
    }
}
/// Write how many kind probabilities the proposed name at index holds to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
/// -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
/// -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_kind_probability_count(
    value: *const ValueHandle,
    index: usize,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe { project(out, error, || Ok(kind(live(value)?, index)?.kinds().len())) }
}
/// Write kind probability choice of the proposed name at index to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
/// -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
/// -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
/// *out borrows from value and lasts until thinkthen_value_free(value); never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
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
            probability(kind(live(value)?, index)?.kinds(), choice)
        })
    }
}
/// Write how many name pairs a relate value's detail judged, related or not, to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
/// -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's. A value without detail
/// writes 0.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_pair_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        project(out, error, || {
            Ok(live(value)?
                .detail::<questions::Relate>()?
                .map_or(0, |detail| detail.pairs().len()))
        })
    }
}
/// Write the judged pair at index to *out.
///
/// Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
/// -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
/// -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
/// *out borrows from value and lasts until thinkthen_value_free(value); never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
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
                live(value)?
                    .detail::<questions::Relate>()?
                    .and_then(|detail| detail.pairs().get(index))
                    .ok_or_else(|| native::argument_error(thinkthen::Code::UsageArgument))?,
            ))
        })
    }
}
