//! Relations between given entities over the shared relation planner and edge assembler.

use serde::Serialize;

use super::{Answered, Chunk, Engine};
use crate::core::{
    AnswerOutcome, Backend, BackendProfile, LimitKind, ModelName, Question, QuestionMap,
    RelateSpec, RelationEdge, RelationEntity, RelationPlan, RelationRule, Usage, assemble_edges,
    plan_relation,
};
use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::engine::prepared_request::SettledRelation;

/// How one concrete relation asks: one choice per asker, or one yes/no per pair.
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Method {
    Choice,
    YesNo,
}

/// One concrete relation, its question map, and the requests prepared for it.
pub(crate) struct PreparedRelation {
    pub(crate) relation: RelationRule,
    pub(crate) mappings: Vec<QuestionMap>,
    pub(crate) method: Method,
    pub(crate) fallback: Option<LimitKind>,
    pub(crate) chunks: Vec<Chunk>,
}

/// One logical question with the answer or failure the backend gave it.
pub(crate) struct Logical {
    pub(crate) relation: RelationRule,
    pub(crate) mapping: QuestionMap,
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

/// Plan and prepare every concrete relation before any engine exists.
pub(crate) fn relations(
    entities: &[RelationEntity],
    spec: &RelateSpec,
    backend: &Backend,
    profile: Option<&BackendProfile>,
) -> Result<Vec<PreparedRelation>, Error> {
    let mut prepared = Vec::new();
    for rule in &spec.relations {
        for planned in
            plan_relation(entities, rule).map_err(|_| Error::Defect("relation planning failed"))?
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
    chunks: Vec<Chunk>,
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

impl Engine {
    /// Send the prepared relations and draw the edges their answers reach.
    ///
    /// No usable logical answer at all fails the call; some failed answers
    /// stay in the result beside the good ones.
    pub(crate) fn relate(
        &self,
        prepared: Vec<PreparedRelation>,
        entities: &[RelationEntity],
        threshold: f64,
        cancel: &Cancel,
    ) -> Result<Execution, Error> {
        let mut execution = Execution {
            replayed: true,
            ..Execution::default()
        };
        let mut rules = Vec::with_capacity(prepared.len());
        let mut chunks = Vec::new();
        for (place, relation) in prepared.into_iter().enumerate() {
            chunks.extend(relation.chunks.into_iter().map(|chunk| (place, chunk)));
            rules.push((relation.relation, relation.mappings.into_iter()));
        }
        let (places, chunks): (Vec<_>, Vec<_>) = chunks.into_iter().unzip();
        let mut places = places.into_iter();
        self.ask_chunks(chunks, cancel, |answered| {
            let (rule, mappings) = places
                .next()
                .and_then(|place| rules.get_mut(place))
                .ok_or(Error::Defect("a relation reply has no relation"))?;
            add_meta(&mut execution, &answered)?;
            add_outcomes(&mut execution, entities, (rule, mappings), &answered, threshold)
        })?;
        if rules.iter_mut().any(|(_, mappings)| mappings.next().is_some()) {
            return Err(Error::Defect(
                "a relation reply did not cover its question map",
            ));
        }
        Ok(execution)
    }
}

/// Keep each logical answer one reply carries, in question-map order.
fn add_outcomes(
    execution: &mut Execution,
    entities: &[RelationEntity],
    (relation, mappings): (&RelationRule, &mut impl Iterator<Item = QuestionMap>),
    answered: &Answered,
    threshold: f64,
) -> Result<(), Error> {
    for outcome in answered.reply.outcomes() {
        let logical = Logical {
            relation: relation.clone(),
            mapping: mappings
                .next()
                .ok_or(Error::Defect("a relation reply exceeds its question map"))?,
            outcome: outcome.clone(),
            request: answered.request.as_str().to_owned(),
        };
        add_logical(execution, entities, logical, threshold);
    }
    Ok(())
}

/// Keep one logical answer and the edges the shared assembler draws from it.
fn add_logical(
    execution: &mut Execution,
    entities: &[RelationEntity],
    logical: Logical,
    threshold: f64,
) {
    if let AnswerOutcome::Answered(answer) = &logical.outcome {
        execution.answered += 1;
        execution.edges.extend(assemble_edges(
            entities,
            &logical.relation,
            std::slice::from_ref(&logical.mapping),
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
