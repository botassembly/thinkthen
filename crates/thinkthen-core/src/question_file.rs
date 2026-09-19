//! The question file, and the one rule that settles what a run asks.
//!
//! Both halves are pure. [`QuestionFile::parse`] takes the text of one file
//! that a caller already read and never a path, and [`resolve`] takes that
//! parsed file beside the values a caller typed and gives back the question
//! that results with the source of every setting. A caller with no command
//! line at all passes [`Typed::default`], so a library over this crate reaches
//! the same question a shell user reaches.

use std::fmt;

use serde::{Serialize, Serializer};
use thiserror::Error;

use crate::json::{Json, JsonError};
use crate::pointer::{Pointer, PointerError};
use crate::question::LabelsError;
use crate::text::{BlankTextError, Meaning, ModelName, QuestionText};
use crate::threshold::{Threshold, ThresholdError};

/// Which of the three question types a file holds or a command asks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Verb {
    /// A yes/no question.
    Decide,
    /// A pick from a fixed list.
    Choose,
    /// A placement on named levels.
    Score,
}

impl Verb {
    /// The word this verb answers to, which is also its key in a question file.
    #[must_use]
    pub const fn word(self) -> &'static str {
        match self {
            Self::Decide => "decide",
            Self::Choose => "choose",
            Self::Score => "score",
        }
    }

    /// The keys a file of this verb may hold, its own key first.
    const fn keys(self) -> &'static [&'static str] {
        match self {
            Self::Decide => &["decide", "true", "false", "threshold", "on", "model"],
            Self::Choose => &["choose", "options", "threshold", "on", "model"],
            Self::Score => &["score", "levels", "on", "model"],
        }
    }
}

impl fmt::Display for Verb {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.word())
    }
}

/// Every key any question file may hold, so an unknown one is told apart.
const EVERY_KEY: [&str; 9] = [
    "decide",
    "choose",
    "score",
    "true",
    "false",
    "options",
    "levels",
    "threshold",
    "on",
];

/// Where one setting of the question that results came from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Source {
    /// The value the user typed beside the question.
    CommandLine,
    /// The value the question file holds.
    File,
    /// The value nobody named, which the tool supplies.
    Default,
}

impl Source {
    /// The words a plan and a diagnostic name this source with.
    #[must_use]
    pub const fn words(self) -> &'static str {
        match self {
            Self::CommandLine => "command line",
            Self::File => "file",
            Self::Default => "default",
        }
    }
}

impl Serialize for Source {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.words())
    }
}

/// Why a question file, or a question file beside what was typed, is refused.
///
/// Every variant names the key at fault and the source it came from. None of
/// them names a path, because this crate never opened one.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum QuestionFileError {
    /// The bytes of the file are not JSON this tool will read.
    #[error("the question file is not JSON this tool reads: {0}")]
    NotJson(#[from] JsonError),
    /// The file holds something other than one JSON object.
    #[error("a question file is one JSON object")]
    NotAnObject,
    /// The file names none of the three question types.
    #[error("a question file holds one of `decide`, `choose`, or `score`")]
    NoVerb,
    /// The file names two of the three question types.
    #[error("a question file holds one question, and this one holds `{0}` and `{1}`")]
    TwoVerbs(&'static str, &'static str),
    /// The file holds a key no question file has.
    #[error("a question file holds no key `{0}`")]
    UnknownKey(String),
    /// The file holds a key another verb takes.
    #[error("a `{verb}` question file takes no key `{key}`")]
    KeyNotForVerb {
        /// The key the file held.
        key: String,
        /// The verb the file names.
        verb: Verb,
    },
    /// The command asks one verb and the file holds another.
    #[error("the command is `{asked}` and the question file holds a `{held}` question")]
    VerbMismatch {
        /// The verb the command line named.
        asked: Verb,
        /// The verb the file holds.
        held: Verb,
    },
    /// A value in the file is not the shape that key takes.
    #[error("`{key}` in the question file {wanted}")]
    Shape {
        /// The key whose value is the wrong shape.
        key: &'static str,
        /// What that key takes, in the words a reader can act on.
        wanted: &'static str,
    },
    /// A text value arrived blank.
    #[error("{}{error}", named(*.origin, *.key))]
    Blank {
        /// Where the value came from.
        origin: Source,
        /// The key or option that carried it.
        key: &'static str,
        /// Why the text is not text.
        error: BlankTextError,
    },
    /// The threshold is not a threshold.
    #[error("{}{error}", named(*.origin, "threshold"))]
    Threshold {
        /// Where the value came from.
        origin: Source,
        /// Why the value is not a rule.
        error: ThresholdError,
    },
    /// A band reached a verb that cuts on one winning probability.
    #[error("{}`choose` takes a single cut and never a band", named(*.0, "threshold"))]
    BandOnChoose(Source),
    /// The options or the levels are not a list the verb takes.
    #[error("{}{error}", named(*.origin, *.key))]
    Labels {
        /// Where the list came from.
        origin: Source,
        /// The key that carried it.
        key: &'static str,
        /// Why the list is not one the verb takes.
        error: LabelsError,
    },
    /// A pointer is not a JSON Pointer.
    #[error("{}`{typed}`: {error}", named(*.origin, *.key))]
    Pointer {
        /// Where the pointer came from.
        origin: Source,
        /// The key that carried it.
        key: &'static str,
        /// The pointer as it was written.
        typed: String,
        /// Why it is not a JSON Pointer.
        error: PointerError,
    },
}

