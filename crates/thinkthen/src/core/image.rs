//! Validated immutable image constituents. Decoding belongs to the outer edge.

use std::fmt;
use std::num::NonZeroU32;
use std::sync::Arc;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use serde::{Serialize, Serializer};

/// Initially validated media; extensions never select a media type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, Serialize)]
pub enum ImageMedia {
    /// JPEG still image.
    #[serde(rename = "image/jpeg")]
    Jpeg,
    /// PNG still image.
    #[serde(rename = "image/png")]
    Png,
}

impl ImageMedia {
    /// Canonical MIME spelling.
    #[must_use]
    pub const fn mime(self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
        }
    }
}

/// Bytes and dimensions whose complete pixel decode succeeded at the edge.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Image {
    pub(crate) media: ImageMedia,
    pub(crate) bytes: Arc<[u8]>,
    pub(crate) width: NonZeroU32,
    pub(crate) height: NonZeroU32,
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Image")
            .field("media", &self.media)
            .field("bytes", &"<withheld>")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl Image {
    pub(crate) fn base64(&self) -> String {
        STANDARD.encode(&self.bytes)
    }

    pub(crate) fn data_url(&self) -> String {
        format!("data:{};base64,{}", self.media.mime(), self.base64())
    }
}

/// Rebuildable identity constituents, never a vendor request state.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct ImageState {
    pub(crate) text: crate::core::Json,
    pub(crate) images: Arc<[Image]>,
}

impl fmt::Debug for ImageState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageState")
            .field("text", &"<withheld>")
            .field("images", &self.images)
            .finish()
    }
}

impl Serialize for ImageState {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Encoded {
            media: ImageMedia,
            base64: String,
        }
        #[derive(Serialize)]
        struct Identity<'a> {
            schema: &'static str,
            text: &'a crate::core::Json,
            images: Vec<Encoded>,
        }
        Identity {
            schema: "thinkthen.image-state/1",
            text: &self.text,
            images: self
                .images
                .iter()
                .map(|image| Encoded {
                    media: image.media,
                    base64: image.base64(),
                })
                .collect(),
        }
        .serialize(serializer)
    }
}

/// The operation selected at an explicit typed input door.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputFunction {
    /// Yes/no judgment.
    Decide,
    /// One option.
    Choose,
    /// Rubric score.
    Score,
    /// Text labels.
    Tag,
    /// Text record gate.
    Filter,
    /// Text ordering.
    Rank,
    /// Text question set.
    Annotate,
    /// Text unit selection.
    Find,
    /// Text entities.
    Recognize,
    /// Text relations.
    Relate,
}
impl InputFunction {
    /// Stable command spelling, safe in diagnostics.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Decide => "decide",
            Self::Choose => "choose",
            Self::Score => "score",
            Self::Tag => "tag",
            Self::Filter => "filter",
            Self::Rank => "rank",
            Self::Annotate => "annotate",
            Self::Find => "find",
            Self::Recognize => "recognize",
            Self::Relate => "relate",
        }
    }
    pub(crate) const fn accepts_images(self) -> bool {
        matches!(self, Self::Decide | Self::Choose | Self::Score)
    }
}

/// Plan validated image constituents through the existing packer.
pub(crate) fn plan(
    asked: (crate::core::ModelName, crate::core::Descriptions),
    mut state: ImageState,
    context: Option<&crate::core::Evidence>,
    questions: Vec<crate::core::Question>,
    profile: Option<&crate::core::BackendProfile>,
    route: crate::core::adapters::built_in::images::ImageRoute,
) -> Result<crate::core::Plan, crate::core::BatchError> {
    use crate::core::{BatchError, Evidence, Json, Plan};
    if let Some(context) = context {
        state.text = Json::Object(vec![
            ("context".to_owned(), context.as_json().clone()),
            ("text".to_owned(), state.text),
        ]);
    }
    let record = match &state.text {
        Json::String(text) => Evidence::image_text(text.clone()),
        other => Evidence::structured(other.clone())
            .map_err(|_| BatchError::Defect("image context could not be represented"))?,
    };
    if let Some(profile) = profile {
        let text = record
            .as_text()
            .map_err(|_| BatchError::Defect("image text could not be represented"))?;
        profile.check_record(&text).map_err(BatchError::Profile)?;
    }
    Plan::new(record, asked.0, asked.1, questions)
        .map(|plan| {
            plan.with_images(state, route)
                .with_image_profile(profile.cloned())
        })
        .map_err(|_| BatchError::Defect("an image plan has no question"))
}
