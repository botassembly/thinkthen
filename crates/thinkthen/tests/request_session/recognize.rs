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
    for session_mode in [false, true] {
        let listener = Listener::answering(recognition_response).unwrap();
        let cache = std::env::temp_dir().join(format!(
            "thinkthen-recognition-{}-{session_mode}",
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
                native_rows(&engine, &request, expected_sends)
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
fn cancellation_stops_waiting_duplicate_recognition_without_extra_sends() {
    let release = Arc::new(Rendezvous::new(2));
    let held = Arc::clone(&release);
    let (sent, received) = mpsc::channel();
    let listener = Listener::answering_with_events(
        move |body| recognition_response(body).after_release(Arc::clone(&held)),
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
    let packets = drain(&session);
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
) -> Vec<CompleteRecord<QuestionInput, CompleteRecognized>> {
    let admitted = request.clone().admit().unwrap();
    let call = engine
        .execute_request(&admitted, RequestEnvironment::default())
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
