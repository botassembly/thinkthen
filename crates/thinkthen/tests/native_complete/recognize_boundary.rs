//! Boundary proposals use ordinary question answers and mode-specific aggregate values.
use super::*;
use thinkthen::{
    RecognitionMode, RecognitionValue, Recognize, RecordInput, Request, RequestArguments,
    RequestCall, RequestEnvironment, RequestInput, RequestOptions, RequestOutcome, RequestQuestion,
    RequestValue,
};

#[test]
fn boundary_proposals_use_exact_saved_bodies_and_shared_request_answers() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../../conformance/recognition-context.json"
    ))
    .unwrap();
    let exchanges: Vec<Value> = fixture["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["exchanges"][0].clone())
        .collect();
    let saved = exchanges.clone();
    let listener = Listener::answering(move |body| {
        let request = std::str::from_utf8(body).unwrap();
        let exchange = saved
            .iter()
            .find(|e| e["request"].as_str() == Some(request))
            .unwrap_or_else(|| panic!("unexpected boundary body {request}"));
        Canned::ok(&exchange["response"].to_string())
    })
    .unwrap();
    let cache = folder();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("jev-1.13.0")
        .unwrap()
        .api_key("fake")
        .unwrap()
        .cache_at(&cache)
        .unwrap()
        .max_retries(0)
        .build()
        .unwrap();
    let declaration = r#"{"version":1,"recognize":{"kinds":{"person":"A person's name.","organization":"An organization name."}}}"#;
    let ask = Recognize::from_json(declaration)
        .unwrap()
        .with_mode(RecognitionMode::BoundaryOnly);
    let text = fixture["text"].as_str().unwrap();
    let rows = fixture["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| RecordInput {
            original: text,
            context: row["context"].as_str().map(Into::into),
            options: None,
            examples: None,
            seed_spans: None,
        });
    let result = engine
        .recognize_records_complete_with(
            &ask,
            rows,
            CallOptions::new().context(fixture["shared_context"].as_str().unwrap()),
        )
        .unwrap();
    assert_eq!(listener.count(), 4);
    assert_eq!(result.facts().requests_sent(), 4);
    for (at, row) in result.value().iter().enumerate() {
        let complete = row.result();
        assert!(matches!(
            complete.value().value(),
            RecognitionValue::BoundaryOnly(_)
        ));
        let proposals = complete.value().proposals().unwrap();
        assert_eq!(
            proposals
                .iter()
                .map(|p| (p.text(), p.start(), p.end(), p.length(), p.probability()))
                .collect::<Vec<_>>(),
            vec![
                ("Amara", 0, 5, 5, 1.0),
                ("Kestrel Labs", 15, 27, 12, 0.9972)
            ]
        );
        let json: Value = serde_json::from_str(&complete.to_json().unwrap()).unwrap();
        assert_eq!(json["question"]["mode"], "boundary_only");
        assert!(json["question"].get("relation_threshold").is_none());
        assert!(json["value"].get("entities").is_none());
        assert!(json["answer"].get("names").is_none());
        assert!(json["answer"].get("pairs").is_none());
        assert_eq!(complete.probabilities().proposals().unwrap().len(), 2);
        assert_eq!(
            String::from_utf8(listener.requests()[at].body.clone()).unwrap(),
            exchanges[at]["request"].as_str().unwrap()
        );
    }
    let request = Request::new(RequestCall::Recognize(RequestArguments {
        question: RequestQuestion::Definition {
            value: Recognize::from_json(declaration).unwrap().into(),
        },
        input: RequestInput::Text {
            text: text.to_owned(),
            images: Vec::new(),
        },
        options: RequestOptions {
            mode: Some(RecognitionMode::BoundaryOnly),
            context: Some("First context".to_owned()),
            ..RequestOptions::default()
        },
    }));
    let admitted = Request::from_json(&serde_json::to_string(&request).unwrap())
        .unwrap()
        .admit()
        .unwrap();
    let RequestOutcome::Complete(call) = engine
        .execute_request(&admitted, RequestEnvironment::default())
        .unwrap()
    else {
        panic!("request failed");
    };
    let RequestValue::Recognized(rows) = call.value() else {
        panic!("wrong typed result");
    };
    assert_eq!(call.facts().requests_sent(), 0);
    assert_eq!(
        rows[0].result().value().to_json(),
        result.value()[0].result().value().to_json()
    );
    assert_eq!(
        rows[0].result().answer_id(),
        result.value()[0].result().answer_id()
    );
    assert_eq!(listener.count(), 4);
    std::fs::remove_dir_all(cache).unwrap();
}

