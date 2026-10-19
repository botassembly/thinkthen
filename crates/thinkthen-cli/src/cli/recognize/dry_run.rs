//! Exact recognition work known before asking the backend.

use std::fmt;
use std::io::Write;
use std::process::ExitCode;

use serde::Serialize;

use crate::cli::intake::{Data, Intake};
use crate::cli::reading::Reading;
use crate::edge;
use crate::failure::Failure;
use crate::render::json_line;
use crate::{Engine, Recognize, ResolvedSettings};

#[derive(Serialize)]
struct DryRun<'a> {
    schema: &'static str,
    url: &'a str,
    model: &'a str,
    key_env: &'a str,
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

#[derive(Debug)]
pub(super) struct Question<'a> {
    pub(super) spec: &'a Recognize,
    pub(super) from_file: bool,
    /// The first key variable of the selected backend.
    pub(super) key_env: &'a str,
    pub(super) backend_name: Option<&'a str>,
    pub(super) context: Option<&'a str>,
    pub(super) context_field: Option<&'a str>,
    pub(super) examples_field: Option<&'a str>,
    pub(super) seed_spans_field: Option<&'a str>,
}

pub(super) fn run(
    reading: &Reading,
    admitted: crate::AdmittedRequest,
    source: Intake,
    (backend, environment): (&ResolvedSettings, &crate::edge::Environment),
    question: Question<'_>,
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    let Question {
        spec,
        context,
        context_field,
        examples_field,
        seed_spans_field,
        ..
    } = &question;
    let composition = super::native::composition(reading, *examples_field, *seed_spans_field)?;
    let records = source
        .map(|item| {
            let item = item.map_err(|placed| placed.cause)?;
            let record = match item.data {
                Data::Images(_) => {
                    return Err(Failure::Usage(
                        "recognize accepts text only; images are unsupported",
                    ));
                }
                Data::Record(record) => record,
                Data::Bytes(bytes) => reading
                    .record(&bytes)
                    .map_err(|error| Failure::record(error, reading.streams()))?,
            };
            let mut native = composition.compose(record.clone()).map_err(Failure::from)?;
            native.context = crate::asking::context::record(
                &record,
                *context_field,
                spec.reading().context_schema(),
            )?;
            Ok(native.map_original(crate::QuestionInput::Record))
        })
        .map(|row| row.map_err(Failure::into_error));
    let request = super::native::request(admitted, *context).admit()?;
    // Plans use the same captured provider and size selection without reading answers.
    let engine: Engine = environment.take_builder()?.no_cache().build()?;
    let preview = engine.plan_request(
        &request,
        crate::RequestEnvironment {
            controls: crate::CallOptions::new().surface(crate::Surface::Cli),
            feed: Some(crate::RequestFeed::from_records("cli-recognize", records)),
        },
    )?;
    print(preview, backend, question, writer)
}

fn print(
    preview: crate::PlanEstimate,
    backend: &ResolvedSettings,
    question: Question<'_>,
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    let Question {
        from_file,
        key_env,
        backend_name,
        ..
    } = question;
    let Some(first) = preview.recognition() else {
        return Ok(ExitCode::SUCCESS);
    };
    let pieces = first.pieces;
    let name_bound = first.name_requests_upper_bound;
    let relation_bound = first.relation_pairs_upper_bound;
    let requests = preview
        .prepared_requests()
        .map(|request| {
            Ok(Request {
                digest: request.digest.to_owned(),
                bytes: request.body.len(),
                body_utf8: String::from_utf8(request.body.to_vec())
                    .map_err(|_| Failure::Defect("an encoded request is not UTF-8"))?,
            })
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    crate::cli::check::say_dropped_detail(preview.drops_detail(), backend_name)?;
    let report = DryRun {
        schema: "thinkthen.recognize-plan/2",
        url: &backend.url,
        model: &backend.model,
        key_env,
        from: from_file.then_some(From { question: "file" }),
        pieces,
        request_count: requests.len(),
        name_requests_upper_bound: name_bound,
        relation_pairs_upper_bound: relation_bound,
        relation_requests_upper_bound: relation_bound,
        requests,
    };
    edge::write_line(&mut *writer, &json_line(&report)?)?;
    let counts = crate::cli::plan_counts::Counts::from(&preview);
    edge::write_line(writer, &json_line(&counts)?)?;
    Ok(ExitCode::SUCCESS)
}

#[cfg(test)]
mod tests {
    //! Stays: dry-run probability accounting requires direct numeric boundary checks.
    use super::{DryRun, Request};
    use crate::render::json_line;

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
            key_env: "THINKTHEN_API_KEY",

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
