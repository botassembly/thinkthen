//! Typed questions and their strict contract representation.

use super::*;

/// A validated decide question with its own options.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "DecideWire", into = "DecideWire")]
#[schemars(with = "DecideWire")]
pub struct Decide {
    pub(crate) inner: crate::core::Decide,
    pub(super) evidence: Option<Pointer>,
}
impl Decide {
    /// Admit this function's required question and domain.
    /// # Errors
    /// Returns the shared question or name validation error.
    pub fn new(question: impl Into<String>) -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::Decide::new(question)?,
            evidence: None,
        })
    }
    /// Select evidence with an admitted JSON Pointer.
    #[must_use]
    pub fn evidence(mut self, pointer: Pointer) -> Self {
        self.evidence = Some(pointer);
        self
    }
    /// The admitted evidence pointer.
    #[must_use]
    pub fn evidence_pointer(&self) -> &Pointer {
        self.evidence
            .as_ref()
            .unwrap_or(&crate::core::record::ROOT_POINTER)
    }
    /// Set this function's typed threshold.
    #[must_use]
    pub fn threshold(mut self, threshold: Threshold) -> Self {
        self.inner.threshold = threshold;
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct DecideWire {
    #[serde(deserialize_with = "crate::core::diagnostic::question")]
    question: String,
    #[serde(default)]
    #[serde(deserialize_with = "crate::core::diagnostic::threshold")]
    threshold: ThresholdInput,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_evidence"
    )]
    #[schemars(with = "Pointer")]
    evidence: Option<String>,
}
impl TryFrom<DecideWire> for Decide {
    type Error = Error;
    fn try_from(value: DecideWire) -> Result<Self, Error> {
        let mut question = Self::new(value.question)?;
        question.evidence = admit_evidence(value.evidence)?;
        question.inner.threshold = value.threshold.admit()?;
        Ok(question)
    }
}
impl From<Decide> for DecideWire {
    fn from(value: Decide) -> Self {
        Self {
            question: value.inner.text,
            threshold: value.inner.threshold.into(),
            evidence: value.evidence.map(|pointer| pointer.to_string()),
        }
    }
}

/// A validated filter question with its own options.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "FilterWire", into = "FilterWire")]
#[schemars(with = "FilterWire")]
pub struct Filter {
    pub(crate) inner: crate::core::Decide,
    pub(super) evidence: Option<Pointer>,
}
impl Filter {
    /// Admit this function's required question and domain.
    /// # Errors
    /// Returns the shared question or name validation error.
    pub fn new(question: impl Into<String>) -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::Decide::new(question)?,
            evidence: None,
        })
    }
    /// Select evidence with an admitted JSON Pointer.
    #[must_use]
    pub fn evidence(mut self, pointer: Pointer) -> Self {
        self.evidence = Some(pointer);
        self
    }
    /// The admitted evidence pointer.
    #[must_use]
    pub fn evidence_pointer(&self) -> &Pointer {
        self.evidence
            .as_ref()
            .unwrap_or(&crate::core::record::ROOT_POINTER)
    }
    /// Set this function's typed threshold.
    #[must_use]
    pub fn threshold(mut self, threshold: Threshold) -> Self {
        self.inner.threshold = threshold;
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct FilterWire {
    #[serde(deserialize_with = "crate::core::diagnostic::question")]
    question: String,
    #[serde(default)]
    #[serde(deserialize_with = "crate::core::diagnostic::threshold")]
    threshold: ThresholdInput,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_evidence"
    )]
    #[schemars(with = "Pointer")]
    evidence: Option<String>,
}
impl TryFrom<FilterWire> for Filter {
    type Error = Error;
    fn try_from(value: FilterWire) -> Result<Self, Error> {
        let mut question = Self::new(value.question)?;
        question.evidence = admit_evidence(value.evidence)?;
        question.inner.threshold = value.threshold.admit()?;
        Ok(question)
    }
}
impl From<Filter> for FilterWire {
    fn from(value: Filter) -> Self {
        Self {
            question: value.inner.text,
            threshold: value.inner.threshold.into(),
            evidence: value.evidence.map(|pointer| pointer.to_string()),
        }
    }
}

/// A validated find question with its own options.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "FindWire", into = "FindWire")]
#[schemars(with = "FindWire")]
pub struct Find {
    pub(crate) inner: crate::core::Find,
    pub(super) evidence: Option<Pointer>,
}
impl Find {
    /// Admit this function's required question and domain.
    /// # Errors
    /// Returns the shared question or name validation error.
    pub fn new(question: impl Into<String>) -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::Find::new(question)?,
            evidence: None,
        })
    }
    /// Select evidence with an admitted JSON Pointer.
    #[must_use]
    pub fn evidence(mut self, pointer: Pointer) -> Self {
        self.evidence = Some(pointer);
        self
    }
    /// The admitted evidence pointer.
    #[must_use]
    pub fn evidence_pointer(&self) -> &Pointer {
        self.evidence
            .as_ref()
            .unwrap_or(&crate::core::record::ROOT_POINTER)
    }
    /// Set this function's typed threshold.
    #[must_use]
    pub fn threshold(mut self, threshold: Cut) -> Self {
        self.inner.threshold = threshold.into();
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct FindWire {
    #[serde(deserialize_with = "crate::core::diagnostic::question")]
    question: String,
    #[schemars(with = "Cut")]
    #[serde(default)]
    #[serde(deserialize_with = "crate::core::diagnostic::threshold")]
    threshold: ThresholdInput,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_evidence"
    )]
    #[schemars(with = "Pointer")]
    evidence: Option<String>,
}
impl TryFrom<FindWire> for Find {
    type Error = Error;
    fn try_from(value: FindWire) -> Result<Self, Error> {
        let mut question = Self::new(value.question)?;
        let threshold = value.threshold.admit()?;
        threshold.require_cut()?;
        question.evidence = admit_evidence(value.evidence)?;
        question.inner.threshold = threshold;
        Ok(question)
    }
}
impl From<Find> for FindWire {
    fn from(value: Find) -> Self {
        Self {
            question: value.inner.text,
            threshold: value.inner.threshold.into(),
            evidence: value.evidence.map(|pointer| pointer.to_string()),
        }
    }
}
