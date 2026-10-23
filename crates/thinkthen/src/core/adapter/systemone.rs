//! The System One decisions API.

use std::collections::BTreeMap;

use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use crate::core::ask::{Ask, AskAnswer, ChoiceOption};
use crate::core::error::{Error, ErrorKind};
use crate::core::probability::Probability;

#[derive(Serialize)]
struct Request<'a> {
    state: &'a str,
    model: &'a str,
    questions: Questions<'a>,
}

struct Questions<'a>(&'a [&'a RawValue]);

impl Serialize for Questions<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (place, question) in self.0.iter().enumerate() {
            map.serialize_entry(&name(place), question)?;
        }
        map.end()
    }
}
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Question<'a> {
    Choice {
        instructions: &'a str,
        criteria: Options<'a>,
    },
    Score {
        instructions: &'a str,
        criteria: &'a [String],
    },
    Noul {
        instructions: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        criteria: Option<Criteria<'a>>,
    },
}

#[derive(Debug)]
struct Options<'a>(&'a [ChoiceOption]);

impl Serialize for Options<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for option in self.0 {
            map.serialize_entry(option.label(), &option.description_text())?;
        }
        map.end()
    }
}

#[derive(Serialize)]
struct Criteria<'a> {
    #[serde(rename = "true", skip_serializing_if = "Option::is_none")]
    yes: Option<&'a str>,
    #[serde(rename = "false", skip_serializing_if = "Option::is_none")]
    no: Option<&'a str>,
}

impl<'a> From<&'a Ask> for Question<'a> {
    fn from(ask: &'a Ask) -> Self {
        match ask {
            Ask::WithState { question, .. } => Self::from(question.as_ref()),
            Ask::Choice {
                instructions,
                options,
            } => Self::Choice {
                instructions,
                criteria: Options(options),
            },
            Ask::Scale {
                instructions,
                levels,
            } => Self::Score {
                instructions,
                criteria: levels,
            },
            Ask::YesNo {
                instructions,
                yes,
                no,
            } => Self::Noul {
                instructions,
                criteria: (yes.is_some() || no.is_some()).then_some(Criteria {
                    yes: yes.as_deref(),
                    no: no.as_deref(),
                }),
            },
        }
    }
}

/// A reply's answers by wire name.
#[derive(Debug, Deserialize)]
pub(crate) struct Reply {
    answers: BTreeMap<String, Box<RawValue>>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ReplyAnswer {
    Choice {
        probabilities: BTreeMap<String, f64>,
        #[serde(default)]
        choice: Option<String>,
    },
    Noul {
        #[serde(default)]
        noul: Option<f64>,
    },
    Score {
        probabilities: BTreeMap<String, f64>,
    },
    #[serde(other)]
    Other,
}

/// The wire name of the question at `place`: `q1`, `q2` and so on.
fn name(place: usize) -> String {
    format!("q{}", place + 1)
}

#[expect(
    clippy::expect_used,
    reason = "a question of strings always serializes"
)]
pub(super) fn question(ask: &Ask) -> Box<RawValue> {
    serde_json::value::to_raw_value(&Question::from(ask)).expect("a question serializes")
}

/// The bytes the question at `place` adds to a request body.
pub(super) fn entry_len(place: usize, question: &RawValue) -> usize {
    // `"qN":` and the question, after a comma unless it comes first.
    name(place).len() + 3 + question.get().len() + usize::from(place > 0)
}

#[expect(
    clippy::expect_used,
    reason = "a request of strings and raw questions always serializes"
)]
pub(super) fn encode(state: &str, model: &str, questions: &[&RawValue]) -> Vec<u8> {
    let request = Request {
        state,
        model,
        questions: Questions(questions),
    };
    serde_json::to_vec(&request).expect("a request serializes")
}

pub(super) fn decode(body: &[u8]) -> Result<Reply, Error> {
    serde_json::from_slice(body).map_err(|_| {
        Error::new(
            ErrorKind::Reply,
            "the backend's reply is not a System One reply",
        )
    })
}

impl Reply {
    pub(super) fn read(&self, place: usize, ask: &Ask) -> Result<AskAnswer, Error> {
        let refused =
            |problem: &str| Error::new(ErrorKind::Reply, format!("the backend {problem}"));
        let answer = self
            .answers
            .get(&name(place))
            .ok_or_else(|| refused("gave no answer to the question"))?;
        let answer: ReplyAnswer = serde_json::from_str(answer.get())
            .map_err(|_| refused("gave a malformed answer to the question"))?;
        match (ask, answer) {
            (
                Ask::Choice { options, .. },
                ReplyAnswer::Choice {
                    probabilities,
                    choice,
                },
            ) => {
                if probabilities.len() != options.len()
                    || probabilities
                        .keys()
                        .any(|label| !options.iter().any(|option| option.label() == label))
                    || choice
                        .is_some_and(|label| !options.iter().any(|option| option.label() == label))
                {
                    return Err(refused("named an unknown option or omitted an option"));
                }
                options
                    .iter()
                    .map(|option| {
                        probabilities
                            .get(option.label())
                            .copied()
                            .and_then(Probability::new)
                            .ok_or_else(|| refused("gave no probability from 0 to 1"))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(AskAnswer::choice)
            }

            (Ask::YesNo { .. }, ReplyAnswer::Noul { noul }) => noul
                .and_then(Probability::new)
                .map(AskAnswer::yes_no)
                .ok_or_else(|| refused("gave no probability from 0 to 1")),
            (Ask::Scale { levels, .. }, ReplyAnswer::Score { probabilities }) => {
                if probabilities.len() != levels.len() {
                    return Err(refused("gave the wrong number of scale probabilities"));
                }
                (0..levels.len())
                    .map(|index| {
                        probabilities
                            .get(&index.to_string())
                            .copied()
                            .and_then(Probability::new)
                            .ok_or_else(|| refused("gave no probability from 0 to 1 for a level"))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .and_then(AskAnswer::scale)
            }
            _ => Err(refused("answered the question with the wrong kind")),
        }
    }
}

/// The System One error code for a request that needs splitting.
pub(super) fn request_too_large(body: &[u8]) -> bool {
    String::from_utf8_lossy(body).contains("max_tokens_exceeded")
}
