//! Existing ordered recognition scheduling; input and output stay on the caller.

use std::collections::BTreeMap;

use crate::engine::fork_safe::{RecvTimeoutError, Sender, channel};

use crate::engine::Cancel;
use crate::engine::facade::scoped_workers;

/// One ready caller input.
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
    ReaderReady,
    Answered(usize, Result<Row<R>, E>),
}

struct Run<R, E> {
    pending: BTreeMap<usize, Result<Row<R>, E>>,
    next: usize,
    finished: usize,
    dispatched: usize,
    in_flight: usize,
    replayed: usize,
    exhausted: bool,
    halted: bool,
    printing: bool,
    stop: Option<E>,
}

impl<R, E> Run<R, E> {
    fn accept<T>(&mut self, event: Event<T, R, E>, work: &Sender<(usize, T)>, ended: fn() -> E) {
        match event {
            Event::Input(input) => {
                if self.halted {
                    return;
                }
                match input {
                    Input::End => self.exhausted = true,
                    Input::Failed(error) => self.refuse(error),
                    Input::Item(value) => self.dispatch(value, work, ended),
                }
            }
            Event::ReaderReady => {}
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
/// each row to `emit` in input order. The caller checks readiness before
/// reading, so withheld input never holds cancellation open. Workers join
/// before returning.
#[allow(
    clippy::too_many_arguments,
    reason = "the runner keeps cancellation and each typed bridge callback explicit"
)]
pub(crate) fn run<T, R, E>(
    jobs: usize,
    cancel: &Cancel,
    readiness: Option<&crate::public::cli_reader::CliReader<'_>>,
    mut read: impl FnMut() -> Option<Input<T, E>>,
    answer: &(impl Fn(T) -> Result<Row<R>, E> + Sync),
    mut emit: impl FnMut(R) -> Result<bool, E>,
    stopped: &(impl Fn(crate::engine::error::Error) -> E + Sync),
    ended: fn() -> E,
) -> Result<Outcome<E>, E>
where
    T: Send + 'static,
    R: Send + 'static,
    E: Send + 'static,
{
    let (events, received) = channel();
    let wake = readiness.map(crate::public::cli_reader::CliReader::wake);
    if let Some(wake) = &wake {
        let events = events.clone();
        wake.register(Box::new(move || {
            let _sent = events.send(Event::ReaderReady);
        }));
    }
    struct Clear(Option<crate::public::cli_reader::Wake>);
    impl Drop for Clear {
        fn drop(&mut self) {
            if let Some(wake) = &self.0 {
                wake.clear();
            }
        }
    }
    let _clear = Clear(wake);
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
                if !run.halted
                    && !run.exhausted
                    && run.dispatched - run.next < jobs
                    && readiness.is_none_or(|reader| reader.ready())
                    && let Some(input) = read()
                {
                    run.accept(Event::Input(input), &work, ended);
                    continue;
                }
                cancel.observed_block();
                match received.recv_timeout(Cancel::poll()) {
                    Ok(event) => run.accept(event, &work, ended),
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => return Err(ended()),
                }
            }
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

#[cfg(all(test, feature = "cli"))]
mod tests;
