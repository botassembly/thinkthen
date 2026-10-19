//! Canonical previews retain native evidence and controls without executing.
use super::*;
use thinkthen::RequestFunction;

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
    let request = Request::named(
        "decide",
        RequestArguments {
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
        },
    )
    .unwrap()
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
            QuestionKind::Decide => RequestCall::new(RequestFunction::Decide, arguments),
            QuestionKind::Choose => RequestCall::new(RequestFunction::Choose, arguments),
            QuestionKind::Tag => RequestCall::new(RequestFunction::Tag, arguments),
            QuestionKind::Score => RequestCall::new(RequestFunction::Score, arguments),
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
        RequestCall::new(
            RequestFunction::Filter,
            args(
                Question::decide("Fits?").unwrap().cut().into(),
                RequestInput::Records {
                    items: vec![item("Alpha."), item("Beta.")],
                },
            ),
        ),
        RequestCall::new(
            RequestFunction::Rank,
            args(
                Question::rank("Fits?").unwrap().into(),
                RequestInput::Records {
                    items: vec![item("Alpha."), item("Beta.")],
                },
            ),
        ),
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
    let request = Request::named(
        "decide",
        args(
            Question::decide("Fits?").unwrap().cut().into(),
            RequestInput::Feed {
                name: "input".into(),
                framing: RequestFraming::Document,
                reading: ReaderOptions::default(),
                images: vec![],
            },
        ),
    )
    .unwrap()
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
        let request = Request::named(
            "decide",
            args(
                Question::decide("Compare originals.").unwrap().cut().into(),
                RequestInput::Text {
                    text: "Caption.".into(),
                    images,
                },
            ),
        )
        .unwrap()
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
fn find_preview_reads_the_owned_feed_without_sending() {
    let listener = Listener::answering(response).unwrap();
    let mut advanced = 0;
    let request = Request::named(
        "find",
        args(
            Question::find("Fits?").unwrap().into(),
            RequestInput::Feed {
                name: "input".into(),
                framing: RequestFraming::Document,
                reading: ReaderOptions::default(),
                images: vec![],
            },
        ),
    )
    .unwrap()
    .admit()
    .unwrap();
    let plan = engine(&listener)
        .plan_request(
            &request,
            RequestEnvironment {
                controls: CallOptions::new(),
                feed: Some(RequestFeed::new(
                    "input",
                    [Ok(item("first candidate")), Ok(item("second candidate"))].into_iter().inspect(|_| {
                        advanced += 1;
                    }),
                )),
            },
        )
        .unwrap();
    assert_eq!(plan.records(), 2);
    assert_eq!(plan.requests(), 1);
    let body = std::str::from_utf8(plan.first_body().unwrap()).unwrap();
    assert!(body.contains("first candidate"));
    assert!(body.contains("second candidate"));
    assert_eq!(advanced, 2);
    assert_eq!(listener.count(), 0);
}

#[test]
fn source_preview_preserves_count_refusal_and_unread_tail() {
    let listener = Listener::answering(response).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .no_cache()
        .max_requests(Some(1))
        .unwrap()
        .build()
        .unwrap();
    let question = Question::decide("Fits?").unwrap().cut();
    let mut pulls = 0;
    let records = std::iter::from_fn(|| {
        pulls += 1;
        assert!(
            pulls <= 2,
            "source preview must leave the refused tail unread"
        );
        Some(Ok(SourceRecord {
            record: "Alpha.".to_owned(),
            file: "private.txt".to_owned(),
            first_line: pulls,
            last_line: pulls,
        }))
    });
    let error = engine
        .try_plan_source_with(&question, records, CallOptions::new())
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.detail().message(),
        "this engine answers at most 1 records in one call"
    );
    assert_eq!(pulls, 2);
    assert_eq!(listener.count(), 0);
}

