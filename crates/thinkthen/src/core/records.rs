//! Framing a stream into records, and building the evidence one record sends.
//!
//! Everything here is a pure function over bytes the binary read. The binary
//! opens the file, reads standard input, and writes the rows. This module
//! decides what one record is and what part of it leaves the machine.

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use thiserror::Error;

use crate::core::batch::BatchRecord;
use crate::core::json::{Json, JsonError};
use crate::core::pointer::Pointer;
use crate::core::question::{Labels, LabelsError};
use crate::core::render::{RenderError, json_line};
use crate::core::text::{BlankTextError, Description, Evidence, EvidenceShapeError, Withheld};

/// The most one record may hold before the tool refuses to judge it.
///
/// The vendor reads about 32,000 tokens of evidence, far under a megabyte, so a
/// record this large is a mistake in the pipeline rather than a judgment anyone
/// asked for. No option sets it, and the bound keeps a hostile or mistaken
/// stream out of this process's memory.
pub(crate) const MAX_RECORD_BYTES: usize = 16 * 1024 * 1024;

/// What one record is.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Framing {
    /// The whole input is one text document and one record.
    Document,
    /// Each line is one text record.
    Lines,
    /// Each line is one JSON value and one record.
    Jsonl,
    /// A comma-separated table with a header row.
    Csv,
    /// A tab-separated table with a header row.
    Tsv,
}

/// Why the framing and the pointers cannot act together.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub(crate) enum ReadingError {
    /// `--field` was given beside `--lines`.
    #[error("--field: a text line has no members, so --lines takes no pointer")]
    TextHasNoMembers,
    /// Two pointers end in one name, so one would hide the other.
    #[error("--field: two pointers end in `{0}`, and one evidence object holds each name once")]
    KeyClash(String),
    /// A question reads `on` under `--lines`, whose records have no members.
    #[error("question `{0}` reads `on`, and a --lines record is text with no members")]
    LinesPart(String),
}

/// Why one record could not become the evidence of one request.
///
/// No variant carries any part of a record, because a record is evidence. A
/// pointer is named, because the user typed it on the command line.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub(crate) enum RecordError {
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
    /// The selected entity member is not text.
    #[error("the entity value at `{0}` is not a string")]
    EntityText(String),
    /// A complete structured entity document is not one list.
    #[error("the entity document is one JSON array")]
    EntityDocument,
    /// A pointer was taken into a text record, which has no members.
    #[error("{}", ReadingError::TextHasNoMembers)]
    TextHasNoMembers,
    /// A question reads `on` in a record whose selection is text.
    #[error("question `{0}` reads `on`, and this record's evidence is text with no members")]
    TextPart(String),
    /// The evidence the record yields is blank.
    #[error("{0}")]
    Blank(#[from] BlankTextError),
    /// A scalar was offered where structured evidence takes an object or a list.
    #[error("{0}")]
    EvidenceShape(#[from] EvidenceShapeError),
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
#[derive(Clone, PartialEq, Serialize)]
#[serde(transparent)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(inline, with = "Json"))]
pub(crate) struct Record(Held);

/// A record is evidence, so `Debug` withholds its bytes and keeps its kind
/// and its length. A JSON record's length is the length of its JSON line.
impl std::fmt::Debug for Record {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (kind, length) = match &self.0 {
            Held::Text(text) => ("text", text.len()),
            Held::Json(value) => ("json", json_line(value).map_or(0, |line| line.len())),
        };
        formatter
            .debug_tuple("Record")
            .field(&format_args!("{kind}"))
            .field(&Withheld(length))
            .finish()
    }
}

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
    /// Build one object record from table headers and their string cells.
    #[must_use]
    pub(crate) fn string_fields(fields: Vec<(String, String)>) -> Self {
        Self(Held::Json(Json::Object(
            fields
                .into_iter()
                .map(|(name, value)| (name, Json::String(value)))
                .collect(),
        )))
    }

    /// True when this record is an object holding the given member.
    #[must_use]
    pub(crate) fn has_member(&self, name: &str) -> bool {
        matches!(&self.0, Held::Json(Json::Object(members)) if members.iter().any(|(held, _)| held == name))
    }

    /// True when this record is a JSON object that annotations can enrich.
    #[must_use]
    pub(crate) const fn is_object(&self) -> bool {
        matches!(&self.0, Held::Json(Json::Object(_)))
    }

    /// Add named answers to an object record, or return the answers alone.
    #[must_use]
    pub(crate) fn annotated(
        self,
        answers: Vec<(String, crate::core::AnnotatedValue)>,
    ) -> AnnotatedRecord {
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
    pub(crate) fn choices(&self, pointer: &Pointer) -> Result<Labels, RecordError> {
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
                    Json::Null => Ok((name.clone(), None)),
                    _ => Description::of_json(held)
                        .map(|description| (name.clone(), Some(description)))
                        .ok_or_else(shape),
                })
                .collect::<Result<Vec<_>, RecordError>>()?,
            _ => return Err(shape()),
        };
        Ok(Labels::described(listed)?)
    }

    /// Read one required entity string without exposing the rest of the record.
    pub(crate) fn entity_text(&self, pointer: &Pointer) -> Result<&str, RecordError> {
        let Held::Json(value) = &self.0 else {
            return Err(RecordError::TextHasNoMembers);
        };
        found(pointer, value)?
            .as_str()
            .ok_or_else(|| RecordError::EntityText(pointer.as_str().to_owned()))
    }
}

