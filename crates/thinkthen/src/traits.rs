//! Standard conversions and printing on public types, so callers use them the usual Rust way.
use crate::{BackendName, Code, Family, Record, RecordId};
use std::fmt;

/// Text field, as [`Record::text`].
impl From<&str> for Record {
    fn from(text: &str) -> Self {
        Self::text(text)
    }
}
/// Text field, as [`Record::text`].
impl From<String> for Record {
    fn from(text: String) -> Self {
        Self::text(text)
    }
}
/// Text field, as [`Record::text`], so `engine.run(&question, &message)` takes a borrowed `String`.
impl From<&String> for Record {
    fn from(text: &String) -> Self {
        Self::text(text.as_str())
    }
}
/// A JSON record, as [`Record::json`].
impl From<serde_json::Value> for Record {
    fn from(value: serde_json::Value) -> Self {
        Self::json(value)
    }
}

/// Prints the dotted code name, such as `record.missing`.
impl fmt::Display for Code {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
/// Prints the family name, such as `usage`.
impl fmt::Display for Family {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
/// Prints the backend name as `THINKTHEN_BACKEND` takes it, such as `stub`.
impl fmt::Display for BackendName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
/// Prints a numeric id as its number and a string id as its text, without quotes.
impl fmt::Display for RecordId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(id) => fmt::Display::fmt(id, formatter),
            Self::Unsigned(id) => fmt::Display::fmt(id, formatter),
            Self::String(id) => formatter.write_str(id),
        }
    }
}
