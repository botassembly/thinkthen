//! Composed native feeds preserve caller authority and joined failure behavior.
use super::*;
use std::cell::Cell;

fn request() -> Request {
    Request::new(RequestCall::Decide(args(
        Question::decide("Fits?").unwrap().cut().into(),
        RequestInput::Feed {
            name: "native".into(),
            framing: RequestFraming::Document,
            reading: ReaderOptions::default(),
            images: vec![],
        },
    )))
}
fn row() -> RecordInput<QuestionInput> {
    let row = RecordReading::new(&[], None, None)
        .unwrap()
        .compose(RawRecord::text("Alpha.").unwrap())
        .unwrap();
    RecordInput {
        original: row
            .original
            .with_location(SourceLocation::new("input.txt".into(), Some(7), Some(7)).unwrap())
            .question_input(),
        context: Some(RecordContext::Text(String::new())),
        options: None,
        examples: None,
        seed_spans: None,
    }
}

#[test]
fn native_feed_eager_refuses_before_sends_and_stream_retains_located_prefix() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let admitted = request().admit().unwrap();
    for eager in [true, false] {
        let rows = vec![
            Ok(row()),
            Err(Error::new(ErrorKind::Local, "reader stopped")),
        ];
        let feed = RequestFeed::from_records("native", rows.into_iter());
        let outcome = engine.execute_request(
            &admitted,
            RequestEnvironment {
                controls: CallOptions::new().context("shared context"),
                feed: Some(if eager { feed.eager() } else { feed }),
            },
        );
        if eager {
            assert_eq!(outcome.unwrap_err().kind(), ErrorKind::Local);
            assert_eq!(listener.count(), 0);
        } else {
            let RequestOutcome::Failed {
                completed: RequestValue::Decisions(rows),
                error,
            } = outcome.unwrap()
            else {
                panic!("stream retains completed decisions")
            };
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].ordinal(), 0);
            let QuestionInput::Record(original) = rows[0].original() else {
                panic!("record original")
            };
            assert_eq!(original.location().unwrap().file(), "input.txt");
            assert_eq!(original.location().unwrap().first_line(), Some(7));
            assert_eq!(error.kind(), ErrorKind::Local);
            assert_eq!(listener.count(), 1);
            assert!(
                !String::from_utf8(listener.requests()[0].body.clone())
                    .unwrap()
                    .contains("shared context")
            );
        }
    }
}

#[test]
fn native_feed_refuses_double_composition_and_cancellation_before_advancing() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    for case in 0..6 {
        let mut request = request();
        let RequestCall::Decide(args) = &mut request.call else {
            panic!("decide request")
        };
        match case {
            0 => args.options.field = Some(vec![]),
            1 => args.options.context_field = Some("/context".into()),
            2 => {
                let RequestInput::Feed { framing, .. } = &mut args.input else {
                    panic!("feed")
                };
                *framing = RequestFraming::Lines;
            }
            _ => {}
        }
        if case == 5 {
            let question = Question::tag_labels("Labels?")
                .unwrap()
                .label("a", None)
                .unwrap()
                .build()
                .unwrap();
            request.call = RequestCall::Tag(super::args(
                question.into(),
                request.call.arguments().input.clone(),
            ));
        }
        let admitted = request.admit().unwrap();
        let advanced = Cell::new(false);
        let feed = RequestFeed::from_records(
            "native",
            std::iter::once_with(|| {
                advanced.set(true);
                Ok(row())
            }),
        );
        let feed = if case >= 4 {
            feed.with_image_inputs()
        } else {
            feed
        };
        let token = CancelToken::new();
        if case == 3 {
            token.cancel();
        }
        let error = engine
            .execute_request(
                &admitted,
                RequestEnvironment {
                    controls: CallOptions::new().cancel(&token),
                    feed: Some(feed),
                },
            )
            .unwrap_err();
        assert_eq!(
            error.kind(),
            if case == 3 {
                ErrorKind::Cancelled
            } else {
                ErrorKind::Usage
            }
        );
        assert!(!advanced.get());
        assert_eq!(listener.count(), 0);
        if case == 5 {
            assert!(error.to_string().contains("tag accepts text only"));
        }
    }
}

