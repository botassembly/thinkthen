//! Reading one response body as the answers the plan asked for.

use std::collections::BTreeMap;
use std::marker::PhantomData;

use serde::Deserialize;
use serde::de::{Deserializer, Error, MapAccess, Visitor};

use crate::core::adapters::systemone::{DecodeError, wire_name, wire_place};
use crate::core::answer::{Answer, Distribution, DistributionError};
use crate::core::plan::Plan;
use crate::core::probability::Probability;
use crate::core::question::{Labels, Question};
use crate::core::reply::{AnswerOutcome, BackendFailure, BackendFailureCause, Reply};
use crate::core::result::Usage;
use crate::core::text::ModelName;

/// Decimal rounding observed in System One probability distributions.
const DISTRIBUTION_ROUNDING: f64 = 0.01;

/// The body one response carries. Its keys may be labels a record gave, so it
/// and [`ResponseAnswer`] derive `Debug` only in tests.
#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug))]
struct Response {
    model: String,
    #[serde(deserialize_with = "unique")]
    answers: BTreeMap<String, ResponseAnswer>,
    #[serde(default)]
    usage: Option<ResponseUsage>,
}

/// One named answer inside a response.
///
/// The vendor also sends `choice`, `score`, and `legend`. Each one is derived
/// from the distribution and the question that was asked, so the adapter
/// computes them rather than reading them, and one answer keeps one arithmetic.
#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug))]
#[serde(tag = "type", rename_all = "snake_case")]
enum ResponseAnswer {
    /// The answer to a yes/no question, as one probability.
    Noul {
        #[serde(default)]
        noul: Option<f64>,
    },
    /// The answer to a pick, keyed by option name.
    Choice {
        #[serde(default, deserialize_with = "unique_some")]
        probabilities: Option<Wire>,
        #[serde(default)]
        confidence: Option<f64>,
    },
    /// The answer to a placement, keyed by the level's number as a string.
    Score {
        #[serde(default, deserialize_with = "unique_some")]
        probabilities: Option<Wire>,
        #[serde(default)]
        confidence: Option<f64>,
    },
    /// An answer of some other shape, which this version does not read.
    #[serde(other)]
    Other,
}

/// One distribution as the wire keys it, by option name or by level number.
type Wire = BTreeMap<String, f64>;

/// Read one JSON object into a map, refusing a member name it already holds.
/// The refusal names no member, because a name can be the user's own label.
fn unique<'de, D: Deserializer<'de>, V: Deserialize<'de>>(
    deserializer: D,
) -> Result<BTreeMap<String, V>, D::Error> {
    deserializer.deserialize_map(Unique(PhantomData))
}

/// The reader behind [`unique`].
struct Unique<V>(PhantomData<V>);

impl<'de, V: Deserialize<'de>> Visitor<'de> for Unique<V> {
    type Value = BTreeMap<String, V>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an object")
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut read = BTreeMap::new();
        while let Some((name, value)) = map.next_entry::<String, V>()? {
            if read.insert(name, value).is_some() {
                return Err(A::Error::custom("a member name repeats"));
            }
        }
        Ok(read)
    }
}

/// Read an optional distribution, where `null` reads as absent.
fn unique_some<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Wire>, D::Error> {
    #[derive(Deserialize)]
    struct Present(#[serde(deserialize_with = "unique")] Wire);
    Ok(Option::<Present>::deserialize(deserializer)?.map(|Present(map)| map))
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
/// names an answer or a label twice, when it answers a place the plan never
/// asked, or when every planned question fails. A missing answer, a wrong
/// shape, a missing probability, a probability outside zero to one, or an
/// incomplete distribution fails only that question.
mod observed;

pub(crate) use observed::decode_observed;

pub(crate) fn decode(plan: &Plan, body: &[u8]) -> Result<Reply, DecodeError> {
    decode_observed(plan, body).reply
}

fn decode_response(
    plan: &Plan,
    response: Response,
    usage: Option<Usage>,
) -> Result<Reply, DecodeError> {
    let model = ModelName::reported(response.model).map_err(|_| DecodeError::NoModel)?;
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
        .any(|name| wire_place(name).is_none_or(|place| place >= wire_count))
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
                probabilities: wire,
                confidence,
            },
        ) => Answer::new_choice(
            spread(options, options.names().cloned(), wire.as_ref(), place)?,
            reported(*confidence, place)?,
        )
        .ok_or(DecodeError::MissingProbability(place)),
        (
            Question::Score { levels, .. },
            ResponseAnswer::Score {
                probabilities: wire,
                confidence,
            },
        ) => Answer::new_score(
            spread(levels, numbered(levels), wire.as_ref(), place)?,
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
    wire: Option<&Wire>,
    place: usize,
) -> Result<Distribution, DecodeError> {
    let wire = wire.ok_or(DecodeError::MissingProbability(place))?;
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
        entries.push((label.clone(), probability(Some(*value), place)?));
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

/// Take one number from the wire as a probability, failing its question when
/// the answer left the number out.
fn probability(value: Option<f64>, place: usize) -> Result<Probability, DecodeError> {
    let value = value.ok_or(DecodeError::MissingProbability(place))?;
    Probability::new(value).map_err(|_| DecodeError::ProbabilityOutOfRange(place))
}

/// Take the backend's own confidence, when the backend reported one.
fn reported(value: Option<f64>, place: usize) -> Result<Option<Probability>, DecodeError> {
    value
        .map(|value| probability(Some(value), place))
        .transpose()
}

#[cfg(test)]
#[path = "response_distribution_tests.rs"]
mod distribution_tests;

#[cfg(test)]
#[path = "response_partial_tests.rs"]
mod partial_tests;

#[cfg(test)]
#[path = "response_tests.rs"]
mod tests;
