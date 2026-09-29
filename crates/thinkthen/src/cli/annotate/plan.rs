//! The grouped dry-run boundary.

use std::io::Write;
use std::process::ExitCode;

use crate::core::{
    Backend, Plan, PlanDocument, PlanSummary, QuestionSet, Reading, Record, Setting, json_line,
};
use crate::edge;
use crate::engine::facade::{Engine, GroupPlanError, GroupPlanner, GroupRequest, GroupWork};
use crate::failure::Failure;
use crate::profile::Mismatch;

use super::collisions;

#[expect(
    clippy::too_many_arguments,
    reason = "the dry-run boundary receives each already-resolved command concern once"
)]
pub(super) fn dry_run(
    set: &QuestionSet,
    backend: &Backend,
    base: &Reading,
    engine: &Engine,
    setting: Setting,
    mismatch: &Mismatch,
    records: impl Iterator<Item = Result<Vec<u8>, Failure>>,
    details: bool,
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    dry_run_record(
        set,
        backend,
        base,
        engine,
        setting,
        mismatch,
        records.map(|bytes| {
            let bytes = bytes?;
            base.annotation_record(&bytes)
                .map_err(|error| Failure::record(error, base.streams()))
        }),
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
    engine: &Engine,
    setting: Setting,
    mismatch: &Mismatch,
    records: impl Iterator<Item = Result<Record, Failure>>,
    details: bool,
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    let mut summary = PlanSummary::new(false);
    let mut first: Option<Plan> = None;
    let mut group_requests = vec![0; set.groups().len()];
    let mut planner =
        GroupPlanner::new(engine, set, setting).map_err(|error| planner_error(error, engine))?;
    for (row, record) in records.enumerate() {
        let record = record?;
        if !details {
            collisions(set, &record)?;
        }
        let batch = base.batch_record(&record)?;
        let _slots = planner
            .push_sliced(engine, set, &batch, row)
            .map_err(|error| planner_error(error, engine))?;
        summary
            .record()
            .map_err(|_| Failure::Defect("a plan is too large"))?;
        while let Some(work) = planner.pop_sliced_at(usize::MAX) {
            tally(work, &mut summary, &mut first, &mut group_requests)?;
        }
    }
    planner
        .finish()
        .map_err(|error| planner_error(error, engine))?;
    while let Some(work) = planner.pop_sliced_at(usize::MAX) {
        tally(work, &mut summary, &mut first, &mut group_requests)?;
    }
    let Some(first) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    mismatch.print_once()?;
    let on = set
        .questions()
        .iter()
        .map(|question| (question.name().to_owned(), question.on().to_vec()))
        .collect();
    let document = PlanDocument::of(backend, &first)
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?
        .questions_on(on)
        .requests(group_requests);
    if summary.first_body() != Some(document.request_body()) {
        return Err(Failure::Defect(
            "the disclosed request changed after preparation",
        ));
    }
    let document = if base.streams() {
        document.reading(base)
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

fn tally(
    work: GroupWork,
    summary: &mut PlanSummary,
    first: &mut Option<Plan>,
    group_requests: &mut [usize],
) -> Result<(), Failure> {
    let group = group_requests
        .get_mut(work.group)
        .ok_or(Failure::Defect("an annotation group disappeared"))?;
    let mut count = |plan: Plan, body: &[u8]| -> Result<(), Failure> {
        summary
            .request(body)
            .map_err(|_| Failure::Defect("a plan is too large"))?;
        *group = group
            .checked_add(1)
            .ok_or(Failure::Defect("a plan is too large"))?;
        if first.is_none() {
            *first = Some(plan);
        }
        Ok(())
    };
    match work.request {
        GroupRequest::Packed { batch, .. } => count(batch.plan, &batch.body),
        GroupRequest::Legacy(prepared) => {
            let prepared = prepared
                .into_inner()
                .map_err(|_| Failure::Defect("an annotation plan was poisoned"))?
                .ok_or(Failure::Defect("an annotation plan disappeared"))?;
            for chunk in prepared.chunks {
                count(chunk.plan, &chunk.request.body)?;
            }
            Ok(())
        }
    }
}

fn planner_error(error: GroupPlanError, engine: &Engine) -> Failure {
    match error {
        GroupPlanError::Part(crate::core::PartError::Reading(error)) => error.into(),
        GroupPlanError::Part(crate::core::PartError::Record(error)) => error.into(),
        GroupPlanError::Batch(error) => {
            crate::failure::context::Limits::new(engine.profile()).refused(error, false)
        }
        GroupPlanError::Defect(message) => Failure::Defect(message),
        GroupPlanError::Engine(error) => error.into(),
    }
}
