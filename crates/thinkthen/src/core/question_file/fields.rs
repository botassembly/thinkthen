//! The value each key of a question file holds, or the refusal naming it.

use crate::core::json::Json;
use crate::core::pointer::Pointer;
use crate::core::question_file::{Described, QuestionFileError, Source, Verb};
use crate::core::text::{BlankTextError, Description, Meaning, ModelName, QuestionText};
use crate::core::threshold::{Threshold, ThresholdError};

/// The text under one key, or the refusal that names the key.
pub(super) fn text_at<'a>(
    value: &'a Json,
    key: &'static str,
) -> Result<&'a str, QuestionFileError> {
    value
        .member(key)
        .and_then(Json::as_str)
        .ok_or(QuestionFileError::Shape {
            key,
            wanted: "is text",
        })
}

/// The question text under the verb's key, or the refusal that names it.
pub(super) fn question_text(
    value: &Json,
    key: &'static str,
) -> Result<QuestionText, QuestionFileError> {
    match value.member(key) {
        Some(Json::String(held)) => {
            QuestionText::new(held.clone()).map_err(|error| QuestionFileError::Blank {
                origin: Source::File,
                key,
                error,
            })
        }
        Some(held) => QuestionText::structured(held).ok_or(QuestionFileError::Shape {
            key,
            wanted: "is text, an object, or a list",
        }),
        None => Err(QuestionFileError::Shape {
            key,
            wanted: "is text, an object, or a list",
        }),
    }
}

/// The text that says what yes or what no means, when the file holds one.
pub(super) fn meaning(
    value: &Json,
    key: &'static str,
) -> Result<Option<Meaning>, QuestionFileError> {
    match value.member(key) {
        None => Ok(None),
        Some(Json::String(held)) => {
            Meaning::new(held.clone())
                .map(Some)
                .map_err(|error| QuestionFileError::Blank {
                    origin: Source::File,
                    key,
                    error,
                })
        }
        Some(held) => Meaning::structured(held)
            .map(Some)
            .ok_or(QuestionFileError::Shape {
                key,
                wanted: "is text, an object, a list, or null",
            }),
    }
}

/// The model name the file names, when it names one.
pub(super) fn model_in(value: &Json) -> Result<Option<ModelName>, QuestionFileError> {
    if value.member("model").is_none() {
        return Ok(None);
    }
    ModelName::new(text_at(value, "model")?)
        .map(Some)
        .map_err(|error| QuestionFileError::Blank {
            origin: Source::File,
            key: "model",
            error,
        })
}

/// The options or the levels the file holds, in the order it holds them.
pub(super) fn labels_in(value: &Json, verb: Verb) -> Result<Option<Described>, QuestionFileError> {
    let (key, wanted) = match verb {
        Verb::Decide => return Ok(None),
        Verb::Choose => (
            "options",
            "is a list of labels, or a map from each label to its description",
        ),
        Verb::Tag => (
            "labels",
            "is a list of labels, or a map from each label to its description",
        ),
        Verb::Score => (
            "levels",
            "is a list of levels, lowest first, or a map from each level to its description",
        ),
    };
    let Some(held) = value.member(key) else {
        return Ok(None);
    };
    let shape = || QuestionFileError::Shape { key, wanted };
    let listed = match held {
        Json::Array(items) => items
            .iter()
            .map(|item| match item {
                Json::String(name) => Ok((name.clone(), None)),
                _ => Err(shape()),
            })
            .collect::<Result<Described, QuestionFileError>>()?,
        Json::Object(members) => members
            .iter()
            .map(|(name, held)| described(verb, name, held, shape))
            .collect::<Result<Described, QuestionFileError>>()?,
        _ => return Err(shape()),
    };
    Ok(Some(listed))
}

/// One member of a labels map, as the name and the description it carries.
///
/// `choose` and `tag` read a `null` as no description, and a blank string the
/// same way later on. `score` keeps `null` as the description the map named,
/// and refuses a blank string because the wire would carry it.
pub(super) fn described(
    verb: Verb,
    name: &str,
    held: &Json,
    shape: impl Fn() -> QuestionFileError,
) -> Result<(String, Option<Description>), QuestionFileError> {
    match held {
        Json::Null if verb != Verb::Score => Ok((name.to_owned(), None)),
        _ => {
            let description = Description::of_json(held).ok_or_else(shape)?;
            if verb == Verb::Score && description.blank() {
                return Err(QuestionFileError::Blank {
                    origin: Source::File,
                    key: "levels",
                    error: BlankTextError::Description,
                });
            }
            Ok((name.to_owned(), Some(description)))
        }
    }
}

/// The rule the file holds, as a number for a cut or a string for either form.
pub(super) fn threshold_in(value: &Json) -> Result<Option<Threshold>, QuestionFileError> {
    let Some(held) = value.member("threshold") else {
        return Ok(None);
    };
    let refused = |error| QuestionFileError::Threshold {
        origin: Source::File,
        error,
    };
    match held {
        Json::String(text) => text.parse().map(Some).map_err(refused),
        Json::Number(number) => number
            .as_f64()
            .ok_or(ThresholdError::NotFinite)
            .and_then(Threshold::cut)
            .map(Some)
            .map_err(refused),
        _ => Err(QuestionFileError::Shape {
            key: "threshold",
            wanted: "is a cut as a number, or a cut or a band as text",
        }),
    }
}

/// The pointers the file names under `on`, as one or as a list.
pub(super) fn pointers_in(value: &Json) -> Result<Option<Vec<Pointer>>, QuestionFileError> {
    let Some(held) = value.member("on") else {
        return Ok(None);
    };
    let shape = QuestionFileError::Shape {
        key: "on",
        wanted: "is a pointer, or a list of pointers",
    };
    let typed: Vec<&str> = match held {
        Json::String(one) => vec![one.as_str()],
        Json::Array(items) => items
            .iter()
            .map(|item| item.as_str().ok_or_else(|| shape.clone()))
            .collect::<Result<Vec<_>, QuestionFileError>>()?,
        _ => return Err(shape),
    };
    pointers(&typed, Source::File, "on").map(Some)
}

/// Read a list of pointers, naming the source of one that is not a pointer.
pub(crate) fn pointers(
    typed: &[&str],
    source: Source,
    key: &'static str,
) -> Result<Vec<Pointer>, QuestionFileError> {
    typed
        .iter()
        .map(|one| {
            Pointer::new(*one).map_err(|error| QuestionFileError::Pointer {
                origin: source,
                key,
                typed: (*one).to_owned(),
                error,
            })
        })
        .collect()
}
