//! `decide`, `filter` and `rank` over a stream, by ADR 0048 items 1 to 6.
//!
//! A record thread reads raw records ahead through a bounded channel. The
//! batch reader parses them, plans batches, and hands the scheduler one batch
//! for each ask. A worker sends a batch as one request and builds one row for
//! each of its records, in input order.

use std::collections::VecDeque;
use std::io::BufRead;
use std::process::ExitCode;
use std::sync::mpsc::{Receiver, RecvTimeoutError, sync_channel};
use std::thread;
use std::time::Duration;

use super::{Asks, Judging, JudgingInput, Planning, print_plan, table_kind};
use crate::core::{
    AnswerOutcome, Batch, BatchError, Batcher, Question, Reading, Record, Reply, Setting, share,
};
use crate::edge;
use crate::engine::facade::{Answered, Completed, Input, InputPort, Judgment};
use crate::failure::Failure;
use crate::schedule::{self, Judged, Output};
use crate::table::Rows as TableRows;

/// How long input may pause before the open batch goes out, by ADR 0048 item 2.
const PAUSE: Duration = Duration::from_millis(50);

/// How many raw records the record thread reads ahead of the batch reader.
const AHEAD: usize = 64;

/// One parsed record, and the bytes it arrived as when it arrived as a line.
struct Held {
    record: Record,
    arrived: Option<Vec<u8>>,
}

/// One batch for a worker: its request, its first record's number, its records.
pub(super) struct Item {
    batch: Batch,
    first: usize,
    records: Vec<Held>,
}

type Parse<T> = fn(&Reading, T) -> Result<Held, Failure>;

fn line(reading: &Reading, bytes: Vec<u8>) -> Result<Held, Failure> {
    let record = reading
        .record(&bytes)
        .map_err(|error| Failure::record(error, true))?;
    Ok(Held {
        record,
        arrived: Some(bytes),
    })
}

fn row(_: &Reading, record: Record) -> Result<Held, Failure> {
    Ok(Held {
        record,
        arrived: None,
    })
}

/// Send the stream in batches of `setting`, or print the first batch's plan.
pub(super) fn run(
    configuration: JudgingInput<'_>,
    reading: &Reading,
    source: Box<dyn BufRead + Send>,
    setting: Setting,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    match table_kind(configuration.common) {
        Some(kind) => {
            let rows = TableRows::new(source, kind)?;
            over(configuration, reading, rows, row, setting, output)
        }
        None => {
            let lines = edge::Chunks::new(source, true);
            over(configuration, reading, lines, line, setting, output)
        }
    }
}

fn over<T, I>(
    configuration: JudgingInput<'_>,
    reading: &Reading,
    records: I,
    parse: Parse<T>,
    setting: Setting,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure>
where
    T: Send + 'static,
    I: Iterator<Item = Result<T, Failure>> + Send + 'static,
{
    let Asks::Fixed(question) = &configuration.asks else {
        return Err(Failure::Defect("a batched verb asks one question"));
    };
    let question = question.clone();
    let batcher = Batcher::new(
        configuration.backend.clone(),
        configuration.profile.clone(),
        question.clone(),
        setting,
        None,
    )
    .map_err(refused)?;
    let former = Former {
        batcher,
        reading: reading.clone(),
        held: Vec::new(),
        taken: 0,
        queue: VecDeque::new(),
    };
    if configuration.common.dry_run {
        return planned(former, records, parse, &configuration, output);
    }
    let judging = Judging::new(configuration)?;
    let recording = judging.engine.recording();
    let outcome = judging.engine.records(
        output.holds(),
        judging.environment.cancel(),
        |asks, events| {
            let (sender, raw) = sync_channel(AHEAD);
            thread::spawn(move || {
                for record in records {
                    let failed = record.is_err();
                    if sender.send(record).is_err() || failed {
                        return;
                    }
                }
            });
            thread::spawn(move || former.answer(parse, &raw, &asks, &events));
        },
        &|item| answered(&judging, reading, &question, item),
        |rows| {
            for row in rows {
                if !output.take(row)? {
                    return Ok(false);
                }
            }
            Ok(true)
        },
    )?;
    schedule::ended(outcome, recording, output)
}

/// Print the plan of the first batch that closes without a pause.
fn planned<T>(
    mut former: Former,
    records: impl Iterator<Item = Result<T, Failure>>,
    parse: Parse<T>,
    configuration: &JudgingInput<'_>,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    for record in records {
        former.push(parse, record);
        if !former.queue.is_empty() {
            break;
        }
    }
    if former.queue.is_empty() {
        former.end();
    }
    match former.queue.pop_front() {
        Some(Input::Item(item)) => print_plan(
            &configuration.backend,
            &configuration.mismatch,
            &former.reading,
            &Planning {
                asks: &configuration.asks,
                sources: configuration.sources,
            },
            &item.batch.plan,
            output.writer(),
        ),
        Some(Input::Failed(error)) => Err(error),
        Some(Input::End) | None => Ok(ExitCode::SUCCESS),
    }
}

/// The batch reader: the planner, the open batch's records, and the closed
/// batches and refusals that wait for an ask, in input order.
struct Former {
    batcher: Batcher,
    reading: Reading,
    held: Vec<Held>,
    /// How many records went into batches before the open one.
    taken: usize,
    queue: VecDeque<Input<Item, Failure>>,
}

