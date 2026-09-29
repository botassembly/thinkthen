//! The plan `--dry-run` prints in place of a request.

use std::io::{self, IsTerminal, Write};
use std::process::ExitCode;

use super::{Asks, JudgingInput, asked_of};
use crate::core::{
    Backend, BackendProfile, Plan, PlanDocument, Reading, Record, Sources, json_line,
};
use crate::edge;
use crate::engine::facade;
use crate::failure::Failure;
use crate::profile::Mismatch;

/// What a plan shows beyond the request: the question and where it came from.
pub(super) struct Planning<'a> {
    pub(super) asks: &'a Asks,
    pub(super) sources: Option<Sources>,
}

impl JudgingInput<'_> {
    pub(super) const fn planning(&self) -> Planning<'_> {
        Planning {
            asks: &self.asks,
            sources: self.sources,
        }
    }
}

/// Print the plan for the first record, and read no further than that record.
#[expect(
    clippy::too_many_arguments,
    reason = "byte and table inputs share one plan path with explicit profile state"
)]
pub(super) fn plan(
    backend: &Backend,
    profile: Option<&BackendProfile>,
    mismatch: &Mismatch,
    reading: &Reading,
    planning: &Planning<'_>,
    first: Option<Vec<u8>>,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    let Some(bytes) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    let record = reading
        .record(&bytes)
        .map_err(|error| Failure::record(error, reading.streams()))?;
    plan_record(
        backend,
        profile,
        mismatch,
        reading,
        planning,
        Some(record),
        writer,
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "the plan path receives each resolved concern without a second configuration type"
)]
pub(super) fn plan_record(
    backend: &Backend,
    profile: Option<&BackendProfile>,
    mismatch: &Mismatch,
    reading: &Reading,
    planning: &Planning<'_>,
    first: Option<Record>,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    let Some(record) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    let sending = asked_of(reading, record, planning.asks)?;
    let plan = Plan::new(
        sending.evidence,
        backend.model().clone(),
        vec![sending.question],
    )
    .map_err(|_| Failure::Defect("a plan of one question asks nothing"))?;
    let _prepared = facade::split(backend, profile, &plan)?;
    print_plan(backend, mismatch, reading, planning, &plan, writer)
}

/// Print the plan document of one checked plan.
pub(super) fn print_plan(
    backend: &Backend,
    mismatch: &Mismatch,
    reading: &Reading,
    planning: &Planning<'_>,
    plan: &Plan,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    mismatch.print_once()?;
    let document = PlanDocument::of(backend, plan)
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?;
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
            "thinkthen: dry-run: request.state is the evidence; request.questions holds what you asked about it."
        )
        .and_then(|()| stderr.flush())
        .map_err(Failure::Output)?;
    }
    edge::write_line(writer, &line)?;
    Ok(ExitCode::SUCCESS)
}
