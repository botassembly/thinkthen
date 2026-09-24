use std::io::Write;

use serde::Serialize;

use super::config::From;
use crate::core::{
    Backend, BackendProfile, Framing, LimitKind, RelateFields, RelateSpec, json_line,
};
use crate::edge;
use crate::engine::facade::{Method, PreparedRelation};
use crate::failure::Failure;

#[derive(Serialize)]
struct Report<'a> {
    schema: &'static str,
    url: &'a str,
    model: &'a str,
    key_env: &'static str,
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
    method: Method,
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
}

pub(super) fn write(
    writer: &mut dyn Write,
    context: Context<'_>,
    prepared: &[PreparedRelation],
) -> Result<(), Failure> {
    let mut relations = Vec::new();
    let mut requests = Vec::new();
    for planned in prepared {
        relations.push(Relation {
            name: &planned.relation.name,
            source: &planned.relation.source,
            target: &planned.relation.target,
            reads: &planned.relation.reads,
            either: planned.relation.either,
            method: planned.method,
            fallback: planned.fallback.map(fallback_name),
            logical_questions: planned.mappings.len(),
            request_count: planned.chunks.len(),
        });
        for chunk in &planned.chunks {
            requests.push(Request {
                digest: chunk.request.digest.as_str(),
                bytes: chunk.request.body.len(),
                body_utf8: str::from_utf8(&chunk.request.body)
                    .map_err(|_| Failure::Defect("an encoded request is not UTF-8"))?,
            });
        }
    }
    let report = Report {
        schema: "thinkthen.relate-plan/1",
        url: context.backend.url().as_str(),
        model: context.backend.model().as_str(),
        key_env: crate::core::KEY_VAR,
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
    edge::write_line(writer, &json_line(&report)?).map(|_| ())
}

/// The backend-profile key whose limit turned a choice into yes/no questions.
const fn fallback_name(kind: LimitKind) -> &'static str {
    match kind {
        LimitKind::Options => "max_options",
        LimitKind::RequestBytes => "max_request_bytes",
        LimitKind::EvidenceBytes => "max_evidence_bytes",
        LimitKind::Questions => "max_questions",
    }
}
