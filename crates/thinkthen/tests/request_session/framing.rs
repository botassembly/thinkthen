use super::*;

#[test]
fn document_descriptors_preserve_typed_originals_and_explicit_context() {
    for (original, expected) in [
        (
            json!({"kind":"text","text":"{\"body\":\"first\"}"}),
            json!("{\"body\":\"first\"}"),
        ),
        (
            json!({"kind":"json","value":{"body":"first"}}),
            json!({"body":"first"}),
        ),
    ] {
        let listener = Listener::answering(response).unwrap();
        let session = engine(&listener).request_session(request(feed())).unwrap();
        session
            .try_push_json(
                &json!({"item":{"original":original,"context":"explicit policy"}}).to_string(),
            )
            .unwrap();
        session.finish(None).unwrap();
        let packets = drain(&session);
        let row = packets
            .iter()
            .find_map(|packet| match packet {
                RequestSessionResult::Row(RequestSessionRow::Decision(row)) => Some(row),
                _ => None,
            })
            .unwrap();
        let QuestionInput::Record(original) = row.original() else {
            panic!("typed original")
        };
        assert_eq!(serde_json::to_value(original.original()).unwrap(), expected);
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
            panic!("terminal")
        };
        assert!(terminal.error.is_none());
        assert_eq!(listener.count(), 1);
        assert!(
            String::from_utf8(listener.requests()[0].body.clone())
                .unwrap()
                .contains("explicit policy")
        );
    }
}

#[test]
fn session_line_projection_refuses_before_intake() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    {
        let framing = RequestFraming::Lines;
        let mut arguments = request(feed()).call.arguments().clone();
        arguments.input = RequestInput::Feed {
            name: "records".into(),
            framing,
            reading: ReaderOptions::default(),
            images: vec![],
        };
        arguments.options.field = Some(vec!["/body".into()]);
        let error = engine
            .request_session(Request::new(RequestCall::Decide(arguments)))
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.detail().message(),
            "--field: a text line has no members, so --lines takes no pointer"
        );
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn line_and_jsonl_descriptors_skip_blank_rows_and_keep_record_endings() {
    for framing in [RequestFraming::Lines, RequestFraming::Jsonl] {
        let listener = Listener::answering(response).unwrap();
        let mut arguments = request(feed()).call.arguments().clone();
        arguments.input = RequestInput::Feed {
            name: "records".into(),
            framing,
            reading: ReaderOptions::default(),
            images: vec![],
        };
        let session = engine(&listener)
            .request_session(Request::new(RequestCall::Decide(arguments)))
            .unwrap();
        session.try_push(descriptor(" \t\r\n")).unwrap();
        let mut pending = descriptor(if matches!(framing, RequestFraming::Jsonl) {
            "\"first\"\r\n"
        } else {
            "first\r\n"
        });
        let until = Instant::now() + Duration::from_secs(5);
        loop {
            match session.try_push(pending).unwrap() {
                RequestSessionPush::Accepted => break,
                RequestSessionPush::Full(value) => pending = value,
                RequestSessionPush::Closed(_) => panic!("valid framed feed closed"),
            }
            assert!(Instant::now() < until);
            std::thread::yield_now();
        }
        session.finish(None).unwrap();
        let packets = drain(&session);
        let row = packets
            .iter()
            .find_map(|packet| match packet {
                RequestSessionResult::Row(RequestSessionRow::Decision(row)) => Some(row),
                _ => None,
            })
            .unwrap();
        let QuestionInput::Record(original) = row.original() else {
            panic!("framed original")
        };
        assert_eq!(original.selected().to_json().unwrap(), "\"first\"");
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
            panic!("terminal")
        };
        assert!(terminal.error.is_none());
        assert_eq!(terminal.facts.as_ref().unwrap().records(), 1);
        assert_eq!(listener.count(), 1);
    }
}

