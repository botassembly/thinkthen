//! Native logical readiness keeps an open producer from holding completed output.
use super::*;

fn start(
    listener: &Listener,
    framing: RequestFraming,
    pause: Option<Duration>,
) -> Result<RequestSession, Error> {
    let mut request = request(RequestInput::Feed {
        name: "ready".to_owned(),
        framing,
        reading: ReaderOptions::default(),
        images: Vec::new(),
    });
    let mut arguments = request.call.arguments().clone();
    arguments.options.batch = Some(RequestBatch::Named("max".to_owned()));
    request = Request::for_function(RequestFunction::Decide, arguments);
    engine(listener).request_session_with_feed_options(
        request,
        Surface::Rust,
        RequestSessionFeedOptions {
            input_pause: pause,
            ..Default::default()
        },
    )
}
fn push(
    session: &RequestSession,
    text: &str,
    packets: &mut Vec<RequestSessionResult>,
) -> Result<(), Error> {
    let mut item = descriptor(text);
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        assert!(
            Instant::now() < deadline,
            "input progresses without losing its readiness wake"
        );
        match session.try_push(item)? {
            RequestSessionPush::Accepted => return Ok(()),
            RequestSessionPush::Full(value) => item = value,
            RequestSessionPush::Closed(_) => {
                return Err(Error::new(
                    ErrorKind::Defect,
                    "intake closed before its descriptor",
                ));
            }
        }
        if let RequestSessionRead::Result(packet) = session.try_read() {
            packets.push(packet);
        }
        let ready = session.wait(RequestSessionWaitInterest::Input, Duration::from_millis(50));
        assert_ne!(ready, RequestSessionReady::End);
    }
}
fn first_row(
    session: &RequestSession,
    packets: &mut Vec<RequestSessionResult>,
) -> Result<(), Error> {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !packets
        .iter()
        .any(|packet| matches!(packet, RequestSessionResult::Row(_)))
    {
        assert!(
            Instant::now() < deadline,
            "first completed row must precede producer EOF"
        );
        match session.try_read() {
            RequestSessionRead::Result(packet) => packets.push(packet),
            RequestSessionRead::Pending => {
                let ready = session.wait(
                    RequestSessionWaitInterest::Result,
                    Duration::from_millis(50),
                );
                assert_ne!(ready, RequestSessionReady::End);
            }
            RequestSessionRead::End => {
                return Err(Error::new(ErrorKind::Defect, "producer remains open"));
            }
        }
    }
    Ok(())
}

#[test]
fn first_rows_precede_eof_after_headers_and_multiple_blank_descriptors() {
    for pause in [None, Some(Duration::ZERO)] {
        for (framing, inputs) in [
            (RequestFraming::Lines, vec!["\n", " \n", "first\n"]),
            (
                RequestFraming::Jsonl,
                vec!["\n", " \n", "{\"note\":\"first\"}\n"],
            ),
            (RequestFraming::Csv, vec!["note\n", "\n", "\n", "first\n"]),
            (RequestFraming::Tsv, vec!["note\n", "\n", "\n", "first\n"]),
        ] {
            let listener = Listener::answering(response).unwrap();
            let session = start(&listener, framing, pause).unwrap();
            let mut packets = Vec::new();
            for input in inputs {
                push(&session, input, &mut packets).unwrap();
            }
            first_row(&session, &mut packets).unwrap();
            assert_eq!(listener.requests().len(), 1);
            session.finish(None).unwrap();
            packets.extend(drain(&session));
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
            assert!(terminal.error.is_none());
            assert_eq!(terminal.facts.as_ref().unwrap().records(), 1);
            assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
        }
    }
}

