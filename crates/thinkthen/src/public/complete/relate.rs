//! Complete relations keep actual logical successes and recoverable failures.
use super::{
    contextual,
    trace::{Totals, meta},
};
use crate::core::{
    self, AnswerOutcome, MemberIdentity, Observation, RelationDirection, RelationMethod,
};
use crate::engine::facade;
use crate::public::options::Stop;
use crate::public::{
    Call, CallOptions, CompleteRelated, Edge, Engine, Entity, Error, InputFunction, Relate,
};
use serde::Serialize;
use std::sync::Arc;
mod records;

impl Engine {
    /// Relate one whole set, retaining rejected and failed logical members.
    /// # Errors
    /// Invalid entities refuse before sending; all-failed calls and transport stops return final facts.
    pub fn relate_complete_with<I>(
        &self,
        ask: &Relate,
        entities: I,
        options: CallOptions<'_>,
    ) -> Result<Call<CompleteRelated>, Error>
    where
        I: IntoIterator<Item = Entity>,
    {
        let entities = admitted(ask, entities)?;
        self.relate_admitted_complete(ask, entities, options, false, &[])
    }

    fn relate_admitted_complete(
        &self,
        ask: &Relate,
        entities: Vec<core::RelationEntity>,
        options: CallOptions<'_>,
        lines: bool,
        inputs: &[Arc<crate::public::QuestionInput>],
    ) -> Result<Call<CompleteRelated>, Error> {
        let engine = contextual(self.for_model(ask.0.model.as_ref())?, &options)?;
        let prepared = facade::relations(&entities, &ask.0, engine.backend(), engine.profile())?;
        let threshold = ask.0.threshold.cut_value().unwrap_or(0.5);
        let stop = Stop::begin(options)?.with_prices(self.prices);
        stop.run_call(usize::from(!prepared.asks.is_empty()), |cancel| {
            let mut positions = [0; 4];
            let execution = engine
                .relate_observed(
                    prepared,
                    &entities,
                    threshold,
                    cancel,
                    |question, answered| {
                        crate::public::results::observe_question_inputs(
                            &stop,
                            engine.backend(),
                            (
                                InputFunction::Relate,
                                "relation",
                                question,
                                Some(ask.0.threshold),
                            ),
                            answered,
                            &mut positions,
                            (0, inputs),
                        )
                    },
                )
                .map_err(Error::from)?;
            if execution.failed > 0 && execution.answered == 0 {
                return Err(crate::public::asking::backend_failed());
            }
            let value = execution
                .edges
                .iter()
                .cloned()
                .map(Edge::from_native)
                .collect::<Result<Vec<_>, _>>()?;
            stop.observe(crate::public::RecordObservation::Row {
                index: 0,
                value: crate::public::ObservedRow::Relations(&value),
            });
            Ok((execution, value))
        })?
        .try_map(|(execution, value)| {
            let members = members(&execution.logical, &entities, threshold)?;
            let children = members
                .iter()
                .filter_map(|member| match &member.identity {
                    MemberIdentity::Answered(id) => Some(id.clone()),
                    MemberIdentity::Failed(_) => None,
                })
                .collect::<Vec<_>>();
            let identity = execution
                .trace
                .identity(
                    InputFunction::Relate,
                    &core::RecordScope { record: 0 },
                    &ask.0.question(lines),
                    &children,
                )
                .map_err(|_| super::wrong())?;
            let meta = meta(
                &engine,
                ask.0.question(lines).sha256().map_err(|_| super::wrong())?,
                Totals {
                    model: execution.model,
                    usage: execution.usage,
                    reported: execution.reported_usage,
                    cached: execution.replayed,
                    sent: execution.requests_sent,
                    requests: execution.requests,
                    failed: execution.failed,
                },
                &options,
                stop.facts().attempts().map(<[_]>::to_vec),
                ask.0.profile.as_ref(),
            );
            Ok(CompleteRelated {
                canonical: core::CompleteRelation {
                    identity,
                    value: execution.edges,
                    question: ask.0.clone(),
                    lines,
                    members,
                    meta,
                },
                value,
            })
        })
    }
}
fn members(
    logical: &[facade::Logical],
    entities: &[core::RelationEntity],
    threshold: f64,
) -> Result<Vec<core::CompleteRelationEntry>, Error> {
    logical
        .iter()
        .enumerate()
        .map(|(at, logical)| member(logical, entities, at, threshold))
        .collect()
}
#[derive(Serialize)]
struct Scope<'a> {
    record: usize,
    member: usize,
    source: &'a core::RelationEntity,
    target: Option<&'a core::RelationEntity>,
}

