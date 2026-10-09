//! Explicit finite descriptors reuse native composition and authorized readers.
use super::admission::Source;
use crate::ImageMedia;
use serde::Deserialize;
use serde_json::value::RawValue;
use std::path::PathBuf;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Descriptor {
    pub(super) text: Option<String>,
    #[serde(default, deserialize_with = "raw")]
    pub(super) json: Option<Box<RawValue>>,
    pub(super) source: Option<Source>,
    #[serde(default)]
    pub(super) images: Vec<Attachment>,
    #[serde(default, deserialize_with = "raw")]
    pub(super) context: Option<Box<RawValue>>,
    #[serde(default, deserialize_with = "raw")]
    pub(super) options: Option<Box<RawValue>>,
}
#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum Attachment {
    Path(PathBuf),
    Declared(DeclaredImage),
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DeclaredImage {
    path: PathBuf,
    media: ImageMedia,
}
impl Attachment {
    pub(super) fn native(&self) -> crate::RequestImage {
        match self {
            Self::Path(path) => crate::RequestImage::File {
                path: path.clone(),
                media: None,
            },
            Self::Declared(image) => crate::RequestImage::File {
                path: image.path.clone(),
                media: Some(image.media),
            },
        }
    }
}
fn raw<'de, D: serde::Deserializer<'de>>(de: D) -> Result<Option<Box<RawValue>>, D::Error> {
    Box::<RawValue>::deserialize(de).map(Some)
}
