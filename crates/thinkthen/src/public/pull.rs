//! The calling thread as the host of one pipeline call, by ADR 0111
//! section 4: `ask_all` runs on a background thread, and the calling thread
//! pulls a caller record on each ask and returns on each row. The caller's
//! records never leave its thread, so they need not be `Send`.

use std::collections::VecDeque;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::engine::facade;
use crate::engine::fork_safe::{Receiver, RecvTimeoutError, Sender, channel};
use crate::engine::pipeline::{Asker, Failed, Flow, Host, Input, Packing, Port};
use crate::public::InputEvidence;
use crate::public::asking::Text;
use crate::public::error::Error;
use crate::public::options::{Stop, guarded};
use crate::public::results::Facts;

use super::batch::{Batch, Source};

/// How often a waiting pull runs the caller's check and drains attempts.
const TICK: Duration = Duration::from_millis(50);

pub(crate) type Row<A> = Result<<A as Asker>::Row, Failed<<A as Asker>::Error>>;

enum Event<A: Asker> {
    Port(Port<A::Input, A::Error>),
    Ask,
    ReaderReady,
    Row(Row<A>),
    End(Result<(), Error>),
}

/// The coordinator's side: each ask and each row goes to the calling thread.
struct Bridge<A: Asker> {
    events: Sender<Event<A>>,
    pause: Option<Duration>,
}

impl<A: Asker> Host<A> for Bridge<A> {
    fn pause(&self) -> Duration {
        self.pause.unwrap_or(TICK)
    }
    fn ask(&mut self) -> bool {
        self.events.send(Event::Ask).is_ok()
    }

    fn row(&mut self, _place: usize, result: Row<A>) -> Flow {
        let failed = result.is_err();
        if self.events.send(Event::Row(result)).is_err() || failed {
            Flow::Stop
        } else {
            Flow::Continue
        }
    }
}

/// Turn one pipeline row and the caller's record into the batch's item, or
/// `None` for a row the batch leaves out. It runs on the calling thread. A
/// row with no record is the refusal of a record past the engine's limit,
/// or the call's stop.
pub(crate) type Pair<'a, A, I, T> =
    Box<dyn FnMut(&Stop<'_>, usize, Option<I>, Row<A>) -> Result<Option<T>, Error> + 'a>;

/// Admit a pulled original into the existing scheduler's concrete input.
pub(crate) type Prepare<'a, A, R> =
    Box<dyn FnMut(usize, &R) -> Result<<A as Asker>::Input, Error> + 'a>;

/// What one pulled call holds besides its records.
pub(crate) struct Call<'a> {
    pub(crate) engine: Arc<facade::Engine>,
    pub(crate) stop: Stop<'a>,
    pub(crate) packing: Packing,
    pub(crate) most: Option<usize>,
}

/// Start `asker` over `records` and return the lazy batch of its rows.
pub(crate) fn start<'a, A, I, T: 'a>(
    call: Call<'a>,
    asker: A,
    records: I,
    pair: Pair<'a, A, I::Item, T>,
) -> Batch<'a, T>
where
    A: Asker<Input = Text> + Send + 'static,
    A::Row: Send + 'static,
    A::Error: From<Error> + Send + 'static,
    I: Iterator + 'a,
    I::Item: InputEvidence,
{
    try_start(call, asker, records.map(Ok), pair)
}

/// Start the same ordered pipeline over a fallible caller iterator.
pub(crate) fn try_start<'a, A, I, R, T: 'a>(
    call: Call<'a>,
    asker: A,
    records: I,
    pair: Pair<'a, A, R, T>,
) -> Batch<'a, T>
where
    A: Asker<Input = Text> + Send + 'static,
    A::Row: Send + 'static,
    A::Error: From<Error> + Send + 'static,
    I: Iterator<Item = Result<R, Error>> + 'a,
    R: InputEvidence + 'a,
{
    try_start_prepared(
        call,
        asker,
        records,
        Box::new(|at, item| {
            Ok(Text {
                at,
                input: item.question_input(),
            })
        }),
        pair,
    )
}

