//! The confirmed top-level data URL image form, shared by the spike backends.
use super::EncodeError;
use crate::core::image::ImageInput;
use serde::Deserialize;
use serde_json::value::RawValue;

pub(crate) const IMAGE_DOMAIN: &[u8] = b"thinkthen.images/data-url/1\0";
const STATE_DOMAIN: &str = "thinkthen.image-state/1";

/// Pure encoding without another dependency. Three bytes make four symbols.
fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut text = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let mut held = chunk.iter().copied();
        let a = held.next().unwrap_or(0);
        let b = held.next();
        let c = held.next();
        let slots = [
            a >> 2,
            ((a & 3) << 4) | (b.unwrap_or(0) >> 4),
            ((b.unwrap_or(0) & 15) << 2) | (c.unwrap_or(0) >> 6),
            c.unwrap_or(0) & 63,
        ];
        for (place, slot) in slots.into_iter().enumerate() {
            let padding = (place == 2 && b.is_none()) || (place == 3 && c.is_none());
            text.push(if padding {
                '='
            } else {
                char::from(*ALPHABET.get(usize::from(slot)).unwrap_or(&b'='))
            });
        }
    }
    text
}

pub(crate) fn images(image: Option<&ImageInput>) -> Result<Option<String>, EncodeError> {
    image
        .map(|image| {
            let url = format!(
                "data:{};base64,{}",
                image.media.mime(),
                base64(&image.bytes)
            );
            serde_json::to_string(&[url]).map_err(|error| EncodeError::of(&error))
        })
        .transpose()
}

/// Store both wire parts under one shared-state digest. The marker is metadata.
pub(crate) fn image_state(state: &str, images: &str) -> String {
    format!("{STATE_DOMAIN}\n[{state},{images}]")
}

/// Read only the typed private shared-state envelope used by this spike.
pub(crate) fn image_parts(state: &str) -> Option<(String, String)> {
    let state = state.strip_prefix(STATE_DOMAIN)?.strip_prefix('\n')?;
    let (state, images): (Box<RawValue>, Box<RawValue>) = serde_json::from_str(state).ok()?;
    Some((state.get().to_owned(), images.get().to_owned()))
}

pub(crate) fn request_has_images(request: &[u8]) -> bool {
    #[derive(Deserialize)]
    struct Body {
        images: Option<Vec<String>>,
    }
    serde_json::from_slice::<Body>(request)
        .ok()
        .is_some_and(|body| body.images.is_some())
}

#[cfg(test)]
mod tests {
    use super::base64;
    #[test]
    fn base64_keeps_every_byte_and_pads_the_last_group() {
        for (bytes, expected) in [
            (b"".as_slice(), ""),
            (b"f", "Zg=="),
            (b"fo", "Zm8="),
            (b"foo", "Zm9v"),
            (&[0, 255, 128, 1], "AP+AAQ=="),
        ] {
            assert_eq!(base64(bytes), expected);
        }
    }
}
