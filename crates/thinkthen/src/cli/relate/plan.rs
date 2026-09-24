use serde::Serialize;

use crate::core::{
    Backend, BackendProfile, LimitKind, Plan, Question, QuestionMap, RelateSpec, RelationEntity,
    RelationPlan, RelationRule, plan_pairs, plan_relation, relation_evidence,
};
use crate::engine::error::Error as EngineError;
use crate::failure::Failure;
use crate::prepared_request::{PreparedChunk, PreparedRequests};

#[derive(Clone, Copy, Debug, Serialize)] #[serde(rename_all = "snake_case")] #[rustfmt::skip]
pub(super) enum Method {
    Choice, YesNo,
}

#[derive(Clone, Copy, Debug, Serialize)] #[serde(rename_all = "snake_case")] #[rustfmt::skip]
pub(super) enum Fallback {
    MaxOptions, MaxRequestBytes,
}

#[rustfmt::skip]
pub(super) struct PreparedRelation {
    pub(super) relation: RelationRule, pub(super) mappings: Vec<QuestionMap>,
    pub(super) method: Method, pub(super) fallback: Option<Fallback>, pub(super) chunks: Vec<PreparedChunk>,
}

#[rustfmt::skip]
pub(super) fn prepare(
    entities: &[RelationEntity], spec: &RelateSpec, backend: &Backend, profile: Option<&BackendProfile>,
) -> Result<Vec<PreparedRelation>, Failure> {
    let mut prepared = Vec::new();
    for rule in &spec.relations {
        for relation in plan_relation(entities, rule)
            .map_err(|_| Failure::Defect("relation planning failed"))?
        {
            prepared.push(settle(entities, backend, profile, relation)?);
        }
    }
    Ok(prepared)
}

#[rustfmt::skip]
fn settle(
    entities: &[RelationEntity], backend: &Backend, profile: Option<&BackendProfile>, planned: RelationPlan,
) -> Result<PreparedRelation, Failure> {
    if planned.questions.is_empty() {
        let method = if planned.relation.source == planned.relation.target {
            Method::YesNo
        } else {
            Method::Choice
        };
        return Ok(PreparedRelation {
            relation: planned.relation,
            mappings: planned.mappings,
            method,
            fallback: None,
            chunks: Vec::new(),
        });
    }
    let plan = request_plan(entities, backend, &planned)?;
    match PreparedRequests::with_profile(backend, &plan, profile) {
        Ok(requests) => Ok(finished(planned, None, requests)),
        Err(EngineError::ProfileLimit(limit))
            if matches!(planned.questions.first(), Some(Question::Choose { .. }))
                && limit.permits_relation_fallback() =>
        {
            let fallback = match limit.kind {
                LimitKind::Options => Fallback::MaxOptions,
                LimitKind::RequestBytes => Fallback::MaxRequestBytes,
                LimitKind::EvidenceBytes | LimitKind::Questions => {
                    return Err(Failure::ProfileLimit(limit));
                }
            };
            let paired = plan_pairs(entities, &planned.relation)
                .map_err(|_| Failure::Defect("relation fallback planning failed"))?;
            let plan = request_plan(entities, backend, &paired)?;
            let requests = PreparedRequests::with_profile(backend, &plan, profile)?;
            Ok(finished(paired, Some(fallback), requests))
        }
        Err(error) => Err(error.into()),
    }
}

#[rustfmt::skip]
fn request_plan(
    entities: &[RelationEntity], backend: &Backend, planned: &RelationPlan,
) -> Result<Plan, Failure> {
    let evidence = relation_evidence(None, entities, &planned.relation)
        .map_err(|_| Failure::Defect("relation state could not be built"))?;
    Plan::new(evidence, backend.model().clone(), planned.questions.clone())
        .map_err(|_| Failure::Defect("relation planned no questions"))
}

#[rustfmt::skip]
fn finished(
    planned: RelationPlan, fallback: Option<Fallback>, requests: PreparedRequests,
) -> PreparedRelation {
    let method = if matches!(planned.questions.first(), Some(Question::Choose { .. })) {
        Method::Choice
    } else {
        Method::YesNo
    };
    PreparedRelation {
        relation: planned.relation,
        mappings: planned.mappings,
        method,
        fallback,
        chunks: requests.into_chunks(),
    }
}

impl PreparedRelation {
    pub(super) fn logical_questions(&self) -> usize {
        self.mappings.len()
    }
}
