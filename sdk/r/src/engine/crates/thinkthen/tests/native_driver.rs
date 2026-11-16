#![cfg(test)]
//! Native host polling reserves input space and shares ordinary row execution.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

#[cfg(feature = "native")]
mod tests {
    use thinkthen::{
        BackendName, Code, OffsetUnit, Request, Settings,
        native::{self, Call, Event},
        questions,
        records::Record,
    };
    #[test]
    fn native_poll_fills_and_drains_bounded_queues_without_losing_rows() {
        let engine = Settings::default()
            .with_backend(BackendName::Stub)
            .batch(32_u64.try_into().unwrap())
            .engine(|_| None)
            .unwrap();
        let request = Request::new(questions::Decide::new("Ready?").unwrap());
        let mut call = Call::open(&engine, &request, OffsetUnit::CodePoints).unwrap();
        assert_eq!(
            call.send(Record::missing()).unwrap_err().code(),
            Code::UsageAbi
        );
        let mut sent = 0;
        let mut seen = 0;
        let count = 257;
        loop {
            match call.next_event().unwrap() {
                Event::NeedsInput => {
                    if sent == count {
                        call.end_input();
                    } else {
                        call.send(if sent % 3 == 0 {
                            Record::missing()
                        } else {
                            Record::text("evidence")
                        })
                        .unwrap();
                        sent += 1;
                    }
                }
                Event::Row(row) => {
                    seen += 1;
                    assert_eq!(row.position(), seen);
                    if seen % 3 == 1 {
                        assert_eq!(row.error().unwrap().code(), Code::RecordMissing);
                    }
                }
                Event::Finished => break,
                _ => panic!("unknown native event"),
            }
        }
        assert_eq!(sent, count);
        assert_eq!(seen, count);
        assert!(matches!(call.next_event().unwrap(), Event::Finished));
    }
    #[test]
    fn native_version_admission_requires_the_major_and_minimum_minor() {
        assert!(native::check_version(0x20001, 0x20002).is_ok());
        for actual in [0x10001, 0x30001, 0x20000] {
            assert_eq!(
                native::check_version(0x20001, actual).unwrap_err().code(),
                Code::UsageAbi
            );
        }
    }
}

#[cfg(feature = "native")]
#[test]
fn finite_prepared_records_share_one_batch_and_keep_failed_rows_on_both_wires() {
    use loopback::{KEY, Loopback};
    use thinkthen::{
        BackendName, Code, Key, OffsetUnit, Request, Settings,
        native::{Call, Event},
        records::Record,
    };
    let case: serde_json::Value = serde_json::from_str(include_str!(
        "../../../conformance/typed/exact-identifiers.json"
    ))
    .unwrap();
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        let server = Loopback::case(&case).unwrap();
        let engine = Settings::default()
            .with_backend(backend)
            .batch(32_u64.try_into().unwrap())
            .url(server.base())
            .api_key(Key::new(KEY).unwrap())
            .engine(|_| None)
            .unwrap();
        let mut request = case["request"].clone();
        let mut records: Vec<_> = request
            .as_object_mut()
            .unwrap()
            .remove("records")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .cloned()
            .map(Record::json)
            .collect();
        let admitted = records.len();
        records.push(Record::invalid());
        records.push(Record::missing());
        let request = Request::from_json(&request.to_string()).unwrap();
        let mut call =
            Call::open_records(&engine, &request, records, OffsetUnit::CodePointsInclusive)
                .unwrap();
        let mut seen = Vec::new();
        loop {
            match call.next_event().unwrap() {
                Event::Row(row) => seen.push(row),
                Event::Finished => break,
                event => panic!("finite input must not ask the host for more: {event:?}"),
            }
        }
        assert_eq!(seen.len(), admitted + 2);
        assert_eq!(seen[admitted].error().unwrap().code(), Code::RecordInvalid);
        assert_eq!(
            seen[admitted + 1].error().unwrap().code(),
            Code::RecordMissing
        );
        for (index, row) in seen.iter().enumerate() {
            assert_eq!(row.position(), index as u64 + 1);
        }
        assert_eq!(server.requests().len(), 1);
        assert!(
            Call::open_records(
                &engine,
                &request
                    .clone()
                    .with_records([serde_json::json!("duplicate")]),
                Vec::new(),
                OffsetUnit::CodePoints
            )
            .is_err()
        );
    }
}

