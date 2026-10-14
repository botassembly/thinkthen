//! Find keeps the complete-set boundary while bounding caller-side source admission.
use super::*;
use thinkthen::RequestFunction;
use thinkthen::{CancelToken, RecordInput};
#[test]
fn cancelled_find_stops_before_pulling_a_suffix_in_complete_and_released_calls() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = engine(&listener);
    let question = Question::find("Which?").unwrap();
    for complete in [false, true] {
        let token = CancelToken::new();
        let pulls = AtomicUsize::new(0);
        let units = std::iter::from_fn(|| {
            let at = pulls.fetch_add(1, Ordering::Relaxed);
            if at == 3 {
                return None;
            }
            token.cancel();
            Some(Ok("Plain."))
        });
        let options = CallOptions::new().cancel(&token);
        let error = if complete {
            crate::api::find(
                &engine,
                &question,
                units.map(|row| row.map(crate::api::record)),
                options,
            )
            .map(|_| ())
            .unwrap_err()
        } else {
            engine
                .find_with(&question, units.map(Result::unwrap), options)
                .map(|_| ())
                .unwrap_err()
        };
        assert_eq!(error.kind(), ErrorKind::Cancelled);
        assert_eq!(
            pulls.load(Ordering::Relaxed),
            1,
            "cancelled admission pulled the suffix"
        );
        assert!(error.facts().is_none());
        assert_eq!(listener.count(), 0);
    }
}
#[test]
fn pre_cancelled_find_never_pulls_units_in_complete_or_released_calls() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = engine(&listener);
    let question = Question::find("Which?").unwrap();
    for complete in [false, true] {
        let token = CancelToken::new();
        token.cancel();
        let pulls = AtomicUsize::new(0);
        let units = (0..3).map(|_| {
            pulls.fetch_add(1, Ordering::Relaxed);
            Ok("Plain.")
        });
        let options = CallOptions::new().cancel(&token);
        let error = if complete {
            crate::api::find(
                &engine,
                &question,
                units.map(|row| row.map(crate::api::record)),
                options,
            )
            .map(|_| ())
            .unwrap_err()
        } else {
            engine
                .find_with(&question, units.map(Result::unwrap), options)
                .map(|_| ())
                .unwrap_err()
        };
        assert_eq!(error.kind(), ErrorKind::Cancelled);
        assert_eq!(pulls.load(Ordering::Relaxed), 0);
        assert!(error.facts().is_none());
    }
    assert_eq!(listener.count(), 0);
}
#[test]
fn composed_find_refuses_one_unit_without_none_and_stops_at_one_excess_candidate() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = engine(&listener);
    for (count, none) in [(1, false), (1_000, false), (1_000, true)] {
        let question = if none {
            Question::find("Which?").unwrap().offering_none().unwrap()
        } else {
            Question::find("Which?").unwrap()
        };
        let pulls = AtomicUsize::new(0);
        let units = (0..count).map(|_| {
            pulls.fetch_add(1, Ordering::Relaxed);
            Ok(crate::api::record("Plain."))
        });
        let error = crate::api::find(&engine, &question, units, CallOptions::new()).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.detail().message(),
            if none {
                "a find question offering none takes 1 to 254 units"
            } else {
                "find takes 2 to 255 units"
            }
        );
        assert_eq!(
            pulls.load(Ordering::Relaxed),
            count.min(if none { 255 } else { 256 })
        );
        assert!(error.facts().is_none());
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn find_count_and_byte_refusals_leave_panicking_tails_unread() {
    let listener = Listener::answering(|_| Canned::ok("unused")).unwrap();
    let engine = engine(&listener);
    for complete in [false, true] {
        for none in [false, true] {
            let question = if none {
                Question::find("Which?").unwrap().offering_none().unwrap()
            } else {
                Question::find("Which?").unwrap()
            };
            let maximum = if none { 254 } else { 255 };
            let units = (0..=maximum)
                .map(|_| Ok("candidate"))
                .chain(std::iter::from_fn(
                    || -> Option<Result<&str, thinkthen::Error>> {
                        panic!("find read beyond its first excess candidate")
                    },
                ));
            let error = if complete {
                crate::api::find(
                    &engine,
                    &question,
                    units.map(|row| row.map(crate::api::record)),
                    CallOptions::new(),
                )
                .map(|_| ())
            } else {
                engine
                    .find_with(&question, units.map(Result::unwrap), CallOptions::new())
                    .map(|_| ())
            }
            .unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Usage);
            assert!(error.facts().is_none());
        }
        let large = "x".repeat(8 * 1024 * 1024 + 1);
        let units = [Ok(large.as_str()), Ok(large.as_str())]
            .into_iter()
            .chain(std::iter::from_fn(
                || -> Option<Result<&str, thinkthen::Error>> {
                    panic!("find read beyond aggregate evidence byte refusal")
                },
            ));
        let question = Question::find("Which?").unwrap();
        let error = if complete {
            crate::api::find(
                &engine,
                &question,
                units.map(|row| row.map(crate::api::record)),
                CallOptions::new(),
            )
            .map(|_| ())
        } else {
            engine
                .find_with(&question, units.map(Result::unwrap), CallOptions::new())
                .map(|_| ())
        }
        .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert!(error.facts().is_none());
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn typed_relations_leave_the_tail_after_the_256th_entity_unread() {
    let listener = Listener::answering(|_| Canned::ok("unused")).unwrap();
    let engine = engine(&listener);
    let ask = thinkthen::Relate::from_json(r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person","reads":"knows"}]}}"#).unwrap();
    let reading = thinkthen::RecordReading::new(&[], None, None).unwrap();
    for complete in [false, true] {
        let entities = (0..256)
            .map(|at| thinkthen::Entity::new(&format!("person {at}"), "person").unwrap())
            .chain(std::iter::from_fn(|| -> Option<thinkthen::Entity> {
                panic!("relate read beyond its first excess entity")
            }));
        let error = if complete {
            crate::api::relation(
                &engine,
                &ask,
                entities.map(|entity| {
                    Ok(reading
                        .compose(
                            thinkthen::RawRecord::json(
                                &serde_json::json!({"name":entity.name(),"kind":entity.kind()})
                                    .to_string(),
                            )
                            .unwrap(),
                        )
                        .unwrap())
                }),
                CallOptions::new(),
            )
            .map(|_| ())
        } else {
            engine
                .relate_with(&ask, entities, CallOptions::new())
                .map(|_| ())
        }
        .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert!(error.detail().message().contains("at most 255 entities"));
        assert!(error.facts().is_none());
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn find_snapshots_each_caller_evidence_once_and_restores_originals() {
    #[derive(Clone)]
    struct Once<'a> {
        calls: &'a AtomicUsize,
        text: &'static str,
    }
    impl thinkthen::Evidence for Once<'_> {
        fn evidence(&self) -> &str {
            assert_eq!(
                self.calls.fetch_add(1, Ordering::Relaxed),
                0,
                "caller evidence was reread"
            );
            self.text
        }
    }
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"u001":0.2,"u002":0.8}}}}"#)).unwrap();
    let engine = engine(&listener);
    let question = Question::find("Which?").unwrap();
    let first = AtomicUsize::new(0);
    let second = AtomicUsize::new(0);
    let call = engine
        .find_with(
            &question,
            [
                Once {
                    calls: &first,
                    text: "first",
                },
                Once {
                    calls: &second,
                    text: "second",
                },
            ],
            CallOptions::new(),
        )
        .unwrap();
    assert_eq!(call.value().as_ref().unwrap().text, "second");
    assert_eq!(first.load(Ordering::Relaxed), 1);
    assert_eq!(second.load(Ordering::Relaxed), 1);
    let refused = AtomicUsize::new(0);
    let error = engine
        .find_with(
            &question,
            [Once {
                calls: &refused,
                text: "only candidate",
            }],
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(refused.load(Ordering::Relaxed), 1);
    assert!(error.facts().is_none());
    assert_eq!(listener.count(), 1);
}

