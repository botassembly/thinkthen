#![cfg(test)]
//! Filtering through the public Rust API.

use loopback::{KEY, Loopback};
use serde_json::json;
use thinkthen::{
    Backend, Cancel, Client, Engine, Filter, OffsetUnit, PullEvent, Record, SdkRequest,
};

#[test]
fn filter_emits_passing_records_and_preserves_its_first_failure_through_both_apis() {
    let server = Loopback::answering(|question| {
        let text = question["instructions"].as_str().unwrap();
        let p = if text.contains("yes") {
            0.9
        } else if text.contains("maybe") {
            0.5
        } else if text.contains("broken") {
            2.0
        } else {
            0.1
        };
        json!({"type": "noul", "noul": p})
    })
    .unwrap();
    let backend = Backend::resolve(Some(server.base()), None, |name| {
        (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
    })
    .unwrap();
    let engine = Engine::new(backend).batch("2".parse().unwrap());
    let question = Filter::new("Keep?")
        .unwrap()
        .threshold("0.2:0.8".parse().unwrap());
    let original = ["yes first", "no", "maybe", "broken", "yes last"];
    let mut kept = Vec::new();
    let result = engine.filter(
        &question,
        original
            .map(|text| (Record::text(text), text))
            .into_iter()
            .chain([(Record::json("invalid", &Default::default()), "invalid")]),
        |text| kept.push(text),
    );
    assert_eq!(kept, ["yes first", "yes last"]);
    assert_eq!(result.unwrap_err().kind().code(), "backend.reply");

    let client = Client::from_json(
        &json!({"url":server.base(),"batch":2}).to_string(),
        |name| (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned()),
    )
    .unwrap();
    let request = SdkRequest::from_json(r#"{"function":"filter","question":{"type":"inline","value":"Keep?"},"options":{"threshold":"0.2:0.8"}}"#).unwrap();
    let mut call = client
        .open(&request, OffsetUnit::CodePoints, Cancel::default())
        .unwrap();
    let mut input = original.into_iter();
    let mut sent_invalid = false;
    let mut kept = Vec::new();
    let mut rows = 0;
    loop {
        match call.next_event() {
            PullEvent::NeedsInput => match input.next() {
                Some(text) => call.send_text(text).unwrap(),
                None if !sent_invalid => {
                    call.send_json("invalid").unwrap();
                    sent_invalid = true;
                }
                None => call.end_input(),
            },
            PullEvent::Row(row) => {
                rows += 1;
                if row.kept {
                    kept.push(row.position);
                }
                assert!(call.final_error().is_none());
            }
            PullEvent::Finished => break,
        }
    }
    assert_eq!(rows, 6);
    assert_eq!(kept, [1, 5]);
    assert_eq!(call.final_error().unwrap().kind().code(), "backend.reply");
}
