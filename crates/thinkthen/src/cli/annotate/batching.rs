//! Request-aligned annotation groups over one bounded record stream.

use std::collections::BTreeMap;
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::annotate_schedule::Input as AnnotateInput;
use crate::core::{Reading, Record, Setting};
use crate::engine::facade::{Completed, GroupAnswer, GroupBatchFailure, GroupWork, RunOutcome};
use crate::failure::Failure;
use crate::schedule::{Judged, Output, Placed};

use super::Judging;

mod former;
use former::Former;

struct Fragment {
    row: usize,
    slot: usize,
    first: usize,
    last: usize,
    answer: GroupAnswer,
}

struct Row {
    record: Record,
    groups: Vec<usize>,
    slots: Vec<Option<(GroupAnswer, usize, usize)>>,
}

struct Shared {
    rows: Mutex<BTreeMap<usize, Row>>,
    completed: AtomicUsize,
}

impl Shared {
    fn new() -> Self {
        Self {
            rows: Mutex::new(BTreeMap::new()),
            completed: AtomicUsize::new(0),
        }
    }

    fn insert(&self, row: usize, record: Record, groups: Vec<usize>) -> Result<(), Failure> {
        let slots = std::iter::repeat_with(|| None).take(groups.len()).collect();
        self.rows
            .lock()
            .map_err(|_| Failure::Defect("annotation rows were poisoned"))?
            .insert(
                row,
                Row {
                    record,
                    groups,
                    slots,
                },
            );
        Ok(())
    }

    fn take_ready(&self, fragments: Vec<Fragment>) -> Result<Vec<(usize, Row)>, Placed> {
        let mut rows = self
            .rows
            .lock()
            .map_err(|_| Placed::from(Failure::Defect("annotation rows were poisoned")))?;
        for fragment in fragments {
            let row = rows.get_mut(&fragment.row).ok_or_else(|| {
                Placed::at(
                    Failure::Defect("an annotation fragment names no row"),
                    fragment.row + 1,
                )
            })?;
            let slot = row.slots.get_mut(fragment.slot).ok_or_else(|| {
                Placed::at(
                    Failure::Defect("an annotation fragment names no slot"),
                    fragment.row + 1,
                )
            })?;
            *slot = Some((fragment.answer, fragment.first, fragment.last));
        }
        let mut ready = Vec::new();
        let mut next = self.completed.load(Ordering::Acquire);
        while rows
            .get(&next)
            .is_some_and(|row| row.slots.iter().all(Option::is_some))
        {
            let row = rows.remove(&next).ok_or_else(|| {
                Placed::at(
                    Failure::Defect("a complete annotation row disappeared"),
                    next + 1,
                )
            })?;
            ready.push((next, row));
            next += 1;
        }
        Ok(ready)
    }
}

fn answered(
    judging: &Judging<'_>,
    work: &GroupWork,
) -> Result<Completed<Vec<Fragment>, Placed>, Placed> {
    let first = work.rows.first().map_or(1, |row| row + 1);
    let last = work.rows.last().map_or(first, |row| row + 1);
    let context = || crate::failure::ReplayContext::AnnotateGroup {
        ordinal: work.group + 1,
        members: work.places.len(),
    };
    let failure = |error| match error {
        GroupBatchFailure::Engine(error) => Failure::from(error).with_replay_context(context()),
        GroupBatchFailure::Batch(error) => {
            crate::failure::context::Limits::new(judging.engine().profile()).refused(error, false)
        }
        GroupBatchFailure::AllFailed => Failure::PartialReply { first, last },
        GroupBatchFailure::Defect(message) => Failure::Defect(message),
    };
    let whole = judging
        .engine()
        .answer_group_batch_raw(work, judging.cancel())
        .map_err(|error| {
            let cause = failure(error);
            let cause = if matches!(cause, Failure::PartialReply { .. }) {
                cause
            } else {
                Failure::BatchFailed {
                    last,
                    cause: Box::new(cause),
                }
            };
            Placed::at(cause, first)
        })?;
    if whole.value.len() > work.rows.len() || whole.value.len() > work.slots.len() {
        return Err(Placed::at(
            Failure::Defect("an annotation request lost a row"),
            first,
        ));
    }
    let completed_rows = whole.value.len();
    let stop_at = work.rows.get(completed_rows).map_or(last, |row| row + 1);
    let value = work
        .rows
        .iter()
        .zip(&work.slots)
        .zip(whole.value)
        .map(|((&row, &slot), answer)| Fragment {
            row,
            slot,
            first,
            last,
            answer,
        })
        .collect();
    Ok(Completed {
        value,
        records: 0,
        replayed: 0,
        partial_failure: false,
        stop: whole.stop.map(|error| {
            let cause = failure(error);
            let cause = if matches!(cause, Failure::PartialReply { .. }) {
                cause
            } else {
                Failure::BatchFailed {
                    last,
                    cause: Box::new(cause),
                }
            };
            Placed::at(cause, stop_at)
        }),
    })
}

