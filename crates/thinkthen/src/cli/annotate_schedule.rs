//! Command framing and output around the engine-owned grouped scheduler.

use std::process::ExitCode;
use std::sync::mpsc::Receiver;
use std::thread;

use crate::annotate::{GroupAnswer, Judging, PreparedGroup, check_model};
use crate::core::{ModelName, Reading, Record};
use crate::engine::annotate_schedule::{
    self as engine_schedule, InputPort, Outcome as RunOutcome, Prepared,
};
use crate::engine::schedule::{Completed, Input as EngineInput};
use crate::failure::Failure;
use crate::schedule::Output;

struct Work {
    group: PreparedGroup,
}

struct Answers {
    ordered: Vec<GroupAnswer>,
    model: Option<ModelName>,
}

pub(crate) enum Input {
    Bytes(Vec<u8>),
    Record(Record),
}

pub(crate) fn run<I>(
    judging: &Judging<'_>,
    reading: &Reading,
    chunks: I,
    jobs: usize,
    cancel: &crate::engine::Cancel,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure>
where
    I: Iterator<Item = Result<Input, Failure>> + Send + 'static,
{
    let outcome = engine_schedule::run(
        jobs,
        reading.streams(),
        cancel,
        |requests, events| {
            thread::spawn(move || read(chunks, &requests, &events));
        },
        |input| prepare(judging, reading, input),
        &|work: Work| judging.answer_group(work.group),
        |answers, answer| {
            let model = answer
                .model
                .as_ref()
                .ok_or(Failure::Defect("an annotate group reported no model"))?;
            check_model(&mut answers.model, model, judging.requested_model())?;
            answers.ordered.push(answer);
            Ok(())
        },
        |record, answers| {
            judging
                .finish(record, answers.ordered)
                .map(|judged| Completed {
                    replayed: judged.replayed,
                    partial_failure: judged.partial_failure,
                    value: judged,
                })
        },
        |judged| output.take(judged),
        Failure::Defect,
        || Failure::from(crate::engine::error::Error::Cancelled),
    )?;
    match outcome {
        RunOutcome::Complete { partial_failure } => Ok(if partial_failure {
            ExitCode::from(6)
        } else {
            ExitCode::SUCCESS
        }),
        RunOutcome::Failed(cause) => Err(cause),
        RunOutcome::Stopped {
            at,
            finished,
            replayed,
            cause,
        } => Err(Failure::Stopped {
            at,
            finished,
            replayed,
            recording: judging.recording_named(),
            held: false,
            cause: Box::new(cause),
        }),
    }
}

fn prepare(
    judging: &Judging<'_>,
    reading: &Reading,
    input: Input,
) -> Result<Prepared<Record, Answers, Work>, Failure> {
    let record = judging.record(reading, input)?;
    let work = judging
        .groups()
        .into_iter()
        .map(|places| {
            judging
                .prepare_group(reading, &record, places)
                .map(|group| Work { group })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Prepared {
        seed: record,
        accumulator: Answers {
            ordered: Vec::with_capacity(work.len()),
            model: None,
        },
        work,
    })
}

fn read<I>(mut chunks: I, requests: &Receiver<()>, events: &InputPort<Input, GroupAnswer, Failure>)
where
    I: Iterator<Item = Result<Input, Failure>>,
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
