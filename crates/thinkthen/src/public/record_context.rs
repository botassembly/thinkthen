//! Explicit typed per-item context stays separate from selected evidence.
use super::{Error, InputDeclaration, QuestionContent, RawRecord};
use crate::core::{self, Json};
use serde::{Serialize, Serializer};
use std::fmt;

/// An ordered JSON object supplied explicitly as per-item context.
#[derive(Clone, Eq, PartialEq)]
pub struct ObjectContext(Json);
impl ObjectContext {
    /// Retain an already parsed original object without parsing text as JSON.
    /// # Errors
    /// Returns Usage unless the original is a parsed object.
    pub fn new(original: &RawRecord) -> Result<Self, Error> {
        original
            .0
            .json()
            .map_or_else(|| Err(context_error()), |value| Self::of(value.clone()))
    }
    fn of(value: Json) -> Result<Self, Error> {
        if matches!(value, Json::Object(_)) {
            Ok(Self(value))
        } else {
            Err(context_error())
        }
    }
    /// Actual object members in authored order, including extra properties.
    #[must_use]
    pub const fn content(&self) -> QuestionContent<'_> {
        QuestionContent(&self.0)
    }
}
impl fmt::Debug for ObjectContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ObjectContext(<withheld>)")
    }
}
impl Serialize for ObjectContext {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

/// Explicit text or a typed object; absence belongs to `RecordInput`.
/// Empty text suppresses shared fallback. No text is interpreted as JSON.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RecordContext {
    /// Exact text, including an explicitly supplied empty string.
    Text(String),
    /// Actual ordered object, admitted only with an object context declaration.
    Object(ObjectContext),
}
impl fmt::Debug for RecordContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RecordContext(<withheld>)")
    }
}
impl From<String> for RecordContext {
    fn from(text: String) -> Self {
        Self::Text(text)
    }
}
impl From<&str> for RecordContext {
    fn from(text: &str) -> Self {
        Self::Text(text.to_owned())
    }
}
impl RecordContext {
    pub(crate) fn value(&self) -> Json {
        match self {
            Self::Text(text) => Json::String(text.clone()),
            Self::Object(object) => object.0.clone(),
        }
    }
    pub(crate) fn validate(&self, schema: Option<&InputDeclaration>) -> Result<(), Error> {
        let valid = schema.map_or_else(
            || matches!(self, Self::Text(_)),
            |schema| schema.accepts(&self.value()),
        );
        if valid { Ok(()) } else { Err(context_error()) }
    }
    pub(crate) fn selected(value: &Json, schema: Option<&InputDeclaration>) -> Result<Self, Error> {
        let context = match value {
            Json::String(text) => Self::Text(text.clone()),
            Json::Object(_) => Self::Object(ObjectContext::of(value.clone())?),
            _ => return Err(context_error()),
        };
        context.validate(schema)?;
        Ok(context)
    }
    pub(crate) fn resolved(
        context: Option<&Self>,
        fallback: Option<&str>,
    ) -> Result<Option<core::Evidence>, Error> {
        match context {
            Some(Self::Object(object)) => core::Evidence::structured(object.0.clone())
                .map(Some)
                .map_err(Error::refused),
            Some(Self::Text(text)) => text_evidence(Some(text)),
            None => text_evidence(fallback),
        }
    }
}
pub(crate) fn digest(context: &core::Evidence) -> Result<String, Error> {
    let text = context.as_text().map_err(Error::refused)?;
    Ok(core::bytes_sha256(text.as_bytes()))
}
fn text_evidence(text: Option<&str>) -> Result<Option<core::Evidence>, Error> {
    text.filter(|text| !text.is_empty())
        .map(super::engine::evidence)
        .transpose()
}
fn context_error() -> Error {
    Error::usage("the per-item context does not match context_schema")
}
