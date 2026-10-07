//! Complete relation values and borrowed logical members.

use crate::core::{self, AnswerId, FailureId, ResultIdentity};
use crate::public::{
    Edge, Entity, Error, FailureCause, Probabilities, RelationDirection, RelationMethod, Usage,
};
use serde::{Serialize, Serializer};
use std::fmt;

/// A complete relation aggregate, retaining rejected and failed logical members.
#[derive(Clone, PartialEq)]
pub struct CompleteRelated {
    pub(crate) canonical: core::CompleteRelation,
    pub(crate) value: Vec<Edge>,
    pub(crate) source_edges: Option<Vec<super::SourceRelationEdge>>,
}

impl CompleteRelated {
    /// The actual resolved relation plan and input mode.
    #[must_use]
    pub fn question(&self) -> super::RelationReading<'_> {
        super::RelationReading {
            spec: &self.canonical.question,
            lines: self.canonical.lines,
        }
    }

    /// Borrow actual metadata without decoding a result document.
    #[must_use]
    pub fn meta(&self) -> super::ResultMetadata<'_> {
        super::ResultMetadata {
            fields: self.canonical.metadata(),
            identity: &self.canonical.identity,
        }
    }

    /// Stable identity of the resolved aggregate.
    #[must_use]
    pub const fn answer_id(&self) -> &AnswerId {
        self.canonical.identity.answer_id()
    }

    /// Accepted edges, in semantic order.
    #[must_use]
    pub fn value(&self) -> &[Edge] {
        &self.value
    }

    /// Expanded physical occurrence pairs, present only for explicit source input.
    #[must_use]
    pub fn source_edges(&self) -> Option<&[super::SourceRelationEdge]> {
        self.source_edges.as_deref()
    }

    pub(crate) fn serialize_input<S: Serializer, T: Serialize>(
        &self,
        input: Option<&T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match &self.source_edges {
            Some(edges) => self
                .canonical
                .serialize_with_value(input, edges, serializer),
            None => self.canonical.serialize_with_input(input, serializer),
        }
    }

    /// Aligned response sources and observation identities.
    #[must_use]
    pub const fn identity(&self) -> &ResultIdentity {
        &self.canonical.identity
    }

    /// Every relation question, including rejected and failed members.
    pub fn members(&self) -> impl ExactSizeIterator<Item = CompleteRelationMember<'_>> {
        self.canonical.members.iter().map(CompleteRelationMember)
    }

    /// Independently reported counts; omitted dimensions stay unknown.
    #[must_use]
    pub fn reported_usage(&self) -> Option<crate::public::ReportedUsage> {
        self.canonical.reported_usage()
    }

    /// Backend-reported usage, retaining absence.
    #[must_use]
    pub fn usage(&self) -> Option<Usage> {
        self.canonical.usage().map(super::usage)
    }

    /// Write the complete canonical result/2 document.
    ///
    /// # Errors
    /// Returns a defect if the typed result cannot be serialized.
    pub fn to_json(&self) -> Result<String, Error> {
        core::json_line(self)
            .map_err(|_| Error::defect("a complete relation result could not be written as JSON"))
    }
}

impl fmt::Debug for CompleteRelated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompleteRelated")
            .field("answer_id", self.answer_id())
            .finish_non_exhaustive()
    }
}

impl Serialize for CompleteRelated {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.serialize_input::<S, ()>(None, serializer)
    }
}

/// One borrowed relation reading, including full menu probabilities.
pub struct CompleteRelationMember<'a>(&'a core::CompleteRelationEntry);

impl CompleteRelationMember<'_> {
    /// Actual normalized primitive question, retained on failures too.
    #[must_use]
    pub fn question(&self) -> super::ResolvedQuestion<'_> {
        super::ResolvedQuestion(&self.0.question, None)
    }
    /// Admitted reading rule, independent of whether the member succeeded.
    #[must_use]
    pub fn threshold(&self) -> Option<super::ResolvedThreshold> {
        Some(super::ResolvedThreshold::of(self.0.threshold))
    }
    /// Actual ordered sources represented by this member.
    #[must_use]
    pub fn question_sources(&self) -> &[crate::public::QuestionSource] {
        &self.0.sources
    }
    /// Accepted observations or failed logical occurrences for this member.
    #[must_use]
    pub fn observations(&self) -> &[crate::public::Observation] {
        &self.0.observations
    }
    /// Actual partial token counts, without filling missing dimensions.
    #[must_use]
    pub fn reported_usage(&self) -> Option<crate::public::ReportedUsage> {
        self.0.reported_usage
    }

    /// Authored relation name.
    #[must_use]
    pub fn relation(&self) -> &str {
        &self.0.relation
    }

    /// Words read by the model for this rule.
    #[must_use]
    pub fn reads(&self) -> &str {
        &self.0.reads
    }

    /// Pair or target-menu question method.
    #[must_use]
    pub const fn method(&self) -> RelationMethod {
        self.0.method
    }

    /// Semantic direction of the rule.
    #[must_use]
    pub const fn direction(&self) -> RelationDirection {
        self.0.direction
    }

    /// Original source endpoint.
    #[must_use]
    pub fn source(&self) -> Entity {
        Entity::of(&self.0.source)
    }

    /// Chosen target, absent when none won or the menu failed.
    #[must_use]
    pub fn target(&self) -> Option<Entity> {
        self.0.target.as_ref().map(Entity::of)
    }

    /// Successful member identity, including a successful null target.
    #[must_use]
    pub fn answer_id(&self) -> Option<&AnswerId> {
        match &self.0.identity {
            core::MemberIdentity::Answered(id) => Some(id),
            core::MemberIdentity::Failed(_) => None,
        }
    }

    /// Failed occurrence identity, absent for a success.
    #[must_use]
    pub fn failure_id(&self) -> Option<&FailureId> {
        match &self.0.identity {
            core::MemberIdentity::Failed(id) => Some(id),
            core::MemberIdentity::Answered(_) => None,
        }
    }

    /// Full successful distribution, including every unchosen target.
    #[must_use]
    pub fn probabilities(&self) -> Option<Probabilities> {
        self.0.answer.as_ref().map(super::complete::probabilities)
    }

    /// Probability used by the existing relation selection rule.
    #[must_use]
    pub const fn probability(&self) -> Option<f64> {
        self.0.probability
    }

    /// Whether this successful reading selected an edge.
    #[must_use]
    pub const fn accepted(&self) -> Option<bool> {
        self.0.accepted
    }

    /// Existing backend member failure cause, absent on success.
    #[must_use]
    pub fn failure(&self) -> Option<FailureCause> {
        self.0.failure.as_ref().map(|failure| {
            crate::public::annotated::cause(core::FailedValue::new(*failure).cause())
        })
    }

    /// Saved question key for this logical member.
    #[must_use]
    pub fn request(&self) -> &str {
        &self.0.request
    }
}

impl fmt::Debug for CompleteRelationMember<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompleteRelationMember")
            .field("answer_id", &self.answer_id())
            .field("failure_id", &self.failure_id())
            .finish_non_exhaustive()
    }
}
