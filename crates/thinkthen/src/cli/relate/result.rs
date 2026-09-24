use std::io::{ErrorKind, Write};

use serde::Serialize;

use crate::core::{
    Answer, AnswerOutcome, Backend, BackendFailure, Framing, Meta, ProfileWarning, QuestionMap,
    RelateQuestion, RelateSpec, RelationEdge, RelationEntity, RequestMeta, json_line, reaches_cut,
};
use crate::engine::facade::{Execution, Logical, Method};
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
struct Details<'a> {
    schema: &'static str,
    value: &'a [RelationEdge<RelationEntity>],
    question: RelateQuestion<'a>,
    answer: Answers<'a>,
    meta: Meta,
}

#[derive(Serialize)]
struct Answers<'a> {
    questions: Vec<Entry<'a>>,
}

/// One of the four ruled answer entries: choice or yes/no, answered or failed.
#[derive(Serialize)]
struct Entry<'a> {
    relation: &'a str,
    reads: &'a str,
    method: Method,
    direction: &'static str,
    #[serde(flatten)]
    body: Body<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure: Option<&'a BackendFailure>,
    request: &'a str,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Body<'a> {
    Choice {
        asker: Role<'a>,
        candidates: Vec<Candidate<'a>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pick: Option<Pick<'a>>,
    },
    Pair {
        source: &'a RelationEntity,
        target: &'a RelationEntity,
        #[serde(flatten)]
        judged: Option<Judged>,
    },
}

#[derive(Serialize)]
struct Role<'a> {
    role: &'static str,
    entity: &'a RelationEntity,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Candidate<'a> {
    Entity {
        #[serde(flatten)]
        role: Role<'a>,
        #[serde(flatten)]
        judged: Option<Judged>,
    },
    NoEntity {
        none: bool,
        #[serde(flatten)]
        judged: Option<Judged>,
    },
}

#[derive(Serialize)]
#[serde(untagged)]
enum Pick<'a> {
    Entity(Role<'a>),
    NoEntity { none: bool },
}

#[derive(Clone, Copy, Serialize)]
struct Judged {
    probability: f64,
    accepted: bool,
}

impl Judged {
    /// A real candidate or yes/no answer, accepted by the shared relation cut.
    fn at(probability: f64, threshold: f64) -> Self {
        Self {
            probability,
            accepted: reaches_cut(probability, threshold),
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
    let (method, body) = match &logical.mapping {
        QuestionMap::Choice { asker, options } => (
            Method::Choice,
            choice_body(entities, *asker, options, answer, threshold)?,
        ),
        QuestionMap::Pair { source, target } => {
            let probability = answer
                .map(|answer| {
                    answer
                        .yes()
                        .ok_or(Failure::Defect("a relation H answer has the wrong kind"))
                })
                .transpose()?;
            let body = Body::Pair {
                source: endpoint(entities, *source)?,
                target: endpoint(entities, *target)?,
                judged: probability.map(|probability| Judged::at(probability, threshold)),
            };
            (Method::YesNo, body)
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

/// The asker, every candidate, and the pick of one choice question.
fn choice_body<'a>(
    entities: &'a [RelationEntity],
    asker: usize,
    options: &'a [(String, usize, usize)],
    answer: Option<&Answer>,
    threshold: f64,
) -> Result<Body<'a>, Failure> {
    let probabilities = answer
        .map(|answer| {
            answer.choice_probabilities().ok_or(Failure::Defect(
                "a relation choice answer has the wrong kind",
            ))
        })
        .transpose()?;
    let chance = |label: &str| {
        probabilities.as_ref().map(|held| {
            held.iter()
                .find_map(|(name, value)| (*name == label).then_some(*value))
                .ok_or(Failure::Defect("a relation choice lost one probability"))
        })
    };
    let asks_as_source = options
        .first()
        .is_some_and(|(_, source, _)| *source == asker);
    let (asker_role, option_role) = if asks_as_source {
        ("source", "target")
    } else {
        ("target", "source")
    };
    let option = |source: usize, target: usize| -> Result<Role<'a>, Failure> {
        let place = if asks_as_source { target } else { source };
        Ok(Role {
            role: option_role,
            entity: endpoint(entities, place)?,
        })
    };
    let mut candidates = Vec::new();
    for (label, source, target) in options {
        candidates.push(Candidate::Entity {
            role: option(*source, *target)?,
            judged: chance(label)
                .transpose()?
                .map(|probability| Judged::at(probability, threshold)),
        });
    }
    let none = chance("none").transpose()?;
    candidates.push(Candidate::NoEntity {
        none: true,
        judged: none.map(|probability| Judged {
            probability,
            accepted: false,
        }),
    });
    let pick = match probabilities.as_ref().and_then(|held| leader(held)) {
        None => None,
        Some("none") => Some(Pick::NoEntity { none: true }),
        Some(label) => {
            let (_, source, target) = options
                .iter()
                .find(|(held, _, _)| held == label)
                .ok_or(Failure::Defect("a relation pick maps no entity"))?;
            Some(Pick::Entity(option(*source, *target)?))
        }
    };
    let asker = Role {
        role: asker_role,
        entity: endpoint(entities, asker)?,
    };
    Ok(Body::Choice {
        asker,
        candidates,
        pick,
    })
}

/// The first wire option with the highest probability, before any cut.
fn leader<'a>(probabilities: &[(&'a str, f64)]) -> Option<&'a str> {
    probabilities
        .iter()
        .copied()
        .reduce(|best, item| if item.1 > best.1 { item } else { best })
        .map(|(label, _)| label)
}

fn endpoint(entities: &[RelationEntity], place: usize) -> Result<&RelationEntity, Failure> {
    entities
        .get(place)
        .ok_or(Failure::Defect("a relation mapping names no entity"))
}