fn judge_row(judging: &Judging<'_>, row: Row, at: usize) -> Result<Judged, Placed> {
    let mut groups = BTreeMap::<usize, (bool, usize, usize)>::new();
    let mut answers = Vec::with_capacity(row.slots.len());
    for (group, slot) in row.groups.into_iter().zip(row.slots) {
        let (answer, first, last) =
            slot.ok_or_else(|| Placed::at(Failure::Defect("an annotation slot disappeared"), at))?;
        let entry = groups.entry(group).or_insert((false, first, last));
        entry.0 |= answer.usable();
        answers.push(answer);
    }
    if let Some((_, (_, first, last))) = groups.iter().find(|(_, (usable, _, _))| !usable) {
        return Err(Placed::at(
            Failure::PartialReply {
                first: *first,
                last: *last,
            },
            at,
        ));
    }
    judging
        .finish(row.record, answers)
        .map_err(|error| Placed::at(error, at))
}

fn feed<I>(chunks: I, sender: std::sync::mpsc::SyncSender<Result<AnnotateInput, Placed>>)
where
    I: Iterator<Item = Result<AnnotateInput, Placed>>,
{
    for chunk in chunks {
        let failed = chunk.is_err();
        if sender.send(chunk).is_err() || failed {
            return;
        }
    }
}

pub(super) fn run<I>(
    judging: &Judging<'_>,
    reading: &Reading,
    chunks: I,
    setting: Setting,
    cancel: &crate::engine::Cancel,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure>
where
    I: Iterator<Item = Result<AnnotateInput, Placed>> + Send + 'static,
{
    let shared = Arc::new(Shared::new());
    let former = Former::new(judging, reading, setting, Arc::clone(&shared))?;
    let recording = judging.engine().recording();
    let mut finished = 0;
    let mut replayed = 0;
    let mut partial_failure = false;
    let outcome = judging.engine().records(
        output.flow(),
        cancel,
        |asks, events| {
            let (sender, raw) = sync_channel(64);
            thread::spawn(move || feed(chunks, sender));
            thread::spawn(move || former.answer(&raw, &asks, &events));
        },
        &|work: &GroupWork| answered(judging, work),
        |fragments| {
            for (row, held) in shared.take_ready(fragments)? {
                let judged = judge_row(judging, held, row + 1)?;
                replayed += usize::from(judged.replayed);
                partial_failure |= judged.partial_failure;
                let more = output.take(judged).map_err(Placed::from)?;
                finished += 1;
                cancel.finished_records(1);
                shared.completed.store(finished, Ordering::Release);
                if !more {
                    return Ok(false);
                }
            }
            Ok(true)
        },
    );
    let stopped = match outcome {
        Ok(RunOutcome::Complete) => {
            return Ok(if partial_failure {
                ExitCode::from(6)
            } else {
                ExitCode::SUCCESS
            });
        }
        Ok(RunOutcome::Stopped { cause, .. }) | Err(cause) => cause,
    };
    Err(Failure::Stopped {
        at: stopped.at.unwrap_or(finished + 1),
        finished,
        replayed,
        recording,
        held: false,
        cause: Box::new(stopped.cause),
    })
}
