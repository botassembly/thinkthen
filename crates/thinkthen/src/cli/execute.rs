//! Shared stream execution, progress and command completion reduction.
use super::{
    input::{Input, InputEvent},
    options::{self, Output},
};
use crate::contract_driver::driver::{Driver, Event};
use crate::results::{AnyRow, Outcome};
use crate::{Cancel, Engine, Error, Request, Status};
use clap::ArgMatches;
use std::io::{self, IsTerminal, Write};

pub(super) fn invocation(function: &str, args: &ArgMatches, cancel: &Cancel) -> Result<u8, Error> {
    let output = Output::resolve(args)?;
    let (input, target) = if function == "run" {
        let (target, files) = super::saved_run::target(args)?;
        (Input::from_files(function, args, files)?, Some(target))
    } else {
        (Input::resolve(function, args)?, None)
    };
    if output.check {
        input.check(function)?;
    }
    super::table::validate(&input, args, output.json)?;
    if !output.quiet
        && input.sources.iter().any(|(path, _)| path == "-")
        && io::stdin().is_terminal()
    {
        let _ = writeln!(
            io::stderr().lock(),
            "reading standard input; press Ctrl-D to end"
        );
    }
    if target.is_some() && !input.table() && options::explicit(args, "into").is_some() {
        return Err(options::usage(
            "into",
            "a direct function call",
            "saved run",
        ));
    }
    if let Some(target) = target {
        return super::saved_run::run(args, &target, input, &output, cancel);
    }
    let request = super::invocation::request(function, args)?;
    if !input.table() && options::explicit(args, "into").is_some() {
        return super::jsonl::run(input, args, &request, &output, cancel);
    }
    let engine = options::client(args)?;
    if input.table() {
        super::table::run(&input, args, &request, &engine, cancel, output.quiet)
    } else {
        stream(input, &output, &request, &engine, cancel)
    }
}

pub(super) fn stream(
    input: Input,
    output: &Output,
    request: &Request,
    engine: &Engine,
    cancel: &Cancel,
) -> Result<u8, Error> {
    let mut items =
        (!output.quiet && output.json && matches!(request.question(), crate::Question::Filter(_)))
            .then(std::collections::VecDeque::new);
    let originals = !output.quiet
        && !output.json
        && matches!(
            request.question(),
            crate::Question::Filter(_) | crate::Question::Rank(_)
        );
    let mut originals = originals.then(super::render::Originals::new).transpose()?;
    let single_text = input.sources.len() == 1
        && input
            .sources
            .iter()
            .all(|(_, format)| *format == super::input::Format::Text);
    let multiple_text = input.sources.len() > 1
        && input
            .sources
            .iter()
            .all(|(_, format)| *format == super::input::Format::Text);
    let keyed = input.sources.len() > 1
        || input
            .sources
            .iter()
            .any(|(_, format)| *format != super::input::Format::Text);
    let selected_field = !input.selection.is_empty();
    let mut input = super::input::StreamInput::new(input, cancel.clone())?;
    let engine = engine.clone().cancel(cancel.clone());
    let mut call = if matches!(request.question(), crate::Question::Rank(_)) {
        let mut records = Vec::new();
        while let Some((record, bytes)) = input.next()? {
            if cancel.is_cancelled() {
                return Err(crate::retry::cancelled());
            }
            if let Some(originals) = &mut originals {
                originals.push(&bytes)?;
            }
            records.push(record);
        }
        Driver::open_records(&engine, request, records, crate::OffsetUnit::CodePoints)?
    } else {
        Driver::open(&engine, request, crate::OffsetUnit::CodePoints)?
    };
    let mut completion = Completion::default();
    let start = std::time::Instant::now();
    loop {
        if cancel.is_cancelled() {
            return Err(crate::retry::cancelled());
        }
        match call.next_event()? {
            Event::NeedsInput => match input.poll()? {
                InputEvent::Record(record, bytes) => {
                    if let Some(items) = &mut items {
                        let format = if selected_field && input.format == super::input::Format::Text
                        {
                            super::input::Format::Jsonl
                        } else {
                            input.format
                        };
                        items.push_back(super::render::item(&bytes, format)?);
                    }
                    if let Some(originals) = &mut originals {
                        originals.push(&bytes)?;
                    }
                    call.send(record)?;
                }
                InputEvent::Finished => call.end_input(),
                InputEvent::Pending => {}
            },
            Event::Row(row) => {
                if !output.quiet && io::stderr().is_terminal() {
                    let _ = write!(io::stderr().lock(), "\r\x1b[2K");
                }
                if single_text
                    && row
                        .failure()
                        .is_some_and(|error| error.code() == crate::Code::RecordEmpty)
                {
                    if let Some(error) = row.failure() {
                        super::report(error);
                    }
                    return Ok(2);
                }
                if completion.rows == 0
                    && let Some(error) = row.failure()
                    && error.code() == crate::Code::BackendTransport
                {
                    return Err(error.clone());
                }
                completion.row(&row);
                report_failure(&row, input.origin(row.position()));
                if output.quiet {
                    continue;
                }
                if let Some(items) = &mut items {
                    let item = items.pop_front().ok_or_else(|| {
                        Error::from_contract(
                            crate::Code::InternalBug,
                            "the engine returned a row without an original item",
                        )
                    })?;
                    super::render::filter_row(&row, &item, cancel)?;
                } else if let Some(originals) = &mut originals {
                    if super::render::keeps(&row) {
                        originals.write(row.position(), cancel)?;
                    }
                } else {
                    let key = keyed.then(|| {
                        if multiple_text {
                            input.origin(row.position()).to_owned()
                        } else {
                            row.id()
                                .map(record_key)
                                .unwrap_or_else(|| row.position().to_string())
                        }
                    });
                    super::render::keyed_row(&row, output.json, key.as_deref(), cancel)?;
                }
                if !output.quiet && io::stderr().is_terminal() {
                    let _ = write!(
                        io::stderr().lock(),
                        "\rthinkthen: {} rows, {} failed",
                        completion.rows,
                        completion.failed
                    );
                }
            }
            Event::Finished => break,
        }
    }
    if !output.quiet && io::stderr().is_terminal() {
        let _ = write!(io::stderr().lock(), "\r\x1b[2K");
    }
    if output.verbose {
        let _ = writeln!(
            io::stderr().lock(),
            "thinkthen: {} rows, {} failed, {:.3}s",
            completion.rows,
            completion.failed,
            start.elapsed().as_secs_f64()
        );
    }
    Ok(completion.code(output.check))
}

