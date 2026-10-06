//! Writing one plan as the request body the backend reads.

#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
use serde::Deserialize;
use serde::{Serialize, Serializer};
use serde_json::value::RawValue;

use crate::core::adapters::systemone::{EncodeError, wire_name};
use crate::core::json::Json;
use crate::core::plan::{Descriptions, Plan};
use crate::core::question::Question;
use crate::core::text::QuestionText;

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
    fn described(yes: Option<&Json>, no: Option<&Json>, form: Descriptions) -> Option<Self> {
        let described =
            |value: Option<&Json>| value.filter(|held| !matches!(held, Json::Null)).cloned();
        let mut yes = described(yes);
        let mut no = described(no);
        if form == Descriptions::BothSides && (yes.is_some() || no.is_some()) {
            yes.get_or_insert_with(|| Json::Object(Vec::new()));
            no.get_or_insert_with(|| Json::Object(Vec::new()));
        }
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
    if let Some(images) = plan.images() {
        let state = crate::core::pack::State::images(
            images,
            plan.image_route(),
            plan.model().as_str(),
            plan.image_profile(),
        )?;
        let body = state.body(&parts.model, parts.questions.iter().map(String::as_str));
        if state.body_limit().is_some_and(|limit| body.len() > limit) {
            return Err(EncodeError::of(&"image route body limit exceeded"));
        }
        return Ok(body);
    }
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
    let form = plan.descriptions();
    let mut written = Vec::new();
    for question in plan.questions() {
        match question {
            Question::Tag { text, labels } => {
                let described = labels
                    .descriptions()
                    .map(|(label, held)| Ok((label, sent(form, held.map(|held| held.as_json()))?)))
                    .collect::<Result<Vec<_>, EncodeError>>()?;
                // A string question over only string descriptions keeps the
                // sentence every older request carried. One structured value
                // turns every label into the array form instead, so no JSON is
                // ever interpolated into a sentence.
                let sentence = tag_sentence(text, &described);
                for (label, description) in &described {
                    let instructions =
                        tag_instructions(text, label, description.as_ref(), sentence)?;
                    written.push((
                        wire_name(written.len()),
                        RequestQuestion::Noul {
                            instructions,
                            criteria: NoulCriteria::described(description.as_ref(), None, form),
                        },
                    ));
                }
            }
            _ => {
                let Some(question) = RequestQuestion::asking(question, form)? else {
                    return Err(EncodeError::of(&"a tag question was not expanded"));
                };
                written.push((wire_name(written.len()), question));
            }
        }
    }
    Ok(Questions(written))
}

/// One description as the plan's form sends it, or `None` for no description
/// (ADR 0115 section 3). `Authored` sends each one exactly as written.
///
/// `Text` is a temporary workaround for an Ollama bug, owned by the debt issue
/// `sdlc/issues/2026-09-30-systemone-adapter-sends-criteria-objects-ollama-refuses.md`.
/// It sends a string as written, an object's nonblank string `what` alone, any
/// other object or list as its compact JSON text, and no empty object or null.
fn sent(form: Descriptions, held: Option<&Json>) -> Result<Option<Json>, EncodeError> {
    let Some(held) = held else {
        return Ok(None);
    };
    if form != Descriptions::Text {
        return Ok(Some(held.clone()));
    }
    Ok(match held {
        Json::Null => None,
        Json::Object(members) if members.is_empty() => None,
        Json::String(_) => Some(held.clone()),
        Json::Object(members) => {
            let what = members
                .iter()
                .find(|(name, _)| name == "what")
                .and_then(|(_, what)| what.as_str())
                .filter(|what| !what.trim().is_empty());
            Some(Json::String(match what {
                Some(what) => what.to_owned(),
                None => written(held)?,
            }))
        }
        _ => Some(Json::String(written(held)?)),
    })
}

