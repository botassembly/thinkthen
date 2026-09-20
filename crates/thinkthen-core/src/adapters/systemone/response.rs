//! Reading one response body as the answers the plan asked for.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::adapters::systemone::{DecodeError, wire_name};
use crate::answer::{Answer, Distribution, DistributionError};
use crate::plan::Plan;
use crate::probability::Probability;
use crate::question::{Labels, Question};
use crate::reply::Reply;
use crate::result::Usage;
use crate::text::ModelName;

/// The body one response carries.
#[derive(Debug, Deserialize)]
struct Response {
    model: String,
    answers: BTreeMap<String, ResponseAnswer>,
    #[serde(default)]
    usage: Option<ResponseUsage>,
}

/// One named answer inside a response.
///
/// The vendor also sends `choice`, `score`, and `legend`. Each one is derived
/// from the distribution and the question that was asked, so the adapter
/// computes them rather than reading them, and one answer keeps one arithmetic.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ResponseAnswer {
    /// The answer to a yes/no question, as one probability.
    Noul { noul: f64 },
    /// The answer to a pick, keyed by option name.
    Choice {
        probabilities: BTreeMap<String, f64>,
        #[serde(default)]
        confidence: Option<f64>,
    },
    /// The answer to a placement, keyed by the level's number as a string.
    Score {
        probabilities: BTreeMap<String, f64>,
        #[serde(default)]
        confidence: Option<f64>,
    },
    /// An answer of some other shape, which this version does not read.
    #[serde(other)]
    Other,
}

/// The token counts a response reports.
#[derive(Debug, Deserialize)]
struct ResponseUsage {
    input_tokens: u64,
    output_tokens: u64,
}

/// Read the response body as one answer per question the plan asked.
///
/// # Errors
///
/// Returns [`DecodeError`] when the body is not a `systemone` response, when it
/// answers a planned question with nothing, when an answer carries the wrong
/// shape, when it leaves an option or a level without a probability, or when a
/// probability falls outside zero to one, or when its probabilities do not
/// make a complete distribution.
pub fn decode(plan: &Plan, body: &[u8]) -> Result<Reply, DecodeError> {
    let response: Response = serde_json::from_slice(body)
        .map_err(|error| DecodeError::Malformed(error.line(), error.column()))?;
    let model = ModelName::new(response.model).map_err(|_| DecodeError::NoModel)?;
    let mut answers = Vec::with_capacity(plan.questions().len());
    for (place, question) in plan.questions().iter().enumerate() {
        let Some(answered) = response.answers.get(&wire_name(place)) else {
            return Err(DecodeError::MissingAnswer(place));
        };
        answers.push(read(question, answered, place)?);
    }
    let usage = response
        .usage
        .map(|usage| Usage::new(usage.input_tokens, usage.output_tokens));
    Ok(Reply::new(model, answers, usage))
}

/// Read one answer against the question that was asked in its place.
fn read(
    question: &Question,
    answered: &ResponseAnswer,
    place: usize,
) -> Result<Answer, DecodeError> {
    match (question, answered) {
        (Question::Decide { .. }, ResponseAnswer::Noul { noul }) => {
            Ok(Answer::new_yes_no(probability(*noul, place)?))
        }
        (
            Question::Choose { options, .. },
            ResponseAnswer::Choice {
                probabilities,
                confidence,
            },
        ) => Answer::new_choice(
            spread(options, options.names().cloned(), probabilities, place)?,
            reported(*confidence, place)?,
        )
        .ok_or(DecodeError::MissingProbability(place)),
        (
            Question::Score { levels, .. },
            ResponseAnswer::Score {
                probabilities,
                confidence,
            },
        ) => Answer::new_score(
            spread(levels, numbered(levels), probabilities, place)?,
            reported(*confidence, place)?,
        )
        .ok_or(DecodeError::MissingProbability(place)),
        _ => Err(DecodeError::WrongKind(place)),
    }
}

/// The wire keys a score answer uses: each level's number, as a string.
fn numbered(levels: &Labels) -> impl Iterator<Item = String> {
    (0..levels.count()).map(|level| level.to_string())
}

/// Read one probability per label, in the order the labels were sent.
fn spread(
    labels: &Labels,
    keys: impl Iterator<Item = String>,
    wire: &BTreeMap<String, f64>,
    place: usize,
) -> Result<Distribution, DecodeError> {
    let keys: Vec<String> = keys.collect();
    for key in &keys {
        if !wire.contains_key(key) {
            return Err(DecodeError::MissingProbability(place));
        }
    }
    let mut entries = Vec::with_capacity(labels.count());
    for (label, key) in labels.names().zip(&keys) {
        let value = wire
            .get(key)
            .ok_or(DecodeError::MissingProbability(place))?;
        entries.push((label.clone(), probability(*value, place)?));
    }
    if wire
        .keys()
        .any(|key| !keys.iter().any(|expected| expected == key))
    {
        return Err(DecodeError::UnexpectedProbability(place));
    }
    Distribution::new(entries).map_err(|error| match error {
        DistributionError::Total => DecodeError::DistributionTotal(place),
    })
}

/// Take one number from the wire as a probability.
fn probability(value: f64, place: usize) -> Result<Probability, DecodeError> {
    Probability::new(value).map_err(|_| DecodeError::ProbabilityOutOfRange(place))
}

/// Take the backend's own confidence, when the backend reported one.
fn reported(value: Option<f64>, place: usize) -> Result<Option<Probability>, DecodeError> {
    value.map(|value| probability(value, place)).transpose()
}

