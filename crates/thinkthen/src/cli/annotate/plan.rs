//! The grouped dry-run boundary.

use std::io::Write;
use std::process::ExitCode;

use crate::core::{Backend, BackendProfile, PlanDocument, QuestionSet, Reading, Record, json_line};
use crate::edge;
use crate::engine::facade;
use crate::failure::Failure;
use crate::profile::Mismatch;

use super::{collisions, plan_for};

#[expect(
    clippy::too_many_arguments,
    reason = "the dry-run boundary receives each already-resolved command concern once"
)]
pub(super) fn dry_run(
    set: &QuestionSet,
    backend: &Backend,
    base: &Reading,
    profile: Option<&BackendProfile>,
    mismatch: &Mismatch,
    first: Option<Vec<u8>>,
    details: bool,
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    let Some(bytes) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    let record = base
        .annotation_record(&bytes)
        .map_err(|error| Failure::record(error, base.streams()))?;
    dry_run_record(
        set,
        backend,
        base,
        profile,
        mismatch,
        Some(record),
        details,
        writer,
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "table and byte inputs share one exact grouped-plan boundary"
)]
pub(super) fn dry_run_record(
    set: &QuestionSet,
    backend: &Backend,
    base: &Reading,
    profile: Option<&BackendProfile>,
    mismatch: &Mismatch,
    first: Option<Record>,
    details: bool,
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    let Some(record) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    if !details {
        collisions(set, &record)?;
    }
    let plans = set
        .groups()
        .into_iter()
        .map(|group| {
            plan_for(set, &group, backend, base, &record).map_err(|error| error.into_failure())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let chunks = plans
        .iter()
        .map(|plan| facade::split(backend, profile, plan))
        .collect::<Result<Vec<_>, _>>()?;
    mismatch.print_once()?;
    let first = chunks
        .first()
        .and_then(|group| group.first())
        .ok_or(Failure::Defect("a set has no group"))?;
    let on = set
        .questions()
        .iter()
        .map(|question| (question.name().to_owned(), question.on().to_vec()))
        .collect();
    let document = PlanDocument::of(backend, &first.plan)
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?
        .questions_on(on)
        .requests(chunks.iter().map(Vec::len).collect());
    let document = if base.streams() {
        document.reading(base)
    } else {
        document
    };
    edge::write_line(writer, &json_line(&document)?)?;
    Ok(ExitCode::SUCCESS)
}
