//! A saved, ordered set of named questions.

use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use sha2::{Digest as _, Sha256};
use std::fmt;
use thiserror::Error;

use crate::core::backend_profile::{ProfileError, ProfileName};
use crate::core::batch::BatchRecord;
use crate::core::digest::Canonical;
use crate::core::json::{Json, JsonError};
use crate::core::pointer::Pointer;
use crate::core::question::Question;
use crate::core::question_file::{QuestionFile, QuestionFileError, Typed, Verb, resolve};
use crate::core::records::{Framing, Reading, ReadingError, RecordError};
use crate::core::render::{RenderError, json_line};
use crate::core::text::Evidence;
use crate::core::threshold::{Threshold, ThresholdError};

/// Why one group's part of a record could not be read.
#[derive(Debug)]
pub(crate) enum PartError {
    /// The group's pointers could not form a reading; a parsed set checked them.
    Reading(ReadingError),
    /// The record's selection is text, or holds nothing at a pointer.
    Record(RecordError),
}

/// Why a question set was refused.
#[derive(Clone, Error, PartialEq)]
pub(crate) enum QuestionSetError {
    /// The bytes are not JSON this tool reads.
    #[error(
        "the question set is not valid JSON: the JSON at line {line} column {column} is not one"
    )]
    Syntax {
        /// One-based line where parsing stopped.
        line: usize,
        /// One-based column where parsing stopped.
        column: usize,
    },
    /// The set nests past the JSON depth limit.
    #[error("the question set is not JSON this tool reads: {}", JsonError::TooDeep)]
    TooDeep,
    /// A duplicate key occurred at an unknown nested path.
    #[error("`{0}` appears more than once in the question set")]
    Duplicate(String),
    /// The top level is not an object.
    #[error("the question set is one JSON object")]
    NotObject,
    /// The required top-level question wrapper is absent.
    #[error("the question set is missing its `questions` object")]
    MissingQuestions,
    /// A key is not allowed at its path.
    #[error("the question set holds no key `{0}`")]
    UnknownKey(String),
    /// A required value has the wrong shape.
    #[error("`{path}` {wanted}")]
    Shape {
        /// Dot-separated key path at fault.
        path: String,
        /// The shape the value must have.
        wanted: &'static str,
    },
    /// The set has no questions.
    #[error("`questions` holds at least one named question")]
    Empty,
    /// A question name is outside the stable grammar.
    #[error("`questions.{0}` uses lowercase letters, digits, and underscores, and is not empty")]
    Name(String),
    /// One nested question was refused.
    #[error("`questions.{name}`: {error}")]
    Question {
        /// Name of the question at fault.
        name: String,
        /// Refusal from the shared single-question grammar.
        error: QuestionFileError,
    },
    /// A nested value broke a rule after its path was known.
    #[error("`{path}`: {why}")]
    Nested {
        /// Dot-separated key path at fault.
        path: String,
        /// The rule the nested value broke.
        why: String,
    },
    /// Two pointers would hide one another in the evidence object.
    #[error(
        "`questions.{name}.on`: two pointers end in `{key}`, and one evidence object holds each name once"
    )]
    PointerClash {
        /// Name of the question at fault.
        name: String,
        /// Last pointer part that occurred twice.
        key: String,
    },
    /// The top-level threshold was refused.
    #[error("`threshold`: {0}")]
    Threshold(ThresholdError),
    /// A canonical document could not be rendered.
    #[error("the resolved question set could not be written as JSON")]
    Render,
}

impl fmt::Debug for QuestionSetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Duplicate(_) => formatter.write_str("Duplicate(<path withheld>)"),
            other => formatter
                .debug_tuple("QuestionSetError")
                .field(&other.to_string())
                .finish(),
        }
    }
}

/// One named and fully resolved question.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NamedQuestion {
    name: String,
    question: Question,
    threshold: Option<Threshold>,
    on: Vec<Pointer>,
}

