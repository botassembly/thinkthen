//! Complete relation membership and provenance shared by native and command paths.
use super::aggregate::{Totals, meta};
use crate::core::image::InputFunction;
use crate::core::{
    self, AnswerOutcome, MemberIdentity, Observation, RelationDirection, RelationMethod,
};
use crate::engine::facade;
use serde::Serialize;
pub(crate) struct RelationRow {
    pub(crate) lines: bool,
    pub(crate) context_sha256: Option<String>,
    pub(crate) attempts: Option<Vec<core::AttemptObservation>>,
}
pub(crate) fn relation(
    engine: &facade::Engine,
    spec: &core::RelateSpec,
    entities: &[core::RelationEntity],
    execution: &facade::Execution,
    row: RelationRow,
) -> Result<core::CompleteRelation, core::RenderError> {
    let threshold = spec.threshold.cut_value().unwrap_or(0.5);
    let members = members(&execution.logical, entities, threshold)?;
    let children = members
        .iter()
        .filter_map(|member| match &member.identity {
            MemberIdentity::Answered(id) => Some(id.clone()),
            MemberIdentity::Failed(_) => None,
        })
        .collect::<Vec<_>>();
    let identity = execution.trace.identity(
        InputFunction::Relate,
        &core::RecordScope { record: 0 },
        &spec.question(row.lines),
        &children,
    )?;
    let meta = meta(
        engine,
        spec.question(row.lines)
            .sha256()
            .map_err(|_| core::RenderError)?,
        Totals {
            model: execution.model.clone(),
            usage: execution.usage,
            reported: execution.reported_usage,
            cached: execution.replayed,
            sent: execution.requests_sent,
            requests: execution.requests.clone(),
            failed: execution.failed,
        },
        row.context_sha256,
        row.attempts,
        spec.profile.as_ref(),
    );
    Ok(core::CompleteRelation {
        identity,
        value: execution.edges.clone(),
        question: spec.clone(),
        lines: row.lines,
        members,
        meta,
    })
}
fn members(
    logical: &[facade::Logical],
    entities: &[core::RelationEntity],
    threshold: f64,
) -> Result<Vec<core::CompleteRelationEntry>, core::RenderError> {
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
) -> Result<core::CompleteRelationEntry, core::RenderError> {
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
                    threshold: Some(core::Threshold::cut(cut).map_err(|_| core::RenderError)?),
                    rank_position: None,
                },
                &[],
            )
            .map_err(|_| core::RenderError)?
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
                .ok_or(core::RenderError)?,
        ),
    };
    Ok(core::CompleteRelationEntry {
        identity,
        question: logical.question.clone(),
        threshold: core::Threshold::cut(cut).map_err(|_| core::RenderError)?,
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
) -> Result<Endpoints, core::RenderError> {
    let endpoint = |at| entities.get(at).cloned().ok_or(core::RenderError);
    match &logical.asked {
        core::RelateAsk::Pair(pair) => {
            let probability = answer
                .map(|answer| answer.yes().ok_or(core::RenderError))
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
                .map(|answer| core::Pick::of(menu, answer).ok_or(core::RenderError))
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