/// Name the source and the key a message is about, or say nothing.
///
/// A value typed on the command line is named by its option, which the caller
/// already wrote, so the message reads as it always has. A value the file
/// holds is named by the file and by its key, because the command line the
/// user is looking at does not carry it.
fn named(source: Source, key: &str) -> String {
    match source {
        Source::CommandLine => match key {
            "threshold" | "model" | "field" | "true" | "false" => format!("--{key}: "),
            _ => String::new(),
        },
        Source::File => format!("the question file's `{key}`: "),
        Source::Default => String::new(),
    }
}

impl QuestionFileError {
    /// Where the value at fault came from, which fixes the exit code.
    #[must_use]
    pub const fn origin(&self) -> Source {
        match self {
            Self::Blank { origin, .. }
            | Self::Threshold { origin, .. }
            | Self::Labels { origin, .. }
            | Self::Pointer { origin, .. }
            | Self::BandOnChoose(origin) => *origin,
            _ => Source::File,
        }
    }
}

/// One question file, read and checked, with every setting it did not hold absent.
#[derive(Clone, Debug, PartialEq)]
pub struct QuestionFile {
    verb: Verb,
    text: QuestionText,
    yes: Option<Meaning>,
    no: Option<Meaning>,
    labels: Option<Vec<(String, Option<String>)>>,
    threshold: Option<Threshold>,
    on: Option<Vec<Pointer>>,
    model: Option<ModelName>,
}

impl QuestionFile {
    /// Which question type this file holds.
    #[must_use]
    pub const fn verb(&self) -> Verb {
        self.verb
    }

    /// Read one question file from the text a caller already holds.
    ///
    /// # Errors
    ///
    /// Returns [`QuestionFileError`] when the text is not one JSON object,
    /// when it names no question type or two of them, when it holds a key the
    /// verb does not take, and when a value is not the shape its key takes.
    pub fn parse(text: &str) -> Result<Self, QuestionFileError> {
        let value = Json::parse(text)?;
        let Json::Object(members) = &value else {
            return Err(QuestionFileError::NotAnObject);
        };
        let verb = verb_of(members)?;
        for (name, _) in members {
            if verb.keys().contains(&name.as_str()) {
                continue;
            }
            if EVERY_KEY.contains(&name.as_str()) {
                return Err(QuestionFileError::KeyNotForVerb {
                    key: name.clone(),
                    verb,
                });
            }
            return Err(QuestionFileError::UnknownKey(name.clone()));
        }
        Ok(Self {
            verb,
            text: QuestionText::new(text_at(&value, verb.word())?).map_err(|error| {
                QuestionFileError::Blank {
                    origin: Source::File,
                    key: verb.word(),
                    error,
                }
            })?,
            yes: meaning(&value, "true")?,
            no: meaning(&value, "false")?,
            labels: labels_in(&value, verb)?,
            threshold: threshold_in(&value)?,
            on: pointers_in(&value)?,
            model: model_in(&value)?,
        })
    }
}

/// Which of the three question types the file names, and only one of them.
fn verb_of(members: &[(String, Json)]) -> Result<Verb, QuestionFileError> {
    let mut found: Option<Verb> = None;
    for verb in [Verb::Decide, Verb::Choose, Verb::Score] {
        if !members.iter().any(|(name, _)| name == verb.word()) {
            continue;
        }
        if let Some(held) = found {
            return Err(QuestionFileError::TwoVerbs(held.word(), verb.word()));
        }
        found = Some(verb);
    }
    found.ok_or(QuestionFileError::NoVerb)
}

/// The text under one key, or the refusal that names the key.
fn text_at<'a>(value: &'a Json, key: &'static str) -> Result<&'a str, QuestionFileError> {
    value
        .member(key)
        .and_then(Json::as_str)
        .ok_or(QuestionFileError::Shape {
            key,
            wanted: "is text",
        })
}

/// The text that says what yes or what no means, when the file holds one.
fn meaning(value: &Json, key: &'static str) -> Result<Option<Meaning>, QuestionFileError> {
    if value.member(key).is_none() {
        return Ok(None);
    }
    Meaning::new(text_at(value, key)?)
        .map(Some)
        .map_err(|error| QuestionFileError::Blank {
            origin: Source::File,
            key,
            error,
        })
}

