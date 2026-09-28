//! The fixed-question, lazy batch planner behind public record decisions.

use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::thread::{self, JoinHandle};

use crate::core::{self, Batcher, Value};
use crate::engine::facade::{self, Input, InputPort};
use crate::public::Evidence;
use crate::public::error::Error;
use crate::public::options::Stop;
use crate::public::question::Question;
use crate::public::results::{
    self, Facts, ObservedQuestion, ObservedRow, QuestionDetail, RecordObservation,
};

use super::{Batch, Event, Source, TICK, join_scheduler, schedule};

type Answered = (Value, f64);
type Pair<I, T> = fn(I, Answered) -> Result<Option<T>, Error>;
type Convert<'a, I, T> =
    Box<dyn Fn(I, Answered, Option<results::Member>) -> Result<Option<T>, Error> + 'a>;

struct Packet {
    value: Option<Answered>,
    detail: Option<ObservedQuestion>,
    member: Option<results::Member>,
}

struct Work {
    batch: core::Batch,
    texts: Vec<String>,
}

struct Stream<'a, I: Iterator, T> {
    items: I,
    held: VecDeque<I::Item>,
    pending_texts: VecDeque<String>,
    ready: VecDeque<Result<T, Error>>,
    queue: VecDeque<Work>,
    planner: Batcher,
    pair: Convert<'a, I::Item, T>,
    events: Receiver<Event<Work, Vec<Packet>>>,
    port: Option<InputPort<Work, Vec<Packet>, Error>>,
    stop: Stop<'a>,
    facts: Option<Facts>,
    fed: usize,
    completed: usize,
    most: Option<usize>,
    interactive: bool,
    deferred_ask: bool,
    refusal: Option<Error>,
    exhausted: bool,
    scheduler: Option<JoinHandle<()>>,
}

/// Start an ordered stream. A pull reads only enough caller records to close
/// one request, and all caller controls remain on that thread.
#[allow(
    clippy::too_many_arguments,
    reason = "one fixed-question planner owns these inputs"
)]
pub(crate) fn start_planned<'a, I, T: 'a>(
    engine: Arc<facade::Engine>,
    records: I,
    stop: Stop<'a>,
    most: Option<usize>,
    question: core::Question,
    threshold: Option<core::Threshold>,
    tuned_for: Option<core::ProfileName>,
    setting: core::Setting,
    context: Option<core::Evidence>,
    pair: Pair<I::Item, T>,
) -> Result<Batch<'a, T>, Error>
where
    I: Iterator + 'a,
    I::Item: crate::public::Evidence,
{
    start(
        engine,
        records,
        stop,
        most,
        question,
        threshold,
        tuned_for,
        setting,
        context,
        false,
        Box::new(move |item, value, _| pair(item, value)),
    )
}

#[allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    reason = "one detailed stream retains its question, input and selected call controls"
)]
pub(crate) fn start_details<'a, I>(
    engine: Arc<facade::Engine>,
    records: I,
    stop: Stop<'a>,
    most: Option<usize>,
    question: &'a Question,
    setting: core::Setting,
    context: Option<core::Evidence>,
    context_sha256: Option<String>,
    profile: Option<&'a core::BackendProfile>,
) -> Result<Batch<'a, crate::public::results::Row<I::Item, results::Details>>, Error>
where
    I: Iterator + 'a,
    I::Item: Evidence + serde::Serialize,
{
    let backend = engine.backend().clone();
    start(
        engine,
        records,
        stop,
        most,
        question.core.clone(),
        question.threshold,
        question.profile.clone(),
        setting,
        context,
        true,
        Box::new(move |item, _, member| {
            let member = member.ok_or_else(|| Error::defect("a detailed row lost its receipt"))?;
            let details = results::Details::of_member(
                member,
                &item,
                question,
                &backend,
                profile,
                setting,
                context_sha256.as_deref(),
            )?;
            Ok(Some(results::Row::new(item, details, 0.0)))
        }),
    )
}

