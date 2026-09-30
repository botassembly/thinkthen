use std::io::{ErrorKind, Write};

use serde::Serialize;

use crate::core::{
    AnswerOutcome, Backend, BackendFailure, Framing, Meta, Pick, ProfileWarning, RelateAsk,
    RelateQuestion, RelateSpec, RelationEdge, RelationEntity, RequestMeta, json_line, reaches_cut,
};
use crate::engine::facade::{Execution, Logical};
use crate::failure::Failure;

pub(super) struct Output<'a> {
    pub(super) details: bool,
    pub(super) framing: Framing,
    pub(super) spec: &'a RelateSpec,
    pub(super) entities: &'a [RelationEntity],
    pub(super) backend: &'a Backend,
    pub(super) warning: Option<ProfileWarning>,
}

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "relateDetails"))]
pub(crate) struct Details<'a> {
    schema: &'static str,
    value: &'a [RelationEdge<RelationEntity>],
    question: RelateQuestion<'a>,
    answer: Answers<'a>,
    meta: Meta,
}

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "relateAnswers"))]
struct Answers<'a> {
    questions: Vec<Entry<'a>>,
}

/// One yes/no pair or menu answer, or a recoverable failure.
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "relateEntry"))]
struct Entry<'a> {
    relation: &'a str,
    reads: &'a str,
    method: &'static str,
    direction: &'static str,
    #[serde(flatten)]
    body: Body<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure: Option<&'a BackendFailure>,
    request: &'a str,
}

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "relateBody"))]
struct Body<'a> {
    source: &'a RelationEntity,
    /// A menu's top candidate, or null when `none` wins or the menu failed.
    target: Option<&'a RelationEntity>,
    #[serde(flatten)]
    judged: Option<Judged>,
}

#[derive(Clone, Copy, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "relateJudged"))]
struct Judged {
    probability: f64,
    accepted: bool,
}

impl Judged {
    /// A yes/no answer, accepted by the shared relation cut.
    fn at(probability: f64, threshold: f64) -> Self {
        Self {
            probability,
            accepted: reaches_cut(probability, threshold),
        }
    }

    /// A menu's top label, accepted only as a target at the cut.
    fn picked(pick: Pick, threshold: f64) -> Self {
        Self {
            probability: pick.probability,
            accepted: pick.accepted(threshold),
        }
    }
}

pub(super) fn write(
    writer: &mut dyn Write,
    output: &Output<'_>,
    execution: &Execution,
) -> Result<(), Failure> {
    let mut text = String::new();
    if output.details {
        text = json_line(&details(output, execution)?)? + "\n";
    } else {
        for edge in &execution.edges {
            text += &(json_line(edge)? + "\n");
        }
    }
    match writer
        .write_all(text.as_bytes())
        .and_then(|()| writer.flush())
    {
        Err(error) if error.kind() != ErrorKind::BrokenPipe => Err(Failure::Output(error)),
        _ => Ok(()),
    }
}

fn details<'a>(output: &Output<'a>, execution: &'a Execution) -> Result<Details<'a>, Failure> {
    let question = output.spec.question(output.framing == Framing::Lines);
    let meta = Meta::new(
        env!("CARGO_PKG_VERSION"),
        question.sha256()?,
        output.backend.url().clone(),
        execution
            .model
            .clone()
            .unwrap_or_else(|| output.backend.model().clone()),
        execution.usage,
        RequestMeta::new(
            execution.replayed,
            execution.requests_sent,
            execution.requests.clone(),
        )
        .with_failed_questions(execution.failed)
        .with_profile_warning(output.warning.clone()),
    );
    let threshold = output.spec.threshold.cut_value().unwrap_or(0.5);
    let questions = execution
        .logical
        .iter()
        .map(|logical| entry(logical, output.entities, threshold))
        .collect::<Result<_, _>>()?;
    Ok(Details {
        schema: crate::core::RESULT_SCHEMA,
        value: &execution.edges,
        question,
        answer: Answers { questions },
        meta,
    })
}

fn entry<'a>(
    logical: &'a Logical,
    entities: &'a [RelationEntity],
    threshold: f64,
) -> Result<Entry<'a>, Failure> {
    let (answer, failure) = match &logical.outcome {
        AnswerOutcome::Answered(answer) => (Some(answer), None),
        AnswerOutcome::Failed(failure) => (None, Some(failure)),
    };
    let wrong = || Failure::Defect("a relation answer has the wrong kind");
    let (method, body) = match &logical.asked {
        RelateAsk::Pair(pair) => {
            let probability = answer
                .map(|answer| answer.yes().ok_or_else(wrong))
                .transpose()?;
            let body = Body {
                source: endpoint(entities, pair.source)?,
                target: Some(endpoint(entities, pair.target)?),
                judged: probability.map(|probability| Judged::at(probability, threshold)),
            };
            ("yes_no", body)
        }
        RelateAsk::Menu(menu) => {
            let pick = answer
                .map(|answer| Pick::of(menu, answer).ok_or_else(wrong))
                .transpose()?;
            let target = pick
                .and_then(|pick| pick.target)
                .map(|place| endpoint(entities, place))
                .transpose()?;
            let body = Body {
                source: endpoint(entities, menu.source)?,
                target,
                judged: pick.map(|pick| Judged::picked(pick, threshold)),
            };
            ("choice", body)
        }
    };
    Ok(Entry {
        relation: &logical.relation.name,
        reads: &logical.relation.reads,
        method,
        direction: if logical.relation.either {
            "either"
        } else {
            "source_to_target"
        },
        body,
        failure,
        request: &logical.request,
    })
}

fn endpoint(entities: &[RelationEntity], place: usize) -> Result<&RelationEntity, Failure> {
    entities
        .get(place)
        .ok_or(Failure::Defect("a relation mapping names no entity"))
}
