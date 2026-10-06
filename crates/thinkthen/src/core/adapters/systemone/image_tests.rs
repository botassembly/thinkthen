//! Documented geometric boundaries, independent of encoded-body admission.
use super::{ImageLimit, ImageRoute};
use crate::core::Json;
use crate::core::image::{Image, ImageMedia, ImageState};
use std::{num::NonZeroU32, sync::Arc};
fn state(dimensions: &[(u32, u32)]) -> ImageState {
    ImageState {
        text: Json::String(String::new()),
        images: dimensions
            .iter()
            .map(|&(w, h)| Image {
                media: ImageMedia::Png,
                bytes: Arc::from([1u8]),
                width: NonZeroU32::new(w).expect("valid saved fixture or loopback configuration"),
                height: NonZeroU32::new(h).expect("valid saved fixture or loopback configuration"),
            })
            .collect(),
    }
}
#[test]
fn liquid_accepts_exact_patch_and_aspect_limits_and_refuses_the_next_pixel() {
    for dimensions in [
        &[(3200, 3200)][..],
        &[(3200, 1600), (3200, 1600)],
        &[(3200, 32)],
    ] {
        assert!(ImageRoute::Liquid.admit("d1", &state(dimensions)).is_ok());
    }
    for dimensions in [
        &[(3201, 3200)][..],
        &[(3200, 1600), (3200, 1600), (32, 32)],
        &[(3201, 32)],
    ] {
        assert!(matches!(
            ImageRoute::Liquid.admit("d1", &state(dimensions)),
            Err(ImageLimit::Dimension { .. })
        ));
    }
}
#[test]
fn perplexity_documented_examples_and_conservative_rounding_pin_tile_boundaries() {
    for dimensions in [(1440, 1440), (2048, 1024), (16, 16), (1, 1)] {
        assert!(
            ImageRoute::Perplexity
                .admit("pplx-decider-v1-27b", &state(&[dimensions]))
                .is_ok()
        );
    }
    for dimensions in [(1600, 1310), (2064, 1024)] {
        assert!(matches!(
            ImageRoute::Perplexity.admit("pplx-decider-v1-27b", &state(&[dimensions])),
            Err(ImageLimit::Dimension { .. })
        ));
    }
}
#[test]
fn model_compatibility_and_dimension_arithmetic_refuse_without_panicking() {
    for (route, model) in [
        (ImageRoute::Liquid, "d1:free"),
        (ImageRoute::Perplexity, "local"),
        (ImageRoute::Unsupported, "d1"),
    ] {
        assert_eq!(
            route.admit(model, &state(&[(32, 32)])),
            Err(ImageLimit::Unsupported)
        );
    }
    for route in [ImageRoute::Liquid, ImageRoute::Perplexity] {
        let model = if route == ImageRoute::Liquid {
            "d1"
        } else {
            "pplx-decider-v1-27b"
        };
        assert!(route.admit(model, &state(&[(u32::MAX, u32::MAX)])).is_err());
    }
}
