//! Stays: image dimension rounding and patch arithmetic need exact numeric boundary tables.
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

#[test]
fn typesafe_pins_models_and_published_boundaries_without_token_guesses() {
    for model in ["jev-1.13.0", "jev-1.14.0"] {
        for dimensions in [
            (256, 400),
            (257, 400),
            (400, 256),
            (400, 257),
            (280, 280),
            (1567, 1567),
            (1568, 1568),
        ] {
            let wire = ImageRoute::TypeSafe
                .admit(model, &state(&[dimensions]))
                .unwrap();
            assert!(!wire.image_tokens_known);
            assert_eq!(wire.tokens_per_question, 0);
            assert_eq!(wire.questions_limit, None);
            assert_eq!(wire.body_limit, 32 * 1024 * 1024);
        }
    }
    for model in ["jev-1.13", "jev-latest", "jev-preview", "jev-1.14.1"] {
        assert_eq!(
            ImageRoute::TypeSafe.admit(model, &state(&[(280, 280)])),
            Err(ImageLimit::Unsupported)
        );
    }
    for (dimensions, message) in [
        ((255, 400), "image 1 is below width: limit 256, actual 255"),
        ((400, 255), "image 1 is below height: limit 256, actual 255"),
        (
            (279, 281),
            "image 1 is below pixels: limit 78400, actual 78399",
        ),
        (
            (1569, 1568),
            "image 1 exceeds width: limit 1568, actual 1569",
        ),
        (
            (1568, 1569),
            "image 1 exceeds height: limit 1568, actual 1569",
        ),
    ] {
        assert_eq!(
            ImageRoute::TypeSafe
                .admit("jev-1.13.0", &state(&[dimensions]))
                .unwrap_err()
                .to_string(),
            message
        );
    }
    for length in [16 * 1024 * 1024 - 1, 16 * 1024 * 1024, 16 * 1024 * 1024 + 1] {
        let mut input = state(&[(280, 280)]);
        Arc::make_mut(&mut input.images)[0].bytes = vec![0; length].into();
        let result = ImageRoute::TypeSafe.admit("jev-1.13.0", &input);
        assert_eq!(result.is_ok(), length <= 16 * 1024 * 1024);
    }
}

#[test]
fn typesafe_pixel_boundaries_remain_inclusive_when_side_limits_mask_the_upper_one() {
    // Above the maximum pixel area, a side must also exceed its limit.
    for actual in [2_458_623, 2_458_624, 2_458_625] {
        assert_eq!(
            super::check(0, "pixels", 2_458_624, actual).is_ok(),
            actual <= 2_458_624
        );
    }
    for actual in [78_399, 78_400, 78_401] {
        assert_eq!(
            super::minimum(0, "pixels", 78_400, actual).is_ok(),
            actual >= 78_400
        );
    }
}