#[test]
fn canonical_find_applies_the_engine_record_cap_before_reading_the_tail() {
    use thinkthen::{
        Request, RequestArguments, RequestCall, RequestEnvironment, RequestFeed, RequestFraming,
        RequestInput, RequestOptions, RequestQuestion,
    };
    let listener = Listener::answering(|_| Canned::ok("unused")).unwrap();
    let engine = crate::api::fixed(listener.base(), "complete-private")
        .no_cache()
        .max_requests(Some(1))
        .unwrap()
        .build()
        .unwrap();
    let request = Request::new(RequestCall::new(
        RequestFunction::Find,
        RequestArguments {
            question: RequestQuestion::Definition {
                value: Question::find("Which?").unwrap().into(),
            },
            input: RequestInput::Feed {
                name: "bounded".into(),
                framing: RequestFraming::Document,
                reading: thinkthen::ReaderOptions::default(),
                images: vec![],
            },
            options: RequestOptions::default(),
        },
    ))
    .admit()
    .unwrap();
    let rows = (0..2)
        .map(|_| {
            Ok(crate::api::record(thinkthen::QuestionInput::Text(
                "candidate".into(),
            )))
        })
        .chain(std::iter::from_fn(
            || -> Option<Result<RecordInput<thinkthen::QuestionInput>, thinkthen::Error>> {
                panic!("canonical find read after the first excess engine record")
            },
        ));
    let error = engine
        .execute_request(
            &request,
            RequestEnvironment {
                controls: CallOptions::new(),
                feed: Some(RequestFeed::from_records("bounded", rows).eager()),
            },
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.detail().message(),
        "this engine answers at most 1 records in one call"
    );
    assert!(error.facts().is_none());
    assert_eq!(listener.count(), 0);
}
