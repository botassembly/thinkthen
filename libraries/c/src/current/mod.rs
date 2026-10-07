//! Owned typed inputs and private native conversion storage.
#![allow(
    dead_code,
    reason = "released projection regressions retain private descriptors and view helpers"
)]
pub(crate) mod author;
pub(crate) mod descriptors;
#[cfg(test)]
mod execute;
pub(crate) mod question;
mod values;
pub(crate) use descriptors::QuestionData;
#[cfg(test)]
mod views;
use crate::failures::Failure;
use crate::ffi::current::carriers::{ContentV1, OptionalStringV1, StringV1};
use serde::Serialize;
use serde_json::value::RawValue;
use std::fmt;
use thinkthen::{Evidence, InputReaderOptions};

/// Immutable question, independent of every constructor buffer/member handle.
#[derive(Clone)]
pub struct QuestionHandle {
    pub(crate) json: String,
    pub(crate) author: Box<author::AuthorOwner>,
    pub(crate) native: question::Native,
    pub(crate) descriptor: Option<Box<QuestionData>>,
    pub(crate) reading: Option<thinkthen::RecordReading>,
}
impl fmt::Debug for QuestionHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QuestionHandle").finish_non_exhaustive()
    }
}
pub(crate) use question::{load, parse};

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
/// Validated immutable image plus caller-authored filename, never guessed media.
#[derive(Clone)]
pub struct ImageHandle {
    pub(crate) native: thinkthen::ImageInput,
    pub(crate) filename: Option<String>,
}
impl fmt::Debug for ImageHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageHandle")
            .field("media", &self.native.media())
            .field("width", &self.native.width())
            .field("height", &self.native.height())
            .finish_non_exhaustive()
    }
}
impl ImageHandle {
    pub(crate) fn view(&self) -> crate::ffi::carriers::ImageViewV1 {
        crate::ffi::carriers::ImageViewV1 {
            media: match self.native.media() {
                thinkthen::ImageMedia::Jpeg => 1,
                thinkthen::ImageMedia::Png => 2,
            },
            bytes: self.native.bytes().as_ptr(),
            bytes_len: self.native.bytes().len(),
            width: self.native.width(),
            height: self.native.height(),
            filename: self
                .filename
                .as_ref()
                .map_or_else(OptionalStringV1::default, |name| OptionalStringV1 {
                    present: 1,
                    value: StringV1 {
                        data: if name.is_empty() {
                            std::ptr::null()
                        } else {
                            name.as_ptr().cast()
                        },
                        len: name.len(),
                    },
                }),
        }
    }
}
#[derive(Clone)]
pub(crate) struct Choice {
    pub(crate) name: String,
    pub(crate) description: Option<Content>,
    pub(crate) weight: Option<f64>,
}
#[derive(Clone)]
pub(crate) struct Position {
    pub(crate) file: String,
    pub(crate) first_line: Option<usize>,
    pub(crate) last_line: Option<usize>,
}
#[derive(Clone)]
pub(crate) struct Input {
    pub(crate) original: Option<Content>,
    pub(crate) context: Option<Content>,
    pub(crate) options: Vec<Choice>,
    pub(crate) images: Vec<ImageHandle>,
    pub(crate) position: Option<Position>,
    pub(crate) index: usize,
}
impl Serialize for Input {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.original.serialize(s)
    }
}
impl Evidence for Input {
    fn evidence(&self) -> &str {
        self.original.as_ref().map_or("", Content::text)
    }
}
impl Input {
    pub(crate) fn from_source(index: usize, item: thinkthen::SourceItem) -> Self {
        match item {
            thinkthen::SourceItem::Text(text) => Self {
                original: Some(Content::Text(text.record)),
                context: None,
                options: Vec::new(),
                images: Vec::new(),
                index,
                position: Some(Position {
                    file: text.file,
                    first_line: Some(text.first_line),
                    last_line: Some(text.last_line),
                }),
            },
            thinkthen::SourceItem::Image(image) => Self {
                original: None,
                context: None,
                options: Vec::new(),
                index,
                images: vec![ImageHandle {
                    native: image.record,
                    filename: Some(image.file.clone()),
                }],
                position: Some(Position {
                    file: image.file,
                    first_line: None,
                    last_line: None,
                }),
            },
        }
    }
    pub(crate) fn question_input(&self) -> Result<thinkthen::QuestionInput, Failure> {
        let text = self.original.as_ref().map(|v| v.text().to_owned());
        if self.images.is_empty() {
            return Ok(thinkthen::QuestionInput::Text(text.ok_or_else(|| {
                Failure::usage("text records require original content")
            })?));
        }
        Ok(thinkthen::QuestionInput::Images(
            thinkthen::ImageEvidence::new(
                text,
                self.images
                    .iter()
                    .map(|image| image.native.clone())
                    .collect(),
            )?,
        ))
    }
}
/// An immutable record snapshot or explicit shared native reader selection.
#[derive(Clone)]
pub struct SourceHandle(pub(crate) Source);
#[derive(Clone)]
pub(crate) enum Source {
    Records(Vec<Input>),
    Files(Vec<String>, InputReaderOptions),
    JsonLines(Vec<String>),
}
impl fmt::Debug for SourceHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceHandle").finish_non_exhaustive()
    }
}
pub(crate) type Inputs<'a> = Box<dyn Iterator<Item = Result<Input, thinkthen::Error>> + 'a>;
impl SourceHandle {
    pub(crate) fn read(&self) -> Result<Inputs<'_>, thinkthen::Error> {
        match &self.0 {
            Source::Records(records) => Ok(Box::new(records.iter().cloned().map(Ok))),
            Source::JsonLines(paths) => {
                let reader = thinkthen::read_files(
                    paths,
                    thinkthen::ReaderOptions {
                        unit: thinkthen::SourceUnit::Line,
                        window: None,
                    },
                )?;
                Ok(Box::new(
                    reader
                        .enumerate()
                        .map(|(index, record)| json_line(index, record?)),
                ))
            }
            Source::Files(paths, options) => {
                let reader = thinkthen::read_inputs(paths, *options)?;
                Ok(Box::new(reader.enumerate().map(|(index, record)| {
                    Ok(Input::from_source(index, record?))
                })))
            }
        }
    }
}

