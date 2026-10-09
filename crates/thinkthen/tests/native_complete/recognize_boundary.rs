//! Boundary proposals use ordinary question answers and mode-specific aggregate values.
use super::*;
use thinkthen::{
    RecognitionMode, RecognitionValue, Recognize, RecordInput, Request, RequestArguments,
    RequestCall, RequestEnvironment, RequestInput, RequestOptions, RequestOutcome, RequestQuestion,
    RequestValue,
};

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "one outside-in case compares the saved four-record exchanges and their canonical Request cache reuse"
)]
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
    let captured = listener.requests();
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
            String::from_utf8(captured[at].body.clone()).unwrap(),
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

#[cfg(test)]
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
            let judgment = complete
                .value()
                .probabilities()
                .judged_proposals()
                .next()
                .unwrap();
            assert!((judgment.span_probability() - 0.50004).abs() < 1e-8);
            assert_eq!(judgment.strength(), Some(0.5));
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

#[test]
fn decoded_proposals_preserve_punctuation_and_actual_source_lines_without_edge_adjustment() {
    use thinkthen::{RawRecord, RecordReading, SourceLocation};
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).unwrap();
        let answers: serde_json::Map<String, Value> = request["questions"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(name, q)| {
                let labels = q["criteria"].as_object().unwrap();
                let words = q["instructions"].as_str().unwrap();
                let picked = match (
                    labels.contains_key("SINGLE"),
                    words.contains("[[Ada]]"),
                    words.contains("[[.]]"),
                ) {
                    (false, _, _) => "Ada",
                    (true, true, _) => "BEGIN",
                    (true, _, true) => "END",
                    _ => "OUT",
                };
                assert!(labels.contains_key(picked));
                let probabilities: serde_json::Map<String, Value> = labels
                    .keys()
                    .map(|label| (label.clone(), json!(u8::from(label == picked))))
                    .collect();
                (
                    name.clone(),
                    json!({"type":"choice","probabilities":probabilities}),
                )
            })
            .collect();
        Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
    })
    .unwrap();
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
    let whole = Recognize::builder().build().unwrap();
    let full = engine
        .recognize_complete_with(&whole, "🙂\r\nAda.", CallOptions::new())
        .unwrap();
    assert_eq!(full.value().value().entities()[0].text(), "Ada");
    assert_eq!(listener.count(), 2);
    let boundary = whole.with_mode(RecognitionMode::BoundaryOnly);
    let mut row = RecordReading::new(&[], None, None)
        .unwrap()
        .compose(RawRecord::text("🙂\r\nAda.").unwrap())
        .unwrap();
    row.original = row
        .original
        .with_location(SourceLocation::new("example.txt".into(), Some(40), Some(41)).unwrap());
    let result = engine
        .recognize_records_complete_with(&boundary, [row], CallOptions::new())
        .unwrap();
    assert_eq!(result.facts().requests_sent(), 0);
    assert_eq!(listener.count(), 2);
    let located = result.value()[0].result().source_value().unwrap();
    assert_eq!(located.mode(), RecognitionMode::BoundaryOnly);
    let proposed = &located.proposals().unwrap()[0];
    assert_eq!(
        (
            proposed.proposal().text(),
            proposed.proposal().start(),
            proposed.proposal().end(),
            proposed.proposal().length()
        ),
        ("Ada.", 3, 7, 4)
    );
    assert_eq!(
        (
            proposed.location().first_line(),
            proposed.location().last_line()
        ),
        (Some(41), Some(41))
    );
    let json: Value = serde_json::from_str(&result.value()[0].result().to_json().unwrap()).unwrap();
    assert!(json["value"].get("entities").is_none());
    assert_eq!(json["value"]["proposals"][0]["file"], "example.txt");
    assert_eq!(json["value"]["proposals"][0]["first_line"], 41);
    std::fs::remove_dir_all(cache).unwrap();
}

#[test]
fn saved_mode_admission_precedes_evidence_and_call_whole_overrides_saved_boundary() {
    use thinkthen::{RequestSource, RequestThreshold};
    let listener = Listener::answering(uncertain).unwrap();
    let engine = engine(&listener);
    let root = folder();
    let saved = root.join("saved.json");
    let request = |options| {
        Request::new(RequestCall::Recognize(RequestArguments {
            question: RequestQuestion::File {
                path: saved.clone(),
            },
            input: RequestInput::Source {
                source: RequestSource {
                    paths: vec![root.join("missing-evidence")],
                    reading: thinkthen::ReaderOptions::default(),
                    media: thinkthen::ReaderMedia::Text,
                },
            },
            options,
        }))
    };
    for declaration in [
        r#"{"version":1,"recognize":{"mode":"boundary_only","relations":[]}}"#,
        r#"{"version":1,"recognize":{"mode":"boundary_only"},"relation_threshold":0.5}"#,
        r#"{"version":1,"recognize":{"mode":"boundary_only","stage_context":{"kind_edge":""}}}"#,
        r#"{"version":1,"recognize":{"mode":"boundary_only","stage_context":{"relation":""}}}"#,
    ] {
        std::fs::write(&saved, declaration).unwrap();
        let admitted = request(RequestOptions::default()).admit().unwrap();
        let refused = engine
            .execute_request(&admitted, RequestEnvironment::default())
            .unwrap_err();
        assert_eq!(refused.kind(), ErrorKind::Local);
        assert_eq!(
            refused.detail().message(),
            "boundary_only recognition takes no relations, relation threshold, kind_edge context or relation context"
        );
    }
    assert_eq!(listener.count(), 0);
    std::fs::write(&saved,r#"{"version":1,"recognize":{"mode":"boundary_only","relations":[],"stage_context":{"kind_edge":"","relation":""}},"relation_threshold":0.5}"#).unwrap();
    let request = Request::new(RequestCall::Recognize(RequestArguments {
        question: RequestQuestion::File { path: saved },
        input: RequestInput::Text {
            text: "lowercase".into(),
            images: Vec::new(),
        },
        options: RequestOptions {
            mode: Some(RecognitionMode::Whole),
            relation_threshold: Some(RequestThreshold::Cut(0.6)),
            ..RequestOptions::default()
        },
    }))
    .admit()
    .unwrap();
    let RequestOutcome::Complete(call) = engine
        .execute_request(&request, RequestEnvironment::default())
        .unwrap()
    else {
        panic!("call failed");
    };
    let RequestValue::Recognized(rows) = call.value() else {
        panic!("wrong typed value");
    };
    assert_eq!(rows[0].result().value().mode(), RecognitionMode::Whole);
    assert_eq!(
        rows[0].result().question().relation_threshold(),
        thinkthen::ResolvedThreshold::Cut(0.6)
    );
    assert_eq!(listener.count(), 1);
    std::fs::remove_dir_all(root).unwrap();
}
