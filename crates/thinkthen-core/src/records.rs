//! Framing a stream into records, and building the evidence one record sends.
//!
//! Everything here is a pure function over bytes the binary read. The binary
//! opens the file, reads standard input, and writes the rows. This module
//! decides what one record is and what part of it leaves the machine.

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use thiserror::Error;

use crate::json::{Json, JsonError};
use crate::pointer::Pointer;
use crate::question::{Labels, LabelsError};
use crate::render::{RenderError, json_line};
use crate::text::{BlankTextError, Evidence};

/// The most one record may hold before the tool refuses to judge it.
///
/// The vendor reads about 32,000 tokens of evidence, far under a megabyte, so a
/// record this large is a mistake in the pipeline rather than a judgment anyone
/// asked for. No option sets it, and the bound keeps a hostile or mistaken
/// stream out of this process's memory.
pub const MAX_RECORD_BYTES: usize = 16 * 1024 * 1024;

/// What one record is.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Framing {
    /// The whole input is one text document and one record.
    Document,
    /// Each line is one text record.
    Lines,
    /// Each line is one JSON value and one record.
    Jsonl,
}

/// Why the framing and the pointers cannot act together.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ReadingError {
    /// `--field` was given beside `--lines`.
    #[error("--field: a text line has no members, so --lines takes no pointer")]
    TextHasNoMembers,
    /// Two pointers end in one name, so one would hide the other.
    #[error("--field: two pointers end in `{0}`, and one evidence object holds each name once")]
    KeyClash(String),
}

/// Why one record could not become the evidence of one request.
///
/// No variant carries any part of a record, because a record is evidence. A
/// pointer is named, because the user typed it on the command line.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum RecordError {
    /// The bytes of the record are not text.
    #[error("the record is not valid UTF-8")]
    NotUtf8,
    /// The record is past [`MAX_RECORD_BYTES`].
    #[error("the record is over 16 MiB, which is far past what a backend reads in one request")]
    TooLarge,
    /// The record is not JSON this tool will read.
    #[error("{0}")]
    Json(#[from] JsonError),
    /// The whole input is not valid JSON where the safe position names.
    #[error("the input is not valid JSON: the JSON at line {line} column {column} is not one")]
    InputJson {
        /// The one-based line where the parser stopped.
        line: usize,
        /// The one-based column where the parser stopped, or zero at empty input.
        column: usize,
    },
    /// The record holds nothing at one of the pointers.
    #[error("the record holds nothing at `{0}`")]
    Missed(String),
    /// A pointer was taken into a text record, which has no members.
    #[error("{}", ReadingError::TextHasNoMembers)]
    TextHasNoMembers,
    /// The evidence the record yields is blank.
    #[error("{0}")]
    Blank(#[from] BlankTextError),
    /// The evidence could not be written as JSON.
    #[error("{0}")]
    Render(#[from] RenderError),
    /// The pointer names no candidate list this tool reads.
    #[error(
        "the options at `{0}` are a list of labels or a map from each label to its description"
    )]
    OptionsShape(String),
    /// The candidate list the record holds is not a list the verb takes.
    #[error("{0}")]
    Options(#[from] LabelsError),
}

/// One record, as it arrived, which `--details` prints back under `input`.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Record(Held);

/// What one record holds, which the framing decides.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
enum Held {
    /// A text record, which is one line or one whole document.
    Text(String),
    /// A JSON record.
    Json(Json),
}

impl Record {
    /// True when this record is an object holding the given member.
    #[must_use]
    pub fn has_member(&self, name: &str) -> bool {
        matches!(&self.0, Held::Json(Json::Object(members)) if members.iter().any(|(held, _)| held == name))
    }

    /// Add named answers to an object record, or return the answers alone.
    #[must_use]
    pub fn annotated(self, answers: Vec<(String, crate::Value)>) -> AnnotatedRecord {
        AnnotatedRecord {
            record: self,
            answers,
        }
    }

    /// Read the candidate list this record carries where the pointer names one.
    ///
    /// A list of labels gives each option no description. A map from label to
    /// description gives each one the description it holds, and the adapter
    /// sends that description with the option, as `specification/backends.md`
    /// writes out.
    ///
    /// # Errors
    ///
    /// Returns [`RecordError`] when the pointer finds nothing, when what it
    /// finds is neither a list of strings nor a map to strings, and when the
    /// list is not one `choose` takes.
    pub fn choices(&self, pointer: &Pointer) -> Result<Labels, RecordError> {
        let Held::Json(value) = &self.0 else {
            return Err(RecordError::TextHasNoMembers);
        };
        let shape = || RecordError::OptionsShape(pointer.as_str().to_owned());
        let listed = match found(pointer, value)? {
            Json::Array(items) => items
                .iter()
                .map(|item| match item {
                    Json::String(name) => Ok((name.clone(), None)),
                    _ => Err(shape()),
                })
                .collect::<Result<Vec<_>, RecordError>>()?,
            Json::Object(members) => members
                .iter()
                .map(|(name, held)| match held {
                    Json::String(text) => Ok((name.clone(), Some(text.clone()))),
                    _ => Err(shape()),
                })
                .collect::<Result<Vec<_>, RecordError>>()?,
            _ => return Err(shape()),
        };
        Ok(Labels::described(listed)?)
    }
}

/// One record with its named bare answers appended.
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotatedRecord {
    record: Record,
    answers: Vec<(String, crate::Value)>,
}

impl Serialize for AnnotatedRecord {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let original = match &self.record.0 {
            Held::Json(Json::Object(members)) => Some(members.as_slice()),
            Held::Text(_) | Held::Json(_) => None,
        };
        let mut map =
            serializer.serialize_map(Some(original.map_or(0, <[_]>::len) + self.answers.len()))?;
        if let Some(members) = original {
            for (name, value) in members {
                map.serialize_entry(name, value)?;
            }
        }
        for (name, value) in &self.answers {
            map.serialize_entry(name, value)?;
        }
        map.end()
    }
}

