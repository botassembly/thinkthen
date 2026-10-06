//! Measured 0036 setups, selected only by a closed operator declaration.
//! Pins live in specification/fixtures/images/local/setups.json and backends.md.
//! No runtime attestation or local image-token calibration is implied.

use super::{ImageLimit, ImageWire, check};
use crate::core::image::ImageState;

/// Exact supported setup identifiers; aliases never select a setup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProfileId {
    Clef,
    ClefFlash,
    Imajev,
}

impl ProfileId {
    pub(crate) fn parse(id: &str) -> Option<Self> {
        match id {
            "clef-llamacpp-v0.6.0-0036" => Some(Self::Clef),
            "clef-flash-llamacpp-v0.6.0-0036" => Some(Self::ClefFlash),
            "imajev-mlx-0036" => Some(Self::Imajev),
            _ => None,
        }
    }
}

/// A tested optional SDK envelope, not vendor maxima or exact context admission.
pub(crate) fn admit(_id: ProfileId, input: &ImageState) -> Result<ImageWire, ImageLimit> {
    check(
        0,
        "image count",
        2,
        u64::try_from(input.images.len()).map_err(|_| ImageLimit::Overflow)?,
    )?;
    for (at, image) in input.images.iter().enumerate() {
        check(
            at,
            "compressed bytes",
            1_048_576,
            u64::try_from(image.bytes.len()).map_err(|_| ImageLimit::Overflow)?,
        )?;
        check(
            at,
            "longest edge",
            1024,
            u64::from(image.width.get().max(image.height.get())),
        )?;
    }
    let urls: Vec<String> = input.images.iter().map(|image| image.data_url()).collect();
    Ok(ImageWire {
        state: serde_json::to_string(&input.text).map_err(|_| ImageLimit::Overflow)?,
        images: Some(serde_json::to_string(&urls).map_err(|_| ImageLimit::Overflow)?),
        body_limit: 2_800_000,
        questions_limit: None,
        // A pixel/tile guess would falsely present a measured local tokenizer.
        // Local estimates stay unavailable until a separately admitted method.
        tokens_per_question: 0,
        image_tokens_known: false,
    })
}
