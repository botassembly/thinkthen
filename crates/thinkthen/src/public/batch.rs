//! The lazy batch behind `filter`, `decide_many`, and `annotate`.
//!
//! The engine's ordered scheduler runs on one background thread. The calling
//! thread pulls the caller's records only when the scheduler asks, so input
//! is read at most one throttle ahead of the rows returned. The caller's
//! interrupt check runs here, on the calling thread, at every tick.

use std::fmt;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::engine::facade::{self, Completed, Input, InputPort, RunOutcome};
use crate::public::error::Error;
use crate::public::options::{Stop, guarded};
use crate::public::results::Facts;

mod annotation;
mod planned;

pub(crate) use annotation::start_annotation;
pub(crate) use planned::start_details;
pub(crate) use planned::start_planned;

/// How often a waiting batch runs the caller's controls, as the engine's poll.
const TICK: Duration = Duration::from_millis(50);

/// The records a batch answers, pulled lazily, with each answer in input order.
///
/// A batch stops at its first failed record: every earlier row comes first,
/// then the error, then nothing. Dropping a batch stops its work and joins
/// every worker. A batch is neither `Send` nor `Sync`, so it stays on the
/// thread whose interrupt check it runs.
pub struct Batch<'a, T> {
    source: Box<dyn Source<T> + 'a>,
}

impl<T> fmt::Debug for Batch<'_, T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Batch").finish_non_exhaustive()
    }
}

impl<T> Iterator for Batch<'_, T> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.pull()
    }
}

impl<'a, T: 'a> Batch<'a, T> {
    /// Final facts after exhaustion or the one terminal error.
    #[must_use]
    pub fn facts(&self) -> Option<&Facts> {
        self.source.facts()
    }

    /// A batch that yields one error, then nothing.
    pub(crate) fn failed(error: Error) -> Self {
        Self {
            source: Box::new(Some(error)),
        }
    }

    pub(crate) fn of(result: Result<Self, Error>) -> Self {
        result.unwrap_or_else(Self::failed)
    }
}

trait Source<T> {
    fn pull(&mut self) -> Option<Result<T, Error>>;
    fn facts(&self) -> Option<&Facts>;
}

impl<T> Source<T> for Option<Error> {
    fn pull(&mut self) -> Option<Result<T, Error>> {
        self.take().map(Err)
    }

    fn facts(&self) -> Option<&Facts> {
        None
    }
}

type ScheduledAnswer<'a, W, V> =
    dyn Fn(&W) -> Result<Completed<V, Error>, Error> + Send + Sync + 'a;

pub(super) enum Event<W, V> {
    Port(InputPort<W, V, Error>),
    Ask,
    Row(V),
    End(Result<(), Error>),
}

/// The scheduler thread: run the engine's ordered scheduler, relay its asks
/// and rows, and report how it ended once its relay has joined.
pub(super) fn schedule<W: Send + 'static, V: Send + 'static>(
    engine: &facade::Engine,
    cancel: &crate::engine::Cancel<'static>,
    answer: &ScheduledAnswer<'_, W, V>,
    sender: &Sender<Event<W, V>>,
) {
    let mut relay = None;
    let ended = guarded(|| {
        let outcome = engine.records(
            crate::engine::schedule::RecordFlow::Streaming,
            cancel,
            |asks: Receiver<()>, port| {
                let _sent = sender.send(Event::Port(port));
                relay = Some(relay_asks(asks, sender.clone()));
            },
            &|work: &W| answer(work),
            |row| Ok(sender.send(Event::Row(row)).is_ok()),
        )?;
        match outcome {
            RunOutcome::Complete => Ok(()),
            RunOutcome::Stopped { cause, .. } => Err(cause),
        }
    });
    let joined = relay.map_or(Ok(()), JoinHandle::join);
    let ended = ended.and_then(|()| joined.map_err(|_| Error::defect("the record relay panicked")));
    let _sent = sender.send(Event::End(ended));
}

/// Turn each scheduler ask into one event for the calling thread.
fn relay_asks<W: Send + 'static, V: Send + 'static>(
    asks: Receiver<()>,
    sender: Sender<Event<W, V>>,
) -> JoinHandle<()> {
    thread::spawn(move || while asks.recv().is_ok() && sender.send(Event::Ask).is_ok() {})
}

/// Stop one typed scheduler, answer its final asks, and join every worker.
pub(super) fn join_scheduler<W, V>(
    scheduler: &mut Option<JoinHandle<()>>,
    stop: &Stop<'_>,
    events: &Receiver<Event<W, V>>,
    port: &mut Option<InputPort<W, V, Error>>,
) -> Result<(), Error> {
    let Some(scheduler) = scheduler.take() else {
        return Ok(());
    };
    stop.fire();
    while !scheduler.is_finished() {
        match events.recv_timeout(TICK) {
            Ok(Event::Port(next)) => *port = Some(next),
            Ok(Event::Ask) => {
                if let Some(port) = port {
                    let _sent = port.send(Input::End);
                }
            }
            Ok(Event::Row(_) | Event::End(_)) | Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
    scheduler
        .join()
        .map_err(|_| Error::defect("the record scheduler panicked"))
}
