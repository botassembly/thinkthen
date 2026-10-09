//! Owned sessions preserve caller responsiveness and actual settlement.
use conformance_backend::{Canned, Listener, Rendezvous};
use serde_json::{Value, json};
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant};
use thinkthen::*;

fn request(input: RequestInput) -> Request {
    Request::new(RequestCall::Decide(RequestArguments {
        question: RequestQuestion::Text {
            text: "Does it pass?".into(),
        },
        input,
        options: RequestOptions::default(),
    }))
}

fn feed() -> RequestInput {
    RequestInput::Feed {
        name: "records".into(),
        framing: RequestFraming::Document,
        reading: ReaderOptions::default(),
        images: vec![],
    }
}
#[allow(
    clippy::unwrap_used,
    reason = "isolated descriptor fixtures must decode before the behavior is exercised"
)]
fn descriptor(text: &str) -> RequestSessionDescriptor {
    RequestSessionDescriptor::from_json(
        &json!({"item":{"original":{"kind":"text","text":text}}}).to_string(),
    )
    .unwrap()
}
#[allow(
    clippy::unwrap_used,
    reason = "the fixture engine uses only its owned loopback listener and fake key"
)]
fn engine(listener: &Listener) -> Engine {
    Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("session-fixture")
        .unwrap()
        .no_cache()
        .max_retries(0)
        .build()
        .unwrap()
}
#[allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "malformed owned loopback request fixtures stop the behavior test"
)]
fn response(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap();
    let answers = request["questions"]
        .as_object()
        .unwrap()
        .keys()
        .map(|name| (name.clone(), json!({"type":"noul","noul":0.9})))
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
}
fn next(session: &RequestSession) -> RequestSessionRead {
    let until = Instant::now() + Duration::from_secs(5);
    loop {
        let packet = session.try_read();
        if !matches!(packet, RequestSessionRead::Pending) {
            return packet;
        }
        assert!(Instant::now() < until, "session did not settle");
        std::thread::yield_now();
    }
}
fn drain(session: &RequestSession) -> Vec<RequestSessionResult> {
    let mut packets = vec![];
    loop {
        match next(session) {
            RequestSessionRead::Result(packet) => packets.push(packet),
            RequestSessionRead::End => return packets,
            RequestSessionRead::Pending => unreachable!(),
        }
    }
}

