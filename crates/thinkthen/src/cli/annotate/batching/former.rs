//! Admit rows to the shared group planner and return its closed requests.

use std::sync::Arc;
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use crate::annotate_schedule::Input as AnnotateInput;
use crate::core::{PartError, QuestionSet, Reading, Setting};
use crate::engine::Cancel;
use crate::engine::facade::{Engine, GroupPlanError, GroupPlanner, GroupWork, Input, InputPort};
use crate::failure::Failure;
use crate::schedule::Placed;

use super::Shared;

pub(super) struct Former {
    engine: Engine,
    set: QuestionSet,
    reading: Reading,
    planner: GroupPlanner,
    shared: Arc<Shared>,
    cancel: Cancel<'static>,
    fed: usize,
    exhausted: bool,
    refusal: Option<Placed>,
}

impl Former {
    pub(super) fn new(
        judging: &super::super::Judging<'_>,
        reading: &Reading,
        setting: Setting,
        shared: Arc<Shared>,
    ) -> Result<Self, Failure> {
        let engine = judging.engine().clone();
        let planner = GroupPlanner::new(&engine, judging.set(), setting)
            .map_err(|error| plan_failure(&engine, error))?;
        Ok(Self {
            engine,
            set: judging.set().clone(),
            reading: reading.clone(),
            planner,
            shared,
            cancel: judging.cancel().clone(),
            fed: 0,
            exhausted: false,
            refusal: None,
        })
    }

    pub(super) fn answer(
        mut self,
        raw: &Receiver<Result<AnnotateInput, Placed>>,
        asks: &Receiver<()>,
        events: &InputPort<GroupWork, Vec<super::Fragment>, Placed>,
    ) {
        while asks.recv().is_ok() {
            let event = self.next(raw);
            let stop = matches!(event, Input::End | Input::Failed(_));
            if events.send(event).is_err() || stop {
                return;
            }
        }
    }

    #[expect(
        clippy::excessive_nesting,
        reason = "the feeder handles one input or close event within its frontier loop"
    )]
    fn next(&mut self, raw: &Receiver<Result<AnnotateInput, Placed>>) -> Input<GroupWork, Placed> {
        loop {
            let oldest = self
                .shared
                .completed
                .load(std::sync::atomic::Ordering::Acquire);
            if let Some(work) = self.planner.pop_sliced_at(oldest) {
                return Input::Item(work);
            }
            if let Some(error) = self.refusal.take() {
                if !self.planner.ready() {
                    return Input::Failed(error);
                }
                self.refusal = Some(error);
            }
            if self.exhausted {
                if !self.planner.ready() {
                    return Input::End;
                }
                thread::sleep(Duration::from_millis(1));
                continue;
            }
            if let Some(error) = self.cancel.stop() {
                return Input::Failed(error.into());
            }
            // The feeder can never outrun the oldest printed row by more than
            // the ordinary record scheduler's bounded admission window.
            if self.fed.saturating_sub(oldest) >= 4096 {
                thread::sleep(Duration::from_millis(1));
                continue;
            }
            match raw.recv_timeout(Duration::from_millis(50)) {
                Ok(Ok(input)) => {
                    let at = match &input {
                        AnnotateInput::Bytes(at, _) | AnnotateInput::Record(at, _) => *at,
                    };
                    if let Err(error) = self.push(input) {
                        self.refuse(Placed::at(error, at));
                    }
                }
                Ok(Err(error)) => self.refuse(error),
                Err(RecvTimeoutError::Disconnected) => self.close(),
                Err(RecvTimeoutError::Timeout) => {
                    if let Err(error) = self.planner.pause() {
                        self.refuse(Placed::from(plan_failure(&self.engine, error)));
                    }
                }
            }
        }
    }

    fn push(&mut self, input: AnnotateInput) -> Result<(), Failure> {
        let record = match input {
            AnnotateInput::Bytes(_, bytes) => self
                .reading
                .annotation_record(&bytes)
                .map_err(|error| Failure::record(error, self.reading.streams()))?,
            AnnotateInput::Record(_, record) => record,
        };
        super::super::collisions(&self.set, &record)?;
        let batch = self.reading.batch_record(&record)?;
        let groups = self
            .planner
            .push_sliced(&self.engine, &self.set, &batch, self.fed)
            .map_err(|error| plan_failure(&self.engine, error))?;
        self.shared.insert(self.fed, record, groups)?;
        self.fed += 1;
        Ok(())
    }

    fn refuse(&mut self, error: Placed) {
        self.refusal = Some(error);
        self.close();
    }

    fn close(&mut self) {
        self.exhausted = true;
        if let Err(error) = self.planner.finish() {
            self.refusal = Some(Placed::from(plan_failure(&self.engine, error)));
        }
    }
}

fn plan_failure(engine: &Engine, error: GroupPlanError) -> Failure {
    match error {
        GroupPlanError::Part(PartError::Reading(error)) => error.into(),
        GroupPlanError::Part(PartError::Record(error)) => error.into(),
        GroupPlanError::Batch(error) => {
            crate::failure::context::Limits::new(engine.profile()).refused(error, false)
        }
        GroupPlanError::Defect(message) => Failure::Defect(message),
        GroupPlanError::Engine(error) => error.into(),
    }
}
