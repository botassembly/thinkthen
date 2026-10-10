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
#[ignore = "large-input boundary runs in the release suite"]
fn release_only_source_rank_charges_original_utf8_bytes_before_projection_and_never_reads_the_tail()
{
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
        Some(crate::public::SourceBudget::rank()),
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

#[test]
fn rank_forwards_the_inclusive_cutoff_before_top_and_refuses_nonprobability_routes() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.79},"q2":{"type":"noul","noul":0.8},"q3":{"type":"noul","noul":0.8}}}"#)).unwrap();
    let executor = NativeExecutor {
        engine: Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .api_key("fake")
            .unwrap()
            .model("fixed")
            .unwrap()
            .no_cache()
            .build()
            .unwrap(),
        schema: serde_json::from_str(crate::complete_call_schema()).unwrap(),
    };
    let params: CallParams = serde_json::from_value(json!({"name":"rank","arguments":{
        "question":"Best?", "records":["below","first","last"],
        "options":{"threshold":0.8,"top":3,"batch":"max"}
    }}))
    .unwrap();
    let reply = executor
        .execute(Invocation::admit(params).unwrap(), &CancelToken::new())
        .unwrap();
    let value = serde_json::to_value(tool_result(reply.object, reply.failed)).unwrap();
    let rows = value["structuredContent"]["value"].as_array().unwrap();
    assert_eq!(rows.len(), 2);
    for (at, original) in ["first", "last"].iter().enumerate() {
        assert_eq!(rows[at]["input"], *original);
        assert_eq!(rows[at]["value"], at + 1);
        assert_eq!(rows[at]["answer"]["probability"], 0.8);
    }
    assert_eq!(listener.count(), 1);
    for (question, threshold) in [
        (json!("Best?"), json!(0)),
        (json!("Best?"), json!("NaN")),
        (json!("Best?"), json!(1.01)),
        (json!("Best?"), json!("0.2:0.8")),
        (json!({"score":"Best?","levels":["low","high"]}), json!(0.8)),
        (
            json!({"score":"Best?","levels":["low","high"],"threshold":0.8}),
            json!(0.8),
        ),
        (
            json!({"version":1,"questions":{"first":{"decide":"Best?"}}}),
            json!(0.8),
        ),
        (
            json!({"version":1,"questions":{"first":{"decide":"Best?","threshold":0.8}}}),
            json!(0.8),
        ),
    ] {
        let params: CallParams = serde_json::from_value(json!({"name":"rank","arguments":{
            "question":question,"records":["unsent"],"options":{"threshold":threshold}
        }}))
        .unwrap();
        let refused = Invocation::admit(params).and_then(|invocation| {
            executor
                .execute(invocation, &CancelToken::new())
                .map(|_| ())
        });
        assert!(refused.is_err());
    }
    assert_eq!(listener.count(), 1, "refused calls send nothing");
}

#[test]
fn recognition_call_controls_and_item_overrides_change_the_actual_stage_and_result() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../../../conformance/recognition-context.json"
    ))
    .unwrap();
    let response = fixture["rows"][0]["exchanges"][0]["response"].to_string();
    let listener = Listener::answering(move |_| Canned::ok(&response)).unwrap();
    let executor = NativeExecutor {
        engine: Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .api_key("fake")
            .unwrap()
            .no_cache()
            .max_retries(0)
            .build()
            .unwrap(),
        schema: serde_json::from_str(crate::complete_call_schema()).unwrap(),
    };
    let mut question: Value =
        serde_json::from_str(fixture["question_json"].as_str().unwrap()).unwrap();
    question["recognize"]
        .as_object_mut()
        .unwrap()
        .remove("relations");
    question
        .as_object_mut()
        .unwrap()
        .remove("relation_threshold");
    let params: CallParams = serde_json::from_value(json!({"name":"recognize","arguments":{
        "question":question,"inputs":[{"text":"Amara works at Kestrel Labs.",
            "context":"item context","examples":[],"seed_spans":[]}],
        "options":{"mode":"boundary_only","snippet_pieces":2,
            "context":"shared context","stage_context":{"boundary":"stage context"},
            "seed_spans":[{"start":1,"end":3}],"examples":[]}
    }}))
    .unwrap();
    let reply = executor
        .execute(Invocation::admit(params).unwrap(), &CancelToken::new())
        .unwrap();
    assert!(!reply.failed);
    let value = serde_json::to_value(tool_result(reply.object, reply.failed)).unwrap();
    assert_eq!(
        value["structuredContent"]["value"][0]["value"]["mode"],
        "boundary_only"
    );
    assert_eq!(listener.count(), 1);
    let request: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
    assert_eq!(request["state"]["context"], "stage context");
    let instructions = request["questions"]["q1"]["instructions"].as_str().unwrap();
    assert!(instructions.contains("[[Amara]] works at"));
    assert!(!instructions.contains("Kestrel"));
}

