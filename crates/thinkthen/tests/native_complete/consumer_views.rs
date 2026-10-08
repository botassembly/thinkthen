use super::*;
use std::sync::Mutex;
use thinkthen::{Entity, FindSelection, RecordInput, RecordObservation, Relate};

#[test]
fn find_selection_reports_synthetic_none_even_when_the_raw_tied_pick_names_a_real_unit() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"u001":0.4,"u002":0.2,"none":0.4}}}}"#)).unwrap();
    let engine = engine(&listener);
    let question = Question::find("Which?").unwrap().offering_none().unwrap();
    let call = engine
        .find_complete_with(&question, ["First.", "Second."], CallOptions::new())
        .unwrap();
    assert_eq!(call.value().raw_pick(), "u001");
    assert_eq!(call.value().selection(), FindSelection::None);
    assert!(call.value().selected().is_none());
    assert!(call.value().candidates()[2].is_none());
    let doc = serde_json::to_value(call.value()).unwrap();
    assert!(doc["index"].is_null());
    assert_eq!(
        doc["candidates"],
        json!([
            {"index":0,"input":"First.","probability":0.4},
            {"index":1,"input":"Second.","probability":0.2},
            {"index":null,"input":null,"probability":0.4}
        ])
    );
    assert_eq!(listener.count(), 1);
    assert_eq!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
        json!({"state":"[{\"id\":\"u001\",\"evidence\":\"First.\"},{\"id\":\"u002\",\"evidence\":\"Second.\"}]","model":"fixed","questions":{"q1":{"type":"choice","instructions":"Which?","criteria":{"u001":null,"u002":null,"none":null}}}})
    );
}

#[test]
fn observer_remapping_after_null_omission_preserves_actual_details_and_identities() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let question = Question::decide("Refund?").unwrap().cut();
    let original = [None, Some("Same."), None, Some("Same.")];
    let retained: Vec<_> = original
        .iter()
        .enumerate()
        .filter_map(|(at, item)| item.map(|item| (at, item)))
        .collect();
    let events = Mutex::new(Vec::new());
    let observer = |event: RecordObservation<'_>| {
        let index = match &event {
            RecordObservation::Question { index, .. } | RecordObservation::Row { index, .. } => {
                *index
            }
        };
        events
            .lock()
            .unwrap()
            .push(event.remap_index(retained[index].0).to_owned());
    };
    let call = engine
        .decide_records_complete_with(
            &question,
            retained.iter().map(|(_, item)| RecordInput {
                examples: None,
                seed_spans: None,
                original: *item,
                context: None,
                options: None,
            }),
            CallOptions::new().observe(&observer),
        )
        .unwrap();
    let events = events.lock().unwrap();
    assert_eq!(events.len(), 4);
    for (at, original_index) in [1, 3].into_iter().enumerate() {
        let thinkthen::OwnedRecordObservation::Question { index, detail, .. } = &events[at * 2]
        else {
            panic!("question")
        };
        assert_eq!(*index, original_index);
        let detail = detail.detail();
        assert_eq!(
            detail.answer_id(),
            Some(call.value()[at].result().answer_id())
        );
        assert_eq!(
            detail.observations(),
            call.value()[at].result().identity().observations()
        );
        assert_eq!(
            detail.input(),
            Some(&thinkthen::QuestionInput::Text("Same.".into()))
        );
        let thinkthen::OwnedRecordObservation::Row { index, .. } = &events[at * 2 + 1] else {
            panic!("row")
        };
        assert_eq!(*index, original_index);
    }
    assert_eq!(listener.count(), 1);
}

