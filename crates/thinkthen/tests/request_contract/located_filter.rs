//! File selection derives from attached composed originals and retains host controls.
use super::*;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

#[allow(
    clippy::unwrap_used,
    reason = "fixed native fixture construction stops on invalid setup"
)]
fn request() -> Request {
    let mut arguments = args(
        Question::decide("Fits?").unwrap().cut().into(),
        RequestInput::Feed {
            name: "located".into(),
            framing: RequestFraming::Document,
            reading: ReaderOptions::default(),
            images: vec![],
        },
    );
    arguments.options.files_only = true;
    Request::filter(arguments)
}

#[allow(
    clippy::unwrap_used,
    reason = "fixed native fixture construction stops on invalid setup"
)]
fn row(file: &str, line: usize) -> RecordInput<QuestionInput> {
    RecordReading::new(&[], None, None)
        .unwrap()
        .compose(RawRecord::text("Alpha.").unwrap())
        .unwrap()
        .map_original(|record| {
            record
                .with_location(SourceLocation::new(file.into(), Some(line), Some(line)).unwrap())
                .question_input()
        })
}

#[test]
fn file_filter_selects_actual_original_locations_and_preserves_host_send_budget() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let request = request().admit().unwrap();
    let observed = AtomicUsize::new(0);
    let observer = |event: RecordObservation<'_>| {
        if matches!(event, RecordObservation::Row { .. }) {
            observed.fetch_add(1, Ordering::Relaxed);
        }
    };
    for eager in [false, true] {
        let budget = SendBudget::new();
        let feed = RequestFeed::from_records(
            "located",
            [
                row("unopened/a.txt", 7),
                row("unopened/a.txt", 9),
                row("unopened/b.txt", 3),
            ]
            .into_iter()
            .map(Ok),
        );
        let outcome = engine
            .execute_request(
                &request,
                RequestEnvironment {
                    controls: CallOptions::new()
                        .send_budget(&budget, Some(1))
                        .observe(&observer),
                    feed: Some(if eager { feed.eager() } else { feed }),
                },
            )
            .unwrap();
        let RequestOutcome::Complete(call) = outcome else {
            panic!("complete filter")
        };
        let RequestValue::Filtered(rows) = call.value() else {
            panic!("filtered rows")
        };
        assert_eq!(rows.len(), 2);
        for (row, file, line) in [
            (&rows[0], "unopened/a.txt", 7),
            (&rows[1], "unopened/b.txt", 3),
        ] {
            let QuestionInput::Record(original) = row.original() else {
                panic!("native original")
            };
            assert_eq!(
                original.location(),
                Some(&SourceLocation::new(file.into(), Some(line), Some(line)).unwrap())
            );
        }
        assert_eq!(observed.load(Ordering::Relaxed), if eager { 6 } else { 3 });
        let blocked = engine.execute_request(
            &request,
            RequestEnvironment {
                controls: CallOptions::new().send_budget(&budget, Some(1)),
                feed: Some(
                    RequestFeed::from_records(
                        "located",
                        std::iter::once(Ok(row("unopened/c.txt", 1))),
                    )
                    .eager(),
                ),
            },
        );
        let error = match blocked {
            Err(error) | Ok(RequestOutcome::Failed { error, .. }) => error,
            Ok(RequestOutcome::Complete(_)) => panic!("host budget must stop the second call"),
        };
        assert!(error.send_budget_denial().is_some());
    }
    assert_eq!(listener.count(), 2);
}

#[test]
fn file_filter_refuses_ordinary_feeds_and_bad_descriptors_before_advancing() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let advanced = Cell::new(0);
    let admitted = request().admit().unwrap();
    let error = engine
        .execute_request(
            &admitted,
            RequestEnvironment {
                controls: CallOptions::new(),
                feed: Some(RequestFeed::new(
                    "located",
                    std::iter::once_with(|| {
                        advanced.set(advanced.get() + 1);
                        Ok(item("Alpha."))
                    }),
                )),
            },
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.detail().message(),
        "files_only requires filter source input"
    );
    let mut projected = request();
    projected.call.arguments.options.field = Some(vec!["/body".into()]);
    let error = engine
        .execute_request(
            &projected.admit().unwrap(),
            RequestEnvironment {
                controls: CallOptions::new(),
                feed: Some(RequestFeed::from_records(
                    "located",
                    std::iter::once_with(|| {
                        advanced.set(advanced.get() + 1);
                        Ok(row("unopened/a.txt", 7))
                    }),
                )),
            },
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.detail().message(),
        "native composed records own framing, projections and attachments"
    );
    let mut invalid = request();
    invalid.call.arguments.options.field = Some(vec!["bad-pointer".into()]);
    assert_eq!(invalid.admit().unwrap_err().kind(), ErrorKind::Usage);
    let mut invalid = request();
    invalid.call = RequestCall::new(RequestFunction::Decide, invalid.call.arguments.clone());
    assert_eq!(
        invalid.admit().unwrap_err().detail().message(),
        "files_only requires filter source input"
    );
    assert_eq!(advanced.get(), 0);
    assert_eq!(listener.count(), 0);
}

#[test]
fn file_filter_retains_caller_cancellation_before_reading_records() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let request = request().admit().unwrap();
    let token = CancelToken::new();
    token.cancel();
    let advanced = Cell::new(0);
    let error = engine
        .execute_request(
            &request,
            RequestEnvironment {
                controls: CallOptions::new().cancel(&token),
                feed: Some(RequestFeed::from_records(
                    "located",
                    std::iter::once_with(|| {
                        advanced.set(advanced.get() + 1);
                        Ok(row("unopened/a.txt", 7))
                    }),
                )),
            },
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Cancelled);
    assert_eq!(advanced.get(), 0);
    assert_eq!(listener.count(), 0);
}

#[test]
fn eager_file_filter_refuses_unlocated_composed_originals_without_sending() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let unlocated = RecordReading::new(&[], None, None)
        .unwrap()
        .compose(RawRecord::text("Beta.").unwrap())
        .unwrap()
        .map_original(|record| record.question_input());
    let error = engine
        .execute_request(
            &request().admit().unwrap(),
            RequestEnvironment {
                controls: CallOptions::new(),
                feed: Some(
                    RequestFeed::from_records(
                        "located",
                        [Ok(row("unopened/a.txt", 7)), Ok(unlocated)].into_iter(),
                    )
                    .eager(),
                ),
            },
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.detail().message(),
        "files_only requires filter source input"
    );
    assert_eq!(listener.count(), 0);
}
