//! The later recognition stages report the actual question before the final row.

use super::*;

#[test]
fn recognition_observes_edge_and_stated_relation_in_order() {
    let _serial = serial();
    let listener = Listener::answering(|body| {
        let request: serde_json::Value = serde_json::from_slice(body).expect("request");
        let mut answers = serde_json::Map::new();
        for (name, question) in request["questions"].as_object().expect("questions") {
            if question["type"] == "noul" {
                answers.insert(name.clone(), serde_json::json!({"type":"noul","noul":0.9}));
                continue;
            }
            let labels = question["criteria"].as_object().expect("labels");
            let words = question["instructions"].as_str().expect("instructions");
            let picked = if labels.contains_key("SINGLE") {
                if words.contains("[[Ada]]") || words.contains("[[Acme]]") {
                    "SINGLE"
                } else {
                    "OUT"
                }
            } else if labels.contains_key("none of these") {
                if words.contains("[[Acme]]") {
                    "organization"
                } else {
                    "person"
                }
            } else {
                labels.keys().next().expect("edge option")
            };
            assert!(labels.contains_key(picked), "fixture choice {picked}");
            let probabilities = labels
                .keys()
                .map(|label| (label.clone(), serde_json::json!(u8::from(label == picked))))
                .collect::<serde_json::Map<_, _>>();
            answers.insert(name.clone(), serde_json::json!({"type":"choice","choice":picked,"probabilities":probabilities}));
        }
        Canned::ok(&serde_json::json!({"model":"jev-latest","answers":answers,"usage":{"input_tokens":3,"output_tokens":1}}).to_string())
    })
    .expect("listener");
    let engine = engine(listener.base());
    let asked = thinkthen::Recognize::builder()
        .kind(thinkthen::Kind::new("person", None).expect("person"))
        .and_then(|builder| {
            builder.kind(thinkthen::Kind::new("organization", None).expect("organization"))
        })
        .and_then(|builder| {
            builder.relation(
                thinkthen::RelationRule::one_way("works_with", "person", "organization")
                    .expect("rule"),
            )
        })
        .and_then(thinkthen::RecognizeBuilder::build)
        .expect("recognize");
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| match event {
        RecordObservation::Question { stage, detail, .. } => {
            assert_eq!(detail.question_sha256().len(), 64);
            assert_eq!(detail.requests().len(), 1);
            seen.lock()
                .expect("observations")
                .push(stage.expect("stage"));
        }
        RecordObservation::Row { value, .. } => {
            assert!(matches!(value, ObservedRow::Recognized(_)));
            seen.lock().expect("observations").push("row");
        }
    };
    let found = engine
        .recognize_with(
            &asked,
            "Ada, met Acme.",
            CallOptions::new().observe(&observe),
        )
        .expect("recognized");
    assert_eq!(found.value().entities().len(), 2);
    assert_eq!(found.value().relations().map(<[_]>::len), Some(1));
    let stages = seen.lock().expect("observations");
    assert_eq!(stages.last(), Some(&"row"));
    assert!(
        stages
            .iter()
            .position(|stage| *stage == "edge")
            .is_some_and(|at| at
                < stages
                    .iter()
                    .position(|stage| *stage == "relation")
                    .expect("relation stage"))
    );
    assert!(
        stages
            .iter()
            .take_while(|stage| **stage != "kind")
            .all(|stage| *stage == "boundary")
    );
    assert_eq!(listener.count(), 3);
}
