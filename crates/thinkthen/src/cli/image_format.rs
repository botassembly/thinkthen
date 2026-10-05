//! Validate image signatures and bounded container structure, without decoding pixels.
use crate::core::image::ImageMedia;

pub(super) fn media(bytes: &[u8]) -> Option<ImageMedia> {
    if png(bytes) {
        Some(ImageMedia::Png)
    } else if jpeg(bytes) {
        Some(ImageMedia::Jpeg)
    } else {
        None
    }
}

fn number(bytes: &[u8]) -> Option<usize> {
    let held: [u8; 4] = bytes.try_into().ok()?;
    usize::try_from(u32::from_be_bytes(held)).ok()
}

fn png(bytes: &[u8]) -> bool {
    let Some(mut rest) = bytes.strip_prefix(b"\x89PNG\r\n\x1a\n") else {
        return false;
    };
    let mut header = false;
    let mut pixels = false;
    while let Some(length) = rest.get(..4).and_then(number) {
        let Some((chunk, after)) = length
            .checked_add(12)
            .and_then(|size| rest.split_at_checked(size))
        else {
            return false;
        };
        let kind = chunk.get(4..8);
        if !header {
            if kind != Some(b"IHDR") || length != 13 {
                return false;
            }
            if chunk.get(8..12).and_then(number).unwrap_or(0) == 0
                || chunk.get(12..16).and_then(number).unwrap_or(0) == 0
            {
                return false;
            }
            header = true;
        } else if kind == Some(b"IHDR") {
            return false;
        }
        if kind == Some(b"IDAT") {
            pixels |= length > 0;
        }
        if kind == Some(b"IEND") {
            return header && pixels && length == 0 && after.is_empty();
        }
        rest = after;
    }
    false
}

fn jpeg(bytes: &[u8]) -> bool {
    let Some(mut rest) = bytes.strip_prefix(b"\xff\xd8") else {
        return false;
    };
    let mut frame = false;
    loop {
        let Some(after) = rest.strip_prefix(b"\xff") else {
            return false;
        };
        let place = after.iter().position(|byte| *byte != 0xff);
        let Some(place) = place else { return false };
        let Some(marker) = after.get(place).copied() else {
            return false;
        };
        let Some(remaining) = after.get(place + 1..) else {
            return false;
        };
        rest = remaining;
        let Some([high, low]) = rest.get(..2) else {
            return false;
        };
        let length = usize::from(u16::from_be_bytes([*high, *low]));
        if length < 2 {
            return false;
        }
        let Some((segment, after)) = rest.split_at_checked(length) else {
            return false;
        };
        if matches!(marker, 0xc0..=0xc3 | 0xc5..=0xc7 | 0xc9..=0xcb | 0xcd..=0xcf) {
            if length < 8
                || segment.get(3..5) == Some(&[0, 0])
                || segment.get(5..7) == Some(&[0, 0])
            {
                return false;
            }
            frame = true;
        }
        if marker == 0xda {
            return frame && length >= 6 && after.len() > 2 && after.ends_with(b"\xff\xd9");
        }
        if matches!(marker, 0xd8 | 0xd9 | 0x00) {
            return false;
        }
        rest = after;
    }
}
