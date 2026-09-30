//! Relations between given entities, planned as one ordered set of pair questions.

use super::each::{Models, summed};
use super::{Answered, Asks, Bound, Engine, Request};
use crate::core::{
    AnswerOutcome, Backend, BackendProfile, Lead, ModelName, Pair, Plan, Question, RelateSpec,
    RelationEdge, RelationEntity, RelationRule, Usage, pair_edges, plan_pairs,
};
use crate::engine::Cancel;
use crate::engine::error::Error;

/// All rules share these questions, in order, and the requests they make
/// with nothing cached.
pub(crate) struct PreparedRelations {
    pub(crate) rules: Vec<RelationRule>,
    pub(crate) pairs: Vec<Pair>,
    pub(crate) asks: Asks,
    pub(crate) requests: Vec<Request>,
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
            asks: Asks::default(),
            requests: Vec::new(),
        });
    };
    let mut asks = Asks::default();
    let plan = Plan::new(
        planned.evidence,
        backend.model().clone(),
        backend.descriptions(),
        planned.questions,
    )
    .map_err(|_| Error::Defect("relation planned no questions"))?;
    asks.add(backend, &plan)?;
    let requests = asks.requests(backend, profile, Bound::pairs(profile))?;
    let mut questions_per_rule = vec![0; rules.len()];
    for pair in &planned.pairs {
        *questions_per_rule
            .get_mut(pair.rule)
            .ok_or(Error::Defect("a pair names no rule"))? += 1;
    }
    let mut requests_per_rule = vec![0; rules.len()];
    for request in &requests {
        let mut previous = None;
        for place in &request.places {
            let pair = planned
                .pairs
                .get(*place)
                .ok_or(Error::Defect("a request exceeds its pairs"))?;
            if previous != Some(pair.rule) {
                *requests_per_rule
                    .get_mut(pair.rule)
                    .ok_or(Error::Defect("a pair names no rule"))? += 1;
                previous = Some(pair.rule);
            }
        }
    }
    Ok(PreparedRelations {
        rules,
        pairs: planned.pairs,
        asks,
        requests,
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
        self.relate_observed(prepared, entities, threshold, cancel, |_, _| Ok(()))
    }

    /// The same ordered relation execution, each answered question handed
    /// to `observe` with its logical question.
    pub(crate) fn relate_observed(
        &self,
        prepared: PreparedRelations,
        entities: &[RelationEntity],
        threshold: f64,
        cancel: &Cancel,
        mut observe: impl FnMut(&Question, &Answered) -> Result<(), Error>,
    ) -> Result<Execution, Error> {
        let mut execution = Execution {
            replayed: true,
            ..Execution::default()
        };
        let mut models = Models::default();
        let bound = Bound::pairs(self.profile());
        let questions = prepared.asks.questions();
        self.ask_each(&prepared.asks, bound, cancel, |place, answered| {
            let question = questions
                .get(place)
                .ok_or(Error::Defect("a relation reply exceeds its pairs"))?;
            observe(question, &answered)?;
            models.take(&answered, |held, model| match held {
                Some(held) if held != model => Err(Error::ModelsDiffer(None)),
                Some(_) => Ok(()),
                None => {
                    *held = Some(model.clone());
                    Ok(())
                }
            })?;
            add_meta(&mut execution, &answered)?;
            let pair = prepared
                .pairs
                .get(place)
                .ok_or(Error::Defect("a relation reply exceeds its pairs"))?;
            let relation = prepared
                .rules
                .get(pair.rule)
                .ok_or(Error::Defect("a pair names no rule"))?;
            for outcome in answered.reply.outcomes() {
                let logical = Logical {
                    relation: relation.clone(),
                    pair: *pair,
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
            Ok(())
        })?;
        execution.model = models.model().cloned();
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
    execution.usage = summed(execution.usage, answered.reply.usage())?;
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
