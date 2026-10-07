//! The annotate dry run: every record's questions packed as a run would
//! pack them, with nothing looked up.

use std::collections::BTreeSet;
use std::io::Write;
use std::process::ExitCode;
use std::sync::Arc;

use super::Judging;
use super::asker::{self, Framed};
use crate::core::adapters::built_in;
use crate::core::pack::{self, Entry, PackError, Packer};
use crate::core::{PlanDocument, PlanSummary, Reading, json_line};
use crate::edge;
use crate::engine::pipeline::Packing;
use crate::failure::Failure;
use crate::schedule::Placed;

pub(super) fn dry_run(
    judging: &Judging<'_>,
    reading: &Reading,
    inputs: impl Iterator<Item = Result<Framed, Placed>>,
    inputs_cap: Option<usize>,
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    let engine = judging.engine();
    let backend = engine.backend();
    let limits = engine.pack_limits(Packing {
        questions: None,
        sized: true,
        inputs: inputs_cap,
        context: false,
        detailed: false,
        continues: false,
    });
    let model = pack::model_json(backend.model().as_str())
        .map_err(|_| Failure::Defect("a model could not be written as JSON"))?;
    let mut packer = Packer::new(limits, model);
    let mut summary = PlanSummary::new(false);
    let mut occurrences = 0_usize;
    let mut group_requests = vec![0; judging.groups().len()];
    let mut closed = Vec::new();
    let parser = asker::Parser::of(judging, reading);
    for framed in inputs {
        let record = parser.record(framed.map_err(|placed| placed.cause)?)?;
        let asks = asker::asks(judging, reading, backend.url(), &record)
            .map_err(super::PrepareError::into_failure)?;
        summary
            .record()
            .map_err(|_| Failure::Defect("a plan is too large"))?;
        let entries: Vec<_> = asks
            .into_iter()
            .flat_map(|(group, asks)| asks.into_iter().map(move |ask| (group, ask)))
            .map(|(group, ask)| Entry {
                state: ask.state.clone(),
                question: Arc::clone(&ask.question),
                options: 0,
                item: group,
            })
            .collect();
        occurrences = occurrences
            .checked_add(entries.len())
            .ok_or(Failure::Defect("a plan is too large"))?;
        packer
            .add(entries, &mut closed)
            .map_err(|error| match error {
                PackError::Profile(limit) => Failure::ProfileLimit(limit),
                PackError::Context { .. } => Failure::Defect("annotate packed a context"),
            })?;
        for request in closed.drain(..) {
            preview_request(
                &mut summary,
                &mut group_requests,
                &request.body,
                &request.items,
            )?;
        }
    }
    if let Some(request) = packer.close() {
        preview_request(
            &mut summary,
            &mut group_requests,
            &request.body,
            &request.items,
        )?;
    }
    let count = summary
        .counts()
        .map_err(|_| Failure::Defect("a plan is too large"))?
        .requests;
    summary.bound_requests(occurrences);
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

fn preview_request(
    summary: &mut PlanSummary,
    group_requests: &mut [usize],
    body: &[u8],
    groups: &[usize],
) -> Result<(), Failure> {
    summary
        .request(body)
        .map_err(|_| Failure::Defect("a plan is too large"))?;
    for group in groups.iter().collect::<BTreeSet<_>>() {
        if let Some(count) = group_requests.get_mut(*group) {
            *count = count
                .checked_add(1)
                .ok_or(Failure::Defect("a plan is too large"))?;
        }
    }
    Ok(())
}
