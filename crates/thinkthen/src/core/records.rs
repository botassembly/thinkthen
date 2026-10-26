//! Owned records admit identifiers and missing values before execution.
#![allow(
    clippy::disallowed_types,
    reason = "schema derives construct metadata; records use raw JSON"
)]
use super::{Code, Error, Pointer};
use serde_json::value::RawValue;
use std::collections::BTreeMap;

super::abi::discriminators! {
 /// Native input kinds. Missing and conversion failure never reach a backend.
 #[non_exhaustive]
 #[derive(Debug,Clone,Copy,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
 #[serde(rename_all="lowercase")]
 pub enum RecordKind {
  /// Literal UTF-8 text.
  Text=0,
  /// Structured application JSON.
  Json=1,
  /// Host None, NA or NULL.
  Missing=2,
  /// Host conversion failed without lossless representation.
  Invalid=3,
 }
}
super::abi::discriminators! {
 /// Exact identifier representation at native typed boundaries.
 #[non_exhaustive]
 #[derive(Debug,Clone,Copy,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
 #[serde(rename_all="lowercase")]
 pub enum IdKind {
  /// The input supplied no admitted identifier.
  Absent=0,
  /// A JSON string, including numeric-looking text.
  String=1,
  /// An integer represented in the signed range.
  Signed=2,
  /// An integer represented in the unsigned range.
  Unsigned=3,
 }
}
/// A record's exact caller-supplied identity.
#[non_exhaustive]
#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(untagged)]
pub enum RecordId {
    /// A signed JSON integer.
    Integer(i64),
    /// An unsigned JSON integer beyond the signed range.
    Unsigned(u64),
    /// A string, including numeric-looking strings.
    String(String),
}
impl RecordId {
    /// Admit an explicit decimal integer without a floating-point intermediate.
    /// # Errors
    /// Rejects fractions, exponents, whitespace and integers outside both ranges.
    pub fn integer_decimal(text: &str) -> Result<Self, Error> {
        let digits = text.strip_prefix('-').unwrap_or(text);
        if !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()) {
            if let Ok(value) = text.parse::<i64>() {
                return Ok(Self::Integer(value));
            }
            if let Ok(value) = text.parse::<u64>() {
                return Ok(Self::Unsigned(value));
            }
        }
        Err(Error::from_contract(
            Code::UsageQuestion,
            crate::core::diagnostic::requirement(
                "id",
                "decimal integer text in the signed or unsigned 64-bit range",
                "invalid integer text (contents redacted)",
            ),
        ))
    }
    /// Read its representation without interpreting a string as an integer.
    #[must_use]
    pub const fn kind(&self) -> IdKind {
        match self {
            Self::Integer(_) => IdKind::Signed,
            Self::Unsigned(_) => IdKind::Unsigned,
            Self::String(_) => IdKind::String,
        }
    }

    /// Read a string identity without interpreting its text.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }
    /// Read an integer when it fits the signed range.
    #[must_use]
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(*value),
            Self::Unsigned(value) => i64::try_from(*value).ok(),
            Self::String(_) => None,
        }
    }
    /// Read a nonnegative integer without losing precision.
    #[must_use]
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Unsigned(value) => Some(*value),
            Self::Integer(value) => u64::try_from(*value).ok(),
            Self::String(_) => None,
        }
    }
}
impl From<String> for RecordId {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for RecordId {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}
impl From<i64> for RecordId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl From<u64> for RecordId {
    fn from(value: u64) -> Self {
        Self::Unsigned(value)
    }
}

#[derive(Debug, Clone)]
enum Input {
    Text(String),
    Json(Box<RawValue>),
    Failed(Error),
}
/// One owned input, including host missing and conversion failures.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Record {
    input: Input,
    id: Option<RecordId>,
}
impl Record {
    /// Retain literal text without interpreting it as JSON.
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            input: Input::Text(text.into()),
            id: None,
        }
    }
    /// Represent a host's None, NA or NULL without a backend call.
    #[must_use]
    pub fn missing() -> Self {
        Self::failed(Code::RecordMissing, "the record is missing")
    }
    /// Represent a host value that cannot be converted without loss.
    #[must_use]
    pub fn invalid() -> Self {
        Self::failed(
            Code::RecordInvalid,
            "the record cannot be converted without loss",
        )
    }
    /// Admit a JSON record, retaining integer identifiers exactly.
    #[must_use]
    pub fn from_json(json: &str) -> Self {
        let Ok(raw) = serde_json::from_str::<Box<RawValue>>(json) else {
            return Self::failed(Code::RecordJson, "the record is not JSON");
        };
        if raw.get() == "null" {
            return Self::missing();
        }
        let id = serde_json::from_str::<BTreeMap<String, &RawValue>>(raw.get())
            .ok()
            .and_then(|fields| fields.get("id").copied());
        let id = match id
            .map(|value| serde_json::from_str::<RecordId>(value.get()))
            .transpose()
        {
            Ok(id) => id,
            Err(_) => return Self::invalid(),
        };
        Self {
            input: Input::Json(raw),
            id,
        }
    }
    /// Insert an already admitted id into a structured object's evidence before admission.
    /// Non-object values keep their original evidence and separate typed id.
    #[must_use]
    pub fn from_json_with_id(json: &str, id: RecordId) -> Self {
        let Ok(mut fields) = serde_json::from_str::<BTreeMap<String, Box<RawValue>>>(json) else {
            return Self::from_json(json).with_id(id);
        };
        let Ok(encoded) = serde_json::value::to_raw_value(&id) else {
            return Self::invalid().with_id(id);
        };
        fields.insert("id".into(), encoded);
        match serde_json::to_string(&fields) {
            Ok(json) => Self::from_json(&json),
            Err(_) => Self::invalid().with_id(id),
        }
    }
    /// Admit a structured value through its JSON representation.
    #[must_use]
    pub fn json(value: impl serde::Serialize) -> Self {
        Self::from_serialize(&value)
    }
    /// Turn a host serialization failure into one failed record.
    #[must_use]
    pub fn from_serialize<T: serde::Serialize + ?Sized>(value: &T) -> Self {
        match serde_json::to_string(value) {
            Ok(json) => Self::from_json(&json),
            Err(_) => Self::invalid(),
        }
    }
    /// Decode UTF-8 without replacing invalid bytes.
    #[must_use]
    pub fn from_utf8(bytes: &[u8]) -> Self {
        match std::str::from_utf8(bytes) {
            Ok(text) => Self::text(text),
            Err(_) => Self::invalid(),
        }
    }
    /// Decode UTF-16 without replacing an unpaired surrogate.
    #[must_use]
    pub fn from_utf16(units: &[u16]) -> Self {
        match String::from_utf16(units) {
            Ok(text) => Self::text(text),
            Err(_) => Self::invalid(),
        }
    }
    /// Attach an already typed identifier.
    #[must_use]
    pub fn with_id(mut self, id: RecordId) -> Self {
        self.id = Some(id);
        self
    }
    /// Read the admitted identifier, never an invented ordinal.
    #[must_use]
    pub const fn id(&self) -> Option<&RecordId> {
        self.id.as_ref()
    }
    pub(crate) fn failed(code: Code, message: &str) -> Self {
        Self {
            input: Input::Failed(Error::from_contract(code, message)),
            id: None,
        }
    }
    pub(crate) fn prepare(&self, pointer: &Pointer) -> super::Record {
        match &self.input {
            Input::Text(text) => super::Record::text(text),
            Input::Json(raw) => super::Record::json(raw.get(), pointer),
            Input::Failed(error) => super::Record::failed(error.clone()),
        }
    }
}
