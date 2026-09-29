//! The shared literal batch corpus through the public C JSON door.

use conformance_backend::{Canned, Listener};
use sha2::{Digest as _, Sha256};

use super::{Script, replies};
use crate::{compile, crate_dir, run, text};

const CORPUS: &str =
    include_str!("../../../../specification/fixtures/batching/portable-records.json");
const BODIES: [&str; 3] = [
    include_str!("../../../../specification/fixtures/batching/portable-1.request.json"),
    include_str!("../../../../specification/fixtures/batching/portable-2.request.json"),
    include_str!("../../../../specification/fixtures/batching/portable-3.request.json"),
];

fn digest(url: &str, body: &[u8]) -> String {
    let mut bytes = b"systemone\n".to_vec();
    bytes.extend_from_slice(url.as_bytes());
    bytes.push(b'\n');
    bytes.extend_from_slice(body);
    Sha256::digest(&bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[test]
fn c_json_records_keep_literal_cuts_bodies_and_request_identities() {
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
    assert_eq!(result["facts"]["requests_sent"], 3);
    let sent = listener.requests();
    assert_eq!(sent.len(), 3);
    let groups = [0, 0, 1, 1, 2];
    let mut digests = Vec::new();
    for (at, (request, literal)) in sent.iter().zip(BODIES).enumerate() {
        let body = literal
            .strip_suffix('\n')
            .expect("fixture newline")
            .as_bytes();
        assert_eq!(request.body, body, "request {at}");
        digests.push(digest(listener.url(), body));
    }
    for (at, row) in rows.iter().enumerate() {
        assert_eq!(row["input"], corpus["texts"][at]);
        assert_eq!(row["meta"]["requests"][0], digests[groups[at]]);
        if at < 4 {
            assert_eq!(row["meta"]["batch"]["closed"], "content");
            assert_eq!(row["meta"]["batch"]["records"], 2);
        } else {
            assert!(
                row["meta"].get("batch").is_none(),
                "singleton keeps old shape"
            );
        }
    }
}