#[test]
fn recognition_item_controls_refuse_null_and_invalid_edges_without_sends() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let executor = NativeExecutor {
        engine: Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .api_key("fake")
            .unwrap()
            .no_cache()
            .max_retries(0)
            .build()
            .unwrap(),
        schema: serde_json::from_str(crate::complete_call_schema()).unwrap(),
    };
    for controls in [
        json!({"examples":null}),
        json!({"seed_spans":null}),
        json!({"seed_spans":[{"start":1,"end":3}]}),
        json!({"seed_spans":[{"start":0,"end":3,"kind":"private-kind"}]}),
        json!({"examples":[{"text":"Ada","entities":[{"start":1,"end":3,"kind":"person"}]}]}),
    ] {
        let mut descriptor = controls;
        descriptor["text"] = json!("Ada");
        let params: CallParams = serde_json::from_value(json!({"name":"recognize","arguments":{
            "question":{"version":1,"recognize":{"kinds":{"person":null}}},
            "inputs":[descriptor]
        }}))
        .unwrap();
        let result =
            Invocation::admit(params).and_then(|call| executor.execute(call, &CancelToken::new()));
        let failure = match result {
            Err(error) => serde_json::to_value(error.complete()).unwrap(),
            Ok(reply) => {
                assert!(reply.failed);
                serde_json::to_value(tool_result(reply.object, reply.failed)).unwrap()["structuredContent"].clone()
            }
        };
        assert_eq!(failure["error"]["kind"], "usage");
        assert!(!failure.to_string().contains("private-kind"));
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn framed_table_sources_execute_logical_rows_and_keep_original_positions() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .unwrap();
    let executor = NativeExecutor {
        engine: Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .api_key("fake")
            .unwrap()
            .no_cache()
            .max_retries(0)
            .build()
            .unwrap(),
        schema: serde_json::from_str(crate::complete_call_schema()).unwrap(),
    };
    let folder = std::env::temp_dir().join(format!("thinkthen-mcp-framing-{}", std::process::id()));
    std::fs::create_dir(&folder).unwrap();
    for (framing, bytes, last) in [
        ("csv", "id,body\n1,\"Alpha.\nBeta.\"\n", 3),
        ("tsv", "id\tbody\n1\tAlpha.\n", 2),
    ] {
        let path = folder.join(framing);
        std::fs::write(&path, bytes).unwrap();
        let params: CallParams = serde_json::from_value(json!({"name":"decide","arguments":{
            "question":"Fits?","source":{"paths":[path],"framing":framing},
            "options":{"field":"/body","batch":1}
        }}))
        .unwrap();
        let reply = executor
            .execute(Invocation::admit(params).unwrap(), &CancelToken::new())
            .unwrap();
        assert!(!reply.failed);
        let value = serde_json::to_value(tool_result(reply.object, reply.failed)).unwrap();
        let call = &value["structuredContent"];
        assert_eq!(call["facts"]["requests_sent"], 1);
        assert_eq!(call["value"].as_array().unwrap().len(), 1);
        assert_eq!(
            call["value"][0]["input"],
            json!({"id":"1","body":if framing == "csv" {"Alpha.\nBeta."} else {"Alpha."}})
        );
        assert_eq!(
            call["value"][0]["source"],
            json!({"file":path,"first_line":2,"last_line":last})
        );
        std::fs::write(&path, "body,body\nfirst,second\n").unwrap();
        let params: CallParams = serde_json::from_value(json!({"name":"decide","arguments":{
            "question":"Fits?","source":{"paths":[path],"framing":"csv"}
        }}))
        .unwrap();
        let refused =
            Invocation::admit(params).and_then(|call| executor.execute(call, &CancelToken::new()));
        match refused {
            Err(error) => assert_eq!(error.kind(), crate::ErrorKind::Usage),
            Ok(reply) => assert!(reply.failed),
        }
    }
    assert_eq!(listener.count(), 2, "invalid headers send nothing");
    std::fs::remove_dir_all(folder).unwrap();
}
