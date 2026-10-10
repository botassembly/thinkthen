//! The annotate dry run: every record's questions packed as a run would
//! pack them, with nothing looked up.

use std::io::Write;
use std::process::ExitCode;

use super::Judging;
use super::asker::{self, Framed};
use crate::core::adapters::built_in;
use crate::core::{PlanDocument, PlanSummary, Reading, Setting, json_line};
use crate::edge;
use crate::failure::Failure;
use crate::schedule::Placed;

pub(super) fn dry_run(
    judging: &Judging<'_>,
    admitted: crate::AdmittedRequest,
    reading: &Reading,
    inputs: impl Iterator<Item = Result<Framed, Placed>>,
    setting: Option<Setting>,
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    let parser = asker::Parser::of(judging, reading);
    let fields = reading
        .fields()
        .iter()
        .map(crate::core::Pointer::as_str)
        .collect::<Vec<_>>();
    let composition = crate::RecordReading::new(&fields, None, None).map_err(Failure::from)?;
    let rows = inputs
        .map(|framed| {
            let original = parser.record(framed.map_err(|placed| placed.cause)?)?;
            super::native::compose(judging, &composition, &original)
        })
        .map(|row| {
            row.map(|record| record.map_original(crate::QuestionInput::Record))
                .map_err(|cause| {
                    crate::Error::usage("the CLI reader failed").with_diagnostic(
                        crate::public::error::diagnostic::Diagnostic::CliInput(Box::new(cause)),
                    )
                })
        });
    let native = crate::Engine::from_cli(
        judging.engine.clone(),
        judging.environment.config().prices(),
    );
    let admitted = admitted
        .retain_cli_definition(crate::QuestionSet(judging.set.clone()).into())
        .map_err(Failure::from)?
        .with_composed_feed("cli-annotate-plan");
    let ready = || true;
    let readiness = crate::public::cli_reader::CliReader::new(&ready, None);
    let mut controls = crate::CallOptions::new()
        .surface(crate::Surface::Cli)
        .cli_reader(&readiness);
    if let Some(context) = judging.context.as_deref() {
        controls = controls.context(context);
    }
    if let Some(setting) = setting {
        controls = controls.batch(match setting {
            Setting::Max => crate::BatchSetting::Max,
            Setting::Records(count) => crate::BatchSetting::Records(count),
        });
    }
    let (summary, count, group_requests) = native
        .plan_annotation_request(
            &admitted,
            crate::RequestEnvironment {
                controls,
                feed: Some(crate::RequestFeed::from_records("cli-annotate-plan", rows)),
            },
        )
        .map_err(Failure::from)?;
    print_plan(judging, reading, &summary, count, group_requests, writer)
}

fn print_plan(
    judging: &Judging<'_>,
    reading: &Reading,
    summary: &PlanSummary,
    count: usize,
    group_requests: Vec<usize>,
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    let backend = judging.engine().backend();
    let Some(first) = summary.first_body() else {
        return Ok(ExitCode::SUCCESS);
    };
    judging.mismatch().print_once()?;
    crate::cli::check::say_dropped_detail(
        judging
            .set()
            .questions()
            .iter()
            .any(|named| built_in::drops_detail_of(backend.descriptions(), named.question())),
        judging.environment.named(),
    )?;
    let on = judging
        .set()
        .questions()
        .iter()
        .map(|question| (question.name().to_owned(), question.on().to_vec()))
        .collect();
    let document = PlanDocument::of_body(backend, first.to_vec())
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?
        .key_env(judging.environment.key_variable())
        .questions_on(on)
        .requests(count, group_requests);
    let document = if reading.streams() {
        document.reading(reading)
    } else {
        document
    };
    edge::write_line(&mut *writer, &json_line(&document)?)?;
    let counts = summary
        .counts()
        .map_err(|_| Failure::Defect("a plan is too large"))?;
    edge::write_line(writer, &json_line(&counts)?)?;
    Ok(ExitCode::SUCCESS)
}
