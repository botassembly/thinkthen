//! Typed relation rules retain the engine's validated wording and endpoints.
use super::*;

/// A validated relation rule.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "RuleWire", into = "RuleWire")]
#[schemars(with = "RuleWire")]
pub struct RelationRule(crate::core::RelationRule);
impl From<crate::core::RelationRule> for RelationRule {
    fn from(value: crate::core::RelationRule) -> Self {
        Self(value)
    }
}
impl RelationRule {
    /// Admit a relation and its endpoint kinds.
    /// # Errors
    /// Returns the shared rule admission error.
    pub fn new(
        relation: impl Into<String>,
        source: impl Into<String>,
        target: impl Into<String>,
    ) -> Result<Self, Error> {
        crate::core::RelationRule::new(relation, source, target).map(Self)
    }
    /// Supply explicit relation wording.
    /// # Errors
    /// Returns a question error for blank wording.
    pub fn reads(self, text: impl Into<String>) -> Result<Self, Error> {
        self.0.reads(text).map(Self)
    }
    /// Select an unordered relation.
    #[must_use]
    pub fn either(self, value: bool) -> Self {
        Self(self.0.either(value))
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct RuleWire {
    #[serde(deserialize_with = "crate::core::diagnostic::relation")]
    relation: String,
    #[serde(deserialize_with = "crate::core::diagnostic::source")]
    source: String,
    #[serde(deserialize_with = "crate::core::diagnostic::target")]
    target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::core::diagnostic::reads")]
    reads: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "crate::core::diagnostic::either")]
    either: bool,
}
impl TryFrom<RuleWire> for RelationRule {
    type Error = Error;
    fn try_from(value: RuleWire) -> Result<Self, Error> {
        let rule = Self::new(value.relation, value.source, value.target)?.either(value.either);
        match value.reads {
            Some(reads) => rule.reads(reads),
            None => Ok(rule),
        }
    }
}
impl From<RelationRule> for RuleWire {
    fn from(value: RelationRule) -> Self {
        Self {
            relation: value.0.name,
            source: value.0.source,
            target: value.0.target,
            reads: Some(value.0.reads),
            either: value.0.either,
        }
    }
}

/// A relation question with a required domain and a numeric cut.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "RelateWire", into = "RelateWire")]
#[schemars(with = "RelateWire")]
pub struct Relate {
    pub(crate) inner: crate::core::Relate,
    pub(super) evidence: Option<Pointer>,
}
impl Relate {
    /// Admit a nonempty list of typed rules.
    /// # Errors
    /// Returns the shared rule-domain error.
    pub fn new(rules: impl IntoIterator<Item = RelationRule>) -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::Relate::new(rules.into_iter().map(|rule| rule.0).collect())?,
            evidence: None,
        })
    }
    /// Set a numeric cut.
    #[must_use]
    pub fn threshold(mut self, cut: Cut) -> Self {
        self.inner.threshold = cut.into();
        self
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
pub(crate) struct RelateWire {
    #[schemars(length(min = MIN_RULES))]
    #[serde(deserialize_with = "crate::core::diagnostic::rules")]
    rules: Vec<RuleWire>,
    #[serde(default)]
    #[schemars(with = "Cut")]
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
impl TryFrom<RelateWire> for Relate {
    type Error = Error;
    fn try_from(value: RelateWire) -> Result<Self, Error> {
        let threshold = value.threshold.admit()?;
        threshold.require_cut()?;
        let mut question = Self::new(
            value
                .rules
                .into_iter()
                .map(RelationRule::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        )?;
        question.evidence = admit_evidence(value.evidence)?;
        question.inner.threshold = threshold;
        Ok(question)
    }
}
impl From<Relate> for RelateWire {
    fn from(value: Relate) -> Self {
        Self {
            rules: value
                .inner
                .rules
                .into_iter()
                .map(RelationRule)
                .map(Into::into)
                .collect(),
            threshold: value.inner.threshold.into(),
            evidence: value.evidence.map(|pointer| pointer.to_string()),
        }
    }
}
