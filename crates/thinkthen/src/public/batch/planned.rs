//! The fixed-question, lazy batch planner behind public record decisions.

use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::thread::{self, JoinHandle};

use crate::core::{self, AnswerOutcome, BatchRecord, Batcher};
use crate::engine::facade::{self, Completed, Input, InputPort};
use crate::public::Evidence;
use crate::public::error::Error;
use crate::public::options::Stop;
use crate::public::results::{self, Answer, Facts};

use super::{Batch, Event, Source, TICK, join_scheduler, schedule};

type Decided = (Answer, f64);

struct Work {
    batch: core::Batch,
    texts: Vec<String>,
}

struct Stream<'a, I: Iterator, T> {
    items: I,
    held: VecDeque<I::Item>,
    pending_texts: VecDeque<String>,
    ready: VecDeque<T>,
    queue: VecDeque<Work>,
    planner: Batcher,
    pair: fn(I::Item, Decided) -> Option<T>,
    events: Receiver<Event<Work, Vec<Decided>>>,
    port: Option<InputPort<Work, Vec<Decided>, Error>>,
    stop: Stop<'a>,
    facts: Option<Facts>,
    fed: usize,
    most: Option<usize>,
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
    setting: core::Setting,
    context: Option<core::Evidence>,
    pair: fn(I::Item, Decided) -> Option<T>,
) -> Result<Batch<'a, T>, Error>
where
    I: Iterator + 'a,
    I::Item: crate::public::Evidence,
{
    let planner = Batcher::new(
        engine.backend().clone(),
        engine.profile().cloned(),
        question.clone(),
        setting,
        context.clone(),
    )
    .map_err(Error::refused)?;
    let (sender, events) = channel();
    let cancel = stop.shared();
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
                    context.as_ref(),
                    &cancel,
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
            most,
            refusal: None,
            exhausted: false,
            scheduler: Some(scheduler),
        }),
    })
}

fn answer(
    engine: &facade::Engine,
    work: &Work,
    question: &core::Question,
    threshold: Option<core::Threshold>,
    context: Option<&core::Evidence>,
    cancel: &crate::engine::Cancel<'static>,
) -> Result<Completed<Vec<Decided>, Error>, Error> {
    match engine.ask_batch(&work.batch, cancel) {
        Ok(answered) => rows(&work.batch, answered, threshold),
        Err(error) if error.too_large() && work.texts.len() > 1 => {
            let records = work
                .texts
                .iter()
                .map(|text| Ok((record(text)?, question.clone())))
                .collect::<Result<Vec<_>, Error>>()?;
            let [left, right] = core::batch::halves_with_questions(
                engine.backend(),
                engine.profile(),
                context,
                records,
            )
            .map_err(Error::refused)?;
            let left = rows(
                &left,
                engine.ask_batch(&left, cancel).map_err(Error::from)?,
                threshold,
            )?;
            if left.stop.is_some() {
                return Ok(left);
            }
            if let Some(stop) = cancel.stop() {
                return Ok(Completed {
                    stop: Some(stop.into()),
                    ..left
                });
            }
            let right = match engine.ask_batch(&right, cancel) {
                Ok(answered) => rows(&right, answered, threshold),
                Err(error) => Err(Error::from(error)),
            };
            match right {
                Ok(right) => Ok(Completed {
                    value: left.value.into_iter().chain(right.value).collect(),
                    records: left.records + right.records,
                    replayed: left.replayed + right.replayed,
                    partial_failure: left.partial_failure || right.partial_failure,
                    stop: right.stop,
                }),
                Err(error) => Ok(Completed {
                    stop: Some(error),
                    ..left
                }),
            }
        }
        Err(error) => Err(error.into()),
    }
}

fn rows(
    batch: &core::Batch,
    answered: crate::engine::prepared_request::Answered,
    threshold: Option<core::Threshold>,
) -> Result<Completed<Vec<Decided>, Error>, Error> {
    if batch.outcomes.len() != batch.questions.len() {
        return Err(Error::defect("a batch lost its row outcomes"));
    }
    let mut values = Vec::with_capacity(batch.outcomes.len());
    let mut stop = None;
    for &place in &batch.outcomes {
        match answered.reply.outcomes().get(place) {
            Some(AnswerOutcome::Answered(answer)) => {
                let (value, _) = answer.read(threshold);
                values.push((results::answer(&value), answer.yes().unwrap_or_default()));
            }
            _ => {
                stop = Some(Error::of(
                    crate::public::error::ErrorKind::Backend,
                    "a backend question failed in a batch",
                ));
                break;
            }
        }
    }
    if values.is_empty()
        && let Some(error) = stop
    {
        return Err(error);
    }
    Ok(Completed {
        records: values.len(),
        replayed: if answered.replayed { values.len() } else { 0 },
        value: values,
        partial_failure: false,
        stop,
    })
}

fn record(text: &str) -> Result<BatchRecord, Error> {
    Ok(BatchRecord {
        evidence: core::Evidence::new(text.to_owned())
            .map_err(|_| Error::usage("evidence is text, not white space"))?,
        value: core::Json::String(text.to_owned()),
    })
}

impl<I, T> Source<T> for Stream<'_, I, T>
where
    I: Iterator,
    I::Item: crate::public::Evidence,
{
    fn pull(&mut self) -> Option<Result<T, Error>> {
        self.scheduler.as_ref()?;
        loop {
            if let Some(row) = self.ready.pop_front() {
                return Some(Ok(row));
            }
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

    fn facts(&self) -> Option<&Facts> {
        self.facts.as_ref()
    }
}

impl<I, T> Stream<'_, I, T>
where
    I: Iterator,
    I::Item: crate::public::Evidence,
{
    fn take(&mut self, event: Event<Work, Vec<Decided>>) -> Option<Option<Result<T, Error>>> {
        match event {
            Event::Port(port) => self.port = Some(port),
            Event::Ask => self.feed(),
            Event::Row(rows) => {
                if let Err(error) = self.take_rows(rows) {
                    return Some(Some(Err(error)));
                }
            }
            Event::End(ended) => {
                let ended = self.join().and(ended);
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

    fn take_rows(&mut self, rows: Vec<Decided>) -> Result<(), Error> {
        for value in rows {
            let Some(item) = self.held.pop_front() else {
                return Err(self.end(Error::defect("a row arrived with no record")));
            };
            if let Some(row) = (self.pair)(item, value) {
                self.ready.push_back(row);
            }
        }
        Ok(())
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
