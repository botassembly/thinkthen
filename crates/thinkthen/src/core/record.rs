//! Records: the evidence a question judges, and the id its row carries.

use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use serde::ser::{Serialize, Serializer};
use serde_json::value::RawValue;

use crate::core::error::{Error, ErrorKind};

/// A JSON Pointer, as RFC 6901 defines it. It names the evidence inside each record.
///
/// The empty pointer, the default, names the whole record.
#[derive(Debug, Clone, Default)]
pub struct Pointer {
    text: String,
    tokens: Vec<String>,
}

impl FromStr for Pointer {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Error> {
        let refused = || {
            Error::new(
                ErrorKind::Pointer,
                format!(
                    "the pointer `{text}` is not a JSON Pointer as RFC 6901 defines it, such as /body"
                ),
            )
        };
        let tokens = match text.strip_prefix('/') {
            None if text.is_empty() => Vec::new(),
            None => return Err(refused()),
            Some(rest) => rest
                .split('/')
                .map(|token| unescape(token).ok_or_else(refused))
                .collect::<Result<_, _>>()?,
        };
        Ok(Self {
            text: text.to_owned(),
            tokens,
        })
    }
}

impl fmt::Display for Pointer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.text)
    }
}

/// `token` with `~1` read as `/` and `~0` as `~`, or `None` when another `~` appears.
fn unescape(token: &str) -> Option<String> {
    let mut plain = String::with_capacity(token.len());
    let mut chars = token.chars();
    while let Some(next) = chars.next() {
        if next == '~' {
            match chars.next() {
                Some('0') => plain.push('~'),
                Some('1') => plain.push('/'),
                _ => return None,
            }
        } else {
            plain.push(next);
        }
    }
    Some(plain)
}

/// The member or item `token` names inside `value`, if any.
fn step<'a>(value: &'a RawValue, token: &str) -> Option<&'a RawValue> {
    if let Ok(members) = serde_json::from_str::<BTreeMap<String, &'a RawValue>>(value.get()) {
        return members.get(token).copied();
    }
    let items = serde_json::from_str::<Vec<&'a RawValue>>(value.get()).ok()?;
    let canonical = token == "0" || !token.starts_with('0');
    if !canonical || !token.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    items.get(token.parse::<usize>().ok()?).copied()
}

/// The text of a JSON string, or the JSON spelling of any other value.
fn text_of(value: &RawValue) -> String {
    serde_json::from_str::<String>(value.get()).unwrap_or_else(|_| value.get().to_owned())
}

/// The id a row carries: the record's own `id`, or its position from 1.
#[derive(Debug, Clone)]
pub(crate) struct Id(IdKind);

#[derive(Debug, Clone)]
enum IdKind {
    Position(usize),
    Given(Box<RawValue>),
}

impl schemars::JsonSchema for Id {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Id".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        true.into()
    }
}

impl Id {
    pub(crate) const fn position(position: usize) -> Self {
        Self(IdKind::Position(position))
    }
}

impl Serialize for Id {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.0 {
            IdKind::Position(position) => position.serialize(serializer),
            IdKind::Given(id) => id.serialize(serializer),
        }
    }
}

/// One record: the evidence its question judges, and its id when it names one.
#[derive(Debug, Clone)]
pub struct Record {
    pub(crate) id: Option<Id>,
    pub(crate) evidence: Result<String, Error>,
    structured: Option<Box<RawValue>>,
}

impl Record {
    /// A record whose evidence is `text`.
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            id: None,
            evidence: Ok(text.into()),
            structured: None,
        }
    }

    /// A record read from the JSON `line`. Its `id` member, unless missing or
    /// null, becomes the row's id, and `on` names its evidence. A string is judged as its text and
    /// any other value as its JSON spelling.
    ///
    /// A line that is not JSON fails with `usage.record`, and a pointer that finds
    /// nothing fails with `usage.missing`. Either fails only this record's row.
    #[must_use]
    pub fn json(line: &str, on: &Pointer) -> Self {
        let Ok(record) = serde_json::from_str::<&RawValue>(line) else {
            return Self {
                id: None,
                evidence: Err(Error::new(ErrorKind::Record, "the record is not JSON")),
                structured: None,
            };
        };
        let selected = select(record, on);
        let evidence = selected
            .map(|value| {
                if on.tokens.is_empty()
                    && let Ok(mut members) =
                        serde_json::from_str::<BTreeMap<String, Box<RawValue>>>(value.get())
                {
                    members.remove("id");
                    return serde_json::to_string(&members)
                        .map_err(|_| Error::new(ErrorKind::Record, "the record is not JSON"));
                }
                Ok(text_of(value))
            })
            .unwrap_or_else(|| Err(missing(on)));
        let structured = selected
            .filter(|value| value.get().starts_with(['{', '[']))
            .map(RawValue::to_owned);
        Self {
            id: step(record, "id")
                .filter(|id| id.get() != "null")
                .map(|id| Id(IdKind::Given(id.to_owned()))),
            evidence,
            structured,
        }
    }

    /// Read a pointer within this record's selected evidence. Text stays text.
    ///
    /// # Errors
    /// Returns the record's admission error, or `usage.missing` for an absent member.
    pub fn evidence_at(&self, on: &Pointer) -> Result<String, Error> {
        let text = self.evidence.as_ref().map_err(Clone::clone)?;
        if on.tokens.is_empty() {
            return Ok(text.clone());
        }
        self.structured
            .as_deref()
            .and_then(|record| select(record, on))
            .map(text_of)
            .ok_or_else(|| missing(on))
    }
}

fn missing(on: &Pointer) -> Error {
    Error::new(
        ErrorKind::Missing,
        format!("the record holds nothing at {on}"),
    )
}

fn select<'a>(record: &'a RawValue, on: &Pointer) -> Option<&'a RawValue> {
    on.tokens
        .iter()
        .try_fold(record, |value, token| step(value, token))
}

/// A record whose selected evidence passed admission.
#[derive(Debug, Clone)]
pub struct AdmittedRecord(Record);

impl AdmittedRecord {
    /// Admit a record before preparing any question about it.
    ///
    /// # Errors
    /// Returns the record's parse or evidence-selection error.
    pub fn new(record: Record) -> Result<Self, Error> {
        record.evidence.as_ref().map_err(Clone::clone)?;
        Ok(Self(record))
    }

    /// The admitted evidence.
    #[must_use]
    #[expect(
        clippy::expect_used,
        reason = "admission rejects every record without evidence"
    )]
    pub fn evidence(&self) -> &str {
        self.0.evidence.as_deref().expect("admitted evidence")
    }

    /// The original record, including its selected JSON type.
    #[must_use]
    pub const fn record(&self) -> &Record {
        &self.0
    }

    pub(crate) fn text(text: impl Into<String>) -> Self {
        Self(Record::text(text))
    }
}
