use super::*;
use thinkthen::{RawRecord, Recognize, RecordInput, RecordReading};

#[test]
fn recognition_keeps_record_context_separate_and_replays_only_unchanged_context() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let cache = folder();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("fake")
        .unwrap()
        .cache_at(&cache)
        .unwrap()
        .max_retries(0)
        .build()
        .unwrap();
    let ask = Recognize::builder().build().unwrap();
    let rows = || {
        [Some("first"), Some("second"), Some(""), None].map(|context| RecordInput {
            examples: None,
            seed_spans: None,
            original: "Ada met Acme.",
            context: context.map(Into::into),
            options: None,
        })
    };
    let options = CallOptions::new().context("shared");
    let first = engine
        .recognize_records_complete_with(&ask, rows(), options)
        .unwrap();
    assert_eq!(first.facts().requests_sent(), 8);
    assert_eq!(first.facts().records(), 4);
    let requests = listener.requests();
    for (pair, context) in requests
        .chunks_exact(2)
        .zip(["first", "second", "", "shared"])
    {
        for request in pair {
            let body: Value = serde_json::from_slice(&request.body).unwrap();
            assert_eq!(
                body["state"],
                if context.is_empty() {
                    json!("Ada met Acme.")
                } else {
                    json!({"context":context,"evidence":"Ada met Acme."})
                }
            );
        }
    }
    let replay = engine
        .recognize_records_complete_with(&ask, rows(), options)
        .unwrap();
    assert_eq!(replay.facts().requests_sent(), 0);
    let changed = engine
        .recognize_records_complete_with(
            &ask,
            [RecordInput {
                examples: None,
                seed_spans: None,
                original: "Ada met Acme.",
                context: Some("changed".into()),
                options: None,
            }],
            options,
        )
        .unwrap();
    assert_eq!(changed.facts().requests_sent(), 2);
    assert_eq!(listener.count(), 10);
    assert_ne!(
        first.value()[0].result().identity().observations(),
        changed.value()[0].result().identity().observations()
    );
    std::fs::remove_dir_all(cache).unwrap();
}

#[test]
fn selected_recognition_context_rejects_missing_null_and_nontext_before_sends() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let engine = engine(&listener);
    let ask = Recognize::builder().build().unwrap();
    let reading = RecordReading::new(&["/body"], Some("/context"), None).unwrap();
    for invalid in [
        r#"{"body":"Ada met Acme."}"#,
        r#"{"body":"Ada met Acme.","context":null}"#,
        r#"{"body":"Ada met Acme.","context":4}"#,
    ] {
        let valid = reading
            .compose(RawRecord::json(r#"{"body":"Ada met Acme.","context":"first"}"#).unwrap())
            .unwrap();
        let failure = engine
            .try_recognize_records_complete_with(
                &ask,
                [
                    Ok(valid),
                    reading.compose(RawRecord::json(invalid).unwrap()),
                ],
                CallOptions::new(),
            )
            .unwrap_err();
        assert_eq!(failure.stopped().at(), Some(2));
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn saved_recognition_exchanges_keep_each_context_on_every_stage() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../../conformance/recognition-context.json"
    ))
    .unwrap();
    let exchanges: Vec<Value> = fixture["rows"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|row| row["exchanges"].as_array().unwrap().clone())
        .collect();
    let saved = exchanges.clone();
    let listener = Listener::answering(move |body| {
        let request = std::str::from_utf8(body).unwrap();
        let exchange = saved
            .iter()
            .find(|e| e["request"].as_str() == Some(request))
            .unwrap_or_else(|| panic!("unexpected request {request}"));
        Canned::ok(&exchange["response"].to_string())
    })
    .unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("jev-1.13.0")
        .unwrap()
        .api_key("fake")
        .unwrap()
        .no_cache()
        .max_retries(0)
        .build()
        .unwrap();
    let ask = Recognize::from_json(fixture["question_json"].as_str().unwrap()).unwrap();
    let text = fixture["text"].as_str().unwrap();
    let rows = fixture["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| RecordInput {
            examples: None,
            seed_spans: None,
            original: text,
            context: row["context"].as_str().map(Into::into),
            options: None,
        });
    let result = engine
        .recognize_records_complete_with(
            &ask,
            rows,
            CallOptions::new().context(fixture["shared_context"].as_str().unwrap()),
        )
        .unwrap();
    assert_eq!(result.facts().records(), 4);
    assert_eq!(result.facts().requests_sent(), 12);
    for row in result.value() {
        assert_eq!(
            serde_json::from_str::<Value>(&row.result().value().to_json()).unwrap(),
            fixture["value"]
        );
    }
    assert_eq!(listener.count(), exchanges.len());
    for (sent, expected) in listener.requests().iter().zip(&exchanges) {
        assert_eq!(
            std::str::from_utf8(&sent.body).unwrap(),
            expected["request"].as_str().unwrap()
        );
    }
}

