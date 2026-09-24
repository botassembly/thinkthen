use std::io::Write;

use serde::Serialize;

use super::config::From;
use super::plan::{Fallback, Method, PreparedRelation};
use crate::core::{Backend, BackendProfile, Framing, RelateFields, json_line};
use crate::edge;
use crate::failure::Failure;

#[derive(Serialize)] #[rustfmt::skip]
struct Report<'a> {
    schema: &'static str, url: &'a str, model: &'a str, key_env: &'static str,
    backend_profile: Option<&'a str>, framing: Framing, fields: Option<&'a RelateFields>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<From>,
    entity_count: usize, relations: Vec<Relation<'a>>, logical_questions: usize,
    request_count: usize, requests: Vec<Request<'a>>,
}

#[derive(Serialize)] #[rustfmt::skip]
struct Relation<'a> {
    name: &'a str, source: &'a str, target: &'a str, reads: &'a str, either: bool,
    method: Method, fallback: Option<Fallback>, logical_questions: usize, request_count: usize,
}

#[derive(Serialize)] #[rustfmt::skip]
struct Request<'a> {
    digest: &'a str, bytes: usize, body_utf8: &'a str,
}

#[rustfmt::skip]
pub(super) struct Context<'a> {
    pub(super) backend: &'a Backend, pub(super) profile: Option<&'a BackendProfile>, pub(super) framing: Framing,
    pub(super) spec: &'a crate::core::RelateSpec, pub(super) from: Option<From>, pub(super) entity_count: usize, pub(super) prepared: &'a [PreparedRelation],
}

#[rustfmt::skip]
pub(super) fn write(writer: &mut dyn Write, context: Context<'_>) -> Result<(), Failure> {
    let Context { backend, profile, framing, spec, from, entity_count, prepared } = context;
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
            fallback: planned.fallback,
            logical_questions: planned.logical_questions(),
            request_count: planned.chunks.len(),
        });
        for chunk in &planned.chunks {
            let body_utf8 = str::from_utf8(&chunk.request.body)
                .map_err(|_| Failure::Defect("an encoded request is not UTF-8"))?;
            requests.push(Request {
                digest: chunk.request.digest.as_str(),
                bytes: chunk.request.body.len(),
                body_utf8,
            });
        }
    }
    let logical_questions = relations.iter().map(|item| item.logical_questions).sum();
    let report = Report {
        schema: "thinkthen.relate-plan/1",
        url: backend.url().as_str(),
        model: backend.model().as_str(),
        key_env: crate::core::KEY_VAR,
        backend_profile: profile.map(|item| item.name().as_str()),
        framing,
        fields: (framing != Framing::Lines).then_some(spec.fields()),
        from,
        entity_count,
        relations,
        logical_questions,
        request_count: requests.len(),
        requests,
    };
    edge::write_line(writer, &json_line(&report)?).map(|_| ())
}
