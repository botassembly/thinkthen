//! Question arguments and `@file` reads (ticket 0110 decision 10).
//!
//! A question is plain text, `'@file.json'`, or the file grammar's JSON. A
//! file opens through the caller's own DuckDB file system, so the caller's
//! file settings decide every read and this binding copies none of them. A
//! file read once stays with the caller's init for the rest of that init.

use std::collections::HashMap;
use std::sync::Arc;

use thinkthen::{Engine, Error, ErrorKind, LoadedQuestion, Question, QuestionSet, Recognize};

use crate::engines::Asked;
use crate::errors::{failure, prefix, usage};

mod ffi;

pub(crate) use ffi::{Files, Opened};

/// One calling database's state for one init: its engine, its file system,
/// and the files this init has read.
#[derive(Debug)]
pub(crate) struct Caller {
    pub(crate) engine: Arc<Engine>,
    pub(crate) asked: Asked,
    files: Files,
    read: HashMap<String, String>,
}

/// A member verb's question: plain text beside a `LIST` of members.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Members {
    Choose,
    Score,
    Tag,
}

impl Caller {
    pub(crate) fn new(engine: Arc<Engine>, asked: Asked, files: Files) -> Self {
        Self {
            engine,
            asked,
            files,
            read: HashMap::new(),
        }
    }

    /// The caller's own client context and file system.
    pub(crate) const fn files(&self) -> &Files {
        &self.files
    }

    /// One `@file` text, opened once per init through the caller's files.
    pub(crate) fn file(&mut self, path: &str) -> Result<String, String> {
        if let Some(text) = self.read.get(path) {
            return Ok(text.clone());
        }
        let text = match self.files.read(path) {
            Opened::Text(text) => text,
            Opened::Missing => {
                return Err(local(&format!(
                    "the question file {path} was not read: it does not exist or could not be opened"
                )));
            }
            Opened::TooLarge => {
                return Err(local(&format!(
                    "the question file {path} was not read: it holds more than 1 MiB"
                )));
            }
            Opened::NotText => {
                return Err(local(&format!(
                    "the question file {path} was not read: it is not UTF-8 text"
                )));
            }
            Opened::Refused => {
                return Err(local(&format!(
                    "the question file {path} was not read: this database's file settings refuse it"
                )));
            }
        };
        self.read.insert(path.to_owned(), text.clone());
        Ok(text)
    }

    /// A question: plain text as a decide under the default cut, a file, or
    /// JSON. A file or JSON question may carry a band or any verb.
    pub(crate) fn question(&mut self, argument: &str) -> Result<LoadedQuestion, String> {
        if let Some(path) = argument.strip_prefix('@') {
            let text = self.file(path)?;
            Question::from_json(&text).map_err(|error| from_file(&error))
        } else {
            inline(argument)
        }
    }

    /// A question set for `thinkthen_annotate`: a file or JSON.
    pub(crate) fn set(&mut self, argument: &str) -> Result<QuestionSet, String> {
        if let Some(path) = argument.strip_prefix('@') {
            let text = self.file(path)?;
            QuestionSet::from_json(&text).map_err(|error| from_file(&error))
        } else if argument.starts_with('{') {
            QuestionSet::from_json(argument).map_err(|error| failure(&error))
        } else {
            Err(usage(
                "annotate names a question set as '@form.json' or JSON",
            ))
        }
    }

    /// A recognize file for `thinkthen_relations`: a file or JSON.
    pub(crate) fn recognize(&mut self, argument: &str) -> Result<Recognize, String> {
        if let Some(path) = argument.strip_prefix('@') {
            let text = self.file(path)?;
            Recognize::from_json(&text).map_err(|error| from_file(&error))
        } else {
            Recognize::from_json(argument).map_err(|error| failure(&error))
        }
    }
}

/// A question written in the call: JSON, or plain text as a decide under
/// the default cut.
pub(crate) fn inline(argument: &str) -> Result<LoadedQuestion, String> {
    if argument.starts_with('{') {
        Question::from_json(argument).map_err(|error| failure(&error))
    } else {
        Question::decide(argument)
            .map(|built| LoadedQuestion::Question(built.cut()))
            .map_err(|error| failure(&error))
    }
}

/// A member verb's question from plain text and its `LIST` members, as the
/// one member of a set named `value`, so bulk calls go through `annotate`.
pub(crate) fn members(kind: Members, text: &str, list: &[String]) -> Result<QuestionSet, String> {
    if text.starts_with('@') || text.starts_with('{') {
        return Err(usage(match kind {
            Members::Choose => {
                "choose takes its question as plain text and its options in the list"
            }
            Members::Score => "score takes its question as plain text and its levels in the list",
            Members::Tag => "tag takes its question as plain text and its labels in the list",
        }));
    }
    let refused = |error: Error| failure(&error);
    let question = match kind {
        Members::Choose => list
            .iter()
            .try_fold(
                Question::choose_labels(text).map_err(refused)?,
                |built, label| built.label(label, None),
            )
            .and_then(thinkthen::LabelBuilder::build),
        Members::Tag => list
            .iter()
            .try_fold(
                Question::tag_labels(text).map_err(refused)?,
                |built, label| built.label(label, None),
            )
            .and_then(thinkthen::LabelBuilder::build),
        Members::Score => list
            .iter()
            .try_fold(Question::score(text).map_err(refused)?, |built, level| {
                built.level(level, None)
            })
            .and_then(thinkthen::ScoreBuilder::build),
    }
    .map_err(refused)?;
    QuestionSet::builder()
        .question("value", question)
        .and_then(thinkthen::QuestionSetBuilder::build)
        .map_err(refused)
}

/// A failure from a file's text reads `local`, per ticket 0095.
pub(crate) fn from_file(error: &Error) -> String {
    if error.kind() == ErrorKind::Usage {
        local(error.detail().message())
    } else {
        failure(error)
    }
}

fn local(message: &str) -> String {
    format!("{}{message}", prefix(ErrorKind::Local))
}
