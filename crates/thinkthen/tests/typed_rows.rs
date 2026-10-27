#![cfg(test)]
//! The canonical row contract distinguishes record failures from call failures.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use serde_json::json;
use thinkthen::records::{Record, RecordId};
use thinkthen::results::{AnyRow, Outcome, Status};
use thinkthen::{BackendName, Code, Settings, questions};

#[test]
fn record_failures_keep_positions_and_exact_ids_without_becoming_call_errors() {
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let question = questions::Decide::new("Ready?").unwrap();
    let records = vec![
        Record::text("ready").with_id(RecordId::from("s1")),
        Record::missing(),
        Record::json(json!({"id":18446744073709551615_u64,"body":"ready"})),
        Record::json(json!({"id":true,"body":"ready"})),
        Record::json(json!(null)),
        Record::from_json("{"),
    ];
    let rows: Vec<_> = engine
        .rows(&question, records)
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(rows.len(), 6);
    for (index, row) in rows.iter().enumerate() {
        assert_eq!(row.position(), (index + 1) as u64);
    }
    assert_eq!(rows[0].id(), Some(&RecordId::from("s1")));
    assert_eq!(rows[2].id().unwrap().as_u64(), Some(u64::MAX));
    for (index, code) in [
        (1, Code::RecordMissing),
        (3, Code::RecordInvalid),
        (4, Code::RecordMissing),
        (5, Code::RecordJson),
    ] {
        assert_eq!(rows[index].status(), Status::Failed);
        assert_eq!(rows[index].clone().into_result().unwrap_err().code(), code);
        let value = serde_json::to_value(&rows[index]).unwrap();
        assert_eq!(value["error"]["code"], code.as_str());
        assert!(value.get("answer").is_none());
        assert!(value.get("probability").is_none());
        assert_eq!(value["source"]["backend"], "stub");
    }
    assert!(matches!(rows[0].outcome(), Outcome::Answered { .. }));
    let row = engine.run(&question, Record::missing()).unwrap();
    assert_eq!(row.status(), Status::Failed);
    let encoded = serde_json::to_string(&rows[0]).unwrap();
    let restored: AnyRow = serde_json::from_str(&encoded).unwrap();
    assert_eq!(
        serde_json::to_value(restored).unwrap(),
        serde_json::to_value(&rows[0]).unwrap()
    );
}

#[test]
fn push_close_does_not_wait_for_output_and_filter_keeps_failed_originals() {
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let question = questions::Decide::new("Ready?").unwrap();
    let call = engine.open(&question).unwrap();
    let sender = call.sender();
    for _ in 0..5 {
        sender.send(Record::missing()).unwrap();
    }
    sender.close();
    let rows: Vec<_> = call.rows().collect::<Result<_, _>>().unwrap();
    assert_eq!(rows.len(), 5);
    let filter = questions::Filter::new("Keep?").unwrap();
    let rows: Vec<_> = engine
        .filter_items(&filter, vec!["first", "missing", "last"], |item| {
            if *item == "missing" {
                Record::missing()
            } else {
                Record::text(*item)
            }
        })
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(
        rows.iter().map(|(item, _)| *item).collect::<Vec<_>>(),
        ["first", "missing", "last"]
    );
    assert_eq!(rows[1].1.status(), Status::Failed);
    assert!(!rows[1].1.kept());
}

#[test]
fn cancellation_releases_a_push_sender_and_finishes_once_without_poisoning_the_engine() {
    use loopback::{HeldReply, KEY, Loopback};
    use std::time::Duration;
    use thinkthen::{Key, Retry};
    let (server, arrived, closed) = Loopback::holding(HeldReply::Headers).unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .batch(1_u64.try_into().unwrap())
        .retry(Retry::new(0, Duration::ZERO, Duration::from_secs(30)).unwrap())
        .engine(|_| None)
        .unwrap();
    let question = questions::Decide::new("Ready?").unwrap();
    let call = engine.open(&question).unwrap();
    let sender = call.sender();
    let mut rows = call.rows();
    let producer = std::thread::spawn(move || {
        let mut sent = 0;
        for _ in 0..10000 {
            if sender.send(Record::text("evidence")).is_err() {
                return sent;
            }
            sent += 1;
        }
        sent
    });
    arrived.recv_timeout(Duration::from_secs(10)).unwrap();
    rows.cancel();
    assert!(producer.join().unwrap() < 10000);
    assert_eq!(
        rows.next().unwrap().unwrap_err().code(),
        Code::LocalCancelled
    );
    assert!(rows.next().is_none());
    assert!(matches!(
        closed.recv_timeout(Duration::from_secs(10)).unwrap(),
        Ok(0) | Err(_)
    ));
    assert_eq!(
        engine.run(&question, Record::missing()).unwrap().status(),
        Status::Failed
    );
}

#[test]
fn push_rows_are_published_before_the_input_owner_closes() {
    use std::{sync::mpsc, time::Duration};
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let call = engine
        .open(&questions::Decide::new("Ready?").unwrap())
        .unwrap();
    let sender = call.sender();
    let mut rows = call.rows();
    sender.send(Record::missing()).unwrap();
    let (send, receive) = mpsc::channel();
    let consumer = std::thread::spawn(move || {
        let first = rows.next();
        let _ = send.send(first);
        rows.collect::<Vec<_>>()
    });
    let first = receive.recv_timeout(Duration::from_secs(10));
    sender.close();
    let remaining = consumer.join().unwrap();
    assert_eq!(
        first
            .expect("a pushed row must arrive while input remains open")
            .unwrap()
            .unwrap()
            .status(),
        Status::Failed
    );
    assert!(remaining.is_empty());
}

