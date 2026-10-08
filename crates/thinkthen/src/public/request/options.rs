//! Typed request controls use the existing native validators.
use super::present;
use crate::{BatchSetting, Error};
use serde::{Deserialize, Serialize};

/// A cut or authored threshold rule.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum RequestThreshold {
    /// Numeric cut.
    Cut(f64),
    /// Existing named or band reading.
    Rule(String),
}
impl RequestThreshold {
    pub(super) fn native(&self) -> Result<crate::core::Threshold, Error> {
        match self {
            Self::Cut(v) => crate::core::Threshold::cut(*v),
            Self::Rule(v) => v.parse(),
        }
        .map_err(Error::refused)
    }
}
/// A bounded native batch setting.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum RequestBatch {
    /// A positive record count.
    Count(usize),
    /// The existing max setting.
    Named(String),
}
impl RequestBatch {
    /// Validate through the existing batch grammar.
    /// # Errors
    /// Refuses zero or an unknown setting.
    pub fn native(&self) -> Result<BatchSetting, Error> {
        let setting = match self {
            Self::Count(n) => crate::core::Setting::parse(&n.to_string()),
            Self::Named(name) if name == "max" => Some(crate::core::Setting::Max),
            Self::Named(_) => None,
        };
        match setting {
            Some(crate::core::Setting::Records(n)) => Ok(BatchSetting::Records(n)),
            Some(crate::core::Setting::Max) => Ok(BatchSetting::Max),
            None => Err(Error::usage(
                "batch requires a positive whole number or max",
            )),
        }
    }
}
/// Optional controls; absent values retain the native defaults, null refuses.
#[derive(Clone, Default, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct RequestOptions {
    /// Explicit model on the engine's configured route.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "String"))]
    pub model: Option<String>,
    /// Explicit reading override.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "RequestThreshold"))]
    pub threshold: Option<RequestThreshold>,
    /// Shared context fallback, including explicit empty text.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "String"))]
    pub context: Option<String>,
    /// Evidence projection pointers in their caller order.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "Vec<String>"))]
    pub field: Option<Vec<String>>,
    /// Per-item context projection.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "String"))]
    pub context_field: Option<String>,
    /// Per-item ordered choose shortlist projection.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "String"))]
    pub options_field: Option<String>,
    /// Native batch scheduling.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "RequestBatch"))]
    pub batch: Option<RequestBatch>,
    /// Existing millisecond deadline, including -1 for no deadline.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "i64"))]
    pub deadline_ms: Option<i64>,
    /// Process send budget for this call.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "u64"))]
    pub max_requests_total: Option<u64>,
    /// Collect observed attempts.
    #[serde(default)]
    pub attempts: bool,
    /// Compatibility details output for scalar calls.
    #[serde(default)]
    pub details: bool,
    /// Offer a none candidate in find.
    #[serde(default)]
    pub none: bool,
    /// Retain the requested number of ranked rows.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "usize"))]
    pub top: Option<usize>,
    /// Keep one selected record per physical file.
    #[serde(default)]
    pub files_only: bool,
}
impl std::fmt::Debug for RequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestOptions(<withheld>)")
    }
}
