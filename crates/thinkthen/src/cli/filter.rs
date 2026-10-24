//! Filter rendering retains each original record's bytes.

use std::cell::Cell;
use std::io::{self, Write};

use thinkthen::{Cancel, Error, Filter, Outcome, Record};

use super::{DecideArgs, InputMode, evidence, family_code, input, local, print, report, settings};

pub(super) fn run(args: DecideArgs, cancel: Cancel) -> Result<u8, Error> {
    let question = Filter::new(args.question)?;
    let question = match args.threshold {
        Some(text) => question.threshold(text.parse()?),
        None => question,
    };
    let args = args.common;
    let (engine, on) = settings(&args, cancel)?;
    let mut unread = None;
    let mut unwritten = None;
    let writing = Cell::new(true);
    let stream = args.lines || args.jsonl;
    let mode = if stream {
        InputMode::Lines
    } else {
        InputMode::Single
    };
    let records = input(mode, &mut unread)
        .take_while(|_| writing.get())
        .map(|original| {
            let record = if args.jsonl {
                Record::json(&original, &on)
            } else {
                let evidence = if stream {
                    evidence(&original)
                } else {
                    &original
                };
                Record::text(evidence)
            };
            (record, original)
        });
    let result = engine.filter_rows(&question, records, |original, row| {
        if let Outcome::Failed { error } = row.outcome()
            && !args.details
        {
            report(error);
        }
        if unwritten.is_none() {
            let result = if args.details {
                print(&row, true)
            } else if thinkthen::passes(row.outcome()) {
                io::stdout()
                    .lock()
                    .write_all(original.as_bytes())
                    .map_err(|_| local("standard output could not be written"))
            } else {
                Ok(())
            };
            unwritten = result.err();
            writing.set(unwritten.is_none());
        }
    });
    let mut first_failed = result.err().map(|error| error.kind().family());
    if let Some(error) = unread.or(unwritten) {
        report(&error);
        first_failed.get_or_insert(error.kind().family());
    }
    Ok(first_failed.map_or(0, family_code))
}
