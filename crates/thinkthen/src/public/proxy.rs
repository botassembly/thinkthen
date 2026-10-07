//! Reserved proxy carriers; activation has no executable 0.2 protocol.

use crate::core;
use crate::public::Error;
use serde::{Serialize, Serializer};
use std::fmt;

/// An opaque reserved proxy identifier, containing no whitespace or controls.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ProxyId(String);

impl ProxyId {
    /// Admit 1–128 ASCII letters, digits, dots, underscores or hyphens, without trimming.
    ///
    /// # Errors
    /// Returns a usage error for any other spelling, withholding supplied bytes.
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if !(1..=128).contains(&value.len())
            || !value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
        {
            return Err(Error::usage(
                "a proxy identifier is 1–128 ASCII letters, digits, dots, underscores or hyphens",
            ));
        }
        Ok(Self(value))
    }

    /// The exact admitted spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ProxyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ProxyId").field(&"<withheld>").finish()
    }
}

/// An admitted existing cut/band reading, or the score/find null reading.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct CodeThreshold(Option<core::Threshold>);

impl CodeThreshold {
    /// The null reading used by functions with no threshold.
    #[must_use]
    pub const fn none() -> Self {
        Self(None)
    }

    /// A single cut above zero and at most one.
    ///
    /// # Errors
    /// Returns a usage error for an invalid existing cut.
    pub fn cut(value: f64) -> Result<Self, Error> {
        core::Threshold::cut(value)
            .map(|t| Self(Some(t)))
            .map_err(Error::refused)
    }

    /// An increasing band within zero to one.
    ///
    /// # Errors
    /// Returns a usage error for an invalid existing band.
    pub fn band(low: f64, high: f64) -> Result<Self, Error> {
        core::Threshold::band(low, high)
            .map(|t| Self(Some(t)))
            .map_err(Error::refused)
    }
}

/// Typed reservation for one logical question; sending it is refused in 0.2.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProxyRequest {
    /// Optional caller-supplied opaque question identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_id: Option<ProxyId>,
    /// The normalized function-specific reading.
    pub code_threshold: CodeThreshold,
    /// Separately scoped recognition relation reading, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_relation_threshold: Option<CodeThreshold>,
}

/// Presence-sensitive activation: every variant is reserved and refuses execution.
#[derive(Clone, Debug, PartialEq)]
pub enum ProxyActivation {
    /// Explicit null is supplied input, not an omitted setting.
    Null,
    /// An explicitly supplied empty object.
    Empty,
    /// A typed reservation.
    Request(ProxyRequest),
}

impl Serialize for ProxyActivation {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Empty => serializer.collect_map(std::iter::empty::<(&str, &str)>()),
            Self::Request(request) => request.serialize(serializer),
        }
    }
}

/// Reserved typed response override; no 0.2 direct reply can populate it.
#[derive(Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ProxyOverride<V> {
    /// The code reading was retained.
    None {
        /// The normalized original reading.
        code_threshold: CodeThreshold,
    },
    /// A function-admitted reading was changed.
    Threshold {
        /// The normalized original reading.
        code_threshold: CodeThreshold,
        /// The effective reading.
        effective_threshold: CodeThreshold,
    },
    /// A typed actionable value was overridden.
    Decision {
        /// The normalized original reading.
        code_threshold: CodeThreshold,
        /// The code's original actionable value.
        code_value: V,
        /// The effective actionable value.
        value: V,
    },
}

/// Reserved metadata for a future admitted proxy response.
#[derive(Clone, PartialEq, Serialize)]
pub struct ProxyMetadata<V> {
    /// The proxy's opaque decision identity.
    pub decision_id: ProxyId,
    /// The typed effective reading or value.
    #[serde(rename = "override")]
    pub reading: ProxyOverride<V>,
}

impl<V> fmt::Debug for ProxyOverride<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self {
            Self::None { .. } => "none",
            Self::Threshold { .. } => "threshold",
            Self::Decision { .. } => "decision",
        };
        f.debug_struct("ProxyOverride")
            .field("kind", &kind)
            .finish_non_exhaustive()
    }
}

impl<V> fmt::Debug for ProxyMetadata<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProxyMetadata")
            .field("decision_id", &self.decision_id)
            .field("override", &self.reading)
            .finish()
    }
}
