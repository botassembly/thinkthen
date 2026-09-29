//! Full-input preview over the same `Former` and batcher as execution.

use std::collections::VecDeque;
use std::process::ExitCode;

use super::{Former, Item, Records};
use crate::asking::{JudgingInput, print_plan};
use crate::core::{Plan, PlanSummary};
use crate::engine::facade::Input;
use crate::failure::Failure;
use crate::schedule::{Output, Placed};

pub(super) fn run(
    mut former: Former,
    records: Records,
    configuration: &JudgingInput<'_>,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let mut summary = PlanSummary::new(false);
    let mut first = None;
    for record in records {
        former.push(record);
        tally(&mut former.queue, &mut summary, &mut first)?;
    }
    former.end();
    tally(&mut former.queue, &mut summary, &mut first)?;
    let Some(first) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    print_plan(
        &configuration.backend,
        &configuration.mismatch,
        &former.reading,
        &configuration.planning(),
        &first,
        &summary,
        output.writer(),
    )
}

fn tally(
    queue: &mut VecDeque<Input<Item, Placed>>,
    summary: &mut PlanSummary,
    first: &mut Option<Plan>,
) -> Result<(), Failure> {
    while let Some(event) = queue.pop_front() {
        match event {
            Input::Item(item) => {
                summary
                    .records_added(item.records.len())
                    .map_err(|_| Failure::Defect("a plan is too large"))?;
                summary
                    .batch(&item.batch)
                    .map_err(|_| Failure::Defect("a plan is too large"))?;
                if first.is_none() {
                    *first = Some(item.batch.plan);
                }
            }
            Input::Failed(error) => return Err(error.cause),
            Input::End => {}
        }
    }
    Ok(())
}
