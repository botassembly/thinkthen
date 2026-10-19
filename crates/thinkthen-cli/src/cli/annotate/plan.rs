//! The annotate dry run: every record's questions packed as a run would
//! pack them, with nothing looked up.

use crate::plan_document::PlanDocument;
use std::io::Write;
use std::process::ExitCode;

use super::Judging;
use super::asker::{self, Framed};
use crate::{PlanEstimate, BatchSetting as Setting};
use crate::cli::reading::Reading;
use crate::render::json_line;
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
        .map(String::as_str)
        .collect::<Vec<_>>();
    let composition = crate::RecordReading::new(&fields, None, None).map_err(Failure::from)?;
    let rows = inputs
        .map(|framed| {
            let original = parser.record(framed.map_err(|placed| placed.cause)?)?;
            super::native::compose(judging, &composition, &original)
        })
        .map(|row| {
            row.map(|record| record.map_original(crate::QuestionInput::Record))
                .map_err(Failure::into_error)

        });
    let native = judging.engine();
    let mut arguments = admitted.request().call.arguments().clone();
    arguments.question = crate::RequestQuestion::Definition { value: judging.set.clone().into() };
    arguments.input = crate::RequestInput::Feed {
        name: "cli-annotate-plan".to_owned(), framing: crate::RequestFraming::Document,
        reading: crate::ReaderOptions::default(), images: Vec::new(),
    };
    arguments.options.field = None;
    arguments.options.context_field = None;
    let admitted = crate::Request::for_function(crate::RequestFunction::Annotate, arguments).admit()?;
    let mut controls = crate::CallOptions::new().surface(crate::Surface::Cli);
    if let Some(context) = judging.context.as_deref() {
        controls = controls.context(context);
    }
    if let Some(setting) = setting {
        controls = controls.batch(match setting {
            Setting::Max => crate::BatchSetting::Max,
            Setting::Records(count) => crate::BatchSetting::Records(count),
        });
    }
    let summary = native.plan_request(&admitted, crate::RequestEnvironment {
        controls, feed: Some(crate::RequestFeed::from_records("cli-annotate-plan", rows)),
    })?;
    print_plan(judging, reading, &summary, writer)
}

fn print_plan(
    judging: &Judging<'_>,
    reading: &Reading,
    summary: &PlanEstimate,
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    let backend = &judging.backend;
    let Some(first) = summary.first_body() else {
        return Ok(ExitCode::SUCCESS);
    };
    judging.mismatch().print_once()?;
    crate::cli::check::say_dropped_detail(summary.drops_detail(), judging.environment.named())?;
    let (count, group_requests) = summary.question_requests()
        .ok_or(Failure::Defect("annotation preview lost its native group counts"))?;
    let document = PlanDocument::of_body(backend, first.to_vec())
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?
        .key_env(&backend.key_variable)
        .questions_on(judging.settings.on.clone())
        .requests(count, group_requests.to_vec());
    let document = if reading.streams() {
        document.reading(reading.framing(), reading.fields().to_vec(), reading.is_default())
    } else {
        document
    };
    edge::write_line(&mut *writer, &json_line(&document)?)?;
    let counts = crate::cli::plan_counts::Counts::from(summary);
    edge::write_line(writer, &json_line(&counts)?)?;
    Ok(ExitCode::SUCCESS)
}
