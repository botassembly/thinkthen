//! Closed profile and decoded-original edge tables; transport tests live outside core.
use super::{ImageLimit, ImageRoute, named};
use crate::core::image::ImageState;
use crate::core::{BackendProfile, Json};
use crate::{ImageInput, ImageMedia};

const CLEF: &str =
    include_str!("../../../../../../../specification/fixtures/images/local/clef-profile.json");
const FLASH: &str = include_str!(
    "../../../../../../../specification/fixtures/images/local/clef-flash-profile.json"
);
const IMAJEV: &str =
    include_str!("../../../../../../../specification/fixtures/images/local/imajev-profile.json");
const PNG: &[u8] = include_bytes!("../../../../../../../specification/fixtures/images/red.png");

#[expect(clippy::unwrap_used, reason = "decoded fixture pixels must be valid")]
fn state(bytes: &[u8], media: ImageMedia, count: usize) -> ImageState {
    let image = ImageInput::new(media, bytes.to_vec()).unwrap();
    ImageState {
        text: Json::String("PRIVATE_CONTEXT".into()),
        images: vec![image.0; count].into(),
    }
}

#[test]
fn declarations_require_exact_ids_closed_fields_and_literal_aliases() {
    for profile in [CLEF, FLASH, IMAJEV] {
        let parsed = BackendProfile::parse(profile).unwrap();
        assert!(parsed.image_profile.is_some());
        assert!(!format!("{parsed:?}").contains("local-0036"));
    }
    let valid = r#"{"schema":"thinkthen.backend-profile/2","name":"p","image_profile":{"id":"clef-llamacpp-v0.6.0-0036","model_alias":"PRIVATE_ALIAS"}}"#;
    let parsed = BackendProfile::parse(valid).unwrap();
    let declaration = parsed.image_profile.as_ref().unwrap();
    assert!(declaration.matches("PRIVATE_ALIAS"));
    assert!(!declaration.matches("private_alias"));
    assert!(!declaration.matches(" PRIVATE_ALIAS"));
    assert!(!format!("{parsed:?}").contains("PRIVATE_ALIAS"));
    for invalid in [
        valid.replace("/2", "/1"),
        valid.replace("clef-llamacpp-v0.6.0-0036", "clef"),
        valid.replace("PRIVATE_ALIAS", " PRIVATE_ALIAS"),
        valid.replace("PRIVATE_ALIAS", ""),
        valid.replace("PRIVATE_ALIAS", "PRIVATE_ALIAS\\n"),
        valid.replace("\"model_alias\"", "\"model\""),
        valid.replace(
            "\"image_profile\":{",
            "\"image_profile\":{\"context\":8192,",
        ),
        valid.replace(
            "\"image_profile\":{",
            "\"url\":\"PRIVATE_VALUE\",\"image_profile\":{",
        ),
        valid.replace("\"image_profile\":{", "\"image_profile\":null,\"extra\":{ "),
    ] {
        let error = BackendProfile::parse(&invalid).unwrap_err();
        assert!(!format!("{error:?} {error}").contains("PRIVATE_"));
    }
}

#[test]
fn named_route_and_alias_must_match_and_unprofiled_local_routes_stay_closed() {
    let image = state(PNG, ImageMedia::Png, 1);
    for (profile, model) in [
        (CLEF, "clef-local-0036"),
        (FLASH, "flash-local-0036"),
        (IMAJEV, "imajev-2b"),
    ] {
        let profile = BackendProfile::parse(profile).unwrap();
        assert!(
            named("llamacpp", "systemone")
                .admit_profiled(model, &image, Some(&profile))
                .is_ok()
        );
        for route in [
            ImageRoute::Liquid,
            ImageRoute::Perplexity,
            ImageRoute::Unsupported,
            named("openrouter", "systemone"),
            named("mlx", "systemone"),
            named("llamacpp", "decisions"),
        ] {
            assert_eq!(
                route.admit_profiled(model, &image, Some(&profile)),
                Err(ImageLimit::Unsupported)
            );
        }
        assert_eq!(
            ImageRoute::Local.admit_profiled("other", &image, Some(&profile)),
            Err(ImageLimit::Unsupported)
        );
        assert_eq!(
            ImageRoute::Local.admit(model, &image),
            Err(ImageLimit::Unsupported)
        );
    }
}