fn json_line(
    index: usize,
    record: thinkthen::SourceRecord<String>,
) -> Result<Input, thinkthen::Error> {
    thinkthen::RawRecord::json(&record.record)?;
    let raw = serde_json::from_str(&record.record).map_err(|_| {
        thinkthen::Error::new(
            thinkthen::ErrorKind::Defect,
            "native JSON record could not be retained",
        )
    })?;
    let mut input = Input::from_source(index, thinkthen::SourceItem::Text(record));
    input.original = Some(Content::Json(raw));
    Ok(input)
}

/// Owned view backing allocations. Moving the owner cannot move their bytes.
#[derive(Default)]
pub(crate) struct Storage(
    pub(crate) Vec<Box<dyn std::any::Any>>,
    pub(crate) Vec<crate::ffi::carriers::DetailsV1>,
    pub(crate) Vec<crate::ffi::carriers::RowV1>,
    pub(crate) Vec<crate::ffi::carriers::QuestionAuthorV1>,
    pub(crate) Vec<Vec<crate::ffi::carriers::QuestionAuthorV1>>,
    pub(crate) Vec<Vec<crate::ffi::carriers::RankViewV1>>,
    pub(crate) Vec<crate::ffi::carriers::SourceRecognitionV1>,
    pub(crate) Vec<crate::ffi::carriers::SourceRelationsV1>,
    pub(crate) Vec<Vec<crate::ffi::carriers::DetailsV1>>,
);
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
#[cfg(test)]
/// Result view memory belongs entirely to this owner, never to the engine.
pub(crate) struct ResultHandle {
    pub(crate) _storage: Storage,
    pub(crate) summary: private::CurrentSummaryV1,
    pub(crate) rows: Vec<views::Row>,
    pub(crate) questions: Vec<private::CurrentQuestionV1>,
    pub(crate) attempts: Vec<private::CurrentAttemptV1>,
}
#[cfg(test)]
impl fmt::Debug for ResultHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResultHandle")
            .field("function", &self.summary.function)
            .field("rows", &self.rows.len())
            .finish_non_exhaustive()
    }
}
#[cfg(test)]
pub(crate) use execute::ask;
#[cfg(test)]
pub(crate) use views::Row;

#[cfg(test)]
#[path = "../ffi/current/carriers.rs"]
pub(crate) mod private;
