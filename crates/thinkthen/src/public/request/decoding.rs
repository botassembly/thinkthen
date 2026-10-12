//! Typed admission causes survive serde without interpreting its diagnostic text.
use super::{Error, RequestInput, RequestItem, RequestQuestion};
use crate::core::Json;

/// The safe reason canonical request decoding refused a value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestDecodeCause {
    /// The canonical grammar is malformed.
    Malformed,
    /// A per-item context has the wrong type.
    PerItemContext,
    /// An authored declaration uses an unsupported feature.
    UnsupportedDeclaration,
}
impl RequestDecodeCause {
    pub(super) fn error(self) -> Error {
        let message = match self {
            Self::Malformed => "invalid canonical request",
            Self::PerItemContext => "the per-item context does not match context_schema",
            Self::UnsupportedDeclaration => "the question declaration uses an unsupported feature",
        };
        Error::usage(message).with_diagnostic(
            crate::public::error::diagnostic::Diagnostic::RequestDecode(self),
        )
    }
}
fn member<'a>(value: &'a Json, name: &str) -> Option<&'a Json> {
    let Json::Object(fields) = value else {
        return None;
    };
    fields
        .iter()
        .find(|(key, _)| key == name)
        .map(|(_, value)| value)
}
fn context(value: &Json) -> Result<(), Error> {
    if member(value, "context")
        .is_some_and(|value| !matches!(value, Json::String(_) | Json::Object(_)))
    {
        return Err(RequestDecodeCause::PerItemContext.error());
    }
    Ok(())
}
pub(super) fn validate_request_contexts(text: &str) -> Result<(), Error> {
    let Ok(value) = Json::parse(text) else {
        return Ok(());
    };
    let Some(input) = member(&value, "call").and_then(|call| member(call, "input")) else {
        return Ok(());
    };
    input_contexts(input)
}
fn input_contexts(input: &Json) -> Result<(), Error> {
    if let Some(Json::Array(items)) = member(input, "items") {
        for item in items {
            context(item)?;
        }
    }
    Ok(())
}
impl RequestInput {
    /// Decode an input selector with typed per-item context diagnostics.
    /// # Errors
    /// Refuses malformed selectors and incompatible per-item context values.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        serde_json::from_str(text).map_err(|_| {
            Json::parse(text)
                .ok()
                .and_then(|value| input_contexts(&value).err())
                .unwrap_or_else(|| RequestDecodeCause::Malformed.error())
        })
    }
}
impl RequestItem {
    /// Decode one descriptor while retaining typed native admission diagnostics.
    /// # Errors
    /// Refuses malformed descriptors and incompatible per-item context values.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        serde_json::from_str(text).map_err(|_| {
            Json::parse(text)
                .ok()
                .and_then(|value| context(&value).err())
                .unwrap_or_else(|| RequestDecodeCause::Malformed.error())
        })
    }
}
impl RequestQuestion {
    /// Decode a selector while retaining authored definition admission diagnostics.
    /// # Errors
    /// Refuses malformed selectors and unsupported declarations.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        serde_json::from_str(text).map_err(|_| {
            super::definition::validate_question_definition(text)
                .err()
                .unwrap_or_else(|| RequestDecodeCause::Malformed.error())
        })
    }
}
