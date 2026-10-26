//! Every failure: a stable code, a family and one message.

use serde::ser::{Serialize, Serializer};
use std::sync::Arc;

use super::code::Code;

/// Which part of a call failed. Each family maps to one exit code.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    /// The caller's arguments or question are wrong.
    Usage,
    /// One input record could not be admitted.
    Record,
    /// An unexpected engine fault stopped the call.
    Internal,
    /// The backend could not be reached, refused, or answered badly.
    Backend,
    /// A local file or stream could not be read or written.
    Local,
}

/// The kind of a failure. Its code never changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ErrorKind {
    /// A threshold is not a valid cut or band.
    Threshold,
    /// A question is blank or malformed.
    Question,
    /// A caller-defined name contains U+0000.
    Name,
    /// Client settings do not match their public grammar.
    Settings,
    /// The selected backend name is unknown.
    Backend,
    /// A batch size is not a whole number of at least 1.
    Batch,
    /// A pointer is not a JSON Pointer.
    Pointer,
    /// A record is not JSON.
    Record,
    /// A record holds nothing where the pointer points.
    Missing,
    /// One record's question is too large for a request on its own.
    BackendLimit,
    /// A backend address is invalid or sends credentials without encryption.
    Url,
    /// No API key is set.
    KeyMissing,
    /// The backend could not be reached.
    Transport,
    /// The backend answered with an error status.
    Status,
    /// The backend's reply does not answer the request.
    Reply,
    /// The caller cancelled the call.
    Cancelled,
    /// A local file or stream could not be read or written.
    #[cfg(feature = "command")]
    Io,
    /// A toolbox cannot be read as valid named questions.
    #[cfg(feature = "command")]
    Toolbox,
}

impl Family {
    /// The canonical host error-family name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Usage => "usage",
            Self::Record => "record",
            Self::Internal => "internal",
            Self::Backend => "backend",
            Self::Local => "local",
        }
    }
}

impl ErrorKind {}

/// A failure with its kind and its message.
#[non_exhaustive]
#[derive(Debug, Clone, schemars::JsonSchema)]
#[schemars(with = "ErrorWire<'static, Code>")]
pub struct Error {
    message: String,
    code: Code,
    pub(crate) http_status: Option<u16>,
    pub(crate) source: Option<Arc<dyn std::error::Error + Send + Sync>>,
}

impl Error {
    pub(crate) fn from_contract(code: Code, message: impl Into<String>) -> Self {
        Self {
            message: escape_diagnostic(&message.into()),
            code,
            http_status: None,
            source: None,
        }
    }

    /// A failure of `kind` that says `message`.
    #[must_use]
    pub(crate) fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            message: escape_diagnostic(&message.into()),
            code: kind.contract_code(),
            http_status: None,
            source: None,
        }
    }

    /// The stable typed identity of this error.
    #[must_use]
    pub const fn code(&self) -> Code {
        self.code
    }

    /// The library's diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// The HTTP status when the backend returned one.
    #[must_use]
    pub const fn http_status(&self) -> Option<u16> {
        self.http_status
    }
}

impl Serialize for Error {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ErrorWire {
            code: self.code,
            message: &self.message,
            http_status: self.http_status,
        }
        .serialize(serializer)
    }
}

// Schema derives emit JSON metadata maps in sibling impls, scoped to these values.
#[allow(
    clippy::disallowed_types,
    reason = "schema derivation builds metadata; evidence remains typed"
)]
mod wire {
    #[derive(serde::Serialize, schemars::JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub(crate) struct ErrorWire<'a, C> {
        pub(super) code: C,
        pub(super) message: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(super) http_status: Option<u16>,
    }
}
pub(crate) use wire::ErrorWire;

/// Escape actual diagnostic controls once, preserving printable text and literal escapes.
#[must_use]
pub(crate) fn escape_diagnostic(text: &str) -> String {
    text.chars().fold(
        String::with_capacity(text.len()),
        |mut escaped, character| {
            match character {
                '\u{0008}' => escaped.push_str("\\b"),
                '\t' => escaped.push_str("\\t"),
                '\n' => escaped.push_str("\\n"),
                '\u{000c}' => escaped.push_str("\\f"),
                '\r' => escaped.push_str("\\r"),
                value if value.is_control() => {
                    escaped.push_str(&format!("\\u{:04x}", u32::from(value)))
                }
                value => escaped.push(value),
            }
            escaped
        },
    )
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &dyn std::error::Error)
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
            && self.message == other.message
            && self.http_status == other.http_status
    }
}
impl Eq for Error {}

impl<'de> serde::Deserialize<'de> for Error {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            code: Code,
            message: String,
            http_status: Option<u16>,
        }
        let wire = Wire::deserialize(deserializer)?;
        Ok(Self {
            code: wire.code,
            message: escape_diagnostic(&wire.message),
            http_status: wire.http_status,
            source: None,
        })
    }
}

// Private worker conveniences map to the single public code registry.
impl ErrorKind {
    const fn contract_code(self) -> Code {
        match self {
            Self::Threshold => Code::UsageThreshold,
            Self::Question => Code::UsageQuestion,
            Self::Name => Code::UsageName,
            Self::Settings => Code::UsageSettings,
            Self::Backend => Code::UsageBackend,
            Self::Batch => Code::UsageBatch,
            Self::Pointer => Code::UsagePointer,
            Self::Record => Code::RecordJson,
            Self::Missing => Code::RecordMissing,
            Self::BackendLimit => Code::RecordTooLarge,
            Self::Url => Code::UsageUrl,
            Self::KeyMissing => Code::UsageKeyMissing,
            Self::Transport => Code::BackendTransport,
            Self::Status => Code::BackendStatus,
            Self::Reply => Code::BackendReply,
            Self::Cancelled => Code::LocalCancelled,
            #[cfg(feature = "command")]
            Self::Io => Code::LocalIo,
            #[cfg(feature = "command")]
            Self::Toolbox => Code::LocalToolbox,
        }
    }
}
