use super::{Atomic, Observation, Origin, QuestionSource, ResultIdentity};
use crate::core::probability::Probability;
use crate::core::{
    Answer, AnswerId, DecisionResult, Meta, ModelName, ObservationId, Question, QuestionText,
    RequestMeta, Threshold, Url, json_line,
};

mod aggregate;

fn atomic(sources: Vec<QuestionSource>, observations: Vec<Observation>) -> Atomic {
    let answer = Answer::new_yes_no(Probability::new(0.9).unwrap());
    let threshold = Threshold::default();
    Atomic {
        declarations: Default::default(),
        rank_position: None,
        source: None,
        legacy: DecisionResult::new(
            answer.read(Some(threshold)).0,
            Question::Decide {
                text: QuestionText::new("refund?").unwrap(),
                yes: None,
                no: None,
            },
            answer,
            Some(threshold),
            Meta::new(
                "0.2.0",
                "question".into(),
                Url::new("http://localhost/decisions").unwrap(),
                ModelName::new("requested").unwrap(),
                None,
                RequestMeta::new(true, 0, (0..sources.len()).map(|_| "key".into()).collect()),
            ),
        ),
        identity: ResultIdentity {
            answer_id: AnswerId::new("a".repeat(64)).unwrap(),
            origin: sources.first().map(QuestionSource::origin),
            answered_by: sources.first().map(|source| source.answered_by.clone()),
            question_sources: sources,
            observations,
        },
    }
}