#[derive(Default)]
pub(super) struct Completion {
    rows: u64,
    failed: u64,
    backend_failed: u64,
    yes: bool,
    unsure: u64,
}
impl Completion {
    pub(super) const fn rows(&self) -> u64 {
        self.rows
    }
    pub(super) fn row(&mut self, row: &AnyRow) {
        self.rows += 1;
        if row.failure().is_some() {
            self.failed += 1;
            if row.status() == Status::Failed
                && row
                    .failure()
                    .is_some_and(|error| error.code().family() == crate::Family::Backend)
            {
                self.backend_failed += 1;
            }
        }
        self.unsure += u64::from(row.status() == Status::Unsure);
        self.yes |= match row {
            AnyRow::Decide(r) => matches!(r.outcome(), Outcome::Answered { answer: true, .. }),
            AnyRow::Filter(r) => matches!(r.outcome(), Outcome::Answered { answer: true, .. }),
            AnyRow::Choose(r) => matches!(r.outcome(), Outcome::Answered { .. }),
            AnyRow::Find(r) => matches!(
                r.outcome(),
                Outcome::Answered {
                    answer: Some(_),
                    ..
                }
            ),
            _ => false,
        };
    }
    pub(super) fn summarize(&self, quiet: bool) {
        super::table::report_counts(self.unsure, self.failed, quiet);
    }
    pub(super) const fn code(&self, check: bool) -> u8 {
        if self.rows > 0 && self.backend_failed == self.rows {
            4
        } else if self.failed > 0 {
            6
        } else if !check || self.yes {
            0
        } else if self.unsure > 0 {
            3
        } else {
            1
        }
    }
}

pub(super) fn report_failure(row: &AnyRow, origin: &str) {
    if let Some(error) = row.failure() {
        let id = row.id().map(record_key).unwrap_or_else(|| "none".into());
        let _ = writeln!(
            io::stderr().lock(),
            "thinkthen: {}: position {} id {} file {}: {error}",
            error.code().as_str(),
            row.position(),
            crate::escape_diagnostic(&id),
            crate::escape_diagnostic(origin)
        );
    }
}

fn record_key(id: &crate::RecordId) -> String {
    match id {
        crate::RecordId::Integer(value) => value.to_string(),
        crate::RecordId::Unsigned(value) => value.to_string(),
        crate::RecordId::String(value) => value.clone(),
    }
}
