//! The quoted form of one record, by ADR 0111 section 1. The packer tests
//! in `pack` cover how questions join into requests.

use super::{BatchError, QUOTED, quoted_plan, quoted_plan_of};
use crate::core::adapters::built_in;
use crate::core::backend_profile::{BackendProfile, LimitKind};
use crate::core::json::Json;
use crate::core::question::Question;
use crate::core::text::{Evidence, ModelName, QuestionText};

fn model() -> ModelName {
    ModelName::new("jev-latest").expect("model")
}

fn decide(text: &str) -> Question {
    Question::Decide {
        text: QuestionText::new(text).expect("text"),
        yes: None,
        no: None,
    }
}

/// A question written as JSON, which cannot take the quote prefix.
fn structured() -> Question {
    let text = Json::parse(r#"{"ask":"urgent?"}"#).expect("json");
    Question::Decide {
        text: QuestionText::structured(&text).expect("text"),
        yes: None,
        no: None,
    }
}

fn evidence(text: &str) -> Evidence {
    Evidence::new(text).expect("evidence")
}

fn body(record: &str, question: Question, context: Option<&Evidence>) -> String {
    let plan = quoted_plan(model(), evidence(record), context, vec![question], None);
    String::from_utf8(built_in::encode(&plan.expect("plan")).expect("body")).expect("text")
}

const URGENT: &str = "Help! My payouts have been failing for 3 days.";

#[test]
fn a_record_is_quoted_in_its_question_and_a_json_question_keeps_it_as_the_state() {
    assert_eq!(
        body(URGENT, decide("Does this convey urgency?"), None),
        r#"{"state":"Each question quotes the text it asks about.","model":"jev-latest","questions":{"q1":{"type":"noul","instructions":"The text is \"Help! My payouts have been failing for 3 days.\". Does this convey urgency?"}}}"#
    );
    let context = evidence("shared context");
    assert!(body("x", decide("Q"), Some(&context)).starts_with(r#"{"state":"shared context","#));
    assert_eq!(
        body(URGENT, structured(), None),
        r#"{"state":"Help! My payouts have been failing for 3 days.","model":"jev-latest","questions":{"q1":{"type":"noul","instructions":{"ask":"urgent?"}}}}"#
    );
    assert_eq!(
        quoted_plan(model(), evidence("x"), Some(&context), vec![structured()], None).err(),
        Some(BatchError::StructuredQuestionWithContext)
    );
}

#[test]
fn a_json_record_is_quoted_compact_in_its_own_key_order() {
    for (value, expected) in [
        ("7", "The text is 7. Q"),
        (r#"{"b":2,"a":1}"#, r#"The text is {"b":2,"a":1}. Q"#),
    ] {
        let plan = quoted_plan_of(
            model(),
            evidence(value),
            &Json::parse(value).expect("json"),
            None,
            vec![decide("Q")],
            None,
        )
        .expect("plan");
        let [Question::Decide { text, .. }] = plan.questions() else {
            panic!("one decide question")
        };
        assert_eq!(text.as_json().as_str(), Some(expected));
    }
}

#[test]
fn the_evidence_limit_bounds_each_quoted_record() {
    let state = QUOTED.len();
    let profile = BackendProfile::parse(&format!(
        r#"{{"schema":"thinkthen.backend-profile/1","name":"test","max_evidence_bytes":{state}}}"#
    ))
    .expect("profile");
    let bounded = |bytes: usize| {
        quoted_plan(
            model(),
            evidence(&"x".repeat(bytes)),
            None,
            vec![decide("Q")],
            Some(&profile),
        )
    };
    assert!(bounded(state).is_ok(), "a record at the limit passes");
    assert!(
        matches!(bounded(state + 1), Err(BatchError::Profile(limit))
            if limit.kind == LimitKind::EvidenceBytes && limit.actual == state + 1),
        "the evidence limit bounds each quoted record"
    );
}