/// The framing and the pointers one run reads its records by.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reading {
    framing: Framing,
    fields: Vec<Pointer>,
}

/// The `input` field a plan carries in record mode.
#[derive(Debug, Serialize)]
pub(crate) struct ReadingPlan<'a> {
    framing: Framing,
    field: &'a [Pointer],
}

impl Reading {
    /// Take the framing and the pointers, or say why they cannot act together.
    ///
    /// # Errors
    ///
    /// Returns [`ReadingError`] when a pointer is given beside `--lines`, and
    /// when two pointers end in one name.
    pub fn new(framing: Framing, fields: Vec<Pointer>) -> Result<Self, ReadingError> {
        if framing == Framing::Lines && !fields.is_empty() {
            return Err(ReadingError::TextHasNoMembers);
        }
        for (place, pointer) in fields.iter().enumerate() {
            if fields
                .iter()
                .skip(place + 1)
                .any(|other| other.key() == pointer.key())
            {
                return Err(ReadingError::KeyClash(pointer.key().to_owned()));
            }
        }
        Ok(Self { framing, fields })
    }

    /// True when the input is a stream of records rather than one document.
    #[must_use]
    pub fn streams(&self) -> bool {
        self.framing != Framing::Document
    }

    /// Name the framing and the pointers, as the record-mode plan prints them.
    pub(crate) fn plan(&self) -> ReadingPlan<'_> {
        ReadingPlan {
            framing: self.framing,
            field: &self.fields,
        }
    }

    /// The record's own bytes as text, with the line ending taken off.
    ///
    /// `filter` and `rank` print a record back exactly as it arrived, so this
    /// parses nothing and writes nothing through a JSON encoder. Odd spacing
    /// and a trailing space survive, and a carriage return before the line
    /// feed goes with it, so the line ending is written back as a line feed.
    ///
    /// # Errors
    ///
    /// Returns [`RecordError::NotUtf8`] when the bytes are not text.
    pub fn as_it_arrived<'a>(&self, bytes: &'a [u8]) -> Result<&'a str, RecordError> {
        str::from_utf8(self.ended(bytes)).map_err(|_| RecordError::NotUtf8)
    }

    /// The bytes without the line feed that ended them, and without a
    /// carriage return before it. Nothing ends a whole input.
    fn ended<'a>(&self, bytes: &'a [u8]) -> &'a [u8] {
        let line = self.streams().then(|| bytes.strip_suffix(b"\n")).flatten();
        line.map_or(bytes, |line| line.strip_suffix(b"\r").unwrap_or(line))
    }

    /// Read one record from the bytes the binary handed over.
    ///
    /// # Errors
    ///
    /// Returns [`RecordError`] when the record is past [`MAX_RECORD_BYTES`],
    /// when the bytes are not text, and when a JSON record is not JSON this
    /// tool will read.
    pub fn record(&self, bytes: &[u8]) -> Result<Record, RecordError> {
        // The line that ended the record is not part of it, so it is dropped
        // before the size is measured. The size is read before the bytes are,
        // because a huge record is too large whatever its bytes turn out to be.
        let bytes = self.ended(bytes);
        if bytes.len() > MAX_RECORD_BYTES {
            return Err(RecordError::TooLarge);
        }
        let text = str::from_utf8(bytes).map_err(|_| RecordError::NotUtf8)?;
        if self.framing == Framing::Jsonl || !self.fields.is_empty() {
            let value = Json::parse(text).map_err(|error| match (self.framing, error) {
                (Framing::Document, JsonError::Syntax { line, column }) => {
                    RecordError::InputJson { line, column }
                }
                (_, other) => RecordError::Json(other),
            })?;
            return Ok(Record(Held::Json(value)));
        }
        Ok(Record(Held::Text(text.to_owned())))
    }

    /// Read an annotation record, preserving a whole JSON object when present.
    ///
    /// A document is normally free text. `annotate` also accepts one JSON
    /// object without requiring record framing because it can append named
    /// answers to that object. Other commands keep the established document
    /// behavior through [`Self::record`].
    pub fn annotation_record(&self, bytes: &[u8]) -> Result<Record, RecordError> {
        if self.framing != Framing::Document || !self.fields.is_empty() {
            return self.record(bytes);
        }
        if bytes.len() > MAX_RECORD_BYTES {
            return Err(RecordError::TooLarge);
        }
        let text = str::from_utf8(bytes).map_err(|_| RecordError::NotUtf8)?;
        match Json::parse(text) {
            Ok(value) => Ok(Record(Held::Json(value))),
            Err(JsonError::Syntax { .. }) => Ok(Record(Held::Text(text.to_owned()))),
            Err(error) => Err(RecordError::Json(error)),
        }
    }

    /// Build the evidence this record sends, which is all that leaves the machine.
    ///
    /// # Errors
    ///
    /// Returns [`RecordError`] when a pointer finds nothing, when the evidence
    /// is blank, and when it cannot be written as JSON.
    pub fn evidence(&self, record: &Record) -> Result<Evidence, RecordError> {
        let value = match (&record.0, self.fields.as_slice()) {
            (Held::Text(text), []) => return Ok(Evidence::new(text.as_str())?),
            (Held::Text(_), _) => return Err(RecordError::TextHasNoMembers),
            (Held::Json(value), []) => value.clone(),
            (Held::Json(value), [pointer]) => found(pointer, value)?.clone(),
            (Held::Json(value), pointers) => Json::Object(
                pointers
                    .iter()
                    .map(|pointer| Ok((pointer.key().to_owned(), found(pointer, value)?.clone())))
                    .collect::<Result<Vec<_>, RecordError>>()?,
            ),
        };
        let text = match value.as_str() {
            Some(text) => text.to_owned(),
            None => json_line(&value)?,
        };
        Ok(Evidence::new(text)?)
    }
}

/// The value the pointer names, or the refusal that names the pointer.
fn found<'a>(pointer: &Pointer, value: &'a Json) -> Result<&'a Json, RecordError> {
    pointer
        .resolve(value)
        .ok_or_else(|| RecordError::Missed(pointer.as_str().to_owned()))
}

#[cfg(test)]
mod tests;
