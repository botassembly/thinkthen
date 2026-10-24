//! The OpenAI decisions wire format.
//!
//! Contract: https://developers.openai.com/api/reference/resources/decisions/methods/create
//! Questions and answers are ordered arrays. Each ask quotes its own evidence,
//! and `input` carries the request's shared state. Choice values retain their types.

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use crate::core::ask::{Ask, AskAnswer};
use crate::core::error::{Error, ErrorKind};
use crate::core::probability::Probability;

#[derive(Serialize)]
struct Request<'a> {
    input: &'a str,
    model: &'a str,
    questions: &'a [&'a RawValue],
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum Value {
    Text(String),
    Bool(bool),
}

#[derive(Serialize)]
struct Choice<'a> {
    value: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

#[derive(Serialize)]
struct Level<'a> {
    label: &'a str,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Question<'a> {
    Predicate {
        instructions: &'a str,
    },
    Choice {
        instructions: &'a str,
        choices: Vec<Choice<'a>>,
    },
    Score {
        instructions: &'a str,
        levels: Vec<Level<'a>>,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) struct Reply {
    answers: Vec<Box<RawValue>>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Answer {
    Predicate {
        probability: f64,
    },
    Choice {
        probabilities: Vec<ChoiceProbability>,
    },
    Score {
        probabilities: Vec<LevelProbability>,
    },
    #[serde(other)]
    Other,
}

#[derive(Deserialize)]
struct ChoiceProbability {
    value: Value,
    probability: f64,
}

#[derive(Deserialize)]
struct LevelProbability {
    label: String,
    probability: f64,
}

#[expect(
    clippy::expect_used,
    reason = "a question contains only strings and booleans"
)]
pub(super) fn question(ask: &Ask) -> Result<Box<RawValue>, Error> {
    let question = match ask {
        Ask::WithState {
            question: inner, ..
        } => return question(inner),
        Ask::YesNo {
            instructions,
            yes: None,
            no: None,
        } => Question::Predicate { instructions },
        Ask::YesNo {
            instructions,
            yes,
            no,
        } => Question::Choice {
            instructions,
            choices: [(true, yes.as_deref()), (false, no.as_deref())]
                .into_iter()
                .map(|(value, description)| Choice {
                    value: Value::Bool(value),
                    description,
                })
                .collect(),
        },
        Ask::Choice {
            instructions,
            options,
        } => Question::Choice {
            instructions,
            choices: options
                .iter()
                .map(|option| Choice {
                    value: Value::Text(option.label().to_owned()),
                    description: option.description_text(),
                })
                .collect(),
        },
        Ask::Scale {
            instructions,
            levels,
        } => Question::Score {
            instructions,
            levels: levels.iter().map(|label| Level { label }).collect(),
        },
    };
    Ok(serde_json::value::to_raw_value(&question).expect("a question serializes"))
}

pub(super) fn entry_len(place: usize, question: &RawValue) -> usize {
    question.get().len() + usize::from(place > 0)
}

#[expect(
    clippy::expect_used,
    reason = "a request contains strings and raw questions"
)]
pub(super) fn encode(state: &str, model: &str, questions: &[&RawValue]) -> Vec<u8> {
    serde_json::to_vec(&Request {
        input: state,
        model,
        questions,
    })
    .expect("a request serializes")
}

fn malformed() -> Error {
    Error::new(
        ErrorKind::Reply,
        "the backend gave a malformed OpenAI decisions answer",
    )
}

pub(super) fn decode(body: &[u8]) -> Result<Reply, Error> {
    serde_json::from_slice(body).map_err(|_| malformed())
}

fn probability(value: f64) -> Result<Probability, Error> {
    Probability::new(value).ok_or_else(malformed)
}

fn choices(
    values: Vec<Value>,
    probabilities: Vec<ChoiceProbability>,
) -> Result<Vec<Probability>, Error> {
    if values.len() != probabilities.len() {
        return Err(malformed());
    }
    values
        .into_iter()
        .map(|value| {
            let mut matching = probabilities.iter().filter(|item| item.value == value);
            let item = matching.next().ok_or_else(malformed)?;
            if matching.next().is_some() {
                return Err(malformed());
            }
            probability(item.probability)
        })
        .collect()
}

impl Reply {
    pub(super) fn read(&self, place: usize, ask: &Ask) -> Result<AskAnswer, Error> {
        let answer = self.answers.get(place).ok_or_else(malformed)?;
        let answer: Answer = serde_json::from_str(answer.get()).map_err(|_| malformed())?;
        match (ask, answer) {
            (
                Ask::YesNo {
                    yes: None,
                    no: None,
                    ..
                },
                Answer::Predicate { probability: p },
            ) => probability(p).map(AskAnswer::yes_no),
            (Ask::YesNo { yes, no, .. }, Answer::Choice { probabilities })
                if yes.is_some() || no.is_some() =>
            {
                let probabilities =
                    choices(vec![Value::Bool(true), Value::Bool(false)], probabilities)?;
                probabilities
                    .first()
                    .copied()
                    .map(AskAnswer::yes_no)
                    .ok_or_else(malformed)
            }
            (Ask::Choice { options, .. }, Answer::Choice { probabilities }) => choices(
                options
                    .iter()
                    .map(|option| Value::Text(option.label().to_owned()))
                    .collect(),
                probabilities,
            )
            .map(AskAnswer::choice),
            (Ask::Scale { levels, .. }, Answer::Score { probabilities }) => {
                if levels.len() != probabilities.len() {
                    return Err(malformed());
                }
                levels
                    .iter()
                    .map(|label| {
                        let mut matching = probabilities.iter().filter(|item| &item.label == label);
                        let item = matching.next().ok_or_else(malformed)?;
                        if matching.next().is_some() {
                            return Err(malformed());
                        }
                        probability(item.probability)
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .and_then(AskAnswer::scale)
            }
            _ => Err(malformed()),
        }
    }
}