#[test]
fn jsonl_projects_located_text_and_preserves_prefix_on_later_syntax_failure() {
    let (sent, received) = mpsc::channel();
    let listener = Listener::answering_with_events(response, sent).unwrap();
    let mut arguments = request(feed()).call.arguments().clone();
    let RequestInput::Feed { framing, .. } = &mut arguments.input else {
        unreachable!()
    };
    *framing = RequestFraming::Jsonl;
    arguments.options.field = Some(vec!["/body".into()]);
    arguments.options.context_field = Some("/policy".into());
    arguments.options.batch = Some(RequestBatch::Count(1));
    let session = engine(&listener)
        .request_session(Request::new(RequestCall::Decide(arguments)))
        .unwrap();
    assert_eq!(session.try_push_json(&json!({"item":{"original":{"kind":"text","text":"{\"body\":\"first\",\"policy\":\"local policy\"}\r\n"}},"location":{"file":"rows.jsonl","first_line":1,"last_line":1}}).to_string()).unwrap(), RequestSessionPushStatus::Accepted);
    received.recv_timeout(Duration::from_secs(5)).unwrap();
    session
        .try_push(descriptor("private malformed JSON"))
        .unwrap();
    session.finish(None).unwrap();
    let packets = drain(&session);
    let rows = packets
        .iter()
        .filter_map(|packet| match packet {
            RequestSessionResult::Row(RequestSessionRow::Decision(row)) => Some(row),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 1, "{packets:?}");
    let QuestionInput::Record(original) = rows[0].original() else {
        panic!("located original")
    };
    assert_eq!(
        serde_json::to_value(original.original()).unwrap(),
        json!({"body":"first","policy":"local policy"})
    );
    assert_eq!(original.selected().to_json().unwrap(), "\"first\"");
    assert_eq!(
        serde_json::to_value(original.location().unwrap()).unwrap(),
        json!({"file":"rows.jsonl","first_line":1,"last_line":1})
    );
    let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
        panic!("terminal")
    };
    let error = terminal.error.as_ref().unwrap();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(!format!("{error} {error:?}").contains("private malformed"));
    assert_eq!(terminal.facts.as_ref().unwrap().records(), 1);
    assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
    assert_eq!(listener.count(), 1);
    let body = String::from_utf8(listener.requests()[0].body.clone()).unwrap();
    assert!(body.contains("first"));
    assert!(body.contains("local policy"));
    assert!(!body.contains("rows.jsonl"));
}

pub(super) fn table_request(framing: RequestFraming) -> Request {
    let mut arguments = request(feed()).call.arguments().clone();
    let RequestInput::Feed {
        framing: declared, ..
    } = &mut arguments.input
    else {
        unreachable!()
    };
    *declared = framing;
    Request::new(RequestCall::Decide(arguments))
}

#[allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "owned session fixtures must remain admitted while retrying backpressure"
)]
pub(super) fn push(session: &RequestSession, mut value: RequestSessionDescriptor) {
    let until = Instant::now() + Duration::from_secs(5);
    loop {
        match session.try_push(value).unwrap() {
            RequestSessionPush::Accepted => return,
            RequestSessionPush::Full(pending) => value = pending,
            RequestSessionPush::Closed(_) => panic!("table intake unexpectedly closed"),
        }
        assert!(Instant::now() < until);
        std::thread::yield_now();
    }
}

#[test]
fn table_descriptors_decode_multiline_rows_and_preserve_projection_controls_and_location() {
    for (framing, header, row) in [
        (
            RequestFraming::Csv,
            "body,policy\r\n",
            "\"first\nsecond\",local policy\r\n",
        ),
        (
            RequestFraming::Tsv,
            "body\tpolicy\n",
            "\"first\nsecond\"\tlocal policy\n",
        ),
    ] {
        let listener = Listener::answering(response).unwrap();
        let mut arguments = table_request(framing).call.arguments().clone();
        arguments.options.field = Some(vec!["/body".into()]);
        if matches!(framing, RequestFraming::Csv) {
            arguments.options.context_field = Some("/policy".into());
        }
        let session = engine(&listener)
            .request_session(Request::new(RequestCall::Decide(arguments)))
            .unwrap();
        push(&session, descriptor(header));
        let mut item = json!({"original":{"kind":"text","text":row}});
        if matches!(framing, RequestFraming::Tsv) {
            item["context"] = json!("explicit policy");
        }
        push(&session, RequestSessionDescriptor::from_json(&json!({"item":item,"location":{"file":"private.csv","first_line":41,"last_line":42}}).to_string()).unwrap());
        session.finish(None).unwrap();
        let packets = drain(&session);
        let rows = packets
            .iter()
            .filter_map(|packet| match packet {
                RequestSessionResult::Row(RequestSessionRow::Decision(row)) => Some(row),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), 1, "{packets:?}");
        let QuestionInput::Record(original) = rows[0].original() else {
            panic!("table original")
        };
        assert_eq!(
            serde_json::to_value(original.original()).unwrap(),
            json!({"body":"first\nsecond","policy":"local policy"})
        );
        assert_eq!(
            original.selected().to_json().unwrap(),
            "\"first\nsecond\"".replace('\n', "\\n")
        );
        assert_eq!(
            serde_json::to_value(original.location().unwrap()).unwrap(),
            json!({"file":"private.csv","first_line":41,"last_line":42})
        );
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
            panic!("terminal")
        };
        assert!(terminal.error.is_none());
        assert_eq!(terminal.facts.as_ref().unwrap().records(), 1);
        assert_eq!(listener.count(), 1);
        let body = String::from_utf8(listener.requests()[0].body.clone()).unwrap();
        assert!(body.contains(if matches!(framing, RequestFraming::Csv) {
            "local policy"
        } else {
            "explicit policy"
        }));
        assert!(!body.contains("private.csv"));
    }
}

