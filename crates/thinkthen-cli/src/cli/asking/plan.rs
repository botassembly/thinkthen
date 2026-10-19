//! The plan `--plan` prints in place of a request.

use crate::plan_document::PlanDocument;
use std::io::{self, IsTerminal, Write};
use std::process::ExitCode;

use super::JudgingInput;
use super::judged::Records;
use crate::{BatchSetting as Setting, PlanEstimate, ResolvedSettings};
use crate::cli::reading::Reading;
use crate::plan_document::Sources;
use crate::render::json_line;
use crate::edge;
use crate::failure::Failure;
use crate::profile::Mismatch;
use crate::schedule::Output;

/// What a plan shows beyond the request: where the question came from.
pub(super) struct Planning<'a> {
    pub(super) sources: Option<&'a Sources>,
    pub(super) key_env: &'a str,
}

impl JudgingInput<'_> {
    pub(super) fn planning(&self) -> Planning<'_> {
        Planning {
            sources: self.sources.as_ref(),
            key_env: &self.backend.key_variable,
        }
    }
}

/// Pack every record's questions as a run would, with nothing looked up,
/// and print the first request and the counts.
pub(super) fn packed(
    configuration: &JudgingInput<'_>,
    reading: &Reading,
    records: Records,
    context: Option<String>,
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
        let held = held.map_err(|placed| placed.cause.into_error())?;
        super::native::compose(&composition, &held)
    });
    let native = crate::cli::construction::engine(
        configuration.common, configuration.environment,
        super::Folders::of(configuration.common, configuration.environment)?,
        configuration.common.jobs,
    )?;
    let mut arguments = admitted.request().call.arguments().clone();
    arguments.question = crate::RequestQuestion::Definition { value: admitted.resolve_question()? };
    arguments.input = crate::RequestInput::Feed {
        name: "cli-plan".to_owned(), framing: crate::RequestFraming::Document,
        reading: crate::ReaderOptions::default(), images: Vec::new(),
    };
    arguments.options.field = None;
    arguments.options.context_field = None;
    arguments.options.options_field = None;
    arguments.options.examples_field = None;
    arguments.options.seed_spans_field = None;
    let admitted = crate::Request::for_function(admitted.request().call.function(), arguments).admit()?;
    let shared = context.as_deref();
    let mut controls = crate::CallOptions::new().surface(crate::Surface::Cli);
    if let Some(shared) = shared {
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
    let summary = native.plan_request(
        &admitted, crate::RequestEnvironment { controls, feed: Some(feed) },
    )?;
    crate::cli::check::say_dropped_detail(summary.drops_detail(), configuration.environment.named())?;
    let Some(first) = summary.first_body() else {
        return Ok(ExitCode::SUCCESS);
    };
    print_plan(
        backend,
        &configuration.mismatch.clone().running_profile(configuration.profile.as_deref(), native.profile_name()),
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
    backend: &ResolvedSettings,
    mismatch: &Mismatch,
    reading: &Reading,
    planning: &Planning<'_>,
    body: Vec<u8>,
    summary: &PlanEstimate,
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
        document.reading(reading.framing(), reading.fields().to_vec(), reading.is_default())
    } else {
        document
    };
    let document = match planning.sources {
        Some(sources) => document.from(sources.clone()),
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
    let counts = Counts::of(summary);
    edge::write_line(&mut writer, &json_line(&counts)?)?;
    Ok(ExitCode::SUCCESS)
}

/// The command's existing counts line omits the separately disclosed body.
#[derive(serde::Serialize)]
struct Counts {
    records: usize, requests: usize, estimated_bytes: usize,
    largest_request_bytes: usize, largest_request_estimated_input_tokens: u64,
    token_estimate_method: &'static str, estimated_input_tokens: TokenBand, upper_bound: bool,
}
#[derive(serde::Serialize)]
struct TokenBand { lower: usize, upper: usize }
impl Counts {
    fn of(plan: &PlanEstimate) -> Self {
        let (lower, upper) = plan.estimated_input_tokens();
        Self {
            records: plan.records(), requests: plan.requests(), estimated_bytes: plan.estimated_bytes(),
            largest_request_bytes: plan.largest_request_bytes(),
            largest_request_estimated_input_tokens: plan.largest_request_estimated_input_tokens(),
            token_estimate_method: plan.token_estimate_method(),
            estimated_input_tokens: TokenBand { lower, upper }, upper_bound: plan.upper_bound(),
        }
    }
}
