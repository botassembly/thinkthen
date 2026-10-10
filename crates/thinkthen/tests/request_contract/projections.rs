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
#[ignore = "release-only large-input boundary; run sdlc/scripts/test-full-cases --run"]
fn release_only_record_descriptor_limit_matches_request_admission() {
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

#[test]
fn ordered_shortlist_descriptors_preserve_description_order_and_native_refusals() {
    let item = RequestItem::from_record_descriptor(r#"{"text":"Alpha."}"#).unwrap();
    let choices = r#"[{"name":"first","description":{"z":null,"a":["text"]}},{"name":"second","description":null},{"name":"third"}]"#;
    let parsed = item.clone().with_options_descriptor(choices).unwrap();
    let options = parsed.options.unwrap();
    assert_eq!(
        options.options()[0].description.as_ref().unwrap().as_json(),
        r#"{"z":null,"a":["text"]}"#
    );
    assert_eq!(
        options.options()[1].description.as_ref().unwrap().as_json(),
        "null"
    );
    assert!(options.options()[2].description.is_none());
    for (source, native) in [
        ("[]", RecordOptions::new(vec![]).unwrap_err()),
        (
            r#"[{"name":"same"},{"name":"same"}]"#,
            RecordOptions::new(vec![
                RecordOption {
                    name: "same".into(),
                    description: None,
                },
                RecordOption {
                    name: "same".into(),
                    description: None,
                },
            ])
            .unwrap_err(),
        ),
        (
            r#"[{"name":"first","description":42}]"#,
            Description::from_json("42").unwrap_err(),
        ),
    ] {
        let refused = item.clone().with_options_descriptor(source).unwrap_err();
        assert_eq!(refused.kind(), native.kind());
        assert_eq!(refused.to_string(), native.to_string());
    }
    for depth in [120, 126, 127, 128] {
        let description = format!("{}null{}", r#"{"child":"#.repeat(depth), "}".repeat(depth));
        let source =
            format!(r#"[{{"name":"first","description":{description}}},{{"name":"second"}}]"#);
        let native = Description::from_json(&description).and_then(|description| {
            RecordOptions::new(vec![
                RecordOption {
                    name: "first".into(),
                    description: Some(description),
                },
                RecordOption {
                    name: "second".into(),
                    description: None,
                },
            ])
        });
        let parsed = item.clone().with_options_descriptor(&source);
        assert_eq!(parsed.is_ok(), native.is_ok(), "description depth {depth}");
        if let (Err(parsed), Err(native)) = (parsed, native) {
            assert_eq!(parsed.to_string(), native.to_string());
        }
        let canonical =
            format!(r#"{{"original":{{"kind":"text","text":"Alpha."}},"options":{source}}}"#);
        assert_eq!(
            serde_json::from_str::<RequestItem>(&canonical).is_ok(),
            depth == 120
        );
    }
    for source in [
        r#"[{"name":"first","extra":true}]"#,
        r#"[{"name":"first","name":"second"}]"#,
    ] {
        assert_eq!(
            item.clone()
                .with_options_descriptor(source)
                .unwrap_err()
                .kind(),
            ErrorKind::Usage
        );
    }
}

#[test]
fn located_find_keeps_duplicate_occurrences_and_separates_the_none_candidate() {
    let folder = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("request-find-source-{}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    let path = folder.join("units.txt");
    std::fs::write(&path, "First.\n\nSecond.\n").unwrap();
    for (unit, none_wins) in [(SourceUnit::File, false), (SourceUnit::Line, true)] {
        let listener = Listener::answering(move |body| {
            let request: Value = serde_json::from_slice(body).unwrap();
            let probabilities = request["questions"]["q1"]["criteria"]
                .as_object().unwrap().keys()
                .map(|key| (key.clone(), json!(u8::from(key == if none_wins { "none" } else { "u002" }))))
                .collect::<serde_json::Map<_, _>>();
            Canned::ok(&json!({"model":"fixed","answers":{"q1":{"type":"choice","probabilities":probabilities}}}).to_string())
        }).unwrap();
        let mut arguments = args(
            RequestDefinition::Find(FindQuestionFile::from_json(r#"{"find":"Which?"}"#).unwrap()),
            RequestInput::Source {
                source: RequestSource {
                    framing: None,
                    paths: vec![path.clone(), path.clone()],
                    reading: ReaderOptions { unit, window: None },
                    media: ReaderMedia::Text,
                },
            },
        );
        arguments.options.none = true;
        let session = engine(&listener)
            .request_session(Request::new(RequestCall::Find(arguments)))
            .unwrap();
        let mut documents = vec![];
        loop {
            match session.try_read() {
                RequestSessionRead::Result(packet) => documents
                    .push(serde_json::from_str::<Value>(&packet.to_json().unwrap()).unwrap()),
                RequestSessionRead::End => break,
                RequestSessionRead::Pending => std::thread::yield_now(),
            }
        }
        let result = &documents
            .iter()
            .find(|packet| packet["kind"] == "aggregate")
            .unwrap()["value"];
        let candidates = result["candidates"].as_array().unwrap();
        let originals = if unit == SourceUnit::File {
            vec!["First.\n\nSecond.\n"; 2]
        } else {
            vec!["First.", "Second.", "First.", "Second."]
        };
        assert_eq!(candidates.len(), originals.len() + 1);
        for (index, (candidate, original)) in candidates.iter().zip(&originals).enumerate() {
            assert_eq!(candidate["index"], index);
            assert_eq!(candidate["input"], *original);
            let first = if unit == SourceUnit::File {
                1
            } else {
                1 + 2 * (index % 2)
            };
            let last = if unit == SourceUnit::File { 3 } else { first };
            assert_eq!(
                candidate["source"],
                json!({"file":path.to_str().unwrap(),"first_line":first,"last_line":last})
            );
        }
        let synthetic = candidates.last().unwrap();
        assert!(synthetic["index"].is_null() && synthetic["input"].is_null());
        assert!(synthetic.get("source").is_none());
        assert_eq!(
            result["index"],
            if none_wins { Value::Null } else { json!(1) }
        );
        let detail = &documents
            .iter()
            .find(|packet| packet["value"]["kind"] == "question")
            .unwrap()["value"]["detail"];
        assert_eq!(detail["inputs"], json!(originals));
        assert_eq!(
            detail["input_sources"],
            json!(
                candidates
                    .iter()
                    .take(originals.len())
                    .enumerate()
                    .map(|(index, candidate)| json!({"index":index,"source":candidate["source"]}))
                    .collect::<Vec<_>>()
            )
        );
        assert!(detail.get("input_source").is_none());
        assert_eq!(listener.count(), 1);
    }
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn empty_find_refuses_before_sending_or_inventing_a_none_answer() {
    let listener = Listener::answering(response).unwrap();
    let mut arguments = args(
        Question::find("Which?").unwrap().into(),
        RequestInput::Units { items: vec![] },
    );
    arguments.options.none = true;
    let request = Request::new(RequestCall::Find(arguments)).admit().unwrap();
    let error = engine(&listener)
        .execute_request(&request, RequestEnvironment::default())
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.detail().message(),
        "a find question offering none takes 1 to 254 units"
    );
    assert!(error.facts().is_none());
    assert_eq!(listener.count(), 0);
}

#[test]
fn located_relation_session_retains_duplicate_sources_without_question_observations() {
    let folder = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("request-relate-source-{}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    let path = folder.join("entities.jsonl");
    std::fs::write(&path, "Ada\n\n").unwrap();
    let listener = Listener::answering(response).unwrap();
    let session = engine(&listener).request_session(Request::new(RequestCall::Relate(args(
        Relate::from_records_json(r#"{"version":1,"relate":{"relations":[{"name":"follows","source":"*","target":"*","reads":"follows"}]}}"#).unwrap().into(),
        RequestInput::Source { source: RequestSource {
            framing: None,
            paths: vec![path.clone(), path.clone()],
            reading: ReaderOptions { unit: SourceUnit::Line, window: None },
            media: ReaderMedia::Text,
        } },
    )))).unwrap();
    let mut documents = vec![];
    loop {
        match session.try_read() {
            RequestSessionRead::Result(packet) => {
                documents.push(serde_json::from_str::<Value>(&packet.to_json().unwrap()).unwrap())
            }
            RequestSessionRead::End => break,
            RequestSessionRead::Pending => std::thread::yield_now(),
        }
    }
    let value = &documents
        .iter()
        .find(|packet| packet["kind"] == "aggregate")
        .unwrap()["value"];
    assert_eq!(value["input"], json!(["Ada", "Ada"]));
    assert_eq!(
        value["input_sources"],
        json!([
            {"index":0,"source":{"file":path.to_str().unwrap(),"first_line":1,"last_line":1}},
            {"index":1,"source":{"file":path.to_str().unwrap(),"first_line":1,"last_line":1}}
        ])
    );
    assert_eq!(value["value"], json!([]));
    assert_eq!(value["answer"]["questions"], json!([]));
    assert_eq!(value["meta"]["observations"], json!([]));
    assert!(
        !documents
            .iter()
            .any(|packet| packet["value"]["kind"] == "question")
    );
    assert_eq!(listener.count(), 0);
    std::fs::remove_dir_all(folder).unwrap();
}
