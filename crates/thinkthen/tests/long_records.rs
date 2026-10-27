#![cfg(test)]
//! Complete answer histories across several requests in one record's round.

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback, Reply};
use serde_json::{Value, json};
use thinkthen::{BackendName, Engine, Key, Recognize, Record, Relate, RelationRule, Settings};

fn engine(server: &Loopback) -> Engine {
    Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .model("test")
        .engine(|_| None)
        .unwrap()
}

fn single() -> Value {
    json!({"type":"choice", "probabilities":{"BEGIN":0.,"INSIDE":0.,"END":0.,"SINGLE":1.,"OUT":0.}})
}

#[test]
fn a_long_recognition_round_keeps_every_name_in_text_order() {
    let words: Vec<_> = (0..2000).map(|n| format!("Name{n}")).collect();
    let text = words.join(" ");
    let server = Loopback::answering(|q| {
        if q["criteria"].get("BEGIN").is_some() {
            single()
        } else {
            json!({"type":"choice","probabilities":{"ENTITY":1.0,"none of these":0.0}})
        }
    })
    .unwrap();
    let mut rows = Vec::new();
    engine(&server)
        .rows(&Recognize::new(["ENTITY"]).unwrap(), [Record::text(&text)])
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["status"], "answered");
    let names = rows[0]["answer"].as_array().unwrap();
    assert_eq!(names.len(), words.len());
    let mut offset = 0;
    for (name, word) in names.iter().zip(words) {
        assert_eq!(
            name,
            &json!({"text":word,"start":offset,"end":offset+word.len(),"kind":"ENTITY","probability":1.})
        );
        offset += word.len() + 1;
    }
    assert!(server.count() > 1);
    assert!(
        server
            .requests()
            .iter()
            .all(|request| request.body.len() <= 96_000)
    );
}

#[test]
fn a_pair_round_spanning_two_requests_keeps_every_relation() {
    let words: Vec<_> = (0..20).map(|n| format!("Name{n}")).collect();
    let server = Loopback::answering(|question| {
        if question["criteria"].get("BEGIN").is_some() {
            single()
        } else if question["type"] == "choice" {
            json!({"type":"choice","probabilities":{"person":1.,"none of these":0.}})
        } else {
            json!({"type":"noul","noul":0.9})
        }
    })
    .unwrap();
    let question = Relate::new(vec![
        RelationRule::new("knows", "person", "person").unwrap(),
    ])
    .unwrap();
    let mut rows = Vec::new();
    engine(&server)
        .rows(&question, [Record::text(words.join(" "))])
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(rows[0]["status"], "answered");
    let relations = rows[0]["answer"].as_array().unwrap();
    let expected: Vec<_> = words
        .iter()
        .flat_map(|source| {
            words
                .iter()
                .filter(move |target| *target != source)
                .map(move |target| (source.as_str(), target.as_str()))
        })
        .collect();
    let actual: Vec<_> = relations
        .iter()
        .map(|relation| {
            assert_eq!(relation["relation"], "knows");
            assert_eq!(relation["probability"], 0.9);
            (
                relation["source"]["text"].as_str().unwrap(),
                relation["target"]["text"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(actual, expected);
    let requests = server.requests();
    let pair_requests: Vec<_> = requests
        .iter()
        .filter(|request| {
            request.json().unwrap()["questions"]
                .as_object()
                .unwrap()
                .values()
                .any(|q| q["type"] == "noul")
        })
        .collect();
    assert_eq!(pair_requests.len(), 2);
    assert!(requests.iter().all(|request| request.body.len() <= 96_000));
}

#[test]
fn one_oversized_ask_fails_its_row_before_the_round_sends() {
    let server = Loopback::noul(0.9).unwrap();
    let question =
        thinkthen::Tag::new("What applies?", ["short".to_owned(), "x".repeat(96_000)]).unwrap();
    let mut rows = Vec::new();
    engine(&server)
        .rows(&question, [Record::text("one")])
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(rows[0]["error"]["code"], "record.too-large");
    assert_eq!(server.count(), 0);
}

#[test]
fn a_failed_fragment_fails_its_record_after_collecting_the_round() {
    let server = Loopback::start(|_| Reply::json(401, &json!({}))).unwrap();
    let text = (0..2000)
        .map(|n| format!("Name{n}"))
        .collect::<Vec<_>>()
        .join(" ");
    let mut rows = Vec::new();
    engine(&server)
        .rows(&Recognize::new(["ENTITY"]).unwrap(), [Record::text(text)])
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["error"]["code"], "backend.status");
    assert!(server.count() > 1);
}

#[test]
fn one_records_labels_span_the_adapters_question_limit() {
    let server = Loopback::start(|request| {
        let body = request.json().unwrap();
        let questions = body["questions"].as_array().unwrap();
        Reply::json(200, &json!({"answers": questions.iter().map(|_| json!({"type":"predicate","probability":0.9})).collect::<Vec<_>>()}))
    }).unwrap();
    let labels: Vec<_> = (0..401).map(|n| format!("label{n}")).collect();
    let question = thinkthen::Tag::new("What applies?", labels.clone()).unwrap();
    let engine = Settings::default()
        .with_backend(BackendName::OpenAi)
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .model("test")
        .engine(|_| None)
        .unwrap();
    let mut rows = Vec::new();
    engine
        .rows(&question, [Record::text("one")])
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(rows[0]["status"], "answered");
    assert_eq!(rows[0]["answer"], json!(labels));
    let sizes: Vec<_> = server
        .requests()
        .iter()
        .map(|request| {
            request.json().unwrap()["questions"]
                .as_array()
                .unwrap()
                .len()
        })
        .collect();
    assert_eq!(sizes, [400, 1]);
}

#[test]
fn an_omitted_request_reply_fails_instead_of_reading_incomplete_answers() {
    let server = Loopback::start(|_| Reply::json(200, &json!({"answers":{}}))).unwrap();
    let question = thinkthen::Decide::new("Is it relevant?").unwrap();
    let row = engine(&server)
        .run(&question, Record::text("evidence"))
        .unwrap();
    assert_eq!(row.error().unwrap().code().as_str(), "backend.reply");
    assert_eq!(server.count(), 1);
    assert!(serde_json::to_value(row).unwrap().get("answer").is_none());
}
