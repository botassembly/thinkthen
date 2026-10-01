//! The ordered runner under the command's many-line `recognize`.
//!
//! Each `recognize` line asks three dependent steps, so one line cannot be
//! one input of the question pipeline, whose inputs each give all their
//! questions at once. This runner asks its reader for one line at a time,
//! answers up to the width of lines at once, and hands rows on in input
//! order. Each line's steps run on the question pipeline. Ticket 0304
//! slice 5 moved it here from the engine's old record scheduler.

use std::collections::BTreeMap;
use std::sync::mpsc::{self, Receiver};

use crate::engine::fork_safe::{RecvTimeoutError, Sender, channel};

use crate::engine::Cancel;
use crate::engine::facade::scoped_workers;

/// One event from the reader thread.
pub(crate) enum Input<T, E> {
    Item(T),
    Failed(E),
    End,
}

/// A run that completed, or the failure that stopped it after `finished`
/// rows, `replayed` of them from a recording.
#[derive(Debug)]
pub(crate) enum Outcome<E> {
    Complete,
    Stopped {
        finished: usize,
        replayed: usize,
        cause: E,
    },
}

/// One answered line: its row and whether a recording answered it.
pub(crate) struct Row<R> {
    pub(crate) value: R,
    pub(crate) replayed: bool,
}

enum Event<T, R, E> {
    Input(Input<T, E>),
    Answered(usize, Result<Row<R>, E>),
}

/// The reader's side of the bridge: one input, failure or end per ask.
pub(crate) struct Port<T, R, E>(Sender<Event<T, R, E>>);

impl<T, R, E> Port<T, R, E> {
    pub(crate) fn send(&self, input: Input<T, E>) -> Result<(), ()> {
        self.0.send(Event::Input(input)).map_err(|_| ())
    }
}

struct Run<R, E> {
    pending: BTreeMap<usize, Result<Row<R>, E>>,
    next: usize,
    finished: usize,
    dispatched: usize,
    in_flight: usize,
    replayed: usize,
    reading: bool,
    exhausted: bool,
    halted: bool,
    printing: bool,
    stop: Option<E>,
}

impl<R, E> Run<R, E> {
    /// Ask for one more line while fewer than `jobs` wait to be handed on.
    fn request(&mut self, ask: &mpsc::Sender<()>, jobs: usize, ended: fn() -> E) {
        if !self.reading && !self.halted && !self.exhausted && self.dispatched - self.next < jobs {
            self.reading = ask.send(()).is_ok();
            if !self.reading {
                self.refuse(ended());
            }
        }
    }

    fn accept<T>(&mut self, event: Event<T, R, E>, work: &Sender<(usize, T)>, ended: fn() -> E) {
        match event {
            Event::Input(input) => {
                self.reading = false;
                if self.halted {
                    return;
                }
                match input {
                    Input::End => self.exhausted = true,
                    Input::Failed(error) => self.refuse(error),
                    Input::Item(value) => self.dispatch(value, work, ended),
                }
            }
            Event::Answered(place, result) => {
                self.in_flight -= 1;
                self.halted |= result.is_err();
                self.pending.insert(place, result);
            }
        }
    }

    fn dispatch<T>(&mut self, value: T, work: &Sender<(usize, T)>, ended: fn() -> E) {
        if work.send((self.dispatched, value)).is_err() {
            self.refuse(ended());
            return;
        }
        self.dispatched += 1;
        self.in_flight += 1;
    }

    fn refuse(&mut self, error: E) {
        self.pending.insert(self.dispatched, Err(error));
        self.dispatched += 1;
        self.halted = true;
    }

    /// Hand on every row that is next in order. A failure, or a host that
    /// takes no more rows, halts the run.
    fn drain(
        &mut self,
        emit: &mut impl FnMut(R) -> Result<bool, E>,
        cancel: &Cancel,
    ) -> Result<(), E> {
        while self.printing {
            let Some(result) = self.pending.remove(&self.next) else {
                return Ok(());
            };
            let more = match result {
                Ok(row) => {
                    self.replayed += usize::from(row.replayed);
                    let more = emit(row.value)?;
                    self.finished += 1;
                    cancel.finished_records(1);
                    self.next += 1;
                    more
                }
                Err(cause) => {
                    self.stop = Some(cause);
                    false
                }
            };
            if !more {
                self.halted = true;
                self.printing = false;
            }
        }
        Ok(())
    }
}

/// Answer the lines the reader hands over on up to `jobs` workers, and hand
/// each row to `emit` in input order. The reader starts on its own thread,
/// so a reader blocked on its input never holds the call open. Every worker
/// has joined on return.
#[allow(
    clippy::too_many_arguments,
    reason = "the runner keeps cancellation and each typed bridge callback explicit"
)]
pub(crate) fn run<T, R, E>(
    jobs: usize,
    cancel: &Cancel,
    start_reader: impl FnOnce(Receiver<()>, Port<T, R, E>),
    answer: &(impl Fn(T) -> Result<Row<R>, E> + Sync),
    mut emit: impl FnMut(R) -> Result<bool, E>,
    stopped: &(impl Fn(crate::engine::error::Error) -> E + Sync),
    ended: fn() -> E,
) -> Result<Outcome<E>, E>
where
    T: Send,
    R: Send,
    E: Send,
{
    let (events, received) = channel();
    let (ask, asked) = mpsc::channel();
    start_reader(asked, Port(events.clone()));
    scoped_workers(
        jobs,
        events,
        &|(place, value)| {
            Event::Answered(
                place,
                match cancel.stop() {
                    Some(stop) => Err(stopped(stop)),
                    None => answer(value),
                },
            )
        },
        |work| {
            let mut run = Run {
                pending: BTreeMap::new(),
                next: 0,
                finished: 0,
                dispatched: 0,
                in_flight: 0,
                replayed: 0,
                reading: false,
                exhausted: false,
                halted: false,
                printing: true,
                stop: None,
            };
            loop {
                run.drain(&mut emit, cancel)?;
                if let Some(stop) = cancel.stop().filter(|_| !run.halted) {
                    run.refuse(stopped(stop));
                    continue;
                }
                if run.in_flight == 0 && (run.halted || run.exhausted) {
                    break;
                }
                run.request(&ask, jobs, ended);
                cancel.observed_block();
                match received.recv_timeout(Cancel::poll()) {
                    Ok(event) => run.accept(event, &work, ended),
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => return Err(ended()),
                }
            }
            drop(ask);
            Ok(match run.stop {
                Some(cause) => Outcome::Stopped {
                    finished: run.finished,
                    replayed: run.replayed,
                    cause,
                },
                None => Outcome::Complete,
            })
        },
    )
}

#[cfg(test)]
mod tests;
