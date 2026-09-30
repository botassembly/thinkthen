//! The plan `--plan` prints in place of a request.

use std::io::{self, IsTerminal, Write};
use std::process::ExitCode;

use super::{Asks, JudgingInput, asked_of};
use crate::core::{
    Backend, BackendProfile, Plan, PlanDocument, PlanSummary, Reading, Record, Sources, json_line,
    quoted_plan,
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

/// Validate and prepare every record before printing the first prepared body.
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
    records: impl Iterator<Item = Result<Vec<u8>, Failure>>,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    plan_record(
        backend,
        profile,
        mismatch,
        reading,
        planning,
        records.map(|bytes| {
            let bytes = bytes?;
            reading
                .record(&bytes)
                .map_err(|error| Failure::record(error, reading.streams()))
        }),
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
    records: impl Iterator<Item = Result<Record, Failure>>,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    let mut summary = PlanSummary::new(false);
    let mut first = None;
    for record in records {
        let sending = asked_of(reading, record?, planning.asks)?;
        let plan = quoted_plan(
            backend.model().clone(),
            sending.evidence,
            None,
            vec![sending.question],
        )
        .map_err(|_| Failure::Defect("a plan of one question could not be quoted"))?;
        summary
            .record()
            .map_err(|_| Failure::Defect("a plan is too large"))?;
        for chunk in facade::split(backend, profile, &plan)? {
            summary
                .request(&chunk.request.body)
                .map_err(|_| Failure::Defect("a plan is too large"))?;
        }
        if first.is_none() {
            first = Some(plan);
        }
    }
    let Some(first) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    print_plan(
        backend, mismatch, reading, planning, &first, &summary, writer,
    )
}

/// Print the plan document of one checked plan.
#[expect(
    clippy::too_many_arguments,
    reason = "shared batched and unbatched previews pass resolved concerns without another configuration type"
)]
pub(super) fn print_plan(
    backend: &Backend,
    mismatch: &Mismatch,
    reading: &Reading,
    planning: &Planning<'_>,
    plan: &Plan,
    summary: &PlanSummary,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    mismatch.print_once()?;
    let document = PlanDocument::of(backend, plan)
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?;
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
            "thinkthen: plan: request.state is the evidence; request.questions holds what you asked about it."
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
