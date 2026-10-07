//! Complete relation details delegate to the shared native renderer.
#[cfg(test)]
use crate::core::RelateQuestion;
use crate::core::{self, Framing, json_line};
#[cfg(test)]
use crate::core::{BackendFailure, Meta, RelationEdge, RelationEntity};
use crate::engine::facade::{Engine, Execution};
use crate::failure::Failure;
#[cfg(test)]
use serde::Serialize;
use std::io::Write;
pub(super) struct Output<'a> {
    pub(super) details: bool,
    pub(super) framing: Framing,
    pub(super) spec: &'a core::RelateSpec,
    pub(super) entities: &'a [core::RelationEntity],
    pub(super) engine: &'a Engine,
}

#[cfg(test)]
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "relateDetails"))]
pub(crate) struct Details<'a> {
    schema: &'static str,
    value: &'a [RelationEdge<RelationEntity>],
    question: RelateQuestion<'a>,
    answer: Answers<'a>,
    meta: Meta,
}

#[cfg(test)]
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "relateAnswers"))]
struct Answers<'a> {
    questions: Vec<Entry<'a>>,
}

/// One yes/no pair or menu answer, or a recoverable failure.
#[cfg(test)]
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

#[cfg(test)]
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "relateBody"))]
struct Body<'a> {
    source: &'a RelationEntity,
    /// A menu's top candidate, or null when `none` wins or the menu failed.
    target: Option<&'a RelationEntity>,
    #[serde(flatten)]
    judged: Option<Judged>,
}

#[cfg(test)]
#[derive(Clone, Copy, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "relateJudged"))]
struct Judged {
    probability: f64,
    accepted: bool,
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
        Err(error) if !Failure::closed_output(&error) => Err(Failure::Output(error)),
        _ => Ok(()),
    }
}

pub(super) fn details(
    output: &Output<'_>,
    execution: &Execution,
) -> Result<core::CompleteRelation, Failure> {
    let mut events = std::collections::BTreeMap::new();
    for logical in &execution.logical {
        for event in &logical.answered.attempts {
            events.insert(event.ordinal(), event.clone());
        }
    }
    crate::result_json::complete::relation(
        output.engine,
        output.spec,
        output.entities,
        execution,
        crate::result_json::complete::RelationRow {
            lines: output.framing == Framing::Lines,
            context_sha256: None,
            attempts: Some(events.into_values().collect()),
        },
    )
    .map_err(|_| Failure::Defect("a complete relation result could not be constructed"))
}