/// One record with its named bare answers appended.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AnnotatedRecord {
    record: Record,
    answers: Vec<(String, crate::core::AnnotatedValue)>,
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
pub(crate) struct Reading {
    framing: Framing,
    fields: Vec<Pointer>,
    by_default: bool,
}

/// The `input` field a plan carries in record mode.
#[derive(Debug, Serialize)]
pub(crate) struct ReadingPlan<'a> {
    framing: Framing,
    field: &'a [Pointer],
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<&'static str>,
}

impl Reading {
    /// Take the framing and the pointers, or say why they cannot act together.
    ///
    /// # Errors
    ///
    /// Returns [`ReadingError`] when a pointer is given beside `--lines`, and
    /// when two pointers end in one name.
    pub(crate) fn new(framing: Framing, fields: Vec<Pointer>) -> Result<Self, ReadingError> {
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
        Ok(Self {
            framing,
            fields,
            by_default: false,
        })
    }

    /// Mark the framing as the default's choice, which the plan names.
    #[must_use]
    pub(crate) const fn by_default(mut self) -> Self {
        self.by_default = true;
        self
    }

    /// True when the input is a stream of records rather than one document.
    #[must_use]
    pub(crate) fn streams(&self) -> bool {
        self.framing != Framing::Document
    }

    /// A blank text line has a place in the input, but no record to judge.
    pub(crate) fn skips(&self, bytes: &[u8]) -> bool {
        self.framing == Framing::Lines
            && str::from_utf8(self.ended(bytes)).is_ok_and(|text| text.trim().is_empty())
    }

