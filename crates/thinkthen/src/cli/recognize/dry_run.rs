//! Exact recognition work known before asking the backend.

use std::io::Write;
use std::process::ExitCode;

use serde::Serialize;

use crate::core::{
    Backend, BackendProfile, KEY_VAR, Plan, Reading, RecognizeSpec, json_line, kind_questions,
    recognition_questions, tokenize,
};
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
    words: usize,
    detection_questions: usize,
    kind_questions: usize,
    request_count: usize,
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
    question: (&RecognizeSpec, bool),
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    let (spec, from_file) = question;
    let mut chunks = edge::Chunks::new(source, reading.streams());
    let Some(bytes) = chunks.next().transpose()? else {
        return Ok(ExitCode::SUCCESS);
    };
    let record = reading
        .record(&bytes)
        .map_err(|error| Failure::record(error, reading.streams()))?;
    let text = reading.evidence(&record)?.as_text()?.into_owned();
    let tokens = tokenize(&text);
    let from = from_file.then_some(From { question: "file" });
    let mut report = DryRun {
        schema: "thinkthen.recognize-plan/1",
        url: backend.url().as_str(),
        model: backend.model().as_str(),
        key_env: KEY_VAR,
        from,
        words: 0,
        detection_questions: 0,
        kind_questions: 0,
        request_count: 0,
        relation_pairs_upper_bound: relation_upper_bound(spec, 0),
        relation_requests_upper_bound: relation_upper_bound(spec, 0),
        requests: Vec::new(),
    };
    if tokens.is_empty() {
        write(writer, &report)?;
        return Ok(ExitCode::SUCCESS);
    }
    let mut questions = recognition_questions(&tokens)
        .map_err(|_| Failure::Defect("fixed detection questions are invalid"))?;
    let kind_questions = kind_questions(&tokens, &spec.kinds)
        .map_err(|_| super::config::error(false, crate::core::RecognizeConfigError::Kinds))?;
    let kind_question_count = kind_questions.len();
    questions.extend(kind_questions);
    let evidence = crate::core::Evidence::new(text)
        .map_err(|_| Failure::Defect("record evidence became blank"))?;
    let plan = Plan::new(evidence, backend.model().clone(), questions)
        .map_err(|_| Failure::Defect("recognize dry run planned no questions"))?;
    let chunks = facade::split(backend, profile, &plan)?;
    for chunk in &chunks {
        report.requests.push(Request {
            digest: chunk.request.digest.as_str().to_owned(),
            bytes: chunk.request.body.len(),
            body_utf8: String::from_utf8(chunk.request.body.clone())
                .map_err(|_| Failure::Defect("an encoded request is not UTF-8"))?,
        });
    }
    report.words = tokens.len();
    report.detection_questions = tokens.len();
    report.kind_questions = kind_question_count;
    report.request_count = chunks.len();
    report.relation_pairs_upper_bound = relation_upper_bound(spec, tokens.len());
    report.relation_requests_upper_bound = report.relation_pairs_upper_bound;
    write(writer, &report)?;
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

fn write(writer: &mut dyn Write, report: &DryRun<'_>) -> Result<(), Failure> {
    edge::write_line(writer, &json_line(report)?).map(|_| ())
}
