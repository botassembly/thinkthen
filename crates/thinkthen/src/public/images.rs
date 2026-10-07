//! Explicit immutable inputs shared by native and foreign image doors.

mod calls;
mod defaults;
mod many;
pub use defaults::{
    choose_input, choose_input_with, decide_input, decide_input_with, details_input,
    details_input_with, score_input, score_input_with,
};

use serde::Serialize;
use std::sync::Arc;

use super::Error;
use crate::core::image::{Image, ImageState};
pub use crate::core::image::{ImageMedia, InputFunction};

/// SDK compressed-byte bound per question, independent of vendor body limits.
pub const MAX_IMAGE_BYTES: usize = crate::engine::image::MAX_IMAGE_BYTES;
/// SDK attachment-count bound. Routes may narrow it.
pub const MAX_IMAGES: usize = crate::engine::image::MAX_IMAGES;

/// An immutable original image whose media, dimensions and pixels were validated.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageInput(pub(crate) Image);

impl ImageInput {
    /// Validate compressed pixels without resizing or re-encoding.
    ///
    /// # Errors
    /// Returns Usage for malformed media, pixels or exceeded SDK bounds.
    pub fn new(media: ImageMedia, bytes: impl Into<Arc<[u8]>>) -> Result<Self, Error> {
        let bytes = bytes.into();
        let (width, height) =
            crate::engine::image::decode(media, &bytes).map_err(Error::refused)?;
        Ok(Self(Image {
            media,
            bytes,
            width,
            height,
        }))
    }

    /// Explicit validated media.
    #[must_use]
    pub const fn media(&self) -> ImageMedia {
        self.0.media
    }
    /// Original compressed bytes, without transformation.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.0.bytes
    }
    /// Original pixel width.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.0.width.get()
    }
    /// Original pixel height.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.0.height.get()
    }
}

impl Serialize for ImageInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

/// Optional ancillary text and ordered images. Duplicates remain separate entries.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ImageEvidence {
    text: Option<String>,
    images: Vec<ImageInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<super::SourceLocation>,
}

impl std::fmt::Debug for ImageEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageEvidence")
            .field("text", &"<withheld>")
            .field("images", &self.images)
            .finish()
    }
}

impl ImageEvidence {
    /// Form one indivisible comparison input.
    ///
    /// # Errors
    /// Returns Usage for no images, more than eight, or more than 24 MiB in total.
    pub fn new(text: Option<String>, images: Vec<ImageInput>) -> Result<Self, Error> {
        crate::engine::image::validate_set(
            images.iter().map(|image| image.bytes().len()),
            text.as_ref().map_or(0, String::len),
        )
        .map_err(Error::refused)?;
        Ok(Self {
            text,
            images,
            location: None,
        })
    }
    /// Authored ancillary text, if provided.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }
    /// Validated images in original order, including duplicates.
    #[must_use]
    pub fn images(&self) -> &[ImageInput] {
        &self.images
    }
    /// Physical source provenance, excluded from model evidence and identity.
    #[must_use]
    pub const fn location(&self) -> Option<&super::SourceLocation> {
        self.location.as_ref()
    }

    pub(crate) fn located(mut self, file: String) -> Self {
        self.location = Some(super::SourceLocation::image(file));
        self
    }

    pub(crate) fn one(image: ImageInput) -> Self {
        Self {
            text: None,
            images: vec![image],
            location: None,
        }
    }

    pub(crate) fn state(&self) -> ImageState {
        ImageState {
            text: crate::core::Json::String(self.text.clone().unwrap_or_default()),
            images: self.images.iter().map(|image| image.0.clone()).collect(),
        }
    }
}

/// Explicit text or image input; paths and ordinary byte arrays never imply images.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum QuestionInput {
    /// Existing text input.
    Text(String),
    /// Ordered immutable images and optional text.
    Images(ImageEvidence),
    /// Native-selected original record with separate images and provenance.
    Record(super::RecordEvidence),
}

impl std::fmt::Debug for QuestionInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(_) => f.write_str("QuestionInput::Text(<withheld>)"),
            Self::Images(images) => images.fmt(f),
            Self::Record(record) => record.fmt(f),
        }
    }
}

/// The record carrier the shared input scheduler accepts.
/// This is separate from the existing text-only Evidence trait.
pub trait InputEvidence {
    /// Snapshot the explicit immutable input without inserting location metadata.
    fn question_input(&self) -> QuestionInput;
}
impl<T: super::Evidence> InputEvidence for T {
    fn question_input(&self) -> QuestionInput {
        QuestionInput::Text(self.evidence().to_owned())
    }
}
impl InputEvidence for QuestionInput {
    fn question_input(&self) -> QuestionInput {
        self.clone()
    }
}

pub(crate) fn guard(function: InputFunction, input: &QuestionInput) -> Result<(), Error> {
    let has_images = match input {
        QuestionInput::Images(_) => true,
        QuestionInput::Record(record) => !record.images().is_empty(),
        QuestionInput::Text(_) => false,
    };
    if has_images && !function.accepts_images() {
        Err(Error::usage(format!(
            "{} accepts text only; images are unsupported",
            function.name()
        )))
    } else {
        Ok(())
    }
}
