//! Exact image decision-route admission and wire fields (0448).
//! Sources: Liquid decision-models and Perplexity decisions/quickstart,
//! checked 2026-10-06. Unknown local runtime/projector budgets fail closed.

use crate::core::backend_profile::{BackendProfile, LimitKind, ProfileLimit, ProfileName};

pub(crate) mod local;
use crate::core::image::ImageState;
use serde::Serialize;
use thiserror::Error;

/// An explicit fixed-route protocol, never inferred from a URL suffix.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum ImageRoute {
    #[default]
    Unsupported,
    Liquid,
    Perplexity,
    /// Named local wire route; admission still needs an explicit declaration.
    Local,
}

pub(crate) fn named(name: &str, path: &str) -> ImageRoute {
    match (name, path) {
        ("liquid", "systemone") => ImageRoute::Liquid,
        ("perplexity", "decisions") => ImageRoute::Perplexity,
        ("llamacpp", "systemone") => ImageRoute::Local,
        _ => ImageRoute::Unsupported,
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub(crate) enum ImageLimit {
    #[error("images are unsupported on this model and decision route")]
    Unsupported,
    #[error("image {ordinal} exceeds {kind}: limit {limit}, actual {actual}")]
    Dimension {
        ordinal: usize,
        kind: &'static str,
        limit: u64,
        actual: u64,
    },
    #[error("image admission arithmetic overflow")]
    Overflow,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct ImageWire {
    pub(crate) state: String,
    pub(crate) images: Option<String>,
    pub(crate) body_limit: usize,
    pub(crate) questions_limit: Option<usize>,
    pub(crate) tokens_per_question: u64,
    pub(crate) image_tokens_known: bool,
}

impl ImageRoute {
    /// Admit route/model authority before an edge opens compressed image files.
    pub(crate) fn admit_header(
        self,
        model: &str,
        profile: Option<&BackendProfile>,
    ) -> Result<(), ImageLimit> {
        let supported = profile
            .and_then(|profile| profile.image_profile.as_ref())
            .map_or_else(
                || {
                    matches!(
                        (self, model),
                        (Self::Liquid, "d1") | (Self::Perplexity, "pplx-decider-v1-27b")
                    )
                },
                |declaration| self == Self::Local && declaration.matches(model),
            );
        if supported {
            Ok(())
        } else {
            Err(ImageLimit::Unsupported)
        }
    }

    pub(crate) fn admit_profiled(
        self,
        model: &str,
        input: &ImageState,
        profile: Option<&BackendProfile>,
    ) -> Result<ImageWire, ImageLimit> {
        self.admit_header(model, profile)?;
        if let Some(declaration) = profile.and_then(|profile| profile.image_profile.as_ref()) {
            return if self == Self::Local && declaration.matches(model) {
                local::admit(declaration.id, input)
            } else {
                Err(ImageLimit::Unsupported)
            };
        }
        self.admit(model, input)
    }

    pub(crate) fn admit(self, model: &str, input: &ImageState) -> Result<ImageWire, ImageLimit> {
        self.admit_header(model, None)?;
        let mut patches = 0u64;
        let mut tokens = 0u64;
        for (at, image) in input.images.iter().enumerate() {
            let (w, h) = (u64::from(image.width.get()), u64::from(image.height.get()));
            let count = match self {
                Self::Liquid => {
                    check(at, "aspect ratio", 100, w.max(h).div_ceil(w.min(h)))?;
                    w.div_ceil(32)
                        .checked_mul(h.div_ceil(32))
                        .ok_or(ImageLimit::Overflow)?
                }
                Self::Perplexity => {
                    // Round ties upward. Clamp rounded zero to one tile: this
                    // refuses uncertain tiny-image boundaries conservatively.
                    let tiles = |side: u64| (side + 16).checked_div(32).unwrap_or(0).max(1);
                    let count = tiles(w).checked_mul(tiles(h)).ok_or(ImageLimit::Overflow)?;
                    check(at, "32-pixel tiles", 2048, count)?;
                    count
                }
                Self::Unsupported | Self::Local => return Err(ImageLimit::Unsupported),
            };
            patches = patches.checked_add(count).ok_or(ImageLimit::Overflow)?;
            tokens = tokens
                .checked_add(
                    count
                        .checked_mul(3)
                        .ok_or(ImageLimit::Overflow)?
                        .div_ceil(2),
                )
                .ok_or(ImageLimit::Overflow)?;
        }
        if self == Self::Liquid {
            check(0, "total 32-pixel patches", 10_000, patches)?;
        }
        let text = serde_json::to_string(&input.text).map_err(|_| ImageLimit::Overflow)?;
        let urls: Vec<String> = input.images.iter().map(|image| image.data_url()).collect();
        let (state, images) = match self {
            Self::Liquid => (
                text,
                Some(serde_json::to_string(&urls).map_err(|_| ImageLimit::Overflow)?),
            ),
            Self::Perplexity => {
                #[derive(Serialize)]
                struct Url<'a> {
                    url: &'a str,
                }
                #[derive(Serialize)]
                struct Part<'a> {
                    r#type: &'static str,
                    image_url: Url<'a>,
                }
                let parts = urls
                    .iter()
                    .map(|url| {
                        serde_json::to_string(&Part {
                            r#type: "image_url",
                            image_url: Url { url },
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| ImageLimit::Overflow)?;
                let mut values = Vec::new();
                if input.text.as_str() != Some("") {
                    values.push(text);
                }
                values.extend(parts);
                (format!("[{}]", values.join(",")), None)
            }
            Self::Unsupported | Self::Local => return Err(ImageLimit::Unsupported),
        };
        Ok(ImageWire {
            state,
            images,
            body_limit: if self == Self::Liquid {
                4_499_999
            } else {
                32 * 1024 * 1024
            },
            questions_limit: (self == Self::Perplexity).then_some(128),
            // Perplexity publishes no exact image tokenizer. This conservative
            // tile estimate is an SDK estimate, never exact context admission.
            tokens_per_question: tokens,
            image_tokens_known: true,
        })
    }
}

fn check(at: usize, kind: &'static str, limit: u64, actual: u64) -> Result<(), ImageLimit> {
    if actual > limit {
        Err(ImageLimit::Dimension {
            ordinal: at + 1,
            kind,
            limit,
            actual,
        })
    } else {
        Ok(())
    }
}

pub(crate) fn body_limit(limit: usize, actual: usize) -> ProfileLimit {
    ProfileLimit {
        name: ProfileName::image_route(),
        kind: LimitKind::RequestBytes,
        limit,
        actual,
    }
}

impl std::fmt::Debug for ImageWire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageWire")
            .field("body_limit", &self.body_limit)
            .field("tokens_per_question", &self.tokens_per_question)
            .field("image_tokens_known", &self.image_tokens_known)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
#[path = "image_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "images/local_tests.rs"]
mod local_tests;
