//! Retained original admission through current native MCP carriers.
use super::super::protocol::MAX_MESSAGE;
use crate::{InputReaderOptions, ReaderMedia, ReaderOptions, SourceItems, SourceUnit};

#[test]
fn image_sources_charge_ordered_duplicates_and_stop_before_the_tail() {
    let folder =
        std::env::temp_dir().join(format!("thinkthen-mcp-image-budget-{}", std::process::id()));
    std::fs::create_dir(&folder).unwrap();
    let image = folder.join("original.png");
    let bytes = std::fs::read(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../specification/fixtures/images/above-spike.png"),
    )
    .unwrap();
    std::fs::write(&image, &bytes).unwrap();
    let tail = folder.join("invalid-tail.png");
    std::fs::write(&tail, b"not an image").unwrap();
    let admitted = MAX_MESSAGE / bytes.len();
    let mut paths = vec![image.clone(); admitted + 1];
    paths.push(tail);
    let options = InputReaderOptions {
        reading: ReaderOptions {
            unit: SourceUnit::File,
            window: None,
        },
        media: ReaderMedia::Image,
    };
    // Native readers retain their own ceiling; the narrower aggregate applies only to MCP.
    assert_eq!(
        crate::read_inputs(std::iter::repeat_n(&image, admitted + 1), options)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len(),
        admitted + 1
    );
    for (limit, successes) in [(bytes.len() * 2, 2), (bytes.len() * 2 - 1, 1)] {
        let mut source = SourceItems::bounded_images([&image, &image], options, limit).unwrap();
        for _ in 0..successes {
            assert!(source.next().unwrap().is_ok());
        }
        if successes == 1 {
            assert_eq!(
                source.next().unwrap().unwrap_err().detail().message(),
                "retained attachments exceed the input byte ceiling"
            );
        }
        assert!(source.next().is_none());
    }
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn transport_inline_preflight_precedes_validation_and_native_defaults_stay_unchanged() {
    use crate::transport::TransportAttachmentLimit;
    let request = || {
        crate::Request::new(crate::RequestCall::Decide(crate::RequestArguments {
            question: crate::RequestQuestion::Text { text: "q".into() },
            input: crate::RequestInput::Records {
                items: vec![crate::RequestItem {
                    original: None,
                    context: None,
                    options: None,
                    seed_spans: None,
                    examples: None,
                    images: vec![
                        crate::RequestImage::Bytes {
                            media: crate::ImageMedia::Png,
                            bytes: vec![0; 6]
                        };
                        2
                    ],
                }],
            },
            options: crate::RequestOptions::default(),
        }))
    };
    assert_eq!(
        request()
            .admit_for_transport(TransportAttachmentLimit::new(11).unwrap())
            .unwrap_err()
            .detail()
            .message(),
        "retained attachments exceed the input byte ceiling"
    );
    let ordinary = request().admit().unwrap_err();
    assert_ne!(
        ordinary.detail().message(),
        "retained attachments exceed the input byte ceiling"
    );
}

#[test]
fn unresolved_descriptors_charge_mixed_images_once_before_opening_evidence() {
    use crate::transport::{TransportAttachmentLimit, TransportDescriptor};
    use conformance_backend::{Canned, Listener};
    let listener = Listener::answering(|_| {
        Canned::ok(include_str!(
            "../../../../../specification/fixtures/images/liquid-decide-reply.json"
        ))
    })
    .unwrap();
    let engine = crate::Engine::builder()
        .backend("liquid")
        .unwrap()
        .model("d1")
        .unwrap()
        .base_url(listener.base())
        .unwrap()
        .api_key("fake-image-key")
        .unwrap()
        .no_cache()
        .max_retries(0)
        .build()
        .unwrap();
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../specification/fixtures/images/red.png");
    let bytes = std::fs::read(&fixture).unwrap();
    let folder =
        std::env::temp_dir().join(format!("thinkthen-mcp-descriptors-{}", std::process::id()));
    std::fs::create_dir(&folder).unwrap();
    let evidence = folder.join("evidence.txt");
    std::fs::write(&evidence, "caption").unwrap();
    for (limit, missing) in [(bytes.len() * 2, false), (bytes.len() * 2 - 1, true)] {
        let request = crate::Request::new(crate::RequestCall::Decide(crate::RequestArguments {
            question: crate::RequestQuestion::Text { text: "q".into() },
            input: crate::RequestInput::Feed {
                name: "owned".into(),
                framing: crate::RequestFraming::Document,
                reading: crate::ReaderOptions::default(),
                images: Vec::new(),
            },
            options: crate::RequestOptions::default(),
        }))
        .admit_for_transport(TransportAttachmentLimit::new(limit).unwrap())
        .unwrap();
        let descriptor = TransportDescriptor {
            item: crate::RequestItem {
                original: None,
                context: None,
                options: None,
                examples: None,
                seed_spans: None,
                images: vec![
                    crate::RequestImage::Bytes {
                        media: crate::ImageMedia::Png,
                        bytes: bytes.clone(),
                    },
                    crate::RequestImage::File {
                        path: fixture.clone(),
                        media: Some(crate::ImageMedia::Png),
                    },
                ],
            },
            source: Some(crate::RequestSource {
                paths: vec![if missing {
                    folder.join("absent.txt")
                } else {
                    evidence.clone()
                }],
                reading: crate::ReaderOptions {
                    unit: crate::SourceUnit::File,
                    window: None,
                },
                media: crate::ReaderMedia::Text,
            }),
        };
        let feed = request
            .admit_descriptor_feed("owned".into(), vec![descriptor])
            .unwrap();
        let before = listener.count();
        let outcome = engine.execute_request(
            &request,
            crate::RequestEnvironment {
                feed: Some(feed),
                controls: crate::CallOptions::new(),
            },
        );
        if missing {
            assert_eq!(
                outcome.unwrap_err().detail().message(),
                "retained attachments exceed the input byte ceiling"
            );
            assert_eq!(listener.count(), before);
        } else {
            assert!(matches!(
                outcome.unwrap(),
                crate::RequestOutcome::Complete(_)
            ));
            assert_eq!(listener.count(), before + 1);
        }
    }
    std::fs::remove_dir_all(folder).unwrap();
}
