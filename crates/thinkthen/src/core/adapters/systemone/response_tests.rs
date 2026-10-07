use super::{decode, decode_observed};
use crate::core::adapters::systemone::DecodeError;
use crate::core::adapters::systemone::tests::{
    LEVELS, TEAMS, disruption_plan, tag_plan, team_plan, urgency_plan,
};
use crate::core::probability::Probability;
use crate::core::reply::AnswerOutcome;
use crate::core::result::Usage;

const RESPONSE: &str =
    include_str!("../../../../../../specification/fixtures/systemone/decide-urgent.response.json");
const MISSING: &str = include_str!(
    "../../../../../../specification/fixtures/systemone/refused-missing-answer.response.json"
);
const WRONG_KIND: &str = include_str!(
    "../../../../../../specification/fixtures/systemone/refused-wrong-kind.response.json"
);
const OUT_OF_RANGE: &str = include_str!(
    "../../../../../../specification/fixtures/systemone/refused-probability-out-of-range.response.json"
);
const CHOOSE: &str =
    include_str!("../../../../../../specification/fixtures/systemone/choose-team.response.json");
const SCORE: &str = include_str!(
    "../../../../../../specification/fixtures/systemone/score-disruption.response.json"
);
const MISSING_PROBABILITY: &str = include_str!(
    "../../../../../../specification/fixtures/systemone/refused-missing-probability.response.json"
);

#[test]
fn decode_reads_the_answer_the_fixture_shows() {
    let reply = decode(&urgency_plan(), RESPONSE.as_bytes()).expect("a systemone response");
    assert_eq!(reply.model().as_str(), "jev-latest");
    let [AnswerOutcome::Answered(answer)] = reply.outcomes() else {
        panic!("one answer per planned question");
    };
    assert_eq!(
        serde_json::to_string(answer).expect("an answer serializes"),
        r#"{"kind":"yes_no","probability":0.92}"#
    );
    assert_eq!(reply.usage(), Some(Usage::new(312, 48)));
}

#[test]
fn validated_usage_survives_when_every_answer_is_refused() {
    let body = br#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","probabilities":{"a":1.0}}},"usage":{"input_tokens":17,"output_tokens":3}}"#;
    let decoded = decode_observed(&urgency_plan(), body);
    assert_eq!(
        decoded.usage,
        Some(crate::core::ReportedUsage::from_complete(Usage::new(17, 3)))
    );
    assert!(matches!(decoded.reply, Err(DecodeError::WrongKind(0))));
}

#[test]
fn a_choice_answer_is_read_back_in_the_order_the_options_were_sent() {
    let reply = decode(&team_plan(), CHOOSE.as_bytes()).expect("a systemone response");
    let [AnswerOutcome::Answered(answer)] = reply.outcomes() else {
        panic!("one answer per planned question");
    };
    let distribution = answer
        .distribution()
        .expect("a choice carries a distribution");
    assert_eq!(distribution.labels().collect::<Vec<_>>(), TEAMS.to_vec());
    assert_eq!(answer.leader(), Some("billing"));
    assert_eq!(answer.confidence().map(Probability::as_f64), Some(1.0));
}

#[test]
fn a_score_answer_is_read_by_level_number_and_keyed_by_the_level_text() {
    let reply = decode(&disruption_plan(), SCORE.as_bytes()).expect("a systemone response");
    let [AnswerOutcome::Answered(answer)] = reply.outcomes() else {
        panic!("one answer per planned question");
    };
    let distribution = answer
        .distribution()
        .expect("a score carries a distribution");
    assert_eq!(distribution.labels().collect::<Vec<_>>(), LEVELS.to_vec());
    assert_eq!(
        distribution
            .probabilities()
            .map(Probability::as_f64)
            .collect::<Vec<_>>(),
        vec![0.0, 0.13, 0.87]
    );
    assert_eq!(answer.confidence().map(Probability::as_f64), Some(0.79));
}

#[test]
fn tag_answers_are_aggregated_in_label_order() {
    let body = br#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.2},"q2":{"type":"noul","noul":0.91}}}"#;
    let reply = decode(&tag_plan(), body).expect("a tag response");
    let [AnswerOutcome::Answered(answer)] = reply.outcomes() else {
        panic!("one logical tag answer");
    };
    assert_eq!(
        serde_json::to_string(answer).expect("answer serializes"),
        r#"{"kind":"tag","probabilities":{"bill\\\"ing":0.2,"urgent":0.91}}"#
    );
    assert_eq!(
        serde_json::to_string(&answer.read(None).0).expect("value serializes"),
        r#"["urgent"]"#
    );
}

