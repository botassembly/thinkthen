//! Ordered live correlation, exact distributions and name-free saved answers.
use crate::core::adapters::built_in::{DecodeError, response::Decoded};
use crate::core::answer::{Distribution, DistributionError};
use crate::core::probability::Probability;
use crate::core::{Answer, Json, ModelName, Plan, Question, Reply, ReportedUsage};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::collections::BTreeSet;

#[derive(Deserialize)]
struct Response {
    model: String,
    answers: Vec<Box<RawValue>>,
    usage: Option<Tokens>,
}
#[derive(Deserialize)]
struct Tokens {
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
}
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Wire {
    Predicate {
        probability: f64,
    },
    Choice {
        choice: String,
        probabilities: Vec<ChoiceProbability>,
        confidence: f64,
    },
    Score {
        score: f64,
        probabilities: Vec<ScoreProbability>,
        confidence: f64,
    },
    Refusal {},
}
#[derive(Deserialize, Serialize)]
struct ChoiceProbability {
    value: String,
    probability: f64,
}
#[derive(Deserialize, Serialize)]
struct ScoreProbability {
    value: usize,
    label: String,
    probability: f64,
}

fn malformed() -> DecodeError {
    DecodeError::OtherMalformed
}
fn parse(body: &[u8]) -> Result<Response, DecodeError> {
    let text = std::str::from_utf8(body).map_err(|_| malformed())?;
    Json::parse(text).map_err(|_| malformed())?;
    serde_json::from_slice(body).map_err(|_| malformed())
}
fn probability(value: f64, place: usize) -> Result<Probability, DecodeError> {
    Probability::new(value).map_err(|_| DecodeError::ProbabilityOutOfRange(place))
}
fn distribution(
    entries: Vec<(String, Probability)>,
    place: usize,
) -> Result<Distribution, DecodeError> {
    Distribution::new(entries).map_err(|error| match error {
        DistributionError::Total {
            total,
            members,
            tolerance,
        } => DecodeError::DistributionTotal {
            place,
            total: total.to_string(),
            members,
            tolerance: tolerance.to_string(),
        },
    })
}
fn decode(question: &Question, wire: &Wire, place: usize) -> Result<Answer, DecodeError> {
    match (question, wire) {
        (Question::Decide { .. }, Wire::Predicate { probability: value }) => {
            Ok(Answer::new_yes_no(probability(*value, place)?))
        }
        (
            Question::Choose { options, .. },
            Wire::Choice {
                choice,
                probabilities,
                confidence,
            },
        ) => {
            if !options.names().any(|name| name == choice) {
                return Err(DecodeError::UnexpectedProbability(place));
            }
            let expected: Vec<_> = options.names().collect();
            let mut seen = BTreeSet::new();
            for member in probabilities {
                if !seen.insert(&member.value) || !expected.contains(&&member.value) {
                    return Err(DecodeError::UnexpectedProbability(place));
                }
            }
            let mut entries = Vec::new();
            for label in expected {
                let member = probabilities
                    .iter()
                    .find(|member| &member.value == label)
                    .ok_or(DecodeError::MissingProbability(place))?;
                entries.push((label.clone(), probability(member.probability, place)?));
            }
            Answer::new_choice(
                distribution(entries, place)?,
                Some(probability(*confidence, place)?),
            )
            .ok_or(DecodeError::MissingProbability(place))
        }
        (
            Question::Score { levels, .. },
            Wire::Score {
                score,
                probabilities,
                confidence,
            },
        ) => {
            if !score.is_finite() {
                return Err(DecodeError::WrongKind(place));
            }
            let labels = levels.names().collect::<Vec<_>>();
            let mut seen = BTreeSet::new();
            for member in probabilities {
                if !seen.insert(member.value)
                    || labels
                        .get(member.value)
                        .is_none_or(|label| **label != member.label)
                {
                    return Err(DecodeError::UnexpectedProbability(place));
                }
            }
            let mut entries = Vec::new();
            for (index, label) in labels.iter().enumerate() {
                let member = probabilities
                    .iter()
                    .find(|member| member.value == index)
                    .ok_or(DecodeError::MissingProbability(place))?;
                entries.push(((*label).clone(), probability(member.probability, place)?));
            }
            Answer::new_score(
                distribution(entries, place)?,
                Some(probability(*confidence, place)?),
            )
            .ok_or(DecodeError::MissingProbability(place))
        }
        (_, Wire::Refusal { .. }) => Err(DecodeError::Refused(place)),
        _ => Err(DecodeError::WrongKind(place)),
    }
}
pub(crate) fn stored(question: &Question, text: &str) -> Result<Answer, DecodeError> {
    let json = Json::parse(text).map_err(|_| malformed())?;
    if json.member("name").is_some() {
        return Err(malformed());
    }
    let wire: Wire = serde_json::from_str(text).map_err(|_| DecodeError::WrongKind(0))?;
    decode(question, &wire, 0)
}
pub(crate) fn split(
    questions: &[Question],
    body: &[u8],
) -> Result<crate::core::pack::Split, (DecodeError, Option<ReportedUsage>)> {
    let response = parse(body).map_err(|error| (error, None))?;
    let usage = response
        .usage
        .map(|v| ReportedUsage::new(v.input_tokens, v.output_tokens));
    let model = ModelName::reported(response.model).map_err(|_| (DecodeError::NoModel, usage))?;
    if response.answers.len() > questions.len() {
        return Err((DecodeError::UnexpectedAnswer, usage));
    }
    let mut seen = BTreeSet::new();
    let mut answers = Vec::new();
    for (place, question) in questions.iter().enumerate() {
        let Some(raw) = response.answers.get(place) else {
            answers.push(Err(DecodeError::MissingAnswer(place)));
            continue;
        };
        let json = Json::parse(raw.get()).map_err(|_| (malformed(), usage))?;
        let name = json.member("name").and_then(Json::as_str);
        if let Some(name) = name
            && !seen.insert(name.to_owned())
        {
            return Err((DecodeError::UnexpectedAnswer, usage));
        }
        if name != Some(&format!("q{}", place + 1)) {
            answers.push(Err(DecodeError::WrongKind(place)));
            continue;
        }
        let answer = serde_json::from_str::<Wire>(raw.get())
            .map_err(|_| DecodeError::WrongKind(place))
            .and_then(|wire| {
                decode(question, &wire, place)?;
                serde_json::to_string(&wire).map_err(|_| malformed())
            });
        answers.push(answer);
    }
    if answers.iter().all(Result::is_err) {
        return Err((
            answers
                .first()
                .and_then(|v| v.as_ref().err())
                .cloned()
                .unwrap_or(DecodeError::UnexpectedAnswer),
            usage,
        ));
    }
    Ok(crate::core::pack::Split {
        model,
        usage,
        answers,
    })
}
pub(crate) fn decode_observed(plan: &Plan, body: &[u8]) -> Decoded {
    let decoders = plan
        .questions()
        .iter()
        .flat_map(crate::core::pack::decoders)
        .collect::<Vec<_>>();
    match split(&decoders, body) {
        Err((error, usage)) => Decoded {
            usage,
            reply: Err(error),
        },
        Ok(split) => {
            let stored = split
                .answers
                .iter()
                .map(|v| v.as_deref().map_err(DecodeError::cause))
                .collect::<Vec<_>>();
            let reply = crate::core::pack::read_for(
                super::super::ApiType::Decisions,
                plan.questions(),
                &stored,
                split.model.as_str(),
            )
            .map(|outcomes| {
                Reply::new(split.model, outcomes, None).with_reported_usage(split.usage)
            });
            Decoded {
                usage: split.usage,
                reply,
            }
        }
    }
}
/// Raw exchanges are correlated against their own original sent question order.
pub(crate) fn validate_exchange(request: &[u8], response: &[u8]) -> Result<(), DecodeError> {
    #[derive(Deserialize)]
    struct Request {
        questions: Vec<Box<RawValue>>,
    }
    let request: Request = serde_json::from_slice(request).map_err(|_| malformed())?;
    let mut questions = Vec::new();
    for (place, raw) in request.questions.into_iter().enumerate() {
        let Json::Object(mut members) = Json::parse(raw.get()).map_err(|_| malformed())? else {
            return Err(malformed());
        };
        if members
            .iter()
            .find(|(key, _)| key == "name")
            .and_then(|(_, value)| value.as_str())
            != Some(&format!("q{}", place + 1))
        {
            return Err(malformed());
        }
        members.retain(|(key, _)| key != "name");
        let text = serde_json::to_string(&Json::Object(members)).map_err(|_| malformed())?;
        questions.push(super::canonical_question(&text).ok_or_else(malformed)?.1);
    }
    // Partial replies remain valid recordings; every retained success is correlated.
    split(&questions, response)
        .map(|_| ())
        .map_err(|(error, _)| error)
}

/// A valid envelope reports an actual model even when all its logical answers fail.
pub(crate) fn reported_model(body: &[u8]) -> Option<ModelName> {
    ModelName::reported(parse(body).ok()?.model).ok()
}