#[test]
fn held_provider_bounds_intake_and_cancel_waits_for_actual_terminal_facts() {
    let release = Arc::new(Rendezvous::new(2));
    let held = Arc::clone(&release);
    let (sent, received) = mpsc::channel();
    let listener = Listener::answering_with_events(
        move |body| response(body).after_release(Arc::clone(&held)),
        sent,
    )
    .unwrap();
    let engine = engine(&listener);
    let session = engine.request_session(request(feed())).unwrap();
    assert!(matches!(
        session.try_push(descriptor("first")).unwrap(),
        RequestSessionPush::Accepted
    ));
    received.recv_timeout(Duration::from_secs(5)).unwrap();
    assert!(matches!(
        session.try_push(descriptor("second")).unwrap(),
        RequestSessionPush::Accepted
    ));
    // Native scheduling may own additional in-flight inputs; the session bounds
    // waiting transfers rather than changing those existing scheduler limits.
    let retry = (0..100)
        .find_map(|_| match session.try_push(descriptor("next")).unwrap() {
            RequestSessionPush::Full(value) => Some(value),
            RequestSessionPush::Accepted => None,
            other => panic!("feed closed before cancellation: {other:?}"),
        })
        .expect("held native work eventually backpressures the one input cell");
    session.finish(None).unwrap();
    session.finish(None).unwrap();
    assert_eq!(
        session
            .finish(Some(RequestReaderFailure::Io { location: None }))
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    let began = Instant::now();
    session.cancel();
    assert!(began.elapsed() < Duration::from_secs(1));
    assert!(matches!(
        session.try_push(retry).unwrap(),
        RequestSessionPush::Closed(_)
    ));
    assert!(matches!(session.try_read(), RequestSessionRead::Pending));
    assert!(release.wait());
    let packets = drain(&session);
    let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
        panic!("terminal comes last")
    };
    assert_eq!(
        terminal.error.as_ref().unwrap().kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
    assert_eq!(terminal.facts.as_ref().unwrap().input_tokens(), None);
    assert_eq!(
        packets
            .iter()
            .filter(|p| matches!(p, RequestSessionResult::Terminal(_)))
            .count(),
        1
    );
    assert!(matches!(session.try_read(), RequestSessionRead::End));
    assert_eq!(listener.count(), 1);
}

#[test]
fn free_returns_while_the_provider_is_held_and_the_engine_owner_is_gone() {
    let release = Arc::new(Rendezvous::new(2));
    let held = Arc::clone(&release);
    let (sent, received) = mpsc::channel();
    let listener = Listener::answering_with_events(
        move |body| response(body).after_release(Arc::clone(&held)),
        sent,
    )
    .unwrap();
    let engine = engine(&listener);
    let session = engine.request_session(request(feed())).unwrap();
    session.try_push(descriptor("first")).unwrap();
    received.recv_timeout(Duration::from_secs(5)).unwrap();
    drop(engine);
    let began = Instant::now();
    drop(session);
    assert!(began.elapsed() < Duration::from_secs(1));
    assert!(release.wait());
}

#[test]
fn reader_failure_follows_owned_completed_rows_and_observations() {
    let listener = Listener::answering(response).unwrap();
    let session = engine(&listener).request_session(request(feed())).unwrap();
    let mut item = descriptor("first");
    item.location =
        Some(SourceLocation::new("private-source-name".into(), Some(3), Some(4)).unwrap());
    session.try_push(item).unwrap();
    session
        .finish(Some(RequestReaderFailure::Io {
            location: Some(SourceLocation::new("secret-failure-path".into(), None, None).unwrap()),
        }))
        .unwrap();
    let packets = drain(&session);
    assert!(matches!(
        &packets[0],
        RequestSessionResult::Observation(OwnedRecordObservation::Question { .. })
    ));
    assert!(matches!(
        &packets[1],
        RequestSessionResult::Observation(OwnedRecordObservation::Row { .. })
    ));
    let RequestSessionResult::Row(RequestSessionRow::Decision(row)) = &packets[2] else {
        panic!("completed decision prefix")
    };
    let QuestionInput::Record(original) = row.original() else {
        panic!("located original")
    };
    assert_eq!(original.location().unwrap().first_line(), Some(3));
    let RequestSessionResult::Terminal(terminal) = &packets[3] else {
        panic!("joined terminal")
    };
    let error = terminal.error.as_ref().unwrap();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
    assert_eq!(terminal.facts.as_ref().unwrap().records(), 1);
    assert!(!format!("{error:?}").contains("secret-failure-path"));
    assert!(
        !String::from_utf8(listener.requests()[0].body.clone())
            .unwrap()
            .contains("private-source-name")
    );
}

#[test]
fn full_output_preserves_a_completed_prefix_when_cancelled() {
    let (sent, received) = mpsc::channel();
    let listener = Listener::answering(move |body| response(body).notifying(sent.clone())).unwrap();
    let session = engine(&listener).request_session(request(feed())).unwrap();
    session.try_push(descriptor("first")).unwrap();
    session.finish(None).unwrap();
    received.recv_timeout(Duration::from_secs(5)).unwrap();
    // Reading the first event lets the worker publish the row observation while
    // its complete row then waits behind that occupied output cell.
    assert!(matches!(
        next(&session),
        RequestSessionRead::Result(RequestSessionResult::Observation(
            OwnedRecordObservation::Question { .. }
        ))
    ));
    session.cancel();
    let packets = drain(&session);
    assert!(matches!(
        &packets[0],
        RequestSessionResult::Observation(OwnedRecordObservation::Row { .. })
    ));
    assert!(matches!(
        &packets[1],
        RequestSessionResult::Row(RequestSessionRow::Decision(_))
    ));
    let RequestSessionResult::Terminal(terminal) = &packets[2] else {
        panic!("terminal follows completed row")
    };
    assert_eq!(terminal.facts.as_ref().unwrap().records(), 1);
}

#[test]
fn descriptor_decoder_refuses_null_duplicates_and_invalid_locations() {
    for text in [
        r#"{"item":null}"#,
        r#"{"item":{},"location":null}"#,
        r#"{"item":{},"location":{"file":"x","first_line":null}}"#,
        r#"{"item":{},"location":{"file":"x","first_line":1}}"#,
        r#"{"item":{},"location":{"file":"x","first_line":2,"last_line":1}}"#,
        r#"{"item":{},"location":{"file":"x","file":"y"}}"#,
        r#"{"item":{},"item":{}}"#,
        r#"{"item":{},"extra":true}"#,
    ] {
        assert_eq!(
            RequestSessionDescriptor::from_json(text)
                .unwrap_err()
                .kind(),
            ErrorKind::Usage,
            "{text}"
        );
    }
}

#[test]
fn empty_success_and_idle_producer_deadline_keep_actual_native_facts() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let empty = engine.request_session(request(feed())).unwrap();
    empty.finish(None).unwrap();
    let packets = drain(&empty);
    assert_eq!(packets.len(), 1);
    let RequestSessionResult::Terminal(terminal) = &packets[0] else {
        panic!("empty success terminal")
    };
    assert!(terminal.error.is_none());
    assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 0);
    assert_eq!(terminal.facts.as_ref().unwrap().records(), 0);
    let mut arguments = request(feed()).call.arguments().clone();
    arguments.options.deadline_ms = Some(20);
    let deadline = engine
        .request_session(Request::new(RequestCall::Decide(arguments)))
        .unwrap();
    let packets = drain(&deadline);
    let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
        panic!("deadline terminal")
    };
    assert_eq!(terminal.error.as_ref().unwrap().kind(), ErrorKind::Deadline);
    assert_eq!(
        terminal.error.as_ref().unwrap().to_string(),
        "the deadline of 20 ms passed before the call answered"
    );
    assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 0);
    assert!(matches!(
        deadline.try_push(descriptor("too late")).unwrap(),
        RequestSessionPush::Closed(_)
    ));
    assert_eq!(listener.count(), 0);
}

