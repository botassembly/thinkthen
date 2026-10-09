//! Native result projections retain SQL observations without changing ordinary Requests.
use super::*;
use std::cell::Cell;

#[allow(
    clippy::unwrap_used,
    reason = "fixed native fixture construction stops on invalid setup"
)]
fn composed(text: &str) -> RecordInput<QuestionInput> {
    RecordReading::new(&[], None, None)
        .unwrap()
        .compose(RawRecord::text(text).unwrap())
        .unwrap()
        .map_original(|record| record.question_input())
}
fn feed_input() -> RequestInput {
    RequestInput::Feed {
        name: "projection".into(),
        framing: RequestFraming::Document,
        reading: ReaderOptions::default(),
        images: vec![],
    }
}
#[allow(
    clippy::panic,
    reason = "an unexpected typed fixture result stops the regression"
)]
fn filters(outcome: RequestOutcome) -> Vec<CompleteRecord<QuestionInput, CompleteFilter>> {
    let value = match outcome {
        RequestOutcome::Complete(call) => call.into_value(),
        RequestOutcome::Failed { completed, error } => {
            assert_eq!(error.kind(), ErrorKind::Local);
            completed
        }
    };
    let RequestValue::Filtered(rows) = value else {
        panic!("filter result")
    };
    rows
}

#[test]
fn native_filter_projection_retains_both_judgments_and_terminal_prefix() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    for threshold in [0.5, 0.95] {
        let q = Question::decide("Fits?")
            .unwrap()
            .cut_at(threshold)
            .unwrap();
        for eager in [true, false] {
            let admitted = Request::new(RequestCall::Filter(args(q.clone().into(), feed_input())))
                .admit()
                .unwrap();
            let records = vec![
                Ok(composed("Alpha.")),
                Err(Error::new(ErrorKind::Local, "reader stopped")),
            ];
            let feed = RequestFeed::from_records(
                "projection",
                if eager {
                    vec![Ok(composed("Alpha."))]
                } else {
                    records
                }
                .into_iter(),
            )
            .with_all_filter_results();
            let before = listener.count();
            let rows = filters(
                engine
                    .execute_request(
                        &admitted,
                        RequestEnvironment {
                            controls: CallOptions::new(),
                            feed: Some(if eager { feed.eager() } else { feed }),
                        },
                    )
                    .unwrap(),
            );
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].ordinal(), 0);
            assert_eq!(rows[0].result().value(), threshold == 0.5);
            assert_eq!(listener.count(), before + 1);
        }
        let admitted = Request::new(RequestCall::Filter(args(
            q.into(),
            RequestInput::Records {
                items: vec![item("Alpha.")],
            },
        )))
        .admit()
        .unwrap();
        let rows = filters(
            engine
                .execute_request(&admitted, RequestEnvironment::default())
                .unwrap(),
        );
        assert_eq!(rows.len(), usize::from(threshold == 0.5));
    }
}

#[test]
fn all_filter_results_refuses_incompatible_feed_without_advancing() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    for case in 0..3 {
        let advanced = Cell::new(0);
        let definition = Question::decide("Fits?").unwrap().cut().into();
        let args = args(
            definition,
            if case == 2 {
                RequestInput::Text {
                    text: "Alpha.".into(),
                    images: vec![],
                }
            } else {
                feed_input()
            },
        );
        let request = Request::new(if case == 0 {
            RequestCall::Decide(args)
        } else {
            RequestCall::Filter(args)
        })
        .admit()
        .unwrap();
        let feed = if case == 1 {
            RequestFeed::new(
                "projection",
                std::iter::once_with(|| {
                    advanced.set(advanced.get() + 1);
                    Ok(item("Alpha."))
                }),
            )
        } else {
            RequestFeed::from_records(
                "projection",
                std::iter::once_with(|| {
                    advanced.set(advanced.get() + 1);
                    Ok(composed("Alpha."))
                }),
            )
        }
        .with_all_filter_results();
        assert_eq!(
            engine
                .execute_request(
                    &request,
                    RequestEnvironment {
                        controls: CallOptions::new(),
                        feed: Some(feed)
                    }
                )
                .unwrap_err()
                .kind(),
            ErrorKind::Usage
        );
        assert_eq!(advanced.get(), 0);
        assert_eq!(listener.count(), 0);
    }
}

