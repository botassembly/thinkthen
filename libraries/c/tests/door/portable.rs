//! The shared literal batch corpus through the public C JSON door. The content
//! cut is gone, by ADR 0111, so every record rides one request, and each row's
//! key is the key of its fixture question.

use conformance_backend::{Canned, Listener};

use super::{Script, keys, replies};
use crate::{compile, crate_dir, run, text};
use serde_json::value::RawValue;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

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
    let got = replies(&output.stdout).expect("driver replies");
    assert_eq!(got.len(), 2);
    assert_eq!(got[0].0, 0);
    assert_eq!(got[1].0, 0, "{}", got[1].1);
    let result: serde_json::Value = serde_json::from_str(&got[1].1).expect("call result");
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
    let keys = keys(listener.url(), &sent[0].body, "jev-1.13.0").expect("question keys");
    for (at, row) in rows.iter().enumerate() {
        assert_eq!(row["input"], corpus["texts"][at]);
        assert_eq!(row["meta"]["requests"], serde_json::json!([keys[at]]));
        assert!(row["meta"].get("batch").is_none(), "{at}");
    }
    replay_v1(&listener, &call, &corpus, &sent[0].body, &keys);
}
fn legacy_snapshot(listener: &Listener, request: &[u8], keys: &[String]) -> String {
    let parts: BTreeMap<String, Box<RawValue>> =
        serde_json::from_slice(request).expect("wire parts");
    let questions: BTreeMap<String, Box<RawValue>> =
        serde_json::from_str(parts["questions"].get()).expect("wire questions");
    let expected_bytes = BODIES
        .iter()
        .flat_map(|body| {
            let body: BTreeMap<String, Box<RawValue>> =
                serde_json::from_str(body).expect("fixture parts");
            let questions: BTreeMap<String, Box<RawValue>> =
                serde_json::from_str(body["questions"].get()).expect("fixture questions");
            questions
                .into_values()
                .map(|q| q.get().to_owned())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let digest = |value: &str| {
        Sha256::digest(value.as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>()
    };
    let state_digest = digest(parts["state"].get());
    let mut v1 = vec![
        serde_json::json!({"sha256": state_digest, "state": parts["state"].get()}).to_string(),
    ];
    for (at, expected) in expected_bytes.iter().enumerate() {
        let question = questions[&format!("q{}", at + 1)].get();
        assert_eq!(question, expected, "original fixture question bytes");
        let original_key = digest(
            &[
                "systemone",
                listener.url(),
                parts["model"].get(),
                parts["state"].get(),
                question,
            ]
            .join("\n"),
        );
        assert_ne!(original_key, keys[at]);
        v1.push(serde_json::json!({"key": original_key, "url": listener.url(), "model": "jev-1.13.0",
            "state": state_digest, "question": question, "answer": "{\"type\":\"noul\",\"noul\":0.9}",
            "answered_by": "jev-1.13.0", "input_tokens": null, "output_tokens": null,
            "taken_at": 0, "origin": "converted"}).to_string());
    }
    v1.join("\n") + "\n"
}
fn replay_v1(
    listener: &Listener,
    call: &str,
    corpus: &serde_json::Value,
    request: &[u8],
    keys: &[String],
) {
    let base = listener.base();
    let saved = crate::scratch("portable-v1-replay");
    let path = saved.join("thinkthen.jsonl");
    let original = legacy_snapshot(listener, request, keys);
    std::fs::write(&path, &original).expect("original v1 fixture");
    let before = std::fs::metadata(&path)
        .expect("fixture metadata")
        .modified()
        .expect("mtime");
    let settings = serde_json::json!({"base_url": base, "model": "jev-1.13.0", "cache": false, "replay": saved}).to_string();
    let mut script = Script::default();
    script.ask("settings", &[base, &settings]);
    script.ask("call", &[base, call]);
    let output = run(
        &compile(&crate_dir().join("tests/c/driver.c")),
        "",
        &script.0,
    );
    assert!(output.status.success(), "{}", text(&output.stderr));
    let got = replies(&output.stdout).expect("replay replies");
    assert_eq!(got[0].0, 0);
    assert_eq!(got[1].0, 0, "{}", got[1].1);
    let result: serde_json::Value = serde_json::from_str(&got[1].1).expect("actual C replay");
    assert_eq!(result["facts"]["requests_sent"], 0);
    assert_eq!(result["facts"]["records"], 5);
    for (at, row) in result["value"]
        .as_array()
        .expect("replayed rows")
        .iter()
        .enumerate()
    {
        assert_eq!(row["input"], corpus["texts"][at]);
        assert_eq!(row["value"], true);
        assert_eq!(row["meta"]["requests"], serde_json::json!([keys[at]]));
        assert_eq!(row["meta"]["cached"], true);
    }
    assert_eq!(listener.count(), 1, "read-only v1 replay sends nothing");
    assert_eq!(
        std::fs::read_to_string(&path).expect("unchanged v1 bytes"),
        original
    );
    assert_eq!(
        std::fs::metadata(&path)
            .expect("fixture metadata")
            .modified()
            .expect("mtime"),
        before
    );
}
