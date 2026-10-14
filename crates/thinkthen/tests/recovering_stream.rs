//! Opt-in recovering callers retain lazy readers, bounded intake and joined stops.
use conformance_backend::{Canned, Listener, Rendezvous};
use serde_json::{Value, json};
use std::{
    cell::{Cell, RefCell},
    sync::{Arc, mpsc},
    time::{Duration, Instant},
};
use thinkthen::{
    BatchSetting, CallOptions, CancelToken, ErrorKind, Question, QuestionInput, ReaderOptions,
    Request, RequestArguments, RequestEnvironment, RequestFeed, RequestFraming, RequestFunction,
    RequestInput, RequestOptions, RequestQuestion, RequestReaderFailure, RequestRecoveredRow,
    RequestRecoveringRead, RequestRecoveringResult, RequestRecoveringSession,
    RequestSessionDescriptor, RequestSessionFeedOptions, RequestSessionPushStatus,
    RequestSessionReady, RequestSessionWaitInterest, RequestValue, Surface,
};
#[path = "support/api.rs"]
mod api;

fn request() -> Request {
    Request::for_function(
        RequestFunction::Decide,
        RequestArguments {
            question: RequestQuestion::Definition {
                value: Question::decide("Does it pass?").unwrap().cut().into(),
            },
            input: RequestInput::Feed {
                name: "records".into(),
                framing: RequestFraming::Document,
                reading: ReaderOptions::default(),
                images: vec![],
            },
            options: RequestOptions::default(),
        },
    )
}
fn descriptor(text: &str) -> RequestSessionDescriptor {
    RequestSessionDescriptor::from_json(
        &json!({"item":{"original":{"kind":"text","text":text}}}).to_string(),
    )
    .unwrap()
}
fn response(body: &[u8]) -> Canned {
    let body: Value = serde_json::from_slice(body).unwrap();
    let answers = body["questions"]
        .as_object()
        .unwrap()
        .keys()
        .map(|name| (name.clone(), json!({"type":"noul","noul":0.99})))
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
}
fn next(session: &RequestRecoveringSession) -> RequestRecoveringRead {
    let end = Instant::now() + Duration::from_secs(5);
    loop {
        let value = session.try_read();
        if !matches!(value, RequestRecoveringRead::Pending) {
            return value;
        }
        assert!(Instant::now() < end, "recovering session did not settle");
        session.wait(
            RequestSessionWaitInterest::Settlement,
            end.saturating_duration_since(Instant::now()),
        );
    }
}
fn drain(session: &RequestRecoveringSession) -> Vec<RequestRecoveringResult> {
    let mut packets = Vec::new();
    loop {
        match next(session) {
            RequestRecoveringRead::Result(packet) => packets.push(packet),
            RequestRecoveringRead::End => return packets,
            RequestRecoveringRead::Pending => unreachable!(),
        }
    }
}
fn push(session: &RequestRecoveringSession, text: &str) {
    let end = Instant::now() + Duration::from_secs(5);
    let encoded = json!({"item":{"original":{"kind":"text","text":text}}}).to_string();
    loop {
        match session.try_push_json(&encoded).unwrap() {
            RequestSessionPushStatus::Accepted => return,
            RequestSessionPushStatus::Closed => panic!("feed closed before admission"),
            RequestSessionPushStatus::Full => {
                assert!(Instant::now() < end, "feed did not admit descriptor");
                session.wait(
                    RequestSessionWaitInterest::Input,
                    end.saturating_duration_since(Instant::now()),
                );
            }
        }
    }
}

