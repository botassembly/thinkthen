//! Exact recognition work known before asking the backend.

use std::fmt;
use std::io::Write;
use std::process::ExitCode;

use serde::Serialize;

use crate::core::{
    Backend, BackendProfile, KEY_VAR, PlanSummary, Reading, RecognizeSpec, Record, json_line,
};
use crate::edge;
use crate::engine::facade;
use crate::failure::Failure;
use crate::table::{Kind as TableKind, Rows as TableRows};

#[derive(Serialize)]
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

impl fmt::Debug for DryRun<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DryRun")
            .field("schema", &self.schema)
            .field("url", &"<withheld>")
            .field("model", &self.model)
            .field("key_env", &self.key_env)
            .field("from", &self.from)
            .field("pieces", &self.pieces)
            .field("request_count", &self.request_count)
            .field("name_requests_upper_bound", &self.name_requests_upper_bound)
            .field(
                "relation_pairs_upper_bound",
                &self.relation_pairs_upper_bound,
            )
            .field(
                "relation_requests_upper_bound",
                &self.relation_requests_upper_bound,
            )
            .field("requests", &self.requests.len())
            .finish()
    }
}

/// One exact split request, as the relate plan prints it.
#[derive(Serialize)]
struct Request {
    digest: String,
    bytes: usize,
    body_utf8: String,
}

#[derive(Debug, Serialize)]
struct From {
    question: &'static str,
}

#[expect(
    clippy::too_many_arguments,
    reason = "the preview boundary receives each already resolved concern once"
)]
pub(super) fn run(
    reading: &Reading,
    source: Box<dyn std::io::BufRead + Send>,
    backend: &Backend,
    profile: Option<&BackendProfile>,
    table: Option<TableKind>,
    question: (&RecognizeSpec, bool, usize),
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    if let Some(kind) = table {
        let rows = TableRows::new(source, kind)?;
        return planned(rows, backend, profile, question, writer, reading);
    }
    let reading_for_rows = reading.clone();
    let records = edge::numbered(edge::Chunks::new(source, reading.streams()), reading).map(
        move |(_, bytes)| {
            let bytes = bytes?;
            reading_for_rows
                .record(&bytes)
                .map_err(|error| Failure::record(error, reading_for_rows.streams()))
        },
    );
    planned(records, backend, profile, question, writer, reading)
}

fn planned(
    records: impl Iterator<Item = Result<Record, Failure>>,
    backend: &Backend,
    profile: Option<&BackendProfile>,
    question: (&RecognizeSpec, bool, usize),
    writer: &mut dyn Write,
    reading: &Reading,
) -> Result<ExitCode, Failure> {
    let (spec, from_file, limit) = question;
    let mut summary = PlanSummary::new(true);
    let mut first = None;
    for record in records {
        let record = record?;
        let text = reading.evidence(&record)?.as_text()?.into_owned();
        let (pieces, prepared) = facade::step_one(backend, profile, spec, &text, limit)?;
        summary
            .record()
            .map_err(|_| Failure::Defect("a plan is too large"))?;
        // Each found name asks at most one kind and one edge question. A
        // profile may split stage two differently, but a request asks at
        // least one question and there cannot be more names than pieces.
        let questions_per_name = if spec.kinds.is_empty() { 1 } else { 2 };
        let name_bound = pieces
            .len()
            .checked_mul(questions_per_name)
            .ok_or(Failure::Defect("a plan is too large"))?;
        let mut requests = Vec::new();
        for chunk in prepared {
            summary
                .request(&chunk.request.body)
                .map_err(|_| Failure::Defect("a plan is too large"))?;
            if first.is_none() {
                requests.push(Request {
                    digest: chunk.request.digest.as_str().to_owned(),
                    bytes: chunk.request.body.len(),
                    body_utf8: String::from_utf8(chunk.request.body)
                        .map_err(|_| Failure::Defect("an encoded request is not UTF-8"))?,
                });
            }
        }
        summary
            .possible_requests(name_bound)
            .map_err(|_| Failure::Defect("a plan is too large"))?;
        let bound = relation_upper_bound(spec, pieces.len());
        if let Some(bound) = bound {
            summary
                .possible_requests(bound)
                .map_err(|_| Failure::Defect("a plan is too large"))?;
        }
        if first.is_none() {
            first = Some((pieces.len(), requests, name_bound, bound));
        }
    }
    let Some((pieces, requests, name_bound, relation_bound)) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    let report = DryRun {
        schema: "thinkthen.recognize-plan/2",
        url: backend.url().as_str(),
        model: backend.model().as_str(),
        key_env: KEY_VAR,
        from: from_file.then_some(From { question: "file" }),
        pieces,
        request_count: requests.len(),
        name_requests_upper_bound: name_bound,
        relation_pairs_upper_bound: relation_bound,
        relation_requests_upper_bound: relation_bound,
        requests,
    };
    edge::write_line(&mut *writer, &json_line(&report)?)?;
    let counts = summary
        .counts()
        .map_err(|_| Failure::Defect("a plan is too large"))?;
    edge::write_line(writer, &json_line(&counts)?)?;
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

#[cfg(test)]
mod tests {
    use super::{DryRun, Request};
    use crate::core::{KEY_VAR, json_line};

    #[test]
    #[allow(
        clippy::expect_used,
        reason = "a broken serialization fixture stops the Debug boundary proof"
    )]
    fn plan_debug_withholds_raw_url_and_request_while_json_keeps_them() {
        let url = "http://localhost/plan-marker-0210";
        let body = "request-marker-0210";
        let plan = DryRun {
            schema: "thinkthen.recognize-plan/2",
            url,
            model: "local-1",
            key_env: KEY_VAR,
            from: None,
            pieces: 1,
            request_count: 1,
            name_requests_upper_bound: 1,
            relation_pairs_upper_bound: None,
            relation_requests_upper_bound: None,
            requests: vec![Request {
                digest: "one".to_owned(),
                bytes: body.len(),
                body_utf8: body.to_owned(),
            }],
        };
        let shown = format!("{plan:?}");
        assert!(shown.contains("url: \"<withheld>\""), "{shown}");
        assert!(!shown.contains("plan-marker-0210"), "{shown}");
        assert!(!shown.contains(body), "{shown}");
        let serialized = json_line(&plan).expect("exact serialized plan");
        assert!(serialized.contains(url));
        assert!(serialized.contains(body));
    }
}