#[test]
fn table_headers_and_complete_descriptors_refuse_before_sending() {
    let invalid = [
        (
            false,
            json!({"original":{"kind":"text","text":"body,body\n"}}),
        ),
        (
            false,
            json!({"original":{"kind":"text","text":"body\nfirst\n"}}),
        ),
        (
            false,
            json!({"original":{"kind":"text","text":"body\n"},"context":""}),
        ),
        (
            false,
            json!({"original":{"kind":"text","text":"body\n"},"examples":[]}),
        ),
        (
            false,
            json!({"original":{"kind":"text","text":"body\n"},"seed_spans":[]}),
        ),
        (
            false,
            json!({"original":{"kind":"json","value":{"body":"first"}}}),
        ),
        (false, json!({"images":[]})),
        (
            false,
            json!({"original":{"kind":"text","text":"body\n"},"images":[{"kind":"file","path":"private-image.png"}]}),
        ),
        (
            true,
            json!({"original":{"kind":"text","text":"first\nsecond\n"}}),
        ),
        (
            true,
            json!({"original":{"kind":"text","text":"\n\r\n"},"context":""}),
        ),
        (
            true,
            json!({"original":{"kind":"json","value":{"body":"first"}}}),
        ),
    ];
    for framing in [RequestFraming::Csv, RequestFraming::Tsv] {
        for (header, item) in &invalid {
            let listener = Listener::answering(response).unwrap();
            let session = engine(&listener)
                .request_session(table_request(framing))
                .unwrap();
            if *header {
                push(&session, descriptor("body\n"));
            }
            let mut item = item.clone();
            if matches!(framing, RequestFraming::Tsv)
                && let Some(text) = item["original"]["text"].as_str()
            {
                item["original"]["text"] = json!(text.replace(',', "\t"));
            }
            push(
                &session,
                RequestSessionDescriptor::from_json(&json!({"item":item}).to_string()).unwrap(),
            );
            session.finish(None).unwrap();
            let packets = drain(&session);
            let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
                panic!("terminal")
            };
            assert_eq!(
                terminal.error.as_ref().unwrap().kind(),
                ErrorKind::Usage,
                "{framing:?}: {item}"
            );
            assert_eq!(listener.count(), 0);
            assert_eq!(packets.len(), 1);
            assert!(matches!(
                session.try_push(descriptor("suffix")).unwrap(),
                RequestSessionPush::Closed(_)
            ));
        }
    }
}

#[test]
fn table_reader_units_refuse_before_intake() {
    for framing in [RequestFraming::Csv, RequestFraming::Tsv] {
        for reading in [
            ReaderOptions {
                unit: SourceUnit::File,
                window: None,
            },
            ReaderOptions {
                unit: SourceUnit::Window,
                window: Some(2),
            },
        ] {
            let listener = Listener::answering(response).unwrap();
            let mut arguments = table_request(framing).call.arguments().clone();
            let RequestInput::Feed {
                reading: declared, ..
            } = &mut arguments.input
            else {
                unreachable!()
            };
            *declared = reading;
            assert_eq!(
                engine(&listener)
                    .request_session(Request::new(RequestCall::Decide(arguments)))
                    .unwrap_err()
                    .kind(),
                ErrorKind::Usage
            );
            assert_eq!(listener.count(), 0);
        }
    }
}