#[cfg(test)]
mod tests {
    use super::decode;
    use crate::adapters::systemone::DecodeError;
    use crate::adapters::systemone::tests::{
        LEVELS, TEAMS, disruption_plan, plan_for, team_plan, urgency_plan,
    };
    use crate::answer::Answer;
    use crate::probability::Probability;
    use crate::result::Usage;

    const RESPONSE: &str =
        include_str!("../../../../../specification/fixtures/systemone/decide-urgent.response.json");
    const MISSING: &str = include_str!(
        "../../../../../specification/fixtures/systemone/refused-missing-answer.response.json"
    );
    const WRONG_KIND: &str = include_str!(
        "../../../../../specification/fixtures/systemone/refused-wrong-kind.response.json"
    );
    const OUT_OF_RANGE: &str = include_str!(
        "../../../../../specification/fixtures/systemone/refused-probability-out-of-range.response.json"
    );
    const CHOOSE: &str =
        include_str!("../../../../../specification/fixtures/systemone/choose-team.response.json");
    const SCORE: &str = include_str!(
        "../../../../../specification/fixtures/systemone/score-disruption.response.json"
    );
    const MISSING_PROBABILITY: &str = include_str!(
        "../../../../../specification/fixtures/systemone/refused-missing-probability.response.json"
    );

    #[test]
    fn decode_reads_the_answer_the_fixture_shows() {
        let reply = decode(&urgency_plan(), RESPONSE.as_bytes()).expect("a systemone response");
        assert_eq!(reply.model().as_str(), "jev-latest");
        let [answer] = reply.answers() else {
            panic!("one answer per planned question");
        };
        assert_eq!(
            serde_json::to_string(answer).expect("an answer serializes"),
            r#"{"kind":"yes_no","probability":0.92}"#
        );
        assert_eq!(reply.usage(), Some(Usage::new(312, 48)));
    }

    #[test]
    fn a_choice_answer_is_read_back_in_the_order_the_options_were_sent() {
        let reply = decode(&team_plan(), CHOOSE.as_bytes()).expect("a systemone response");
        let [answer] = reply.answers() else {
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
        let [answer] = reply.answers() else {
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
        assert_eq!(
            error.to_string(),
            "the response is not a systemone response: the JSON at line 1 column 56 is not one"
        );
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
    fn the_key_order_of_an_answers_object_carries_nothing() {
        let plan = plan_for("Help!", &["is urgent", "asks for a refund"]);
        let body = concat!(
            r#"{"answers":{"q2":{"type":"noul","noul":0.25},"#,
            r#""q1":{"type":"noul","noul":0.75}},"model":"jev-latest"}"#
        );
        let reply = decode(&plan, body.as_bytes()).expect("a systemone response");
        let [first, second] = reply.answers() else {
            panic!("one answer per planned question");
        };
        let rendered = |answer: &Answer| serde_json::to_string(answer).expect("an answer");
        assert_eq!(rendered(first), r#"{"kind":"yes_no","probability":0.75}"#);
        assert_eq!(rendered(second), r#"{"kind":"yes_no","probability":0.25}"#);
    }

    #[test]
    fn a_response_decodes_without_usage_and_past_unknown_fields() {
        let body = concat!(
            r#"{"model":"jev-1.13.0","request_id":"abc","#,
            r#""answers":{"q1":{"type":"noul","noul":0.5,"rationale":"none"},"#,
            r#""q9":{"type":"noul","noul":0.1}}}"#,
        );
        let reply = decode(&urgency_plan(), body.as_bytes()).expect("a systemone response");
        assert_eq!(reply.model().as_str(), "jev-1.13.0");
        assert_eq!(reply.answers().len(), 1);
        assert_eq!(reply.usage(), None);
    }

    #[test]
    fn a_choice_and_score_refuse_a_total_outside_member_count_epsilon() {
        let choice = concat!(
            r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","#,
            r#""probabilities":{"billing":1.0,"shipping":1.0,"account":1.0,"other":1.0}}}}"#,
        );
        let score = concat!(
            r#"{"model":"jev-latest","answers":{"q1":{"type":"score","#,
            r#""probabilities":{"0":1.0,"1":1.0,"2":1.0}}}}"#,
        );
        let choice_below = concat!(
            r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","#,
            r#""probabilities":{"billing":0.0,"shipping":0.0,"account":0.0,"other":0.0}}}}"#,
        );
        let score_below = concat!(
            r#"{"model":"jev-latest","answers":{"q1":{"type":"score","#,
            r#""probabilities":{"0":0.0,"1":0.0,"2":0.0}}}}"#,
        );
        assert_eq!(
            decode(&team_plan(), choice.as_bytes()),
            Err(DecodeError::DistributionTotal(0))
        );
        assert_eq!(
            decode(&disruption_plan(), score.as_bytes()),
            Err(DecodeError::DistributionTotal(0))
        );
        assert_eq!(
            decode(&team_plan(), choice_below.as_bytes()),
            Err(DecodeError::DistributionTotal(0))
        );
        assert_eq!(
            decode(&disruption_plan(), score_below.as_bytes()),
            Err(DecodeError::DistributionTotal(0))
        );
    }

    #[test]
    fn a_distribution_total_error_names_the_rule_without_reply_values() {
        let body = concat!(
            r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","#,
            r#""probabilities":{"billing":0.2,"shipping":0.2,"account":0.2,"other":0.2}}}}"#,
        );
        let error = decode(&team_plan(), body.as_bytes()).expect_err("an invalid total");
        assert_eq!(
            error.to_string(),
            "the answer to question `q1` has probabilities whose total differs from one by more than member count × f64::EPSILON"
        );
        assert!(!error.to_string().contains("0.2"));
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
}