    /// Name the framing and the pointers, as the record-mode plan prints them.
    pub(crate) fn plan(&self) -> ReadingPlan<'_> {
        ReadingPlan {
            framing: self.framing,
            field: &self.fields,
            from: self.by_default.then_some("default"),
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
    pub(crate) fn as_it_arrived<'a>(&self, bytes: &'a [u8]) -> Result<&'a str, RecordError> {
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
    pub(crate) fn record(&self, bytes: &[u8]) -> Result<Record, RecordError> {
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

    /// Read one structured document as a complete ordered entity set.
    pub(crate) fn entity_document(&self, bytes: &[u8]) -> Result<Vec<Record>, RecordError> {
        let Record(Held::Json(Json::Array(items))) = self.record(bytes)? else {
            return Err(RecordError::EntityDocument);
        };
        Ok(items
            .into_iter()
            .map(|item| Record(Held::Json(item)))
            .collect())
    }

    /// Read an annotation record, preserving a whole JSON object when present.
    ///
    /// A document is normally free text. `annotate` also accepts one JSON
    /// object without requiring record framing because it can append named
    /// answers to that object. Other commands keep the established document
    /// behavior through [`Self::record`].
    pub(crate) fn annotation_record(&self, bytes: &[u8]) -> Result<Record, RecordError> {
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
    /// With no pointer the whole record stays the text it always was. A
    /// pointer selects, and the selected value decides the state's type: a
    /// string is its text, a number, `true`, `false`, or `null` is its compact
    /// spelling, and an object or a list travels as that JSON value.
    ///
    /// # Errors
    ///
    /// Returns [`RecordError`] when a pointer finds nothing, when the evidence
    /// is blank, and when it cannot be written as JSON.
    pub(crate) fn evidence(&self, record: &Record) -> Result<Evidence, RecordError> {
        self.selected(record)?.evidence()
    }

    /// Build the record a batch reads: today's evidence, and the value a batch
    /// quotes. A whole JSON record keeps its own value, not its compact text.
    pub(crate) fn batch_record(&self, record: &Record) -> Result<BatchRecord, RecordError> {
        let selected = self.selected(record)?;
        let value = match &selected {
            Selected::Text(text) => Json::String((*text).to_owned()),
            Selected::Whole(value) => (*value).clone(),
            Selected::Chosen(value) => value.clone(),
        };
        let evidence = selected.evidence()?;
        Ok(BatchRecord { evidence, value })
    }

    /// What this reading selects from one record.
    fn selected<'a>(&self, record: &'a Record) -> Result<Selected<'a>, RecordError> {
        match (&record.0, self.fields.as_slice()) {
            (Held::Text(text), []) => Ok(Selected::Text(text)),
            (Held::Text(_), _) => Err(RecordError::TextHasNoMembers),
            (Held::Json(value), _) => self.chosen(value),
        }
    }

    /// The evidence these pointers select inside one JSON value, by the
    /// `state` rule. `annotate` reads each `on` group's part this way.
    pub(crate) fn part(&self, value: &Json) -> Result<Evidence, RecordError> {
        self.chosen(value)?.evidence()
    }

    /// What this reading selects from one JSON value.
    fn chosen<'a>(&self, value: &'a Json) -> Result<Selected<'a>, RecordError> {
        Ok(match self.fields.as_slice() {
            [] => Selected::Whole(value),
            [pointer] => Selected::Chosen(found(pointer, value)?.clone()),
            pointers => Selected::Chosen(Json::Object(
                pointers
                    .iter()
                    .map(|pointer| Ok((pointer.key().to_owned(), found(pointer, value)?.clone())))
                    .collect::<Result<Vec<_>, RecordError>>()?,
            )),
        })
    }
}

/// What a reading selects: a whole text, a whole JSON record, or a selection.
enum Selected<'a> {
    Text(&'a str),
    Whole(&'a Json),
    Chosen(Json),
}

impl Selected<'_> {
    fn evidence(self) -> Result<Evidence, RecordError> {
        match self {
            Self::Text(text) => Ok(Evidence::new(text)?),
            Self::Whole(value) => whole(value),
            Self::Chosen(value @ (Json::Array(_) | Json::Object(_))) => {
                Ok(Evidence::structured(value)?)
            }
            Self::Chosen(Json::String(text)) => Ok(Evidence::new(text)?),
            Self::Chosen(scalar) => Ok(Evidence::new(json_line(&scalar)?)?),
        }
    }
}

/// The evidence an unpointed JSON record sends: a string is its text and every
/// other value is its compact spelling, as text.
fn whole(value: &Json) -> Result<Evidence, RecordError> {
    let text = match value.as_str() {
        Some(text) => text.to_owned(),
        None => json_line(value)?,
    };
    Ok(Evidence::new(text)?)
}

/// The value the pointer names, or the refusal that names the pointer.
fn found<'a>(pointer: &Pointer, value: &'a Json) -> Result<&'a Json, RecordError> {
    pointer
        .resolve(value)
        .ok_or_else(|| RecordError::Missed(pointer.as_str().to_owned()))
}

#[cfg(test)]
mod tests;
