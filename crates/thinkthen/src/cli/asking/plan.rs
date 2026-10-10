//! The plan `--plan` prints in place of a request.

use std::io::{self, IsTerminal, Write};
use std::process::ExitCode;

use super::JudgingInput;
use super::judged::Records;
use crate::core::{
    Backend, Evidence, PlanDocument, PlanSummary, Reading, Setting, Sources, json_line,
};
use crate::edge;
use crate::failure::Failure;
use crate::profile::Mismatch;
use crate::schedule::Output;

/// What a plan shows beyond the request: where the question came from.
pub(super) struct Planning<'a> {
    pub(super) sources: Option<Sources>,
    pub(super) key_env: &'a str,
}

impl JudgingInput<'_> {
    pub(super) fn planning(&self) -> Planning<'_> {
        Planning {
            sources: self.sources,
            key_env: self.environment.key_variable(),
        }
    }
}

/// Pack every record's questions as a run would, with nothing looked up,
/// and print the first request and the counts.
pub(super) fn packed(
    configuration: &JudgingInput<'_>,
    reading: &Reading,
    records: Records,
    context: Option<Evidence>,
    setting: Option<Setting>,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let backend = &configuration.backend;
    let admitted = configuration
        .admitted
        .as_ref()
        .ok_or(Failure::Defect("judgment plan has no admitted request"))?;
    let composition = super::native::composition(admitted, reading)?;
    let rows = records.map(|held| {
        let held = held.map_err(|placed| {
            crate::Error::usage("the CLI reader failed").with_diagnostic(
                crate::public::error::diagnostic::Diagnostic::CliInput(Box::new(placed.cause)),
            )
        })?;
        super::native::compose(&composition, &held)
    });
    let inner = crate::cli::construction::engine(
        configuration.common,
        configuration.environment,
        super::Folders::of(configuration.common, configuration.environment)?,
        (backend.clone(), configuration.profile.clone()),
        configuration.common.jobs,
        false,
    )?;
    let native = crate::Engine::from_cli(inner, configuration.environment.config().prices());
    let admitted = admitted.clone().with_composed_feed("cli-plan");
    let shared = context.as_ref().map(|value| value.as_text()).transpose()?;
    let ready = || true;
    let readiness = crate::public::cli_reader::CliReader::new(&ready, None);
    let mut controls = crate::CallOptions::new()
        .surface(crate::Surface::Cli)
        .cli_reader(&readiness);
    if let Some(shared) = &shared {
        controls = controls.context(shared);
    }
    if let Some(setting) = setting {
        controls = controls.batch(match setting {
            Setting::Max => crate::BatchSetting::Max,
            Setting::Records(count) => crate::BatchSetting::Records(count),
        });
    }
    let mut feed = crate::RequestFeed::from_records("cli-plan", rows);
    if configuration.common.images() {
        feed = feed.with_image_inputs();
    }
    let (summary, dropped) = native
        .plan_request_summary(
            &admitted,
            crate::RequestEnvironment {
                controls,
                feed: Some(feed),
            },
        )
        .map_err(Failure::from)?;
    crate::cli::check::say_dropped_detail(dropped, configuration.environment.named())?;
    let Some(first) = summary.first_body() else {
        return Ok(ExitCode::SUCCESS);
    };
    print_plan(
        backend,
        &configuration.mismatch,
        reading,
        &configuration.planning(),
        first.to_vec(),
        &summary,
        output.writer(),
    )
}

/// Print the plan document of one checked plan.
#[expect(
    clippy::too_many_arguments,
    reason = "the judging and annotate previews pass resolved concerns without another configuration type"
)]
pub(super) fn print_plan(
    backend: &Backend,
    mismatch: &Mismatch,
    reading: &Reading,
    planning: &Planning<'_>,
    body: Vec<u8>,
    summary: &PlanSummary,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    mismatch.print_once()?;
    let document = PlanDocument::of_body(backend, body)
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?
        .key_env(planning.key_env);
    if summary.first_body() != Some(document.request_body()) {
        return Err(Failure::Defect(
            "the disclosed request changed after preparation",
        ));
    }
    let document = if reading.streams() {
        document.reading(reading)
    } else {
        document
    };
    let document = match planning.sources {
        Some(sources) => document.from(sources),
        None => document,
    };
    let line = json_line(&document)?;
    if !reading.streams() && io::stdout().is_terminal() {
        let mut stderr = io::stderr().lock();
        writeln!(
            stderr,
            "thinkthen: plan: each question in request.questions quotes the evidence it asks about."
        )
        .and_then(|()| stderr.flush())
        .map_err(Failure::Output)?;
    }
    edge::write_line(&mut writer, &line)?;
    let counts = summary
        .counts()
        .map_err(|_| Failure::Defect("a plan is too large"))?;
    edge::write_line(&mut writer, &json_line(&counts)?)?;
    Ok(ExitCode::SUCCESS)
}
