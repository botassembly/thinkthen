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
fn session_framing_refuses_tables_and_line_projections_before_intake() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    for framing in [
        RequestFraming::Csv,
        RequestFraming::Tsv,
        RequestFraming::Lines,
    ] {
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
            if matches!(framing, RequestFraming::Lines) {
                "--field: a text line has no members, so --lines takes no pointer"
            } else {
                "native session table framing is not implemented"
            }
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
    assert_eq!(rows.len(), 1);
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
