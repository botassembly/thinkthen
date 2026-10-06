//! Owned native inputs and typed result/1 helpers; no new semantic pipeline.
mod execute;
mod question;
mod views;
use crate::failures::Failure;
use crate::ffi::current::carriers::{
    ContentV1, CurrentAttemptV1, CurrentQuestionV1, CurrentSummaryV1, OptionalStringV1, StringV1,
};
use serde::Serialize;
use serde_json::value::RawValue;
use std::fmt;
use thinkthen::{Evidence, ReaderOptions, SourceRecord};

/// Immutable question, independent of every constructor buffer/member handle.
#[derive(Clone)]
pub struct QuestionHandle {
    pub(crate) json: String,
    pub(crate) native: question::Native,
}
impl fmt::Debug for QuestionHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QuestionHandle").finish_non_exhaustive()
    }
}
pub(crate) use question::{load, parse, plain};

#[derive(Clone)]
pub(crate) enum Content {
    Text(String),
    Json(Box<RawValue>),
}
impl Content {
    pub(crate) fn text(&self) -> &str {
        match self {
            Self::Text(text) => text,
            Self::Json(json) => json.get(),
        }
    }
}
impl Serialize for Content {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Text(text) => text.serialize(s),
            Self::Json(json) => json.serialize(s),
        }
    }
}
#[derive(Clone)]
pub(crate) struct Input {
    pub(crate) original: Content,
    pub(crate) position: Option<SourceRecord<()>>,
    pub(crate) index: usize,
}
impl Serialize for Input {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.original.serialize(s)
    }
}
impl Evidence for Input {
    fn evidence(&self) -> &str {
        self.original.text()
    }
}
// Serialize only original evidence for the native detail/annotation pipeline.
// Its physical metadata remains in the adapter-owned position.

/// An immutable record snapshot or explicit native reader selection.
pub struct SourceHandle(pub(crate) Source);
pub(crate) enum Source {
    Records(Vec<Input>),
    Files(Vec<String>, ReaderOptions),
}
impl fmt::Debug for SourceHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceHandle").finish_non_exhaustive()
    }
}
pub(crate) type Inputs<'a> = Box<dyn Iterator<Item = Result<Input, thinkthen::Error>> + 'a>;
impl SourceHandle {
    pub(crate) fn read(&self) -> Result<Inputs<'_>, Failure> {
        match &self.0 {
            Source::Records(records) => Ok(Box::new(records.iter().cloned().map(Ok))),
            Source::Files(paths, options) => {
                let reader = thinkthen::read_files(paths, *options)?;
                Ok(Box::new(reader.enumerate().map(|(index, record)| {
                    let record = record?;
                    Ok(Input {
                        original: Content::Text(record.record),
                        position: Some(SourceRecord {
                            record: (),
                            file: record.file,
                            first_line: record.first_line,
                            last_line: record.last_line,
                        }),
                        index,
                    })
                })))
            }
        }
    }
}

/// Owned view backing allocations. Moving the owner cannot move their bytes.
#[derive(Default)]
pub(crate) struct Storage(pub(crate) Vec<Box<dyn std::any::Any>>);
impl Storage {
    pub(crate) fn array<T: 'static>(&mut self, values: Vec<T>) -> (*const T, usize) {
        if values.is_empty() {
            return (std::ptr::null(), 0);
        }
        let values = values.into_boxed_slice();
        let pair = (values.as_ptr(), values.len());
        self.0.push(Box::new(values));
        pair
    }
    pub(crate) fn string(&mut self, value: &str) -> StringV1 {
        let (data, len) = self.array(value.as_bytes().to_vec());
        StringV1 {
            data: data.cast(),
            len,
        }
    }
    pub(crate) fn optional_string(&mut self, value: Option<&str>) -> OptionalStringV1 {
        value.map_or_else(OptionalStringV1::default, |value| OptionalStringV1 {
            present: 1,
            value: self.string(value),
        })
    }
    pub(crate) fn content(&mut self, value: &Content) -> ContentV1 {
        ContentV1 {
            kind: match value {
                Content::Text(_) => 1,
                Content::Json(_) => 2,
            },
            data: self.string(value.text()),
        }
    }
}
/// Result view memory belongs entirely to this owner, never to the engine.
pub struct ResultHandle {
    pub(crate) _storage: Storage,
    pub(crate) summary: CurrentSummaryV1,
    pub(crate) rows: Vec<views::Row>,
    pub(crate) questions: Vec<CurrentQuestionV1>,
    pub(crate) attempts: Vec<CurrentAttemptV1>,
}
impl fmt::Debug for ResultHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResultHandle")
            .field("function", &self.summary.function)
            .field("rows", &self.rows.len())
            .finish_non_exhaustive()
    }
}
pub(crate) use execute::ask;
pub(crate) use views::Row;