#[test]
fn materialized_convenience_rank_find_recognize_and_entities_validate_before_any_send() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = engine(&listener);
    let rank = Question::rank_from_json(
        r#"{"decide":"Refund?","item_schema":{"type":"object","properties":{}}}"#,
    )
    .unwrap();
    assert_eq!(
        engine
            .rank_with(&rank, ["Valid text.", "Other text."], CallOptions::new())
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    let find = Question::find("Which?")
        .unwrap()
        .with_item_schema(thinkthen::InputDeclaration::Object(
            thinkthen::ObjectDeclaration::new(Vec::new(), Vec::new()).unwrap(),
        ))
        .unwrap();
    assert_eq!(
        engine
            .find_with(&find, ["A.", "B."], CallOptions::new())
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    let recognize = thinkthen::Recognize::from_json(
        r#"{"version":1,"recognize":{},"item_schema":{"type":"object","properties":{}}}"#,
    )
    .unwrap();
    assert_eq!(
        engine
            .recognize_with(&recognize, "Ada.", CallOptions::new())
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    let relate = Relate::from_json(r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person","reads":"knows"}]},"item_schema":{"type":"string"}}"#).unwrap();
    for complete in [false, true] {
        let entities = [
            Entity::new("Ada", "person").unwrap(),
            Entity::new("Bea", "person").unwrap(),
        ];
        let error = if complete {
            engine
                .relate_complete_with(&relate, entities, CallOptions::new())
                .unwrap_err()
        } else {
            engine
                .relate_with(&relate, entities, CallOptions::new())
                .unwrap_err()
        };
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.detail().message(),
            "the item does not match item_schema"
        );
        assert!(error.facts().is_none());
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn loaded_atomic_and_rank_preparation_selects_original_fields_and_retains_authored_controls() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.8}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let authored = r#"{"name":"selected","wording_version":2,"decide":"Refund?","model":"fixed","batch":1,"on":["/payload"],"item_schema":{"type":"string"}}"#;
    let question = Question::from_json(authored).unwrap();
    let restored = Question::from_json(&question.to_json().unwrap()).unwrap();
    assert_eq!(restored, question);
    for text in [
        r#"{"choose":"Route?","options":{"first":{"z":null,"a":"first description"},"second":null},"threshold":0.8}"#,
        r#"{"score":"Grade?","levels":{"low":null,"high":"Excellent."}}"#,
    ] {
        let loaded = Question::from_json(text).unwrap();
        assert_eq!(
            Question::from_json(&loaded.to_json().unwrap()).unwrap(),
            loaded
        );
    }

    let location = thinkthen::SourceLocation::new("notes.jsonl".into(), Some(7), Some(7)).unwrap();
    let input = thinkthen::QuestionInput::annotation_text(
        r#"{"payload":"Yes.","private":"Never send."}"#,
        location,
    )
    .unwrap();
    let records = || {
        [thinkthen::RecordInput {
            examples: None,
            seed_spans: None,
            original: input.clone(),
            context: None,
            options: None,
        }]
    };
    let call = engine
        .decide_records_complete_with(&restored, records(), CallOptions::new())
        .unwrap();
    let row = &call.value()[0];
    assert_eq!(row.result().value(), Answer::Yes);
    assert_eq!(row.result().question().model(), Some("fixed"));
    assert_eq!(
        row.result().question().batch(),
        Some(thinkthen::BatchSetting::Records(
            std::num::NonZeroUsize::MIN
        ))
    );
    assert_eq!(
        row.result().question().on().collect::<Vec<_>>(),
        ["/payload"]
    );
    assert_eq!(row.result().question().name().unwrap().as_str(), "selected");
    assert_eq!(row.original(), &input);
    let body = &listener.requests()[0].body;
    assert_eq!(
        serde_json::from_slice::<Value>(body).unwrap(),
        json!({"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Yes.\". Refund?"}}})
    );
    let rank = Question::rank_from_json(authored).unwrap();
    let ranked = engine
        .rank_records_complete_with(&rank, records(), CallOptions::new())
        .unwrap();
    assert_eq!(ranked.value()[0].ordinal(), 0);
    assert_eq!(
        ranked.value()[0]
            .result()
            .question()
            .on()
            .collect::<Vec<_>>(),
        ["/payload"]
    );
    assert_eq!(listener.count(), 2);
    let missing = thinkthen::QuestionInput::annotation_text(
        r#"{"other":"No."}"#,
        thinkthen::SourceLocation::new("bad.jsonl".into(), Some(1), Some(1)).unwrap(),
    )
    .unwrap();
    let error = engine
        .decide_records_complete_with(
            &question,
            [thinkthen::RecordInput {
                original: missing,
                ..records()[0].clone()
            }],
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(error.facts().is_none());
    assert_eq!(listener.count(), 2);
}

#[test]
fn located_annotation_documents_use_the_native_json_or_literal_reading_without_losing_source() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.8}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let set = thinkthen::QuestionSet::from_json(
        r#"{"version":1,"questions":{"selected":{"decide":"Refund?","on":["/payload"]}}}"#,
    )
    .unwrap();
    let location = thinkthen::SourceLocation::new("input.txt".into(), Some(3), Some(3)).unwrap();
    let structured =
        thinkthen::QuestionInput::annotation_text(r#"{"payload":"Yes."}"#, location.clone())
            .unwrap();
    let call = engine
        .annotate_records_complete_with(
            &set,
            [RecordInput {
                examples: None,
                seed_spans: None,
                original: structured.clone(),
                context: None,
                options: None,
            }],
            CallOptions::new(),
        )
        .unwrap();
    assert_eq!(call.value()[0].original(), &structured);
    let thinkthen::QuestionInput::Record(record) = &structured else {
        panic!("record")
    };
    assert_eq!(record.location(), Some(&location));
    assert!(record.original().content().is_some());
    let literal = thinkthen::QuestionInput::annotation_text("{plain text", location).unwrap();
    let root = thinkthen::QuestionSet::builder()
        .question("selected", Question::decide("Refund?").unwrap().cut())
        .unwrap()
        .build()
        .unwrap();
    let call = engine
        .annotate_records_complete_with(
            &root,
            [RecordInput {
                examples: None,
                seed_spans: None,
                original: literal.clone(),
                context: None,
                options: None,
            }],
            CallOptions::new(),
        )
        .unwrap();
    assert_eq!(call.value()[0].original(), &literal);
    let thinkthen::QuestionInput::Record(record) = &literal else {
        panic!("record")
    };
    assert_eq!(record.original().literal(), Some("{plain text"));
    assert_eq!(listener.count(), 2);
    assert!(
        listener
            .requests()
            .iter()
            .all(|request| !String::from_utf8_lossy(&request.body).contains("input.txt"))
    );
}