impl Former {
    /// Answer each ask with the next batch, pulling records only while an
    /// ask waits, and sending the open batch when input pauses.
    fn answer<T>(
        mut self,
        parse: Parse<T>,
        raw: &Receiver<Result<T, Failure>>,
        asks: &Receiver<()>,
        events: &InputPort<Item, Vec<Judged>, Failure>,
    ) {
        let mut ended = false;
        while asks.recv().is_ok() {
            let event = loop {
                if let Some(event) = self.queue.pop_front() {
                    break event;
                }
                if ended {
                    break Input::End;
                }
                let next = if self.held.is_empty() {
                    raw.recv().ok()
                } else {
                    match raw.recv_timeout(PAUSE) {
                        Ok(record) => Some(record),
                        Err(RecvTimeoutError::Timeout) => {
                            let paused = self.batcher.pause();
                            self.closed(paused);
                            continue;
                        }
                        Err(RecvTimeoutError::Disconnected) => None,
                    }
                };
                match next {
                    Some(record) => self.push(parse, record),
                    None => {
                        self.end();
                        ended = true;
                    }
                }
            };
            if events.send(event).is_err() {
                return;
            }
        }
    }

    /// Parse one record and plan it, queueing the batches it closes. A record
    /// refused before planning sends the open batch first.
    fn push<T>(&mut self, parse: Parse<T>, raw: Result<T, Failure>) {
        let parsed = raw.and_then(|raw| parse(&self.reading, raw)).and_then(|held| {
            let record = self.reading.batch_record(&held.record)?;
            Ok((record, held))
        });
        let (record, held) = match parsed {
            Ok(parsed) => parsed,
            Err(error) => {
                self.end();
                self.queue.push_back(Input::Failed(error));
                return;
            }
        };
        self.held.push(held);
        let mut closed = Vec::new();
        let pushed = self.batcher.push(record, &mut closed);
        for batch in closed {
            self.queue_batch(batch);
        }
        if let Err(error) = pushed {
            self.held.clear();
            self.queue.push_back(Input::Failed(refused(error)));
        }
    }

    fn end(&mut self) {
        let finished = self.batcher.finish();
        self.closed(finished);
    }

    fn closed(&mut self, batch: Result<Option<Batch>, BatchError>) {
        match batch {
            Ok(Some(batch)) => self.queue_batch(batch),
            Ok(None) => {}
            Err(error) => self.queue.push_back(Input::Failed(refused(error))),
        }
    }

    /// Queue a closed batch with the records it answers, the oldest held.
    fn queue_batch(&mut self, batch: Batch) {
        let count = batch.questions.len().min(self.held.len());
        let records: Vec<Held> = self.held.drain(..count).collect();
        let first = self.taken + 1;
        self.taken += count;
        self.queue.push_back(Input::Item(Item {
            batch,
            first,
            records,
        }));
    }
}

/// Ask one batch and build each record's row from its question's answer.
/// The first record with no usable answer stops the run after the rows before it.
fn answered(
    judging: &Judging<'_>,
    reading: &Reading,
    question: &Question,
    item: &Item,
) -> Result<Completed<Vec<Judged>, Failure>, Failure> {
    let Item {
        batch,
        first,
        records,
    } = item;
    let count = records.len();
    let last = first + count.saturating_sub(1);
    let whole = judging
        .engine
        .ask_batch(batch, judging.environment.cancel())
        .map_err(|error| failed(error.into(), *first, last))?;
    let reply = &whole.reply;
    let mut rows = Vec::with_capacity(count);
    let mut stop = None;
    for (position, (held, &asked)) in records.iter().zip(&batch.questions).enumerate() {
        let Some(AnswerOutcome::Answered(answer)) = reply.outcomes().get(asked) else {
            stop = Some(Failure::PartialReply {
                first: *first,
                last,
            });
            break;
        };
        let (value, outcome) = answer.read(judging.threshold);
        let own = Answered {
            reply: Reply::new(
                reply.model().clone(),
                vec![AnswerOutcome::Answered(answer.clone())],
                reply.usage().map(|usage| usage.share(count, position)),
            ),
            replayed: whole.replayed,
            request: whole.request.clone(),
            requests_sent: share(whole.requests_sent, count, position),
        };
        let judgment = Judgment {
            answer: answer.clone(),
            value,
            outcome,
            answered: own,
        };
        rows.push(judging.row_of(
            reading,
            held.record.clone(),
            question.clone(),
            &judgment,
            held.arrived.as_deref(),
        )?);
    }
    Ok(Completed {
        records: rows.len(),
        value: rows,
        replayed: whole.replayed,
        partial_failure: false,
        stop,
    })
}

/// A request, a whole reply, or a replay that failed a batch of two or more
/// records names their range on one line. Every other cause stays as it is.
fn failed(cause: Failure, first: usize, last: usize) -> Failure {
    match cause {
        Failure::Transport(_)
        | Failure::Status(_)
        | Failure::TokenLimit
        | Failure::Reply(_)
        | Failure::ReplayMiss(_)
            if last > first =>
        {
            Failure::BatchFailed {
                last,
                cause: Box::new(cause),
            }
        }
        other => other,
    }
}

/// The failure a planner refusal stops the run with. The command passes no
/// context, so only a profile limit or a defect can arise.
fn refused(error: BatchError) -> Failure {
    match error {
        BatchError::Profile(limit) => Failure::ProfileLimit(limit),
        BatchError::Defect(what) => Failure::Defect(what),
        BatchError::StructuredQuestionWithContext | BatchError::ContextOverLimit { .. } => {
            Failure::Defect("a context reached the command's batches")
        }
    }
}
