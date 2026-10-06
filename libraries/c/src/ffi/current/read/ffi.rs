//! Checked extents before creating slices from counted C storage.
#![allow(
    unsafe_code,
    reason = "counted buffers are read only under the reviewed C contract"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use super::carriers::{ContentV1, OptionalContentV1, OptionalStringV1, StringV1, StringsV1};
use crate::current::Content;
use crate::failures::Failure;
pub(super) fn required<T>(ptr: *const T) -> Result<(), Failure> {
    if ptr.is_null() {
        Err(Failure::usage("a null required pointer"))
    } else {
        Ok(())
    }
}
pub(super) unsafe fn reference<'a, T>(ptr: *const T) -> Result<&'a T, Failure> {
    // SAFETY: contract excludes forged, stale and concurrently freed handles.
    unsafe { ptr.as_ref() }.ok_or_else(|| Failure::usage("a null required pointer"))
}
pub(super) unsafe fn slice<'a, T>(ptr: *const T, len: usize) -> Result<&'a [T], Failure> {
    if len == 0 {
        return Ok(&[]);
    }
    required(ptr)?;
    if len
        .checked_mul(std::mem::size_of::<T>())
        .is_none_or(|n| n > isize::MAX as usize)
    {
        return Err(Failure::usage("counted storage exceeds addressable length"));
    }
    // SAFETY: checked nonnull pointer and addressable extent; caller supplies readable storage.
    Ok(unsafe { std::slice::from_raw_parts(ptr, len) })
}
pub(super) unsafe fn string<'a>(value: StringV1) -> Result<&'a str, Failure> {
    // SAFETY: the host promises exactly len readable bytes.
    let bytes = unsafe { slice(value.data.cast::<u8>(), value.len) }?;
    std::str::from_utf8(bytes).map_err(|_| Failure::usage("counted text is not UTF-8"))
}
pub(super) fn flag(value: i32) -> Result<bool, Failure> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Failure::usage("flags must be 0 or 1")),
    }
}
pub(super) unsafe fn optional_string(value: OptionalStringV1) -> Result<Option<String>, Failure> {
    if !flag(value.present)? {
        return Ok(None);
    }
    // SAFETY: active payload has the declared counted extent.
    Ok(Some(unsafe { string(value.value) }?.to_owned()))
}
pub(super) unsafe fn content(value: ContentV1) -> Result<Content, Failure> {
    // SAFETY: active counted content is readable through return.
    let text = unsafe { string(value.data) }?;
    match value.kind {
        1 => Ok(Content::Text(text.to_owned())),
        2 => {
            Ok(Content::Json(serde_json::from_str(text).map_err(|_| {
                Failure::usage("content is not one JSON value")
            })?))
        }
        _ => Err(Failure::usage("invalid content kind")),
    }
}
pub(super) unsafe fn optional_content(
    value: OptionalContentV1,
) -> Result<Option<Content>, Failure> {
    if !flag(value.present)? {
        return Ok(None);
    }
    // SAFETY: active payload follows content's storage contract.
    unsafe { content(value.value) }.map(Some)
}
pub(super) unsafe fn strings(value: StringsV1) -> Result<Vec<String>, Failure> {
    // SAFETY: the counted array and every entry are readable.
    unsafe { slice(value.data, value.len) }?
        .iter()
        .map(|s| {
            // SAFETY: each entry follows its declared counted extent.
            Ok(unsafe { string(*s) }?.to_owned())
        })
        .collect()
}