#[test]
fn atomic_and_rank_pointer_reading_preserves_literal_text_and_admits_explicit_json() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.8}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let authored =
        r#"{"decide":"Refund?","on":"/payload","item_schema":{"type":"object","properties":{}}}"#;
    let atomic = Question::from_json(authored).unwrap();
    let rank = Question::rank_from_json(authored).unwrap();
    let text = r#"{"payload":{}}"#;
    let reading = thinkthen::RecordReading::new(&[], None, None).unwrap();
    let literal = reading
        .compose(thinkthen::RawRecord::text(text).unwrap())
        .unwrap()
        .original;
    for input in [
        thinkthen::QuestionInput::Text(text.into()),
        thinkthen::QuestionInput::Record(literal),
    ] {
        let records = || {
            [RecordInput {
                examples: None,
                seed_spans: None,
                original: input.clone(),
                context: None,
                options: None,
            }]
        };
        assert_eq!(
            engine
                .decide_records_complete_with(&atomic, records(), CallOptions::new())
                .unwrap_err()
                .kind(),
            ErrorKind::Usage
        );
        assert_eq!(
            engine
                .rank_records_complete_with(&rank, records(), CallOptions::new())
                .unwrap_err()
                .kind(),
            ErrorKind::Usage
        );
    }
    assert_eq!(listener.count(), 0);
    let parsed = reading
        .compose(thinkthen::RawRecord::json(text).unwrap())
        .unwrap()
        .original;
    let located = thinkthen::QuestionInput::annotation_text(
        text,
        thinkthen::SourceLocation::new("typed.json".into(), None, None).unwrap(),
    )
    .unwrap();
    for input in [thinkthen::QuestionInput::Record(parsed), located] {
        let records = || {
            [RecordInput {
                examples: None,
                seed_spans: None,
                original: input.clone(),
                context: None,
                options: None,
            }]
        };
        engine
            .decide_records_complete_with(&atomic, records(), CallOptions::new())
            .unwrap();
        engine
            .rank_records_complete_with(&rank, records(), CallOptions::new())
            .unwrap();
    }
    assert_eq!(listener.count(), 4);
    for request in listener.requests() {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(
            body["state"],
            "Each question quotes the text it asks about."
        );
        assert!(!String::from_utf8(request.body).unwrap().contains("payload"));
    }
}

#[test]
fn annotation_documents_send_structural_json_without_inventing_a_location() {
    let listener = Listener::answering(|body| {
        assert_eq!(serde_json::from_slice::<Value>(body).unwrap(), json!({
            "state":"Each question quotes the text it asks about.","model":"fixed",
            "questions":{"q1":{"type":"noul","instructions":r#"The text is "{\"body\":\"x\",\"ready\":false}". Refund?"#}}
        }));
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.8}}}"#)
    }).unwrap();
    let engine = engine(&listener);
    let set = thinkthen::QuestionSet::from_json(r#"{"version":1,"questions":{"result":{"decide":"Refund?","item_schema":{"type":"object","properties":{"body":{"type":"string"},"ready":{"type":"boolean"}},"required":["body"]}}}}"#).unwrap();
    let input =
        thinkthen::QuestionInput::annotation_document(r#"{"body":"x","ready":false}"#).unwrap();
    let thinkthen::QuestionInput::Record(record) = &input else {
        panic!("native document")
    };
    assert!(record.location().is_none());
    let call = engine
        .annotate_records_complete_with(
            &set,
            [RecordInput {
                examples: None,
                seed_spans: None,
                original: input.clone(),
                context: None,
                options: None,
            }],
            CallOptions::new(),
        )
        .unwrap();
    assert_eq!(call.value()[0].original(), &input);
    assert_eq!(call.facts().requests_sent(), 1);
    let mut batch = engine.try_annotate_records_complete_with(
        &set,
        [Ok(RecordInput {
            examples: None,
            seed_spans: None,
            original: input.clone(),
            context: None,
            options: None,
        })],
        CallOptions::new(),
    );
    assert_eq!(batch.next().unwrap().unwrap().original(), &input);
    assert!(batch.next().is_none());
    assert_eq!(listener.count(), 2);
}

#[test]
fn annotation_document_constructor_retains_native_literal_and_refusal_boundaries() {
    let literal = thinkthen::QuestionInput::annotation_document("{literal document").unwrap();
    let thinkthen::QuestionInput::Record(record) = literal else {
        panic!("native document")
    };
    assert_eq!(record.original().literal(), Some("{literal document"));
    assert!(record.location().is_none());
    let depth = format!("{}0{}", "[".repeat(128), "]".repeat(128));
    let oversized = "x".repeat(16 * 1024 * 1024 + 1);
    for input in [" ", r#"{"private":1,"private":2}"#, &depth, &oversized] {
        let error = thinkthen::QuestionInput::annotation_document(input).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert!(!error.to_string().contains("private"));
    }
}
