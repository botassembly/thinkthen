//! Writing one plan as the request body the backend reads.

#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
use serde::Deserialize;
use serde::{Serialize, Serializer};
use serde_json::value::RawValue;

use crate::core::adapters::systemone::{EncodeError, wire_name};
use crate::core::json::Json;
use crate::core::plan::Plan;
use crate::core::question::{Labels, Question};
use crate::core::text::{Description, QuestionText};

/// The body one request carries, as the tests read it back. [`join`] writes
/// it: `state` is a string for text evidence and the object or list itself
/// when a pointer selection made one.
#[cfg(test)]
#[derive(Debug, serde::Deserialize, PartialEq)]
pub(crate) struct Request {
    state: Json,
    model: String,
    questions: Questions,
}

/// The wire questions in request order.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct Questions(Vec<(String, RequestQuestion)>);

#[cfg(test)]
impl Questions {
    fn get(&self, name: &str) -> Option<&RequestQuestion> {
        self.0
            .iter()
            .find(|(held, _)| held == name)
            .map(|(_, question)| question)
    }
}

#[cfg(test)]
impl<'de> Deserialize<'de> for Questions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        BTreeMap::<String, RequestQuestion>::deserialize(deserializer)
            .map(|questions| Self(questions.into_iter().collect()))
    }
}

/// One named question inside a request, in the shape its verb asks for.
#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, serde::Deserialize, PartialEq))]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum RequestQuestion {
    /// A yes/no question, which the vendor calls `noul`.
    Noul {
        instructions: Json,
        #[serde(skip_serializing_if = "Option::is_none")]
        criteria: Option<NoulCriteria>,
    },
    /// A pick, with the options as the keys of `criteria`.
    Choice {
        instructions: Json,
        criteria: Criteria,
    },
    /// A placement, with the levels as the `criteria` array, lowest first.
    Score {
        instructions: Json,
        criteria: Vec<Json>,
    },
}

/// What a yes means and what a no means, as the vendor's `criteria` object.
///
/// The tool's own words for these two are `--true` and `--false`, and this
/// module alone knows they travel here. An absent or null description has no
/// wire member; a question with neither carries no `criteria` at all.
#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, serde::Deserialize, PartialEq))]
pub(crate) struct NoulCriteria {
    #[serde(rename = "true", skip_serializing_if = "Option::is_none")]
    yes: Option<Json>,
    #[serde(rename = "false", skip_serializing_if = "Option::is_none")]
    no: Option<Json>,
}

impl NoulCriteria {
    fn described(yes: Option<&Json>, no: Option<&Json>) -> Option<Self> {
        let described =
            |value: Option<&Json>| value.filter(|held| !matches!(held, Json::Null)).cloned();
        let yes = described(yes);
        let no = described(no);
        (yes.is_some() || no.is_some()).then_some(Self { yes, no })
    }
}

/// The options of a pick, as a map from each option to its description.
///
/// The command line carries labels alone, so every description is `null`
/// there. `--options` reads a map from a record, and each value travels as the
/// description. The keys keep the order they were given, because option order
/// moves the odds.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct Criteria(Vec<(String, Option<Json>)>);

impl Serialize for Criteria {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|(option, described)| (option, described)))
    }
}

/// The visitor that reads a `criteria` map back, keeping its document order.
#[cfg(test)]
struct Keys;

#[cfg(test)]
impl<'de> serde::de::Visitor<'de> for Keys {
    type Value = Criteria;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a map from each option to its description")
    }

    fn visit_map<M: serde::de::MapAccess<'de>>(self, mut map: M) -> Result<Criteria, M::Error> {
        let mut options = Vec::new();
        while let Some((option, described)) = map.next_entry::<String, Option<Json>>()? {
            options.push((option, described));
        }
        Ok(Criteria(options))
    }
}

#[cfg(test)]
impl<'de> serde::Deserialize<'de> for Criteria {
    /// Read the keys back in document order, so a fixture pins that order.
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Keys)
    }
}

/// Write the plan as the request body the backend reads.
///
/// # Errors
///
/// Returns [`EncodeError`] when the body cannot be written as JSON.
pub(crate) fn encode(plan: &Plan) -> Result<Vec<u8>, EncodeError> {
    let parts = parts(plan)?;
    Ok(join(
        &parts.state,
        &parts.model,
        parts.questions.iter().map(String::as_str),
    ))
}

/// Write the plan as the request body the plan document embeds.
pub(crate) fn encode_raw(plan: &Plan) -> Result<Box<RawValue>, EncodeError> {
    let body = String::from_utf8(encode(plan)?).map_err(|error| EncodeError::of(&error))?;
    RawValue::from_string(body).map_err(|error| EncodeError::of(&error))
}

/// One plan's state, model and wire questions, each as the compact JSON the
/// body carries, by ADR 0111 section 2. Each part is written once.
pub(crate) struct Parts {
    pub(crate) state: String,
    pub(crate) model: String,
    pub(crate) questions: Vec<String>,
}

