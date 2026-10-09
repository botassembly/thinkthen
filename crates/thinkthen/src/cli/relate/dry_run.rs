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
    pub(super) shared_context: Option<&'a str>,
}

pub(super) fn write(
    writer: &mut dyn Write,
    context: Context<'_>,
    prepared: &PreparedRelations,
) -> Result<(), Failure> {
    let asks = match context.shared_context {
        Some(shared) => prepared
            .asks
            .clone()
            .with_context(context.backend, shared)?,
        None => prepared.asks.clone(),
    };
    let prepared_requests = asks.requests(
        context.backend,
        context.profile,
        crate::engine::facade::Bound::pairs(context.profile),
    )?;
    let mut summary = PlanSummary::new(true).with_accounting(context.backend.accounting());
    summary
        .records_added(context.entity_count)
        .map_err(|_| Failure::Defect("a plan is too large"))?;
    for request in &prepared_requests {
        summary
            .request(&request.body)
            .map_err(|_| Failure::Defect("a plan is too large"))?;
    }
    let relations = prepared
        .rules
        .iter()
        .zip(&prepared.questions_per_rule)
        .enumerate()
        .map(|(at, (rule, questions))| {
            Ok(Relation {
                name: &rule.name,
                source: &rule.source,
                target: &rule.target,
                reads: &rule.reads,
                either: rule.either,
                method: if rule.single { "choice" } else { "yes_no" },
                fallback: None,
                logical_questions: *questions,
                request_count: if context.shared_context.is_none() {
                    prepared
                        .requests_per_rule
                        .get(at)
                        .copied()
                        .ok_or(Failure::Defect("a relation has no request count"))?
                } else {
                    let belongs = |place: &usize| {
                        prepared
                            .asked
                            .get(*place)
                            .is_some_and(|asked| asked.rule() == at)
                    };
                    prepared_requests
                        .iter()
                        .filter(|request| request.places.iter().any(&belongs))
                        .count()
                },
            })
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    let requests = prepared_requests
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