#[test]
fn a_producer_fault_preserves_published_rows_and_ends_with_one_call_error() {
    use std::sync::mpsc;
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .batch(1_u64.try_into().unwrap())
        .engine(|_| None)
        .unwrap();
    let (release, wait) = mpsc::channel();
    let records = std::iter::once(Record::missing()).chain(std::iter::from_fn(move || {
        wait.recv().unwrap();
        panic!("synthetic iterator failure")
    }));
    let mut rows = engine
        .rows(&questions::Decide::new("Ready?").unwrap(), records)
        .unwrap();
    assert_eq!(rows.next().unwrap().unwrap().status(), Status::Failed);
    release.send(()).unwrap();
    assert_eq!(rows.next().unwrap().unwrap_err().code(), Code::InternalBug);
    assert!(rows.next().is_none());
}

#[test]
fn the_last_sender_owner_closes_input_and_a_general_toolbox_can_execute_rank() {
    use std::collections::BTreeMap;
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let call = engine
        .open(&questions::Decide::new("Ready?").unwrap())
        .unwrap();
    let sender = call.sender();
    let last = sender.clone();
    let rows = call.rows();
    drop(sender);
    last.send(Record::missing()).unwrap();
    drop(last);
    assert_eq!(rows.collect::<Result<Vec<_>, _>>().unwrap().len(), 1);
    let toolbox = questions::Toolbox::new(BTreeMap::from([(
        "priority".to_owned(),
        questions::Rank::new("How much?", ["low", "high"])
            .unwrap()
            .into(),
    )]))
    .unwrap();
    let request = thinkthen::Request::new(toolbox.get("priority").unwrap().clone())
        .with_records([json!("a"), json!("b")]);
    let call = engine
        .open_request(&request, thinkthen::OffsetUnit::CodePoints)
        .unwrap();
    call.sender().close();
    let rows: Vec<_> = call.rows().collect::<Result<_, _>>().unwrap();
    assert_eq!(rows.len(), 2);
    assert!(
        rows.iter()
            .all(|row| row.function() == thinkthen::FunctionName::Rank)
    );
}

#[test]
fn host_conversion_failures_are_record_rows_and_transport_retains_its_cause() {
    use loopback::{KEY, UnavailableBackend};
    use std::{error::Error as _, time::Duration};
    use thinkthen::{Key, Retry};
    struct CannotSerialize;
    impl serde::Serialize for CannotSerialize {
        fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
            Err(serde::ser::Error::custom(
                "synthetic host conversion failure",
            ))
        }
    }
    let unavailable = UnavailableBackend::new().unwrap();
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        let engine = Settings::default()
            .with_backend(backend)
            .url(unavailable.base())
            .api_key(Key::new(KEY).unwrap())
            .retry(Retry::new(0, Duration::ZERO, Duration::from_secs(5)).unwrap())
            .engine(|_| None)
            .unwrap();
        let question = questions::Decide::new("Ready?").unwrap();
        let rows: Vec<_> = engine
            .rows(
                &question,
                [
                    Record::from_serialize(&CannotSerialize),
                    Record::from_utf16(&[0xd800]),
                ],
            )
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(rows.len(), 2);
        for row in rows {
            assert_eq!(row.error().unwrap().code(), Code::RecordInvalid);
            assert_eq!(row.source().backend(), backend);
        }
        let row = engine.run(&question, Record::text("ready")).unwrap();
        let error = row.error().unwrap();
        assert_eq!(error.code(), Code::BackendTransport);
        assert!(error.source().is_some());
        assert_eq!(
            serde_json::to_value(error).unwrap()["code"],
            "backend.transport"
        );
    }
}

#[test]
fn separately_admitted_ids_reach_evidence_before_record_admission() {
    use loopback::{KEY, Loopback};
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        let server=Loopback::case(&json!({"replies":[{"contains":"18446744073709551615","probability":0.9},{"probability":0.1}]})).unwrap();
        let engine = Settings::default()
            .with_backend(backend)
            .url(server.base())
            .api_key(KEY.parse().unwrap())
            .engine(|_| None)
            .unwrap();
        let question = questions::Decide::new("Ready?")
            .unwrap()
            .evidence("/id".parse().unwrap());
        for id in [
            RecordId::integer_decimal("18446744073709551615").unwrap(),
            RecordId::from("18446744073709551615"),
        ] {
            let row = engine
                .run(
                    &question,
                    Record::from_json_with_id(r#"{"id":"wrong","body":"evidence"}"#, id.clone()),
                )
                .unwrap();
            assert_eq!(row.id(), Some(&id));
            assert_eq!(row.probability().unwrap().get(), 0.9);
        }
        assert_eq!(server.count(), 2);
    }
}

#[test]
fn abandoned_calls_do_not_cancel_the_attached_external_signal() {
    let external = thinkthen::Cancel::default();
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap()
        .cancel(external.clone());
    let question = questions::Decide::new("Ready?").unwrap();
    let call = engine.open(&question).unwrap();
    drop(call.rows());
    assert!(!external.is_cancelled());
    assert!(
        engine
            .run(&question, Record::text("evidence"))
            .unwrap()
            .error()
            .is_none()
    );
    let call = engine.open(&question).unwrap();
    let mut rows = call.rows();
    external.cancel();
    assert_eq!(
        rows.next().unwrap().unwrap_err().code(),
        Code::LocalCancelled
    );
    assert!(rows.next().is_none());
    assert!(external.is_cancelled());
}