#[test]
fn table_eof_requires_a_header_and_preserves_data_cell_bom() {
    for framing in [RequestFraming::Csv, RequestFraming::Tsv] {
        for (header, failure, expected) in [
            (false, None, Some(ErrorKind::Usage)),
            (
                false,
                Some(RequestReaderFailure::Io { location: None }),
                Some(ErrorKind::Local),
            ),
            (true, None, None),
        ] {
            let listener = Listener::answering(response).unwrap();
            let session = engine(&listener)
                .request_session(table_request(framing))
                .unwrap();
            if header {
                let mut header = descriptor("\u{feff}body\n");
                header.location =
                    Some(SourceLocation::new("header-only".into(), Some(99), Some(99)).unwrap());
                push(&session, header);
            }
            session.finish(failure).unwrap();
            let packets = drain(&session);
            let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
                panic!("terminal")
            };
            assert_eq!(terminal.error.as_ref().map(Error::kind), expected);
            assert_eq!(listener.count(), 0);
        }
        let listener = Listener::answering(response).unwrap();
        let session = engine(&listener)
            .request_session(table_request(framing))
            .unwrap();
        push(&session, descriptor("body\n"));
        push(&session, descriptor("\n\r\n"));
        push(&session, descriptor("\u{feff}first\n"));
        session.finish(None).unwrap();
        let packets = drain(&session);
        let row = packets
            .iter()
            .find_map(|packet| match packet {
                RequestSessionResult::Row(RequestSessionRow::Decision(row)) => Some(row),
                _ => None,
            })
            .unwrap();
        let QuestionInput::Record(original) = row.original() else {
            panic!("table original")
        };
        assert_eq!(
            serde_json::to_value(original.original()).unwrap(),
            json!({"body":"\u{feff}first"})
        );
        assert!(original.location().is_none());
        assert_eq!(listener.count(), 1);
    }
}

#[test]
fn table_later_failures_preserve_completed_rows_and_actual_facts() {
    for failure in [false, true] {
        let (sent, received) = mpsc::channel();
        let listener = Listener::answering_with_events(response, sent).unwrap();
        let mut arguments = table_request(RequestFraming::Csv).call.arguments().clone();
        arguments.options.batch = Some(RequestBatch::Count(1));
        let session = engine(&listener)
            .request_session(Request::new(RequestCall::Decide(arguments)))
            .unwrap();
        push(&session, descriptor("body\n"));
        push(&session, descriptor("first\n"));
        received.recv_timeout(Duration::from_secs(5)).unwrap();
        if !failure {
            push(&session, descriptor("private,extra\n"));
        }
        session
            .finish(failure.then_some(RequestReaderFailure::Io { location: None }))
            .unwrap();
        let packets = drain(&session);
        assert_eq!(
            packets
                .iter()
                .filter(|packet| matches!(packet, RequestSessionResult::Row(_)))
                .count(),
            1
        );
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
            panic!("terminal")
        };
        let error = terminal.error.as_ref().unwrap();
        assert_eq!(
            error.kind(),
            if failure {
                ErrorKind::Local
            } else {
                ErrorKind::Usage
            }
        );
        assert!(!format!("{error} {error:?}").contains("private"));
        assert_eq!(terminal.facts.as_ref().unwrap().records(), 1);
        assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
        assert_eq!(listener.count(), 1);
    }
}

#[test]
fn table_native_stops_take_precedence_over_missing_headers() {
    for cancel in [false, true] {
        let listener = Listener::answering(response).unwrap();
        let mut arguments = table_request(RequestFraming::Csv).call.arguments().clone();
        if !cancel {
            arguments.options.deadline_ms = Some(20);
        }
        let session = engine(&listener)
            .request_session_with_feed_options(
                Request::new(RequestCall::Decide(arguments)),
                Surface::Rust,
                RequestSessionFeedOptions {
                    eager: true,
                    ..Default::default()
                },
            )
            .unwrap();
        if cancel {
            session.cancel();
            session.finish(None).unwrap();
        }
        let packets = drain(&session);
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
            panic!("terminal")
        };
        assert_eq!(
            terminal.error.as_ref().unwrap().kind(),
            if cancel {
                ErrorKind::Cancelled
            } else {
                ErrorKind::Deadline
            }
        );
        assert_eq!(listener.count(), 0);
    }
}

