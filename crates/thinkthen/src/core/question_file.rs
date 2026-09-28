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

use crate::core::backend_profile::ProfileName;
use crate::core::json::{Json, JsonError};
use crate::core::pointer::{Pointer, PointerError};
use crate::core::question::LabelsError;
use crate::core::text::{BlankTextError, Description, Meaning, ModelName, QuestionText};
use crate::core::threshold::{Threshold, ThresholdError};

/// Which question type a file holds or a command asks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Verb {
    /// A yes/no question.
    Decide,
    /// A pick from a fixed list.
    Choose,
    /// Any number of independent labels from a fixed list.
    Tag,
    /// A placement on named levels.
    Score,
}

impl Verb {
    /// The word this verb answers to, which is also its key in a question file.
    #[must_use]
    pub(crate) const fn word(self) -> &'static str {
        match self {
            Self::Decide => "decide",
            Self::Choose => "choose",
            Self::Tag => "tag",
            Self::Score => "score",
        }
    }

    /// The keys a file of this verb may hold, its own key first.
    const fn keys(self) -> &'static [&'static str] {
        match self {
            Self::Decide => &[
                "decide",
                "true",
                "false",
                "threshold",
                "on",
                "model",
                "profile",
            ],
            Self::Choose => &["choose", "options", "threshold", "on", "model", "profile"],
            Self::Tag => &["tag", "labels", "threshold", "on", "model", "profile"],
            Self::Score => &["score", "levels", "on", "model", "profile"],
        }
    }
}

impl fmt::Display for Verb {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.word())
    }
}

/// A list of labels, each with the description that rides beside it or none.
pub(crate) type Described = Vec<(String, Option<Description>)>;

/// Every key any question file may hold, so an unknown one is told apart.
const EVERY_KEY: [&str; 12] = [
    "decide",
    "choose",
    "tag",
    "score",
    "true",
    "false",
    "options",
    "labels",
    "levels",
    "threshold",
    "on",
    "profile",
];

/// Where one setting of the question that results came from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Source {
    /// The value the user typed beside the question.
    CommandLine,
    /// The value the question file holds.
    File,
    /// The value the read-only process configuration holds.
    Configuration,
    /// The value nobody named, which the tool supplies.
    Default,
}

impl Source {
    /// The words a plan and a diagnostic name this source with.
    #[must_use]
    pub(crate) const fn words(self) -> &'static str {
        match self {
            Self::CommandLine => "command line",
            Self::File => "file",
            Self::Configuration => "configuration",
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
pub(crate) enum QuestionFileError {
    /// The bytes of the file are not JSON this tool will read.
    #[error("{}", question_json(.0))]
    NotJson(#[from] JsonError),
    /// The file holds something other than one JSON object.
    #[error("a question file is one JSON object")]
    NotAnObject,
    /// The file names none of the four question types.
    #[error("a question file holds one of `decide`, `choose`, `tag`, or `score`")]
    NoVerb,
    /// The file names two of the four question types.
    #[error("a question file holds one question, and this one holds `{0}` and `{1}`")]
    TwoVerbs(&'static str, &'static str),
    /// The file holds a key no question file has.
    #[error("a question file takes no key `{}`{}", safe_key(.0), elsewhere(.0))]
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
    #[error("{}{error}", named(*.origin, .key))]
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
    /// A band reached `tag`, which applies one cut to every label.
    #[error("{}`tag` takes a single cut and never a band", named(*.0, "threshold"))]
    BandOnTag(Source),
    /// A rule reached the verb that answers with a number and no rule.
    #[error("{}`score` takes no rule, so cut on the number with `jq -e`", named(*.0, "threshold"))]
    RuleOnScore(Source),
    /// A band reached a caller that keeps or drops each record.
    #[error(
        "{}`filter` takes a single cut, so ask `decide --details` and split the three piles with `jq`",
        named(*.0, "threshold")
    )]
    BandOnFilter(Source),
    /// A rule reached the caller that orders records and selects none.
    #[error("{}`rank` orders and never selects, so put a cut in `filter --threshold`", named(*.0, "threshold"))]
    RuleOnRank(Source),
    /// The options or the levels are not a list the verb takes.
    #[error("{}{error}", named(*.origin, .key))]
    Labels {
        /// Where the list came from.
        origin: Source,
        /// The key that carried it.
        key: &'static str,
        /// Why the list is not one the verb takes.
        error: LabelsError,
    },
    /// A pointer is not a JSON Pointer.
    #[error("{}`{}`: {error}", pointed(*.origin, .key), safe_key(.typed))]
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

/// Where `version` belongs, for a user who copied it from another file.
fn elsewhere(key: &str) -> &'static str {
    if key == "version" {
        "; `version` belongs in a question set, a recognize file, or a relate file"
    } else {
        ""
    }
}

pub(crate) fn safe_key(key: &str) -> String {
    let encoded = serde_json::to_string(key).unwrap_or_else(|_| "\"<unprintable>\"".to_owned());
    encoded
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or("<unprintable>")
        .to_owned()
}

/// Name malformed JSON by the question file while preserving other refusals.
fn question_json(error: &JsonError) -> String {
    match error {
        JsonError::Syntax { line, column } => format!(
            "the question file is not valid JSON: the JSON at line {line} column {column} is not one"
        ),
        _ => format!("the question file is not JSON this tool reads: {error}"),
    }
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
        Source::Configuration | Source::Default => String::new(),
    }
}

