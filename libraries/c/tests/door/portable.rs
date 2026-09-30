//! The shared literal batch corpus through the public C JSON door. The content
//! cut is gone, by ADR 0111, so every record rides one request, and each row's
//! key is the key of its fixture question.

use conformance_backend::{Canned, Listener};

use super::{Script, keys, replies};
use crate::{compile, crate_dir, run, text};

const CORPUS: &str =
    include_str!("../../../../specification/fixtures/batching/portable-records.json");
const BODIES: [&str; 3] = [
    include_str!("../../../../specification/fixtures/batching/portable-1.request.json"),
    include_str!("../../../../specification/fixtures/batching/portable-2.request.json"),
    include_str!("../../../../specification/fixtures/batching/portable-3.request.json"),
];

#[test]
fn c_json_records_keep_fixture_questions_and_keys_in_one_request() {
    let corpus: serde_json::Value = serde_json::from_str(CORPUS).expect("literal corpus");
    let listener = Listener::answering(|body| {
        let request: serde_json::Value = serde_json::from_slice(body).expect("request JSON");
        let questions = request["questions"].as_object().expect("wire questions");
        let answers: serde_json::Map<String, serde_json::Value> = questions
            .keys()
            .map(|name| (name.clone(), serde_json::json!({"type":"noul","noul":0.9})))
            .collect();
        Canned::ok(&serde_json::json!({"model":"jev-1.13.0","answers":answers}).to_string())
    })
    .expect("listener");
    let base = listener.base();
    let settings =
        serde_json::json!({"base_url":base,"cache":false,"throttle":1,"max_retries":0}).to_string();
    let call = serde_json::json!({"decide":"Is it relevant?","records":corpus["texts"],
        "details":true,"call":{"batch":"max"}})
    .to_string();
    let mut script = Script::default();
    script.ask("settings", &[base, &settings]);
    script.ask("call", &[base, &call]);
    let output = run(
        &compile(&crate_dir().join("tests/c/driver.c")),
        "",
        &script.0,
    );
    assert!(output.status.success(), "{}", text(&output.stderr));
    let replies = replies(&output.stdout).expect("driver replies");
    assert_eq!(replies.len(), 2);
    assert_eq!(replies[0].0, 0);
    assert_eq!(replies[1].0, 0, "{}", replies[1].1);
    let result: serde_json::Value = serde_json::from_str(&replies[1].1).expect("call result");
    let rows = result["value"].as_array().expect("five rows");
    assert_eq!(rows.len(), 5);
    assert_eq!(result["facts"]["requests_sent"], 1);
    let sent = listener.requests();
    assert_eq!(sent.len(), 1, "no content cut remains, by ADR 0111");
    let body: serde_json::Value = serde_json::from_slice(&sent[0].body).expect("request");
    let fixture: Vec<serde_json::Value> = BODIES
        .iter()
        .flat_map(|literal| {
            let literal: serde_json::Value = serde_json::from_str(literal).expect("fixture");
            (1..=literal["questions"]
                .as_object()
                .map_or(0, serde_json::Map::len))
                .map(move |place| literal["questions"][format!("q{place}")].clone())
        })
        .collect();
    let questions: Vec<serde_json::Value> = (1..=5)
        .map(|place| body["questions"][format!("q{place}")].clone())
        .collect();
    assert_eq!(questions, fixture, "each question keeps the fixture bytes");
    let keys = keys(listener.url(), &sent[0].body).expect("question keys");
    for (at, row) in rows.iter().enumerate() {
        assert_eq!(row["input"], corpus["texts"][at]);
        assert_eq!(row["meta"]["requests"], serde_json::json!([keys[at]]));
        assert!(row["meta"].get("batch").is_none(), "{at}");
    }
}
