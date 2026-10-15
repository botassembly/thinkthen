//! Opt-in recovering callers retain lazy readers, bounded intake and joined stops.
use conformance_backend::{Canned, Listener, Rendezvous};
use serde_json::{Value, json};
use std::{
    cell::{Cell, RefCell},
    sync::{Arc, mpsc},
    time::{Duration, Instant},
};
use thinkthen::{
    BatchSetting, CancelToken, ErrorKind, Question, QuestionInput, ReaderOptions, Request,
    RequestArguments, RequestEnvironment, RequestFeed, RequestFraming, RequestFunction,
    RequestInput, RequestOptions, RequestQuestion, RequestReaderFailure, RequestRecoveredRow,
    RequestRecoveringRead, RequestRecoveringResult, RequestRecoveringSession,
    RequestSessionDescriptor, RequestSessionFeedOptions, RequestSessionPushStatus,
    RequestSessionReady, RequestSessionWaitInterest, RequestValue, Surface,
};
#[path = "support/api.rs"]
mod api;

#[expect(
    clippy::unwrap_used,
    reason = "the isolated fixture question must admit before the behavior runs"
)]
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
#[expect(
    clippy::unwrap_used,
    reason = "the closed fixture descriptor must decode before ownership is exercised"
)]
fn descriptor(text: &str) -> RequestSessionDescriptor {
    RequestSessionDescriptor::from_json(
        &json!({"item":{"original":{"kind":"text","text":text}}}).to_string(),
    )
    .unwrap()
}
#[expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "the loopback fixture answers the actual typed native request"
)]
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
        let _ready = session.wait(
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
#[expect(
    clippy::unwrap_used,
    clippy::panic,
    reason = "the bounded fixture must admit its descriptor or fail the caller behavior"
)]
fn push(session: &RequestRecoveringSession, text: &str) {
    let end = Instant::now() + Duration::from_secs(5);
    let encoded = json!({"item":{"original":{"kind":"text","text":text}}}).to_string();
    loop {
        match session.try_push_json(&encoded).unwrap() {
            RequestSessionPushStatus::Accepted => return,
            RequestSessionPushStatus::Closed => panic!("feed closed before admission"),
            RequestSessionPushStatus::Full => {
                assert!(Instant::now() < end, "feed did not admit descriptor");
                let _ready = session.wait(
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

#[test]
fn cancelled_session_publishes_final_facts_only_after_held_attempt_settles() {
    let release = Arc::new(Rendezvous::new(2));
    let backend_release = Arc::clone(&release);
    let (entered, arrived) = mpsc::channel();
    let listener = Listener::answering(move |body| {
        entered.send(()).unwrap();
        response(body).after_release(Arc::clone(&backend_release))
    })
    .unwrap();
    let engine = api::fixed(listener.base(), "sk-settlement-recovery")
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
    session.cancel();
    assert!(matches!(session.try_read(), RequestRecoveringRead::Pending));
    assert_eq!(
        session
            .next_packet(RequestSessionWaitInterest::Result)
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    assert!(release.wait());
    let packets = drain(&session);
    let RequestRecoveringResult::Terminal(terminal) = packets.last().unwrap() else {
        panic!("joined cancellation");
    };
    assert_eq!(
        terminal.error.as_ref().unwrap().kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
    assert_eq!(listener.count(), 1);
    let rows = packets
        .iter()
        .filter_map(|packet| match packet {
            RequestRecoveringResult::Row(row) => Some(row),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(
        rows.iter()
            .all(|row| row.outcome().is_ok() && row.ordinal() == 0)
    );
    assert_eq!(
        terminal.facts.as_ref().unwrap().records(),
        rows.len() as u64
    );
}

#[test]
fn lazy_dynamic_choices_recover_in_one_pack_with_each_records_actual_candidates() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"wrong"},"q2":{"type":"choice","probabilities":{"d":0.8,"c":0.2}}}}"#)).unwrap();
    let engine = api::loopback(listener.base(), "sk-dynamic-recovery");
    let request = Request::for_function(
        RequestFunction::Choose,
        RequestArguments {
            question: RequestQuestion::Definition {
                value: Question::choose_records("Which?").unwrap().into(),
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
    .admit()
    .unwrap();
    let records = [(["b", "a"], "first"), (["d", "c"], "second")]
        .into_iter()
        .map(|(options, original)| {
            let mut record = api::record(QuestionInput::Text(original.into()));
            record.options = Some(api::dynamic_candidates(&json!(options).to_string()));
            Ok(record)
        });
    let rows = RefCell::new(Vec::new());
    let sink = |row| rows.borrow_mut().push(row);
    let call = engine
        .execute_request_recovering_sink(
            &request,
            RequestEnvironment {
                controls: thinkthen::CallOptions::new().batch(BatchSetting::Records(
                    std::num::NonZeroUsize::new(2).unwrap(),
                )),
                feed: Some(RequestFeed::from_records("records", records)),
            },
            &sink,
        )
        .unwrap();
    let rows = rows.into_inner();
    assert_eq!(
        rows.iter()
            .map(RequestRecoveredRow::ordinal)
            .collect::<Vec<_>>(),
        [0, 1]
    );
    assert_eq!(rows[0].outcome().unwrap_err().kind(), ErrorKind::Backend);
    assert!(rows[0].cause().is_some());
    let RequestValue::Choices(good) = rows[1].outcome().unwrap() else {
        panic!("dynamic choice");
    };
    assert_eq!(good[0].result().value(), Some("d"));
    assert_eq!(good[0].result().probability(), Some(0.8));
    assert_eq!(call.facts().records(), 2);
    assert_eq!(call.facts().requests_sent(), 1);
    assert_eq!(listener.count(), 1);
    let body: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
    assert_eq!(
        body["questions"]["q1"]["criteria"],
        json!({"b":null,"a":null})
    );
    assert_eq!(
        body["questions"]["q2"]["criteria"],
        json!({"d":null,"c":null})
    );
}

#[test]
fn declared_file_source_yields_located_prefix_before_reading_invalid_utf8_suffix() {
    let path = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("recovering-source-{}.txt", std::process::id()));
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, b"first\n\xff\nunread\n").unwrap();
    let listener = Listener::answering(response).unwrap();
    let engine = api::loopback(listener.base(), "sk-source-recovery");
    let request = Request::for_function(
        RequestFunction::Decide,
        RequestArguments {
            question: RequestQuestion::Definition {
                value: Question::decide("Does it pass?").unwrap().cut().into(),
            },
            input: RequestInput::Source {
                source: thinkthen::RequestSource {
                    paths: vec![path.clone()],
                    reading: ReaderOptions {
                        unit: thinkthen::SourceUnit::Line,
                        window: None,
                    },
                    framing: None,
                    media: thinkthen::ReaderMedia::Text,
                },
            },
            options: RequestOptions::default(),
        },
    )
    .admit()
    .unwrap();
    let rows = RefCell::new(Vec::new());
    let error = engine
        .execute_request_recovering_sink(
            &request,
            RequestEnvironment {
                controls: api::dynamic_one(),
                feed: None,
            },
            &|row| rows.borrow_mut().push(row),
        )
        .unwrap_err();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(rows.borrow().len(), 1);
    let rows = rows.into_inner();
    let RequestValue::Decisions(values) = rows[0].outcome().unwrap() else {
        panic!("located source decision");
    };
    let QuestionInput::Record(original) = values[0].original() else {
        panic!("located source original");
    };
    assert_eq!(original.location().unwrap().file(), path.to_str().unwrap());
    assert_eq!(original.location().unwrap().first_line(), Some(1));
    assert_eq!(error.facts().unwrap().records(), 1);
    assert_eq!(error.facts().unwrap().requests_sent(), 1);
    assert_eq!(listener.count(), 1);
}

#[path = "recovering_stream/terminal.rs"]
mod terminal;

#[test]
fn push_all_then_finish_or_cancel_preserves_parked_recovering_rows() {
    for (blocking, cancel) in [(false, false), (true, false), (true, true)] {
        let listener = Listener::answering(response).unwrap();
        let engine = api::fixed(listener.base(), "sk-parked-recovery")
            .no_cache()
            .max_retries(0)
            .throttle(1)
            .unwrap()
            .max_requests(if cancel { None } else { Some(3) })
            .unwrap()
            .batch(BatchSetting::Records(
                std::num::NonZeroUsize::new(1).unwrap(),
            ))
            .build()
            .unwrap();
        let session = Arc::new(
            engine
                .start_request_recovering_session(request(), Surface::Rust)
                .unwrap(),
        );
        for index in 0..if cancel { 8 } else { 32 } {
            let status = push_bounded(&session, index, blocking);
            if status == RequestSessionPushStatus::Closed {
                break;
            }
        }
        if cancel {
            session.cancel();
        } else {
            session.finish(None).unwrap();
        }
        let packets = drain(&session);
        let rows = packets
            .iter()
            .filter_map(|packet| match packet {
                RequestRecoveringResult::Row(row) => Some(row),
                _ => None,
            })
            .collect::<Vec<_>>();
        if cancel {
            assert!(rows.len() >= 3, "push parked a completed prefix");
        } else {
            assert_eq!(rows.len(), 3);
        }
        for (index, row) in rows.iter().enumerate() {
            assert_eq!(row.ordinal(), index);
            let RequestValue::Decisions(values) = row.outcome().unwrap() else {
                panic!("decision row")
            };
            let thinkthen::QuestionInput::Record(original) = values[0].original() else {
                panic!("feed original")
            };
            assert_eq!(
                original.original().literal(),
                Some(format!("row {index}").as_str())
            );
        }
        let RequestRecoveringResult::Terminal(terminal) = packets.last().unwrap() else {
            panic!("terminal follows parked rows")
        };
        assert_eq!(
            terminal.error.as_ref().unwrap().kind(),
            if cancel {
                ErrorKind::Cancelled
            } else {
                ErrorKind::Usage
            }
        );
        if !cancel {
            assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 3);
            assert_eq!(listener.count(), 3);
        }
    }
}

fn push_bounded(
    session: &Arc<RequestRecoveringSession>,
    index: usize,
    blocking: bool,
) -> RequestSessionPushStatus {
    let waiting = Arc::clone(&session);
    let (sent, received) = mpsc::channel();
    let producer = std::thread::spawn(move || {
        let status = if blocking {
            waiting
                .push_with(|| Ok::<_, thinkthen::Error>(descriptor(&format!("row {index}"))))
                .unwrap()
        } else {
            let encoded =
                json!({"item":{"original":{"kind":"text","text":format!("row {index}")}}})
                    .to_string();
            loop {
                let status = waiting.try_push_json(&encoded).unwrap();
                if status != RequestSessionPushStatus::Full {
                    break status;
                }
                std::thread::yield_now();
            }
        };
        let _ = sent.send(status);
    });
    let status = received.recv_timeout(Duration::from_secs(2));
    if status.is_err() {
        session.cancel();
    }
    producer.join().unwrap();
    status.unwrap()
}
