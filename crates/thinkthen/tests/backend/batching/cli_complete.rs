//! Aggregate context and full original occurrences execute through the CLI.
use super::{KEY, folder, text};
use crate::harness::{Canned, Listener, spawn};
use serde_json::{Value, json};
use std::fs;

#[test]
fn filter_and_rank_keep_actual_ordinals_for_equal_records() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .expect("listener");
    for verb in ["filter", "rank"] {
        let output = spawn(
            &[
                verb,
                "Refund?",
                "--jsonl",
                "--details",
                "--no-cache",
                "--url",
                listener.base(),
            ],
            &[KEY],
            b"{\"text\":\"equal\"}\n{\"text\":\"equal\"}\n",
        )
        .expect("command");
        assert_eq!(
            output.status.code(),
            Some(0),
            "{verb}: {}",
            text(&output.stderr)
        );
        let rows: Vec<Value> = text(&output.stdout)
            .lines()
            .map(|row| serde_json::from_str(row).expect("row"))
            .collect();
        assert_eq!(
            rows.iter()
                .map(|row| row["index"].clone())
                .collect::<Vec<_>>(),
            [json!(0), json!(1)]
        );
        assert_eq!(rows[0]["input"], json!({"text":"equal"}));
        assert_eq!(rows[1]["input"], rows[0]["input"]);
    }
}

#[test]
fn find_keeps_every_original_candidate_and_shared_context_separate() {
    let place = folder("find-complete-context");
    fs::create_dir_all(&place).expect("folder");
    let context = format!("{place}/context.txt");
    fs::write(&context, "Reference text").expect("context");
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"local-1","answers":{"q1":{"type":"choice","choice":"u002","probabilities":{"u001":0.1,"u002":0.9}}}}"#)).expect("listener");
    let output = spawn(
        &[
            "find",
            "Which unit answers?",
            "--jsonl",
            "--field",
            "/text",
            "--context",
            &context,
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
        ],
        &[KEY],
        b"{\"text\":\"first\",\"extra\":false}\n{\"text\":\"second\",\"extra\":null}\n",
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let row: Value = serde_json::from_slice(&output.stdout).expect("row");
    assert_eq!(row["value"], json!({"text":"second","extra":null}));
    assert_eq!(
        row["candidates"][0]["input"],
        json!({"text":"first","extra":false})
    );
    assert_eq!(row["candidates"][1]["input"], row["value"]);
    assert_eq!(row["candidates"][0]["index"], 0);
    assert!(row["meta"]["context_sha256"].is_string());
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let body: Value = serde_json::from_slice(&requests[0].body).expect("request");
    assert_eq!(
        body["state"],
        json!({"context":"Reference text","evidence":"[{\"id\":\"u001\",\"evidence\":\"first\"},{\"id\":\"u002\",\"evidence\":\"second\"}]"})
    );
}

