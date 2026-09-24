//! Recognition's execution of the shared pure relation plans.

use super::{Aggregate, Running, execute};
use crate::core::{RecognizeSpec, RecognizedName, RelationEdge, assemble_edges, plan_relation};
use crate::failure::Failure;
use crate::prepared_request::SettledRelation;

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
            let settled = SettledRelation::settle(
                &running.backend,
                running.profile.as_ref(),
                Some(source),
                entities,
                planned,
            )?;
            prepared.push((settled.planned, settled.plan));
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