/// Pull fallible originals with typed per-record preparation on their owning thread.
pub(crate) fn try_start_prepared<'a, A, I, R: 'a, T: 'a>(
    call: Call<'a>,
    asker: A,
    records: I,
    prepare: Prepare<'a, A, R>,
    pair: Pair<'a, A, R, T>,
) -> Batch<'a, T>
where
    A: Asker + Send + 'static,
    A::Row: Send + 'static,
    A::Error: From<Error> + Send + 'static,
    I: Iterator<Item = Result<R, Error>> + 'a,
{
    let (events, received) = channel();
    let cli_reader = call.stop.cli_reader;
    if let Some(reader) = cli_reader {
        let wake_events = events.clone();
        reader.wake().register(Box::new(move || {
            let _sent = wake_events.send(Event::ReaderReady);
        }));
    }
    let pause = cli_reader.and_then(|reader| reader.pause);
    let cancel = call.stop.shared();
    let (engine, packing) = (call.engine, call.packing);
    let coordinator = thread::spawn(move || {
        let ended = guarded(|| {
            engine
                .ask_all(
                    &asker,
                    packing,
                    |port| {
                        let _sent = events.send(Event::Port(port));
                        Bridge {
                            events: events.clone(),
                            pause,
                        }
                    },
                    &cancel,
                )
                .map_err(Error::from)
        });
        let _sent = events.send(Event::End(ended));
    });
    Batch::from_source(Box::new(Pull {
        items: records,
        held: VecDeque::new(),
        ready: VecDeque::new(),
        pair,
        prepare,
        events: received,
        port: None,
        stop: call.stop,
        facts: None,
        fed: 0,
        completed: 0,
        most: call.most,
        interactive: packing.inputs == Some(1) && cli_reader.is_none(),
        pending_ask: false,
        deferred: false,
        coordinator: Some(coordinator),
    }))
}

struct Pull<'a, A: Asker, I: Iterator, R, T> {
    items: I,
    held: VecDeque<R>,
    ready: VecDeque<Result<T, Error>>,
    pair: Pair<'a, A, R, T>,
    prepare: Prepare<'a, A, R>,
    events: Receiver<Event<A>>,
    port: Option<Port<A::Input, A::Error>>,
    stop: Stop<'a>,
    facts: Option<Facts>,
    fed: usize,
    completed: usize,
    most: Option<usize>,
    /// Batch 1 returns each row before it pulls the next record.
    interactive: bool,
    deferred: bool,
    pending_ask: bool,
    coordinator: Option<JoinHandle<()>>,
}

impl<A, I, R, T> Source<T> for Pull<'_, A, I, R, T>
where
    A: Asker,
    A::Error: From<Error>,
    I: Iterator<Item = Result<R, Error>>,
{
    fn pull(&mut self) -> Option<Result<T, Error>> {
        if self.deferred && self.coordinator.is_some() {
            self.deferred = false;
            self.feed();
        }
        loop {
            if let Some(row) = self.ready.pop_front() {
                return Some(row);
            }
            self.coordinator.as_ref()?;
            if self.stop.interrupted() {
                self.stop.fire();
            }
            if self.pending_ask {
                self.feed();
            }
            let event = if self.stop.polls() {
                match self.events.recv_timeout(TICK) {
                    Ok(event) => event,
                    Err(RecvTimeoutError::Timeout) => continue,
                    Err(RecvTimeoutError::Disconnected) => ended_early(),
                }
            } else {
                self.events.recv().unwrap_or_else(|_| ended_early())
            };
            if let Some(ended) = self.take(event) {
                return ended;
            }
            if self.deferred && self.ready.is_empty() && self.fed == self.completed {
                self.deferred = false;
                self.feed();
            }
        }
    }

    fn facts(&self) -> Option<&Facts> {
        self.facts.as_ref()
    }
}

fn ended_early<A: Asker>() -> Event<A> {
    Event::End(Err(Error::defect("the record scheduler ended early")))
}

