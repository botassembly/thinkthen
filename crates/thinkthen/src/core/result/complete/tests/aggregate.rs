use crate::core::probability::Probability;
use crate::core::result::complete::{
    MemberIdentity, Recognition, Relation, RelationDirection, RelationEntry, RelationMethod,
};
use crate::core::{
    Answer, AnswerId, BackendFailure, BackendFailureCause, FailureId, NameOdds, Odds, PairOdds,
    PieceOdds, Place, RecognitionOdds, RecognizeSpec, RecognizedValue, RelateSpec, RelationEntity,
    json_line,
};

#[test]
fn complete_recognition_keeps_every_stage_probability_without_outer_atomic_answer() {
    let base = super::atomic(Vec::new(), Vec::new());
    let canonical = Recognition {
        identity: base.identity,
        value: RecognizedValue {
            entities: Vec::new(),
            relations: Some(Vec::new()),
        },
        input: None,
        question: RecognizeSpec::parse(
            r#"{"version":1,"recognize":{"kinds":{"person":"A person."}}}"#,
        )
        .unwrap(),
        answer: RecognitionOdds {
            pieces: vec![PieceOdds {
                start: 0,
                end: 3,
                tags: Odds(vec![
                    ("B".into(), 0.1),
                    ("I".into(), 0.0),
                    ("L".into(), 0.1),
                    ("O".into(), 0.0),
                    ("U".into(), 0.8),
                ]),
            }],
            names: vec![NameOdds {
                start: 0,
                end: 3,
                kinds: Some(Odds(vec![("person".into(), 0.8), ("none".into(), 0.2)])),
                edges: None,
            }],
            pairs: vec![PairOdds {
                relation: "works_for".into(),
                source: Place { start: 0, end: 3 },
                target: Place { start: 8, end: 12 },
                probability: 0.3,
            }],
        },
        meta: base.legacy.meta,
    };
    let document: serde_json::Value =
        serde_json::from_str(&json_line(&canonical).unwrap()).unwrap();
    assert_eq!(
        document.get("schema"),
        Some(&serde_json::json!("thinkthen.result/2"))
    );
    assert_eq!(
        document.get("value"),
        Some(&serde_json::json!({"entities":[],"relations":[]}))
    );
    assert_eq!(
        document.get("answer"),
        Some(&serde_json::json!({
            "pieces":[{"start":0,"end":3,"tags":{"B":0.1,"I":0.0,"L":0.1,"O":0.0,"U":0.8}}],
            "names":[{"start":0,"end":3,"kinds":{"person":0.8,"none":0.2},"edges":null}],
            "pairs":[{"relation":"works_for","source":{"start":0,"end":3},"target":{"start":8,"end":12},"probability":0.3}]
        }))
    );
    assert!(document.get("threshold").is_none());
    assert!(document.get("input").is_none());
}

fn member() -> RelationEntry {
    RelationEntry {
        question: super::atomic(Vec::new(), Vec::new()).legacy.question,
        threshold: crate::core::Threshold::default(),
        sources: Vec::new(),
        observations: Vec::new(),
        reported_usage: None,
        identity: MemberIdentity::Answered(AnswerId::new("b".repeat(64)).unwrap()),
        relation: "works_for".into(),
        reads: "works for".into(),
        method: RelationMethod::YesNo,
        direction: RelationDirection::SourceToTarget,
        source: RelationEntity::new("Ada", "person").unwrap(),
        target: Some(RelationEntity::new("Acme", "organization").unwrap()),
        answer: Some(Answer::new_yes_no(Probability::new(0.4).unwrap())),
        probability: Some(0.4),
        accepted: Some(false),
        failure: None,
        request: "saved-key".into(),
    }
}

#[test]
fn relation_members_emit_success_or_failure_with_no_fabricated_success_fields() {
    let answered = member();
    assert_eq!(
        json_line(&answered).unwrap(),
        format!(
            r#"{{"relation":"works_for","reads":"works for","method":"yes_no","direction":"source_to_target","source":{{"name":"Ada","kind":"person"}},"target":{{"name":"Acme","kind":"organization"}},"answer_id":"{}","probability":0.4,"accepted":false,"answer":{{"kind":"yes_no","probability":0.4}},"request":"saved-key"}}"#,
            "b".repeat(64),
        )
    );
    let mut failed = member();
    failed.identity = MemberIdentity::Failed(FailureId::new("c".repeat(64)).unwrap());
    failed.answer = None;
    failed.probability = None;
    failed.accepted = None;
    failed.failure = Some(BackendFailure::new(BackendFailureCause::MissingProbability));
    let failed_json = json_line(&failed).unwrap();
    assert!(failed_json.contains(&format!(
        r#""failure_id":"{}","failure":{{"kind":"backend","cause":"missing_probability"}}"#,
        "c".repeat(64)
    )));
    for absent in ["answer_id", "probability", "accepted"] {
        assert!(!failed_json.contains(&format!(r#""{absent}":"#)));
    }
    failed.answer = answered.answer;
    assert!(json_line(&failed).is_err());
}

#[test]
fn empty_complete_relation_is_a_zero_observation_aggregate() {
    let base = super::atomic(Vec::new(), Vec::new());
    let canonical = Relation {
        identity: base.identity, value: Vec::new(), members: Vec::new(), lines: false,
        question: RelateSpec::parse(
            r#"{"version":1,"relate":{"relations":[{"name":"link","source":"*","target":"*","single":true}]}}"#,
        ).unwrap(),
        meta: base.legacy.meta,
    };
    let document: serde_json::Value =
        serde_json::from_str(&json_line(&canonical).unwrap()).unwrap();
    assert_eq!(document.get("value"), Some(&serde_json::json!([])));
    assert_eq!(
        document.get("answer"),
        Some(&serde_json::json!({"questions":[]}))
    );
    let meta = document.get("meta").unwrap();
    assert_eq!(meta.get("origin"), Some(&serde_json::Value::Null));
    assert_eq!(meta.get("cached"), Some(&serde_json::json!(false)));
    for name in ["requests", "question_sources", "observations"] {
        assert_eq!(meta.get(name), Some(&serde_json::json!([])));
    }
    assert!(meta.get("answered_by").is_none());
}
