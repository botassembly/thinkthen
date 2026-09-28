//! Caller-owned annotation input and ordered group-fragment assembly.

use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::thread::{self, JoinHandle};

use super::{Batch, Event, Source, TICK, join_scheduler, schedule};
use crate::core::{self, AnnotatedValue};
use crate::engine::facade::{
    self, Completed, GroupAnswer, GroupPlanError, GroupPlanner, GroupWork, Input, InputPort,
};
use crate::public::annotated::AnnotatedRecord;
use crate::public::bulk::{
    annotation_record, observe_annotated, observe_annotated_questions, render_annotation,
};
use crate::public::error::{Error, ErrorKind};
use crate::public::options::Stop;
use crate::public::{Evidence, results::Facts};

struct Fragment {
    row: usize,
    group: usize,
    answer: GroupAnswer,
}

fn plan_error(error: GroupPlanError) -> Error {
    match error {
        GroupPlanError::Part(core::PartError::Record(error)) => Error::refused(error),
        GroupPlanError::Part(core::PartError::Reading(_)) => {
            Error::defect("a checked question set could not read its parts")
        }
        GroupPlanError::Batch(error) => Error::refused(error),
        GroupPlanError::Defect(message) => Error::defect(message),
    }
}

struct RowState {
    groups: Vec<Option<GroupAnswer>>,
}

struct Stream<'a, I: Iterator> {
    items: I,
    held: BTreeMap<usize, I::Item>,
    rows: BTreeMap<usize, RowState>,
    ready: VecDeque<Result<AnnotatedRecord<I::Item>, Error>>,
    planner: GroupPlanner,
    set: core::QuestionSet,
    engine: Arc<facade::Engine>,
    events: Receiver<Event<GroupWork, Vec<Fragment>>>,
    port: Option<InputPort<GroupWork, Vec<Fragment>, Error>>,
    stop: Stop<'a>,
    facts: Option<Facts>,
    fed: usize,
    completed: usize,
    most: Option<usize>,
    refusal: Option<Error>,
    exhausted: bool,
    pending_asks: usize,
    scheduler: Option<JoinHandle<()>>,
}

/// Begin an annotation stream without moving the caller's iterator to a worker.
pub(crate) fn start_annotation<'a, I>(
    engine: Arc<facade::Engine>,
    set: core::QuestionSet,
    records: I,
    stop: Stop<'a>,
    most: Option<usize>,
    setting: core::Setting,
) -> Result<Batch<'a, AnnotatedRecord<I::Item>>, Error>
where
    I: Iterator + 'a,
    I::Item: Evidence,
{
    let planner = GroupPlanner::new(&engine, &set, setting).map_err(plan_error)?;
    let (sender, events) = channel();
    let cancel = stop.shared();
    let worker = Arc::clone(&engine);
    let scheduler = thread::spawn(move || {
        schedule(
            &worker,
            &cancel,
            &|work: &GroupWork| {
                let answered = worker.answer_group_batch(work, &cancel)?;
                if answered.value.len() > work.rows.len() {
                    return Err(Error::defect("an annotate request lost a row"));
                }
                let value = work
                    .rows
                    .iter()
                    .copied()
                    .zip(answered.value)
                    .map(|(row, answer)| Fragment {
                        row,
                        group: work.group,
                        answer,
                    })
                    .collect();
                Ok(Completed {
                    value,
                    records: 0,
                    replayed: 0,
                    partial_failure: false,
                    stop: answered.stop,
                })
            },
            &sender,
        );
    });
    Ok(Batch {
        source: Box::new(Stream {
            items: records,
            held: BTreeMap::new(),
            rows: BTreeMap::new(),
            ready: VecDeque::new(),
            planner,
            set,
            engine,
            events,
            port: None,
            stop,
            facts: None,
            fed: 0,
            completed: 0,
            most,
            refusal: None,
            exhausted: false,
            pending_asks: 0,
            scheduler: Some(scheduler),
        }),
    })
}

impl<I> Source<AnnotatedRecord<I::Item>> for Stream<'_, I>
where
    I: Iterator,
    I::Item: Evidence,
{
    fn pull(&mut self) -> Option<Result<AnnotatedRecord<I::Item>, Error>> {
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
                    Event::End(Err(Error::defect("the annotation scheduler ended early")))
                }
            };
            self.take(event);
        }
    }

    fn facts(&self) -> Option<&Facts> {
        self.facts.as_ref()
    }
}

