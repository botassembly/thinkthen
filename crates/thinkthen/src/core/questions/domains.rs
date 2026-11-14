//! Typed questions and their strict contract representation.

use super::*;

/// A validated choose question with its own options.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "ChooseWire", into = "ChooseWire")]
#[schemars(with = "ChooseWire")]
pub struct Choose<C: Choice = String> {
    #[serde(skip)]
    pub(super) choice: std::marker::PhantomData<C>,
    pub(crate) inner: crate::core::Choose,
    pub(super) field: Option<Pointer>,
}
impl<C: Choice> Choose<C> {
    /// Admit named choices with optional descriptions.
    /// # Errors
    /// Returns the shared question or domain validation error.
    pub(super) fn from_options(
        question: impl Into<String>,
        choices: impl IntoIterator<Item = AnswerOption>,
    ) -> Result<Self, Error> {
        let choices: Vec<_> = choices.into_iter().collect();
        choice::validate::<C>(&choices)?;
        Ok(Self {
            choice: std::marker::PhantomData,
            inner: crate::core::Choose::new(question, choices)?,
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
    pub(super) fn typed<D: Choice>(self) -> Result<Choose<D>, Error> {
        choice::validate::<D>(&self.inner.options)?;
        Ok(Choose {
            inner: self.inner,
            field: self.field,
            choice: std::marker::PhantomData,
        })
    }
    pub(crate) fn erase(self) -> Choose {
        Choose {
            inner: self.inner,
            field: self.field,
            choice: std::marker::PhantomData,
        }
    }
}
impl Choose<String> {
    /// Admit explicit string labels and descriptions.
    /// # Errors
    /// Refuses invalid question text or label domains.
    pub fn with_options(
        question: impl Into<String>,
        values: impl IntoIterator<Item = AnswerOption>,
    ) -> Result<Self, Error> {
        Self::from_options(question, values)
    }

    /// Admit this function's required question and domain.
    /// # Errors
    /// Returns the shared question or name validation error.
    pub fn with_strings(
        question: impl Into<String>,
        choices: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, Error> {
        Self::from_options(
            question,
            choices
                .into_iter()
                .map(|value| AnswerOption::new(value.into()))
                .collect::<Result<Vec<_>, _>>()?,
        )
    }
}
impl<C: Choices> Choose<C> {
    /// Admit a question using this enum's declaration order.
    /// # Errors
    /// Refuses invalid question text or an invalid enum domain.
    pub fn new(question: impl Into<String>) -> Result<Self, Error> {
        Self::from_options(question, choice::options::<C>()?)
    }
    /// Describe enum members without changing their declaration order.
    #[must_use]
    pub fn descriptions(
        mut self,
        values: impl IntoIterator<Item = (C, impl Into<String>)>,
    ) -> Self {
        choice::descriptions(&mut self.inner.options, values);
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
    #[schemars(with = "Vec<AnswerOption>")]
    choices: Vec<OptionWire>,
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
impl<C: Choice> TryFrom<ChooseWire> for Choose<C> {
    type Error = Error;
    fn try_from(value: ChooseWire) -> Result<Self, Error> {
        let mut question = Self::from_options(
            value.question,
            value
                .choices
                .into_iter()
                .map(AnswerOption::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        )?;
        let threshold = value.threshold.admit()?;
        threshold.require_cut()?;
        question.field = admit_field(value.field)?;
        question.inner.threshold = threshold;
        Ok(question)
    }
}
impl<C: Choice> From<Choose<C>> for ChooseWire {
    fn from(value: Choose<C>) -> Self {
        Self {
            question: value.inner.text,
            choices: value.inner.options.into_iter().map(Into::into).collect(),
            threshold: value.inner.threshold.into(),
            field: value.field.map(|pointer| pointer.to_string()),
        }
    }
}

/// A validated tag question with its own options.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "TagWire", into = "TagWire")]
#[schemars(with = "TagWire")]
pub struct Tag<C: Choice = String> {
    #[serde(skip)]
    pub(super) choice: std::marker::PhantomData<C>,
    pub(crate) inner: crate::core::Tag,
    pub(super) field: Option<Pointer>,
}
impl<C: Choice> Tag<C> {
    /// Admit named answers with optional descriptions.
    /// # Errors
    /// Returns the shared question or domain validation error.
    pub(super) fn from_options(
        question: impl Into<String>,
        labels: impl IntoIterator<Item = AnswerOption>,
    ) -> Result<Self, Error> {
        let labels: Vec<_> = labels.into_iter().collect();
        choice::validate::<C>(&labels)?;
        Ok(Self {
            choice: std::marker::PhantomData,
            inner: crate::core::Tag::with_options(question, labels)?,
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
    pub(super) fn typed<D: Choice>(self) -> Result<Tag<D>, Error> {
        choice::validate::<D>(&self.inner.labels)?;
        Ok(Tag {
            inner: self.inner,
            field: self.field,
            choice: std::marker::PhantomData,
        })
    }
    pub(crate) fn erase(self) -> Tag {
        Tag {
            inner: self.inner,
            field: self.field,
            choice: std::marker::PhantomData,
        }
    }
}
impl Tag<String> {
    /// Admit explicit string labels and descriptions.
    /// # Errors
    /// Refuses invalid question text or label domains.
    pub fn with_options(
        question: impl Into<String>,
        values: impl IntoIterator<Item = AnswerOption>,
    ) -> Result<Self, Error> {
        Self::from_options(question, values)
    }

    /// Admit this function's required question and domain.
    /// # Errors
    /// Returns the shared question or name validation error.
    pub fn with_strings(
        question: impl Into<String>,
        labels: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, Error> {
        Self::from_options(
            question,
            labels
                .into_iter()
                .map(|value| AnswerOption::new(value.into()))
                .collect::<Result<Vec<_>, _>>()?,
        )
    }
}
impl<C: Choices> Tag<C> {
    /// Admit a question using this enum's declaration order.
    /// # Errors
    /// Refuses invalid question text or an invalid enum domain.
    pub fn new(question: impl Into<String>) -> Result<Self, Error> {
        Self::from_options(question, choice::options::<C>()?)
    }
    /// Describe enum members without changing their declaration order.
    #[must_use]
    pub fn descriptions(
        mut self,
        values: impl IntoIterator<Item = (C, impl Into<String>)>,
    ) -> Self {
        choice::descriptions(&mut self.inner.labels, values);
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
    #[schemars(with = "Vec<AnswerOption>")]
    labels: Vec<OptionWire>,
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
impl<C: Choice> TryFrom<TagWire> for Tag<C> {
    type Error = Error;
    fn try_from(value: TagWire) -> Result<Self, Error> {
        let mut question = Self::from_options(
            value.question,
            value
                .labels
                .into_iter()
                .map(AnswerOption::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        )?;
        let threshold = value.threshold.admit()?;
        threshold.require_cut()?;
        question.field = admit_field(value.field)?;
        question.inner.threshold = threshold;
        Ok(question)
    }
}
impl<C: Choice> From<Tag<C>> for TagWire {
    fn from(value: Tag<C>) -> Self {
        Self {
            question: value.inner.text,
            labels: value.inner.labels.into_iter().map(Into::into).collect(),
            threshold: value.inner.threshold.into(),
            field: value.field.map(|pointer| pointer.to_string()),
        }
    }
}

/// A validated score question with its own options.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "ScoreWire", into = "ScoreWire")]
#[schemars(with = "ScoreWire")]
pub struct Score<C: Choice = String> {
    #[serde(skip)]
    pub(super) choice: std::marker::PhantomData<C>,
    pub(crate) inner: crate::core::Score,
    pub(super) field: Option<Pointer>,
}
impl<C: Choice> Score<C> {
    /// Admit named answers with optional descriptions.
    /// # Errors
    /// Returns the shared question or domain validation error.
    pub(super) fn from_options(
        question: impl Into<String>,
        levels: impl IntoIterator<Item = AnswerOption>,
    ) -> Result<Self, Error> {
        let levels: Vec<_> = levels.into_iter().collect();
        choice::validate::<C>(&levels)?;
        Ok(Self {
            choice: std::marker::PhantomData,
            inner: crate::core::Score::with_options(question, levels)?,
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
    pub(super) fn typed<D: Choice>(self) -> Result<Score<D>, Error> {
        choice::validate::<D>(&self.inner.levels)?;
        Ok(Score {
            inner: self.inner,
            field: self.field,
            choice: std::marker::PhantomData,
        })
    }
    pub(crate) fn erase(self) -> Score {
        Score {
            inner: self.inner,
            field: self.field,
            choice: std::marker::PhantomData,
        }
    }
}
impl Score<String> {
    /// Admit explicit string labels and descriptions.
    /// # Errors
    /// Refuses invalid question text or label domains.
    pub fn with_options(
        question: impl Into<String>,
        values: impl IntoIterator<Item = AnswerOption>,
    ) -> Result<Self, Error> {
        Self::from_options(question, values)
    }

    /// Admit this function's required question and domain.
    /// # Errors
    /// Returns the shared question or name validation error.
    pub fn with_strings(
        question: impl Into<String>,
        levels: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, Error> {
        Self::from_options(
            question,
            levels
                .into_iter()
                .map(|value| AnswerOption::new(value.into()))
                .collect::<Result<Vec<_>, _>>()?,
        )
    }
}
impl<C: Choices> Score<C> {
    /// Admit a question using this enum's declaration order.
    /// # Errors
    /// Refuses invalid question text or an invalid enum domain.
    pub fn new(question: impl Into<String>) -> Result<Self, Error> {
        Self::from_options(question, choice::options::<C>()?)
    }
    /// Describe enum members without changing their declaration order.
    #[must_use]
    pub fn descriptions(
        mut self,
        values: impl IntoIterator<Item = (C, impl Into<String>)>,
    ) -> Self {
        choice::descriptions(&mut self.inner.levels, values);
        self
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ScoreWire {
    #[serde(deserialize_with = "crate::core::diagnostic::question")]
    question: String,
    #[schemars(length(min = MIN_LEVELS))]
    #[serde(deserialize_with = "crate::core::diagnostic::levels")]
    #[schemars(with = "Vec<AnswerOption>")]
    levels: Vec<OptionWire>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Pointer")]
    field: Option<String>,
}
impl<C: Choice> TryFrom<ScoreWire> for Score<C> {
    type Error = Error;
    fn try_from(value: ScoreWire) -> Result<Self, Error> {
        let mut question = Self::from_options(
            value.question,
            value
                .levels
                .into_iter()
                .map(AnswerOption::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        )?;
        question.field = admit_field(value.field)?;
        Ok(question)
    }
}
impl<C: Choice> From<Score<C>> for ScoreWire {
    fn from(value: Score<C>) -> Self {
        Self {
            question: value.inner.text,
            levels: value.inner.levels.into_iter().map(Into::into).collect(),
            field: value.field.map(|pointer| pointer.to_string()),
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
    pub(super) field: Option<Pointer>,
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
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Pointer")]
    field: Option<String>,
}
impl TryFrom<RecognizeWire> for Recognize {
    type Error = Error;
    fn try_from(value: RecognizeWire) -> Result<Self, Error> {
        let mut question = Self::new(value.kinds)?;
        let threshold = value.threshold.admit()?;
        threshold.require_cut()?;
        question.field = admit_field(value.field)?;
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
            field: value.field.map(|pointer| pointer.to_string()),
        }
    }
}
