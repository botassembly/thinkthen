use serde::Serialize;

use crate::core::{
    Backend, BackendProfile, LimitKind, Question, QuestionMap, RelateSpec, RelationEntity,
    RelationPlan, RelationRule, plan_relation,
};
use crate::failure::Failure;
use crate::prepared_request::{PreparedChunk, SettledRelation};

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Method {
    Choice,
    YesNo,
}

pub(super) struct PreparedRelation {
    pub(super) relation: RelationRule,
    pub(super) mappings: Vec<QuestionMap>,
    pub(super) method: Method,
    pub(super) fallback: Option<LimitKind>,
    pub(super) chunks: Vec<PreparedChunk>,
}

pub(super) fn prepare(
    entities: &[RelationEntity],
    spec: &RelateSpec,
    backend: &Backend,
    profile: Option<&BackendProfile>,
) -> Result<Vec<PreparedRelation>, Failure> {
    let mut prepared = Vec::new();
    for rule in &spec.relations {
        for planned in plan_relation(entities, rule)
            .map_err(|_| Failure::Defect("relation planning failed"))?
        {
            if planned.questions.is_empty() {
                prepared.push(finished(planned, None, Vec::new()));
                continue;
            }
            let settled = SettledRelation::settle(backend, profile, None, entities, planned)?;
            let chunks = settled.requests.into_chunks();
            prepared.push(finished(settled.planned, settled.fallback, chunks));
        }
    }
    Ok(prepared)
}

fn finished(
    planned: RelationPlan,
    fallback: Option<LimitKind>,
    chunks: Vec<PreparedChunk>,
) -> PreparedRelation {
    let choice = match planned.questions.first() {
        Some(question) => matches!(question, Question::Choose { .. }),
        None => planned.relation.source != planned.relation.target,
    };
    PreparedRelation {
        relation: planned.relation,
        mappings: planned.mappings,
        method: if choice {
            Method::Choice
        } else {
            Method::YesNo
        },
        fallback,
        chunks,
    }
}