impl<A, I, R, T> Pull<'_, A, I, R, T>
where
    A: Asker,
    A::Error: From<Error>,
    I: Iterator<Item = Result<R, Error>>,
{
    /// Take one event; `Some` once the call has ended.
    fn take(&mut self, event: Event<A>) -> Option<Option<Result<T, Error>>> {
        match event {
            Event::Port(port) => self.port = Some(port),
            Event::Ask if self.interactive && self.fed > self.completed => self.deferred = true,
            Event::Ask => {
                self.pending_ask = true;
                self.feed();
            }
            Event::ReaderReady => {
                if self.pending_ask {
                    self.feed();
                }
            }
            Event::Row(row) => self.row(row),
            Event::End(ended) => {
                let joined = self.join();
                let ended = self.stop.finish(joined.and(ended));
                let facts = self.stop.facts();
                self.facts = Some(facts.clone());
                return Some(ended.err().map(|error| Err(error.with_facts(facts))));
            }
        }
        None
    }

    /// Answer one ask with the caller's next record, its end, or the refusal
    /// of one record past the engine's limit.
    fn feed(&mut self) {
        if self.stop.interrupted() {
            self.stop.fire();
            self.pending_ask = false;
            return;
        }
        if self.stop.cli_reader.is_some_and(|reader| !reader.ready()) {
            self.pending_ask = true;
            return;
        }
        self.pending_ask = false;
        let input = match self.items.next() {
            None => Input::End,
            Some(Err(error)) => Input::Failed(A::Error::from(error)),
            Some(Ok(_)) if self.most.is_some_and(|most| self.fed >= most) => {
                Input::Failed(A::Error::from(Error::usage(format!(
                    "this engine answers at most {} records in one call",
                    self.fed
                ))))
            }
            Some(Ok(item)) => match (self.prepare)(self.fed, &item) {
                Ok(input) => {
                    self.held.push_back(item);
                    self.fed += 1;
                    Input::Item(input)
                }
                Err(error) => Input::Failed(A::Error::from(error)),
            },
        };
        if let Some(port) = &self.port {
            let _sent = port.send(input);
        }
    }

    fn row(&mut self, row: Row<A>) {
        let item = match &row {
            Err(Failed::Stopped(_)) => None,
            _ => self.held.pop_front(),
        };
        let paired = (self.pair)(&self.stop, self.completed, item, row);
        if self.stop.observer_panicked() {
            let error = self.end(Error::cancelled());
            self.ready.push_back(Err(error));
            return;
        }
        if paired.is_ok() {
            self.completed += 1;
            self.stop.shared().finished_records(1);
        }
        match paired {
            Ok(Some(value)) => self.ready.push_back(Ok(value)),
            Ok(None) => {}
            Err(error) => {
                let error = self.end(error);
                self.ready.push_back(Err(error));
            }
        }
    }

    /// Stop the call at a failed row, join it, and give the error its facts.
    fn end(&mut self, error: Error) -> Error {
        let _joined = self.join();
        self.deferred = false;
        let error = match self.stop.finish::<()>(Err(error)) {
            Err(error) => error,
            Ok(()) => Error::defect("a failed batch returned a value"),
        };
        let facts = self.stop.facts();
        self.facts = Some(facts.clone());
        error.with_facts(facts)
    }
}

impl<A: Asker, I: Iterator, R, T> Pull<'_, A, I, R, T> {
    /// Stop the coordinator and join it, draining attempts while it winds down.
    fn join(&mut self) -> Result<(), Error> {
        let Some(coordinator) = self.coordinator.take() else {
            return Ok(());
        };
        self.stop.fire();
        self.pending_ask = false;
        if let Some(reader) = self.stop.cli_reader {
            reader.wake().clear();
        }
        self.port = None;
        while !coordinator.is_finished() {
            self.stop.drain_attempts();
            match self.events.recv_timeout(TICK) {
                Ok(_) | Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
        let joined = coordinator
            .join()
            .map_err(|_| Error::defect("the record scheduler panicked"));
        self.stop.drain_attempts();
        self.stop.resume_panic_after_join();
        joined
    }
}

impl<A: Asker, I: Iterator, R, T> Drop for Pull<'_, A, I, R, T> {
    fn drop(&mut self) {
        let _joined = self.join();
    }
}

/// The packing one public call asks for.
pub(crate) fn packing(setting: crate::core::Setting, context: bool, continues: bool) -> Packing {
    Packing {
        inputs: match setting {
            crate::core::Setting::Records(most) => Some(most.get()),
            crate::core::Setting::Max => None,
        },
        questions: None,
        sized: true,
        strict_singleton: false,
        context,
        detailed: false,
        continues,
    }
}
