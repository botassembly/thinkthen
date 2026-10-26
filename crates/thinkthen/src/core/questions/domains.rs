//! Typed questions and their strict contract representation.

use super::*;

/// A validated choose question with its own options.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "ChooseWire", into = "ChooseWire")]
#[schemars(with = "ChooseWire")]
pub struct Choose {
    pub(crate) inner: crate::core::Choose,
    pub(super) evidence: Option<Pointer>,
}
impl Choose {
    /// Admit this function's required question and domain.
    /// # Errors
    /// Returns the shared question or name validation error.
    pub fn new(
        question: impl Into<String>,
        choices: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::Choose::new(
                question,
                choices
                    .into_iter()
                    .map(|value| crate::core::ChoiceOption::new(value.into()))
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
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
pub(crate) struct ChooseWire {
    #[serde(deserialize_with = "crate::core::diagnostic::question")]
    question: String,
    #[schemars(length(min = MIN_CHOICES))]
    #[serde(deserialize_with = "crate::core::diagnostic::choices")]
    choices: Vec<String>,
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
impl TryFrom<ChooseWire> for Choose {
    type Error = Error;
    fn try_from(value: ChooseWire) -> Result<Self, Error> {
        let mut question = Self::new(value.question, value.choices)?;
        question.evidence = admit_evidence(value.evidence)?;
        question.inner.threshold = value.threshold.admit()?;
        Ok(question)
    }
}
impl From<Choose> for ChooseWire {
    fn from(value: Choose) -> Self {
        Self {
            question: value.inner.text,
            choices: value
                .inner
                .options
                .iter()
                .map(|value| value.label().to_owned())
                .collect(),
            threshold: value.inner.threshold.into(),
            evidence: value.evidence.map(|pointer| pointer.to_string()),
        }
    }
}

/// A validated tag question with its own options.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "TagWire", into = "TagWire")]
#[schemars(with = "TagWire")]
pub struct Tag {
    pub(crate) inner: crate::core::Tag,
    pub(super) evidence: Option<Pointer>,
}
impl Tag {
    /// Admit this function's required question and domain.
    /// # Errors
    /// Returns the shared question or name validation error.
    pub fn new(
        question: impl Into<String>,
        labels: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::Tag::new(question, labels)?,
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
pub(crate) struct TagWire {
    #[serde(deserialize_with = "crate::core::diagnostic::question")]
    question: String,
    #[schemars(length(min = MIN_LABELS))]
    #[serde(deserialize_with = "crate::core::diagnostic::labels")]
    labels: Vec<String>,
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
impl TryFrom<TagWire> for Tag {
    type Error = Error;
    fn try_from(value: TagWire) -> Result<Self, Error> {
        let mut question = Self::new(value.question, value.labels)?;
        let threshold = value.threshold.admit()?;
        threshold.require_cut()?;
        question.evidence = admit_evidence(value.evidence)?;
        question.inner.threshold = threshold;
        Ok(question)
    }
}
impl From<Tag> for TagWire {
    fn from(value: Tag) -> Self {
        Self {
            question: value.inner.text,
            labels: value.inner.labels,
            threshold: value.inner.threshold.into(),
            evidence: value.evidence.map(|pointer| pointer.to_string()),
        }
    }
}

/// A validated score question with its own options.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "ScoreWire", into = "ScoreWire")]
#[schemars(with = "ScoreWire")]
pub struct Score {
    pub(crate) inner: crate::core::Score,
    pub(super) evidence: Option<Pointer>,
}
impl Score {
    /// Admit this function's required question and domain.
    /// # Errors
    /// Returns the shared question or name validation error.
    pub fn new(
        question: impl Into<String>,
        levels: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::Score::new(question, levels)?,
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
}
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ScoreWire {
    #[serde(deserialize_with = "crate::core::diagnostic::question")]
    question: String,
    #[schemars(length(min = MIN_LEVELS))]
    #[serde(deserialize_with = "crate::core::diagnostic::levels")]
    levels: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_evidence"
    )]
    #[schemars(with = "Pointer")]
    evidence: Option<String>,
}
impl TryFrom<ScoreWire> for Score {
    type Error = Error;
    fn try_from(value: ScoreWire) -> Result<Self, Error> {
        let mut question = Self::new(value.question, value.levels)?;
        question.evidence = admit_evidence(value.evidence)?;
        Ok(question)
    }
}
impl From<Score> for ScoreWire {
    fn from(value: Score) -> Self {
        Self {
            question: value.inner.text,
            levels: value.inner.levels,
            evidence: value.evidence.map(|pointer| pointer.to_string()),
        }
    }
}

/// A validated recognize question with its own options.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "RecognizeWire", into = "RecognizeWire")]
#[schemars(with = "RecognizeWire")]
pub struct Recognize {
    pub(crate) inner: crate::core::Recognize,
    pub(super) evidence: Option<Pointer>,
}
impl Recognize {
    /// Admit this function's required question and domain.
    /// # Errors
    /// Returns the shared question or name validation error.
    pub fn new(kinds: impl IntoIterator<Item = impl Into<String>>) -> Result<Self, Error> {
        let kinds: Vec<String> = kinds.into_iter().map(Into::into).collect();
        if kinds.len() < MIN_KINDS {
            return Err(question_error(crate::core::diagnostic::requirement(
                "kinds",
                "a nonempty list of kinds",
                "an empty list",
            )));
        }
        Ok(Self {
            inner: crate::core::Recognize::new(kinds)?,
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
pub(crate) struct RecognizeWire {
    #[schemars(length(min = MIN_KINDS))]
    #[serde(deserialize_with = "crate::core::diagnostic::kinds")]
    kinds: Vec<String>,
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
impl TryFrom<RecognizeWire> for Recognize {
    type Error = Error;
    fn try_from(value: RecognizeWire) -> Result<Self, Error> {
        let mut question = Self::new(value.kinds)?;
        let threshold = value.threshold.admit()?;
        threshold.require_cut()?;
        question.evidence = admit_evidence(value.evidence)?;
        question.inner.threshold = threshold;
        Ok(question)
    }
}
impl From<Recognize> for RecognizeWire {
    fn from(value: Recognize) -> Self {
        Self {
            kinds: value
                .inner
                .kinds
                .iter()
                .map(|value| value.label().to_owned())
                .collect(),
            threshold: value.inner.threshold.into(),
            evidence: value.evidence.map(|pointer| pointer.to_string()),
        }
    }
}
