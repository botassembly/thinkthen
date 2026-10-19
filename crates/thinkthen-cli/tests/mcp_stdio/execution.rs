use super::session::Session;
use conformance_backend::{Backend, Canned, Listener};
use serde_json::{Value, json};

#[test]
fn scalar_and_record_calls_keep_false_null_original_types_and_observed_facts() {
    let listener = Listener::answering(|request| {
        let body = String::from_utf8_lossy(request);
        let probability = if body.contains("uncertain") { 0.5 } else { 0.1 };
        Canned::ok(
            &json!({"model":"fixed","answers":{"q1":{"type":"noul","noul":probability}}})
                .to_string(),
        )
    })
    .unwrap();
    let mut session = Session::launch(listener.base());
    session.initialize();
    for (evidence, expected) in [("negative", json!(false)), ("uncertain", Value::Null)] {
        let result = session.call(
            "decide",
            json!({
                "question":{"decide":"Q?","threshold":"0.2:0.8"},"evidence":evidence
            }),
        );
        assert_eq!(result["isError"], false);
        assert_eq!(
            result["structuredContent"]["value"].get("value"),
            Some(&expected)
        );
        assert_eq!(result["structuredContent"]["facts"]["requests_sent"], 1);
    }
    let result = session.call(
        "decide",
        json!({"question":"Q?","records":[false,null],"options":{"batch":1}}),
    );
    assert_eq!(result["isError"], false);
    let rows = result["structuredContent"]["value"].as_array().unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["input"], false);
    assert_eq!(rows[1].get("input"), Some(&Value::Null));
    for row in rows {
        assert_eq!(row["value"], false);
    }
    assert_eq!(result["structuredContent"]["facts"]["requests_sent"], 2);
    assert_eq!(listener.count(), 4);
    session.finish();
}

#[test]
fn projected_records_keep_originals_and_send_native_mcp_identity_and_model_override() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"selected","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .unwrap();
    let mut session = Session::launch(listener.base());
    session.initialize();
    let original = json!({"body":"Please refund.","ready":false,"private":"unsent"});
    let result = session.call(
        "decide",
        json!({
            "question":{"decide":"Refund?","model":"old"},"records":[original],
            "options":{"field":["/body","/ready"],"model":"selected","batch":1,"attempts":true}
        }),
    );
    let call = &result["structuredContent"];
    assert_eq!(result["isError"], false);
    assert_eq!(call["value"][0]["input"], original);
    assert_eq!(call["value"][0]["value"], true);
    assert_eq!(call["facts"]["requests_sent"], 1);
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        requests[0].header("User-Agent"),
        Some(concat!("thinkthen/", env!("CARGO_PKG_VERSION"), " (mcp)"))
    );
    assert_eq!(
        requests[0].header("X-ThinkThen-Call-Id"),
        call["facts"]["call_id"].as_str()
    );
    assert!(requests[0].header("X-ThinkThen-Request-Id").is_some());
    let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["model"], "selected");
    assert_eq!(
        body["state"],
        "Each question quotes the text it asks about."
    );
    assert!(!String::from_utf8_lossy(&requests[0].body).contains("unsent"));
    let instructions = body["questions"]["q1"]["instructions"].as_str().unwrap();
    for selected in ["Please refund.", "ready", "false"] {
        assert!(instructions.contains(selected));
    }
    session.finish();
}