#[cfg(feature = "native")]
#[test]
fn native_call_fault_stays_an_error_for_next_and_poll() {
    use loopback::{KEY, Loopback};
    use std::time::Duration;
    use thinkthen::{
        BackendName, Code, Key, OffsetUnit, Request, Settings,
        native::{Call, Event},
        questions,
        records::Record,
    };
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        let server = Loopback::case(&serde_json::json!({"replies":[{"probability":0.9}]})).unwrap();
        let engine = Settings::default()
            .with_backend(backend)
            .batch(1_u64.try_into().unwrap())
            .url(server.base())
            .api_key(Key::new(KEY).unwrap())
            .engine(|_| None)
            .unwrap();
        let request = Request::new(questions::Decide::new("Ready?").unwrap());
        let mut call = Call::open(&engine, &request, OffsetUnit::CodePoints).unwrap();
        assert!(matches!(call.next_event().unwrap(), Event::NeedsInput));
        call.send(Record::text("evidence")).unwrap();
        loop {
            match call.next_event().unwrap() {
                Event::Row(row) => {
                    assert!(row.error().is_none());
                    break;
                }
                Event::NeedsInput => call.end_input(),
                event => panic!("expected the published row, got {event:?}"),
            }
        }
        call.cancel();
        let failure = call.next_event().unwrap_err();
        assert_eq!(failure.code(), Code::LocalCancelled);
        for _ in 0..2 {
            let repeated = call.next_event().unwrap_err();
            assert_eq!(repeated.code(), failure.code());
            assert_eq!(repeated.message(), failure.message());
            let repeated = call.poll(Duration::ZERO).unwrap_err();
            assert_eq!(repeated.code(), failure.code());
            assert_eq!(repeated.message(), failure.message());
        }
        assert_eq!(server.count(), 1);
    }
}

#[cfg(feature = "native")]
#[test]
fn reserved_input_does_not_hide_a_backend_terminal_error() {
    use loopback::{KEY, Loopback, Reply};
    use std::sync::Mutex;
    use thinkthen::{
        Code, Key, OffsetUnit, Request, Settings,
        native::{Call, Event},
        questions,
        records::Record,
    };
    let (release, gate) = std::sync::mpsc::channel();
    let gate = Mutex::new(gate);
    let server = Loopback::start(move |_| {
        gate.lock().unwrap().recv().unwrap();
        Reply::json(401, &serde_json::json!({}))
    })
    .unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .batch(1_u64.try_into().unwrap())
        .engine(|_| None)
        .unwrap();
    let request = Request::new(questions::Decide::new("Ready?").unwrap());
    let mut call = Call::open(&engine, &request, OffsetUnit::CodePoints).unwrap();
    assert!(matches!(call.next_event().unwrap(), Event::NeedsInput));
    call.send(Record::text("first")).unwrap();
    assert!(matches!(call.next_event().unwrap(), Event::NeedsInput));
    let token = call.cancellation();
    let events = call.events();
    release.send(()).unwrap();
    loop {
        let version = events.version();
        if token.is_cancelled() {
            break;
        }
        events.wait(version);
    }
    call.send(Record::text("reserved before rejection"))
        .unwrap();
    for _ in 0..2 {
        let failure = call.next_event().unwrap_err();
        assert_eq!(failure.code(), Code::BackendStatus);
        assert_eq!(failure.http_status(), Some(401));
    }
    assert_eq!(server.count(), 1);
}
