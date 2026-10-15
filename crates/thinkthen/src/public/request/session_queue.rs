//! One input cell, one output cell and independent terminal cells.
use super::{
    RequestReaderFailure, RequestSessionDescriptor, RequestSessionPush, RequestSessionPushStatus,
    RequestSessionRead, RequestSessionResult, RequestSessionTerminal,
};
use crate::{CallOptions, CancelToken, Error};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

#[derive(Debug)]
enum Finish {
    Transport(Option<RequestReaderFailure>),
    Native(Option<Error>),
}

#[derive(Debug, Default)]
struct State {
    input: Option<RequestSessionDescriptor>,
    finish: Option<Finish>,
    intake_closed: bool,
    output: Option<RequestSessionResult>,
    terminal: Option<RequestSessionTerminal>,
    settled: bool,
    receiver_closed: bool,
    settlement_interrupted: bool,
}

/// A separate generation prevents lost wakeups across the independent intake and output locks.
#[derive(Debug, Default)]
pub(super) struct Change {
    generation: Mutex<u64>,
    wake: Condvar,
}
impl Change {
    pub(super) fn observed(&self) -> u64 {
        *self
            .generation
            .lock()
            .unwrap_or_else(|error| error.into_inner())
    }
    pub(super) fn notify(&self) {
        let mut generation = self
            .generation
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        *generation = generation.wrapping_add(1);
        self.wake.notify_all();
    }
    pub(super) fn wait(&self, observed: u64, timeout: std::time::Duration) {
        let generation = self
            .generation
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if *generation == observed {
            drop(
                self.wake
                    .wait_timeout(generation, timeout)
                    .unwrap_or_else(|error| error.into_inner()),
            );
        }
    }
}

#[derive(Debug, Default)]
pub(super) struct Queue {
    state: Mutex<State>,
    wake: Condvar,
    changed: Option<Arc<Change>>,
    pub(super) cancel: CancelToken,
}

