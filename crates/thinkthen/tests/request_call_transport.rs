//! Named calls preserve transport admission and explicit failed prefixes.
use conformance_backend::{Canned, Listener};
use std::path::PathBuf;
use thinkthen::{
    CallOptions, CancelToken, Engine, ErrorKind, ImageMedia, ReaderMedia, ReaderOptions,
    RequestArguments, RequestBatch, RequestDescriptor, RequestEnvironment, RequestFeed,
    RequestFraming, RequestImage, RequestInput, RequestItem, RequestOptions, RequestOriginal,
    RequestQuestion, RequestSource, RequestValue, SourceUnit,
};

fn listener() -> Listener {
    Listener::answering(|_| {
        Canned::ok(include_str!(
            "../../../specification/fixtures/images/liquid-decide-reply.json"
        ))
    })
    .unwrap()
}
fn engine(listener: &Listener) -> Engine {
    Engine::builder()
        .backend("liquid")
        .unwrap()
        .model("d1")
        .unwrap()
        .base_url(listener.base())
        .unwrap()
        .api_key("fake-transport-key")
        .unwrap()
        .no_cache()
        .max_retries(0)
        .build()
        .unwrap()
}
fn image_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../specification/fixtures/images/red.png")
}
fn item() -> RequestItem {
    RequestItem {
        original: None,
        images: vec![],
        context: None,
        options: None,
        examples: None,
        seed_spans: None,
    }
}
fn source(path: PathBuf) -> RequestSource {
    RequestSource {
        paths: vec![path],
        framing: None,
        reading: ReaderOptions {
            unit: SourceUnit::File,
            window: None,
        },
        media: ReaderMedia::Text,
    }
}
fn arguments(input: RequestInput) -> RequestArguments {
    RequestArguments {
        question: RequestQuestion::Text { text: "q".into() },
        input,
        options: RequestOptions::default(),
    }
}
fn feed_input(images: Vec<RequestImage>) -> RequestInput {
    RequestInput::Feed {
        name: "owned".into(),
        framing: RequestFraming::Document,
        reading: ReaderOptions::default(),
        images,
    }
}
fn run_descriptors(
    engine: &Engine,
    descriptors: Vec<RequestDescriptor>,
    limit: usize,
) -> Result<thinkthen::Call<RequestValue>, thinkthen::Error> {
    engine.call(
        "decide",
        arguments(feed_input(vec![])),
        RequestEnvironment {
            controls: CallOptions::new().retained_attachment_limit(limit).unwrap(),
            feed: Some(RequestFeed::from_descriptors("owned", descriptors)),
        },
    )
}

#[test]
fn descriptor_admission_precedes_every_question_and_source_read() {
    let listener = listener();
    let engine = engine(&listener);
    let missing = image_path().with_file_name("absent-evidence.txt");
    let descriptors = vec![
        RequestDescriptor {
            item: item(),
            source: Some(source(missing.clone())),
        },
        RequestDescriptor {
            item: item(),
            source: Some(source(PathBuf::new())),
        },
    ];
    let error = run_descriptors(&engine, descriptors, 1).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.detail().message(),
        "an input source requires explicit whole text files"
    );
    assert!(error.completed().is_none());
    assert_eq!(listener.count(), 0);
    let mut contextual = item();
    contextual.original = Some(RequestOriginal::Text { text: "x".into() });
    contextual.context = Some(thinkthen::RecordContext::Text("c".into()));
    let mut args = arguments(feed_input(vec![]));
    args.options.context_field = Some("/context".into());
    let error = engine
        .call(
            "decide",
            args,
            RequestEnvironment {
                controls: CallOptions::new(),
                feed: Some(RequestFeed::from_descriptors(
                    "owned",
                    vec![RequestDescriptor {
                        item: contextual,
                        source: None,
                    }],
                )),
            },
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 0);
    let mut input = item();
    input.images = vec![RequestImage::Bytes {
        media: ImageMedia::Png,
        bytes: vec![0; 2],
    }];
    let error = run_descriptors(
        &engine,
        vec![RequestDescriptor {
            item: input,
            source: Some(source(missing)),
        }],
        1,
    )
    .unwrap_err();
    assert_eq!(
        error.detail().message(),
        "retained attachments exceed the input byte ceiling"
    );
    assert_eq!(listener.count(), 0);
}

#[test]
fn mixed_and_repeated_attachments_share_the_exact_budget_before_evidence() {
    let listener = listener();
    let engine = engine(&listener);
    let path = image_path();
    let bytes = std::fs::read(&path).unwrap();
    for (limit, missing) in [(bytes.len() * 2, false), (bytes.len() * 2 - 1, true)] {
        let mut input = item();
        input.images = vec![
            RequestImage::Bytes {
                media: ImageMedia::Png,
                bytes: bytes.clone(),
            },
            RequestImage::File {
                path: path.clone(),
                media: Some(ImageMedia::Png),
            },
        ];
        if !missing {
            input.original = Some(RequestOriginal::Text {
                text: "caption".into(),
            });
        }
        let descriptor = RequestDescriptor {
            item: input,
            source: missing.then(|| source(path.with_file_name("absent-evidence.txt"))),
        };
        let before = listener.count();
        let outcome = run_descriptors(&engine, vec![descriptor], limit);
        if missing {
            assert_eq!(
                outcome.unwrap_err().detail().message(),
                "retained attachments exceed the input byte ceiling"
            );
            assert_eq!(listener.count(), before);
        } else {
            assert!(outcome.is_ok());
            assert_eq!(listener.count(), before + 1);
        }
    }
    let mut input = item();
    input.images = vec![
        RequestImage::File {
            path: path.clone(),
            media: None,
        },
        RequestImage::File { path, media: None },
    ];
    let before = listener.count();
    let error = run_descriptors(
        &engine,
        vec![RequestDescriptor {
            item: input,
            source: None,
        }],
        bytes.len(),
    )
    .unwrap_err();
    assert_eq!(
        error.detail().message(),
        "retained attachments exceed the input byte ceiling"
    );
    assert_eq!(listener.count(), before);
}

