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
            None => Err(
                Error::usage("batch requires a positive whole number or max")
                    .with_diagnostic(crate::public::error::diagnostic::Diagnostic::Batch),
            ),
        }
    }
}
/// Optional controls; absent values retain native defaults. Null refuses except
/// where it is explicit authored yes/no or label-description content.
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
    /// Replacement yes meaning; explicit null is authored content, absence retains the saved meaning.
    #[serde(default, deserialize_with = "super::wording::description", serialize_with = "super::wording::serialize_description", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, schemars(with = "crate::core::Description"))]
    pub yes: Option<crate::Description>,
    /// Replacement no meaning; explicit null is authored content, absence retains the saved meaning.
    #[serde(default, deserialize_with = "super::wording::description", serialize_with = "super::wording::serialize_description", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, schemars(with = "crate::core::Description"))]
    pub no: Option<crate::Description>,
    /// Whole replacement choose options or tag labels, in caller order with descriptions.
    #[serde(default, deserialize_with = "super::wording::list", serialize_with = "super::wording::serialize_list", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, schemars(with = "Vec<super::wording::OptionSchema>"))]
    pub labels: Option<Vec<crate::RecordOption>>,
    /// Whole replacement score levels, from lowest to highest with descriptions.
    #[serde(default, deserialize_with = "super::wording::list", serialize_with = "super::wording::serialize_list", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, schemars(with = "Vec<super::wording::OptionSchema>"))]
    pub levels: Option<Vec<crate::RecordOption>>,
    /// Explicit recognition stage selection; omission retains the saved mode.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "crate::RecognitionMode"))]
    pub mode: Option<crate::RecognitionMode>,
    /// Recognition tokenizer pieces on each side; omission retains the saved width.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "u32", range(max = u32::MAX)))]
    pub snippet_pieces: Option<u32>,
    /// Positive original text-byte ceiling for recognition; omission retains its native limit.
    #[serde(default, deserialize_with = "present", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, schemars(with = "usize", range(min = 1)))]
    pub max_text_bytes: Option<usize>,
    /// Replacement recognition task wording; absence retains saved wording.
    #[serde(default, deserialize_with = "present", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, schemars(with = "String"))]
    pub instructions: Option<String>,
    /// Replacement literal entity definition; absence retains the saved definition.
    #[serde(default, deserialize_with = "present", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, schemars(with = "String"))]
    pub entity_definition: Option<String>,
    /// Authored relation cut, applicable only to whole recognition.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "RequestThreshold"))]
    pub relation_threshold: Option<RequestThreshold>,
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
    /// Literal recognition context overrides, resolved member by member.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "empty_stage_context"
    )]
    #[cfg_attr(test, schemars(with = "crate::RecognitionStageContext"))]
    pub stage_context: Option<crate::RecognitionStageContext>,
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
    /// Unconfirmed recognition proposals using exact Unicode scalar piece edges.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "Vec<crate::RecognitionSeedSpan>"))]
    pub seed_spans: Option<Vec<crate::RecognitionSeedSpan>>,
    /// Optional recognition boundary examples; an empty list clears the fallback.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "Vec<crate::RecognitionExample>"))]
    pub examples: Option<Vec<crate::RecognitionExample>>,
    /// Per-record seed selection; missing retains fallback and an empty list clears it.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "String"))]
    pub seed_spans_field: Option<String>,
    /// Optional per-record recognition example projection.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(test, schemars(with = "String"))]
    pub examples_field: Option<String>,
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

fn empty_stage_context(value: &Option<crate::RecognitionStageContext>) -> bool {
    value
        .as_ref()
        .is_none_or(crate::RecognitionStageContext::is_empty)
}
