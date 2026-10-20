use super::*;

fn arguments(framing: RequestFraming) -> RequestArguments {
    let mut args = request(feed()).call.arguments().clone();
    let RequestInput::Feed { framing: value, .. } = &mut args.input else { unreachable!() };
    *value = framing;
    args.options.batch = Some(RequestBatch::Count(1));
    args
}

#[test]
fn pull_reader_failures_match_session_kinds_messages_positions_and_prefixes() {
    for failure in [
        RequestReaderFailure::Producer,
        RequestReaderFailure::Io { location: Some(SourceLocation::new("private-input".into(), Some(9), Some(9)).unwrap()) },
        RequestReaderFailure::Utf8 { location: None },
        RequestReaderFailure::InvalidInput { location: None },
    ] {
        let listener = Listener::answering(response).unwrap();
        let engine = engine(&listener);
        let args = arguments(RequestFraming::Document);
        let session = engine.request_session(Request::named("decide", args.clone()).unwrap()).unwrap();
        session.try_push(descriptor("first")).unwrap();
        session.finish(Some(failure.clone())).unwrap();
        let packets = drain(&session);
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else { panic!("terminal") };
        let expected = terminal.error.as_ref().unwrap();
        let rows = vec![Ok(descriptor("first")), Err(failure)];
        let error = engine.call("decide", args, RequestEnvironment {
            controls: CallOptions::new(),
            feed: Some(RequestFeed::from_session_descriptors("records", rows.into_iter())),
        }).unwrap_err();
        assert_eq!(error.kind(), expected.kind());
        assert_eq!(error.detail().message(), expected.detail().message());
        assert_eq!(error.record_position(), expected.record_position());
        assert_eq!(error.record_position(), Some(1));
        let Some(RequestValue::Decisions(rows)) = error.completed() else { panic!("prefix") };
        assert_eq!(rows.len(), 1);
        assert_eq!(listener.count(), 2);
    }
}

#[test]
fn pull_framing_skips_headers_and_blanks_and_retains_physical_locations() {
    for (framing, texts) in [
        (RequestFraming::Lines, vec![" \r\n", "first\r\n"]),
        (RequestFraming::Jsonl, vec![" \n", "\"first\"\n"]),
        (RequestFraming::Csv, vec!["body\n", "\n", "first\n"]),
        (RequestFraming::Tsv, vec!["body\n", "\n", "first\n"]),
    ] {
        let listener = Listener::answering(response).unwrap();
        let mut args = arguments(framing);
        if matches!(framing, RequestFraming::Csv | RequestFraming::Tsv) {
            args.options.field = Some(vec!["/body".into()]);
        }
        let location = SourceLocation::new("private-input".into(), Some(17), Some(18)).unwrap();
        let rows = texts.into_iter().map(|text| {
            let mut value = descriptor(text);
            value.location = Some(location.clone());
            Ok(value)
        });
        let call = engine(&listener).call("decide", args, RequestEnvironment {
            controls: CallOptions::new(),
            feed: Some(RequestFeed::from_session_descriptors("records", rows)),
        }).unwrap();
        let RequestValue::Decisions(rows) = call.value() else { panic!("rows") };
        assert_eq!(rows.len(), 1);
        let QuestionInput::Record(original) = rows[0].original() else { panic!("original") };
        assert_eq!(original.location(), Some(&location));
        assert_eq!(call.facts().records(), 1);
        assert_eq!(listener.count(), 1);
    }
}

#[test]
fn pull_header_and_stop_admission_do_not_advance_the_producer() {
    for (wrong_name, invalid_header, cancelled) in [(true, false, false), (false, true, false), (false, false, true)] {
        let listener = Listener::answering(response).unwrap();
        let count = std::cell::Cell::new(0);
        let token = CancelToken::new();
        if cancelled { token.cancel(); }
        let mut args = arguments(RequestFraming::Lines);
        if invalid_header { args.options.field = Some(vec!["/body".into()]); }
        let rows = std::iter::from_fn(|| {
            count.set(count.get() + 1);
            Some(Ok(descriptor("first")))
        });
        let error = engine(&listener).call("decide", args, RequestEnvironment {
            controls: CallOptions::new().cancel(&token),
            feed: Some(RequestFeed::from_session_descriptors(if wrong_name { "wrong" } else { "records" }, rows)),
        }).unwrap_err();
        assert_eq!(error.kind(), if cancelled { ErrorKind::Cancelled } else { ErrorKind::Usage });
        assert_eq!(count.get(), 0);
        assert_eq!(listener.count(), 0);
    }
}

#[test]
fn pull_table_eof_uses_the_shared_decoder_refusal() {
    let listener = Listener::answering(response).unwrap();
    let error = engine(&listener).call("decide", arguments(RequestFraming::Csv), RequestEnvironment {
        controls: CallOptions::new(),
        feed: Some(RequestFeed::from_session_descriptors("records", std::iter::empty())),
    }).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(error.detail().message().contains("header"));
    assert_eq!(error.record_position(), Some(0));
    assert_eq!(listener.count(), 0);
}
