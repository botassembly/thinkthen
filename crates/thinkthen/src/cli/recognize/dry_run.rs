//! Exact recognition work known before asking the backend.

use std::io::Write;
use std::process::ExitCode;

use serde::Serialize;

use crate::core::{
    Backend, BackendProfile, Plan, Reading, RecognizeSpec, json_line, kind_questions,
    recognition_questions, tokenize,
};
use crate::edge;
use crate::failure::Failure;
use crate::prepared_request::PreparedRequests;

#[derive(Debug, Serialize)]
struct DryRun {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<From>,
    tokens: usize,
    detection_questions: usize,
    kind_questions: usize,
    requests: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    relation_pairs_upper_bound: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relation_requests_upper_bound: Option<usize>,
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
    if tokens.is_empty() {
        write_empty(spec, from_file, writer)?;
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
    let requests = PreparedRequests::with_profile(backend, &plan, profile)?
        .into_chunks()
        .len();
    let relation_pairs_upper_bound = relation_upper_bound(spec, tokens.len());
    write(
        writer,
        DryRun {
            from: from_file.then_some(From { question: "file" }),
            tokens: tokens.len(),
            detection_questions: tokens.len(),
            kind_questions: kind_question_count,
            requests,
            relation_pairs_upper_bound,
            relation_requests_upper_bound: relation_pairs_upper_bound,
        },
    )?;
    Ok(ExitCode::SUCCESS)
}

fn write_empty(
    spec: &RecognizeSpec,
    from_file: bool,
    writer: &mut dyn Write,
) -> Result<(), Failure> {
    write(
        writer,
        DryRun {
            from: from_file.then_some(From { question: "file" }),
            tokens: 0,
            detection_questions: 0,
            kind_questions: 0,
            requests: 0,
            relation_pairs_upper_bound: (!spec.relations.is_empty()).then_some(0),
            relation_requests_upper_bound: (!spec.relations.is_empty()).then_some(0),
        },
    )
}

fn relation_upper_bound(spec: &RecognizeSpec, tokens: usize) -> Option<usize> {
    (!spec.relations.is_empty()).then(|| {
        let directed = tokens.saturating_mul(tokens.saturating_sub(1));
        spec.relations.iter().fold(0_usize, |total, rule| {
            total.saturating_add(if rule.either { directed / 2 } else { directed })
        })
    })
}

fn write(writer: &mut dyn Write, report: DryRun) -> Result<(), Failure> {
    edge::write_line(writer, &json_line(&report)?).map(|_| ())
}
