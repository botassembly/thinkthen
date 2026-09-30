use super::{QuestionKey, State, asks, read, shares};
use crate::core::adapters::built_in;
use crate::core::question::{Labels, Question};
use crate::core::reply::{AnswerOutcome, BackendFailure, BackendFailureCause};
use crate::core::result::Usage;
use crate::core::text::{Evidence, ModelName, QuestionText, Url};
use crate::core::{Plan, quoted_plan};

const URL: &str = "https://api.typesafe.ai/v1/systemone";

fn url() -> Url {
    Url::new(URL).expect("an address")
}

fn decide(text: &str) -> Question {
    Question::Decide {
        text: QuestionText::new(text).expect("not blank"),
        yes: None,
        no: None,
    }
}

fn quoted(record: &str, question: Question) -> Plan {
    quoted_plan(
        ModelName::new("jev-1.13.0").expect("a model"),
        Evidence::new(record).expect("not blank"),
        None,
        vec![question],
        None,
    )
    .expect("a quoted plan")
}

/// The SHA-256 of `systemone`, the address, `"jev-1.13.0"`, the fixed
/// sentence as a JSON string and the question below, joined by line feeds,
/// taken outside this program with `printf … | sha256sum`.
#[test]
fn the_key_hashes_the_five_parts_as_sent() {
    let plan = quoted("Ringo", decide("Is this a drummer?"));
    let asked = asks(&url(), &plan).expect("asks");
    let [ask] = asked.as_slice() else {
        panic!("one question")
    };
    assert_eq!(
        &*ask.question,
        r#"{"type":"noul","instructions":"The text is \"Ringo\". Is this a drummer?"}"#
    );
    assert_eq!(
        ask.key.hex(),
        "6950b38cb4dbbf9ccc90be362a126dee1e40c1ff72dd1f81168ccd37a110fa5f"
    );
    assert_eq!(
        crate::core::digest::hex(ask.state.sha256()),
        "4f9c0ce7d0e33fbd5efeb2f53966ef2798e02d35bc63309a0f9f36fbdbc89f4a"
    );
    assert_eq!(QuestionKey::parse(&ask.key.hex()), Some(ask.key));
    assert_eq!(QuestionKey::parse("6950"), None);
    assert_eq!(QuestionKey::parse(&"G".repeat(64)), None);
}

#[test]
fn the_body_joins_the_bytes_each_key_hashes() {
    let tag = Question::Tag {
        text: QuestionText::new("Which topics?").expect("not blank"),
        labels: Labels::tags(vec![
            ("billing".to_owned(), None),
            ("urgent".to_owned(), None),
        ])
        .expect("two labels"),
    };
    let plan = quoted("The invoice failed.", tag);
    let asked = asks(&url(), &plan).expect("asks");
    assert_eq!(asked.len(), 2);
    let body = built_in::join(
        asked[0].state.json(),
        "\"jev-1.13.0\"",
        asked.iter().map(|ask| &*ask.question),
    );
    assert_eq!(body, built_in::encode(&plan).expect("a body"));
    assert!(
        asked
            .iter()
            .all(|ask| matches!(ask.decoder, Question::Decide { .. }))
    );
    assert_ne!(asked[0].key, asked[1].key);
    assert_eq!(
        asked[0].state,
        State::new(asked[1].state.json().to_owned(), 0)
    );
}

#[test]
fn a_tag_answer_joins_its_labels_and_fails_with_its_first_failed_label() {
    let tag = Question::Tag {
        text: QuestionText::new("Which topics?").expect("not blank"),
        labels: Labels::tags(vec![
            ("billing".to_owned(), None),
            ("urgent".to_owned(), None),
        ])
        .expect("two labels"),
    };
    let noul = r#"{"type":"noul","noul":0.75}"#;
    let joined = read(std::slice::from_ref(&tag), &[Ok(noul), Ok(noul)], "jev-1").expect("read");
    assert!(matches!(joined.as_slice(), [AnswerOutcome::Answered(_)]));
    let failed = read(
        &[tag, decide("Is it late?")],
        &[
            Ok(noul),
            Err(BackendFailureCause::InvalidProbability),
            Ok(r#"{"type":"noul","noul":0.1}"#),
        ],
        "jev-1",
    )
    .expect("read");
    let [AnswerOutcome::Failed(failure), AnswerOutcome::Answered(_)] = failed.as_slice() else {
        panic!("one failed tag and one answer")
    };
    assert_eq!(
        *failure,
        BackendFailure::new(BackendFailureCause::InvalidProbability)
    );
}

#[test]
fn a_stored_answer_of_the_wrong_shape_does_not_decode() {
    let wrong = r#"{"type":"choice","probabilities":{"a":1.0}}"#;
    assert!(read(&[decide("Is it late?")], &[Ok(wrong)], "jev-1").is_err());
}

#[test]
fn shares_split_usage_evenly_with_the_remainder_to_the_earliest() {
    assert_eq!(
        shares(Some(Usage::new(10, 3)), 4),
        vec![
            Some(Usage::new(3, 1)),
            Some(Usage::new(3, 1)),
            Some(Usage::new(2, 1)),
            Some(Usage::new(2, 0)),
        ]
    );
    assert_eq!(shares(None, 2), vec![None, None]);
}
