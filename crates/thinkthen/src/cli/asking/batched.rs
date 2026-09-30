//! `decide`, `filter` and `rank` over a stream, by ADR 0048 items 1 to 6. A
//! record thread parses records ahead, a batch reader plans a batch for each
//! ask, and a worker sends it as one request and builds a row for each record.

use std::collections::VecDeque;
use std::io::BufRead;
use std::process::ExitCode;
use std::sync::mpsc::{Receiver, RecvTimeoutError, SyncSender, sync_channel};
use std::thread;
use std::time::Duration;

use super::batch_meta::Description;
use super::{Asks, Judging, JudgingInput, RowContext, table_kind};
use crate::core::batch::halves_with_questions;
use crate::core::{
    AnswerOutcome, Backend, BackendProfile, Batch, BatchError, Batcher, Evidence, Reading, Record,
    Reply, Setting, share,
};
use crate::edge;
use crate::engine::facade::{Answered, Completed, Input, InputPort, Judgment};
use crate::failure::Failure;
use crate::failure::context::Limits;
use crate::judge::Keeping;
use crate::schedule::{self, Judged, Output, Placed};
use crate::table::Rows as TableRows;

mod choose;
mod planned;

/// How long input may pause before the open batch goes out, by ADR 0048 item 2.
const PAUSE: Duration = Duration::from_millis(50);

/// How many raw records the record thread reads ahead of the batch reader.
const AHEAD: usize = 64;

/// One parsed record, and the bytes it arrived as when it arrived as a line.
struct Held {
    record: Record,
    arrived: Option<Vec<u8>>,
    at: usize,
}

/// One batch for a worker: its request, its first record's number, its records.
pub(super) struct Item {
    batch: Batch,
    first: usize,
    records: Vec<Held>,
}

type Records = Box<dyn Iterator<Item = Result<Held, Placed>> + Send>;

/// Send the stream in batches of `setting`, or print the first batch's plan.
pub(super) fn run(
    configuration: JudgingInput<'_>,
    reading: &Reading,
    source: Box<dyn BufRead + Send>,
    setting: Setting,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let records: Records = if let Some(kind) = table_kind(configuration.common) {
        let rows = TableRows::new(source, kind)?;
        Box::new(rows.enumerate().map(|(place, row)| {
            row.map(|record| Held {
                record,
                arrived: None,
                at: place + 1,
            })
            .map_err(|error| Placed::at(error, place + 1))
        }))
    } else {
        let reading = reading.clone();
        Box::new(
            edge::numbered(edge::Chunks::new(source, true), &reading).map(move |(at, bytes)| {
                let bytes = bytes.map_err(|error| Placed::at(error, at))?;
                let record = reading
                    .record(&bytes)
                    .map_err(|error| Placed::at(Failure::record(error, true), at))?;
                Ok(Held {
                    record,
                    arrived: Some(bytes),
                    at,
                })
            }),
        )
    };
    let limits = Limits::new(configuration.profile.as_ref());
    let backend = configuration.backend.clone();
    let profile = configuration.profile.clone();
    let context = configuration.context.as_ref().map(super::Context::evidence);
    let batcher = match &configuration.asks {
        Asks::Fixed(question) => Some(
            Batcher::new(
                backend.clone(),
                profile.clone(),
                question.clone(),
                setting,
                context.clone(),
            )
            .map_err(|error| limits.refused(error, true))?,
        ),
        Asks::FromRecord { .. } => None,
    };
    let downstream = edge::Downstream::default();
    let former = Former {
        batcher,
        asks: configuration.asks.clone(),
        backend,
        profile,
        context,
        setting,
        reading: reading.clone(),
        held: Vec::new(),
        downstream: downstream.clone(),
        cancel: configuration.environment.cancel().clone(),
        queue: VecDeque::new(),
        limits,
    };
    if configuration.common.dry_run {
        return planned::run(former, records, &configuration, output);
    }
    if matches!(configuration.keeping, Keeping::Passing | Keeping::Ordered) {
        output.guard_models();
    }
    let judging = Judging::new(configuration)?;
    let recording = judging.engine.recording();
    let outcome = judging.engine.records(
        output.flow(),
        judging.environment.cancel(),
        |asks, events| {
            let (sender, raw) = sync_channel(AHEAD);
            thread::spawn(move || feed(records, &sender));
            thread::spawn(move || former.answer(&raw, &asks, &events));
        },
        &|item| answered(&judging, reading, setting, item),
        |rows| {
            for row in rows {
                if !output.take(row).map_err(Placed::from)? {
                    return Ok(false);
                }
            }
            Ok(true)
        },
    )?;
    if downstream.latched() {
        return Ok(ExitCode::SUCCESS);
    }
    schedule::ended(outcome, recording, output)
}

