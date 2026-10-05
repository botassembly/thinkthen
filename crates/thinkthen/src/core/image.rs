//! One bounded image, with media verified at the input edge.

use crate::core::text::Withheld;
use std::fmt;
use std::sync::Arc;

/// The two formats admitted by the image spike.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ImageMedia {
    Jpeg,
    Png,
}

impl ImageMedia {
    pub(crate) const fn mime(self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
        }
    }
}

/// Immutable bytes; no source name is model evidence or identity.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct ImageInput {
    pub(crate) media: ImageMedia,
    pub(crate) bytes: Arc<[u8]>,
}

impl fmt::Debug for ImageInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageInput")
            .field("media", &self.media)
            .field("bytes", &Withheld(self.bytes.len()))
            .finish()
    }
}
