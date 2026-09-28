//! Relations between given entities, planned as one ordered set of pair questions.

use super::{Answered, Chunk, Engine};
use crate::core::{
    AnswerOutcome, Backend, BackendProfile, Lead, ModelName, Pair, RelateSpec, RelationEdge,
    RelationEntity, RelationRule, Usage, pair_edges, plan_pairs,
};
use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::engine::prepared_request::pair_chunks;

/// All rules share these prepared requests, in question order.
pub(crate) struct PreparedRelations {
    pub(crate) rules: Vec<RelationRule>,
    pub(crate) pairs: Vec<Pair>,
    pub(crate) chunks: Vec<Chunk>,
    pub(crate) questions_per_rule: Vec<usize>,
    pub(crate) requests_per_rule: Vec<usize>,
}

/// One logical pair answer, including a recoverable failure.
pub(crate) struct Logical {
    pub(crate) relation: RelationRule,
    pub(crate) pair: Pair,
    pub(crate) outcome: AnswerOutcome,
    pub(crate) request: String,
}

/// Every logical answer, the edges drawn from them, and the metadata.
#[derive(Default)]
pub(crate) struct Execution {
    pub(crate) edges: Vec<RelationEdge<RelationEntity>>,
    pub(crate) logical: Vec<Logical>,
    pub(crate) model: Option<ModelName>,
    pub(crate) usage: Option<Usage>,
    pub(crate) replayed: bool,
    pub(crate) requests_sent: u64,
    pub(crate) requests: Vec<String>,
    pub(crate) failed: usize,
    pub(crate) answered: usize,
}

/// Plan every rule before any engine exists, then prepare the shared requests.
pub(crate) fn relations(
    entities: &[RelationEntity],
    spec: &RelateSpec,
    backend: &Backend,
    profile: Option<&BackendProfile>,
) -> Result<PreparedRelations, Error> {
    let rules = spec.relations.clone();
    let Some(planned) = plan_pairs(None, entities, &rules, Lead::Known)
        .map_err(|_| Error::Defect("relation planning failed"))?
    else {
        return Ok(PreparedRelations {
            questions_per_rule: vec![0; rules.len()],
            requests_per_rule: vec![0; rules.len()],
            rules,
            pairs: Vec::new(),
            chunks: Vec::new(),
        });
    };
    let chunks = pair_chunks(backend, profile, &planned)?;
    let mut questions_per_rule = vec![0; rules.len()];
    for pair in &planned.pairs {
        *questions_per_rule
            .get_mut(pair.rule)
            .ok_or(Error::Defect("a pair names no rule"))? += 1;
    }
    let mut requests_per_rule = vec![0; rules.len()];
    let mut start = 0;
    for chunk in &chunks {
        let end = start + chunk.plan.questions().len();
        let mut previous = None;
        for pair in planned
            .pairs
            .get(start..end)
            .ok_or(Error::Defect("a chunk exceeds its pairs"))?
        {
            if previous != Some(pair.rule) {
                *requests_per_rule
                    .get_mut(pair.rule)
                    .ok_or(Error::Defect("a pair names no rule"))? += 1;
                previous = Some(pair.rule);
            }
        }
        start = end;
    }
    if start != planned.pairs.len() {
        return Err(Error::Defect("a pair has no prepared request"));
    }
    Ok(PreparedRelations {
        rules,
        pairs: planned.pairs,
        chunks,
        questions_per_rule,
        requests_per_rule,
    })
}

impl Engine {
    /// Send shared requests and retain recoverable logical failures.
    pub(crate) fn relate(
        &self,
        prepared: PreparedRelations,
        entities: &[RelationEntity],
        threshold: f64,
        cancel: &Cancel,
    ) -> Result<Execution, Error> {
        let mut execution = Execution {
            replayed: true,
            ..Execution::default()
        };
        if prepared.chunks.is_empty() {
            return Ok(execution);
        }
        let mut pairs = prepared.pairs.into_iter();
        self.ask_chunks(prepared.chunks, cancel, |answered| {
            add_meta(&mut execution, &answered)?;
            for outcome in answered.reply.outcomes() {
                let pair = pairs
                    .next()
                    .ok_or(Error::Defect("a relation reply exceeds its pairs"))?;
                let relation = prepared
                    .rules
                    .get(pair.rule)
                    .ok_or(Error::Defect("a pair names no rule"))?;
                let logical = Logical {
                    relation: relation.clone(),
                    pair,
                    outcome: outcome.clone(),
                    request: answered.request.as_str().to_owned(),
                };
                add_logical(
                    &mut execution,
                    entities,
                    &prepared.rules,
                    logical,
                    threshold,
                );
            }
            Ok::<(), Error>(())
        })?;
        if pairs.next().is_some() {
            return Err(Error::Defect("a relation reply did not cover its pairs"));
        }
        Ok(execution)
    }
}

fn add_logical(
    execution: &mut Execution,
    entities: &[RelationEntity],
    rules: &[RelationRule],
    logical: Logical,
    threshold: f64,
) {
    if let AnswerOutcome::Answered(answer) = &logical.outcome {
        execution.answered += 1;
        execution.edges.extend(pair_edges(
            entities,
            rules,
            std::slice::from_ref(&logical.pair),
            std::slice::from_ref(answer),
            threshold,
        ));
    } else {
        execution.failed += 1;
    }
    execution.logical.push(logical);
}

fn add_meta(execution: &mut Execution, answered: &Answered) -> Result<(), Error> {
    let model = answered.reply.model();
    if execution.model.as_ref().is_some_and(|held| held != model) {
        return Err(Error::ModelsDiffer(None));
    }
    execution.model.get_or_insert_with(|| model.clone());
    execution.usage = match (execution.usage, answered.reply.usage()) {
        (Some(left), Some(right)) => Some(left.checked_plus(right).ok_or(Error::UsageOverflow)?),
        (None, held) | (held, None) => held,
    };
    execution.replayed &= answered.replayed;
    execution.requests_sent = execution
        .requests_sent
        .checked_add(answered.requests_sent)
        .ok_or(Error::UsageOverflow)?;
    execution
        .requests
        .push(answered.request.as_str().to_owned());
    Ok(())
}
