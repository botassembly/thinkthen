//! Rank rendering retains each original record's bytes.

use super::{InputMode, RankArgs, evidence, family_code, input, local, print, report, settings};
use std::io::{self, Write};
use thinkthen::{Cancel, Decide, Error, Rank, Record, Score};

pub(super) fn run(args: RankArgs, cancel: Cancel) -> Result<u8, Error> {
    let top = args.top.as_deref().map(str::parse).transpose()?;
    let question = if args.levels.is_empty() {
        Rank::Decide(Decide::new(args.question)?)
    } else {
        Rank::Score(Score::new(args.question, args.levels)?)
    };
    let (engine, on) = settings(&args.common, cancel)?;
    let mut unread = None;
    let mut unwritten = None;
    let stream = args.common.lines || args.common.jsonl;
    let mode = if stream {
        InputMode::Lines
    } else {
        InputMode::Single
    };
    let records = input(mode, &mut unread).map(|original| {
        let record = if args.common.jsonl {
            Record::json(&original, &on)
        } else {
            Record::text(if stream {
                evidence(&original)
            } else {
                &original
            })
        };
        (record, original)
    });
    let mut ranked = Vec::new();
    let mut first_failed = engine
        .rank(&question, records, top, |original, row| {
            ranked.push((original, row))
        })
        .err();
    if let Some(error) = unread {
        report(&error);
        return Ok(family_code(error.kind().family()));
    }
    for (original, row) in ranked {
        if unwritten.is_none() {
            unwritten = if args.common.details {
                print(&row, true)
            } else {
                io::stdout()
                    .lock()
                    .write_all(original.as_bytes())
                    .map_err(|_| local("standard output could not be written"))
            }
            .err();
        }
    }
    if let Some(error) = &first_failed {
        report(error);
    }
    if let Some(error) = unwritten {
        report(&error);
        first_failed.get_or_insert(error);
    }
    Ok(first_failed.map_or(0, |error| family_code(error.kind().family())))
}