#[test]
fn each_refused_response_names_its_own_cause() {
    let cases = [
        (MISSING, DecodeError::MissingAnswer(0)),
        (WRONG_KIND, DecodeError::WrongKind(0)),
        (OUT_OF_RANGE, DecodeError::ProbabilityOutOfRange(0)),
    ];
    for (body, expected) in cases {
        assert!(expected.to_string().contains("`q1`"), "{expected}");
        assert_eq!(
            decode(&urgency_plan(), body.as_bytes()),
            Err(expected.clone()),
            "{expected}"
        );
    }
    assert_eq!(
        decode(&team_plan(), MISSING_PROBABILITY.as_bytes()),
        Err(DecodeError::MissingProbability(0))
    );
}

#[test]
fn an_answer_of_another_shape_than_the_question_asked_is_refused() {
    let noul = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let choice = concat!(
        r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","choice":"billing","#,
        r#""probabilities":{"billing":1.0,"shipping":0.0,"account":0.0,"other":0.0}}}}"#,
    );
    assert_eq!(
        decode(&team_plan(), noul.as_bytes()),
        Err(DecodeError::WrongKind(0))
    );
    assert_eq!(
        decode(&disruption_plan(), choice.as_bytes()),
        Err(DecodeError::WrongKind(0))
    );
}

#[test]
fn a_body_that_is_not_a_systemone_response_is_refused() {
    let cases = ["", "not json at all", "[]", r#"{"model":"jev-latest"}"#];
    for body in cases {
        assert!(
            matches!(
                decode(&urgency_plan(), body.as_bytes()),
                Err(DecodeError::Malformed(..))
            ),
            "{body:?}"
        );
    }
}

/// A backend can quote the evidence back, and no refusal repeats it.
///
/// A JSON reader names the value it stopped on. That value came from the
/// backend's reply, which may hold whatever was sent to it, so the refusal
/// says where the reply broke and never what it held.
#[test]
fn no_refusal_of_a_reply_quotes_what_the_reply_held() {
    let evidence = "marker-evidence-7b3ac5";
    let body = format!(r#"{{"model":"jev-latest","answers":"{evidence}"}}"#);

    let error = decode(&urgency_plan(), body.as_bytes()).expect_err("a reply is refused");

    assert!(!error.to_string().contains(evidence), "{error}");
}

#[test]
fn a_response_that_does_not_name_the_model_is_refused() {
    let answers = r#""answers":{"q1":{"type":"noul","noul":0.5}}}"#;
    for body in [
        format!(r#"{{"model":"",{answers}"#),
        format!(r#"{{"model":" \t ",{answers}"#),
    ] {
        assert_eq!(
            decode(&urgency_plan(), body.as_bytes()),
            Err(DecodeError::NoModel),
            "{body}"
        );
    }

    // A reply with no `model` field at all never reaches the name at all.
    let missing = format!("{{{answers}");
    assert!(
        matches!(
            decode(&urgency_plan(), missing.as_bytes()),
            Err(DecodeError::Malformed(..))
        ),
        "{missing}"
    );
}

#[test]
fn an_extra_choice_or_score_key_is_refused() {
    let choice = concat!(
        r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","#,
        r#""probabilities":{"billing":1.0,"shipping":0.0,"account":0.0,"other":0.0,"extra":0.0}}}}"#,
    );
    let score = concat!(
        r#"{"model":"jev-latest","answers":{"q1":{"type":"score","#,
        r#""probabilities":{"0":1.0,"1":0.0,"2":0.0,"3":0.0}}}}"#,
    );
    assert_eq!(
        decode(&team_plan(), choice.as_bytes()),
        Err(DecodeError::UnexpectedProbability(0))
    );
    assert_eq!(
        decode(&disruption_plan(), score.as_bytes()),
        Err(DecodeError::UnexpectedProbability(0))
    );
}

#[test]
fn an_extra_label_refusal_is_complete_and_does_not_repeat_reply_values() {
    let sentinel = "sentinel-extra-label-4f8e";
    let body = format!(
        r#"{{"model":"jev-latest","answers":{{"q1":{{"type":"choice","probabilities":{{"billing":1.0,"shipping":0.0,"account":0.0,"other":0.0,"{sentinel}":0.25}}}}}}}}"#
    );
    let error = decode(&team_plan(), body.as_bytes()).expect_err("an extra label");
    assert_eq!(
        error.to_string(),
        "the answer to question `q1` has a probability for an option or level the question did not send"
    );
    assert!(!error.to_string().contains(sentinel));
    assert!(!error.to_string().contains("0.25"));
    assert!(!error.to_string().contains("1.0"));
}

#[test]
fn a_missing_key_precedes_an_extra_key() {
    let body = concat!(
        r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","#,
        r#""probabilities":{"shipping":0.0,"account":0.0,"other":0.0,"extra":1.0}}}}"#,
    );
    assert_eq!(
        decode(&team_plan(), body.as_bytes()),
        Err(DecodeError::MissingProbability(0))
    );
}
