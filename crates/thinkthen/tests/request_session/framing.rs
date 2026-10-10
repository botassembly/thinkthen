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
    for framing in [RequestFraming::Lines] {
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

fn table_request(framing: RequestFraming) -> Request {
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
            if matches!(framing, RequestFraming::Tsv) {
                if let Some(text) = item["original"]["text"].as_str() {
                    item["original"]["text"] = json!(text.replace(',', "\t"));
                }
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
            .request_session(Request::new(RequestCall::Decide(arguments)))
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
