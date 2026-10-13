//! Checked extents before creating slices from counted C storage.
#![allow(
    unsafe_code,
    reason = "counted buffers are read only under the reviewed C contract"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use crate::failures::Failure;
use crate::ffi::carriers::StringV1;
pub(crate) fn required<T>(ptr: *const T) -> Result<(), Failure> {
    if ptr.is_null() {
        Err(Failure::usage("a null required pointer"))
    } else {
        Ok(())
    }
}
pub(crate) unsafe fn slice<'a, T>(ptr: *const T, len: usize) -> Result<&'a [T], Failure> {
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
pub(crate) unsafe fn string<'a>(value: StringV1) -> Result<&'a str, Failure> {
    // SAFETY: the host promises exactly len readable bytes.
    let bytes = unsafe { slice(value.data.cast::<u8>(), value.len) }?;
    std::str::from_utf8(bytes).map_err(|_| Failure::usage("counted text is not UTF-8"))
}