impl Queue {
    pub(super) fn recovering(changed: Arc<Change>) -> Self {
        Self {
            changed: Some(changed),
            ..Self::default()
        }
    }
    fn notify(&self) {
        self.wake.notify_all();
        if let Some(changed) = &self.changed {
            changed.notify();
        }
    }
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|error| error.into_inner())
    }
    fn wait<'a>(&self, state: MutexGuard<'a, State>) -> MutexGuard<'a, State> {
        self.wake
            .wait(state)
            .unwrap_or_else(|error| error.into_inner())
    }
    pub(super) fn ready(
        &self,
        interest: super::RequestSessionWaitInterest,
        timeout: std::time::Duration,
        feed: bool,
    ) -> super::RequestSessionReady {
        use super::{RequestSessionReady as Ready, RequestSessionWaitInterest as Interest};
        let started = std::time::Instant::now();
        let mut state = self.lock();
        if interest == Interest::SettlementArm {
            state.settlement_interrupted = false;
        }
        loop {
            if state.output.is_some() || state.terminal.is_some() {
                return Ready::Result;
            }
            if state.settled || state.receiver_closed {
                return Ready::End;
            }
            if (interest == Interest::InterruptibleSettlement && state.settlement_interrupted)
                || (matches!(interest, Interest::Result | Interest::Input)
                    && self.cancel.is_cancelled())
            {
                return Ready::Cancelled;
            }
            if interest == Interest::SettlementArm {
                return Ready::Timeout;
            }
            if interest == Interest::Input
                && feed
                && (state.intake_closed || state.finish.is_some())
            {
                return Ready::Closed;
            }
            if interest == Interest::Input && feed && state.input.is_none() {
                return Ready::Input;
            }
            let Some(remaining) = timeout.checked_sub(started.elapsed()) else {
                return Ready::Timeout;
            };
            if remaining.is_zero() {
                return Ready::Timeout;
            }
            state = self
                .wake
                .wait_timeout(state, remaining)
                .unwrap_or_else(|error| error.into_inner())
                .0;
        }
    }
    pub(super) fn capacity(&self) -> RequestSessionPushStatus {
        let state = self.lock();
        if state.intake_closed || state.finish.is_some() {
            RequestSessionPushStatus::Closed
        } else if state.input.is_some() {
            RequestSessionPushStatus::Full
        } else {
            RequestSessionPushStatus::Accepted
        }
    }
    pub(super) fn input_room(&self) -> bool {
        let mut state = self.lock();
        while state.input.is_some() && !state.intake_closed && state.finish.is_none() {
            state = self.wait(state);
        }
        !state.intake_closed && state.finish.is_none()
    }
    pub(super) fn push_wait(
        &self,
        descriptor: RequestSessionDescriptor,
    ) -> RequestSessionPushStatus {
        let mut state = self.lock();
        while state.input.is_some() && !state.intake_closed && state.finish.is_none() {
            state = self.wait(state);
        }
        if state.intake_closed || state.finish.is_some() {
            RequestSessionPushStatus::Closed
        } else {
            state.input = Some(descriptor);
            self.notify();
            RequestSessionPushStatus::Accepted
        }
    }
    pub(super) fn push(&self, descriptor: RequestSessionDescriptor) -> RequestSessionPush {
        let mut state = self.lock();
        if state.intake_closed || state.finish.is_some() {
            RequestSessionPush::Closed(descriptor)
        } else if state.input.is_some() {
            RequestSessionPush::Full(descriptor)
        } else {
            state.input = Some(descriptor);
            self.notify();
            RequestSessionPush::Accepted
        }
    }
    pub(super) fn finish(&self, failure: Option<RequestReaderFailure>) -> Result<(), Error> {
        self.finish_input(Finish::Transport(failure))
    }
    pub(super) fn finish_native(&self, error: Error) -> Result<(), Error> {
        self.finish_input(Finish::Native(Some(error)))
    }
    fn finish_input(&self, finish: Finish) -> Result<(), Error> {
        let mut state = self.lock();
        if let Some(previous) = &state.finish {
            if !matches!((previous, &finish), (Finish::Transport(previous), Finish::Transport(next)) if previous == next)
            {
                return Err(Error::usage(
                    "session input was already finished differently",
                ));
            }
        } else {
            state.finish = Some(finish);
        }
        self.notify();
        Ok(())
    }
    pub(super) fn next(
        &self,
        controls: CallOptions<'_>,
    ) -> Option<Result<RequestSessionDescriptor, Error>> {
        let mut state = self.lock();
        loop {
            if state.intake_closed || self.cancel.is_cancelled() {
                return None;
            }
            if let Err(error) = controls.reader_admission() {
                state.intake_closed = true;
                self.notify();
                return Some(Err(error));
            }
            if let Some(item) = state.input.take() {
                self.notify();
                return Some(Ok(item));
            }
            if let Some(failure) = &mut state.finish {
                let error = match failure {
                    Finish::Transport(failure) => failure.as_ref().map(RequestReaderFailure::error),
                    Finish::Native(error) => error.take(),
                };
                state.intake_closed = true;
                self.notify();
                return error.map(Err);
            }
            state = match controls.reader_remaining() {
                Ok(Some(remaining)) => {
                    self.wake
                        .wait_timeout(state, remaining)
                        .unwrap_or_else(|error| error.into_inner())
                        .0
                }
                Ok(None) => self.wait(state),
                Err(error) => {
                    state.intake_closed = true;
                    self.notify();
                    return Some(Err(error));
                }
            };
        }
    }
    pub(super) fn publish(&self, packet: RequestSessionResult) {
        let mut state = self.lock();
        while state.output.is_some() && !state.receiver_closed {
            state = self.wait(state);
        }
        if !state.receiver_closed {
            state.output = Some(packet);
            self.notify();
        }
    }
    pub(super) fn settle(&self, terminal: RequestSessionTerminal) {
        let mut state = self.lock();
        state.intake_closed = true;
        state.input = None;
        state.terminal = Some(terminal);
        state.settled = true;
        self.notify();
    }
    pub(super) fn read(&self) -> RequestSessionRead {
        self.read_locked(&mut self.lock())
    }
    pub(super) fn next_packet(
        &self,
        interest: super::RequestSessionWaitInterest,
    ) -> Result<RequestSessionRead, crate::Error> {
        use super::RequestSessionWaitInterest as Interest;
        if !matches!(interest, Interest::Result | Interest::Settlement) {
            return Err(crate::Error::usage(
                "packet reads require result or settlement interest",
            ));
        }
        let mut state = self.lock();
        loop {
            // Consume queued output before observing cancellation.
            let packet = self.read_locked(&mut state);
            if !matches!(packet, RequestSessionRead::Pending) {
                return Ok(packet);
            }
            if interest != Interest::Settlement && self.cancel.is_cancelled() {
                return Err(crate::Error::cancelled());
            }
            state = self.wait(state);
        }
    }
    fn read_locked(&self, state: &mut State) -> RequestSessionRead {
        if let Some(packet) = state.output.take() {
            self.notify();
            RequestSessionRead::Result(packet)
        } else if let Some(terminal) = state.terminal.take() {
            RequestSessionRead::Result(RequestSessionResult::Terminal(terminal))
        } else if state.settled || state.receiver_closed {
            RequestSessionRead::End
        } else {
            RequestSessionRead::Pending
        }
    }
    pub(super) fn close_intake(&self) {
        let mut state = self.lock();
        state.intake_closed = true;
        state.input = None;
        self.notify();
    }
    pub(super) fn stop(&self, drop_receiver: bool) {
        let mut state = self.lock();
        self.cancel.cancel();
        state.settlement_interrupted = true;
        state.intake_closed = true;
        state.input = None;
        if drop_receiver {
            state.receiver_closed = true;
            state.output = None;
            state.terminal = None;
        }
        self.notify();
    }
}
