//! Typed text encoding and request-local correlation.
use crate::core::adapters::built_in::EncodeError;
use crate::core::{Json, Plan, Question};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum Encoded {
    Predicate {
        instructions: String,
    },
    Choice {
        instructions: String,
        choices: Vec<Choice>,
    },
    Score {
        instructions: String,
        levels: Vec<Level>,
    },
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Choice {
    pub(super) value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Level {
    pub(super) label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}
pub(super) fn render(value: &Json) -> Result<String, EncodeError> {
    match value {
        Json::String(text) => Ok(text.clone()),
        _ => written(value),
    }
}
fn written<T: Serialize + ?Sized>(value: &T) -> Result<String, EncodeError> {
    serde_json::to_string(value).map_err(|error| EncodeError::of(&error))
}
fn description(value: Option<&Json>) -> Result<Option<String>, EncodeError> {
    value
        .filter(|value| !matches!(value, Json::Null))
        .map(render)
        .transpose()
}
fn predicate(text: &Json, yes: Option<&Json>, no: Option<&Json>) -> Result<Encoded, EncodeError> {
    let mut instructions = render(text)?;
    for (prefix, meaning) in [("True means: ", yes), ("False means: ", no)] {
        if let Some(meaning) = description(meaning)? {
            instructions.push('\n');
            instructions.push_str(prefix);
            instructions.push_str(&meaning);
        }
    }
    Ok(Encoded::Predicate { instructions })
}
pub(crate) fn parts(plan: &Plan) -> Result<super::Parts, EncodeError> {
    if plan.images().is_some() {
        return Err(EncodeError::of(&"images are unsupported on this route"));
    }
    let mut questions = Vec::new();
    for question in plan.questions() {
        match question {
            Question::Decide { text, yes, no } => questions.push(written(&predicate(
                text.as_json(),
                yes.as_ref().map(|v| v.as_json()),
                no.as_ref().map(|v| v.as_json()),
            )?)?),
            Question::Choose { text, options } => questions.push(written(&Encoded::Choice {
                instructions: render(text.as_json())?,
                choices: options
                    .descriptions()
                    .map(|(name, value)| {
                        Ok(Choice {
                            value: name.clone(),
                            description: description(value.map(|v| v.as_json()))?,
                        })
                    })
                    .collect::<Result<_, EncodeError>>()?,
            })?),
            Question::Score { text, levels } => questions.push(written(&Encoded::Score {
                instructions: render(text.as_json())?,
                levels: levels
                    .descriptions()
                    .map(|(name, value)| {
                        Ok(Level {
                            label: name.clone(),
                            description: description(value.map(|v| v.as_json()))?,
                        })
                    })
                    .collect::<Result<_, EncodeError>>()?,
            })?),
            Question::Tag { text, labels } => questions.extend(tag(text, labels)?),
        }
    }
    Ok(super::Parts {
        state: written(&plan.evidence().as_json())?,
        model: written(plan.model().as_str())?,
        questions,
    })
}
/// State is already validated compact JSON; string evidence keeps its exact text.
pub(crate) fn input(state: &str) -> String {
    // Only a JSON string needs unquoting. Structured state is already compact text.
    let text = serde_json::from_str::<String>(state).unwrap_or_else(|_| state.to_owned());
    serde_json::to_string(&text).unwrap_or_else(|_| state.to_owned())
}
pub(crate) fn join<'a>(
    state: &str,
    model: &str,
    questions: impl Iterator<Item = &'a str>,
) -> Vec<u8> {
    let mut body = format!(
        "{{\"model\":{model},\"input\":{},\"questions\":[",
        input(state)
    );
    for (place, question) in questions.enumerate() {
        if place > 0 {
            body.push(',');
        }
        body.push_str(&format!("{{\"name\":\"q{}\",", place + 1));
        body.push_str(question.strip_prefix('{').unwrap_or(question));
    }
    body.push_str("]}");
    body.into_bytes()
}
pub(crate) fn canonical_question(text: &str) -> Option<(String, Question)> {
    Json::parse(text).ok()?;
    let encoded: Encoded = serde_json::from_str(text).ok()?;
    let wording = crate::core::QuestionText::new("stored").ok()?;
    let question = match &encoded {
        Encoded::Predicate { .. } => Question::Decide {
            text: wording,
            yes: None,
            no: None,
        },
        Encoded::Choice { choices, .. } => Question::Choose {
            text: wording,
            options: crate::core::Labels::options(
                choices.iter().map(|v| v.value.clone()).collect(),
            )
            .ok()?,
        },
        Encoded::Score { levels, .. } => Question::Score {
            text: wording,
            levels: crate::core::Labels::levels(
                levels.iter().map(|v| (v.label.clone(), None)).collect(),
            )
            .ok()?,
        },
    };
    Some((written(&encoded).ok()?, question))
}

fn tag(
    text: &crate::core::QuestionText,
    labels: &crate::core::Labels,
) -> Result<Vec<String>, EncodeError> {
    let described = labels.descriptions().collect::<Vec<_>>();
    let sentence = text.as_json().as_str().filter(|_| {
        described.iter().all(|(_, value)| {
            value.is_none_or(|v| matches!(v.as_json(), Json::String(_) | Json::Null))
        })
    });
    described
        .into_iter()
        .map(|(name, value)| {
            let instructions = match sentence {
                Some(sentence) => Json::String(format!(
                    "{sentence}\n\nDetermine whether the label {} applies to this item.",
                    written(name)?
                )),
                None => {
                    let mut members = vec![("label".to_owned(), Json::String(name.clone()))];
                    if let Some(value) =
                        value.filter(|value| !matches!(value.as_json(), Json::Null))
                    {
                        members.push(("description".to_owned(), value.as_json().clone()));
                    }
                    Json::Array(vec![text.as_json().clone(), Json::Object(members)])
                }
            };
            written(&predicate(&instructions, value.map(|v| v.as_json()), None)?)
        })
        .collect()
}