impl NamedQuestion {
    /// Read the stable output name.
    #[must_use]
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    /// Read the question sent to the backend.
    #[must_use]
    pub(crate) const fn question(&self) -> &Question {
        &self.question
    }
    /// Read the rule applied to its answer.
    #[must_use]
    pub(crate) const fn threshold(&self) -> Option<Threshold> {
        self.threshold
    }
    /// Read the normalized evidence pointers.
    #[must_use]
    pub(crate) fn on(&self) -> &[Pointer] {
        &self.on
    }
}

/// A resolved question set in file order.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct QuestionSet {
    questions: Vec<NamedQuestion>,
    profile: Option<ProfileName>,
    batch: Option<Json>,
}

impl QuestionSet {
    /// Parse and resolve one question set without touching a file.
    pub(crate) fn parse(text: &str) -> Result<Self, QuestionSetError> {
        let value = Json::parse(text).map_err(json_error)?;
        let Json::Object(members) = &value else {
            return Err(QuestionSetError::NotObject);
        };
        if value.member("questions").is_none() {
            return Err(QuestionSetError::MissingQuestions);
        }
        for (key, _) in members {
            if !["version", "threshold", "profile", "questions", "batch"].contains(&key.as_str()) {
                return Err(QuestionSetError::UnknownKey(key.clone()));
            }
        }
        match value.member("version") {
            Some(Json::Number(number)) if number.as_u64() == Some(1) => {}
            _ => {
                return Err(QuestionSetError::Shape {
                    path: "version".to_owned(),
                    wanted: "is the number 1",
                });
            }
        }
        let inherited = threshold(&value)?;
        let profile = profile(&value)?;
        let Some(Json::Object(entries)) = value.member("questions") else {
            return Err(QuestionSetError::Shape {
                path: "questions".to_owned(),
                wanted: "is an object",
            });
        };
        if entries.is_empty() {
            return Err(QuestionSetError::Empty);
        }
        let mut questions = Vec::with_capacity(entries.len());
        for (name, held) in entries {
            check_name(name)?;
            let Json::Object(fields) = held else {
                return Err(QuestionSetError::Shape {
                    path: format!("questions.{name}"),
                    wanted: "is one question object",
                });
            };
            if let Some((key, _)) = fields
                .iter()
                .find(|(key, _)| matches!(key.as_str(), "model" | "profile"))
            {
                return Err(QuestionSetError::UnknownKey(format!(
                    "questions.{name}.{key}"
                )));
            }
            let written = json_line(held).map_err(|_| QuestionSetError::Render)?;
            let file = QuestionFile::parse(&written).map_err(|error| nested(name, error))?;
            let typed = Typed {
                threshold: (file.verb() == Verb::Decide && !file.has_threshold())
                    .then(|| inherited.map(|rule| rule.to_string()))
                    .flatten(),
                ..Typed::default()
            };
            let resolved = resolve(file.verb(), None, Some(&file), &typed)
                .map_err(|error| nested(name, error))?;
            pointer_clash(name, resolved.on())?;
            let on = if resolved.on().is_empty() {
                vec![Pointer::new("").map_err(|_| QuestionSetError::Render)?]
            } else {
                resolved.on().to_vec()
            };
            let question = resolved
                .question()
                .cloned()
                .ok_or(QuestionSetError::Render)?;
            questions.push(NamedQuestion {
                name: name.clone(),
                question,
                threshold: resolved.threshold(),
                on,
            });
        }
        Ok(Self {
            questions,
            profile,
            batch: value.member("batch").cloned(),
        })
    }

    /// Name built questions, in order, each reading the whole record.
    pub(crate) fn from_parts(
        members: Vec<(String, Question, Option<Threshold>)>,
    ) -> Result<Self, QuestionSetError> {
        if members.is_empty() {
            return Err(QuestionSetError::Empty);
        }
        let root = Pointer::new("").map_err(|_| QuestionSetError::Render)?;
        let mut questions: Vec<NamedQuestion> = Vec::with_capacity(members.len());
        for (name, question, threshold) in members {
            check_name(&name)?;
            if questions.iter().any(|held| held.name == name) {
                return Err(QuestionSetError::Duplicate(name));
            }
            questions.push(NamedQuestion {
                name,
                question,
                threshold,
                on: vec![root.clone()],
            });
        }
        Ok(Self {
            questions,
            profile: None,
            batch: None,
        })
    }