#[allow(
    clippy::too_many_arguments,
    reason = "one planned stream carries selected controls and its row conversion"
)]
fn start<'a, I, T: 'a>(
    engine: Arc<facade::Engine>,
    records: I,
    stop: Stop<'a>,
    most: Option<usize>,
    question: core::Question,
    threshold: Option<core::Threshold>,
    tuned_for: Option<core::ProfileName>,
    setting: core::Setting,
    context: Option<core::Evidence>,
    details: bool,
    pair: Convert<'a, I::Item, T>,
) -> Result<Batch<'a, T>, Error>
where
    I: Iterator + 'a,
    I::Item: Evidence,
{
    let planner = Batcher::new(
        engine.backend().clone(),
        engine.profile().cloned(),
        question.clone(),
        setting,
        context.clone(),
    )
    .map_err(Error::refused)?;
    let interactive = matches!(setting, core::Setting::Records(most) if most.get() == 1);
    let (sender, events) = channel();
    let cancel = stop.shared();
    let observing = stop.observing();
    let scheduler = thread::spawn(move || {
        schedule(
            &engine,
            &cancel,
            &|work: &Work| {
                answer(
                    &engine,
                    work,
                    &question,
                    threshold,
                    tuned_for.as_ref(),
                    context.as_ref(),
                    &cancel,
                    observing,
                    details,
                    setting,
                )
            },
            &sender,
        );
    });
    Ok(Batch {
        source: Box::new(Stream {
            items: records,
            held: VecDeque::new(),
            pending_texts: VecDeque::new(),
            ready: VecDeque::new(),
            queue: VecDeque::new(),
            planner,
            pair,
            events,
            port: None,
            stop,
            facts: None,
            fed: 0,
            completed: 0,
            most,
            interactive,
            deferred_ask: false,
            refusal: None,
            exhausted: false,
            scheduler: Some(scheduler),
        }),
    })
}

mod answer;
use answer::{answer, record};

impl<I, T> Source<T> for Stream<'_, I, T>
where
    I: Iterator,
    I::Item: crate::public::Evidence,
{
    fn pull(&mut self) -> Option<Result<T, Error>> {
        if self.deferred_ask && self.scheduler.is_some() {
            self.deferred_ask = false;
            self.feed();
        }
        loop {
            if let Some(row) = self.ready.pop_front() {
                return Some(row);
            }
            self.scheduler.as_ref()?;
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
            if self.deferred_ask && self.ready.is_empty() && self.fed == self.completed {
                self.deferred_ask = false;
                self.feed();
            }
        }
    }

    fn facts(&self) -> Option<&Facts> {
        self.facts.as_ref()
    }
}

