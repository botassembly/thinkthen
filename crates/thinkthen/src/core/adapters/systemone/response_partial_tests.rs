use super::decode;
use crate::core::adapters::systemone::DecodeError;
use crate::core::adapters::systemone::tests::{plan_for, urgency_plan};
use crate::core::{AnswerOutcome, Evidence, Labels, ModelName, Plan, Question, QuestionText};

fn mixed_choice_plan() -> Plan {
    Plan::authored(
        Evidence::new("Help!").expect("evidence"),
        ModelName::new("jev-latest").expect("model"),
        vec![
            Question::Decide {
                text: QuestionText::new("good?").expect("question"),
                yes: None,
                no: None,
            },
            Question::Choose {
                text: QuestionText::new("which?").expect("question"),
                options: Labels::options(vec!["a".to_owned(), "b".to_owned()]).expect("options"),
            },
        ],
    )
    .expect("plan")
}

#[test]
fn a_mixed_reply_preserves_its_valid_logical_answer() {
    let plan = plan_for("Help!", &["is urgent", "asks for a refund"]);
    let body = concat!(
        r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.75},"#,
        r#""q2":{"type":"choice","probabilities":{"yes":1.0}}}}"#,
    );
    let reply = decode(&plan, body.as_bytes()).expect("one valid answer makes this partial");
    assert!(matches!(
        reply.outcomes(),
        [AnswerOutcome::Answered(_), AnswerOutcome::Failed(_)]
    ));
}

#[test]
fn the_key_order_of_an_answers_object_carries_nothing() {
    let plan = plan_for("Help!", &["is urgent", "asks for a refund"]);
    let body = concat!(
        r#"{"answers":{"q2":{"type":"noul","noul":0.25},"#,
        r#""q1":{"type":"noul","noul":0.75}},"model":"jev-latest"}"#
    );
    let reply = decode(&plan, body.as_bytes()).expect("a systemone response");
    let [
        AnswerOutcome::Answered(first),
        AnswerOutcome::Answered(second),
    ] = reply.outcomes()
    else {
        panic!("one answer per planned question");
    };
    assert_eq!(
        serde_json::to_string(first).expect("an answer"),
        r#"{"kind":"yes_no","probability":0.75}"#
    );
    assert_eq!(
        serde_json::to_string(second).expect("an answer"),
        r#"{"kind":"yes_no","probability":0.25}"#
    );
}

#[test]
fn an_unexpected_answer_name_refuses_the_whole_reply() {
    // Ticket 0132 reads the number from the name, so each spelling the number
    // parser takes but `wire_name` never writes stays refused.
    for name in ["q2", "q0", "q01", "q+1", "Q1", "q", "1"] {
        let body = format!(
            r#"{{"model":"jev-1.13.0","answers":{{"q1":{{"type":"noul","noul":0.5}},"{name}":{{"type":"noul","noul":0.1}}}}}}"#
        );
        assert_eq!(
            decode(&urgency_plan(), body.as_bytes()),
            Err(DecodeError::UnexpectedAnswer),
            "{name}"
        );
    }
}

#[test]
fn a_response_decodes_without_usage_and_past_unknown_fields() {
    let body = concat!(
        r#"{"model":"jev-1.13.0","request_id":"abc","#,
        r#""answers":{"q1":{"type":"noul","noul":0.5,"rationale":"none"}}}"#,
    );
    let reply = decode(&urgency_plan(), body.as_bytes()).expect("a systemone response");
    assert_eq!(reply.model().as_str(), "jev-1.13.0");
    assert!(matches!(reply.outcomes(), [AnswerOutcome::Answered(_)]));
    assert_eq!(reply.usage(), None);
}

#[test]
fn every_partial_failure_cause_has_its_exact_marker() {
    let cases = [
        (
            r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}}}"#,
            "missing_answer",
        ),
        (
            r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.5}}}"#,
            "wrong_kind",
        ),
        (
            r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"choice","probabilities":{"a":1.0}}}}"#,
            "missing_probability",
        ),
        (
            r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"choice","probabilities":{"a":1.2,"b":-0.2}}}}"#,
            "invalid_probability",
        ),
        (
            r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"choice","probabilities":{"a":0.2,"b":0.2}}}}"#,
            "invalid_distribution",
        ),
        (
            r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"choice","probabilities":{"a":0.5,"b":0.5,"c":0.0}}}}"#,
            "unexpected_probability",
        ),
    ];
    for (body, cause) in cases {
        let reply = decode(&mixed_choice_plan(), body.as_bytes()).expect("a partial reply");
        let [AnswerOutcome::Answered(_), AnswerOutcome::Failed(failure)] = reply.outcomes() else {
            panic!("one answer and one failure");
        };
        assert_eq!(
            serde_json::to_string(failure).expect("a failure serializes"),
            format!(r#"{{"kind":"backend","cause":"{cause}"}}"#)
        );
    }
}
