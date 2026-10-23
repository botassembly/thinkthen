//! Every failure: a stable code, a family and one message.

use serde::ser::{Serialize, Serializer};

/// Which part of a call failed. Each family maps to one exit code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    /// The caller's arguments or question are wrong.
    Usage,
    /// The backend could not be reached, refused, or answered badly.
    Backend,
    /// A local file or stream could not be read or written.
    Local,
}

/// The kind of a failure. Its code never changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
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
    /// The selected client mode has no transport.
    Mode,
    /// A named question has no available resolver.
    Reference,
    /// A key source differs from the selected backend's own variable.
    KeySource,
    /// The database administrator's backend/model policy is malformed.
    SqlPolicy,
    /// The selected backend/model pair is outside the database policy.
    SqlSelection,
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
    Io,
    /// A toolbox cannot be read as valid named questions.
    Toolbox,
}

impl Family {
    /// The canonical host error-family name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Usage => "usage",
            Self::Backend => "backend",
            Self::Local => "local",
        }
    }
}

impl ErrorKind {
    /// The stable dotted code, such as `usage.threshold`.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Threshold => "usage.threshold",
            Self::Question => "usage.question",
            Self::Name => "usage.name",
            Self::Settings => "usage.settings",
            Self::Backend => "usage.backend",
            Self::Mode => "usage.mode",
            Self::Reference => "usage.reference",
            Self::KeySource => "usage.key-source",
            Self::SqlPolicy => "usage.sql-policy",
            Self::SqlSelection => "usage.sql-selection",
            Self::Batch => "usage.batch",
            Self::Pointer => "usage.pointer",
            Self::Record => "usage.record",
            Self::Missing => "usage.missing",
            Self::BackendLimit => "backend.limit",
            Self::Url => "usage.url",
            Self::KeyMissing => "usage.key-missing",
            Self::Transport => "backend.transport",
            Self::Status => "backend.status",
            Self::Reply => "backend.reply",
            Self::Cancelled => "local.cancelled",
            Self::Io => "local.io",
            Self::Toolbox => "local.toolbox",
        }
    }

    /// The family this kind belongs to.
    #[must_use]
    pub const fn family(self) -> Family {
        match self {
            Self::Threshold
            | Self::Question
            | Self::Name
            | Self::Batch
            | Self::Pointer
            | Self::Record
            | Self::Missing
            | Self::Url
            | Self::Settings
            | Self::Backend
            | Self::Mode
            | Self::Reference
            | Self::KeySource
            | Self::SqlPolicy
            | Self::SqlSelection
            | Self::KeyMissing => Family::Usage,
            Self::BackendLimit | Self::Transport | Self::Status | Self::Reply => Family::Backend,
            Self::Cancelled | Self::Io | Self::Toolbox => Family::Local,
        }
    }
}

/// A failure with its kind and its message.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, schemars::JsonSchema)]
#[error("{message}")]
#[schemars(with = "ErrorWire<'static>")]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    /// A failure of `kind` that says `message`.
    #[must_use]
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: escape_diagnostic(&message.into()),
        }
    }

    /// Treat a toolbox supplied as a call argument as a usage failure.
    #[must_use]
    pub fn into_argument(mut self) -> Self {
        if self.kind == ErrorKind::Toolbox {
            self.kind = ErrorKind::Question;
        }
        self
    }

    /// The kind of failure.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl Serialize for Error {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ErrorWire {
            code: self.kind.code(),
            message: &self.message,
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
    pub(super) struct ErrorWire<'a> {
        pub(super) code: &'a str,
        pub(super) message: &'a str,
    }
}
use wire::ErrorWire;

/// Escape actual diagnostic controls once, preserving printable text and literal escapes.
#[must_use]
pub fn escape_diagnostic(text: &str) -> String {
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