#[test]
fn header_blank_then_eof_or_cancel_settles_without_a_wire_question() {
    for cancel in [false, true] {
        let listener = Listener::answering(response).unwrap();
        let session = start(&listener, RequestFraming::Csv, None).unwrap();
        let mut packets = Vec::new();
        push(&session, "note\n", &mut packets).unwrap();
        push(&session, "\n", &mut packets).unwrap();
        if cancel {
            session.cancel();
        } else {
            session.finish(None).unwrap();
        }
        packets.extend(drain(&session));
        assert!(
            !packets
                .iter()
                .any(|packet| matches!(packet, RequestSessionResult::Row(_)))
        );
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
            panic!("terminal")
        };
        assert_eq!(
            terminal.error.as_ref().map(Error::kind),
            cancel.then_some(ErrorKind::Cancelled)
        );
        assert_eq!(listener.count(), 0);
    }
}

#[test]
fn malformed_logical_input_stays_after_the_completed_prefix() {
    let listener = Listener::answering(response).unwrap();
    let session = start(&listener, RequestFraming::Jsonl, None).unwrap();
    let mut packets = Vec::new();
    push(&session, "{\"note\":\"first\"}\n", &mut packets).unwrap();
    first_row(&session, &mut packets).unwrap();
    push(&session, "{\"private-marker\":\n", &mut packets).unwrap();
    session.finish(None).unwrap();
    packets.extend(drain(&session));
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
    assert!(error.diagnostic().is_some());
    assert!(!format!("{error:?} {error}").contains("private-marker"));
    assert_eq!(terminal.facts.as_ref().unwrap().records(), 1);
    assert_eq!(listener.count(), 1);
}

#[test]
fn recovering_sessions_publish_a_completed_row_while_the_feed_stays_open() {
    let listener = Listener::answering(response).unwrap();
    let session = engine(&listener)
        .request_recovering_session_with_feed_options(
            request(feed()),
            Surface::Rust,
            RequestSessionFeedOptions {
                input_pause: Some(Duration::ZERO),
                ..Default::default()
            },
        )
        .unwrap();
    assert!(matches!(
        session.try_push(descriptor("first")).unwrap(),
        RequestSessionPush::Accepted
    ));
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut rows = 0;
    let mut finished_input = false;
    let mut terminal = None;
    loop {
        assert!(
            Instant::now() < deadline,
            "recovering input readiness must release its first row"
        );
        match session.try_read() {
            RequestRecoveringRead::Result(RequestRecoveringResult::Row(row)) => {
                assert!(!finished_input, "the first native row precedes feed EOF");
                assert_eq!(row.ordinal(), 0);
                assert!(row.outcome().is_ok());
                rows += 1;
                session.finish(None).unwrap();
                finished_input = true;
            }
            RequestRecoveringRead::Result(RequestRecoveringResult::Terminal(value)) => {
                terminal = Some(value)
            }
            RequestRecoveringRead::Result(RequestRecoveringResult::Observation { .. }) => {}
            RequestRecoveringRead::Pending => {
                let ready = session.wait(
                    RequestSessionWaitInterest::Result,
                    Duration::from_millis(50),
                );
                assert_ne!(ready, RequestSessionReady::End);
            }
            RequestRecoveringRead::End => break,
        }
    }
    assert_eq!(rows, 1);
    let terminal = terminal.unwrap();
    assert!(terminal.error.is_none());
    assert_eq!(terminal.facts.as_ref().unwrap().records(), 1);
    assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
    assert_eq!(listener.count(), 1);
}

#[test]
fn an_input_pause_is_a_feed_control_and_refuses_other_shapes_before_work() {
    let listener = Listener::answering(response).unwrap();
    let request = request(RequestInput::Source {
        source: RequestSource {
            paths: vec!["absent-pause-source".into()],
            framing: None,
            reading: ReaderOptions::default(),
            media: ReaderMedia::Text,
        },
    });
    let error = engine(&listener)
        .request_session_with_feed_options(
            request,
            Surface::Rust,
            RequestSessionFeedOptions {
                input_pause: Some(Duration::ZERO),
                ..Default::default()
            },
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 0);
}
