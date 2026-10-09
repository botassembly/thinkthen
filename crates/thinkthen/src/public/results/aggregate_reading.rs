//! Borrowed resolved aggregate grammars; execution supplies all fields.
use super::{QuestionContent, ResolvedOption, ResolvedThreshold};
use crate::core;
use std::fmt;

/// One actual relation rule, preserving authored order and semantics.
#[derive(Clone, Copy)]
pub struct ResolvedRelationRule<'a>(&'a core::RelationRule);
impl ResolvedRelationRule<'_> {
    /// Authored name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.0.name
    }
    /// Source kind or explicit wildcard.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.0.source
    }
    /// Target kind or explicit wildcard.
    #[must_use]
    pub fn target(&self) -> &str {
        &self.0.target
    }
    /// Actual text read by the model.
    #[must_use]
    pub fn reads(&self) -> &str {
        &self.0.reads
    }
    /// True when the rule holds in either direction.
    #[must_use]
    pub const fn either(&self) -> bool {
        self.0.either
    }
    /// True when each source asks one complete target menu.
    #[must_use]
    pub const fn single(&self) -> bool {
        self.0.single
    }
}

/// Recognition's resolved kinds and separately scoped reading rules.
#[derive(Clone, Copy)]
pub struct RecognitionReading<'a>(pub(crate) &'a core::RecognizeSpec);
impl RecognitionReading<'_> {
    /// The resolved recognition stages.
    #[must_use]
    pub const fn mode(&self) -> crate::RecognitionMode {
        self.0.mode
    }
    /// Resolved literal stage overrides, including explicit empty strings.
    #[must_use]
    pub fn stage_context(&self) -> &crate::RecognitionStageContext {
        &self.0.stage_context
    }

    /// Caller task wording, absent for the compatibility default.
    #[must_use]
    pub fn instructions(&self) -> Option<&str> {
        self.0
            .instructions
            .as_ref()
            .and_then(|v| v.as_json().as_str())
    }
    /// Tokenizer pieces shown on each side of a token or span.
    #[must_use]
    pub fn snippet_pieces(&self) -> u32 {
        self.0.snippet_pieces
    }
    /// Borrow the authored entity definition.
    #[must_use]
    pub fn entity_definition(&self) -> Option<&str> {
        self.0
            .entity_definition
            .as_ref()
            .and_then(|v| v.as_json().as_str())
    }

    /// Explicit authored model, independent of accepted response provenance.
    #[must_use]
    pub fn model(&self) -> Option<&str> {
        self.0.model.as_ref().map(core::ModelName::as_str)
    }
    /// Original authored evidence pointers.
    pub fn on(&self) -> impl Iterator<Item = &str> {
        self.0.on.iter().map(core::Pointer::as_str)
    }

    /// Optional author name on the aggregate question, never its generated primitives.
    #[must_use]
    pub fn name(&self) -> Option<&crate::public::QuestionName> {
        self.0.metadata.name.as_ref()
    }
    /// Optional author wording version.
    #[must_use]
    pub fn wording_version(&self) -> Option<crate::public::WordingVersion> {
        self.0.metadata.wording_version
    }
    /// Optional selected item declaration.
    #[must_use]
    pub fn item_schema(&self) -> Option<&crate::public::InputDeclaration> {
        self.0.metadata.item_schema.as_ref()
    }
    /// Optional explicit per-item context declaration.
    #[must_use]
    pub fn context_schema(&self) -> Option<&crate::public::InputDeclaration> {
        self.0.metadata.context_schema.as_ref()
    }

    /// Original declared kinds and arbitrary descriptions, in authored order.
    pub fn kinds(&self) -> impl ExactSizeIterator<Item = ResolvedOption<'_>> {
        self.0
            .kinds
            .iter()
            .map(|(name, description)| ResolvedOption {
                name,
                description: description.as_ref().map(|d| QuestionContent(d.as_json())),
            })
    }
    /// Original declared relation rules, in authored order.
    pub fn relations(&self) -> impl ExactSizeIterator<Item = ResolvedRelationRule<'_>> {
        self.0.relations.iter().map(ResolvedRelationRule)
    }
    /// Effective name acceptance rule.
    #[must_use]
    pub const fn threshold(&self) -> ResolvedThreshold {
        ResolvedThreshold::of(self.0.threshold)
    }
    /// Effective relation acceptance rule.
    #[must_use]
    pub const fn relation_threshold(&self) -> ResolvedThreshold {
        ResolvedThreshold::of(self.0.relation_threshold)
    }
    /// Saved calibration profile, when named.
    #[must_use]
    pub fn profile(&self) -> Option<&str> {
        self.0.profile.as_ref().map(core::ProfileName::as_str)
    }
}

/// Relate's actual resolved plan and input reading mode.
#[derive(Clone, Copy)]
pub struct RelationReading<'a> {
    pub(crate) spec: &'a core::RelateSpec,
    pub(crate) lines: bool,
}
impl RelationReading<'_> {
    /// Explicit authored model, independent of accepted response provenance.
    #[must_use]
    pub fn model(&self) -> Option<&str> {
        self.spec.model.as_ref().map(core::ModelName::as_str)
    }

    /// Optional author name on the aggregate question, never its generated primitives.
    #[must_use]
    pub fn name(&self) -> Option<&crate::public::QuestionName> {
        self.spec.metadata.name.as_ref()
    }
    /// Optional author wording version.
    #[must_use]
    pub fn wording_version(&self) -> Option<crate::public::WordingVersion> {
        self.spec.metadata.wording_version
    }
    /// Optional selected item declaration.
    #[must_use]
    pub fn item_schema(&self) -> Option<&crate::public::InputDeclaration> {
        self.spec.metadata.item_schema.as_ref()
    }
    /// Optional explicit per-item context declaration.
    #[must_use]
    pub fn context_schema(&self) -> Option<&crate::public::InputDeclaration> {
        self.spec.metadata.context_schema.as_ref()
    }

    /// Declared rules, in original plan order.
    pub fn relations(&self) -> impl ExactSizeIterator<Item = ResolvedRelationRule<'_>> {
        self.spec.relations.iter().map(ResolvedRelationRule)
    }
    /// Effective edge acceptance rule.
    #[must_use]
    pub const fn threshold(&self) -> ResolvedThreshold {
        ResolvedThreshold::of(self.spec.threshold)
    }
    /// Name and kind pointers, absent for physical line entities.
    #[must_use]
    pub fn fields(&self) -> Option<(&str, &str)> {
        (!self.lines).then(|| {
            (
                self.spec.name_field().as_str(),
                self.spec.kind_field().as_str(),
            )
        })
    }
    /// Saved calibration profile, when named.
    #[must_use]
    pub fn profile(&self) -> Option<&str> {
        self.spec.profile.as_ref().map(core::ProfileName::as_str)
    }
}
macro_rules! withheld {
    ($($name:ident),+) => { $(impl fmt::Debug for $name<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct(stringify!($name)).finish_non_exhaustive() }
    })+ };
}
withheld!(ResolvedRelationRule, RecognitionReading, RelationReading);
