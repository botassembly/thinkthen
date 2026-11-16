//! Typed questions and their strict contract representation.

use super::*;

/// A validated decide question with its own options.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "DecideWire", into = "DecideWire")]
#[schemars(with = "DecideWire")]
pub struct Decide {
    pub(crate) inner: crate::core::Decide,
    pub(super) field: Option<Pointer>,
}
impl Decide {
    /// Admit this function's required question and domain.
    /// # Errors
    /// Returns the shared question or name validation error.
    pub fn new(question: impl Into<String>) -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::Decide::new(question)?,
            field: None,
        })
    }
    /// Set the meaning of a true answer.
    #[must_use]
    pub fn yes_means(mut self, meaning: impl Into<String>) -> Self {
        self.inner.yes = Some(meaning.into());
        self
    }
    /// Set the meaning of a false answer.
    #[must_use]
    pub fn no_means(mut self, meaning: impl Into<String>) -> Self {
        self.inner.no = Some(meaning.into());
        self
    }
    /// Select field with an admitted JSON Pointer.
    #[must_use]
    pub fn field(mut self, pointer: Pointer) -> Self {
        self.field = Some(pointer);
        self
    }
    /// The admitted field pointer.
    #[must_use]
    pub fn field_pointer(&self) -> &Pointer {
        self.field
            .as_ref()
            .unwrap_or(&crate::core::record::ROOT_POINTER)
    }
    /// Set this function's typed threshold.
    #[must_use]
    pub fn threshold(mut self, threshold: impl Into<Threshold>) -> Self {
        self.inner.threshold = threshold.into();
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct DecideWire {
    #[serde(deserialize_with = "crate::core::diagnostic::question")]
    question: String,
    #[serde(default, rename = "yes_means", skip_serializing_if = "Option::is_none")]
    yes: Option<String>,
    #[serde(default, rename = "no_means", skip_serializing_if = "Option::is_none")]
    no: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "crate::core::diagnostic::threshold")]
    threshold: ThresholdInput,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Pointer")]
    field: Option<String>,
}
impl TryFrom<DecideWire> for Decide {
    type Error = Error;
    fn try_from(value: DecideWire) -> Result<Self, Error> {
        let mut question = Self::new(value.question)?;
        question.inner.yes = value.yes;
        question.inner.no = value.no;
        question.field = admit_field(value.field)?;
        question.inner.threshold = value.threshold.admit()?;
        Ok(question)
    }
}
impl From<Decide> for DecideWire {
    fn from(value: Decide) -> Self {
        Self {
            question: value.inner.text,
            yes: value.inner.yes,
            no: value.inner.no,
            threshold: value.inner.threshold.into(),
            field: value.field.map(|pointer| pointer.to_string()),
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
    pub(super) field: Option<Pointer>,
}
impl Filter {
    /// Admit this function's required question and domain.
    /// # Errors
    /// Returns the shared question or name validation error.
    pub fn new(question: impl Into<String>) -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::Decide::new(question)?,
            field: None,
        })
    }
    /// Set the meaning of a true answer.
    #[must_use]
    pub fn yes_means(mut self, meaning: impl Into<String>) -> Self {
        self.inner.yes = Some(meaning.into());
        self
    }
    /// Set the meaning of a false answer.
    #[must_use]
    pub fn no_means(mut self, meaning: impl Into<String>) -> Self {
        self.inner.no = Some(meaning.into());
        self
    }
    /// Select field with an admitted JSON Pointer.
    #[must_use]
    pub fn field(mut self, pointer: Pointer) -> Self {
        self.field = Some(pointer);
        self
    }
    /// The admitted field pointer.
    #[must_use]
    pub fn field_pointer(&self) -> &Pointer {
        self.field
            .as_ref()
            .unwrap_or(&crate::core::record::ROOT_POINTER)
    }
    /// Set the single decision cut; use decide for a low/high band.
    /// ```compile_fail
    /// let filter = thinkthen::Filter::new("Keep?").unwrap()
    ///     .threshold(thinkthen::Threshold::band(0.2, 0.8).unwrap());
    /// ```
    #[must_use]
    pub fn threshold(mut self, threshold: Cut) -> Self {
        self.inner.threshold = threshold.into();
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct FilterWire {
    #[serde(deserialize_with = "crate::core::diagnostic::question")]
    question: String,
    #[serde(default, rename = "yes_means", skip_serializing_if = "Option::is_none")]
    yes: Option<String>,
    #[serde(default, rename = "no_means", skip_serializing_if = "Option::is_none")]
    no: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "crate::core::diagnostic::threshold")]
    #[schemars(with = "Cut")]
    threshold: ThresholdInput,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Pointer")]
    field: Option<String>,
}
impl TryFrom<FilterWire> for Filter {
    type Error = Error;
    fn try_from(value: FilterWire) -> Result<Self, Error> {
        let mut question = Self::new(value.question)?;
        question.inner.yes = value.yes;
        question.inner.no = value.no;
        question.field = admit_field(value.field)?;
        question.inner.threshold = value.threshold.admit()?.filter_cut()?.into();
        Ok(question)
    }
}
impl From<Filter> for FilterWire {
    fn from(value: Filter) -> Self {
        Self {
            question: value.inner.text,
            yes: value.inner.yes,
            no: value.inner.no,
            threshold: value.inner.threshold.into(),
            field: value.field.map(|pointer| pointer.to_string()),
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
    pub(super) field: Option<Pointer>,
}
impl Find {
    /// Admit this function's required question and domain.
    /// # Errors
    /// Returns the shared question or name validation error.
    pub fn new(question: impl Into<String>) -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::Find::new(question)?,
            field: None,
        })
    }
    /// Select field with an admitted JSON Pointer.
    #[must_use]
    pub fn field(mut self, pointer: Pointer) -> Self {
        self.field = Some(pointer);
        self
    }
    /// The admitted field pointer.
    #[must_use]
    pub fn field_pointer(&self) -> &Pointer {
        self.field
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
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Pointer")]
    field: Option<String>,
}
impl TryFrom<FindWire> for Find {
    type Error = Error;
    fn try_from(value: FindWire) -> Result<Self, Error> {
        let mut question = Self::new(value.question)?;
        let threshold = value.threshold.admit()?;
        threshold.require_cut()?;
        question.field = admit_field(value.field)?;
        question.inner.threshold = threshold;
        Ok(question)
    }
}
impl From<Find> for FindWire {
    fn from(value: Find) -> Self {
        Self {
            question: value.inner.text,
            threshold: value.inner.threshold.into(),
            field: value.field.map(|pointer| pointer.to_string()),
        }
    }
}