    /// Read the resolved questions in file order.
    #[must_use]
    pub(crate) fn questions(&self) -> &[NamedQuestion] {
        &self.questions
    }

    /// The profile this set's thresholds were tuned under, when named.
    pub(crate) const fn profile(&self) -> Option<&ProfileName> {
        self.profile.as_ref()
    }

    /// The optional top-level batch setting, outside the resolved digest.
    pub(crate) const fn batch(&self) -> Option<&Json> {
        self.batch.as_ref()
    }

    /// Group question indexes by identical normalized pointer lists.
    #[must_use]
    pub(crate) fn groups(&self) -> Vec<Vec<usize>> {
        let mut groups: Vec<(Vec<Pointer>, Vec<usize>)> = Vec::new();
        for (place, question) in self.questions.iter().enumerate() {
            if let Some((_, places)) = groups
                .iter_mut()
                .find(|(pointers, _)| *pointers == question.on)
            {
                places.push(place);
            } else {
                groups.push((question.on.clone(), vec![place]));
            }
        }
        groups.into_iter().map(|(_, places)| places).collect()
    }

    /// The name of the first question, in file order, that reads a part.
    #[must_use]
    pub(crate) fn first_part(&self) -> Option<&str> {
        self.questions
            .iter()
            .find(|question| !reads_root(&question.on))
            .map(NamedQuestion::name)
    }

    /// The evidence one group sees: the record's own evidence when the group
    /// reads the root, or else the parts its pointers select inside the
    /// record's JSON value. A string is text and is never parsed again.
    pub(crate) fn group_evidence(
        &self,
        group: &[usize],
        record: &BatchRecord,
    ) -> Result<Evidence, PartError> {
        let first = group.first().and_then(|place| self.questions.get(*place));
        let on = first.map_or(&[][..], |first| first.on.as_slice());
        if reads_root(on) {
            return Ok(record.evidence.clone());
        }
        if !matches!(record.value, Json::Object(_) | Json::Array(_)) {
            let name = first.map_or(String::new(), |first| first.name.clone());
            return Err(PartError::Record(RecordError::TextPart(name)));
        }
        let reading = Reading::new(Framing::Document, on.to_vec()).map_err(PartError::Reading)?;
        reading.part(&record.value).map_err(PartError::Record)
    }

    /// Digest the resolved behavior, independent of its path and whitespace.
    pub(crate) fn sha256(&self) -> Result<String, RenderError> {
        let canonical = json_line(&CanonicalSet(self))?;
        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        Ok(hasher
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect())
    }

    /// Write an explicit source spelling of the resolved behavior for round-trip tests.
    #[cfg(test)]
    fn resolved_json(&self) -> Result<String, RenderError> {
        resolved::json(self)
    }
}

/// True when an `on` list reads the whole record.
fn reads_root(on: &[Pointer]) -> bool {
    matches!(on, [root] if root.as_str().is_empty())
}

fn profile(value: &Json) -> Result<Option<ProfileName>, QuestionSetError> {
    match value.member("profile") {
        None => Ok(None),
        Some(Json::String(name)) => ProfileName::new(name)
            .map(Some)
            .map_err(|error| match error {
                ProfileError::Name => QuestionSetError::Shape {
                    path: "profile".to_owned(),
                    wanted: "is a safe profile name",
                },
                _ => QuestionSetError::Render,
            }),
        Some(_) => Err(QuestionSetError::Shape {
            path: "profile".to_owned(),
            wanted: "is a safe profile name",
        }),
    }
}

