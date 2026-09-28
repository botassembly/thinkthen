//! The lazy batch behind `filter`, `decide_many`, and `annotate`.
//!
//! The engine's ordered scheduler runs on one background thread. The calling
//! thread pulls the caller's records only when the scheduler asks, so input
//! is read at most one throttle ahead of the rows returned. The caller's
//! interrupt check runs here, on the calling thread, at every tick.

use std::collections::VecDeque;
use std::fmt;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, channel};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::engine::facade::{self, Completed, Input, InputPort, RunOutcome};
use crate::public::error::Error;
use crate::public::options::{Stop, guarded};

mod planned;

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
}

impl<T> Source<T> for Option<Error> {
    fn pull(&mut self) -> Option<Result<T, Error>> {
        self.take().map(Err)
    }
}

/// One record's answer, computed on an engine worker.
pub(crate) type Answer<V> = dyn Fn(&str) -> Result<Completed<V, Error>, Error> + Send + Sync;
type ScheduledAnswer<'a, W, V> =
    dyn Fn(&W) -> Result<Completed<V, Error>, Error> + Send + Sync + 'a;

pub(super) enum Event<W, V> {
    Port(InputPort<W, V, Error>),
    Ask,
    Row(V),
    End(Result<(), Error>),
}

/// The caller's records, the rows the scheduler returns, and the stop.
struct Stream<'a, I: Iterator, V, T> {
    items: I,
    held: VecDeque<I::Item>,
    pair: fn(I::Item, V) -> Option<T>,
    events: Receiver<Event<String, V>>,
    port: Option<InputPort<String, V, Error>>,
    stop: Stop<'a>,
    fed: usize,
    most: Option<usize>,
    scheduler: Option<JoinHandle<()>>,
}

/// Start the scheduler for these records. Nothing is read or sent until the
/// first pull.
pub(crate) fn start<'a, I, V, T>(
    engine: Arc<facade::Engine>,
    records: I,
    stop: Stop<'a>,
    most: Option<usize>,
    answer: Arc<Answer<V>>,
    pair: fn(I::Item, V) -> Option<T>,
) -> Batch<'a, T>
where
    I: Iterator + 'a,
    I::Item: super::Evidence,
    V: Send + 'static,
    T: 'a,
{
    let (sender, events) = channel::<Event<String, V>>();
    let cancel = stop.shared();
    let scheduler = thread::spawn(move || {
        schedule::<String, V>(&engine, &cancel, &|work: &String| answer(work), &sender);
    });
    Batch {
        source: Box::new(Stream {
            items: records,
            held: VecDeque::new(),
            pair,
            events,
            port: None,
            stop,
            fed: 0,
            most,
            scheduler: Some(scheduler),
        }),
    }
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

impl<I, V, T> Source<T> for Stream<'_, I, V, T>
where
    I: Iterator,
    I::Item: super::Evidence,
{
    fn pull(&mut self) -> Option<Result<T, Error>> {
        self.scheduler.as_ref()?;
        loop {
            if self.stop.interrupted() {
                self.stop.fire();
            }
            let event = match self.events.recv_timeout(TICK) {
                Ok(event) => event,
                Err(RecvTimeoutError::Timeout) => continue,
                Err(RecvTimeoutError::Disconnected) => {
                    Event::End(Err(Error::defect("the scheduler ended early")))
                }
            };
            if let Some(pulled) = self.take(event) {
                return pulled;
            }
        }
    }
}

impl<I, V, T> Stream<'_, I, V, T>
where
    I: Iterator,
    I::Item: super::Evidence,
{
    /// Act on one event; `Some` is what this pull returns.
    fn take(&mut self, event: Event<String, V>) -> Option<Option<Result<T, Error>>> {
        match event {
            Event::Port(port) => self.port = Some(port),
            Event::Ask => self.feed(),
            Event::Row(value) => {
                let Some(item) = self.held.pop_front() else {
                    return Some(Some(Err(
                        self.end(Error::defect("a row arrived with no record"))
                    )));
                };
                return (self.pair)(item, value).map(|row| Some(Ok(row)));
            }
            Event::End(ended) => {
                let ended = self.join().and(ended);
                let ended = self.stop.finish(ended);
                return Some(ended.err().map(Err));
            }
        }
        None
    }

    /// Answer one ask with the next record, the end, or the spent limit.
    fn feed(&mut self) {
        let input = match self.items.next() {
            None => Input::End,
            Some(_) if self.most.is_some_and(|most| self.fed >= most) => {
                Input::Failed(Error::usage(format!(
                    "this engine answers at most {} records in one call",
                    self.fed
                )))
            }
            Some(item) => {
                let text = super::Evidence::evidence(&item).to_owned();
                self.held.push_back(item);
                self.fed += 1;
                Input::Item(text)
            }
        };
        self.send(input);
    }

    /// Stop and join the scheduler, then report this error.
    fn end(&mut self, error: Error) -> Error {
        let _ended = self.join();
        error
    }
}

impl<I: Iterator, V, T> Stream<'_, I, V, T> {
    fn send(&self, input: Input<String, Error>) {
        if let Some(port) = &self.port {
            let _sent = port.send(input);
        }
    }

    /// Stop the scheduler, answer its last asks with the end, and join it.
    fn join(&mut self) -> Result<(), Error> {
        join_scheduler(
            &mut self.scheduler,
            &self.stop,
            &self.events,
            &mut self.port,
        )
    }
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

impl<I: Iterator, V, T> Drop for Stream<'_, I, V, T> {
    fn drop(&mut self) {
        let _joined = self.join();
    }
}