#[test]
fn partial_question_failures_keep_usable_answers_and_mark_the_tool_error() {
    let contract: Value =
        serde_json::from_str(include_str!("../../../../conformance/cases.json")).unwrap();
    let case = contract["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == "17-annotate-partial")
        .unwrap();
    #[derive(serde::Deserialize)]
    struct Cases {
        cases: Vec<Case>,
    }
    #[derive(serde::Deserialize)]
    struct Case {
        id: String,
        question_set: Option<Box<serde_json::value::RawValue>>,
    }
    let raw: Cases =
        serde_json::from_str(include_str!("../../../../conformance/cases.json")).unwrap();
    let question = raw
        .cases
        .into_iter()
        .find(|row| row.id == "17-annotate-partial")
        .unwrap()
        .question_set
        .unwrap();
    #[derive(serde::Serialize)]
    struct Arguments<'a> {
        question: &'a serde_json::value::RawValue,
        records: Vec<&'a str>,
        options: Value,
    }
    let response = case["exchanges"][0]["response"].to_string();
    let listener = Listener::answering(move |_| Canned::ok(&response)).unwrap();
    let mut session = Session::launch(listener.base());
    session.initialize();
    let result = session.call(
        "annotate",
        Arguments {
            question: &question,
            records: vec![case["exchanges"][0]["evidence"].as_str().unwrap()],
            options: json!({"batch":"max"}),
        },
    );
    assert_eq!(result["isError"], true);
    let call = &result["structuredContent"];
    assert_eq!(call["facts"]["requests_sent"], 1);
    assert_eq!(call["value"][0]["value"].get("refund"), Some(&Value::Null));
    assert_eq!(call["value"][0]["meta"]["failed_questions"], 1, "{call}");
    assert_eq!(
        call["value"][0]["answers"]["team"]["failure"]["cause"],
        "missing_probability"
    );
    assert_eq!(
        call["value"][0]["value"]["topics"],
        json!(["billing", "urgent"])
    );
    assert_eq!(listener.count(), 1);
    session.finish();
}

#[test]
fn rank_maps_inclusive_threshold_before_top_and_retains_tie_order() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.79},"q2":{"type":"noul","noul":0.8},"q3":{"type":"noul","noul":0.8}}}"#)).unwrap();
    let mut session = Session::launch(listener.base());
    session.initialize();
    let result = session.call("rank", json!({"question":"Best?","records":["below","first","last"],"options":{"threshold":0.8,"top":3,"batch":"max"}}));
    assert_eq!(result["isError"], false);
    let rows = result["structuredContent"]["value"].as_array().unwrap();
    assert_eq!(rows.len(), 2);
    for (at, original) in ["first", "last"].iter().enumerate() {
        assert_eq!(rows[at]["input"], *original);
        assert_eq!(rows[at]["value"], at + 1);
        assert_eq!(rows[at]["answer"]["probability"], 0.8);
    }
    assert_eq!(listener.count(), 1);
    session.finish();
}

#[test]
fn backend_failures_preserve_actual_started_facts_and_hide_private_payloads() {
    let backend = Backend::start().unwrap();
    let mut session = Session::launch(&format!("{}/arm/refuse/v1", backend.origin()));
    session.initialize();
    let result = session.call(
        "decide",
        json!({"question":"private-question","evidence":"private-evidence"}),
    );
    assert_eq!(result["isError"], true);
    assert_eq!(result["structuredContent"]["error"]["kind"], "backend");
    assert_eq!(result["structuredContent"]["facts"]["requests_sent"], 1);
    assert_eq!(backend.count(), 1);
    for secret in [
        "sk-mcp-loopback-only",
        "private-question",
        "private-evidence",
    ] {
        assert!(!result.to_string().contains(secret));
    }
    session.finish();
}

#[test]
fn source_failure_preserves_the_completed_prefix_and_observed_facts() {
    let listener = Listener::answering(|_| Canned::ok(
        r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#,
    )).unwrap();
    let mut session = Session::launch(listener.base());
    session.initialize();
    let first = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../specification/fixtures/recognize/README.md");
    let expected = std::fs::read_to_string(&first).unwrap();
    // Both paths must pass native manifest admission. A later content error
    // then tests a real started prefix, rather than an upfront missing path.
    let invalid = session.home.join("invalid-prefix-source-0639.txt");
    std::fs::write(&invalid, [0xff]).unwrap();
    let result = session.call("decide", json!({
        "question":"Q?",
        "source":{"paths":[first, invalid],"unit":"file"},
        "options":{"batch":1}
    }));
    assert_eq!(result["isError"], true);
    let content = &result["structuredContent"];
    assert_eq!(content["error"]["kind"], "usage");
    assert_eq!(content["error"]["message"], "the record is not valid UTF-8");
    assert!(content.get("value").is_none());
    let completed = content["completed"].as_array().unwrap();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0]["input"], expected);
    assert_eq!(content["facts"]["requests_sent"], 1);
    assert_eq!(listener.count(), 1);
    session.finish();
}