#[test]
fn image_descriptors_keep_physical_provenance_out_of_model_evidence() {
    let listener = Listener::serving(vec![Canned::ok(include_str!(
        "../../../specification/fixtures/images/liquid-decide-reply.json"
    ))])
    .unwrap();
    let engine = Engine::builder()
        .backend("liquid")
        .unwrap()
        .base_url(listener.base())
        .unwrap()
        .model("d1")
        .unwrap()
        .api_key("session-image-fixture")
        .unwrap()
        .no_cache()
        .max_retries(0)
        .build()
        .unwrap();
    let session = engine.request_session(request(feed())).unwrap();
    let source = SourceLocation::new("private-image-source.png".into(), None, None).unwrap();
    let descriptor = RequestSessionDescriptor {
        item: RequestItem {
            original: None,
            context: None,
            options: None,
            examples: None,
            seed_spans: None,
            images: vec![RequestImage::Bytes {
                media: ImageMedia::Png,
                bytes: include_bytes!("../../../specification/fixtures/images/red.png").to_vec(),
            }],
        },
        location: Some(source.clone()),
    };
    session.try_push(descriptor).unwrap();
    session.finish(None).unwrap();
    let packets = drain(&session);
    let row = packets
        .iter()
        .find_map(|packet| match packet {
            RequestSessionResult::Row(RequestSessionRow::Decision(row)) => Some(row),
            _ => None,
        })
        .unwrap();
    let QuestionInput::Images(images) = row.original() else {
        panic!("image original")
    };
    assert_eq!(images.location(), Some(&source));
    assert!(
        !String::from_utf8(listener.requests()[0].body.clone())
            .unwrap()
            .contains(source.file())
    );
}

#[test]
fn whole_set_requests_publish_one_aggregate_then_truthful_terminal() {
    let listener = Listener::answering(response).unwrap();
    let arguments = RequestArguments {
        question: RequestQuestion::Definition {
            value: Question::rank("Does it pass?").unwrap().into(),
        },
        input: RequestInput::Records {
            items: vec![descriptor("first").item, descriptor("second").item],
        },
        options: RequestOptions::default(),
    };
    let session = engine(&listener)
        .request_session(Request::new(RequestCall::Rank(arguments)))
        .unwrap();
    let packets = drain(&session);
    let aggregates = packets
        .iter()
        .filter_map(|packet| match packet {
            RequestSessionResult::Aggregate(value) => Some(value),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(aggregates.len(), 1);
    let RequestValue::Ranked(rows) = aggregates[0] else {
        panic!("rank aggregate")
    };
    assert_eq!(rows.len(), 2);
    assert!(
        !packets
            .iter()
            .any(|packet| matches!(packet, RequestSessionResult::Row(_)))
    );
    let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
        panic!("final terminal")
    };
    assert!(terminal.error.is_none());
    assert_eq!(terminal.facts.as_ref().unwrap().records(), 2);
}

#[test]
fn filter_observes_rejected_occurrences_and_selects_only_first_passing_file() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    for threshold in [0.5, 0.95] {
        let mut arguments = request(feed()).call.arguments().clone();
        arguments.options.threshold = Some(RequestThreshold::Cut(threshold));
        arguments.options.files_only = true;
        assert_eq!(
            Request::new(RequestCall::Filter(arguments.clone()))
                .admit()
                .unwrap_err()
                .detail()
                .message(),
            "files_only requires filter source input"
        );
        let session = engine
            .request_session(Request::new(RequestCall::Filter(arguments)))
            .unwrap();
        let until = Instant::now() + Duration::from_secs(5);
        for text in ["first", "second"] {
            let mut pending = descriptor(text);
            pending.location = Some(SourceLocation::new("same-file".into(), None, None).unwrap());
            loop {
                match session.try_push(pending).unwrap() {
                    RequestSessionPush::Accepted => break,
                    RequestSessionPush::Full(value) => pending = value,
                    RequestSessionPush::Closed(_) => panic!("valid filter feed closed"),
                }
                assert!(Instant::now() < until);
                std::thread::yield_now();
            }
        }
        session.finish(None).unwrap();
        let packets = drain(&session);
        assert_eq!(
            packets
                .iter()
                .filter(|p| matches!(
                    p,
                    RequestSessionResult::Observation(OwnedRecordObservation::Row { .. })
                ))
                .count(),
            2
        );
        let rows = packets
            .iter()
            .filter_map(|p| match p {
                RequestSessionResult::Row(RequestSessionRow::Filter(row)) => Some(row),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), usize::from(threshold < 0.9));
        if let Some(row) = rows.first() {
            let QuestionInput::Record(original) = row.original() else {
                panic!("located filter original")
            };
            assert_eq!(original.original().literal(), Some("first"));
        }
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
            panic!("filter terminal")
        };
        assert_eq!(terminal.facts.as_ref().unwrap().records(), 2);
    }
}

