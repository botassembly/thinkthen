//! Framing a stream into records, and building the evidence one record sends.
//!
//! Everything here is a pure function over bytes the binary read. The binary
//! opens the file, reads standard input, and writes the rows. This module
//! decides what one record is and what part of it leaves the machine.

use serde::Serialize;
use thiserror::Error;

use crate::json::{Json, JsonError};
use crate::pointer::Pointer;
use crate::render::{RenderError, json_line};
use crate::text::{BlankTextError, Evidence};

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
    /// The record is not JSON this tool will read.
    #[error("{0}")]
    Json(#[from] JsonError),
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

    /// Read one record from the bytes the binary handed over.
    ///
    /// A line arrives with the line feed that ended it, and a carriage return
    /// before that line feed is stripped with it.
    ///
    /// # Errors
    ///
    /// Returns [`RecordError`] when the bytes are not text, and when a JSON
    /// record is not JSON this tool will read.
    pub fn record(&self, bytes: &[u8]) -> Result<Record, RecordError> {
        let text = str::from_utf8(bytes).map_err(|_| RecordError::NotUtf8)?;
        let text = if self.streams() {
            text.strip_suffix('\n')
                .map_or(text, |line| line.strip_suffix('\r').unwrap_or(line))
        } else {
            text
        };
        if self.framing == Framing::Jsonl || !self.fields.is_empty() {
            return Ok(Record(Held::Json(Json::parse(text)?)));
        }
        Ok(Record(Held::Text(text.to_owned())))
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
mod tests {
    use super::{Framing, Held, Reading, ReadingError, Record, RecordError};
    use crate::json::JsonError;
    use crate::pointer::Pointer;
    use crate::render::json_line;
    use crate::text::BlankTextError;
    use proptest::collection::vec;
    use proptest::prelude::{Strategy, any};
    use proptest::{prop_assert_eq, proptest};

    fn reading(framing: Framing, fields: &[&str]) -> Reading {
        let pointers = fields
            .iter()
            .map(|text| Pointer::new(*text).expect("a pointer"))
            .collect();
        Reading::new(framing, pointers).expect("a framing and its pointers")
    }

    /// The evidence one record sends under one reading.
    fn sent(reading: &Reading, bytes: &[u8]) -> Result<String, RecordError> {
        let record = reading.record(bytes)?;
        reading
            .evidence(&record)
            .map(|evidence| evidence.as_str().to_owned())
    }

    #[test]
    fn each_framing_says_what_one_record_is() {
        let document = reading(Framing::Document, &[]);
        assert_eq!(
            sent(&document, b"two\nlines\n").as_deref(),
            Ok("two\nlines\n")
        );
        let lines = reading(Framing::Lines, &[]);
        assert_eq!(sent(&lines, b"one line\n").as_deref(), Ok("one line"));
        assert_eq!(sent(&lines, b"no line feed").as_deref(), Ok("no line feed"));
        let jsonl = reading(Framing::Jsonl, &[]);
        assert_eq!(
            sent(&jsonl, br#"{"id":"T-91","body":"Payouts failed."}"#).as_deref(),
            Ok(r#"{"id":"T-91","body":"Payouts failed."}"#)
        );
    }

    #[test]
    fn a_carriage_return_before_the_line_feed_is_stripped() {
        let lines = reading(Framing::Lines, &[]);
        assert_eq!(sent(&lines, b"one line\r\n").as_deref(), Ok("one line"));
        assert_eq!(
            sent(&lines, b"kept\rinside\n").as_deref(),
            Ok("kept\rinside")
        );
        let jsonl = reading(Framing::Jsonl, &["/a"]);
        assert_eq!(sent(&jsonl, b"{\"a\":\"x\"}\r\n").as_deref(), Ok("x"));
    }

    #[test]
    fn one_pointer_sends_the_value_it_names_and_several_send_an_object() {
        let one = reading(Framing::Jsonl, &["/body"]);
        let line = br#"{"id":"T-91","body":"Payouts failed.","count":3}"#;
        assert_eq!(sent(&one, line).as_deref(), Ok("Payouts failed."));
        let number = reading(Framing::Jsonl, &["/count"]);
        assert_eq!(sent(&number, line).as_deref(), Ok("3"));
        let several = reading(Framing::Jsonl, &["/body", "/id"]);
        assert_eq!(
            sent(&several, line).as_deref(),
            Ok(r#"{"body":"Payouts failed.","id":"T-91"}"#)
        );
    }

    #[test]
    fn a_pointer_without_jsonl_reads_the_whole_input_as_one_json_value() {
        let document = reading(Framing::Document, &["/a/text"]);
        assert_eq!(
            sent(&document, b"{\n  \"a\": {\"text\": \"inner\"}\n}\n").as_deref(),
            Ok("inner")
        );
    }

    #[test]
    fn the_framing_and_the_pointers_are_refused_when_they_cannot_act_together() {
        let pointer = |text: &str| Pointer::new(text).expect("a pointer");
        assert_eq!(
            Reading::new(Framing::Lines, vec![pointer("/body")]),
            Err(ReadingError::TextHasNoMembers)
        );
        assert_eq!(
            Reading::new(Framing::Jsonl, vec![pointer("/a/text"), pointer("/b/text")]),
            Err(ReadingError::KeyClash("text".to_owned()))
        );
        assert!(Reading::new(Framing::Jsonl, vec![pointer("/a"), pointer("/b")]).is_ok());
    }

    #[test]
    fn a_record_the_tool_refuses_names_no_part_of_itself() {
        let jsonl = reading(Framing::Jsonl, &["/body"]);
        let cases = [
            (
                &b"{\"id\":1,\"id\":2}"[..],
                RecordError::Json(JsonError::DuplicateName),
            ),
            (
                &b"{\"body\":1e999}"[..],
                RecordError::Json(JsonError::NotFinite),
            ),
            (&b"not json"[..], RecordError::Json(JsonError::Syntax)),
            (&b"\xff\xfe"[..], RecordError::NotUtf8),
            (
                &b"{\"other\":\"x\"}"[..],
                RecordError::Missed("/body".to_owned()),
            ),
            (
                &b"{\"body\":\"  \"}"[..],
                RecordError::Blank(BlankTextError::Evidence),
            ),
        ];
        for (bytes, expected) in cases {
            let refused = sent(&jsonl, bytes).expect_err("a refused record");
            assert_eq!(refused, expected);
            let said = refused.to_string();
            assert!(!said.contains("Payouts"), "{said}");
            assert!(!said.contains("other"), "{said}");
        }
    }

    #[test]
    fn a_record_mode_plan_names_the_framing_and_the_pointers() {
        let jsonl = reading(Framing::Jsonl, &["/body", "/id"]);
        assert_eq!(
            json_line(&jsonl.plan()).expect("a plan is writable"),
            r#"{"framing":"jsonl","field":["/body","/id"]}"#
        );
        let lines = reading(Framing::Lines, &[]);
        assert_eq!(
            json_line(&lines.plan()).expect("a plan is writable"),
            r#"{"framing":"lines","field":[]}"#
        );
    }

    #[test]
    fn a_record_is_written_back_as_it_arrived() {
        let jsonl = reading(Framing::Jsonl, &["/body"]);
        let record = jsonl
            .record(br#"{"id":"T-91","body":"Payouts failed."}"#)
            .expect("a record");
        assert_eq!(
            json_line(&record).expect("a record is writable"),
            r#"{"id":"T-91","body":"Payouts failed."}"#
        );
        let lines = reading(Framing::Lines, &[]);
        let record = lines.record(b"one line\n").expect("a record");
        assert_eq!(record, Record(Held::Text("one line".to_owned())));
        assert_eq!(
            json_line(&record).expect("a record is writable"),
            r#""one line""#
        );
    }

    fn texts() -> impl Strategy<Value = String> {
        vec(any::<char>(), 1..24)
            .prop_map(|characters| characters.into_iter().collect::<String>())
            .prop_filter("a line that is one line and is not blank", |text| {
                !text.trim().is_empty() && !text.contains(['\n', '\r'])
            })
    }

    proptest! {
        /// Any text line reaches the evidence unchanged, and any JSON record
        /// hands its pointed member over as the string it holds.
        #[test]
        fn framing_a_line_keeps_the_line(text in texts()) {
            let lines = reading(Framing::Lines, &[]);
            let framed = sent(&lines, format!("{text}\n").as_bytes());
            prop_assert_eq!(framed.as_deref(), Ok(text.as_str()));
            let jsonl = reading(Framing::Jsonl, &["/body"]);
            let line = json_line(&super::Json::String(text.clone())).expect("a string is writable");
            let pointed = sent(&jsonl, format!("{{\"body\":{line}}}\n").as_bytes());
            prop_assert_eq!(pointed.as_deref(), Ok(text.as_str()));
        }
    }
}