fn nested(name: &str, error: QuestionFileError) -> QuestionSetError {
    let path = |key: &str| format!("questions.{name}.{key}");
    match error {
        QuestionFileError::UnknownKey(key) => QuestionSetError::UnknownKey(path(&key)),
        QuestionFileError::Shape { key, wanted } => QuestionSetError::Shape {
            path: path(key),
            wanted,
        },
        QuestionFileError::KeyNotForVerb { key, verb } => QuestionSetError::Nested {
            path: path(&key),
            why: format!("a `{verb}` question takes no such key"),
        },
        QuestionFileError::Blank { key, error, .. } => QuestionSetError::Nested {
            path: path(key),
            why: error.to_string(),
        },
        QuestionFileError::Threshold { error, .. } => QuestionSetError::Nested {
            path: path("threshold"),
            why: error.to_string(),
        },
        QuestionFileError::Labels { key, error, .. } => QuestionSetError::Nested {
            path: path(key),
            why: error.to_string(),
        },
        QuestionFileError::Pointer { key, error, .. } => QuestionSetError::Nested {
            path: path(key),
            why: error.to_string(),
        },
        other => QuestionSetError::Question {
            name: name.to_owned(),
            error: other,
        },
    }
}

fn json_error(error: JsonError) -> QuestionSetError {
    match error {
        JsonError::Syntax { line, column } => QuestionSetError::Syntax { line, column },
        JsonError::DuplicateName { path } => QuestionSetError::Duplicate(path),
        JsonError::TooDeep => QuestionSetError::TooDeep,
        JsonError::NotFinite => QuestionSetError::Shape {
            path: "number".to_owned(),
            wanted: "is finite",
        },
    }
}

fn threshold(value: &Json) -> Result<Option<Threshold>, QuestionSetError> {
    let Some(held) = value.member("threshold") else {
        return Ok(None);
    };
    match held {
        Json::String(text) => text.parse().map(Some).map_err(QuestionSetError::Threshold),
        Json::Number(number) => number
            .as_f64()
            .ok_or(ThresholdError::NotFinite)
            .and_then(Threshold::cut)
            .map(Some)
            .map_err(QuestionSetError::Threshold),
        _ => Err(QuestionSetError::Shape {
            path: "threshold".to_owned(),
            wanted: "is a cut or band",
        }),
    }
}

fn pointer_clash(name: &str, pointers: &[Pointer]) -> Result<(), QuestionSetError> {
    for (place, pointer) in pointers.iter().enumerate() {
        if pointers
            .iter()
            .skip(place + 1)
            .any(|other| other.key() == pointer.key())
        {
            return Err(QuestionSetError::PointerClash {
                name: name.to_owned(),
                key: pointer.key().to_owned(),
            });
        }
    }
    Ok(())
}

struct CanonicalSet<'a>(&'a QuestionSet);
impl Serialize for CanonicalSet<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("version", &1)?;
        if let Some(profile) = &self.0.profile {
            map.serialize_entry("profile", profile.as_str())?;
        }
        map.serialize_entry("questions", &CanonicalQuestions(&self.0.questions))?;
        map.end()
    }
}
struct CanonicalQuestions<'a>(&'a [NamedQuestion]);
impl Serialize for CanonicalQuestions<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for question in self.0 {
            seq.serialize_element(&CanonicalNamed(question))?;
        }
        seq.end()
    }
}
struct CanonicalNamed<'a>(&'a NamedQuestion);
impl Serialize for CanonicalNamed<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("name", &self.0.name)?;
        map.serialize_entry(
            "question",
            &Canonical::new(&self.0.question, self.0.threshold),
        )?;
        map.serialize_entry("on", &self.0.on)?;
        map.end()
    }
}

#[cfg(test)]
mod resolved;

#[cfg(test)]
mod tests;

/// A member name is lowercase ASCII letters, digits, and underscores.
pub(crate) fn check_name(name: &str) -> Result<(), QuestionSetError> {
    if name.is_empty()
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return Err(QuestionSetError::Name(name.to_owned()));
    }
    Ok(())
}