fn member(
    logical: &facade::Logical,
    entities: &[core::RelationEntity],
    at: usize,
    cut: f64,
) -> Result<core::CompleteRelationEntry, Error> {
    let answer = match &logical.outcome {
        AnswerOutcome::Answered(answer) => Some(answer),
        AnswerOutcome::Failed(_) => None,
    };
    let (method, source, target, probability, accepted) =
        endpoints(logical, entities, answer, cut)?;
    let scope = Scope {
        record: 0,
        member: at,
        source: &source,
        target: target.as_ref(),
    };
    let identity = match &logical.outcome {
        AnswerOutcome::Answered(_) => MemberIdentity::Answered(
            core::ResultIdentity::of(
                InputFunction::Relate,
                &scope,
                logical.answered.sources.clone(),
                logical.answered.observations.clone(),
                &core::AtomicReading {
                    question: &logical.question,
                    threshold: Some(core::Threshold::cut(cut).map_err(Error::refused)?),
                    rank_position: None,
                },
                &[],
            )
            .map_err(|_| super::wrong())?
            .answer_id()
            .clone(),
        ),
        AnswerOutcome::Failed(_) => MemberIdentity::Failed(
            logical
                .answered
                .observations
                .iter()
                .find_map(|observation| match observation {
                    Observation::Failed { failure_id } => Some(failure_id.clone()),
                    Observation::Answered { .. } => None,
                })
                .ok_or_else(super::wrong)?,
        ),
    };
    Ok(core::CompleteRelationEntry {
        identity,
        question: logical.question.clone(),
        threshold: core::Threshold::cut(cut).map_err(Error::refused)?,
        sources: logical.answered.sources.clone(),
        observations: logical.answered.observations.clone(),
        reported_usage: logical.answered.reply.reported_usage(),
        relation: logical.relation.name.clone(),
        reads: logical.relation.reads.clone(),
        method,
        direction: if logical.relation.either {
            RelationDirection::Either
        } else {
            RelationDirection::SourceToTarget
        },
        source,
        target,
        answer: answer.cloned(),
        probability,
        accepted,
        failure: match logical.outcome {
            AnswerOutcome::Failed(failure) => Some(failure),
            AnswerOutcome::Answered(_) => None,
        },
        request: logical.request.clone(),
    })
}

type Endpoints = (
    RelationMethod,
    core::RelationEntity,
    Option<core::RelationEntity>,
    Option<f64>,
    Option<bool>,
);
fn endpoints(
    logical: &facade::Logical,
    entities: &[core::RelationEntity],
    answer: Option<&core::Answer>,
    cut: f64,
) -> Result<Endpoints, Error> {
    let endpoint = |at| entities.get(at).cloned().ok_or_else(super::wrong);
    match &logical.asked {
        core::RelateAsk::Pair(pair) => {
            let probability = answer
                .map(|answer| answer.yes().ok_or_else(super::wrong))
                .transpose()?;
            Ok((
                RelationMethod::YesNo,
                endpoint(pair.source)?,
                Some(endpoint(pair.target)?),
                probability,
                probability.map(|p| core::reaches_cut(p, cut)),
            ))
        }
        core::RelateAsk::Menu(menu) => {
            let pick = answer
                .map(|answer| core::Pick::of(menu, answer).ok_or_else(super::wrong))
                .transpose()?;
            Ok((
                RelationMethod::Choice,
                endpoint(menu.source)?,
                pick.and_then(|pick| pick.target)
                    .map(endpoint)
                    .transpose()?,
                pick.map(|pick| pick.probability),
                pick.map(|pick| pick.accepted(cut)),
            ))
        }
    }
}

fn admitted(
    ask: &Relate,
    entities: impl IntoIterator<Item = Entity>,
) -> Result<Vec<core::RelationEntity>, Error> {
    let pairs = entities
        .into_iter()
        .take(256)
        .map(|entity| (entity.name().to_owned(), entity.kind().to_owned()))
        .collect::<Vec<_>>();
    ask.0.admit(&pairs).map_err(Error::refused)
}
