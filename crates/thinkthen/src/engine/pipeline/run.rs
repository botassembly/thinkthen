//! The coordinator's state: the unemitted window, the keys on their way,
//! the open request, and the call's store.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, SyncSender};
use std::time::{Duration, Instant};

use super::send::{Done, Job};
use super::{Answered, Asker, Event, Failed, Flow, Host};
use crate::core::ModelName;
use crate::core::pack::{self, Ask, Entry, Packer, QuestionKey};
use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::engine::schedule::Input;
use crate::engine::store::{Found, JSONL, Row, Store};
use crate::engine::usage::Counters;

mod slot;
use slot::Slot;

/// How long input may pause before the open request goes out.
pub(super) const PAUSE: Duration = Duration::from_millis(50);

/// The process counters a stored answer reaches.
#[derive(Clone, Copy)]
pub(super) struct Counts<'a> {
    pub(super) usage: &'a Counters,
    pub(super) cache_answers: bool,
}

/// The address and model every row of one call records.
pub(super) struct Call {
    pub(super) url: String,
    pub(super) model: String,
}

/// How much one call holds at once: W unemitted inputs, by ADR 0111
/// section 4, and one request for each send worker.
pub(super) struct Bounds {
    pub(super) window: usize,
    pub(super) jobs: usize,
    /// Whether the host takes rows past a failure. When it does not, a
    /// failed request sends nothing further, as one job would.
    pub(super) continues: bool,
}

pub(super) struct Run<'a, A: Asker> {
    asker: &'a A,
    call: Call,
    store: Option<Store>,
    packer: Packer<(Ask, usize)>,
    counts: Counts<'a>,
    window: usize,
    jobs: usize,
    continues: bool,
    slots: VecDeque<Slot<A>>,
    /// The place of the window's first input.
    first: usize,
    /// Each key on its way, and every input place and position it answers.
    waiting: HashMap<QuestionKey, Vec<(usize, usize)>>,
    closed: VecDeque<Job>,
    busy: usize,
    reading: bool,
    exhausted: bool,
    /// Why the call is stopping: it sends and reads nothing more, and waits
    /// for the requests on their way so their rows still print.
    stopping: Option<Error>,
    halted: bool,
    arrived: Instant,
}

impl<'a, A: Asker> Run<'a, A> {
    pub(super) fn new(
        asker: &'a A,
        call: Call,
        store: Option<Store>,
        packer: Packer<(Ask, usize)>,
        bounds: Bounds,
        counts: Counts<'a>,
    ) -> Self {
        Self {
            asker,
            call,
            store,
            packer,
            counts,
            window: bounds.window.max(1),
            jobs: bounds.jobs,
            continues: bounds.continues,
            slots: VecDeque::new(),
            first: 0,
            waiting: HashMap::new(),
            closed: VecDeque::new(),
            busy: 0,
            reading: false,
            exhausted: false,
            stopping: None,
            halted: false,
            arrived: Instant::now(),
        }
    }

    /// Run until every input is emitted or the host stops, and every sent
    /// request has come back.
    pub(super) fn drive(
        mut self,
        host: &mut impl Host<A>,
        received: &Receiver<Event<A::Input, A::Error>>,
        work: &SyncSender<Job>,
        cancel: &Cancel,
    ) {
        loop {
            if !self.settle(host, cancel) {
                return;
            }
            if !self.halted && self.stopping.is_none() {
                self.dispatch(host, work);
            }
            cancel.observed_block();
            self.receive(received, cancel);
        }
    }

    /// Emit what is ready and take in a stop. False once the call is over.
    fn settle(&mut self, host: &mut impl Host<A>, cancel: &Cancel) -> bool {
        self.emit_ready(host);
        self.halted |= self.asker.gone();
        if !self.halted
            && self.stopping.is_none()
            && let Some(stop) = cancel.stop_between_sends()
        {
            self.stopping = Some(stop);
            self.closed.clear();
        }
        if !self.halted
            && self.busy == 0
            && let Some(stop) = self.stopping.clone()
        {
            if !self.slots.is_empty() || !self.exhausted {
                let _flow = host.row(self.first, Err(Failed::Stopped(stop)));
            }
            self.halted = true;
        }
        if self.halted {
            self.closed.clear();
            return self.busy != 0;
        }
        !(self.exhausted && self.slots.is_empty() && self.busy == 0)
    }

    /// Close a full window, hand closed requests to free workers, and ask
    /// for one more input while the window has room.
    fn dispatch(&mut self, host: &mut impl Host<A>, work: &SyncSender<Job>) {
        if self.slots.len() >= self.window || self.exhausted {
            self.close();
        }
        while self.busy < self.jobs
            && let Some(job) = self.closed.pop_front()
        {
            if work.send(job).is_err() {
                self.halted = true;
                return;
            }
            self.busy += 1;
        }
        if !self.reading && !self.exhausted && self.slots.len() < self.window {
            self.reading = host.ask();
            self.exhausted |= !self.reading;
        }
    }

