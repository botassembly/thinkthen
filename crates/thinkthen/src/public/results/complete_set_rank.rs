//! Complete set rank exposes actual ordered members and its turns winner.
use super::CompleteRank;
use crate::public::Error;
use serde::{Serialize, Serializer};
use std::fmt;

/// One saved member's ranked judgment for the original occurrence.
#[derive(Clone, PartialEq)]
pub struct CompleteRankMember {
    pub(crate) name: String,
    pub(crate) result: CompleteRank,
}
impl CompleteRankMember {
    /// The saved member key, distinct from optional authored question name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Complete member judgment with its stable position within that member.
    #[must_use]
    pub const fn result(&self) -> &CompleteRank {
        &self.result
    }
}
impl fmt::Debug for CompleteRankMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompleteRankMember")
            .field("result", &self.result)
            .finish_non_exhaustive()
    }
}
/// Final set rank, retaining every ordered member judgment for this original.
#[derive(Clone, PartialEq)]
pub struct CompleteSetRank {
    pub(crate) result: CompleteRank,
    pub(crate) question_name: String,
    pub(crate) members: Vec<CompleteRankMember>,
}
impl CompleteSetRank {
    /// One-based final turns position, before caller top truncation.
    #[must_use]
    pub const fn value(&self) -> usize {
        self.result.value()
    }
    /// The saved member key that first selected this original.
    #[must_use]
    pub fn question_name(&self) -> &str {
        &self.question_name
    }
    /// Winning judgment, with final position and aggregate actual member metadata/identity.
    #[must_use]
    pub const fn result(&self) -> &CompleteRank {
        &self.result
    }
    /// All member judgments in authored order, including visits skipped by turns.
    #[must_use]
    pub fn members(&self) -> &[CompleteRankMember] {
        &self.members
    }
    fn member_documents(&self) -> Vec<crate::core::RankMemberDocument<'_>> {
        self.members()
            .iter()
            .map(|member| crate::core::RankMemberDocument {
                name: member.name(),
                result: &member.result().canonical,
            })
            .collect()
    }
    /// Write the canonical result/2 rank document with every ordered member.
    /// # Errors
    /// Returns Defect when this concrete result cannot be serialized.
    pub fn to_json(&self) -> Result<String, Error> {
        crate::core::json_line(self)
            .map_err(|_| Error::defect("a complete set rank could not be written as JSON"))
    }
}
impl fmt::Debug for CompleteSetRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompleteSetRank")
            .field("result", &self.result)
            .field("members", &self.members.len())
            .finish_non_exhaustive()
    }
}
impl Serialize for CompleteSetRank {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.result
            .canonical
            .serialize_occurrence_members::<S, String>(
                None,
                Some(&self.question_name),
                None,
                Some(self.member_documents()),
                serializer,
            )
    }
}
impl<T: Serialize> Serialize for super::CompleteRecord<T, CompleteSetRank> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.result.result.canonical.serialize_occurrence_members(
            Some(&self.original),
            Some(&self.result.question_name),
            Some(self.ordinal),
            Some(self.result.member_documents()),
            serializer,
        )
    }
}
