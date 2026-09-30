use std::io::Write;

use serde::Serialize;

use super::config::From;
use crate::core::{
    Backend, BackendProfile, Framing, PlanSummary, RelateFields, RelateSpec, json_line,
};
use crate::edge;
use crate::engine::facade::PreparedRelations;
use crate::failure::Failure;

#[derive(Serialize)]
struct Report<'a> {
    schema: &'static str,
    url: &'a str,
    model: &'a str,
    key_env: &'a str,
    backend_profile: Option<&'a str>,
    framing: Framing,
    fields: Option<&'a RelateFields>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<From>,
    entity_count: usize,
    relations: Vec<Relation<'a>>,
    logical_questions: usize,
    request_count: usize,
    requests: Vec<Request<'a>>,
}

#[derive(Serialize)]
struct Relation<'a> {
    name: &'a str,
    source: &'a str,
    target: &'a str,
    reads: &'a str,
    either: bool,
    method: &'static str,
    fallback: Option<&'static str>,
    logical_questions: usize,
    request_count: usize,
}

#[derive(Serialize)]
struct Request<'a> {
    digest: &'a str,
    bytes: usize,
    body_utf8: &'a str,
}

/// What one plan reports beside the prepared relations.
pub(super) struct Context<'a> {
    pub(super) backend: &'a Backend,
    pub(super) profile: Option<&'a BackendProfile>,
    pub(super) framing: Framing,
    pub(super) spec: &'a RelateSpec,
    pub(super) from: Option<From>,
    pub(super) entity_count: usize,
    pub(super) key_env: &'a str,
}

pub(super) fn write(
    writer: &mut dyn Write,
    context: Context<'_>,
    prepared: &PreparedRelations,
) -> Result<(), Failure> {
    let mut summary = PlanSummary::new(true);
    summary
        .records_added(context.entity_count)
        .map_err(|_| Failure::Defect("a plan is too large"))?;
    for request in &prepared.requests {
        summary
            .request(&request.body)
            .map_err(|_| Failure::Defect("a plan is too large"))?;
    }
    let relations = prepared
        .rules
        .iter()
        .zip(&prepared.questions_per_rule)
        .zip(&prepared.requests_per_rule)
        .map(|((rule, questions), requests)| Relation {
            name: &rule.name,
            source: &rule.source,
            target: &rule.target,
            reads: &rule.reads,
            either: rule.either,
            method: "yes_no",
            fallback: None,
            logical_questions: *questions,
            request_count: *requests,
        })
        .collect::<Vec<_>>();
    let requests = prepared
        .requests
        .iter()
        .map(|request| {
            Ok(Request {
                digest: request.digest.as_str(),
                bytes: request.body.len(),
                body_utf8: str::from_utf8(&request.body)
                    .map_err(|_| Failure::Defect("an encoded request is not UTF-8"))?,
            })
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    let report = Report {
        schema: "thinkthen.relate-plan/1",
        url: context.backend.url().as_str(),
        model: context.backend.model().as_str(),
        key_env: context.key_env,
        backend_profile: context.profile.map(|profile| profile.name().as_str()),
        framing: context.framing,
        fields: (context.framing != Framing::Lines).then_some(context.spec.fields()),
        from: context.from,
        entity_count: context.entity_count,
        logical_questions: relations.iter().map(|item| item.logical_questions).sum(),
        relations,
        request_count: requests.len(),
        requests,
    };
    edge::write_line(&mut *writer, &json_line(&report)?)?;
    let counts = summary
        .counts()
        .map_err(|_| Failure::Defect("a plan is too large"))?;
    edge::write_line(writer, &json_line(&counts)?)?;
    Ok(())
}