/// Read records ahead into the channel, and stop after the first refusal.
fn feed(records: Records, sender: &SyncSender<Result<Held, Placed>>) {
    for record in records {
        let failed = record.is_err();
        if sender.send(record).is_err() || failed {
            return;
        }
    }
}

/// The batch reader: the planner, the open batch's records, and the closed
/// batches and refusals that wait for an ask, in input order.
struct Former {
    batcher: Option<Batcher>,
    asks: Asks,
    backend: Backend,
    profile: Option<BackendProfile>,
    context: Option<Evidence>,
    setting: Setting,
    reading: Reading,
    held: Vec<Held>,
    downstream: edge::Downstream,
    cancel: crate::engine::Cancel<'static>,
    queue: VecDeque<Input<Item, Placed>>,
    limits: Limits,
}

impl Former {
    /// Answer each ask with the next batch.
    fn answer(
        mut self,
        raw: &Receiver<Result<Held, Placed>>,
        asks: &Receiver<()>,
        events: &InputPort<Item, Vec<Judged>, Placed>,
    ) {
        while asks.recv().is_ok() {
            if events.send(self.next(raw)).is_err() {
                return;
            }
        }
    }

    /// The next closed batch or refusal. It pulls records only while an ask
    /// waits, and sends the open batch when input pauses.
    fn next(&mut self, raw: &Receiver<Result<Held, Placed>>) -> Input<Item, Placed> {
        loop {
            if self.downstream.gone() || self.cancel.stop().is_some() {
                return Input::End;
            }
            if let Some(event) = self.queue.pop_front() {
                return event;
            }
            let next = raw.recv_timeout(PAUSE);
            match next {
                Ok(record) => self.push(record),
                Err(RecvTimeoutError::Timeout) if self.held.is_empty() => {}
                Err(RecvTimeoutError::Timeout) => self.pause(),
                Err(RecvTimeoutError::Disconnected) => {
                    self.end();
                    return self.queue.pop_front().unwrap_or(Input::End);
                }
            }
        }
    }

    fn pause(&mut self) {
        if let Some(paused) = self.batcher.as_mut().map(Batcher::pause) {
            self.closed(paused);
        }
    }

    fn end(&mut self) {
        if let Some(finished) = self.batcher.as_mut().map(Batcher::finish) {
            self.closed(finished);
        }
    }

    fn closed(&mut self, batch: Result<Option<Batch>, BatchError>) {
        match batch {
            Ok(Some(batch)) => self.queue_batch(batch),
            Ok(None) => {}
            Err(error) => self.queue.push_back(Input::Failed(Placed::from(
                self.limits.refused(error, false),
            ))),
        }
    }

