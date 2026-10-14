use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use thinkthen::{Recognize, RecordObservation};

fn recognition_response(body: &[u8]) -> Canned {
    let body: Value = serde_json::from_slice(body).unwrap();
    let answers = body["questions"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, question)| {
            let labels = question["criteria"].as_object().unwrap();
            let picked = if labels.contains_key("SINGLE") {
                "SINGLE"
            } else {
                "person"
            };
            let probabilities = labels
                .keys()
                .map(|label| (label.clone(), json!(u8::from(label == picked))))
                .collect::<serde_json::Map<_, _>>();
            (
                name.clone(),
                json!({"type":"choice","probabilities":probabilities}),
            )
        })
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
}

fn feed_request(
    function: RequestFunction,
    definition: thinkthen::RequestDefinition,
) -> thinkthen::AdmittedRequest {
    Request::for_function(
        function,
        RequestArguments {
            question: RequestQuestion::Definition { value: definition },
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
    .unwrap()
}

#[test]
fn eager_recovery_counts_completed_rows_when_the_observer_cancels() {
    let cases = [
        (
            RequestFunction::Decide,
            Question::decide("Pass?").unwrap().cut().into(),
            2,
            2,
        ),
        (
            RequestFunction::Annotate,
            thinkthen::QuestionSet::from_json(
                r#"{"version":1,"questions":{"pass":{"decide":"Pass?"}}}"#,
            )
            .unwrap()
            .into(),
            2,
            2,
        ),
        (
            RequestFunction::Recognize,
            Recognize::builder().build().unwrap().into(),
            1,
            1,
        ),
    ];
    for (function, definition, completed, sends) in cases {
        let listener = Listener::answering(move |body| {
            if function == RequestFunction::Recognize {
                recognition_response(body)
            } else {
                response(body)
            }
        })
        .unwrap();
        let engine = api::loopback(listener.base(), "sk-prefix-recovery");
        let request = feed_request(function, definition);
        let cancel = CancelToken::new();
        let observed = AtomicUsize::new(0);
        let observer = |event: RecordObservation<'_>| {
            if matches!(event, RecordObservation::Row { .. })
                && observed.fetch_add(1, Ordering::SeqCst) + 1 == completed
            {
                cancel.cancel();
            }
        };
        let feed = RequestFeed::from_records(
            "records",
            api::plain_records([
                QuestionInput::Text("Ada".into()),
                QuestionInput::Text("Bob".into()),
            ]),
        )
        .eager();
        let error = engine
            .execute_request_recovering(
                &request,
                RequestEnvironment {
                    controls: api::dynamic_one().cancel(&cancel).observe(&observer),
                    feed: Some(feed),
                },
            )
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Cancelled);
        assert_eq!(observed.load(Ordering::SeqCst), completed);
        assert_eq!(error.facts().unwrap().records(), completed as u64);
        assert_eq!(
            error.facts().unwrap().requests_sent(),
            sends,
            "{function:?}"
        );
        assert_eq!(listener.count(), sends as usize);
        assert!(!format!("{error:?}").contains("sk-prefix-recovery"));
    }
}

#[test]
fn lazy_recognition_schema_refusal_is_terminal_before_sending() {
    let listener = Listener::answering(recognition_response).unwrap();
    let engine = api::loopback(listener.base(), "sk-schema-recovery");
    let ask =
        Recognize::from_json(r#"{"version":1,"recognize":{},"item_schema":{"type":"object","properties":{"name":{"type":"string"}}}}"#)
            .unwrap();
    let request = feed_request(RequestFunction::Recognize, ask.into());
    let pulls = Cell::new(0);
    let feed = RequestFeed::from_records(
        "records",
        ["Ada", "Bob"].into_iter().map(|text| {
            pulls.set(pulls.get() + 1);
            Ok(api::record(QuestionInput::Text(text.into())))
        }),
    );
    let rows = RefCell::new(Vec::new());
    let error = engine
        .execute_request_recovering_sink(
            &request,
            RequestEnvironment {
                controls: api::dynamic_one(),
                feed: Some(feed),
            },
            &|row| rows.borrow_mut().push(row),
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(error.stopped().at(), Some(1));
    assert_eq!(pulls.get(), 1);
    assert!(rows.borrow().is_empty());
    assert_eq!(error.facts().unwrap().records(), 0);
    assert_eq!(error.facts().unwrap().requests_sent(), 0);
    assert_eq!(listener.count(), 0);
    assert!(!format!("{error:?}").contains("sk-schema-recovery"));
}

#[test]
fn recovery_record_limit_stops_the_second_record_before_another_send() {
    for eager in [false, true] {
        let listener = Listener::answering(response).unwrap();
        let engine = api::fixed(listener.base(), "sk-limit-recovery")
            .no_cache()
            .max_retries(0)
            .max_requests(Some(1))
            .unwrap()
            .build()
            .unwrap();
        let request = request().admit().unwrap();
        let feed = RequestFeed::from_records(
            "records",
            api::plain_records([
                QuestionInput::Text("first".into()),
                QuestionInput::Text("second".into()),
            ]),
        );
        let feed = if eager { feed.eager() } else { feed };
        let rows = RefCell::new(Vec::new());
        let error = engine
            .execute_request_recovering_sink(
                &request,
                RequestEnvironment {
                    controls: api::dynamic_one(),
                    feed: Some(feed),
                },
                &|row| rows.borrow_mut().push(row),
            )
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.detail().message(),
            "this engine answers at most 1 records in one call"
        );
        assert_eq!(rows.borrow().len(), usize::from(!eager));
        assert_eq!(listener.count(), usize::from(!eager));
        if !eager {
            assert_eq!(error.facts().unwrap().requests_sent(), 1);
            assert_eq!(error.facts().unwrap().records(), 1);
        }
        assert!(!format!("{error:?}").contains("sk-limit-recovery"));
    }
}