/// Write each part of the plan once.
pub(crate) fn parts(plan: &Plan) -> Result<Parts, EncodeError> {
    let state = written(&plan.evidence().as_json())?;
    let model = written(plan.model().as_str())?;
    let questions = questions(plan)?
        .0
        .iter()
        .map(|(_, question)| written(question))
        .collect::<Result<_, _>>()?;
    Ok(Parts {
        state,
        model,
        questions,
    })
}

/// The body those same bytes join into: `{"state":S,"model":M,"questions":{"q1":Q1,…}}`.
pub(crate) fn join<'a>(
    state: &str,
    model: &str,
    questions: impl Iterator<Item = &'a str>,
) -> Vec<u8> {
    let mut body = Vec::with_capacity(state.len() + model.len() + 64);
    body.extend_from_slice(b"{\"state\":");
    body.extend_from_slice(state.as_bytes());
    body.extend_from_slice(b",\"model\":");
    body.extend_from_slice(model.as_bytes());
    body.extend_from_slice(b",\"questions\":{");
    for (place, question) in questions.enumerate() {
        if place > 0 {
            body.push(b',');
        }
        body.extend_from_slice(b"\"");
        body.extend_from_slice(wire_name(place).as_bytes());
        body.extend_from_slice(b"\":");
        body.extend_from_slice(question.as_bytes());
    }
    body.extend_from_slice(b"}}");
    body
}

/// Compact JSON for one part.
fn written<T: Serialize + ?Sized>(value: &T) -> Result<String, EncodeError> {
    serde_json::to_string(value).map_err(|error| EncodeError::of(&error))
}

/// Expand logical tag questions into one wire yes/no question per label.
fn questions(plan: &Plan) -> Result<Questions, EncodeError> {
    let mut written = Vec::new();
    for question in plan.questions() {
        match question {
            Question::Tag { text, labels } => {
                // A string question over only string descriptions keeps the
                // sentence every older request carried. One structured value
                // turns every label into the array form instead, so no JSON is
                // ever interpolated into a sentence.
                let sentence = tag_sentence(text, labels);
                for (label, description) in labels.descriptions() {
                    let instructions = tag_instructions(text, label, description, sentence)?;
                    written.push((
                        wire_name(written.len()),
                        RequestQuestion::Noul {
                            instructions,
                            criteria: NoulCriteria::described(
                                description.map(Description::as_json),
                                None,
                            ),
                        },
                    ));
                }
            }
            _ => {
                let Some(question) = RequestQuestion::asking(question) else {
                    return Err(EncodeError::of(&"a tag question was not expanded"));
                };
                written.push((wire_name(written.len()), question));
            }
        }
    }
    Ok(Questions(written))
}

/// The sentence a string-only tag question keeps, or none when the text or one
/// description is structured.
fn tag_sentence<'a>(text: &'a QuestionText, labels: &Labels) -> Option<&'a str> {
    text.as_json().as_str().filter(|_| {
        labels
            .descriptions()
            .all(|(_, described)| described.is_none_or(|held| held.as_json().as_str().is_some()))
    })
}

/// The instruction one tag label asks: the sentence a string-only question has
/// always sent, or the array form a structured value requires.
fn tag_instructions(
    text: &QuestionText,
    label: &str,
    description: Option<&Description>,
    sentence: Option<&str>,
) -> Result<Json, EncodeError> {
    match sentence {
        Some(sentence) => {
            let label = serde_json::to_string(label).map_err(|error| EncodeError::of(&error))?;
            Ok(Json::String(format!(
                "{sentence}\n\nDetermine whether the label {label} applies to this item."
            )))
        }
        None => {
            let mut members = vec![("label".to_owned(), Json::String(label.to_owned()))];
            if let Some(description) = description {
                members.push(("description".to_owned(), description.as_json().clone()));
            }
            Ok(Json::Array(vec![
                text.as_json().clone(),
                Json::Object(members),
            ]))
        }
    }
}

impl RequestQuestion {
    /// Write one question in the shape its verb asks for.
    fn asking(question: &Question) -> Option<Self> {
        Some(match question {
            Question::Decide { text, yes, no } => Self::Noul {
                instructions: text.as_json().clone(),
                criteria: NoulCriteria::described(
                    yes.as_ref().map(crate::core::text::Meaning::as_json),
                    no.as_ref().map(crate::core::text::Meaning::as_json),
                ),
            },
            Question::Choose { text, options } => Self::Choice {
                instructions: text.as_json().clone(),
                criteria: Criteria(
                    options
                        .descriptions()
                        .map(|(name, described)| {
                            (name.clone(), described.map(|held| held.as_json().clone()))
                        })
                        .collect(),
                ),
            },
            Question::Score { text, levels } => Self::Score {
                instructions: text.as_json().clone(),
                criteria: levels
                    .descriptions()
                    .map(|(name, described)| {
                        described.map_or_else(
                            || Json::String(name.clone()),
                            |held| match held.as_json() {
                                Json::Null => Json::Object(Vec::new()),
                                held => held.clone(),
                            },
                        )
                    })
                    .collect(),
            },
            Question::Tag { .. } => return None,
        })
    }
}

#[cfg(test)]
#[path = "request_tests.rs"]
mod tests;