    /// Queue a closed batch with the records it answers, the oldest held.
    fn queue_batch(&mut self, batch: Batch) {
        let count = batch.questions.len().min(self.held.len());
        let records: Vec<Held> = self.held.drain(..count).collect();
        let first = records.first().map_or(1, |record| record.at);
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
    setting: Setting,
    item: &Item,
) -> Result<Completed<Vec<Judged>, Placed>, Placed> {
    let Item {
        batch,
        first,
        records,
    } = item;
    let count = records.len();
    if batch.row_questions.len() != count {
        return Err(Placed::at(
            Failure::Defect("a batch lost a row question"),
            *first,
        ));
    }
    let last = records.last().map_or(*first, |held| held.at);
    let observed = super::Observed::new(judging.environment.cancel(), judging.view.details);
    let cancel = &observed.cancel;
    let whole = match judging.engine.ask_batch(batch, cancel) {
        Ok(whole) => {
            let attempts = observed.events();
            return answer_rows(
                judging,
                reading,
                batch,
                records,
                whole,
                Description::new(setting, batch.closed, false, judging.context.is_some()),
                &attempts,
            );
        }
        Err(error)
            if count > 1
                && (error.too_large()
                    || matches!(error, crate::engine::error::Error::ReplayMiss(_))) =>
        {
            error
        }
        Err(error) => return Err(Placed::at(failed(error.into(), *first, last), *first)),
    };
    split_answered(judging, reading, setting, item, whole, observed.events())
}

/// Rebuild the two halves within the refused batch's scheduled place.
fn split_answered(
    judging: &Judging<'_>,
    reading: &Reading,
    setting: Setting,
    item: &Item,
    whole: crate::engine::error::Error,
    parent_attempts: Vec<crate::public::AttemptObservation>,
) -> Result<Completed<Vec<Judged>, Placed>, Placed> {
    let Item {
        batch,
        first,
        records,
    } = item;
    let description = Description::new(setting, batch.closed, true, judging.context.is_some());
    let count = records.len();
    let last = records.last().map_or(*first, |held| held.at);
    let cancel = judging.environment.cancel();
    let refused_attempts = u64::from(whole.too_large());
    let values = records
        .iter()
        .zip(&batch.row_questions)
        .map(|(held, question)| {
            let record = reading
                .batch_record(&held.record)
                .map_err(|error| Placed::at(error.into(), held.at))?;
            Ok::<_, Placed>((record, question.clone()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let context = judging.context.as_ref().map(super::Context::evidence);
    let [left, right] = halves_with_questions(
        judging.engine.backend(),
        judging.engine.profile(),
        context.as_ref(),
        values,
    )
    .map_err(|error| {
        Placed::at(
            Limits::new(judging.engine.profile()).refused(error, false),
            *first,
        )
    })?;
    let middle = count.div_ceil(2);
    let (left_records, right_records) = records.split_at(middle);
    let left_last = left_records.last().map_or(*first, |held| held.at);
    if let Some(stop) = cancel.stop() {
        return Err(Placed::at(stop.into(), *first));
    }
    let first_observed = super::Observed::new(cancel, judging.view.details);
    let mut first_answer = judging
        .engine
        .ask_batch(&left, &first_observed.cancel)
        .map_err(|error| {
            let range_last = if matches!(error, crate::engine::error::Error::ReplayMiss(_)) {
                last
            } else {
                left_last
            };
            Placed::at(failed(error.into(), *first, range_last), *first)
        })?;
    first_answer.requests_sent += refused_attempts;
    let mut first_attempts = parent_attempts.clone();
    first_attempts.extend(first_observed.events());
    first_attempts.sort_by_key(crate::public::AttemptObservation::ordinal);
    first_attempts.dedup_by_key(|event| event.ordinal());
    let mut first_done = answer_rows(
        judging,
        reading,
        &left,
        left_records,
        first_answer,
        description,
        &first_attempts,
    )?;
    if first_done.stop.is_some() {
        return Ok(first_done);
    }
    let right_first = right_records.first().map_or(last, |held| held.at);
    if let Some(stop) = cancel.stop() {
        first_done.stop = Some(Placed::at(stop.into(), right_first));
        return Ok(first_done);
    }
    let second_observed = super::Observed::new(cancel, judging.view.details);
    let second_answer = match judging.engine.ask_batch(&right, &second_observed.cancel) {
        Ok(answer) => answer,
        Err(error) => {
            first_done.stop = Some(Placed::at(
                failed(error.into(), right_first, last),
                right_first,
            ));
            return Ok(first_done);
        }
    };
    let mut second_attempts = parent_attempts;
    second_attempts.extend(second_observed.events());
    second_attempts.sort_by_key(crate::public::AttemptObservation::ordinal);
    second_attempts.dedup_by_key(|event| event.ordinal());
    let second_done = answer_rows(
        judging,
        reading,
        &right,
        right_records,
        second_answer,
        description,
        &second_attempts,
    )?;
    first_done.records += second_done.records;
    first_done.replayed += second_done.replayed;
    first_done.value.extend(second_done.value);
    first_done.stop = second_done.stop;
    Ok(first_done)
}

/// Turn one answered request into rows, sharing its attempts over its members.
fn answer_rows(
    judging: &Judging<'_>,
    reading: &Reading,
    batch: &Batch,
    records: &[Held],
    whole: Answered,
    description: Description,
    attempts: &[crate::public::AttemptObservation],
) -> Result<Completed<Vec<Judged>, Placed>, Placed> {
    let count = records.len();
    let first = records.first().map_or(1, |held| held.at);
    let last = records.last().map_or(first, |held| held.at);
    let reply = &whole.reply;
    let mut rows = Vec::with_capacity(count);
    let mut stop = None;
    if batch.questions.len() != count
        || batch.outcomes.len() != count
        || batch.row_questions.len() != count
    {
        return Err(Placed::at(
            Failure::Defect("a batch lost a row question"),
            first,
        ));
    }
    for (position, (held, &asked)) in records.iter().zip(&batch.outcomes).enumerate() {
        let Some(AnswerOutcome::Answered(answer)) = reply.outcomes().get(asked) else {
            stop = Some(Placed::at(Failure::PartialReply { first, last }, held.at));
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
        let batch_meta = description.row(count, position + 1, reply.usage(), whole.requests_sent);
        let question = batch
            .row_questions
            .get(position)
            .ok_or_else(|| Placed::at(Failure::Defect("a batch lost a row question"), held.at))?;
        rows.push(
            judging
                .row_of(
                    reading,
                    held.record.clone(),
                    question.clone(),
                    &judgment,
                    RowContext {
                        arrived: held.arrived.as_deref(),
                        batch: batch_meta,
                        attempts: attempts.to_vec(),
                    },
                )
                .map_err(|error| Placed::at(error, held.at))?,
        );
    }
    Ok(Completed {
        records: rows.len(),
        replayed: if whole.replayed { rows.len() } else { 0 },
        value: rows,
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
        | Failure::ReplayMiss { .. }
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