#[test]
fn borrowed_lazy_reader_returns_prefix_then_reader_failure_without_extra_pulls() {
    let listener = Listener::answering(response).unwrap();
    let engine = api::loopback(listener.base(), "sk-lazy-recovery");
    let request = request().admit().unwrap();
    let pulls = Cell::new(0);
    let records = std::iter::from_fn(|| {
        let at = pulls.get();
        pulls.set(at + 1);
        Some(if at == 2 {
            RequestSessionDescriptor::from_json("{}").map(|_| unreachable!())
        } else {
            Ok(api::record(QuestionInput::Text(format!("row {at}"))))
        })
    });
    let seen = RefCell::new(Vec::new());
    let sink = |row: RequestRecoveredRow| {
        assert_eq!(
            pulls.get(),
            row.ordinal() + 1,
            "batch one must yield before another pull"
        );
        seen.borrow_mut().push(row);
    };
    let error = engine
        .execute_request_recovering_sink(
            &request,
            RequestEnvironment {
                controls: api::dynamic_one(),
                feed: Some(RequestFeed::from_records("records", records)),
            },
            &sink,
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(pulls.get(), 3);
    assert_eq!(
        seen.borrow()
            .iter()
            .map(RequestRecoveredRow::ordinal)
            .collect::<Vec<_>>(),
        [0, 1]
    );
    assert_eq!(error.facts().unwrap().records(), 2);
    assert_eq!(error.facts().unwrap().requests_sent(), 2);
    assert_eq!(listener.count(), 2);
    assert!(!format!("{error:?}").contains("sk-lazy-recovery"));
}

#[test]
fn cancellation_in_sink_joins_completed_prefix_without_advancing_reader() {
    let listener = Listener::answering(response).unwrap();
    let engine = api::loopback(listener.base(), "sk-cancel-recovery");
    let request = request().admit().unwrap();
    let cancel = CancelToken::new();
    let pulls = Cell::new(0);
    let records = std::iter::from_fn(|| {
        pulls.set(pulls.get() + 1);
        Some(Ok(api::record(QuestionInput::Text("content".into()))))
    });
    let seen = Cell::new(0);
    let sink = |row: RequestRecoveredRow| {
        assert!(row.outcome().is_ok());
        seen.set(seen.get() + 1);
        cancel.cancel();
    };
    let error = engine
        .execute_request_recovering_sink(
            &request,
            RequestEnvironment {
                controls: api::dynamic_one().cancel(&cancel),
                feed: Some(RequestFeed::from_records("records", records)),
            },
            &sink,
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Cancelled);
    assert_eq!((seen.get(), pulls.get()), (1, 1));
    assert_eq!(error.facts().unwrap().requests_sent(), 1);
    assert_eq!(listener.count(), 1);
}

#[test]
fn owned_session_recovers_packed_bad_answer_and_retains_actual_observations() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"wrong"},"q2":{"type":"noul","noul":0.99}}}"#)).unwrap();
    let engine = api::fixed(listener.base(), "sk-packed-recovery")
        .no_cache()
        .max_retries(0)
        .batch(BatchSetting::Records(
            std::num::NonZeroUsize::new(2).unwrap(),
        ))
        .build()
        .unwrap();
    let session = engine
        .request_recovering_session_with_feed_options(
            request(),
            Surface::Rust,
            RequestSessionFeedOptions::default(),
        )
        .unwrap();
    push(&session, "first");
    push(&session, "second");
    session.finish(None).unwrap();
    let packets = drain(&session);
    let rows = packets
        .iter()
        .filter_map(|packet| match packet {
            RequestRecoveringResult::Row(row) => Some(row),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        rows.iter().map(|row| row.ordinal()).collect::<Vec<_>>(),
        [0, 1]
    );
    assert_eq!(rows[0].outcome().unwrap_err().kind(), ErrorKind::Backend);
    assert!(rows[0].cause().is_some());
    let RequestValue::Decisions(values) = rows[1].outcome().unwrap() else {
        panic!("decision row");
    };
    assert_eq!(values[0].result().value(), thinkthen::Answer::Yes);
    assert_eq!(values[0].result().probability(), Some(0.99));
    let RequestRecoveringResult::Terminal(terminal) = packets.last().unwrap() else {
        panic!("joined terminal");
    };
    assert!(terminal.error.is_none());
    assert_eq!(terminal.facts.as_ref().unwrap().records(), 2);
    assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
    assert_eq!(listener.count(), 1);
    assert_eq!(listener.questions(), 2);
    assert!(
        packets
            .iter()
            .any(|packet| matches!(packet, RequestRecoveringResult::Observation { .. }))
    );
    assert!(!format!("{packets:?}").contains("sk-packed-recovery"));
}

#[test]
fn producer_failure_is_terminal_after_accepted_descriptor_with_final_counts() {
    let listener = Listener::answering(response).unwrap();
    let engine = api::loopback(listener.base(), "sk-producer-recovery");
    let session = engine
        .start_request_recovering_session(request(), Surface::Rust)
        .unwrap();
    push(&session, "accepted");
    session
        .finish(Some(RequestReaderFailure::Producer))
        .unwrap();
    let packets = drain(&session);
    let rows = packets
        .iter()
        .filter(|packet| matches!(packet, RequestRecoveringResult::Row(_)))
        .count();
    assert_eq!(rows, 1);
    let RequestRecoveringResult::Terminal(terminal) = packets.last().unwrap() else {
        panic!("terminal reader failure");
    };
    let error = terminal.error.as_ref().unwrap();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert_eq!(
        error.detail().message(),
        "session producer raised an exception"
    );
    assert_eq!(terminal.facts.as_ref().unwrap().records(), 1);
    assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
    assert_eq!(listener.count(), 1);
}

#[test]
fn full_and_cancelled_intake_do_not_convert_and_drop_does_not_join_held_provider() {
    let release = Arc::new(Rendezvous::new(2));
    let backend_release = Arc::clone(&release);
    let (entered, arrived) = mpsc::channel();
    let listener = Listener::answering(move |body| {
        entered.send(()).unwrap();
        response(body).after_release(Arc::clone(&backend_release))
    })
    .unwrap();
    let engine = api::fixed(listener.base(), "sk-held-recovery")
        .no_cache()
        .max_retries(0)
        .batch(BatchSetting::Records(
            std::num::NonZeroUsize::new(1).unwrap(),
        ))
        .build()
        .unwrap();
    let session = engine
        .start_request_recovering_session(request(), Surface::Rust)
        .unwrap();
    push(&session, "first");
    arrived.recv_timeout(Duration::from_secs(5)).unwrap();
    push(&session, "queued");
    let conversions = Cell::new(0);
    let convert = || -> Result<RequestSessionDescriptor, thinkthen::Error> {
        conversions.set(conversions.get() + 1);
        Ok(descriptor("unaccepted"))
    };
    assert_eq!(
        session.try_push_with(convert).unwrap(),
        RequestSessionPushStatus::Full
    );
    assert_eq!(conversions.get(), 0);
    session.cancel();
    assert_eq!(
        session.wait(RequestSessionWaitInterest::Result, Duration::ZERO),
        RequestSessionReady::Cancelled
    );
    assert_eq!(
        session.try_push_with(convert).unwrap(),
        RequestSessionPushStatus::Closed
    );
    assert_eq!(conversions.get(), 0);
    let (dropped, drop_done) = mpsc::channel();
    std::thread::spawn(move || {
        drop(session);
        dropped.send(()).unwrap();
    });
    drop_done.recv_timeout(Duration::from_secs(1)).unwrap();
    assert!(release.wait());
    assert_eq!(listener.count(), 1);
}