/// The model name the file names, when it names one.
fn model_in(value: &Json) -> Result<Option<ModelName>, QuestionFileError> {
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
fn labels_in(
    value: &Json,
    verb: Verb,
) -> Result<Option<Vec<(String, Option<String>)>>, QuestionFileError> {
    let (key, wanted) = match verb {
        Verb::Decide => return Ok(None),
        Verb::Choose => (
            "options",
            "is a list of labels, or a map from each label to its description",
        ),
        Verb::Score => ("levels", "is a list of levels, lowest first"),
    };
    let Some(held) = value.member(key) else {
        return Ok(None);
    };
    let shape = QuestionFileError::Shape { key, wanted };
    let listed = match held {
        Json::Array(items) => items
            .iter()
            .map(|item| match item {
                Json::String(name) => Ok((name.clone(), None)),
                _ => Err(shape.clone()),
            })
            .collect::<Result<Vec<_>, QuestionFileError>>()?,
        Json::Object(members) if verb == Verb::Choose => members
            .iter()
            .map(|(name, described)| match described {
                Json::String(text) => Ok((name.clone(), Some(text.clone()))),
                Json::Null => Ok((name.clone(), None)),
                _ => Err(shape.clone()),
            })
            .collect::<Result<Vec<_>, QuestionFileError>>()?,
        _ => return Err(shape),
    };
    Ok(Some(listed))
}

/// The rule the file holds, as a number for a cut or a string for either form.
fn threshold_in(value: &Json) -> Result<Option<Threshold>, QuestionFileError> {
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
fn pointers_in(value: &Json) -> Result<Option<Vec<Pointer>>, QuestionFileError> {
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

mod resolve;

pub use crate::question_file::resolve::{Resolved, Sources, Typed, resolve};

#[cfg(test)]
mod tests {
    use super::{QuestionFile, QuestionFileError as Refused, Source, Verb};
    use crate::json::JsonError;
    use crate::text::BlankTextError;
    use crate::threshold::ThresholdError;

    fn refused(text: &str) -> Refused {
        QuestionFile::parse(text).expect_err("a refused question file")
    }

    #[test]
    fn every_refusal_of_the_grammar_names_the_key_at_fault() {
        let cases: [(&str, Refused, &str); 12] = [
            ("not json", Refused::NotJson(JsonError::Syntax), "not JSON"),
            ("[1,2]", Refused::NotAnObject, "one JSON object"),
            (r#"{"model":"m"}"#, Refused::NoVerb, "`decide`"),
            (
                r#"{"decide":"a","choose":"b"}"#,
                Refused::TwoVerbs("decide", "choose"),
                "`decide` and `choose`",
            ),
            (
                r#"{"decide":"a","nonsense":1}"#,
                Refused::UnknownKey("nonsense".to_owned()),
                "nonsense",
            ),
            (
                r#"{"choose":"a","options":["x","y"],"true":"t"}"#,
                Refused::KeyNotForVerb {
                    key: "true".to_owned(),
                    verb: Verb::Choose,
                },
                "takes no key `true`",
            ),
            (
                r#"{"score":"a","levels":["x","y"],"threshold":0.5}"#,
                Refused::KeyNotForVerb {
                    key: "threshold".to_owned(),
                    verb: Verb::Score,
                },
                "takes no key `threshold`",
            ),
            (
                r#"{"decide":7}"#,
                Refused::Shape {
                    key: "decide",
                    wanted: "is text",
                },
                "`decide` in the question file is text",
            ),
            (
                r#"{"decide":"  "}"#,
                Refused::Blank {
                    origin: Source::File,
                    key: "decide",
                    error: BlankTextError::QuestionText,
                },
                "the question file's `decide`",
            ),
            (
                r#"{"choose":"a","options":"x"}"#,
                Refused::Shape {
                    key: "options",
                    wanted: "is a list of labels, or a map from each label to its description",
                },
                "map from each label",
            ),
            (
                r#"{"decide":"a","threshold":90}"#,
                Refused::Threshold {
                    origin: Source::File,
                    error: ThresholdError::CutOutOfRange,
                },
                "the question file's `threshold`",
            ),
            (
                r#"{"decide":"a","on":"body"}"#,
                Refused::Pointer {
                    origin: Source::File,
                    key: "on",
                    typed: "body".to_owned(),
                    error: crate::pointer::PointerError::NotAPointer,
                },
                "the question file's `on`",
            ),
        ];
        for (text, expected, said) in cases {
            let refusal = refused(text);
            assert_eq!(refusal, expected, "{text}");
            assert!(refusal.to_string().contains(said), "{refusal} :: {text}");
        }
    }
}
