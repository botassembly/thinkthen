//! Command framing and output around the engine-owned grouped scheduler.

use std::cell::Cell;
use std::io::Write;
use std::process::ExitCode;
use std::sync::mpsc::Receiver;
use std::thread;

use crate::annotate::{GroupAnswer, Judging, PrepareError, PreparedGroup, check_model};
use crate::core::{ModelName, Reading, Record};
use crate::engine::facade::{
    GroupOutcome as RunOutcome, GroupPort as InputPort, Input as EngineInput, Prepared,
};
use crate::failure::{Failure, ReplayContext};
use crate::schedule::{Judged, Output, Placed};

struct Work {
    group: PreparedGroup,
    at: usize,
    ordinal: usize,
    members: usize,
}

struct Answers {
    ordered: Vec<GroupAnswer>,
    model: Option<ModelName>,
    at: usize,
}

enum Seed {
    Record(Record),
    Missing { at: usize, pointer: String },
}

struct Finished {
    judged: Judged,
    skipped: bool,
}

type PreparedInput = Prepared<Seed, Answers, Work>;

pub(crate) enum Input {
    Bytes(usize, Vec<u8>),
    Record(usize, Record),
}

pub(crate) fn run<I>(
    judging: &Judging<'_>,
    reading: &Reading,
    chunks: I,
    cancel: &crate::engine::Cancel,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure>
where
    I: Iterator<Item = Result<Input, Placed>> + Send + 'static,
{
    let skipped = Cell::new(0usize);
    let outcome = judging.engine().groups(
        reading.streams(),
        cancel,
        |requests, events| {
            thread::spawn(move || read(chunks, &requests, &events));
        },
        |input| prepare(judging, reading, input),
        &|work: Work| {
            judging.answer_group(work.group).map_err(|error| {
                let source = ReplayContext::AnnotateGroup {
                    ordinal: work.ordinal,
                    members: work.members,
                };
                Placed::at(error.with_replay_context(source), work.at)
            })
        },
        |answers, answer| {
            let model = answer.model.as_ref().ok_or_else(|| {
                Placed::at(
                    Failure::Defect("an annotate group reported no model"),
                    answers.at,
                )
            })?;
            check_model(&mut answers.model, model, judging.requested_model())
                .map_err(|error| Placed::at(error.into(), answers.at))?;
            answers.ordered.push(answer);
            Ok(())
        },
        |seed, answers| {
            let finished = match seed {
                Seed::Record(record) => Finished {
                    judged: judging
                        .finish(record, answers.ordered)
                        .map_err(|error| Placed::at(error, answers.at))?,
                    skipped: false,
                },
                Seed::Missing { at, pointer } => Finished {
                    judged: crate::annotate::error_row::missed(at, &pointer)
                        .map_err(|error| Placed::at(error, at))?,
                    skipped: true,
                },
            };
            let (replayed, partial) = (finished.judged.replayed, finished.judged.partial_failure);
            Ok(crate::engine::facade::Completed::one(
                finished, replayed, partial,
            ))
        },
        |finished| {
            let emitted = output.take(finished.judged).map_err(Placed::from)?;
            if emitted && finished.skipped {
                skipped.set(skipped.get() + 1);
            }
            Ok(emitted)
        },
    );
    if skipped.get() > 0 {
        let count = skipped.get();
        writeln!(
            std::io::stderr().lock(),
            "thinkthen: {count} record{} skipped",
            if count == 1 { "" } else { "s" }
        )
        .map_err(Failure::Output)?;
    }
    let outcome = outcome?;
    match outcome {
        RunOutcome::Complete { .. } if skipped.get() > 0 => Ok(ExitCode::from(7)),
        RunOutcome::Complete { partial_failure } => Ok(if partial_failure {
            ExitCode::from(6)
        } else {
            ExitCode::SUCCESS
        }),
        RunOutcome::Failed(cause) => Err(cause.cause),
        RunOutcome::Stopped {
            finished,
            replayed,
            cause,
        } => Err(Failure::Stopped {
            at: cause.at.unwrap_or(finished + 1),
            finished,
            replayed,
            recording: judging.engine().recording(),
            held: false,
            cause: Box::new(cause.cause),
        }),
    }
}

fn prepare(
    judging: &Judging<'_>,
    reading: &Reading,
    input: Input,
) -> Result<PreparedInput, Placed> {
    let at = match &input {
        Input::Bytes(at, _) | Input::Record(at, _) => *at,
    };
    let record = judging
        .record(reading, input)
        .map_err(|error| Placed::at(error, at))?;
    let work = judging
        .groups()
        .into_iter()
        .enumerate()
        .map(|(index, places)| {
            let members = places.len();
            judging
                .prepare_group(reading, &record, places)
                .map(|group| Work {
                    group,
                    at,
                    ordinal: index + 1,
                    members,
                })
        })
        .collect::<Result<Vec<_>, _>>();
    let work = match work {
        Ok(work) => work,
        Err(PrepareError::MissingOn(pointer)) if judging.continue_missing() => {
            return Ok(Prepared {
                seed: Seed::Missing { at, pointer },
                accumulator: Answers {
                    ordered: Vec::new(),
                    model: None,
                    at,
                },
                work: Vec::new(),
            });
        }
        Err(error) => return Err(Placed::at(error.into_failure(), at)),
    };
    Ok(Prepared {
        seed: Seed::Record(record),
        accumulator: Answers {
            ordered: Vec::with_capacity(work.len()),
            model: None,
            at,
        },
        work,
    })
}

fn read<I>(mut chunks: I, requests: &Receiver<()>, events: &InputPort<Input, GroupAnswer, Placed>)
where
    I: Iterator<Item = Result<Input, Placed>>,
{
    while requests.recv().is_ok() {
        let event = match chunks.next() {
            None => EngineInput::End,
            Some(Ok(input)) => EngineInput::Item(input),
            Some(Err(error)) => EngineInput::Failed(error),
        };
        if events.send(event).is_err() {
            return;
        }
    }
}