impl<I, T> Stream<'_, I, T>
where
    I: Iterator,
    I::Item: crate::public::Evidence,
{
    fn take(&mut self, event: Event<Work, Vec<Packet>>) -> Option<Option<Result<T, Error>>> {
        match event {
            Event::Port(port) => self.port = Some(port),
            Event::Ask if self.interactive && self.fed > self.completed => {
                self.deferred_ask = true;
            }
            Event::Ask => self.feed(),
            Event::Row(rows) => self.take_rows(rows),
            Event::End(ended) => {
                let ended = self.join().and(ended);
                self.deferred_ask = false;
                let ended = self.stop.finish(ended);
                let facts = self.stop.facts();
                self.facts = Some(facts.clone());
                return Some(ended.err().map(|error| Err(error.with_facts(facts))));
            }
        }
        None
    }

    fn feed(&mut self) {
        while self.queue.is_empty() && self.refusal.is_none() && !self.exhausted {
            self.advance();
        }
        let input = if let Some(work) = self.queue.pop_front() {
            Input::Item(work)
        } else if let Some(error) = self.refusal.take() {
            Input::Failed(error)
        } else {
            Input::End
        };
        self.send(input);
    }

    fn advance(&mut self) {
        match self.items.next() {
            None => {
                self.exhausted = true;
                match self.planner.finish() {
                    Ok(Some(batch)) => self.queue_batch(batch),
                    Ok(None) => {}
                    Err(error) => self.refusal = Some(Error::refused(error)),
                }
            }
            Some(_) if self.most.is_some_and(|most| self.fed >= most) => {
                self.finish_before_refusal();
                self.refusal = Some(Error::usage(format!(
                    "this engine answers at most {} records in one call",
                    self.fed
                )));
            }
            Some(item) => self.push_item(item),
        }
    }

    fn push_item(&mut self, item: I::Item) {
        let text = item.evidence().to_owned();
        self.held.push_back(item);
        self.pending_texts.push_back(text.clone());
        let mut closed = Vec::new();
        let pushed = record(&text).and_then(|record| {
            self.planner
                .push(record, &mut closed)
                .map_err(Error::refused)
        });
        for batch in closed {
            self.queue_batch(batch);
        }
        match pushed {
            Ok(()) => self.fed += 1,
            Err(error) => {
                self.held.pop_back();
                self.pending_texts.pop_back();
                self.finish_before_refusal();
                self.refusal = Some(error);
            }
        }
    }

    fn take_rows(&mut self, rows: Vec<Packet>) {
        for packet in rows {
            if let Some(detail) = packet.detail.as_ref() {
                self.stop.observe(RecordObservation::Question {
                    index: self.completed,
                    member: None,
                    stage: None,
                    position: 0,
                    detail: QuestionDetail::of(detail),
                });
            }
            if self.stop.observer_panicked() {
                let _ = self.end(Error::cancelled());
                return;
            }
            let Some(value) = packet.value else {
                return;
            };
            let Some(item) = self.held.pop_front() else {
                let error = self.end(Error::defect("a row arrived with no record"));
                self.ready.push_back(Err(error));
                return;
            };
            if self.observe_row(&value.0) {
                return;
            }
            self.completed += 1;
            match (self.pair)(item, value, packet.member) {
                Ok(Some(row)) => self.ready.push_back(Ok(row)),
                Ok(None) => {}
                Err(error) => {
                    let error = self.end(error);
                    self.ready.push_back(Err(error));
                    return;
                }
            }
        }
    }

    fn observe_row(&mut self, value: &Value) -> bool {
        if !self.stop.observing() {
            return false;
        }
        let judgment = results::judgment(value);
        self.stop.observe(RecordObservation::Row {
            index: self.completed,
            value: ObservedRow::Judgment(&judgment),
        });
        if self.stop.observer_panicked() {
            let _ = self.end(Error::cancelled());
            return true;
        }
        false
    }

    fn queue_batch(&mut self, batch: core::Batch) {
        let texts = self.pending_texts.drain(..batch.questions.len()).collect();
        self.queue.push_back(Work { batch, texts });
    }

    fn finish_before_refusal(&mut self) {
        match self.planner.finish() {
            Ok(Some(batch)) => self.queue_batch(batch),
            Ok(None) => {}
            Err(error) => self.refusal = Some(Error::refused(error)),
        }
    }

    fn end(&mut self, error: Error) -> Error {
        let _joined = self.join();
        self.deferred_ask = false;
        let error = match self.stop.finish::<()>(Err(error)) {
            Err(error) => error,
            Ok(()) => Error::defect("a failed stream returned a value"),
        };
        let facts = self.stop.facts();
        self.facts = Some(facts.clone());
        error.with_facts(facts)
    }
}

impl<I: Iterator, T> Stream<'_, I, T> {
    fn send(&self, input: Input<Work, Error>) {
        if let Some(port) = &self.port {
            let _sent = port.send(input);
        }
    }

    fn join(&mut self) -> Result<(), Error> {
        join_scheduler(
            &mut self.scheduler,
            &self.stop,
            &self.events,
            &mut self.port,
        )
    }
}

impl<I: Iterator, T> Drop for Stream<'_, I, T> {
    fn drop(&mut self) {
        let _joined = self.join();
    }
}