/// Whether sending this plan turns a description object or list into text,
/// which `check` and `--plan` report (ADR 0115 section 4).
pub(crate) fn drops_detail(plan: &Plan) -> bool {
    plan.questions()
        .iter()
        .any(|question| drops_detail_of(plan.descriptions(), question))
}

/// Whether this form turns one of this question's descriptions into text.
pub(crate) fn drops_detail_of(form: Descriptions, question: &Question) -> bool {
    let held: Vec<&Json> = match question {
        Question::Decide { yes, no, .. } => [yes, no]
            .into_iter()
            .flatten()
            .map(crate::core::text::Meaning::as_json)
            .collect(),
        Question::Choose {
            options: labels, ..
        }
        | Question::Score { levels: labels, .. }
        | Question::Tag { labels, .. } => labels
            .descriptions()
            .filter_map(|(_, held)| held.map(crate::core::text::Description::as_json))
            .collect(),
    };
    drops_any(form, held)
}

/// Whether this form turns one of these descriptions, an object or a list,
/// into text. Only `Text` does, the Ollama workaround the debt issue
/// `sdlc/issues/2026-09-30-systemone-adapter-sends-criteria-objects-ollama-refuses.md` owns.
pub(crate) fn drops_any<'a>(form: Descriptions, held: impl IntoIterator<Item = &'a Json>) -> bool {
    form == Descriptions::Text
        && held.into_iter().any(|held| match held {
            Json::Object(members) => !members.is_empty(),
            Json::Array(_) => true,
            _ => false,
        })
}

/// The sentence a string-only tag question keeps, or none when the text or one
/// sent description is structured.
fn tag_sentence<'a>(
    text: &'a QuestionText,
    described: &[(&String, Option<Json>)],
) -> Option<&'a str> {
    text.as_json().as_str().filter(|_| {
        described
            .iter()
            .all(|(_, held)| held.as_ref().is_none_or(|held| held.as_str().is_some()))
    })
}

/// The instruction one tag label asks: the sentence a string-only question has
/// always sent, or the array form a structured value requires.
fn tag_instructions(
    text: &QuestionText,
    label: &str,
    description: Option<&Json>,
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
                members.push(("description".to_owned(), description.clone()));
            }
            Ok(Json::Array(vec![
                text.as_json().clone(),
                Json::Object(members),
            ]))
        }
    }
}

impl RequestQuestion {
    /// Write one question in the shape its verb asks for, each description in
    /// the plan's form.
    fn asking(question: &Question, form: Descriptions) -> Result<Option<Self>, EncodeError> {
        let meaning = |held: &Option<crate::core::text::Meaning>| {
            sent(form, held.as_ref().map(crate::core::text::Meaning::as_json))
        };
        Ok(Some(match question {
            Question::Decide { text, yes, no } => Self::Noul {
                instructions: text.as_json().clone(),
                criteria: NoulCriteria::described(
                    meaning(yes)?.as_ref(),
                    meaning(no)?.as_ref(),
                    form,
                ),
            },
            Question::Choose { text, options } => Self::Choice {
                instructions: text.as_json().clone(),
                criteria: Criteria(
                    options
                        .descriptions()
                        .map(|(name, held)| {
                            Ok((name.clone(), sent(form, held.map(|held| held.as_json()))?))
                        })
                        .collect::<Result<_, EncodeError>>()?,
                ),
            },
            Question::Score { text, levels } => Self::Score {
                instructions: text.as_json().clone(),
                criteria: levels
                    .descriptions()
                    .map(|(name, held)| {
                        Ok(match sent(form, held.map(|held| held.as_json()))? {
                            None => Json::String(name.clone()),
                            Some(Json::Null) => Json::Object(Vec::new()),
                            Some(held) => held,
                        })
                    })
                    .collect::<Result<_, EncodeError>>()?,
            },
            Question::Tag { .. } => return Ok(None),
        }))
    }
}

#[cfg(test)]
#[path = "request_tests.rs"]
mod tests;
