//! Bounded edge decode. Original bytes alone enter the pure core.

use crate::core::image::ImageMedia;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub(crate) struct ImageError(&'static str);

pub(crate) const MAX_IMAGE_BYTES: usize = 24 * 1024 * 1024;
pub(crate) const MAX_IMAGES: usize = 8;
use image::{ImageDecoder as _, ImageFormat, ImageReader, Limits};
use std::io::Cursor;
use std::num::NonZeroU32;

const SIDE: u32 = 8192;
const PIXELS: u64 = 16_777_216;
const ALLOCATION: u64 = 128 * 1024 * 1024;

pub(crate) fn decode(
    media: ImageMedia,
    bytes: &[u8],
) -> Result<(NonZeroU32, NonZeroU32), ImageError> {
    if bytes.len() > MAX_IMAGE_BYTES {
        return Err(ImageError(
            "image exceeds the 25165824 compressed byte SDK limit",
        ));
    }
    let format = match media {
        ImageMedia::Jpeg => ImageFormat::Jpeg,
        ImageMedia::Png => ImageFormat::Png,
    };
    if image::guess_format(bytes).ok() != Some(format) {
        return Err(malformed());
    }
    match media {
        ImageMedia::Jpeg => jpeg(bytes),
        ImageMedia::Png => png(bytes),
    }
}

fn limits() -> Limits {
    let mut limits = Limits::default();
    limits.max_image_width = Some(SIDE);
    limits.max_image_height = Some(SIDE);
    limits.max_alloc = Some(ALLOCATION);
    limits
}

fn dimensions(width: u32, height: u32) -> Result<(NonZeroU32, NonZeroU32), ImageError> {
    if width > SIDE || height > SIDE || u64::from(width) * u64::from(height) > PIXELS {
        return Err(ImageError(
            "image exceeds SDK dimensions: side 8192, pixels 16777216",
        ));
    }
    Ok((
        NonZeroU32::new(width).ok_or_else(malformed)?,
        NonZeroU32::new(height).ok_or_else(malformed)?,
    ))
}

fn png(bytes: &[u8]) -> Result<(NonZeroU32, NonZeroU32), ImageError> {
    png_container(bytes)?;
    let mut reader = ImageReader::with_format(Cursor::new(bytes), ImageFormat::Png);
    reader.limits(limits());
    let mut decoder = reader.into_decoder().map_err(|_| malformed())?;
    let (width, height) = decoder.dimensions();
    let dimensions = dimensions(width, height)?;
    decoder.set_limits(limits()).map_err(|_| malformed())?;
    let size = usize::try_from(decoder.total_bytes()).map_err(|_| malformed())?;
    if u64::try_from(size).is_ok_and(|size| size > ALLOCATION) {
        return Err(malformed());
    }
    let mut pixels = Vec::new();
    pixels
        .try_reserve_exact(size)
        .map_err(|_| ImageError("image decode allocation refused"))?;
    pixels.resize(size, 0);
    decoder.read_image(&mut pixels).map_err(|_| malformed())?;
    Ok(dimensions)
}

fn jpeg(bytes: &[u8]) -> Result<(NonZeroU32, NonZeroU32), ImageError> {
    if !bytes.ends_with(&[0xff, 0xd9]) {
        return Err(malformed());
    }
    // The selected decoder avoids the malformed progressive component panic
    // in zune 0.5.15. Default parallel features are disabled. Its documented
    // recovery can zero-fill missing entropy; validation means decoded pixels,
    // not certification of a strictly encoded JPEG bitstream.
    let mut decoder = jpeg_decoder::Decoder::new(Cursor::new(bytes));
    decoder.set_max_decoding_buffer_size(ALLOCATION as usize);
    decoder.read_info().map_err(|_| malformed())?;
    let info = decoder.info().ok_or_else(malformed)?;
    let dimensions = dimensions(u32::from(info.width), u32::from(info.height))?;
    let _pixels = decoder.decode().map_err(|_| malformed())?;
    Ok(dimensions)
}

fn malformed() -> ImageError {
    ImageError("image media or compressed pixels are invalid or exceed decoder limits")
}

// The pixel decoder reads one frame; require a complete single-frame container
// rather than accepting an animation or a missing final chunk as a still image.
fn png_container(bytes: &[u8]) -> Result<(), ImageError> {
    let mut offset = 8usize;
    let mut ended = false;
    while let Some(header) = bytes.get(offset..offset.saturating_add(8)) {
        let length: [u8; 4] = header
            .get(..4)
            .ok_or_else(malformed)?
            .try_into()
            .map_err(|_| malformed())?;
        let length = usize::try_from(u32::from_be_bytes(length)).map_err(|_| malformed())?;
        let kind = header.get(4..8).ok_or_else(malformed)?;
        offset = offset
            .checked_add(length)
            .and_then(|offset| offset.checked_add(12))
            .ok_or_else(malformed)?;
        if offset > bytes.len() || kind == b"acTL" {
            return Err(malformed());
        }
        if kind == b"IEND" {
            ended =
                length == 0 && offset == bytes.len() && bytes.ends_with(&[0xae, 0x42, 0x60, 0x82]);
            break;
        }
    }
    if ended { Ok(()) } else { Err(malformed()) }
}

/// Shared original-input bounds, including replay's restored ordered set.
pub(crate) fn validate_set(
    lengths: impl Iterator<Item = usize>,
    text_bytes: usize,
) -> Result<(), ImageError> {
    let mut count = 0usize;
    let mut total = 0usize;
    for length in lengths {
        count += 1;
        total = total
            .checked_add(length)
            .filter(|total| *total <= MAX_IMAGE_BYTES)
            .ok_or(ImageError(
                "image evidence exceeds the 25165824 compressed byte SDK limit",
            ))?;
    }
    if count == 0 || count > MAX_IMAGES {
        return Err(ImageError("image evidence requires 1 to 8 images"));
    }
    if text_bytes > crate::core::MAX_RECORD_BYTES {
        return Err(ImageError("image text exceeds the 16 MiB SDK limit"));
    }
    Ok(())
}
