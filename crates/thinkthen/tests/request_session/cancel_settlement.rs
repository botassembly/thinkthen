//! One-cell output backpressure fixes cancellation before native final settlement.
use super::*;

fn ranked_request() -> Result<Request, Error> {
    Ok(Request::for_function(
        RequestFunction::Rank,
        RequestArguments {
            question: RequestQuestion::Definition {
                value: Question::rank("Does it pass?")?.into(),
            },
            input: RequestInput::Records {
                items: vec![descriptor("first").item],
            },
            options: RequestOptions {
                batch: Some(RequestBatch::Named("max".into())),
                ..Default::default()
            },
        },
    ))
}

fn options(retain: bool) -> RequestSessionFeedOptions {
    RequestSessionFeedOptions {
        retain_success_on_cancel: retain,
        ..Default::default()
    }
}

fn terminal(terminal: &RequestSessionTerminal, retain: bool) {
    assert_eq!(terminal.facts.as_ref().map(Facts::requests_sent), Some(1));
    assert_eq!(terminal.facts.as_ref().map(Facts::records), Some(1));
    if retain {
        assert!(terminal.error.is_none());
        assert_eq!(terminal.resolution, AnswerResolution::Resolved);
    } else {
        assert_eq!(
            terminal.error.as_ref().map(Error::kind),
            Some(ErrorKind::Cancelled)
        );
        assert_eq!(terminal.resolution, AnswerResolution::Failed);
    }
}

#[test]
fn final_settlement_retains_success_only_when_the_caller_opts_in() {
    for retain in [false, true] {
        let listener = Listener::answering(response).unwrap();
        let session = engine(&listener)
            .request_session_with_feed_options(
                ranked_request().unwrap(),
                Surface::Rust,
                options(retain),
            )
            .unwrap();
        // Do not consume Question. Its occupied output cell blocks Row publication,
        // so the worker cannot reach Stop::finish before cancellation is latched.
        assert_eq!(
            session.wait(RequestSessionWaitInterest::Result, Duration::from_secs(5)),
            RequestSessionReady::Result
        );
        session.cancel();
        let packets = drain(&session);
        let kinds = packets
            .iter()
            .map(|packet| match packet {
                RequestSessionResult::Observation {
                    value: OwnedRecordObservation::Question { .. },
                    ..
                } => "question",
                RequestSessionResult::Observation {
                    value: OwnedRecordObservation::Row { .. },
                    ..
                } => "row",
                RequestSessionResult::Aggregate(RequestValue::Ranked(rows)) => {
                    assert!(retain);
                    assert_eq!(rows.len(), 1);
                    "aggregate"
                }
                RequestSessionResult::Terminal(value) => {
                    terminal(value, retain);
                    "terminal"
                }
                other => panic!("unexpected packet {other:?}"),
            })
            .collect::<Vec<_>>();
        assert_eq!(
            kinds,
            if retain {
                vec!["question", "row", "aggregate", "terminal"]
            } else {
                vec!["question", "row", "terminal"]
            }
        );
        assert_eq!(listener.count(), 1);
    }
}

#[test]
fn recovering_settlement_uses_the_same_explicit_success_policy() {
    for retain in [false, true] {
        let listener = Listener::answering(response).unwrap();
        let session = engine(&listener)
            .request_recovering_session_with_feed_options(
                ranked_request().unwrap(),
                Surface::Rust,
                options(retain),
            )
            .unwrap();
        assert_eq!(
            session.wait(RequestSessionWaitInterest::Result, Duration::from_secs(5)),
            RequestSessionReady::Result
        );
        session.cancel();
        let until = Instant::now() + Duration::from_secs(5);
        let mut kinds = vec![];
        loop {
            match session.try_read() {
                RequestRecoveringRead::Result(packet) => {
                    kinds.push(recovered_kind(packet, retain).unwrap())
                }
                RequestRecoveringRead::Pending => {
                    assert!(Instant::now() < until, "recovering session did not settle");
                    assert!(matches!(
                        session.wait(
                            RequestSessionWaitInterest::Settlement,
                            until.saturating_duration_since(Instant::now()),
                        ),
                        RequestSessionReady::Result | RequestSessionReady::End
                    ));
                }
                RequestRecoveringRead::End => break,
            }
        }
        assert_eq!(
            kinds,
            if retain {
                vec!["question", "row", "aggregate", "terminal"]
            } else {
                vec!["question", "row", "terminal"]
            }
        );
        assert_eq!(listener.count(), 1);
    }
}

#[test]
fn opting_in_still_stops_an_empty_feed_before_any_send() {
    for retain in [false, true] {
        let listener = Listener::answering(response).unwrap();
        let session = engine(&listener)
            .request_session_with_feed_options(request(feed()), Surface::Rust, options(retain))
            .unwrap();
        session.cancel();
        let packets = drain(&session);
        assert_eq!(packets.len(), 1);
        let RequestSessionResult::Terminal(terminal) = &packets[0] else {
            panic!("terminal")
        };
        assert_eq!(
            terminal.error.as_ref().unwrap().kind(),
            ErrorKind::Cancelled
        );
        assert_eq!(listener.count(), 0);
    }
}

fn recovered_kind(
    packet: RequestRecoveringResult,
    retain: bool,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    Ok(match packet {
        RequestRecoveringResult::Observation {
            value: OwnedRecordObservation::Question { .. },
            ..
        } => "question",
        RequestRecoveringResult::Observation {
            value: OwnedRecordObservation::Row { .. },
            ..
        } => "row",
        RequestRecoveringResult::Row(row) => {
            assert!(retain);
            let RequestValue::Ranked(rows) = row.outcome().map_err(ToString::to_string)? else {
                return Err("rank aggregate".into());
            };
            assert_eq!(rows.len(), 1);
            "aggregate"
        }
        RequestRecoveringResult::Terminal(value) => {
            terminal(&value, retain);
            "terminal"
        }
    })
}

#[test]
fn owned_interrupt_cancels_or_contains_panics_before_any_send() {
    for panics in [false, true] {
        let listener = Listener::answering(response).unwrap();
        let session = engine(&listener)
            .request_session_with_feed_options(
                ranked_request().unwrap(),
                Surface::Rust,
                RequestSessionFeedOptions {
                    interrupt: Some(Arc::new(move || {
                        assert!(!panics, "owned interrupt panic");
                        true
                    })),
                    ..Default::default()
                },
            )
            .unwrap();
        let packets = drain(&session);
        assert_eq!(packets.len(), 1);
        let RequestSessionResult::Terminal(terminal) = &packets[0] else {
            panic!("terminal")
        };
        assert_eq!(
            terminal.error.as_ref().unwrap().kind(),
            if panics { ErrorKind::Defect } else { ErrorKind::Cancelled }
        );
        assert_eq!(listener.count(), 0);
    }
}