#[test]
fn eager_native_feed_keeps_item_declarations_and_recognition_admission() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let LoadedQuestion::Question(question) = Question::from_json(
        r#"{"decide":"Fits?","item_schema":{"type":"object","properties":{"body":{"type":"string"}},"required":["body"]}}"#,
    ).unwrap() else { panic!("plain question") };
    let mut invalid_original = row();
    invalid_original.original = QuestionInput::Text("undeclared secret input".into());
    let cases = [
        (
            RequestCall::Decide(args(
                question.into(),
                request().call.arguments().input.clone(),
            )),
            invalid_original,
        ),
        (
            RequestCall::Recognize(args(
                Recognize::builder().build().unwrap().into(),
                request().call.arguments().input.clone(),
            )),
            RecordInput {
                seed_spans: Some(vec![RecognitionSeedSpan {
                    start: 1,
                    end: 3,
                    kind: None,
                }]),
                ..row()
            },
        ),
        (
            RequestCall::Recognize(args(
                Recognize::builder().build().unwrap().into(),
                request().call.arguments().input.clone(),
            )),
            RecordInput {
                examples: Some(vec![RecognitionExample::Brackets(
                    "[Ada | secret-kind]".into(),
                )]),
                ..row()
            },
        ),
    ];
    for (call, invalid) in cases {
        let admitted = Request::new(call).admit().unwrap();
        let mut valid = row();
        if matches!(admitted.request().call.function(), RequestFunction::Decide) {
            let record = RecordReading::new(&[], None, None)
                .unwrap()
                .compose(RawRecord::json(r#"{"body":"Alpha."}"#).unwrap())
                .unwrap();
            valid.original = record.original.question_input();
        }
        let feed =
            RequestFeed::from_records("native", vec![Ok(valid), Ok(invalid)].into_iter()).eager();
        let error = engine
            .execute_request(
                &admitted,
                RequestEnvironment {
                    controls: CallOptions::new(),
                    feed: Some(feed),
                },
            )
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert!(!error.to_string().contains("secret"));
        assert_eq!(listener.count(), 0);
    }
}

#[test]
fn native_choose_feed_preserves_candidate_validation_before_sends() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let question = Question::choose_labels("Which?")
        .unwrap()
        .label("a", None)
        .unwrap()
        .label("b", None)
        .unwrap()
        .build()
        .unwrap();
    let admitted = Request::new(RequestCall::Choose(args(
        question.into(),
        request().call.arguments().input.clone(),
    )))
    .admit()
    .unwrap();
    let invalid = || {
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
        .map(|options| RecordInput {
            options: Some(options),
            ..row()
        })
    };
    for eager in [true, false] {
        let rows = std::iter::once(Ok(row())).chain(std::iter::once_with(invalid));
        let feed = RequestFeed::from_records("native", rows);
        let outcome = engine.execute_request(
            &admitted,
            RequestEnvironment {
                controls: CallOptions::new(),
                feed: Some(if eager { feed.eager() } else { feed }),
            },
        );
        if eager {
            assert_eq!(outcome.unwrap_err().kind(), ErrorKind::Usage);
            assert_eq!(listener.count(), 0);
        } else {
            let RequestOutcome::Failed {
                completed: RequestValue::Choices(rows),
                error,
            } = outcome.unwrap()
            else {
                panic!("stream retains the completed choice")
            };
            assert_eq!(rows.len(), 1);
            assert_eq!(error.kind(), ErrorKind::Usage);
            assert_eq!(listener.count(), 1);
        }
    }
}

#[test]
fn undeclared_native_images_still_refuse_function_and_route_without_sends() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    for tag in [false, true] {
        let mut request = request();
        if tag {
            let question = Question::tag_labels("Labels?")
                .unwrap()
                .label("a", None)
                .unwrap()
                .build()
                .unwrap();
            request.call = RequestCall::Tag(args(
                question.into(),
                request.call.arguments().input.clone(),
            ));
        }
        let admitted = request.admit().unwrap();
        let image = ImageInput::new(
            ImageMedia::Png,
            include_bytes!("../../../../specification/fixtures/images/red.png").to_vec(),
        )
        .unwrap();
        let record = RecordInput {
            original: QuestionInput::Images(ImageEvidence::new(None, vec![image]).unwrap()),
            context: None,
            options: None,
            examples: None,
            seed_spans: None,
        };
        let error = engine
            .execute_request(
                &admitted,
                RequestEnvironment {
                    controls: CallOptions::new(),
                    feed: Some(
                        RequestFeed::from_records("native", std::iter::once(Ok(record))).eager(),
                    ),
                },
            )
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        if tag {
            assert!(error.to_string().contains("tag accepts text only"));
        }
        assert_eq!(listener.count(), 0);
    }
}
