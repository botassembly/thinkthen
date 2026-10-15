use super::*;

#[cfg(test)]
fn recognition_response(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap();
    let answers = request["questions"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, question)| {
            let labels = question["criteria"].as_object().unwrap();
            let pick = if labels.contains_key("SINGLE") {
                "SINGLE"
            } else {
                "person"
            };
            let probabilities = labels
                .keys()
                .map(|label| (label.clone(), json!(u8::from(label == pick))))
                .collect::<serde_json::Map<_, _>>();
            (
                name.clone(),
                json!({"type":"choice","probabilities":probabilities}),
            )
        })
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
}

#[test]
fn repeated_recognition_retains_sources_and_replays_at_parallel_width() {
    for mode in 0..3 {
        let session_mode = mode == 2;
        let listener = Listener::answering(recognition_response).unwrap();
        let cache = std::env::temp_dir().join(format!(
            "thinkthen-recognition-{}-{mode}",
            std::process::id()
        ));
        std::fs::create_dir(&cache).unwrap();
        let engine = crate::api::fixed(listener.base(), "recognition-fixture")
            .cache_at(&cache)
            .unwrap()
            .throttle(4)
            .unwrap()
            .max_retries(0)
            .build()
            .unwrap();
        let input = if session_mode {
            RequestInput::Feed {
                name: "records".into(),
                framing: RequestFraming::Lines,
                reading: ReaderOptions::default(),
                images: vec![],
            }
        } else {
            RequestInput::Records {
                items: vec![descriptor("Ada").item; 6],
            }
        };
        let request = recognition_request(input);
        for expected_sends in [2, 0] {
            let rows = if session_mode {
                session_rows(&engine, &request, expected_sends)
            } else {
                native_rows(&engine, &request, expected_sends, mode == 1)
            };
            assert_rows(&rows, session_mode);
            assert_eq!(listener.count(), 2);
        }
        drop(engine);
        std::fs::remove_dir_all(cache).unwrap();
    }
}

#[cfg(test)]
fn session_rows(
    engine: &Engine,
    request: &Request,
    expected_sends: u64,
) -> Vec<CompleteRecord<QuestionInput, CompleteRecognized>> {
    let session = engine.request_session(request.clone()).unwrap();
    let mut packets = Vec::new();
    for index in 0..6 {
        let mut item = descriptor("Ada\n");
        item.location =
            Some(SourceLocation::new(format!("source-{index}"), Some(1), Some(1)).unwrap());
        loop {
            match session.try_push(item).unwrap() {
                RequestSessionPush::Accepted => break,
                RequestSessionPush::Full(returned) => item = returned,
                RequestSessionPush::Closed(_) => panic!("recognition intake closed"),
            }
            if let RequestSessionRead::Result(packet) = session.try_read() {
                packets.push(packet);
            }
            let _ready = session.wait(RequestSessionWaitInterest::Input, Duration::from_millis(50));
        }
    }
    session.finish(None).unwrap();
    let mut rows = Vec::new();
    packets.extend(drain(&session));
    for packet in packets {
        match packet {
            RequestSessionResult::Aggregate(RequestValue::Recognized(value)) => rows = value,
            RequestSessionResult::Terminal(value) => {
                assert!(value.error.is_none());
                assert_eq!(value.facts.unwrap().requests_sent(), expected_sends);
            }
            _ => {}
        }
    }
    rows
}

#[cfg(test)]
fn recognition_request(input: RequestInput) -> Request {
    let question = Recognize::builder()
        .kind(Kind::new("person", None).unwrap())
        .unwrap()
        .kind(Kind::new("organization", None).unwrap())
        .unwrap()
        .build()
        .unwrap();
    Request::new(RequestCall::new(
        RequestFunction::Recognize,
        RequestArguments {
            question: RequestQuestion::Definition {
                value: question.into(),
            },
            input,
            options: RequestOptions::default(),
        },
    ))
}

