//! Complete relation details delegate to the shared native renderer.
#[cfg(test)]
use crate::core::RelateQuestion;
use crate::core::{self, json_line};
#[cfg(test)]
use crate::core::{BackendFailure, Meta, RelationEdge, RelationEntity};
use crate::failure::Failure;
use serde::{Serialize, Serializer};
use std::io::Write;
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
    details: bool,
    canonical: &core::CompleteRelation,
    originals: &[core::Record],
) -> Result<(), Failure> {
    let mut text = String::new();
    if details {
        text = json_line(&Complete {
            canonical,
            originals,
        })? + "\n";
    } else {
        for edge in &canonical.value {
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

struct Complete<'a> {
    canonical: &'a core::CompleteRelation,
    originals: &'a [core::Record],
}
impl Serialize for Complete<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.canonical.serialize_occurrence(
            Some(&self.originals),
            &self.canonical.value,
            Some(0),
            serializer,
        )
    }
}