#[test]
fn complete_rank_serializes_a_positive_integer_while_legacy_keeps_its_value() {
    let mut canonical = atomic(Vec::new(), Vec::new());
    canonical.legacy.value = crate::core::Value::YesNo(None);
    canonical.legacy.threshold = None;
    canonical.rank_position = std::num::NonZeroUsize::new(7);
    let json = json_line(&canonical).unwrap();
    assert!(json.contains(r#""value":7,"question":"#));
    assert!(json.contains(r#""threshold":null"#));
    assert!(
        json_line(&canonical.legacy)
            .unwrap()
            .contains(r#""value":null,"question":"#)
    );
}

#[test]
fn complete_atomic_serialization_keeps_legacy_projection_and_reports_actual_sources() {
    let canonical = atomic(
        vec![QuestionSource {
            origin: Origin::Replay,
            answered_by: ModelName::new("actual").unwrap(),
            batch_size: std::num::NonZeroU32::new(13),
        }],
        vec![Observation::Answered {
            observation_id: ObservationId::new("b".repeat(64)).unwrap(),
        }],
    );
    let expected = format!(
        concat!(
            "{{\"schema\":\"thinkthen.result/2\",\"answer_id\":\"{}\",\"value\":true,",
            "\"question\":{{\"verb\":\"decide\",\"text\":\"refund?\"}},\"answer\":{{\"kind\":\"yes_no\",\"probability\":0.9}},\"threshold\":0.5,",
            "\"meta\":{{\"tool\":\"thinkthen 0.2.0\",\"question_sha256\":\"question\",\"url\":\"http://localhost/decisions\",\"model\":\"requested\",",
            "\"requests_sent\":0,\"cached\":true,\"requests\":[\"key\"],\"failed_questions\":0,\"origin\":\"replay\",",
            "\"question_sources\":[{{\"origin\":\"replay\",\"answered_by\":\"actual\",\"batch_size\":13}}],\"observations\":[{{\"observation_id\":\"{}\"}}],\"answered_by\":\"actual\"}}}}"
        ),
        "a".repeat(64),
        "b".repeat(64)
    );
    assert_eq!(json_line(&canonical).unwrap(), expected);
    let legacy = json_line(&canonical.legacy).unwrap();
    assert!(legacy.starts_with(r#"{"schema":"thinkthen.result/1","value":true,"question":"#));
    assert!(!legacy.contains("observation_id"));
    assert!(!legacy.contains("answer_id"));
}

#[test]
fn zero_observation_metadata_does_not_invent_a_cache_hit_or_answered_model() {
    let canonical = atomic(Vec::new(), Vec::new());
    let json = json_line(&canonical).unwrap();
    assert!(json.contains(r#""cached":false,"requests":[],"failed_questions":0,"origin":null,"question_sources":[],"observations":[]"#));
    assert!(!json.contains("answered_by"));
    assert!(!json.contains("usage"));
}

#[test]
fn complete_find_keeps_its_whole_set_question_and_all_ordered_probabilities() {
    use crate::core::answer::Distribution;
    use crate::core::{Evidence, Find};
    let find = Find::new(
        QuestionText::new("Which one?").unwrap(),
        &[
            Evidence::new("alpha").unwrap(),
            Evidence::new("beta").unwrap(),
        ],
        ModelName::new("requested").unwrap(),
        true,
    )
    .unwrap();
    let distribution = Distribution::new(
        [("u001", 0.3), ("u002", 0.2), ("none", 0.5)]
            .into_iter()
            .map(|(label, p)| (label.into(), Probability::new(p).unwrap()))
            .collect(),
    )
    .unwrap();
    let answer = Answer::new_choice(distribution, None).unwrap();
    let base = atomic(Vec::new(), Vec::new());
    let canonical = super::Find {
        declarations: Default::default(),
        identity: base.identity,
        legacy: find.result(None, find.select(&answer).unwrap(), base.legacy.meta),
    };
    let document = json_line(&canonical).unwrap();
    assert!(document.starts_with(&format!(
        r#"{{"schema":"thinkthen.result/2","answer_id":"{}","value":null,"question":{{"verb":"find","text":"Which one?","none":true}},"answer":{{"kind":"find","pick":"none","probabilities":{{"u001":0.3,"u002":0.2,"none":0.5}}}},"threshold":null,"meta":"#,
        "a".repeat(64),
    )));
    assert!(!document.contains("confidence"));
    assert!(
        json_line(&canonical.legacy).unwrap().starts_with(
            r#"{"schema":"thinkthen.result/1","value":null,"question":{"verb":"find""#,
        )
    );
}

#[test]
fn annotation_successful_null_and_failure_have_exclusive_typed_identities() {
    use super::AnnotationMember;
    use crate::core::{
        AnnotatedAnswer, AnnotatedEntry, AnnotatedFailure, BackendFailure, BackendFailureCause,
        MemberIdentity, Value,
    };
    let row = atomic(Vec::new(), Vec::new());
    let question = row.legacy.question;
    let answer = row.legacy.answer;
    let successful = AnnotationMember {
        declarations: Default::default(),
        threshold: Some(Threshold::band(0.2, 0.95).unwrap()),
        sources: Vec::new(),
        observations: Vec::new(),
        reported_usage: None,
        identity: MemberIdentity::Answered(AnswerId::new("c".repeat(64)).unwrap()),
        legacy: AnnotatedEntry::Answered(AnnotatedAnswer::new(
            Value::YesNo(None),
            question.clone(),
            answer,
            Some(Threshold::band(0.2, 0.95).unwrap()),
            "success-key".into(),
        )),
    };
    let failed = AnnotationMember {
        declarations: Default::default(),
        threshold: Some(Threshold::default()),
        sources: Vec::new(),
        observations: Vec::new(),
        reported_usage: None,
        identity: MemberIdentity::Failed(crate::core::FailureId::new("d".repeat(64)).unwrap()),
        legacy: AnnotatedEntry::Failed(AnnotatedFailure::new(
            question,
            BackendFailure::new(BackendFailureCause::MissingProbability),
            "failure-key".into(),
        )),
    };
    assert_eq!(successful.value(), Some(&Value::YesNo(None)));
    assert_eq!(failed.value(), None);
    let success_json = json_line(&successful).unwrap();
    assert!(success_json.starts_with(&format!(
        r#"{{"answer_id":"{}","value":null,"question":"#,
        "c".repeat(64)
    )));
    assert!(!success_json.contains("failure"));
    let failure_json = json_line(&failed).unwrap();
    assert_eq!(
        failure_json,
        format!(
            r#"{{"failure_id":"{}","question":{{"verb":"decide","text":"refund?"}},"failure":{{"kind":"backend","cause":"missing_probability"}},"request":"failure-key"}}"#,
            "d".repeat(64)
        )
    );
    for absent in [
        r#""answer":"#,
        r#""value":"#,
        r#""threshold":"#,
        r#""answer_id":"#,
    ] {
        assert!(!failure_json.contains(absent));
    }
    let mismatched = AnnotationMember {
        declarations: Default::default(),
        identity: MemberIdentity::Answered(AnswerId::new("c".repeat(64)).unwrap()),
        legacy: failed.legacy,
        threshold: failed.threshold,
        sources: failed.sources,
        observations: failed.observations,
        reported_usage: failed.reported_usage,
    };
    assert!(json_line(&mismatched).is_err());
}