#[test]
#[expect(clippy::print_stderr, reason = "report Windows cancellation phases")]
fn cancellation_stops_waiting_duplicate_recognition_without_extra_sends() {
    let release = Arc::new(Rendezvous::new(2));
    let held = Arc::clone(&release);
    let (sent, received) = mpsc::channel();
    let (written, writes) = mpsc::channel();
    let listener = Listener::answering_with_events(
        move |body| {
            recognition_response(body)
                .after_release(Arc::clone(&held))
                .notifying(written.clone())
        },
        sent,
    )
    .unwrap();
    let cache = std::env::temp_dir().join(format!(
        "thinkthen-recognition-cancel-{}",
        std::process::id()
    ));
    std::fs::create_dir(&cache).unwrap();
    let engine = crate::api::fixed(listener.base(), "recognition-fixture")
        .throttle(4)
        .unwrap()
        .cache_at(&cache)
        .unwrap()
        .max_retries(0)
        .build()
        .unwrap();
    let session = engine
        .request_session(recognition_request(RequestInput::Records {
            items: vec![descriptor("Ada").item; 6],
        }))
        .unwrap();
    received.recv_timeout(Duration::from_secs(5)).unwrap();
    session.cancel();
    assert!(release.wait());
    let packets = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drain(&session)));
    eprintln!(
        "recognition cancel: drain returned; sends={}; write_attempt_ack={:?}; readiness={:?}; session={session:?}",
        listener.count(),
        writes.try_recv(),
        session.wait(RequestSessionWaitInterest::Settlement, Duration::ZERO),
    );
    let packets = packets.unwrap_or_else(|panic| std::panic::resume_unwind(panic));
    let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
        panic!("joined terminal")
    };
    assert_eq!(
        terminal.error.as_ref().unwrap().kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
    assert_eq!(terminal.facts.as_ref().unwrap().records(), 0);
    assert_eq!(listener.count(), 1);
    drop(engine);
    std::fs::remove_dir_all(cache).unwrap();
}

#[cfg(test)]
fn assert_rows(rows: &[CompleteRecord<QuestionInput, CompleteRecognized>], session_mode: bool) {
    assert_eq!(rows.len(), 6);
    for (index, row) in rows.iter().enumerate() {
        assert_eq!(row.ordinal(), index);
        let value: Value = serde_json::from_str(&row.result().to_json().unwrap()).unwrap();
        assert_eq!(value["value"]["entities"][0]["text"], "Ada");
        if session_mode {
            let QuestionInput::Record(original) = row.original() else {
                panic!("located original")
            };
            assert_eq!(
                original.location().unwrap().file(),
                format!("source-{index}")
            );
            assert_eq!(value["source"]["file"], format!("source-{index}"));
        } else {
            let QuestionInput::Record(original) = row.original() else {
                panic!("record original")
            };
            assert_eq!(original.original().literal(), Some("Ada"));
            assert!(original.location().is_none());
        }
    }
}

#[cfg(test)]
fn native_rows(
    engine: &Engine,
    request: &Request,
    expected_sends: u64,
    observe: bool,
) -> Vec<CompleteRecord<QuestionInput, CompleteRecognized>> {
    let admitted = request.clone().admit().unwrap();
    let observer = |_: RecordObservation<'_>| {};
    let controls = if observe {
        CallOptions::new().observe(&observer)
    } else {
        CallOptions::new()
    };
    let call = engine
        .execute_request(
            &admitted,
            RequestEnvironment {
                controls,
                feed: None,
            },
        )
        .unwrap();
    let RequestOutcome::Complete(call) = call else {
        panic!("complete recognition")
    };
    assert_eq!(call.facts().requests_sent(), expected_sends);
    let RequestValue::Recognized(rows) = call.into_value() else {
        panic!("recognized rows")
    };
    rows
}

#[test]
fn uncached_and_refreshed_repeated_recognition_keep_parallel_sends() {
    for refresh in [false, true] {
        let release = Arc::new(Rendezvous::new(3));
        let held = Arc::clone(&release);
        let (sent, received) = mpsc::channel();
        let listener = Listener::answering_with_events(
            move |body| recognition_response(body).after_release(Arc::clone(&held)),
            sent,
        )
        .unwrap();
        let cache = std::env::temp_dir().join(format!(
            "thinkthen-recognition-parallel-{}-{refresh}",
            std::process::id()
        ));
        std::fs::create_dir(&cache).unwrap();
        let builder = crate::api::fixed(listener.base(), "recognition-fixture")
            .throttle(4)
            .unwrap()
            .max_retries(0);
        let builder = if refresh {
            builder.cache_at(&cache).unwrap().refresh_cache(true)
        } else {
            builder.no_cache()
        };
        let engine = builder.build().unwrap();
        let session = engine
            .request_session(recognition_request(RequestInput::Records {
                items: vec![descriptor("Ada").item; 2],
            }))
            .unwrap();
        for round in 0..2 {
            for arrival in 0..2 {
                received.recv_timeout(Duration::from_secs(5)).unwrap_or_else(|error| {
                    panic!("recognition arrival {round}/{arrival}, refresh {refresh}: {error:?}; sends {}; requests {:?}; session {session:?}", listener.count(), listener.requests());
                });
            }
            assert!(release.wait());
        }
        let packets = drain(&session);
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
            panic!("joined terminal")
        };
        assert!(terminal.error.is_none());
        assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 4);
        assert_eq!(terminal.facts.as_ref().unwrap().records(), 2);
        assert_eq!(listener.count(), 4);
        drop(engine);
        std::fs::remove_dir_all(cache).unwrap();
    }
}
