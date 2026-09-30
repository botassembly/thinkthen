//! Existing distinct command behavior kept as one focused case.

use super::*;

#[test]
fn a_tag_context_is_shared_evidence_and_keeps_two_logical_rows() {
    let directory = folder("tag-context");
    fs::create_dir_all(&directory).expect("context directory");
    let context = format!("{directory}/context.txt");
    fs::write(&context, b"Shared catalog\n").expect("context file");
    let answer = json!({"model":"local-1","answers":{
        "q1":{"type":"noul","noul":0.9},
        "q2":{"type":"noul","noul":0.1},
        "q3":{"type":"noul","noul":0.1},
        "q4":{"type":"noul","noul":0.9}
    }})
    .to_string();
    let listener = Listener::serving(vec![Canned::ok(&answer)]).expect("listener");
    let output = run(
        listener.base(),
        "tag",
        &["billing", "urgent"],
        &["--context", &context],
        "first\nsecond\n",
    );
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let request: Value = serde_json::from_slice(&requests[0].body).expect("request");
    assert_eq!(request["state"], "Shared catalog\n");
    assert_eq!(
        request["questions"].as_object().map(serde_json::Map::len),
        Some(4)
    );
    let rows = details(&output);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["value"], json!(["billing"]));
    assert_eq!(rows[1]["value"], json!(["urgent"]));
    assert!(
        rows.iter()
            .all(|row| row["meta"]["context_sha256"].is_string())
    );
}
