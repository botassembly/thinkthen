//! Borrowed command projections retain the native prepared bytes and rule order.
use super::PlanEstimate;
use crate::{Error, RelationMethod};
use std::fmt;

/// One exact native prepared request. Explicit access can reveal original evidence.
#[derive(Clone, Copy)]
pub struct PreparedRequest<'a> {
    /// Existing address-and-body digest, without recomputing the body.
    pub digest: &'a str,
    /// Exact encoded request bytes.
    pub body: &'a [u8],
}
impl fmt::Debug for PreparedRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PreparedRequest").field("bytes", &self.body.len()).finish_non_exhaustive()
    }
}

/// Recognition work disclosed for the first input record, matching native dry runs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecognitionPlan {
    /// Tokenizer pieces in the first record.
    pub pieces: usize,
    /// Native bound on later name questions.
    pub name_requests_upper_bound: usize,
    /// Native bound on later relation pairs and their requests, when applicable.
    pub relation_pairs_upper_bound: Option<usize>,
}

/// One native relation rule and its uninterrupted preview counts.
#[derive(Clone, Copy)]
pub struct PlannedRelation<'a> {
    /// Authored rule name.
    pub name: &'a str,
    /// Authored source kind.
    pub source: &'a str,
    /// Authored target kind.
    pub target: &'a str,
    /// Authored relation question.
    pub reads: &'a str,
    /// Whether the rule admits either direction.
    pub either: bool,
    /// Native question method.
    pub method: RelationMethod,
    /// Logical questions belonging to this rule.
    pub logical_questions: usize,
    /// Prepared requests containing a question from this rule.
    pub request_count: usize,
}
impl fmt::Debug for PlannedRelation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlannedRelation").field("method", &self.method)
            .field("logical_questions", &self.logical_questions)
            .field("request_count", &self.request_count).finish_non_exhaustive()
    }
}

/// Borrowed whole-set relation counts and rules in their authored order.
#[derive(Clone, Copy)]
pub struct RelationPlan<'a>(&'a RelationData);
impl fmt::Debug for RelationPlan<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RelationPlan").field("entities", &self.0.entities)
            .field("rules", &self.0.rules.len()).finish_non_exhaustive()
    }
}
impl RelationPlan<'_> {
    /// Distinct admitted entities, using the native whole-set projection.
    #[must_use]
    pub const fn entity_count(&self) -> usize { self.0.entities }
    /// Every relation rule, including rules with zero eligible questions.
    pub fn rules(&self) -> impl ExactSizeIterator<Item = PlannedRelation<'_>> {
        self.0.rules.iter().map(|rule| PlannedRelation {
            name: &rule.name, source: &rule.source, target: &rule.target, reads: &rule.reads,
            either: rule.either, method: rule.method, logical_questions: rule.logical_questions,
            request_count: rule.request_count,
        })
    }
}

#[derive(Clone, Default, Eq, PartialEq)]
pub(super) struct Details {
    requests: Vec<RequestData>,
    recognition: Option<RecognitionPlan>,
    relations: Option<RelationData>,
}
#[derive(Clone, Eq, PartialEq)]
struct RequestData { digest: String, body: Vec<u8> }
#[derive(Clone, Eq, PartialEq)]
struct RelationData { entities: usize, rules: Vec<RuleData> }
#[derive(Clone, Eq, PartialEq)]
struct RuleData {
    name: String, source: String, target: String, reads: String, either: bool,
    method: RelationMethod, logical_questions: usize, request_count: usize,
}
fn requests(rows: Vec<crate::engine::facade::Request>) -> Vec<RequestData> {
    rows.into_iter().map(|row| RequestData {
        digest: row.digest.as_str().to_owned(), body: row.body,
    }).collect()
}
impl PlanEstimate {
    /// Exact native request bodies retained by recognition or relation previews.
    /// Recognition retains the first record's requests; counts cover the full input.
    /// These explicit views can reveal authored questions and original evidence.
    pub fn prepared_requests(&self) -> impl ExactSizeIterator<Item = PreparedRequest<'_>> {
        self.details.requests.iter().map(|row| PreparedRequest {
            digest: &row.digest, body: &row.body,
        })
    }
    /// First-record recognition disclosure, absent for empty input or other verbs.
    #[must_use]
    pub const fn recognition(&self) -> Option<&RecognitionPlan> { self.details.recognition.as_ref() }
    /// Whole-set relation counts and authored rules, including empty input.
    #[must_use]
    pub fn relations(&self) -> Option<RelationPlan<'_>> { self.details.relations.as_ref().map(RelationPlan) }

    pub(in crate::public) fn from_recognition(
        preview: crate::public::complete::recognize::RecognitionPreview,
        dropped: bool,
        adapter: crate::core::adapters::ApiType,
    ) -> Result<Self, Error> {
        let mut plan = super::from_summary(preview.summary, adapter)?.with_request_details(dropped, None);
        if let Some(first) = preview.first {
            plan.details.recognition = Some(RecognitionPlan { pieces: first.pieces,
                name_requests_upper_bound: first.names, relation_pairs_upper_bound: first.relations });
            plan.details.requests = requests(first.requests);
        }
        Ok(plan)
    }
    pub(in crate::public) fn from_relations(
        preview: crate::public::complete::relate::RelationPreview,
        adapter: crate::core::adapters::ApiType,
    ) -> Result<Self, Error> {
        let mut plan = super::from_summary(preview.summary, adapter)?;
        let mut rules = Vec::new();
        for (at, rule) in preview.prepared.rules.into_iter().enumerate() {
            rules.push(RuleData { name: rule.name, source: rule.source, target: rule.target,
                reads: rule.reads, either: rule.either,
                method: if rule.single { RelationMethod::Choice } else { RelationMethod::YesNo },
                logical_questions: *preview.prepared.questions_per_rule.get(at)
                    .ok_or_else(|| Error::defect("a relation has no logical question count"))?,
                request_count: *preview.requests_per_rule.get(at)
                    .ok_or_else(|| Error::defect("a relation has no request count"))?,
            });
        }
        plan.details.relations = Some(RelationData { entities: preview.entities, rules });
        plan.details.requests = requests(preview.requests);
        Ok(plan)
    }
}
