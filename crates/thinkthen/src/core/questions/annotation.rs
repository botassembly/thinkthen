//! Per-record annotation excludes collection-wide ranking.
use super::collection::{RawNamed, admit_names};
use super::*;
use std::collections::BTreeMap;

/// Independently answer typed questions for one record.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "AnnotateWire", into = "AnnotateWire")]
#[schemars(with = "AnnotateWire")]
pub struct Annotate {
    questions: BTreeMap<String, AnnotatableQuestion>,
    pub(super) field: Option<Pointer>,
}
impl Annotate {
    /// Admit a nonempty named collection of per-record questions, such as an
    /// array of `(name, question)` pairs or a map.
    /// # Errors
    /// Returns a shared question or name error, including for a repeated name.
    pub fn new<N: Into<String>, Q: Into<AnnotatableQuestion>>(
        questions: impl IntoIterator<Item = (N, Q)>,
    ) -> Result<Self, Error> {
        let mut named = BTreeMap::new();
        for (name, question) in questions {
            let name = name.into();
            if named.contains_key(&name) {
                return Err(question_error(crate::core::diagnostic::requirement(
                    "name",
                    "a unique name",
                    "a repeated name",
                )));
            }
            named.insert(name, question.into());
        }
        admit_names(&named)?;
        Ok(Self {
            questions: named,
            field: None,
        })
    }
    /// Select shared field; individual entries can select their own field.
    #[must_use]
    pub fn field(mut self, pointer: Pointer) -> Self {
        self.field = Some(pointer);
        self
    }
    /// The admitted shared field pointer.
    #[must_use]
    pub fn field_pointer(&self) -> &Pointer {
        self.field
            .as_ref()
            .unwrap_or(&crate::core::record::ROOT_POINTER)
    }
    /// The immutable member questions.
    #[must_use]
    pub fn questions(&self) -> &BTreeMap<String, AnnotatableQuestion> {
        &self.questions
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct AnnotateWire {
    #[serde(deserialize_with = "crate::core::diagnostic::questions")]
    questions: RawNamed<RawAnnotatableQuestion>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_field"
    )]
    #[schemars(with = "Pointer")]
    field: Option<String>,
}
impl TryFrom<AnnotateWire> for Annotate {
    type Error = Error;
    fn try_from(value: AnnotateWire) -> Result<Self, Error> {
        let questions = value
            .questions
            .0
            .into_iter()
            .map(|(name, question)| Ok((name, question.try_into()?)))
            .collect::<Result<BTreeMap<String, AnnotatableQuestion>, Error>>()?;
        let mut question = Self::new(questions)?;
        question.field = admit_field(value.field)?;
        Ok(question)
    }
}
impl From<Annotate> for AnnotateWire {
    fn from(value: Annotate) -> Self {
        Self {
            questions: RawNamed(
                value
                    .questions
                    .into_iter()
                    .map(|(name, question)| (name, question.into()))
                    .collect(),
            ),
            field: value.field.map(|pointer| pointer.to_string()),
        }
    }
}
