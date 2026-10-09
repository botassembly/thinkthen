//! Actual adapter execution checks disclosure, controls and native transport identity.
use super::super::{
    admission::{CallParams, Invocation},
    executor::NativeExecutor,
    output::tool_result,
    runtime::Executor,
};
use crate::{CancelToken, Engine};
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};

#[test]
fn record_projection_with_model_override_sends_only_selected_fields_and_native_mcp_headers() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"selected","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .api_key("sk-mcp-loopback-only")
        .unwrap()
        .no_cache()
        .max_retries(0)
        .build()
        .unwrap();
    let executor = NativeExecutor {
        engine,
        schema: serde_json::from_str(crate::complete_call_schema()).unwrap(),
    };
    let original = json!({"body":"Please refund.","ready":false,"private":"unsent"});
    let params: CallParams = serde_json::from_value(json!({"name":"decide","arguments":{
        "question":{"decide":"Refund?","model":"old"}, "records":[original],
        "options":{"field":["/body","/ready"],"model":"selected","batch":1,"attempts":true}
    }}))
    .unwrap();
    let reply = executor
        .execute(Invocation::admit(params).unwrap(), &CancelToken::new())
        .unwrap();
    let value = serde_json::to_value(tool_result(reply.object, reply.failed)).unwrap();
    let call = &value["structuredContent"];
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
    assert!(!String::from_utf8_lossy(&requests[0].body).contains("unsent"));
    assert_eq!(
        body["state"],
        "Each question quotes the text it asks about."
    );
    let instructions = body["questions"]["q1"]["instructions"].as_str().unwrap();
    assert!(instructions.contains("Please refund."));
    assert!(instructions.contains("false"));
    assert!(instructions.contains("ready"));
}

#[test]
fn source_rank_charges_original_utf8_bytes_before_projection_and_never_reads_the_tail() {
    use crate::{CallOptions, RecordReading, SourceItem, SourceRecord};
    use std::cell::Cell;
    let envelope = r#"{"public":"x","private":""}"#;
    let padding = crate::core::MAX_RECORD_BYTES - envelope.len();
    let private = "é".repeat(padding / 2) + if padding.is_multiple_of(2) { "" } else { "x" };
    let original = envelope.replacen("\"\"", &format!("\"{private}\""), 1);
    assert_eq!(original.len(), crate::core::MAX_RECORD_BYTES);
    let pulled = Cell::new(0);
    let source = (0..3).map(|at| {
        pulled.set(pulled.get() + 1);
        let record = match at {
            0 => original.clone(),
            1 => "é".to_owned(), // two bytes, and invalid JSON if composed too early
            _ => panic!("source rank read its refused tail"),
        };
        Ok(SourceItem::Text(SourceRecord {
            record,
            file: "source.jsonl".to_owned(),
            first_line: at + 1,
            last_line: at + 1,
        }))
    });
    let mut records = crate::transport::source_records(
        RecordReading::new(&["/public"], None, None).unwrap(),
        source,
        CallOptions::new(),
        false,
        true,
    );
    assert!(records.next().unwrap().is_ok());
    assert_eq!(
        records.next().unwrap().unwrap_err().detail().message(),
        "source rank reads at most 16 MiB across all input records"
    );
    assert!(records.next().is_none());
    assert!(records.next().is_none());
    assert_eq!(pulled.get(), 2);
}

#[test]
fn recognition_context_selectors_refuse_missing_null_and_nontext_before_dispatch() {
    let listener =
        Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{}}"#)).unwrap();
    let executor = NativeExecutor {
        engine: Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .api_key("fake")
            .unwrap()
            .no_cache()
            .build()
            .unwrap(),
        schema: serde_json::from_str(crate::complete_call_schema()).unwrap(),
    };
    for invalid in [
        json!({"body":"Bob."}),
        json!({"body":"Bob.","context":null}),
        json!({"body":"Bob.","context":4}),
    ] {
        let params: CallParams = serde_json::from_value(json!({"name":"recognize","arguments":{
            "question":{"version":1,"recognize":{}},
            "records":[{"body":"Ada.","context":"First context"},invalid],
            "options":{"field":"/body","context_field":"/context"}
        }}))
        .unwrap();
        let failure = executor
            .execute(
                Invocation {
                    tool: params.name,
                    arguments: params.arguments,
                },
                &CancelToken::new(),
            )
            .err()
            .expect("context selector refusal");
        assert_eq!(failure.kind(), crate::ErrorKind::Usage);
        assert_eq!(failure.stopped().at(), Some(2));
    }
    assert_eq!(listener.count(), 0);
}
