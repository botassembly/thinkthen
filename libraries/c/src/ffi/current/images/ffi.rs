//! Image pointer operations stay at the C boundary; native code validates pixels.
#![allow(
    unsafe_code,
    reason = "counted image bytes and live image handles follow the C lifetime contract"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use super::{
    carriers::{ImageViewV1, OptionalStringV1},
    read,
};
use crate::ffi::values as abi;
use crate::{
    Door,
    current::ImageHandle,
    failures::{Failure, OK, USAGE, guard},
};

/// Validate and clone compressed image bytes and an optional UTF-8 filename.
/// # Safety
/// Counted storage is readable and output writable until return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_image_clone(
    engine: *const Door,
    bytes: *const u8,
    len: usize,
    media: u32,
    filename: OptionalStringV1,
    out: *mut *mut ImageHandle,
) -> std::ffi::c_int {
    // SAFETY: caller supplies live handles and the declared counted extents.
    unsafe {
        crate::ffi::typed(
            engine,
            |_| {
                read::required(out)?;
                let media = match media {
                    abi::THINKTHEN_IMAGE_JPEG_V1 => thinkthen::ImageMedia::Jpeg,
                    abi::THINKTHEN_IMAGE_PNG_V1 => thinkthen::ImageMedia::Png,
                    _ => return Err(Failure::usage("invalid image media")),
                };
                thinkthen::ImageInput::admit_length(len)?;
                let filename = read::optional_string(filename)?;
                let native = thinkthen::ImageInput::new(media, read::slice(bytes, len)?)?;
                Ok(ImageHandle { native, filename })
            },
            |_, image| {
                *out = Box::into_raw(Box::new(image));
                OK
            },
        )
    }
}
/// Borrow original bytes, dimensions and filename from an immutable image owner.
/// # Safety
/// Owner is live throughout the view's use; output is writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_image_view(
    owner: *const ImageHandle,
    out: *mut ImageViewV1,
) -> std::ffi::c_int {
    guard(None, USAGE, || {
        // SAFETY: only null or live handles are allowed by the header.
        let Some(owner) = (unsafe { owner.as_ref() }) else {
            return USAGE;
        };
        if out.is_null() {
            return USAGE;
        }
        // SAFETY: checked nonnull output has storage for the full descriptor.
        unsafe {
            *out = owner.view();
        }
        OK
    })
}
/// Free an owned image; NULL is harmless.
/// # Safety
/// A nonnull image is live, freed once, after all borrowers finish.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_image_free(owner: *mut ImageHandle) {
    guard(None, (), || {
        if !owner.is_null() {
            // SAFETY: only this library frees its allocated handle, once.
            drop(unsafe { Box::from_raw(owner) });
        }
    });
}