#[test]
#[ignore = "large-input boundary runs in the release suite"]
fn release_only_table_descriptor_bounds_include_trailing_blank_bytes() {
    let listener = Listener::answering(response).unwrap();
    let session = engine(&listener)
        .request_session(table_request(RequestFraming::Csv))
        .unwrap();
    push(&session, descriptor("body\n"));
    let mut row = "x".repeat(16 * 1024 * 1024);
    row.push_str("\n\n\n");
    push(&session, descriptor(&row));
    session.finish(None).unwrap();
    let packets = drain(&session);
    let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
        panic!("terminal")
    };
    assert_eq!(terminal.error.as_ref().unwrap().kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 0);
}

#[allow(
    clippy::unwrap_used,
    reason = "isolated native definitions must admit before the caller behavior is exercised"
)]
fn owned_eager_cases() -> Vec<Request> {
    let arguments = |definition| RequestArguments {
        question: RequestQuestion::Definition { value: definition },
        input: feed(),
        options: RequestOptions::default(),
    };
    let set = r#"{"version":1,"questions":{"fits":{"decide":"Fits?"}}}"#;
    let decide = Question::decide("Fits?").unwrap().cut();
    let choice = Question::choose_labels("Which?")
        .unwrap()
        .label("a", None)
        .unwrap()
        .label("b", None)
        .unwrap()
        .build()
        .unwrap();
    let tags = Question::tag_labels("Which?")
        .unwrap()
        .label("a", None)
        .unwrap()
        .build()
        .unwrap();
    let score = Question::score("Grade?")
        .unwrap()
        .level("low", None)
        .unwrap()
        .level("high", None)
        .unwrap()
        .build()
        .unwrap();
    vec![
        RequestCall::Decide(arguments(decide.clone().into())),
        RequestCall::Choose(arguments(choice.into())),
        RequestCall::Tag(arguments(tags.into())),
        RequestCall::Score(arguments(score.into())),
        RequestCall::Filter(arguments(decide.into())),
        RequestCall::Annotate(arguments(QuestionSet::from_json(set).unwrap().into())),
        RequestCall::Rank(arguments(Question::rank("Fits?").unwrap().into())),
        RequestCall::Rank(arguments(RankSet::from_json(set).unwrap().into())),
        RequestCall::Find(arguments(Question::find("Which?").unwrap().into())),
        RequestCall::Recognize(arguments(Recognize::builder().build().unwrap().into())),
        RequestCall::Relate(arguments(Relate::from_records_json(r#"{"version":1,"relate":{"relations":[{"name":"follows","source":"*","target":"*","reads":"follows"}]}}"#).unwrap().into())),
        RequestCall::Choose(arguments(RecordChooseQuestion::from_json(r#"{"choose":"Which?"}"#).unwrap().into())),
    ].into_iter().map(Request::new).collect()
}

#[allow(
    clippy::unwrap_used,
    reason = "isolated owned descriptors must admit before the caller behavior is exercised"
)]
fn owned_descriptor(text: &str, dynamic: bool) -> RequestSessionDescriptor {
    let mut row = descriptor(text);
    row.location = Some(SourceLocation::new("input".into(), None, None).unwrap());
    if dynamic {
        row.item.options = Some(
            RecordOptions::new(
                ["a", "b"]
                    .into_iter()
                    .map(|name| RecordOption {
                        name: name.into(),
                        description: None,
                    })
                    .collect(),
            )
            .unwrap(),
        );
    }
    row
}

#[test]
fn owned_eager_admission_closes_before_finish_and_leaves_suffix_unconsumed() {
    for (case, request) in owned_eager_cases().into_iter().enumerate() {
        let listener = Listener::answering(response).unwrap();
        let engine = Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .api_key("session-fixture")
            .unwrap()
            .no_cache()
            .max_retries(0)
            .max_requests(Some(2))
            .unwrap()
            .build()
            .unwrap();
        let session = engine
            .request_session_with_feed_options(
                request,
                Surface::Rust,
                RequestSessionFeedOptions {
                    eager: true,
                    ..Default::default()
                },
            )
            .unwrap();
        let until = Instant::now() + Duration::from_secs(5);
        let mut accepted = 0;
        let mut pending = owned_descriptor("Ada", case == 11);
        loop {
            assert!(
                Instant::now() < until,
                "case {case} waited for EOF instead of refusing"
            );
            match session.try_push(pending).unwrap() {
                RequestSessionPush::Accepted => {
                    accepted += 1;
                    assert!(accepted <= 4, "case {case} consumed the refused suffix");
                    pending = owned_descriptor("Ada", case == 11);
                }
                RequestSessionPush::Full(row) => {
                    pending = row;
                    std::thread::yield_now();
                }
                RequestSessionPush::Closed(_) => break,
            }
        }
        let packets = drain(&session);
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
            panic!("terminal")
        };
        assert_eq!(
            terminal.error.as_ref().unwrap().to_string(),
            "this engine answers at most 2 records in one call",
            "case {case}"
        );
        assert!(terminal.facts.is_none(), "case {case}");
        assert_eq!(listener.count(), 0, "case {case}");
    }
}

#[test]
fn owned_eager_late_invalid_input_sends_nothing_and_keeps_precedence() {
    for (case, request) in owned_eager_cases().into_iter().enumerate() {
        let listener = Listener::answering(response).unwrap();
        let session = engine(&listener)
            .request_session_with_feed_options(
                request,
                Surface::Rust,
                RequestSessionFeedOptions {
                    eager: true,
                    ..Default::default()
                },
            )
            .unwrap();
        push(&session, owned_descriptor("Ada", case == 11));
        let invalid = owned_descriptor(" ", case == 11);
        push(&session, invalid);
        session
            .finish_native_reader_error(Error::new(ErrorKind::Local, "later reader failure"))
            .unwrap();
        let packets = drain(&session);
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
            panic!("terminal")
        };
        assert_eq!(
            terminal.error.as_ref().unwrap().kind(),
            ErrorKind::Usage,
            "case {case}"
        );
        assert_ne!(
            terminal.error.as_ref().unwrap().to_string(),
            "later reader failure"
        );
        assert_eq!(listener.count(), 0, "case {case}");
    }
}

#[test]
fn native_reader_finish_preserves_diagnostic_after_completed_prefix() {
    let listener = Listener::answering(response).unwrap();
    let session = engine(&listener)
        .request_session_with_feed_options(
            request(feed()),
            Surface::Rust,
            RequestSessionFeedOptions::default(),
        )
        .unwrap();
    push(&session, descriptor("first"));
    assert_eq!(
        session
            .finish_native_reader_error(Error::new(ErrorKind::Backend, "refused"))
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    session
        .finish_native_reader_error(Error::new(ErrorKind::Local, "native reader diagnostic"))
        .unwrap();
    assert_eq!(session.finish(None).unwrap_err().kind(), ErrorKind::Usage);
    let packets = drain(&session);
    assert_eq!(
        packets
            .iter()
            .filter(|packet| matches!(packet, RequestSessionResult::Row(_)))
            .count(),
        1
    );
    let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
        panic!("terminal")
    };
    assert_eq!(
        terminal.error.as_ref().unwrap().to_string(),
        "native reader diagnostic"
    );
    assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
}

#[test]
fn owned_feed_projection_preserves_false_filters_and_refuses_conflicts_before_intake() {
    let listener = Listener::answering(response).unwrap();
    let mut arguments = request(feed()).call.arguments().clone();
    arguments.options.threshold = Some(RequestThreshold::Cut(0.95));
    let options = RequestSessionFeedOptions {
        eager: true,
        all_filter_results: true,
        ..Default::default()
    };
    let session = engine(&listener)
        .request_session_with_feed_options(
            Request::new(RequestCall::Filter(arguments)),
            Surface::Rust,
            options,
        )
        .unwrap();
    push(&session, descriptor("first"));
    session.finish(None).unwrap();
    let packets = drain(&session);
    assert_eq!(
        packets
            .iter()
            .filter(|packet| matches!(
                packet,
                RequestSessionResult::Row(RequestSessionRow::Filter(_))
            ))
            .count(),
        1
    );
    for controls in [
        RequestSessionFeedOptions {
            all_filter_results: true,
            ..Default::default()
        },
        RequestSessionFeedOptions {
            image_inputs: true,
            ..Default::default()
        },
    ] {
        assert_eq!(
            engine(&listener)
                .request_session_with_feed_options(request(feed()), Surface::Rust, controls)
                .unwrap_err()
                .kind(),
            ErrorKind::Usage
        );
    }
    let reading = RecordReading::new(&["/body"], Some("/policy"), None).unwrap();
    let mut arguments = request(feed()).call.arguments().clone();
    arguments.options.field = Some(vec!["/body".into()]);
    assert_eq!(
        engine(&listener)
            .request_session_with_feed_options(
                Request::new(RequestCall::Decide(arguments)),
                Surface::Rust,
                RequestSessionFeedOptions {
                    record_reading: Some(reading),
                    ..Default::default()
                }
            )
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    assert_eq!(listener.count(), 1);
}

#[test]
fn owned_find_and_relate_keep_native_set_limits_without_engine_cap() {
    for (case, message) in [
        (8, "find takes 2 to 255 units"),
        (10, "source relate takes at most 255 source records"),
    ] {
        let request = owned_eager_cases().remove(case);
        let listener = Listener::answering(response).unwrap();
        let session = engine(&listener)
            .request_session_with_feed_options(
                request,
                Surface::Rust,
                RequestSessionFeedOptions {
                    eager: true,
                    ..Default::default()
                },
            )
            .unwrap();
        let until = Instant::now() + Duration::from_secs(5);
        let mut accepted = 0;
        let mut pending = owned_descriptor("Ada", false);
        loop {
            assert!(Instant::now() < until);
            match session.try_push(pending).unwrap() {
                RequestSessionPush::Accepted => {
                    accepted += 1;
                    assert!(accepted <= 257);
                    pending = owned_descriptor("Ada", false);
                }
                RequestSessionPush::Full(row) => {
                    pending = row;
                    std::thread::yield_now();
                }
                RequestSessionPush::Closed(_) => break,
            }
        }
        let packets = drain(&session);
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
            panic!("terminal")
        };
        assert_eq!(terminal.error.as_ref().unwrap().to_string(), message);
        assert!(terminal.facts.is_none());
        assert_eq!(listener.count(), 0);
    }
}

#[test]
fn owned_whole_set_preparation_error_wins_on_first_excess_record() {
    for case in [8, 10] {
        let request = owned_eager_cases().remove(case);
        let listener = Listener::answering(response).unwrap();
        let engine = Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .api_key("session-fixture")
            .unwrap()
            .no_cache()
            .max_requests(Some(2))
            .unwrap()
            .build()
            .unwrap();
        let records = (0..3).map(|at| RecordInput {
            original: QuestionInput::Text("Ada".into()),
            context: (at == 2).then(|| "policy".into()),
            options: None,
            seed_spans: None,
            examples: None,
        });
        let RequestQuestion::Definition { value } = &request.call.arguments().question else {
            unreachable!()
        };
        let expected = match value {
            RequestDefinition::Atomic(LoadedQuestion::Question(q)) => engine
                .try_find_records_complete_with(q, records.map(Ok), CallOptions::new())
                .unwrap_err(),
            RequestDefinition::Relate(q) => engine
                .try_relate_records_complete_with(q, records.map(Ok), CallOptions::new())
                .unwrap_err(),
            _ => unreachable!(),
        };
        let session = engine
            .request_session_with_feed_options(
                request,
                Surface::Rust,
                RequestSessionFeedOptions {
                    eager: true,
                    ..Default::default()
                },
            )
            .unwrap();
        push(&session, owned_descriptor("Ada", false));
        push(&session, owned_descriptor("Ada", false));
        let mut invalid = owned_descriptor("Ada", false);
        invalid.item.context = Some("policy".into());
        push(&session, invalid);
        session
            .finish_native_reader_error(Error::new(ErrorKind::Local, "suffix failure"))
            .unwrap();
        let packets = drain(&session);
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
            panic!("terminal")
        };
        let actual = terminal.error.as_ref().unwrap();
        assert_eq!(actual.to_string(), expected.to_string());
        assert_eq!(actual.stopped().at(), expected.stopped().at());
        assert_eq!(actual.stopped().at(), Some(3));
        assert!(terminal.facts.is_none());
        assert_eq!(listener.count(), 0);
    }
}