/// Name the source and the key a pointer is under, or say nothing.
///
/// A pointer sits inside its own backquotes in the message, so the option
/// alone leads the line and no colon stands between the two.
fn pointed(source: Source, key: &str) -> String {
    match source {
        Source::CommandLine => format!("--{key} "),
        Source::File => format!("the question file's `{key}` "),
        Source::Configuration | Source::Default => String::new(),
    }
}

impl QuestionFileError {
    /// Where the value at fault came from, which fixes the exit code.
    #[must_use]
    pub(crate) const fn origin(&self) -> Source {
        match self {
            Self::Blank { origin, .. }
            | Self::Threshold { origin, .. }
            | Self::Labels { origin, .. }
            | Self::Pointer { origin, .. }
            | Self::BandOnChoose(origin)
            | Self::BandOnTag(origin)
            | Self::RuleOnScore(origin)
            | Self::BandOnFilter(origin)
            | Self::RuleOnRank(origin) => *origin,
            // The file is a good question file and the command asked for
            // another verb, so the line to fix is the one the user typed.
            Self::VerbMismatch { .. } => Source::CommandLine,
            _ => Source::File,
        }
    }
}

/// One question file, read and checked, with every setting it did not hold absent.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct QuestionFile {
    verb: Verb,
    text: QuestionText,
    yes: Option<Meaning>,
    no: Option<Meaning>,
    labels: Option<Described>,
    threshold: Option<Threshold>,
    on: Option<Vec<Pointer>>,
    model: Option<ModelName>,
    profile: Option<ProfileName>,
}

impl QuestionFile {
    /// Which question type this file holds.
    #[must_use]
    pub(crate) const fn verb(&self) -> Verb {
        self.verb
    }

    pub(crate) const fn has_threshold(&self) -> bool {
        self.threshold.is_some()
    }

    /// Read one question file from the text a caller already holds.
    ///
    /// # Errors
    ///
    /// Returns [`QuestionFileError`] when the text is not one JSON object,
    /// when it names no question type or two of them, when it holds a key the
    /// verb does not take, and when a value is not the shape its key takes.
    pub(crate) fn parse(text: &str) -> Result<Self, QuestionFileError> {
        Self::parsed(Json::parse(text)?)
    }

    /// Read one whole question file, taking `batch` off the top of a `decide`
    /// or `choose` file first and returning its raw value. A question set's entries use
    /// [`Self::parse`], so they still refuse `batch`.
    ///
    /// # Errors
    ///
    /// Returns [`QuestionFileError`] as [`Self::parse`] does.
    pub(crate) fn parse_top(text: &str) -> Result<(Self, Option<Json>), QuestionFileError> {
        let mut value = Json::parse(text)?;
        let mut batch = None;
        if let Json::Object(members) = &mut value
            && matches!(verb_of(members), Ok(Verb::Decide | Verb::Choose))
            && let Some(at) = members.iter().position(|(name, _)| name == "batch")
        {
            batch = Some(members.remove(at).1);
        }
        Ok((Self::parsed(value)?, batch))
    }

    fn parsed(value: Json) -> Result<Self, QuestionFileError> {
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
            text: question_text(&value, verb.word())?,
            yes: meaning(&value, "true")?,
            no: meaning(&value, "false")?,
            labels: labels_in(&value, verb)?,
            threshold: threshold_in(&value)?,
            on: pointers_in(&value)?,
            model: model_in(&value)?,
            profile: profile_in(&value)?,
        })
    }
}

/// Which of the four question types the file names, and only one of them.
fn verb_of(members: &[(String, Json)]) -> Result<Verb, QuestionFileError> {
    let mut found: Option<Verb> = None;
    for verb in [Verb::Decide, Verb::Choose, Verb::Tag, Verb::Score] {
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

mod fields;
mod profile;
mod resolve;

pub(crate) use crate::core::question_file::fields::pointers;
pub(crate) use crate::core::question_file::resolve::{Cutting, Resolved, Sources, Typed, resolve};
use fields::{labels_in, meaning, model_in, pointers_in, question_text, threshold_in};
use profile::profile_in;

#[cfg(test)]
mod tests;