fn uncertain(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap();
    let out = request["state"].as_str() == Some("lowercase");
    let answers: serde_json::Map<String,Value> = request["questions"].as_object().unwrap().iter().map(|(name,q)| {
        assert!(q["criteria"].get("SINGLE").is_some(), "a later stage sent");
        (name.clone(), json!({"type":"choice","probabilities":{"BEGIN":0,"INSIDE":0,"END":0,"SINGLE":if out {0.4} else {0.50004},"OUT":if out {0.6} else {0.49996}}}))
    }).collect();
    Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
}

#[test]
fn proposal_cuts_use_printed_probability_and_modes_keep_distinct_ids_without_later_questions() {
    let listener = Listener::answering(uncertain).unwrap();
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
    for (text, count) in [("Åda", 1), ("lowercase", 0), ("", 0)] {
        let whole = Recognize::builder().build().unwrap();
        let boundary = whole.clone().with_mode(RecognitionMode::BoundaryOnly);
        let complete = engine
            .recognize_complete_with(&whole, text, CallOptions::new())
            .unwrap();
        let proposed = engine
            .recognize_complete_with(&boundary, text, CallOptions::new())
            .unwrap();
        assert_eq!(proposed.facts().requests_sent(), 0);
        assert_ne!(complete.value().answer_id(), proposed.value().answer_id());
        assert_eq!(
            complete.value().identity().observations().len(),
            proposed.value().identity().observations().len()
        );
        assert_eq!(
            complete.value().identity().observations(),
            proposed.value().identity().observations()
        );
        assert_eq!(proposed.value().value().proposals().unwrap().len(), count);
        if count == 1 {
            let p = &proposed.value().value().proposals().unwrap()[0];
            assert_eq!(
                (p.text(), p.start(), p.end(), p.length(), p.probability()),
                ("Åda", 0, 3, 3, 0.5)
            );
            let cut = Recognize::builder()
                .mode(RecognitionMode::BoundaryOnly)
                .threshold(0.5001)
                .unwrap()
                .build()
                .unwrap();
            let rejected = engine
                .recognize_complete_with(&cut, text, CallOptions::new())
                .unwrap();
            assert_eq!(rejected.facts().requests_sent(), 0);
            assert_eq!(rejected.value().value().proposals().unwrap().len(), 0);
            assert_eq!(
                rejected.value().probabilities().proposals().unwrap()[0].probability(),
                0.5
            );
        }
    }
    assert_eq!(listener.count(), 2);
    let seeded = Recognize::builder()
        .mode(RecognitionMode::BoundaryOnly)
        .build()
        .unwrap()
        .with_seed_spans(vec![thinkthen::RecognitionSeedSpan {
            start: 0,
            end: 9,
            kind: None,
        }]);
    let proposed = engine
        .recognize_complete_with(&seeded, "lowercase", CallOptions::new())
        .unwrap();
    assert!(proposed.value().value().proposals().unwrap().is_empty());
    assert_eq!(listener.count(), 2);
    std::fs::remove_dir_all(cache).unwrap();
}

#[test]
fn skipped_stage_controls_refuse_before_native_record_enumeration() {
    let listener = Listener::answering(uncertain).unwrap();
    let engine = engine(&listener);
    let advanced = AtomicUsize::new(0);
    for ask in [
        Recognize::builder()
            .build()
            .unwrap()
            .with_mode(RecognitionMode::BoundaryOnly)
            .kind_edge_context(""),
        Recognize::builder()
            .build()
            .unwrap()
            .with_mode(RecognitionMode::BoundaryOnly)
            .relation_context(""),
        Recognize::builder()
            .relation_threshold(0.5)
            .unwrap()
            .build()
            .unwrap()
            .with_mode(RecognitionMode::BoundaryOnly),
        Recognize::from_json(
            r#"{"version":1,"recognize":{"mode":"boundary_only","relations":[]}}"#,
        )
        .unwrap(),
    ] {
        let rows = std::iter::from_fn(|| {
            advanced.fetch_add(1, Ordering::SeqCst);
            Some(Ok(RecordInput {
                original: "Åda",
                context: None,
                options: None,
                examples: None,
                seed_spans: None,
            }))
        });
        let refused = engine
            .try_recognize_records_complete_with(&ask, rows, CallOptions::new())
            .unwrap_err();
        assert_eq!(
            refused.detail().message(),
            "boundary_only recognition takes no relations, relation threshold, kind_edge context or relation context"
        );
    }
    assert_eq!(advanced.load(Ordering::SeqCst), 0);
    assert_eq!(listener.count(), 0);
}