impl<I> Stream<'_, I>
where
    I: Iterator,
    I::Item: Evidence,
{
    fn take(&mut self, event: Event<GroupWork, Vec<Fragment>>) {
        match event {
            Event::Port(port) => self.port = Some(port),
            Event::Ask => self.pending_asks += 1,
            Event::Row(fragments) => self.take_fragments(fragments),
            Event::End(ended) => {
                let ended = self.join().and(ended);
                let ended = self.stop.finish(ended);
                let facts = self.stop.facts();
                self.facts = Some(facts.clone());
                if let Err(error) = ended {
                    self.ready.push_back(Err(error.with_facts(facts)));
                }
            }
        }
        self.feed();
    }

    fn feed(&mut self) {
        while self.pending_asks > 0 && self.scheduler.is_some() {
            while !self.planner.ready()
                && self.refusal.is_none()
                && !self.exhausted
                && self.fed.saturating_sub(self.completed) < 4_096
            {
                self.advance();
            }
            let input = if let Some(work) = self.planner.pop() {
                Input::Item(work)
            } else if let Some(error) = self.refusal.take() {
                Input::Failed(error)
            } else if self.exhausted {
                Input::End
            } else {
                return;
            };
            if let Some(port) = &self.port {
                let _sent = port.send(input);
            }
            self.pending_asks -= 1;
        }
    }

    fn advance(&mut self) {
        match self.items.next() {
            None => {
                self.exhausted = true;
                if let Err(error) = self.planner.finish() {
                    self.refusal = Some(plan_error(error));
                }
            }
            Some(_) if self.most.is_some_and(|most| self.fed >= most) => {
                self.finish_before_refusal();
                self.refusal = Some(Error::usage(format!(
                    "this engine answers at most {} records in one call",
                    self.fed
                )));
            }
            Some(item) => {
                let prepared = annotation_record(&self.set, item.evidence());
                let pushed = prepared.and_then(|record| {
                    self.planner
                        .push(&self.set, &record, self.fed)
                        .map_err(plan_error)
                });
                match pushed {
                    Ok(()) => {
                        self.held.insert(self.fed, item);
                        self.rows.insert(
                            self.fed,
                            RowState {
                                groups: (0..self.planner.groups()).map(|_| None).collect(),
                            },
                        );
                        self.fed += 1;
                    }
                    Err(error) => {
                        self.finish_before_refusal();
                        self.refusal = Some(error);
                    }
                }
            }
        }
    }

    fn finish_before_refusal(&mut self) {
        if let Err(error) = self.planner.finish() {
            self.refusal = Some(plan_error(error));
        }
    }

    fn take_fragments(&mut self, fragments: Vec<Fragment>) {
        for fragment in fragments {
            let Some(row) = self.rows.get_mut(&fragment.row) else {
                let error = self.end(Error::defect("an annotate fragment names no input row"));
                self.ready.push_back(Err(error));
                return;
            };
            let Some(slot) = row.groups.get_mut(fragment.group) else {
                let error = self.end(Error::defect("an annotate fragment names no group"));
                self.ready.push_back(Err(error));
                return;
            };
            *slot = Some(fragment.answer);
        }
        while self
            .rows
            .get(&self.completed)
            .is_some_and(|row| row.groups.iter().all(Option::is_some))
        {
            if let Err(error) = self.assemble_next() {
                let error = self.end(error);
                self.ready.push_back(Err(error));
                return;
            }
        }
    }

    fn assemble_next(&mut self) -> Result<(), Error> {
        let row = self
            .rows
            .remove(&self.completed)
            .ok_or_else(|| Error::defect("an annotation row disappeared"))?;
        let fragments = row
            .groups
            .into_iter()
            .map(|group| group.ok_or_else(|| Error::defect("an annotation group disappeared")))
            .collect::<Result<Vec<_>, _>>()?;
        let annotation = facade::assemble(&self.set, fragments, self.engine.backend().model())
            .map_err(Error::from)?;
        let all_failed = annotation
            .values
            .iter()
            .all(|(_, value)| matches!(value, AnnotatedValue::Failed(_)));
        let value =
            render_annotation(&self.set, &self.engine, annotation, self.stop.observing())?.value;
        if all_failed {
            observe_annotated_questions(&value, self.completed, &self.stop)?;
            return Err(Error::of(
                ErrorKind::Backend,
                "a backend question failed in a batch",
            ));
        }
        observe_annotated(&value, self.completed, &self.stop)?;
        let item = self
            .held
            .remove(&self.completed)
            .ok_or_else(|| Error::defect("an annotation input disappeared"))?;
        self.stop.shared().finished_records(1);
        self.ready
            .push_back(Ok(AnnotatedRecord::new(item, value.values, value.json)));
        self.completed += 1;
        Ok(())
    }

    fn end(&mut self, error: Error) -> Error {
        let _joined = self.join();
        let error = match self.stop.finish::<()>(Err(error)) {
            Err(error) => error,
            Ok(()) => Error::defect("a failed annotation stream returned a value"),
        };
        let facts = self.stop.facts();
        self.facts = Some(facts.clone());
        error.with_facts(facts)
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

impl<I: Iterator> Drop for Stream<'_, I> {
    fn drop(&mut self) {
        let _joined = join_scheduler(
            &mut self.scheduler,
            &self.stop,
            &self.events,
            &mut self.port,
        );
    }
}
