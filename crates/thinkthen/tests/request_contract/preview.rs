//! Canonical previews retain native evidence and controls without executing.
use super::*;

#[test]
fn canonical_preview_resolves_question_files_and_ordered_duplicate_sources() {
    let listener = Listener::answering(response).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .no_cache()
        .build()
        .unwrap();
    let folder = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("request-preview-source-{}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    let question_path = folder.join("question.json");
    let source_path = folder.join("evidence.txt");
    std::fs::write(&question_path, r#"{"decide":"Fits?"}"#).unwrap();
    std::fs::write(&source_path, "Alpha.\nBeta.\n").unwrap();
    let request = Request::new(RequestCall::Decide(RequestArguments {
        question: RequestQuestion::File {
            path: question_path,
        },
        input: RequestInput::Source {
            source: RequestSource {
                framing: None,
                paths: vec![source_path.clone(), source_path],
                reading: ReaderOptions::default(),
                media: ReaderMedia::Text,
            },
        },
        options: RequestOptions::default(),
    }))
    .admit()
    .unwrap();
    let actual = engine
        .plan_request(&request, RequestEnvironment::default())
        .unwrap();
    let expected = engine
        .plan(
            &Question::decide("Fits?").unwrap().cut(),
            ["Alpha.", "Beta.", "Alpha.", "Beta."],
        )
        .unwrap();
    assert_eq!(actual, expected);
    assert_eq!(listener.count(), 0);
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn canonical_atomic_previews_match_existing_plans_without_a_key() {
    let listener = Listener::answering(response).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .no_cache()
        .build()
        .unwrap();
    for authored in [
        r#"{"decide":"Fits?"}"#,
        r#"{"choose":"Which?","options":["a","b"]}"#,
        r#"{"tag":"Which?","labels":["a"]}"#,
        r#"{"score":"Grade?","levels":["low","high"]}"#,
    ] {
        let loaded = Question::from_json(authored).unwrap();
        let LoadedQuestion::Question(question) = &loaded else {
            panic!("ordinary question")
        };
        let mut arguments = args(
            loaded.clone().into(),
            RequestInput::Records {
                items: vec![item("Alpha."), item("Beta.")],
            },
        );
        arguments.options.context = Some("Policy.".into());
        arguments.options.batch = Some(RequestBatch::Count(1));
        arguments.options.model = Some("override".into());
        arguments.options.max_requests_total = Some(0);
        let call = match question.kind() {
            QuestionKind::Decide => RequestCall::Decide(arguments),
            QuestionKind::Choose => RequestCall::Choose(arguments),
            QuestionKind::Tag => RequestCall::Tag(arguments),
            QuestionKind::Score => RequestCall::Score(arguments),
            _ => panic!("atomic fixture"),
        };
        let request = Request::new(call).admit().unwrap();
        let actual = engine
            .plan_request(&request, RequestEnvironment::default())
            .unwrap();
        let prepared =
            Question::from_json(&authored.replacen("{", "{\"model\":\"override\",", 1)).unwrap();
        let LoadedQuestion::Question(prepared) = prepared else {
            panic!("ordinary fixture")
        };
        let expected = engine
            .plan_with(
                &prepared,
                ["Alpha.", "Beta."],
                CallOptions::new()
                    .context("Policy.")
                    .batch(RequestBatch::Count(1).native().unwrap()),
            )
            .unwrap();
        assert_eq!(actual, expected);
        assert!(!format!("{actual:?}").contains("Policy."));
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn filter_and_rank_previews_reuse_decide_packing_without_sending() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let expected = engine
        .plan(
            &Question::decide("Fits?").unwrap().cut(),
            ["Alpha.", "Beta."],
        )
        .unwrap();
    for call in [
        RequestCall::Filter(args(
            Question::decide("Fits?").unwrap().cut().into(),
            RequestInput::Records {
                items: vec![item("Alpha."), item("Beta.")],
            },
        )),
        RequestCall::Rank(args(
            Question::rank("Fits?").unwrap().into(),
            RequestInput::Records {
                items: vec![item("Alpha."), item("Beta.")],
            },
        )),
    ] {
        let estimate = engine
            .plan_request(
                &Request::new(call).admit().unwrap(),
                RequestEnvironment::default(),
            )
            .unwrap();
        assert_eq!(estimate, expected);
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn canonical_preview_keeps_native_cancellation_and_reader_failures() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let request = Request::new(RequestCall::Decide(args(
        Question::decide("Fits?").unwrap().cut().into(),
        RequestInput::Feed {
            name: "input".into(),
            framing: RequestFraming::Document,
            reading: ReaderOptions::default(),
            images: vec![],
        },
    )))
    .admit()
    .unwrap();
    let token = CancelToken::new();
    token.cancel();
    let mut advanced = false;
    let error = engine
        .plan_request(
            &request,
            RequestEnvironment {
                controls: CallOptions::new().cancel(&token),
                feed: Some(RequestFeed::new(
                    "input",
                    std::iter::from_fn(|| {
                        advanced = true;
                        Some(Ok(item("private")))
                    }),
                )),
            },
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Cancelled);
    assert!(!advanced);
    let pulls = std::cell::Cell::new(0);
    let error = engine
        .plan_request(
            &request,
            RequestEnvironment {
                controls: CallOptions::new(),
                feed: Some(RequestFeed::new(
                    "input",
                    std::iter::from_fn(|| {
                        pulls.set(pulls.get() + 1);
                        match pulls.get() {
                            1 => Some(Ok(item("Alpha."))),
                            2 => Some(Err(Error::new(ErrorKind::Local, "reader failed"))),
                            _ => panic!("preview must stop after the reader failure"),
                        }
                    }),
                )),
            },
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert_eq!(pulls.get(), 2);
    assert_eq!(listener.count(), 0);
}

#[test]
fn canonical_image_preview_keeps_order_duplicates_and_native_wire_bytes() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"local","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .unwrap();
    let engine = Engine::builder()
        .backend("llamacpp")
        .unwrap()
        .base_url(listener.base())
        .unwrap()
        .model("clef-local-0036")
        .unwrap()
        .profile_json(include_str!(
            "../../../../specification/fixtures/images/local/clef-profile.json"
        ))
        .unwrap()
        .api_key("request-fixture")
        .unwrap()
        .no_cache()
        .build()
        .unwrap();
    let red = include_bytes!("../../../../specification/fixtures/images/red.png");
    let blue = include_bytes!("../../../../specification/fixtures/images/blue.png");
    for (at, bytes) in [
        [red.as_slice(), blue.as_slice()],
        [red.as_slice(), red.as_slice()],
    ]
    .into_iter()
    .enumerate()
    {
        let images = bytes
            .map(|bytes| RequestImage::Bytes {
                media: ImageMedia::Png,
                bytes: bytes.to_vec(),
            })
            .to_vec();
        let request = Request::new(RequestCall::Decide(args(
            Question::decide("Compare originals.").unwrap().cut().into(),
            RequestInput::Text {
                text: "Caption.".into(),
                images,
            },
        )))
        .admit()
        .unwrap();
        let estimate = engine
            .plan_request(&request, RequestEnvironment::default())
            .unwrap();
        assert_eq!(estimate.records(), 1);
        assert_eq!(listener.count(), at);
        engine
            .execute_request(&request, RequestEnvironment::default())
            .unwrap();
        assert_eq!(listener.requests()[0].body, estimate.first_body().unwrap());
    }
}

#[test]
fn preview_retains_json_context_shortlists_and_matches_execution() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let request = Request::from_json(r#"{
      "schema":"thinkthen.request/1",
      "call":{"function":"choose","question":{"kind":"definition","value":{"choose":"Which?","options":["fallback","other"]}},
        "input":{"kind":"records","items":[
          {"original":{"kind":"json","value":{"body":"Alpha.","private":"unsent"}},"context":"policy","options":[{"name":"OUT","description":null},{"name":"other"}]},
          {"original":{"kind":"json","value":{"body":"Beta.","private":"unsent"}},"context":"","options":[{"name":"IN"},{"name":"other"}]}
        ]},"options":{"field":["/body"],"context":"fallback context","batch":1,"max_requests_total":0}}
    }"#).unwrap().admit().unwrap();
    let estimate = engine
        .plan_request(&request, RequestEnvironment::default())
        .unwrap();
    assert_eq!(estimate.records(), 2);
    assert_eq!(estimate.requests(), 2);
    assert_eq!(listener.count(), 0, "preview must not execute");
    let body = std::str::from_utf8(estimate.first_body().unwrap()).unwrap();
    assert!(body.contains("Alpha."));
    assert!(body.contains("policy"));
    assert!(body.contains("OUT"));
    assert!(!body.contains("unsent"));
    assert!(!body.contains("fallback context"));
    let mut wire: Value =
        serde_json::from_str(&serde_json::to_string(&request.request()).unwrap()).unwrap();
    wire["call"]["options"]
        .as_object_mut()
        .unwrap()
        .remove("max_requests_total");
    let execute = Request::from_json(&wire.to_string())
        .unwrap()
        .admit()
        .unwrap();
    engine
        .execute_request(&execute, RequestEnvironment::default())
        .unwrap();
    assert!(
        listener
            .requests()
            .iter()
            .any(|request| request.body == estimate.first_body().unwrap())
    );
}

#[test]
fn unsupported_preview_refuses_before_advancing_the_feed() {
    let listener = Listener::answering(response).unwrap();
    let mut advanced = false;
    let request = Request::new(RequestCall::Find(args(
        Question::find("Fits?").unwrap().into(),
        RequestInput::Feed {
            name: "input".into(),
            framing: RequestFraming::Document,
            reading: ReaderOptions::default(),
            images: vec![],
        },
    )))
    .admit()
    .unwrap();
    let error = engine(&listener)
        .plan_request(
            &request,
            RequestEnvironment {
                controls: CallOptions::new(),
                feed: Some(RequestFeed::new(
                    "input",
                    std::iter::from_fn(|| {
                        advanced = true;
                        Some(Ok(item("must not read")))
                    }),
                )),
            },
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(!advanced);
    assert_eq!(listener.count(), 0);
}