#[test]
fn annotate_and_relate_send_shared_context_and_keep_originals() {
    let place = folder("aggregate-complete-context");
    fs::create_dir_all(&place).expect("folder");
    let context = format!("{place}/context.txt");
    let questions = format!("{place}/questions.json");
    fs::write(&context, "Reference text").expect("context");
    fs::write(
        &questions,
        r#"{"version":1,"questions":{"refund":{"decide":"Refund?"}}}"#,
    )
    .expect("questions");
    let listener = Listener::answering(|body| {
        let body: Value = serde_json::from_slice(body).expect("request");
        let answers: serde_json::Map<String, Value> = body["questions"]
            .as_object()
            .expect("questions")
            .keys()
            .map(|name| (name.clone(), json!({"type":"noul","noul":0.9})))
            .collect();
        Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
    })
    .expect("listener");
    let annotated = spawn(
        &[
            "annotate",
            &questions,
            "--jsonl",
            "--context",
            &context,
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
        ],
        &[KEY],
        b"{\"text\":\"refund\",\"extra\":false}\n",
    )
    .expect("annotate");
    assert_eq!(
        annotated.status.code(),
        Some(0),
        "{}",
        text(&annotated.stderr)
    );
    let row: Value = serde_json::from_slice(&annotated.stdout).expect("row");
    assert_eq!(row["input"], json!({"text":"refund","extra":false}));
    assert!(row["meta"]["context_sha256"].is_string());
    let related = spawn(&["relate", "knows=person:person", "--context", &context, "--details", "--no-cache", "--url", listener.base()], &[KEY], br#"[{"name":"Ada","kind":"person","extra":false},{"name":"Bob","kind":"person","extra":null}]"#).expect("relate");
    assert_eq!(related.status.code(), Some(0), "{}", text(&related.stderr));
    let row: Value = serde_json::from_slice(&related.stdout).expect("row");
    assert_eq!(
        row["input"],
        json!([{"name":"Ada","kind":"person","extra":false},{"name":"Bob","kind":"person","extra":null}])
    );
    assert!(row["meta"]["context_sha256"].is_string());
    let sent = listener.requests();
    assert_eq!(sent.len(), 2);
    let body: Value = serde_json::from_slice(&sent[0].body).expect("request");
    assert_eq!(body["state"], "Reference text");
    assert_eq!(
        body["questions"]["q1"]["instructions"],
        "The text is \"{\\\"text\\\":\\\"refund\\\",\\\"extra\\\":false}\". Refund?"
    );
    let body: Value = serde_json::from_slice(&sent[1].body).expect("request");
    assert_eq!(body["state"]["context"], "Reference text");
    assert_eq!(
        body["state"],
        json!({"context":"Reference text","evidence":{"entities":[{"id":"i1","kind":"person","name":"Ada"},{"id":"i2","kind":"person","name":"Bob"}]}})
    );
}

#[test]
fn recognize_shared_context_keeps_originals_and_selected_text_offsets() {
    let place = folder("recognize-complete-context");
    fs::create_dir_all(&place).expect("folder");
    let context = format!("{place}/context.txt");
    fs::write(&context, "Reference text").expect("context");
    let listener = Listener::answering(crate::recognize::automatic).expect("listener");
    let output = crate::recognize::run(
        &listener,
        &[
            "person",
            "organization",
            "--jsonl",
            "--field",
            "/text",
            "--context",
            &context,
            "--details",
        ],
        b"{\"text\":\"Ada met Acme.\",\"extra\":false}\n",
    );
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let row: Value = serde_json::from_slice(&output.stdout).expect("row");
    assert_eq!(row["input"], json!({"text":"Ada met Acme.","extra":false}));
    assert_eq!(row["value"]["entities"][0]["text"], "Ada");
    assert_eq!(row["value"]["entities"][0]["start"], 0);
    assert_eq!(row["value"]["entities"][0]["end"], 3);
    assert!(row["meta"]["context_sha256"].is_string());
    let sent = listener.requests();
    assert_eq!(sent.len(), 2);
    for request in sent {
        let body: Value = serde_json::from_slice(&request.body).expect("request");
        assert_eq!(body["state"]["context"], "Reference text");
    }
}

#[test]
fn relate_originals_do_not_hide_bad_object_ids_or_explicit_audit_pointers() {
    let place = folder("relate-audit-identities");
    fs::create_dir_all(&place).expect("folder");
    let key = format!("{place}/key.jsonl");
    fs::write(&key, "{\"id\":1,\"value\":[]}\n").expect("key");
    for (input, pointer) in [(json!({"id":1.5}), "/id"), (json!([]), "/record-id")] {
        let source = format!(
            "{}\n",
            json!({"input":input,"value":[],"question":{"verb":"relate"}})
        );
        let output = spawn(
            &["audit", "-", &key, "--id", pointer],
            &[],
            source.as_bytes(),
        )
        .expect("audit");
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert_eq!(
            text(&output.stderr),
            "thinkthen: audit: results line 1 has no string or integer id at the --id pointer\n"
        );
    }
}