#[test]
fn descriptor_feeds_refuse_wrong_names_shared_images_and_initial_cancellation() {
    let listener = listener();
    let engine = engine(&listener);
    for (name, shared, cancelled) in [
        ("wrong", false, false),
        ("owned", true, false),
        ("owned", false, true),
    ] {
        let token = CancelToken::new();
        if cancelled {
            token.cancel();
        }
        let images = if shared {
            vec![RequestImage::File {
                path: image_path(),
                media: None,
            }]
        } else {
            vec![]
        };
        let error = engine
            .call(
                "decide",
                arguments(feed_input(images)),
                RequestEnvironment {
                    controls: CallOptions::new().cancel(&token),
                    feed: Some(RequestFeed::from_descriptors(name, vec![])),
                },
            )
            .unwrap_err();
        assert_eq!(
            error.kind(),
            if cancelled {
                ErrorKind::Cancelled
            } else {
                ErrorKind::Usage
            }
        );
        assert_eq!(listener.count(), 0);
    }
}

#[test]
fn failed_named_calls_expose_only_the_actual_prefix_through_explicit_access() {
    let listener = listener();
    let engine = engine(&listener);
    let path = image_path();
    let bytes = std::fs::read(&path).unwrap();
    let mut args = arguments(RequestInput::Source {
        source: RequestSource {
            paths: vec![path.clone(), path],
            framing: None,
            reading: ReaderOptions {
                unit: SourceUnit::File,
                window: None,
            },
            media: ReaderMedia::Image,
        },
    });
    args.options.batch = Some(RequestBatch::Count(1));
    let error = engine
        .call(
            "decide",
            args,
            RequestEnvironment {
                controls: CallOptions::new()
                    .retained_attachment_limit(bytes.len())
                    .unwrap(),
                feed: None,
            },
        )
        .unwrap_err();
    let Some(RequestValue::Decisions(rows)) = error.completed() else {
        panic!("completed decision prefix");
    };
    assert_eq!(rows.len(), 1);
    assert_eq!(listener.count(), 1);
    assert!(error.facts().is_some());
    assert!(!format!("{error:?}").contains("original"));
    assert!(!error.to_string().contains("red.png"));
    assert!(
        !serde_json::to_string(&error.complete())
            .unwrap()
            .contains("original")
    );
}

#[test]
fn failed_feed_originals_require_explicit_prefix_access() {
    let listener = listener();
    let engine = engine(&listener);
    let marker = "sensitive-prefix-original-0639";
    let mut first = item();
    first.original = Some(RequestOriginal::Text {
        text: marker.into(),
    });
    let rows = vec![
        Ok(first),
        Err(thinkthen::Error::new(ErrorKind::Local, "producer stopped")),
    ];
    let mut args = arguments(feed_input(vec![]));
    args.options.batch = Some(RequestBatch::Count(1));
    let error = engine
        .call(
            "decide",
            args,
            RequestEnvironment {
                controls: CallOptions::new(),
                feed: Some(RequestFeed::new("owned", rows.into_iter())),
            },
        )
        .unwrap_err();
    let prefix = error.completed().expect("actual completed prefix");
    let RequestValue::Decisions(rows) = prefix else {
        panic!("decision prefix");
    };
    assert_eq!(rows.len(), 1);
    assert!(serde_json::to_string(prefix).unwrap().contains(marker));
    assert!(!format!("{error:?}").contains(marker));
    assert!(!error.to_string().contains(marker));
    assert!(
        !serde_json::to_string(&error.complete())
            .unwrap()
            .contains(marker)
    );
    let cause = std::error::Error::source(&error).expect("typed producer cause");
    assert!(!cause.to_string().contains(marker));
    assert_eq!(listener.count(), 1);
}

#[test]
fn adapter_owned_sources_are_exposed_only_by_standard_source_access() {
    let marker = "private-reader-cause-0639";
    let error = thinkthen::Error::new(ErrorKind::Local, "the reader failed")
        .with_source(std::io::Error::other(marker))
        .with_source(std::io::Error::other(
            "later cause must not replace the first",
        ));
    assert!(!format!("{error:?}").contains(marker));
    assert!(!error.to_string().contains(marker));
    assert!(
        !serde_json::to_string(&error.complete())
            .unwrap()
            .contains(marker)
    );
    let source = std::error::Error::source(&error).unwrap();
    assert_eq!(
        source.downcast_ref::<std::io::Error>().unwrap().to_string(),
        marker
    );
}
