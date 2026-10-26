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
    let input = Input::resolve(function, args)?;
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
    if function == "run" {
        return super::saved_run::run(args, input, &output, cancel);
    }
    let request = super::invocation::request(function, args)?;
    let engine = options::client(args)?;
    if input.table() {
        super::table::run(&input, args, &request, &engine, cancel)
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
    let originals = !output.json
        && matches!(
            request.question(),
            crate::Question::Filter(_) | crate::Question::Rank(_)
        );
    let mut originals = originals.then(super::render::Originals::new).transpose()?;
    let multiple_sources = input.sources.len() > 1;
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
                    if let Some(originals) = &mut originals {
                        originals.push(&bytes)?;
                    }
                    call.send(record)?;
                }
                InputEvent::Finished => call.end_input(),
                InputEvent::Pending => {}
            },
            Event::Row(row) => {
                completion.row(&row);
                if let Some(error) = row.failure() {
                    if multiple_sources {
                        let _ = writeln!(
                            io::stderr().lock(),
                            "thinkthen: input {} position {}",
                            input.origin(row.position()),
                            row.position()
                        );
                    }
                    super::report(error);
                }
                if let Some(originals) = &mut originals {
                    if super::render::keeps(&row) {
                        originals.write(row.position(), cancel)?;
                    }
                } else {
                    super::render::row(&row, output.json, cancel)?;
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
        let _ = writeln!(io::stderr().lock());
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
struct Completion {
    rows: u64,
    failed: u64,
    yes: bool,
    unsure: bool,
}
impl Completion {
    fn row(&mut self, row: &AnyRow) {
        self.rows += 1;
        if row.failure().is_some() {
            self.failed += 1;
        }
        self.unsure |= row.status() == Status::Unsure;
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
    const fn code(&self, check: bool) -> u8 {
        if self.failed > 0 {
            6
        } else if !check || self.yes {
            0
        } else if self.unsure {
            3
        } else {
            1
        }
    }
}