#[test]
#[ignore = "release suite: cumulative source boundary retains more than 16 MiB"]
fn release_only_canonical_source_preview_bounds_originals_before_projection_and_unread_tail() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let folder = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("request-preview-budget-{}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    let paths: Vec<_> = (0..3)
        .map(|at| folder.join(format!("{at}.jsonl")))
        .collect();
    let original = format!(
        "{{\"body\":\"small\",\"private\":\"{}\"}}",
        "x".repeat(8 * 1024 * 1024)
    );
    for path in &paths[..2] {
        std::fs::write(path, &original).unwrap();
    }
    std::fs::write(&paths[2], [0xff]).unwrap();
    for framing in [None, Some(RequestFraming::Jsonl)] {
        let mut arguments = args(
            Question::decide("Fits?").unwrap().cut().into(),
            RequestInput::Source {
                source: RequestSource {
                    paths: paths.clone(),
                    framing,
                    reading: ReaderOptions {
                        unit: if framing.is_none() {
                            SourceUnit::File
                        } else {
                            SourceUnit::Line
                        },
                        ..ReaderOptions::default()
                    },
                    media: ReaderMedia::Text,
                },
            },
        );
        arguments.options.field = Some(vec!["/body".into()]);
        let request = Request::named("decide", arguments)
            .unwrap()
            .admit()
            .unwrap();
        let error = engine
            .plan_request(&request, RequestEnvironment::default())
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(error.detail().message(), "source plan input exceeds 16 MiB");
    }
    assert_eq!(listener.count(), 0);
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn annotation_preview_retains_group_counts_without_adding_generic_json_fields() {
    let listener = Listener::answering(response).unwrap();
    let engine = Engine::builder().base_url(listener.base()).unwrap().no_cache().build().unwrap();
    let set = QuestionSet::from_json(r#"{"version":1,"questions":{"first":{"decide":"Fits?"},"second":{"decide":"Urgent?"}}}"#).unwrap();
    for rows in [vec![], vec![item("Alpha."), item("Beta.")]] {
        let records = rows.len();
        let mut arguments = args(set.clone().into(), RequestInput::Records { items: rows });
        arguments.options.batch = Some(RequestBatch::Count(1));
        let request = Request::named("annotate", arguments).unwrap().admit().unwrap();
        let plan = engine.plan_request(&request, RequestEnvironment::default()).unwrap();
        assert_eq!(plan.records(), records);
        // Both questions read the root, so native normalization puts them in one group.
        assert_eq!(plan.question_requests(), Some((records, [records].as_slice())));
        assert!(!plan.drops_detail());
        let json = serde_json::to_value(&plan).unwrap();
        assert!(json.get("question_requests").is_none());
        assert!(json.get("dropped_detail").is_none());
        assert_eq!(json["largest_request_bytes"], plan.largest_request_bytes());
        assert_eq!(json["largest_request_estimated_input_tokens"], plan.largest_request_estimated_input_tokens());
        assert_eq!(json["token_estimate_method"], plan.token_estimate_method());
        assert_eq!(listener.count(), 0);
        assert_eq!(engine.usage().requests_sent(), 0);
    }
}

#[test]
fn recognition_preview_discloses_only_first_record_requests_with_full_input_counts() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let ask = Recognize::builder().mode(RecognitionMode::BoundaryOnly).build().unwrap();
    for rows in [vec![], vec![item("Ada"), item("Bob")]] {
        let count = rows.len();
        let request = Request::named("recognize", args(ask.clone().into(),
            RequestInput::Records { items: rows })).unwrap().admit().unwrap();
        let plan = engine.plan_request(&request, RequestEnvironment::default()).unwrap();
        assert_eq!(plan.records(), count);
        let prepared = plan.prepared_requests().collect::<Vec<_>>();
        if count == 0 {
            assert!(plan.recognition().is_none());
            assert!(prepared.is_empty());
        } else {
            let recognition = plan.recognition().unwrap();
            assert_eq!(recognition.pieces, 1);
            assert_eq!(recognition.name_requests_upper_bound, 0);
            assert_eq!(recognition.relation_pairs_upper_bound, Some(0));
            assert_eq!(prepared.len(), 1);
            assert_eq!(plan.first_body(), Some(prepared[0].body));
            assert_eq!(prepared[0].digest.len(), 64);
            let body = std::str::from_utf8(prepared[0].body).unwrap();
            assert!(body.contains("Ada"));
            assert!(!body.contains("Bob"));
            assert!(!format!("{:?}", prepared[0]).contains("Ada"));
        }
        let json = serde_json::to_value(&plan).unwrap();
        for member in ["prepared_requests", "recognition", "relations", "details"] {
            assert!(json.get(member).is_none());
        }
        assert_eq!(listener.count(), 0);
    }
}

#[test]
fn relation_preview_retains_rule_order_empty_counts_and_native_requests() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let ask = Relate::from_records_json(r#"{"version":1,"relate":{"relations":[{"name":"zeta","source":"*","target":"*","reads":"private_relation_preview_marker"},{"name":"alpha","source":"*","target":"*","reads":"linked","either":true}]}}"#).unwrap();
    let records = ["Ada", "Bob"].map(|name| RequestItem {
        original: Some(RequestOriginal::Json { value: RawRecord::json(
            &format!(r#"{{"name":"{name}","kind":"person"}}"#)).unwrap() }),
        ..item("")
    });
    for rows in [vec![], records.to_vec()] {
        let count = rows.len();
        let request = Request::named("relate", args(ask.clone().into(),
            RequestInput::Records { items: rows })).unwrap().admit().unwrap();
        let plan = engine.plan_request(&request, RequestEnvironment::default()).unwrap();
        let relations = plan.relations().unwrap();
        assert_eq!(relations.entity_count(), count);
        let rules = relations.rules().collect::<Vec<_>>();
        assert_eq!(rules.iter().map(|rule| rule.name).collect::<Vec<_>>(), ["zeta", "alpha"]);
        assert_eq!(rules[0].method, RelationMethod::YesNo);
        assert_eq!(rules[0].logical_questions, if count == 0 { 0 } else { 2 });
        assert_eq!(rules[1].logical_questions, usize::from(count != 0));
        assert!(!format!("{rules:?}").contains("private_relation_preview_marker"));
        let prepared = plan.prepared_requests().collect::<Vec<_>>();
        if count == 0 {
            assert!(prepared.is_empty());
            assert!(rules.iter().all(|rule| rule.request_count == 0));
        } else {
            assert_eq!(plan.first_body(), Some(prepared[0].body));
            assert!(std::str::from_utf8(prepared[0].body).unwrap().contains("private_relation_preview_marker"));
            assert!(rules.iter().all(|rule| rule.request_count > 0));
        }
        assert_eq!(listener.count(), 0);
        assert_eq!(engine.usage().requests_sent(), 0);
    }
}

#[test]
fn recognition_wording_overrides_saved_values_independently_before_preview() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let saved = thinkthen::Recognize::from_json(r#"{"version":1,"recognize":{"instructions":"saved_task_marker","entity_definition":"saved_entity_marker"}}"#).unwrap();
    for (instructions, entity_definition, task, entity) in [
        (None, None, "saved_task_marker", "saved_entity_marker"),
        (Some("replacement_task_marker"), None, "replacement_task_marker", "saved_entity_marker"),
        (None, Some("replacement_entity_marker"), "saved_task_marker", "replacement_entity_marker"),
    ] {
        let arguments = RequestArguments {
            question: RequestQuestion::Definition { value: saved.clone().into() },
            input: RequestInput::Text { text: "Ada".into(), images: vec![] },
            options: RequestOptions {
                instructions: instructions.map(str::to_owned),
                entity_definition: entity_definition.map(str::to_owned),
                ..RequestOptions::default()
            },
        };
        let request = Request::named("recognize", arguments).unwrap().admit().unwrap();
        let preview = engine.plan_request(&request, RequestEnvironment::default()).unwrap();
        assert!(preview.token_estimate_note().contains("token-estimate"));
        let bodies = preview.prepared_requests().map(|request| String::from_utf8_lossy(request.body).into_owned()).collect::<Vec<_>>().join("\n");
        assert!(bodies.contains(task));
        assert!(bodies.contains(entity));
    }
    for (function, instructions, entity_definition) in [
        ("decide", Some("task"), None),
        ("decide", None, Some("entity")),
        ("recognize", Some(" "), None),
        ("recognize", None, Some("")),
    ] {
        let arguments = RequestArguments {
            question: RequestQuestion::Text { text: "Fits?".into() },
            input: RequestInput::Text { text: "Ada".into(), images: vec![] },
            options: RequestOptions {
                instructions: instructions.map(str::to_owned),
                entity_definition: entity_definition.map(str::to_owned),
                ..RequestOptions::default()
            },
        };
        assert!(Request::named(function, arguments).unwrap().admit().is_err());
    }
    assert_eq!(listener.count(), 0);
}
