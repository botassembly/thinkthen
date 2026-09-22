//! Reading one response body as the answers the plan asked for.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::core::adapters::systemone::{DecodeError, wire_name};
use crate::core::answer::{Answer, Distribution, DistributionError};
use crate::core::plan::Plan;
use crate::core::probability::Probability;
use crate::core::question::{Labels, Question};
use crate::core::reply::{AnswerOutcome, BackendFailure, BackendFailureCause, Reply};
use crate::core::result::Usage;
use crate::core::text::ModelName;

/// Decimal rounding observed in System One probability distributions.
const DISTRIBUTION_ROUNDING: f64 = 0.01;

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
pub(crate) fn decode(plan: &Plan, body: &[u8]) -> Result<Reply, DecodeError> {
    let response: Response = serde_json::from_slice(body)
        .map_err(|error| DecodeError::Malformed(error.line(), error.column()))?;
    let model = ModelName::new(response.model).map_err(|_| DecodeError::NoModel)?;
    let wire_count = plan
        .questions()
        .iter()
        .map(|question| match question {
            Question::Tag { labels, .. } => labels.count(),
            _ => 1,
        })
        .sum::<usize>();
    if response
        .answers
        .keys()
        .any(|name| !(0..wire_count).any(|place| wire_name(place) == *name))
    {
        return Err(DecodeError::UnexpectedAnswer);
    }
    let mut answers = Vec::with_capacity(plan.questions().len());
    let mut first_error = None;
    let mut wire_place = 0;
    for question in plan.questions() {
        if let Question::Tag { labels, .. } = question {
            let decoded = read_tag(&response.answers, labels, &mut wire_place);
            remember(&decoded, &mut first_error);
            answers.push(outcome(decoded));
            continue;
        }
        let decoded = response
            .answers
            .get(&wire_name(wire_place))
            .ok_or(DecodeError::MissingAnswer(wire_place))
            .and_then(|answered| read(question, answered, wire_place));
        remember(&decoded, &mut first_error);
        answers.push(outcome(decoded));
        wire_place += 1;
    }
    if answers
        .iter()
        .all(|answer| matches!(answer, AnswerOutcome::Failed(_)))
    {
        return Err(first_error.unwrap_or(DecodeError::UnexpectedAnswer));
    }
    let usage = response
        .usage
        .map(|usage| Usage::new(usage.input_tokens, usage.output_tokens));
    Ok(Reply::new(model, answers, usage))
}

fn remember(result: &Result<Answer, DecodeError>, first: &mut Option<DecodeError>) {
    if first.is_none() {
        *first = result.as_ref().err().cloned();
    }
}

fn outcome(result: Result<Answer, DecodeError>) -> AnswerOutcome {
    match result {
        Ok(answer) => AnswerOutcome::Answered(answer),
        Err(error) => AnswerOutcome::Failed(BackendFailure::new(cause(&error))),
    }
}

const fn cause(error: &DecodeError) -> BackendFailureCause {
    match error {
        DecodeError::MissingAnswer(_) => BackendFailureCause::MissingAnswer,
        DecodeError::WrongKind(_) => BackendFailureCause::WrongKind,
        DecodeError::MissingProbability(_) => BackendFailureCause::MissingProbability,
        DecodeError::ProbabilityOutOfRange(_) => BackendFailureCause::InvalidProbability,
        DecodeError::DistributionTotal { .. } => BackendFailureCause::InvalidDistribution,
        DecodeError::UnexpectedProbability(_) => BackendFailureCause::UnexpectedProbability,
        DecodeError::Malformed(..) | DecodeError::NoModel | DecodeError::UnexpectedAnswer => {
            BackendFailureCause::WrongKind
        }
    }
}

/// Read the adjacent yes-or-no wire answers that make one tag answer.
fn read_tag(
    answered: &BTreeMap<String, ResponseAnswer>,
    labels: &Labels,
    wire_place: &mut usize,
) -> Result<Answer, DecodeError> {
    let mut probabilities = Vec::with_capacity(labels.count());
    let mut failed = None;
    for label in labels.names() {
        let place = *wire_place;
        *wire_place += 1;
        let decoded = answered
            .get(&wire_name(place))
            .ok_or(DecodeError::MissingAnswer(place))
            .and_then(|answer| match answer {
                ResponseAnswer::Noul { noul } => probability(*noul, place),
                _ => Err(DecodeError::WrongKind(place)),
            });
        match decoded {
            Ok(value) => probabilities.push((label.clone(), value)),
            Err(error) if failed.is_none() => failed = Some(error),
            Err(_) => {}
        }
    }
    if let Some(error) = failed {
        return Err(error);
    }
    Ok(Answer::new_tag(probabilities))
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
        (Question::Tag { .. }, _) => Err(DecodeError::WrongKind(place)),
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
    let tolerance = DISTRIBUTION_ROUNDING + entries.len() as f64 * f64::EPSILON;
    Distribution::with_tolerance(entries, tolerance).map_err(|error| match error {
        DistributionError::Total {
            total,
            members,
            tolerance: _,
        } => DecodeError::DistributionTotal {
            place,
            total: total.to_string(),
            members,
            tolerance: DISTRIBUTION_ROUNDING.to_string(),
        },
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
#[path = "response_distribution_tests.rs"]
mod distribution_tests;

#[cfg(test)]
#[path = "response_partial_tests.rs"]
mod partial_tests;

#[cfg(test)]
mod tests {
    use super::decode;
    use crate::core::adapters::systemone::DecodeError;
    use crate::core::adapters::systemone::tests::{
        LEVELS, TEAMS, disruption_plan, tag_plan, team_plan, urgency_plan,
    };
    use crate::core::probability::Probability;
    use crate::core::reply::AnswerOutcome;
    use crate::core::result::Usage;

    const RESPONSE: &str = include_str!(
        "../../../../../../specification/fixtures/systemone/decide-urgent.response.json"
    );
    const MISSING: &str = include_str!(
        "../../../../../../specification/fixtures/systemone/refused-missing-answer.response.json"
    );
    const WRONG_KIND: &str = include_str!(
        "../../../../../../specification/fixtures/systemone/refused-wrong-kind.response.json"
    );
    const OUT_OF_RANGE: &str = include_str!(
        "../../../../../../specification/fixtures/systemone/refused-probability-out-of-range.response.json"
    );
    const CHOOSE: &str = include_str!(
        "../../../../../../specification/fixtures/systemone/choose-team.response.json"
    );
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
