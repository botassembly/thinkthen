#![cfg(test)]
//! Push input and typed pulled rows preserve record and cancellation boundaries.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback};
use serde_json::json;
use thinkthen::{Decide, Engine, Filter, Key, OffsetUnit, Question, Record, Request, Settings};

fn engine(server: &Loopback) -> Engine {
    Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .batch("1".parse().unwrap())
        .engine(|_| None)
        .unwrap()
}

#[test]
fn pulled_records_preserve_raw_text_exact_ids_and_independent_record_failures() {
    let server = Loopback::noul(0.9).unwrap();
    let call = engine(&server)
        .open(
            &Decide::new("Ready?")
                .unwrap()
                .evidence("/body".parse().unwrap()),
        )
        .unwrap();
    let sender = call.sender();
    let worker = std::thread::spawn(move || {
        sender.send(Record::text("raw \"quoted\" 😀\n")).unwrap();
        sender
            .send(Record::from_json(
                r#"{"id":18446744073709551615,"body":"good"}"#,
            ))
            .unwrap();
        sender
            .send(Record::from_json(r#"{"wrong":"missing"}"#))
            .unwrap();
        sender.close();
    });
    let mut rows = call.rows();
    let actual: Vec<_> = rows.by_ref().collect::<Result<Vec<_>, _>>().unwrap();
    worker.join().unwrap();
    assert_eq!(actual.len(), 3);
    assert_eq!(actual[0].clone().into_result().unwrap(), Some(true));
    assert_eq!(actual[1].id().unwrap().as_u64(), Some(u64::MAX));
    assert_eq!(actual[2].error().unwrap().code().as_str(), "record.missing");
    rows.cancel();
    assert!(rows.next().is_none());
    assert_eq!(server.count(), 2);
}

#[test]
fn every_offset_unit_applies_to_answers_and_details_including_annotation_pointers() {
    let server = Loopback::answering(|q| {
        if q["criteria"].get("BEGIN").is_some() {
            json!({"type":"choice","probabilities":{"BEGIN":0,"INSIDE":0,"END":0,"SINGLE":1,"OUT":0}})
        } else { json!({"type":"choice","probabilities":{"person":1,"none of these":0}}) }
    }).unwrap();
    let engine = engine(&server);
    for (unit, start, end, emoji_end) in [
        (OffsetUnit::CodePoints, 2, 5, 1),
        (OffsetUnit::Utf16, 3, 6, 2),
        (OffsetUnit::Utf8, 5, 8, 4),
    ] {
        for annotate in [false, true] {
            let request = if annotate {
                json!({"function":"annotate","questions":{"names":{"function":"recognize","kinds":["person"],"evidence":"/body"}},"records":[{"body":"😀 Ada"}]})
            } else {
                json!({"function":"recognize","kinds":["person"],"records":["😀 Ada"]})
            };
            let mut question = request.clone();
            let records = question.as_object_mut().unwrap().remove("records").unwrap();
            let request = Request::new(Question::from_json(&question.to_string()).unwrap())
                .with_records(records.as_array().unwrap().clone());
            let call = engine.open_request(&request, unit).unwrap();
            call.sender().close();
            let actual = call.rows().collect::<Result<Vec<_>, _>>().unwrap();
            assert_eq!(actual.len(), 1);
            let value = serde_json::to_value(&actual[0]).unwrap();
            let row = if annotate {
                &value["answer"]["names"]
            } else {
                &value
            };
            assert_eq!(row["answer"].as_array().unwrap().len(), 1);
            assert_eq!(row["answer"][0]["text"], "Ada");
            assert_eq!(row["answer"][0]["start"], start);
            assert_eq!(row["answer"][0]["end"], end);
            assert_eq!(row["detail"]["pieces"][0]["end"], emoji_end);
        }
    }
    assert_eq!(server.count(), 12);
}

#[test]
fn filters_keep_rows_after_record_failure_and_cancel_wakes_pending_input() {
    let server = Loopback::noul(0.9).unwrap();
    let engine = engine(&server);
    let question = Filter::new("Ready?")
        .unwrap()
        .evidence("/body".parse().unwrap());
    let call = engine.open(&question).unwrap();
    let sender = call.sender();
    let worker = std::thread::spawn(move || {
        for record in [r#"{"body":"first"}"#, r#"{}"#, r#"{"body":"last"}"#] {
            sender.send(Record::from_json(record)).unwrap();
        }
        sender.close();
    });
    let actual = call.rows().collect::<Result<Vec<_>, _>>().unwrap();
    worker.join().unwrap();
    assert_eq!(actual.len(), 3);
    assert_eq!(
        actual
            .iter()
            .filter(|row| row.kept())
            .map(|row| row.position())
            .collect::<Vec<_>>(),
        [1, 3]
    );
    assert_eq!(actual[1].error().unwrap().code().as_str(), "record.missing");
    let pending = engine.open(&question).unwrap();
    let sender = pending.sender();
    let mut rows = pending.rows();
    rows.cancel();
    assert_eq!(
        rows.next().unwrap().unwrap_err().code().as_str(),
        "local.cancelled"
    );
    assert!(rows.next().is_none());
    assert_eq!(
        sender
            .send(Record::text("late"))
            .unwrap_err()
            .code()
            .as_str(),
        "local.cancelled"
    );
    assert_eq!(server.count(), 2);
}

#[test]
fn host_utf16_admission_preserves_unicode_and_refuses_unpaired_surrogates() {
    let server = Loopback::noul(0.9).unwrap();
    let question = Decide::new("Ready?").unwrap();
    let engine = engine(&server);
    let row = engine
        .run(&question, Record::from_utf16(&[0xd83d, 0xde00, 65]))
        .unwrap();
    assert_eq!(row.into_result().unwrap(), Some(true));
    assert!(
        server.requests()[0]
            .json()
            .unwrap()
            .to_string()
            .contains("😀A")
    );
    for invalid in [&[0xd800][..], &[0xdc00][..], &[0xd800, 65][..]] {
        assert_eq!(
            engine
                .run(&question, Record::from_utf16(invalid))
                .unwrap()
                .error()
                .unwrap()
                .code()
                .as_str(),
            "record.invalid"
        );
    }
    assert_eq!(server.count(), 1);
}

#[test]
fn object_field_on_array_reports_the_stream_form_without_rejecting_array_records() {
    let server = Loopback::noul(0.9).unwrap();
    let engine = engine(&server);
    let array = r#"[{"text":"first"},{"text":"second"}]"#;
    let question = Decide::new("Ready?").unwrap();
    let row = engine
        .run(
            &question.clone().evidence("/text".parse().unwrap()),
            Record::from_json(array),
        )
        .unwrap();
    let error = row.error().unwrap();
    assert_eq!(error.code().as_str(), "record.missing");
    assert_eq!(
        error.message(),
        "the record is an array of 2 values and holds nothing at /text. Use the stream form of the call to judge each value"
    );
    assert_eq!(server.count(), 0);
    for pointer in ["", "/0/text"] {
        let row = engine
            .run(
                &question.clone().evidence(pointer.parse().unwrap()),
                Record::from_json(array),
            )
            .unwrap();
        assert!(row.error().is_none());
    }
    let row = engine
        .run(
            &question.evidence("/2".parse().unwrap()),
            Record::from_json(array),
        )
        .unwrap();
    assert_eq!(
        row.error().unwrap().message(),
        "the record holds nothing at /2"
    );
    assert_eq!(server.count(), 2);
}
