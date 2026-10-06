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