#[test]
fn recognition_preserves_declared_object_context_and_refuses_mistyped_context_before_sends() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let engine = engine(&listener);
    let ask = Recognize::from_json(r#"{"version":1,"recognize":{},"context_schema":{"type":"object","properties":{"z":{"type":"boolean"},"a":{"type":"string"}},"required":["z","a"]}}"#).unwrap();
    let context = thinkthen::RecordContext::Object(
        thinkthen::ObjectContext::new(
            &RawRecord::json(r#"{"z":false,"a":"Object context","extra":null}"#).unwrap(),
        )
        .unwrap(),
    );
    let rows = [RecordInput {
        examples: None,
        seed_spans: None,
        original: "Ada met Acme.",
        context: Some(context.clone()),
        options: None,
    }];
    let result = engine
        .recognize_records_complete_with(&ask, rows, CallOptions::new().context("unsent fallback"))
        .unwrap();
    assert_eq!(result.facts().requests_sent(), 2);
    for request in listener.requests() {
        let raw = std::str::from_utf8(&request.body).unwrap();
        assert!(raw.contains(r#""context":{"z":false,"a":"Object context","extra":null}"#));
        assert!(!raw.contains("unsent fallback"));
    }
    let refused = engine
        .recognize_records_complete_with(
            &ask,
            [
                RecordInput {
                    examples: None,
                    seed_spans: None,
                    original: "Ada met Acme.",
                    context: Some(context),
                    options: None,
                },
                RecordInput {
                    examples: None,
                    seed_spans: None,
                    original: "Ada met Acme.",
                    context: Some("mistyped".into()),
                    options: None,
                },
            ],
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(refused.kind(), ErrorKind::Usage);
    assert_eq!(refused.stopped().at(), Some(2));
    assert_eq!(listener.count(), 2);
}

#[test]
fn stage_context_changes_only_its_requests_and_complete_identity() {
    let listener = Listener::answering(super::aggregates::recognized_response).unwrap();
    let cache = folder();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("fake")
        .unwrap()
        .cache_at(&cache)
        .unwrap()
        .max_retries(0)
        .build()
        .unwrap();
    let ask = Recognize::builder().build().unwrap();
    let fallback = CallOptions::new().context("shared");
    let initial = engine
        .recognize_complete_with(&ask, "Ada met Acme.", fallback)
        .unwrap();
    let overridden = ask.clone().boundary_context("boundary\nonly");
    let changed = engine
        .recognize_complete_with(&overridden, "Ada met Acme.", fallback)
        .unwrap();
    assert_eq!(changed.facts().requests_sent(), 1);
    assert_eq!(listener.count(), 3);
    let requests = listener.requests();
    let body: Value = serde_json::from_slice(&requests[2].body).unwrap();
    assert_eq!(
        body["state"],
        json!({"context":"boundary\nonly","evidence":"Ada met Acme."})
    );
    let first: Value = serde_json::from_str(&initial.value().to_json().unwrap()).unwrap();
    let second: Value = serde_json::from_str(&changed.value().to_json().unwrap()).unwrap();
    assert_ne!(first["answer_id"], second["answer_id"]);
    assert!(first["answer_id"].is_string());
    assert_eq!(
        second["question"]["stage_context"],
        json!({"boundary":"boundary\nonly"})
    );
    assert_eq!(
        changed
            .value()
            .question()
            .stage_context()
            .boundary
            .as_deref(),
        Some("boundary\nonly")
    );
    let cleared = ask.clone().boundary_context("").kind_edge_context("");
    let cleared = engine
        .recognize_complete_with(&cleared, "Ada met Acme.", fallback)
        .unwrap();
    assert_eq!(cleared.facts().requests_sent(), 2);
    for request in &listener.requests() {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(body["state"], json!("Ada met Acme."));
    }
    assert_eq!(
        engine
            .recognize_complete_with(&overridden, "Ada met Acme.", fallback)
            .unwrap()
            .facts()
            .requests_sent(),
        0
    );
    assert!(!format!("{:?}", overridden).contains("boundary\nonly"));
    std::fs::remove_dir_all(cache).unwrap();
}