#[test]
fn local_envelope_admits_exact_decoded_edges_and_refuses_the_next_byte_pixel_or_attachment() {
    let profile = BackendProfile::parse(CLEF).unwrap();
    let cases = [
        (
            include_bytes!(
                "../../../../../../../specification/fixtures/images/local/bytes-1048576.png"
            )
            .as_slice(),
            2,
            None,
        ),
        (
            include_bytes!(
                "../../../../../../../specification/fixtures/images/local/bytes-1048577.png"
            )
            .as_slice(),
            1,
            Some(("compressed bytes", 1_048_576, 1_048_577)),
        ),
        (
            include_bytes!(
                "../../../../../../../specification/fixtures/images/local/edge-1024.png"
            )
            .as_slice(),
            2,
            None,
        ),
        (
            include_bytes!(
                "../../../../../../../specification/fixtures/images/local/edge-1025.png"
            )
            .as_slice(),
            1,
            Some(("longest edge", 1024, 1025)),
        ),
        (PNG, 3, Some(("image count", 2, 3))),
    ];
    for (bytes, count, expected) in cases {
        let input = state(bytes, ImageMedia::Png, count);
        match expected {
            None => {
                let wire = ImageRoute::Local
                    .admit_profiled("clef-local-0036", &input, Some(&profile))
                    .unwrap();
                assert_eq!(wire.body_limit, 2_800_000);
                assert_eq!(wire.questions_limit, None);
                assert!(!wire.image_tokens_known);
                let urls: Vec<String> =
                    serde_json::from_str(wire.images.as_ref().unwrap()).unwrap();
                assert_eq!(urls.len(), count);
                assert!(urls.iter().all(|url| url == &input.images[0].data_url()));
                assert!(!format!("{wire:?}").contains("PRIVATE_CONTEXT"));
                assert!(!format!("{wire:?}").contains("base64"));
            }
            Some((kind, limit, actual)) => assert!(matches!(
                ImageRoute::Local.admit_profiled("clef-local-0036",&input,Some(&profile)),
                Err(ImageLimit::Dimension {kind: got,limit: bound,actual: size,..}) if (got,bound,size)==(kind,limit,actual)
            )),
        }
    }
}

#[test]
fn native_receipt_pair_and_grouped_bodies_preserve_originals_and_ancillary_state() {
    use base64::Engine as _;
    for (profile, exchange) in [
        (
            CLEF,
            include_str!(
                "../../../../../../../specification/fixtures/images/local/0036-clef-receipt.json"
            ),
        ),
        (
            CLEF,
            include_str!(
                "../../../../../../../specification/fixtures/images/local/0036-clef-grouped.json"
            ),
        ),
        (
            FLASH,
            include_str!(
                "../../../../../../../specification/fixtures/images/local/0036-flash-pair.json"
            ),
        ),
        (
            IMAJEV,
            include_str!(
                "../../../../../../../specification/fixtures/images/local/0036-imajev-partial.json"
            ),
        ),
    ] {
        let exchange: serde_json::Value = serde_json::from_str(exchange).unwrap();
        let request = &exchange["request"];
        let images = request["images"]
            .as_array()
            .unwrap()
            .iter()
            .map(|url| {
                let (prefix, encoded) = url.as_str().unwrap().split_once(',').unwrap();
                let media = if prefix == "data:image/jpeg;base64" {
                    ImageMedia::Jpeg
                } else {
                    ImageMedia::Png
                };
                ImageInput::new(
                    media,
                    base64::engine::general_purpose::STANDARD
                        .decode(encoded)
                        .unwrap(),
                )
                .unwrap()
                .0
            })
            .collect();
        let input = ImageState {
            text: Json::parse(&request["state"].to_string()).unwrap(),
            images,
        };
        let profile = BackendProfile::parse(profile).unwrap();
        let wire = ImageRoute::Local
            .admit_profiled(request["model"].as_str().unwrap(), &input, Some(&profile))
            .unwrap();
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&wire.state).unwrap(),
            request["state"]
        );
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(wire.images.as_ref().unwrap()).unwrap(),
            request["images"]
        );
        assert!(!wire.image_tokens_known);
    }
}