    /// Wait for one input or reply, closing the open request at a pause.
    fn receive(&mut self, received: &Receiver<Event<A::Input, A::Error>>, cancel: &Cancel) {
        let wait = if self.packer.is_open() && self.reading {
            PAUSE
                .saturating_sub(self.arrived.elapsed())
                .max(Duration::from_millis(1))
        } else {
            Cancel::poll()
        };
        match received.recv_timeout(wait) {
            Ok(Event::Input(input)) => self.input(input, cancel),
            Ok(Event::Done(done)) => {
                self.busy -= 1;
                for one in done {
                    self.answered(one, cancel);
                }
            }
            Err(RecvTimeoutError::Timeout) => {
                if self.packer.is_open() && self.arrived.elapsed() >= PAUSE {
                    self.close();
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                self.exhausted = true;
                self.reading = false;
            }
        }
    }

    fn close(&mut self) {
        if let Some(packed) = self.packer.close() {
            self.closed.push_back(Job {
                state: packed.state,
                asks: packed.items,
                body: packed.body,
            });
        }
    }

    /// Emit every finished input at the head of the window, in order.
    fn emit_ready(&mut self, host: &mut impl Host<A>) {
        while !self.halted && self.slots.front().is_some_and(Slot::ready) {
            let Some(slot) = self.slots.pop_front() else {
                return;
            };
            let place = self.first;
            self.first += 1;
            let result = match (slot.failed, slot.input) {
                (Some(failure), _) => Err(failure),
                (None, Some(input)) => {
                    let answers = slot.answers.into_iter().flatten().collect();
                    self.asker.row(input, answers).map_err(Failed::Asker)
                }
                (None, None) => Err(Failed::Stopped(Error::Defect(
                    "an input disappeared before its row",
                ))),
            };
            if host.row(place, result) == Flow::Stop {
                self.halted = true;
            }
        }
    }

    fn input(&mut self, input: Input<A::Input, A::Error>, cancel: &Cancel) {
        self.reading = false;
        self.arrived = Instant::now();
        match input {
            Input::End => self.exhausted = true,
            // A stopping call admits nothing more.
            Input::Item(_) | Input::Failed(_) if self.stopping.is_some() => {}
            Input::Failed(error) => {
                self.slots.push_back(Slot::failed(Failed::Asker(error)));
                self.exhausted = true;
            }
            Input::Item(input) => {
                let slot = self.admit(input, cancel);
                self.slots.push_back(slot);
            }
        }
    }

    /// Look one input's questions up, and pack the ones nothing answers.
    fn admit(&mut self, input: A::Input, cancel: &Cancel) -> Slot<A> {
        let place = self.first + self.slots.len();
        let label = self.asker.label(&input);
        let engine = |error| {
            Slot::failed(Failed::Engine {
                error,
                first: label,
                last: label,
            })
        };
        let asks = match self.asker.asks(&input) {
            Ok(asks) => asks,
            Err(error) => return Slot::failed(Failed::Asker(error)),
        };
        let mut slot = Slot {
            input: Some(input),
            answers: vec![None; asks.len()],
            missing: asks.len(),
            failed: None,
            failed_at: 0,
        };
        let found = match self.lookup(&asks, cancel) {
            Ok(found) => found,
            Err(error) => return engine(error),
        };
        let mut entries = Vec::new();
        let mut joining = Vec::new();
        let mut added = HashSet::new();
        for (position, (ask, stored)) in asks.into_iter().zip(found).enumerate() {
            let stored = match stored.map(|found| self.stored(&ask, found, label, cancel)) {
                Some(Ok(answered)) => Some(answered),
                Some(Err(Some(error))) => return engine(error),
                Some(Err(None)) | None => None,
            };
            if let Some(answered) = stored {
                slot.answer(position, answered);
                continue;
            }
            if self.store.as_ref().is_some_and(Store::replays) {
                return engine(Error::QuestionMiss(ask.key.hex()));
            }
            if self.waiting.contains_key(&ask.key) || !added.insert(ask.key) {
                joining.push((ask.key, position));
                continue;
            }
            joining.push((ask.key, position));
            entries.push(Entry {
                state: ask.state.clone(),
                question: Arc::clone(&ask.question),
                options: options(&ask),
                item: (ask, label),
            });
        }
        let mut closed = Vec::new();
        if let Err(error) = self.packer.add(entries, &mut closed) {
            return Slot::failed(Failed::Pack { error, at: label });
        }
        self.closed.extend(closed.into_iter().map(|packed| Job {
            state: packed.state,
            asks: packed.items,
            body: packed.body,
        }));
        for (key, position) in joining {
            self.waiting.entry(key).or_default().push((place, position));
        }
        slot
    }

    fn lookup(&mut self, asks: &[Ask], cancel: &Cancel) -> Result<Vec<Option<Found>>, Error> {
        match self.store.as_mut().filter(|store| store.looks_up()) {
            Some(store) => {
                let keys: Vec<_> = asks.iter().map(|ask| ask.key).collect();
                store.lookup(&keys, cancel)
            }
            None => Ok(vec![None; asks.len()]),
        }
    }

    /// A stored answer as an answer, or `Err(None)` when it no longer
    /// decodes under a cache, which counts it a miss.
    fn stored(
        &self,
        ask: &Ask,
        found: Found,
        label: usize,
        cancel: &Cancel,
    ) -> Result<Answered, Option<Error>> {
        let decodes = pack::read(
            std::slice::from_ref(&ask.decoder),
            &[Ok(found.answer.as_str())],
            &found.answered_by,
        )
        .is_ok_and(|outcomes| {
            outcomes
                .iter()
                .all(|outcome| matches!(outcome, crate::core::AnswerOutcome::Answered(_)))
        });
        if !decodes {
            return Err(self
                .store
                .as_ref()
                .filter(|store| store.replays())
                .map(|_| {
                    Error::Entry(
                        JSONL.to_owned(),
                        format!(
                            "holds an answer to question `{}` that no longer decodes",
                            ask.key.hex()
                        ),
                    )
                }));
        }
        if self.counts.cache_answers {
            self.counts.usage.cache_answer();
            cancel.cache_answer();
        }
        if let Ok(model) = ModelName::new(found.answered_by.clone()) {
            self.counts.usage.answered_by(&model);
        }
        cancel.answered_by(&found.answered_by);
        Ok(Answered {
            key: ask.key,
            answer: Ok(Arc::from(found.answer)),
            answered_by: Arc::from(found.answered_by),
            usage: found.usage,
            cached: true,
            requests_sent: 0,
            attempts: Arc::from([]),
            span: (label, label),
        })
    }

    /// Store a reply's good answers and hand every answer to the inputs that wait on it.
    fn answered(&mut self, done: Done, cancel: &Cancel) {
        let span = done
            .asks
            .iter()
            .fold((usize::MAX, 0), |(low, high), (_, label)| {
                (low.min(*label), high.max(*label))
            });
        let split = match done.result {
            Ok(split) => split,
            Err(error) => {
                // Every earlier input's questions went out before this
                // request, so their rows and this failure still print.
                if !self.continues && self.stopping.is_none() {
                    self.stopping = Some(error.clone());
                    self.closed.clear();
                }
                return self.fail_all(&done.asks, &error, span);
            }
        };
        if split.answers.len() != done.asks.len() {
            let error = Error::Defect("a reply answered the wrong number of questions");
            return self.fail_all(&done.asks, &error, span);
        }
        let count = done.asks.len();
        let shares = pack::shares(split.usage, count);
        let rows: Vec<Row<'_>> = done
            .asks
            .iter()
            .zip(&split.answers)
            .zip(&shares)
            .filter_map(|(((ask, _), answer), usage)| {
                Some(Row {
                    key: ask.key,
                    url: &self.call.url,
                    model: &self.call.model,
                    state: &ask.state,
                    question: &ask.question,
                    answer: answer.as_deref().ok()?,
                    answered_by: split.model.as_str(),
                    usage: *usage,
                    taken_at: done.taken_at,
                    origin: "live",
                })
            })
            .collect();
        let written = match self.store.as_mut() {
            Some(store) => store.write(&rows, cancel),
            None => Ok(()),
        };
        drop(rows);
        if let Err(error) = written {
            return self.fail_all(&done.asks, &error, span);
        }
        let answered_by: Arc<str> = Arc::from(split.model.as_str());
        for (position, (((ask, _), answer), usage)) in
            done.asks.iter().zip(split.answers).zip(shares).enumerate()
        {
            let answered = Answered {
                key: ask.key,
                answer: answer.map(Arc::from),
                answered_by: Arc::clone(&answered_by),
                usage,
                cached: false,
                requests_sent: crate::core::share(done.requests_sent, count, position),
                attempts: Arc::clone(&done.attempts),
                span,
            };
            self.deliver(&ask.key, answered);
        }
    }

    /// Hand one answer to every input waiting on its key. The first waiter
    /// counts the send; a waiter that joined it shares the answer and its
    /// usage share, and sent nothing.
    fn deliver(&mut self, key: &QuestionKey, mut answered: Answered) {
        for (place, at) in self.waiting.remove(key).unwrap_or_default() {
            if let Some(slot) = self.slot(place) {
                slot.answer(at, answered.clone());
            }
            answered.requests_sent = 0;
        }
    }

    fn fail_all(&mut self, asks: &[(Ask, usize)], error: &Error, (first, last): (usize, usize)) {
        let waiters: Vec<(usize, usize)> = asks
            .iter()
            .flat_map(|(ask, _)| self.waiting.remove(&ask.key).unwrap_or_default())
            .collect();
        for (place, at) in waiters {
            if let Some(slot) = self.slot(place) {
                let error = error.clone();
                slot.fail(at, Failed::Engine { error, first, last });
            }
        }
    }

    fn slot(&mut self, place: usize) -> Option<&mut Slot<A>> {
        place
            .checked_sub(self.first)
            .and_then(|index| self.slots.get_mut(index))
    }
}

/// The options a pick carries, which a profile may limit.
pub(crate) fn options(ask: &Ask) -> usize {
    match &ask.decoder {
        crate::core::Question::Choose { options, .. } => options.count(),
        _ => 0,
    }
}
