//! Recognition's execution of the shared pure relation plans.

use super::{Aggregate, Running, execute};
use crate::core::{
    Question, RecognizeSpec, RecognizedName, RelationEdge, RelationPlan, assemble_edges,
    plan_pairs, plan_relation, relation_evidence,
};
use crate::engine::error::Error as EngineError;
use crate::failure::Failure;
use crate::prepared_request::PreparedRequests;

pub(super) fn recognize(
    running: &Running<'_>,
    spec: &RecognizeSpec,
    source: &str,
    entities: &[RecognizedName],
    aggregate: &mut Aggregate,
) -> Result<Option<Vec<RelationEdge<RecognizedName>>>, Failure> {
    if spec.relations.is_empty() {
        return Ok(None);
    }
    let mut prepared = Vec::new();
    for rule in &spec.relations {
        let plans = plan_relation(entities, rule)
            .map_err(|_| Failure::Defect("relation planning failed"))?;
        for planned in plans {
            if planned.questions.is_empty() {
                continue;
            }
            let planned = settle_fallback(running, source, entities, planned)?;
            let request_plan = request_plan(running, source, entities, &planned)?;
            PreparedRequests::with_profile(
                &running.backend,
                &request_plan,
                running.profile.as_ref(),
            )?;
            prepared.push((planned, request_plan));
        }
    }
    let mut edges = Vec::new();
    for (planned, request_plan) in prepared {
        let (answers, meta) = execute(running, &request_plan)?;
        aggregate.add(meta)?;
        edges.extend(assemble_edges(
            entities,
            &planned.relation,
            &planned.mappings,
            &answers,
            spec.relation_threshold.cut_value().unwrap_or(0.5),
        ));
    }
    Ok(Some(edges))
}

fn settle_fallback(
    running: &Running<'_>,
    source: &str,
    entities: &[RecognizedName],
    planned: RelationPlan,
) -> Result<RelationPlan, Failure> {
    let request_plan = request_plan(running, source, entities, &planned)?;
    match PreparedRequests::with_profile(&running.backend, &request_plan, running.profile.as_ref())
    {
        Ok(_) => Ok(planned),
        Err(EngineError::ProfileLimit(limit))
            if matches!(planned.questions.first(), Some(Question::Choose { .. }))
                && limit.permits_relation_fallback() =>
        {
            plan_pairs(entities, &planned.relation)
                .map_err(|_| Failure::Defect("relation fallback planning failed"))
        }
        Err(error) => Err(error.into()),
    }
}

fn request_plan(
    running: &Running<'_>,
    source: &str,
    entities: &[RecognizedName],
    planned: &RelationPlan,
) -> Result<crate::core::Plan, Failure> {
    let evidence = relation_evidence(Some(source), entities, &planned.relation)
        .map_err(|_| Failure::Defect("relation state could not be built"))?;
    crate::core::Plan::new(
        evidence,
        running.backend.model().clone(),
        planned.questions.clone(),
    )
    .map_err(|_| Failure::Defect("relation planned no questions"))
}
