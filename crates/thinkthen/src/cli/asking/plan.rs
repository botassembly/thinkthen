//! The plan `--plan` prints in place of a request.

use std::io::{self, IsTerminal, Write};
use std::process::ExitCode;

use std::collections::HashSet;
use std::sync::Arc;

use super::JudgingInput;
use super::judged::{Planner, Records};
use crate::core::adapters::built_in;
use crate::core::pack::{self, Entry, PackLimits, Packer};
use crate::core::{Backend, Evidence, PlanDocument, PlanSummary, Reading, Sources, json_line};
use crate::edge;
use crate::engine::pipeline::{self, MOST_INPUTS, Packing};
use crate::failure::Failure;
use crate::failure::context::Limits;
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
    inputs: Option<usize>,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let backend = &configuration.backend;
    let planner = Planner {
        asks: &configuration.asks,
        reading,
        asked: backend.asked(),
        context,
        profile: configuration.profile.as_ref(),
        limits: Limits::new(configuration.profile.as_ref()),
        route: backend.image_route(),
    };
    let packing = Packing {
        questions: None,
        sized: true,
        inputs,
        context: planner.context.is_some(),
        detailed: false,
        continues: false,
    };
    let mut packer = packer(&planner, backend, packing)?;
    let mut summary = PlanSummary::new(false);
    let mut seen = HashSet::new();
    let mut closed = Vec::new();
    let mut dropped = false;
    for held in records {
        let held = held.map_err(|placed| placed.cause)?;
        summary
            .record()
            .map_err(|_| Failure::Defect("a plan is too large"))?;
        let mut entries = Vec::new();
        for plan in planner.plans(&held)? {
            dropped |= built_in::drops_detail(&plan);
            let asks =
                pack::asks(backend.url(), &plan).map_err(|error| super::encoded(&plan, error))?;
            entries.extend(
                asks.into_iter()
                    .filter(|ask| seen.insert(ask.key))
                    .map(|ask| Entry {
                        state: ask.state.clone(),
                        question: Arc::clone(&ask.question),
                        options: pipeline::options(&ask),
                        item: (),
                    }),
            );
        }
        packer
            .add(entries, &mut closed)
            .map_err(|error| planner.refused(error))?;
    }
    closed.extend(packer.close());
    crate::cli::check::say_dropped_detail(dropped, configuration.environment.named())?;
    for request in &closed {
        summary
            .request(&request.body)
            .map_err(|_| Failure::Defect("a plan is too large"))?;
    }
    let Some(first) = closed.into_iter().next() else {
        return Ok(ExitCode::SUCCESS);
    };
    print_plan(
        backend,
        &configuration.mismatch,
        reading,
        &configuration.planning(),
        first.body,
        &summary,
        output.writer(),
    )
}

/// Refuse a context whose request with no question passes a limit, before
/// any record is read.
pub(super) fn check_context(
    planner: &Planner<'_>,
    backend: &Backend,
    packing: Packing,
) -> Result<(), Failure> {
    packer(planner, backend, packing).map(|_| ())
}

fn packer(
    planner: &Planner<'_>,
    backend: &Backend,
    packing: Packing,
) -> Result<Packer<()>, Failure> {
    let limits = PackLimits {
        image_ceiling: backend.image_ceiling(),
        ceiling: backend.ceiling(),
        profile: planner.profile.cloned(),
        inputs: packing.inputs.unwrap_or(MOST_INPUTS).max(1),
        questions: None,
        context: packing.context,
    };
    let model = pack::model_json(backend.model().as_str())
        .map_err(|_| Failure::Defect("a model could not be written as JSON"))?;
    let packer = Packer::new(limits, model);
    if let Some(context) = planner.context.as_ref() {
        let state = pack::state(context)
            .map_err(|_| Failure::Defect("a context could not be written as JSON"))?;
        packer
            .check_state(&state)
            .map_err(|error| planner.refused(error))?;
    }
    Ok(packer)
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
