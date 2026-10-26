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
    pub(super) evidence: Option<Pointer>,
}
impl Annotate {
    /// Admit a nonempty named collection of per-record questions.
    /// # Errors
    /// Returns a shared question or name error.
    pub fn new(questions: BTreeMap<String, AnnotatableQuestion>) -> Result<Self, Error> {
        admit_names(&questions)?;
        Ok(Self {
            questions,
            evidence: None,
        })
    }
    /// Select shared evidence; individual entries can select their own evidence.
    #[must_use]
    pub fn evidence(mut self, pointer: Pointer) -> Self {
        self.evidence = Some(pointer);
        self
    }
    /// The admitted shared evidence pointer.
    #[must_use]
    pub fn evidence_pointer(&self) -> &Pointer {
        self.evidence
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
        deserialize_with = "deserialize_evidence"
    )]
    #[schemars(with = "Pointer")]
    evidence: Option<String>,
}
impl TryFrom<AnnotateWire> for Annotate {
    type Error = Error;
    fn try_from(value: AnnotateWire) -> Result<Self, Error> {
        let questions = value
            .questions
            .0
            .into_iter()
            .map(|(name, question)| Ok((name, question.try_into()?)))
            .collect::<Result<_, Error>>()?;
        let mut question = Self::new(questions)?;
        question.evidence = admit_evidence(value.evidence)?;
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
            evidence: value.evidence.map(|pointer| pointer.to_string()),
        }
    }
}
