#![cfg(test)]
//! A caller drives one bounded operation and owns its terminal failure.

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use thinkthen::{Cancel, Client, OffsetUnit, PullEvent, SdkRequest};

fn client(server: &Loopback) -> Client {
    Client::from_json(
        &json!({"url":server.base(),"batch":1}).to_string(),
        |name| (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned()),
    )
    .unwrap()
}

#[test]
fn pulled_records_preserve_raw_text_structured_ids_and_report_failure_after_all_rows() {
    let server = Loopback::noul(0.9).unwrap();
    let client = client(&server);
    let request = SdkRequest::from_json(r#"{"function":"decide","question":{"type":"inline","value":"Ready?"},"options":{"on":"/body"}}"#).unwrap();
    let mut call = client
        .open(&request, OffsetUnit::CodePoints, Cancel::default())
        .unwrap();
    assert_eq!(
        call.send_text("too early").unwrap_err().kind().code(),
        "usage.record"
    );
    let mut sent = 0;
    let mut rows = Vec::new();
    loop {
        match call.next_event() {
            PullEvent::NeedsInput => {
                match sent {
                    0 => call.send_text("raw \"quoted\" 😀\n").unwrap(),
                    1 => call
                        .send_json(r#"{"id":{"start":4,"end":8},"body":"good"}"#)
                        .unwrap(),
                    2 => call.send_json(r#"{"wrong":"missing"}"#).unwrap(),
                    _ => call.end_input(),
                }
                sent += 1;
            }
            PullEvent::Row(row) => rows.push(serde_json::from_str::<Value>(&row.json).unwrap()),
            PullEvent::Finished => break,
        }
    }
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0]["answer"], true);
    assert_eq!(rows[1]["id"], json!({"start":4,"end":8}));
    assert_eq!(rows[2]["error"]["code"], "usage.missing");
    assert_eq!(
        call.final_error().map(|e| e.kind().code()),
        Some("usage.missing")
    );
    call.cancel();
    assert!(matches!(call.next_event(), PullEvent::Finished));
    assert_eq!(
        call.final_error().map(|e| e.kind().code()),
        Some("usage.missing")
    );
    assert_eq!(server.count(), 2);
}

#[test]
fn every_offset_unit_applies_to_answers_and_details_including_annotation_pointers() {
    let server = Loopback::answering(|_| json!({"type":"choice","probabilities":{"BEGIN":0,"INSIDE":0,"END":0,"SINGLE":1,"OUT":0}})).unwrap();
    let client = client(&server);
    for (unit, start, end, emoji_end) in [
        (OffsetUnit::CodePoints, 2, 5, 1),
        (OffsetUnit::Utf16, 3, 6, 2),
        (OffsetUnit::Utf8, 5, 8, 4),
    ] {
        for annotate in [false, true] {
            let request = if annotate {
                json!({"function":"annotate","question":{"type":"inline","value":{"questions":{"names":{"recognize":[],"on":"/body"}}}},"records":[{"body":"😀 Ada"}]})
            } else {
                json!({"function":"recognize","question":{"type":"inline","value":[]},"records":["😀 Ada"]})
            };
            let mut call = client
                .open(
                    &SdkRequest::from_json(&request.to_string()).unwrap(),
                    unit,
                    Cancel::default(),
                )
                .unwrap();
            let mut count = 0;
            loop {
                match call.next_event() {
                    PullEvent::NeedsInput => call.end_input(),
                    PullEvent::Row(row) => {
                        count += 1;
                        let value: Value = serde_json::from_str(&row.json).unwrap();
                        let (answer, detail) = if annotate {
                            (
                                value["answer"]["names"]["answer"].clone(),
                                value["answer"]["names"]["detail"].clone(),
                            )
                        } else {
                            (
                                value["answer"].clone(),
                                serde_json::from_str::<Value>(row.detail.as_ref().unwrap())
                                    .unwrap()["probabilities"]
                                    .clone(),
                            )
                        };
                        assert_eq!(answer.as_array().unwrap().len(), 1);
                        assert_eq!(answer[0]["text"], "Ada");
                        assert_eq!(answer[0]["start"], start);
                        assert_eq!(answer[0]["end"], end);
                        assert_eq!(detail["pieces"][0]["end"], emoji_end);
                    }
                    PullEvent::Finished => break,
                }
            }
            assert_eq!(count, 1);
            assert!(call.final_error().is_none());
        }
    }
    assert_eq!(server.count(), 6);
}

#[test]
fn filters_keep_passing_records_before_reporting_failure_and_cancel_stops_pending_input() {
    let server = Loopback::noul(0.9).unwrap();
    let client = client(&server);
    let request = SdkRequest::from_json(r#"{"function":"filter","question":{"type":"inline","value":"Ready?"},"options":{"on":"/body"}}"#).unwrap();
    let mut call = client
        .open(&request, OffsetUnit::CodePoints, Cancel::default())
        .unwrap();
    let mut input = [r#"{"body":"first"}"#, r#"{}"#, r#"{"body":"last"}"#].into_iter();
    let mut kept = Vec::new();
    let mut count = 0;
    loop {
        match call.next_event() {
            PullEvent::NeedsInput => match input.next() {
                Some(record) => call.send_json(record).unwrap(),
                None => call.end_input(),
            },
            PullEvent::Row(row) => {
                count += 1;
                if row.kept {
                    kept.push(row.position);
                }
                assert!(call.final_error().is_none());
            }
            PullEvent::Finished => break,
        }
    }
    assert_eq!(count, 3);
    assert_eq!(kept, [1, 3]);
    assert_eq!(call.final_error().unwrap().kind().code(), "usage.missing");
    assert_eq!(
        call.final_error().unwrap().kind().family().as_str(),
        "usage"
    );
    let mut pending = client
        .open(&request, OffsetUnit::CodePoints, Cancel::default())
        .unwrap();
    assert!(matches!(pending.next_event(), PullEvent::NeedsInput));
    assert!(pending.poll(std::time::Duration::ZERO).is_none());
    pending.cancel();
    assert!(matches!(pending.next_event(), PullEvent::Finished));
    assert_eq!(
        pending.final_error().unwrap().kind().code(),
        "local.cancelled"
    );
    assert_eq!(
        pending.final_error().unwrap().kind().family().as_str(),
        "local"
    );
    assert_eq!(server.count(), 2);
}

#[test]
fn host_utf16_admission_preserves_unicode_and_refuses_unpaired_surrogates() {
    assert_eq!(thinkthen::utf16_text(&[0xd83d, 0xde00, 65]).unwrap(), "😀A");
    for invalid in [&[0xd800][..], &[0xdc00][..], &[0xd800, 65][..]] {
        assert_eq!(
            thinkthen::utf16_text(invalid).unwrap_err().kind().code(),
            "usage.record"
        );
    }
}
