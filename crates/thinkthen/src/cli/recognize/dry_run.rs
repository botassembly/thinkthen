//! Exact recognition work known before asking the backend.

use std::io::Write;
use std::process::ExitCode;

use serde::Serialize;

use crate::core::{Backend, BackendProfile, KEY_VAR, Reading, RecognizeSpec, json_line};
use crate::edge;
use crate::engine::facade;
use crate::failure::Failure;

#[derive(Debug, Serialize)]
struct DryRun<'a> {
    schema: &'static str,
    url: &'a str,
    model: &'a str,
    key_env: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<From>,
    pieces: usize,
    request_count: usize,
    name_requests_upper_bound: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    relation_pairs_upper_bound: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relation_requests_upper_bound: Option<usize>,
    requests: Vec<Request>,
}

/// One exact split request, as the relate plan prints it.
#[derive(Debug, Serialize)]
struct Request {
    digest: String,
    bytes: usize,
    body_utf8: String,
}

#[derive(Debug, Serialize)]
struct From {
    question: &'static str,
}

pub(super) fn run(
    reading: &Reading,
    source: Box<dyn std::io::BufRead + Send>,
    backend: &Backend,
    profile: Option<&BackendProfile>,
    question: (&RecognizeSpec, bool, usize),
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    let (spec, from_file, limit) = question;
    let mut chunks = edge::Chunks::new(source, reading.streams());
    let Some(bytes) = chunks.next().transpose()? else {
        return Ok(ExitCode::SUCCESS);
    };
    let record = reading
        .record(&bytes)
        .map_err(|error| Failure::record(error, reading.streams()))?;
    let text = reading.evidence(&record)?.as_text()?.into_owned();
    let (pieces, chunks) = facade::step_one(backend, profile, spec, &text, limit)?;
    let requests = chunks
        .into_iter()
        .map(|chunk| {
            Ok(Request {
                digest: chunk.request.digest.as_str().to_owned(),
                bytes: chunk.request.body.len(),
                body_utf8: String::from_utf8(chunk.request.body)
                    .map_err(|_| Failure::Defect("an encoded request is not UTF-8"))?,
            })
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    let report = DryRun {
        schema: "thinkthen.recognize-plan/2",
        url: backend.url().as_str(),
        model: backend.model().as_str(),
        key_env: KEY_VAR,
        from: from_file.then_some(From { question: "file" }),
        pieces: pieces.len(),
        request_count: requests.len(),
        name_requests_upper_bound: requests.len(),
        relation_pairs_upper_bound: relation_upper_bound(spec, pieces.len()),
        relation_requests_upper_bound: relation_upper_bound(spec, pieces.len()),
        requests,
    };
    edge::write_line(writer, &json_line(&report)?)?;
    Ok(ExitCode::SUCCESS)
}

fn relation_upper_bound(spec: &RecognizeSpec, tokens: usize) -> Option<usize> {
    (!spec.relations.is_empty()).then(|| {
        let directed = tokens.saturating_mul(tokens.saturating_sub(1));
        spec.relations.iter().fold(0_usize, |total, rule| {
            total.saturating_add(if rule.either { directed / 2 } else { directed })
        })
    })
}
