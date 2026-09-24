//! Relate's one route the shared sweep cannot reach: a mixed logical failure.
//!
//! Every other backend, recording, cache, and refusal route runs over relate in
//! `secrecy.rs` and `refusals.rs`.

use serde_json::Value;

use crate::harness::{Canned, Listener, spawn};
use crate::secrecy::{EVIDENCE, environment, folder, nothing_leaked};

#[test]
fn a_partial_relation_answer_prints_and_exits_six_without_quoting_evidence() {
    let into = folder("relate-partial").expect("folder");
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request");
        let names = request["questions"]
            .as_object()
            .expect("questions")
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        Canned::ok(
            &serde_json::json!({"model":"local-1","answers":{
                names[0].clone(): {"type":"noul","noul":0.9},
                names[1].clone(): {"type":"choice","probabilities":{"wrong":1.0}}
            }})
            .to_string(),
        )
    })
    .expect("listener");
    let output = spawn(
        &[
            "relate",
            "linked=person:person",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--no-cache",
            "--details",
        ],
        &environment(true),
        format!(r#"[{{"name":"{EVIDENCE}","kind":"person"}},{{"name":"Ada","kind":"person"}}]"#)
            .as_bytes(),
    )
    .expect("partial run");
    assert_eq!(output.status.code(), Some(6));
    nothing_leaked("relate partial", &output, &into);
    assert!(!output.stdout.is_empty());
    assert_eq!(listener.connections(), 1);
}
