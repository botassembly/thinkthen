use std::io::Write;

use serde::Serialize;

use super::config::From;
use crate::{ResolvedSettings, RequestFraming as Framing};
use super::config::Fields;
use crate::render::json_line;
use crate::edge;
use crate::failure::Failure;

#[derive(Serialize)]
struct Report<'a> {
    schema: &'static str,
    url: &'a str,
    model: &'a str,
    key_env: &'a str,
    backend_profile: Option<&'a str>,
    framing: Framing,
    fields: Option<&'a Fields>,
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
    pub(super) backend: &'a ResolvedSettings,
    pub(super) profile: Option<&'a str>,
    pub(super) framing: Framing,
    pub(super) fields: &'a Fields,
    pub(super) from: Option<From>,
    pub(super) key_env: &'a str,
    pub(super) shared_context: Option<&'a str>,
}

pub(super) fn write(
    writer: &mut dyn Write,
    context: Context<'_>,
    admitted: crate::AdmittedRequest,
    (originals, sources): (&[crate::RawRecord], Option<&super::source::Sources>),
    engine: &crate::Engine,
) -> Result<(), Failure> {
    let request = admitted;
    let mut controls = crate::CallOptions::new().surface(crate::Surface::Cli);
    if let Some(shared) = context.shared_context {
        controls = controls.context(shared);
    }
    let rows = super::records(originals, sources)?;
    let preview = engine.plan_request(&request, crate::RequestEnvironment {
        controls, feed: Some(crate::RequestFeed::from_records("cli-relate", rows)),
    })?;
    let prepared = preview.relations().ok_or(Failure::Defect("a relation plan has no rules"))?;
    let relations = prepared.rules().map(|rule| Relation {
        name: rule.name, source: rule.source, target: rule.target, reads: rule.reads,
        either: rule.either, method: match rule.method {
            crate::RelationMethod::Choice => "choice", crate::RelationMethod::YesNo => "yes_no",
        }, fallback: None, logical_questions: rule.logical_questions, request_count: rule.request_count,
    }).collect::<Vec<_>>();
    let requests = preview.prepared_requests()
        .map(|request| {
            Ok(Request {
                digest: request.digest,
                bytes: request.body.len(),
                body_utf8: str::from_utf8(request.body)
                    .map_err(|_| Failure::Defect("an encoded request is not UTF-8"))?,
            })
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    let report = Report {
        schema: "thinkthen.relate-plan/1",
        url: &context.backend.url,
        model: &context.backend.model,
        key_env: context.key_env,
        backend_profile: context.profile,
        framing: context.framing,
        fields: (context.framing != Framing::Lines).then_some(context.fields),
        from: context.from,
        entity_count: prepared.entity_count(),
        logical_questions: relations.iter().map(|item| item.logical_questions).sum(),
        relations,
        request_count: requests.len(),
        requests,
    };
    edge::write_line(&mut *writer, &json_line(&report)?)?;
    let counts = crate::cli::plan_counts::Counts::from(&preview);
    edge::write_line(writer, &json_line(&counts)?)?;
    Ok(())
}
