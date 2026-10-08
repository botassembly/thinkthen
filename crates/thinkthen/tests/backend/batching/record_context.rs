//! Context belongs to the selected original, including a single document.
use super::{KEY, details, folder, text};
use crate::harness::{Listener, spawn};
use serde_json::Value;
use std::fs;

#[test]
fn per_record_context_reaches_the_wire_and_empty_suppresses_shared_context() {
    let listener = Listener::answering(super::answering).expect("listener");
    let place = folder("record-context");
    fs::create_dir_all(&place).expect("folder");
    let shared = format!("{place}/shared.txt");
    fs::write(&shared, "Shared context").expect("shared context");
    let input = b"{\"text\":\"line 1\",\"context\":\"First context\",\"extra\":false}\n{\"text\":\"line 2\",\"context\":\"\"}\n";
    let output = spawn(
        &[
            "decide",
            super::QUESTION,
            "--jsonl",
            "--field",
            "/text",
            "--context-field",
            "/context",
            "--context",
            &shared,
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
        ],
        &[KEY],
        input,
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let mut bodies: Vec<Value> = listener
        .requests()
        .iter()
        .map(|request| serde_json::from_slice(&request.body).expect("request"))
        .collect();
    bodies.sort_by_key(|body| body["state"].as_str().unwrap_or_default().to_owned());
    assert_eq!(bodies.len(), 2);
    assert_eq!(bodies[1]["state"], "First context");
    assert_eq!(
        bodies[0]["state"],
        "Each question quotes the text it asks about."
    );
    assert_eq!(
        bodies[1]["questions"]["q1"]["instructions"],
        "The text is \"line 1\". It names a place."
    );
    assert_eq!(
        bodies[0]["questions"]["q1"]["instructions"],
        "The text is \"line 2\". It names a place."
    );
    let rows = details(&output);
    assert_eq!(rows[0]["input"]["extra"], false);
    assert!(rows[0]["meta"]["context_sha256"].is_string());
    assert!(rows[1]["meta"].get("context_sha256").is_none());
}

#[test]
fn single_document_shared_context_is_sent_and_strictly_replayed() {
    let place = folder("document-context");
    fs::create_dir_all(&place).expect("folder");
    let context = format!("{place}/context.txt");
    let recording = format!("{place}/recording");
    fs::write(&context, "Shared context").expect("context");
    let listener = Listener::answering(super::answering).expect("listener");
    let base = [
        "decide",
        super::QUESTION,
        "--context",
        &context,
        "--details",
        "--url",
        listener.base(),
    ];
    let recorded = spawn(
        &[&base[..], &["--record", &recording]].concat(),
        &[KEY],
        b"line 1",
    )
    .expect("record");
    assert_eq!(
        recorded.status.code(),
        Some(1),
        "{}",
        text(&recorded.stderr)
    );
    let sent = listener.requests();
    assert_eq!(sent.len(), 1);
    let body: Value = serde_json::from_slice(&sent[0].body).expect("request");
    assert_eq!(body["state"], "Shared context");
    assert_eq!(
        body["questions"]["q1"]["instructions"],
        "The text is \"line 1\". It names a place."
    );
    let replayed = spawn(
        &[&base[..], &["--replay", &recording]].concat(),
        &[],
        b"line 1",
    )
    .expect("replay");
    assert_eq!(
        replayed.status.code(),
        Some(1),
        "{}",
        text(&replayed.stderr)
    );
    assert_eq!(listener.count(), 1, "strict replay sends nothing");
}

#[test]
fn declared_object_context_preserves_member_order_and_null_refuses_before_send() {
    let place = folder("object-context");
    fs::create_dir_all(&place).expect("folder");
    let question = format!("{place}/question.json");
    fs::write(
        &question,
        r#"{"decide":"It names a place.","context_schema":{"type":"object","properties":{}}}"#,
    )
    .expect("question");
    let operand = format!("@{question}");
    let listener = Listener::answering(super::answering).expect("listener");
    let base = [
        "decide",
        &operand,
        "--jsonl",
        "--field",
        "/text",
        "--context-field",
        "/context",
        "--no-cache",
        "--url",
        listener.base(),
    ];
    let output = spawn(
        &base,
        &[KEY],
        b"{\"text\":\"line 1\",\"context\":{\"z\":false,\"a\":null}}\n",
    )
    .expect("object context");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let sent = listener.requests();
    assert_eq!(sent.len(), 1);
    assert!(text(&sent[0].body).contains(r#""state":{"z":false,"a":null}"#));
    let refused = spawn(
        &base,
        &[KEY],
        b"{\"text\":\"line 1\",\"context\":{}}\n{\"text\":\"line 2\",\"context\":null}\n",
    )
    .expect("null refusal");
    assert_eq!(refused.status.code(), Some(2), "{}", text(&refused.stderr));
    assert!(refused.stdout.is_empty());
    assert_eq!(listener.count(), 1, "the malformed stage sends nothing");
}