#[test]
fn annotation_declarations_use_each_members_selected_evidence() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let set=QuestionSet::from_json(r#"{"version":1,"questions":{"body":{"decide":"Fits?","on":"/body","item_schema":{"type":"string"}}}}"#).unwrap();
    for invalid in [false, true] {
        let values = if invalid {
            vec![r#"{"body":"Alpha."}"#, r#"{"body":42}"#]
        } else {
            vec![r#"{"body":"Alpha."}"#]
        };
        let records = values
            .iter()
            .map(|value| {
                RecordReading::new(&[], None, None)
                    .unwrap()
                    .compose(RawRecord::json(value).unwrap())
                    .unwrap()
                    .map_original(|record| record.question_input())
            })
            .collect::<Vec<_>>();
        let admitted = Request::new(RequestCall::Annotate(args(
            set.clone().into(),
            feed_input(),
        )))
        .admit()
        .unwrap();
        let before = listener.count();
        let outcome = engine.execute_request(
            &admitted,
            RequestEnvironment {
                controls: CallOptions::new(),
                feed: Some(
                    RequestFeed::from_records("projection", records.into_iter().map(Ok)).eager(),
                ),
            },
        );
        if invalid {
            assert_eq!(outcome.unwrap_err().kind(), ErrorKind::Usage);
            assert_eq!(listener.count(), before);
        } else {
            assert!(matches!(outcome.unwrap(), RequestOutcome::Complete(_)));
            assert_eq!(listener.count(), before + 1);
        }
    }
    let admitted = Request::new(RequestCall::Annotate(args(
        set.into(),
        RequestInput::Json {
            value: RawRecord::json(r#"{"body":"Alpha."}"#).unwrap(),
            images: vec![],
        },
    )))
    .admit()
    .unwrap();
    assert!(matches!(
        engine
            .execute_request(&admitted, RequestEnvironment::default())
            .unwrap(),
        RequestOutcome::Complete(_)
    ));
}

#[test]
fn record_descriptors_keep_legacy_originals_and_shortlist_replacement() {
    let reading = RecordReading::new(&["/body"], Some("/context"), Some("/options")).unwrap();
    let descriptor = RequestItem::from_record_descriptor(
        r#"{"json":{"body":"Alpha.","context":"fallback","options":["old","fallback"]},"context":"","options":["new",{"name":"other","description":null}]}"#,
    ).unwrap();
    let row = descriptor.compose_record(&reading).unwrap();
    assert!(matches!(row.context, Some(RecordContext::Text(ref text)) if text.is_empty()));
    let options = row.options.unwrap();
    assert_eq!(
        options
            .options()
            .iter()
            .map(|v| v.name.as_str())
            .collect::<Vec<_>>(),
        ["new", "other"]
    );
    assert_eq!(
        options
            .options()
            .get(1)
            .unwrap()
            .description
            .as_ref()
            .unwrap()
            .as_json(),
        "null"
    );
    for (source, literal) in [
        (r#"{"text":"null"}"#, true),
        (r#"{"document":"null"}"#, false),
        (r#"{"document":"literal text"}"#, true),
        (r#"{"json_text":"null"}"#, false),
        (r#"{"json":null}"#, false),
    ] {
        let item = RequestItem::from_record_descriptor(source).unwrap();
        assert_eq!(
            matches!(item.original, Some(RequestOriginal::Text { .. })),
            literal
        );
        item.compose_record(&RecordReading::new(&[], None, None).unwrap())
            .unwrap();
    }
}

#[test]
fn record_descriptor_refusals_keep_usage_diagnostics() {
    for (source, message) in [
        (r#"{"text":4}"#, "record text is literal text"),
        (r#"{"document":4}"#, "document is text"),
        (r#"{"json_text":4}"#, "JSON text is text"),
        (
            r#"{"text":"Alpha.","json":null}"#,
            "invalid complete record descriptor",
        ),
        (
            r#"{"text":"Alpha.","extra":true}"#,
            "invalid complete record descriptor",
        ),
        (
            r#"{"text":"Alpha.","options":4}"#,
            "options is an ordered array",
        ),
        (
            r#"{"text":"Alpha.","options":[{}]}"#,
            "option requires a name",
        ),
        (
            r#"{"text":"Alpha.","images":4}"#,
            "images is an explicit ordered array",
        ),
        (
            r#"{"text":"Alpha.","images":[{}]}"#,
            "image media is image/png or image/jpeg",
        ),
    ] {
        let error = RequestItem::from_record_descriptor(source).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(error.to_string(), message);
    }
    for source in [
        r#"{"text":"Alpha.","text":"Beta."}"#,
        r#"{"json":{"a":1,"a":2}}"#,
        r#"{"text":"Alpha.","images":[]}"#,
    ] {
        assert_eq!(
            RequestItem::from_record_descriptor(source)
                .unwrap_err()
                .kind(),
            ErrorKind::Usage
        );
    }
}

#[test]
fn record_descriptor_limit_matches_request_admission() {
    let text = "x".repeat(16 * 1024 * 1024 + 1);
    let descriptor = serde_json::to_string(&json!({"text": text})).unwrap();
    let converted = RequestItem::from_record_descriptor(&descriptor).unwrap_err();
    let canonical = Request::new(RequestCall::Decide(args(
        Question::decide("Fits?").unwrap().cut().into(),
        RequestInput::Text {
            text,
            images: vec![],
        },
    )))
    .admit()
    .unwrap_err();
    assert_eq!(converted.kind(), ErrorKind::Usage);
    assert_eq!(converted.kind(), canonical.kind());
    assert_eq!(converted.to_string(), canonical.to_string());
}

#[test]
fn record_descriptors_compose_ordered_images_with_native_validation() {
    let bytes = include_bytes!("../../../../specification/fixtures/images/red.png");
    let reading = RecordReading::new(&["/ignored"], None, None).unwrap();
    let source = json!({"images": [
        {"media": "image/png", "bytes": bytes.as_slice()},
        {"media": "image/png", "bytes": bytes.as_slice()}
    ]})
    .to_string();
    let item = RequestItem::from_record_descriptor(&source).unwrap();
    let row = item.compose_record(&reading).unwrap();
    let QuestionInput::Images(images) = row.original else {
        panic!("image-only record")
    };
    assert_eq!(images.images().len(), 2);
    assert!(images.images().iter().all(|image| image.bytes() == bytes));
    let item = RequestItem::from_record_descriptor(
        r#"{"text":"Alpha.","images":[{"media":"image/png","bytes":[1,2]}]}"#,
    )
    .unwrap();
    let converted = item
        .compose_record(&RecordReading::new(&[], None, None).unwrap())
        .unwrap_err();
    let native = ImageInput::new(ImageMedia::Png, vec![1, 2]).unwrap_err();
    assert_eq!(converted.kind(), native.kind());
    assert_eq!(converted.to_string(), native.to_string());
}
