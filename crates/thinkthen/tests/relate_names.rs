#![cfg(test)]
//! One relation invocation supplies every recognized name as well as its relations.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback};
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use thinkthen::{Key, OffsetUnit, Recognize, Relate, RelationRule, Request, Settings};

#[test]
fn relate_returns_all_recognized_names_without_a_second_recognition_call() {
    for (text, expected_names, asks, pairs, kept, requests) in [
        (
            "😀 Ada joined Acme Bea Ada",
            &["Ada", "Acme", "Bea", "Ada"][..],
            10,
            2,
            1,
            3,
        ),
        (
            "😀 Ada joined Bea Ada",
            &["Ada", "Bea", "Ada"][..],
            8,
            0,
            0,
            2,
        ),
    ] {
        for unit in [
            OffsetUnit::Utf8,
            OffsetUnit::Utf16,
            OffsetUnit::CodePoints,
            OffsetUnit::CodePointsInclusive,
        ] {
            let recognition = Arc::new(AtomicUsize::new(0));
            let observed = recognition.clone();
            let server = Loopback::answering(move |q| {
            let words = q["instructions"].as_str().unwrap();
            if q["criteria"].get("BEGIN").is_some() {
                observed.fetch_add(1, Ordering::SeqCst);
                let named = ["Ada", "Bea", "Acme"].iter().any(|name| words.contains(&format!("[[{name}]]")));
                json!({"type":"choice","probabilities":{"BEGIN":0.,"INSIDE":0.,"END":0.,"SINGLE":if named {1.} else {0.},"OUT":if named {0.} else {1.}}})
            } else if q.get("criteria").is_some() {
                observed.fetch_add(1, Ordering::SeqCst);
                let company = words.contains("[[Acme]]");
                json!({"type":"choice","probabilities":{"person":if company {0.} else {1.},"company":if company {1.} else {0.},"none of these":0.}})
            } else {
                json!({"type":"noul","noul":if words.contains(r#""text":"Bea""#) {0.1} else {0.9}})
            }
        }).unwrap();
            let engine = Settings::default()
                .url(server.base())
                .api_key(Key::new(KEY).unwrap())
                .engine(|_| None)
                .unwrap();
            let relate =
                Relate::new([RelationRule::new("works_for", "person", "company").unwrap()])
                    .unwrap();
            let rows: Vec<_> = engine
                .open_request(&Request::new(relate).with_records([json!(text)]), unit)
                .unwrap()
                .rows()
                .map(|row| serde_json::to_value(row.unwrap()).unwrap())
                .collect();
            assert_eq!(rows.len(), 1);
            let row = &rows[0];
            assert_eq!(row["status"], "answered");
            assert_eq!(server.count(), requests);
            assert_eq!(recognition.load(Ordering::SeqCst), asks);
            assert_eq!(row["answer"].as_array().unwrap().len(), kept);
            assert_eq!(row["detail"]["pairs"].as_array().unwrap().len(), pairs);
            let actual_names = row["detail"]["names"].as_array().unwrap();
            assert_eq!(actual_names.len(), expected_names.len());
            for (actual, expected) in actual_names.iter().zip(expected_names) {
                assert_eq!(actual["text"], *expected);
                assert_eq!(actual["probability"], 1.0);
            }
            let recognize = Recognize::new(["person", "company"]).unwrap();
            let names: Vec<_> = engine
                .open_request(&Request::new(recognize).with_records([json!(text)]), unit)
                .unwrap()
                .rows()
                .map(|row| serde_json::to_value(row.unwrap()).unwrap())
                .collect();
            assert_eq!(names.len(), 1);
            assert_eq!(row["detail"]["names"], names[0]["answer"]);
            assert_eq!(server.count(), requests + 2);
            assert_eq!(recognition.load(Ordering::SeqCst), asks * 2);
        }
    }
}