#[test]
fn file_selection_refuses_locationless_descriptors_before_sending_them() {
    let (sent, received) = mpsc::channel();
    let listener = Listener::answering_with_events(response, sent).unwrap();
    let mut arguments = request(feed()).call.arguments().clone();
    arguments.options.files_only = true;
    let session = engine(&listener)
        .request_session(Request::new(RequestCall::Filter(arguments)))
        .unwrap();
    let mut first = descriptor("first");
    first.location = Some(SourceLocation::new("first-source".into(), None, None).unwrap());
    session.try_push(first).unwrap();
    received.recv_timeout(Duration::from_secs(5)).unwrap();
    session.try_push(descriptor("unlocated")).unwrap();
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
    let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
        panic!("file selection failure")
    };
    assert_eq!(terminal.error.as_ref().unwrap().kind(), ErrorKind::Usage);
    assert_eq!(
        terminal.error.as_ref().unwrap().detail().message(),
        "file selection requires a source location on every session descriptor"
    );
    assert_eq!(terminal.facts.as_ref().unwrap().records(), 1);
    assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
    assert_eq!(listener.count(), 1);
}

#[test]
fn inline_provider_failure_preserves_completed_rows_and_final_facts() {
    let calls = std::sync::atomic::AtomicUsize::new(0);
    let listener = Listener::answering(move |body| {
        if calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst) == 0 {
            response(body)
        } else {
            Canned::status(401, "refused")
        }
    })
    .unwrap();
    let mut arguments = request(RequestInput::Records {
        items: vec![descriptor("first").item, descriptor("second").item],
    })
    .call
    .arguments()
    .clone();
    arguments.options.batch = Some(RequestBatch::Count(1));
    let session = engine(&listener)
        .request_session(Request::new(RequestCall::Decide(arguments)))
        .unwrap();
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
        panic!("completed inline original")
    };
    assert_eq!(original.selected().to_json().unwrap(), "\"first\"");
    let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
        panic!("joined provider failure")
    };
    assert!(terminal.error.is_some());
    assert_eq!(terminal.facts.as_ref().unwrap().records(), 1);
    assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 2);
    assert_eq!(listener.count(), 2);
}

#[test]
fn oversized_inline_session_refuses_the_whole_set_without_sending() {
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
        .max_requests(Some(1))
        .unwrap()
        .build()
        .unwrap();
    let session = engine
        .request_session(request(RequestInput::Records {
            items: vec![descriptor("first").item, descriptor("second").item],
        }))
        .unwrap();
    let packets = drain(&session);
    assert_eq!(listener.count(), 0);
    assert_eq!(packets.len(), 1);
    let RequestSessionResult::Terminal(terminal) = &packets[0] else {
        panic!("whole-set admission refusal")
    };
    assert_eq!(terminal.error.as_ref().unwrap().kind(), ErrorKind::Usage);
}

#[test]
fn inline_sessions_refuse_feed_controls_and_own_their_engine() {
    let engine = Engine::builder().no_cache().build().unwrap();
    let session = engine
        .request_session(request(RequestInput::Text {
            text: "content".into(),
            images: vec![],
        }))
        .unwrap();
    session.finish(None).unwrap();
    session.finish(None).unwrap();
    assert_eq!(
        session
            .finish(Some(RequestReaderFailure::Utf8 { location: None }))
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    drop(engine);
    session.cancel();
    drop(session);
}
