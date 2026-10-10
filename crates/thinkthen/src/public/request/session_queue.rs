//! One input cell, one output cell and independent terminal cells.
use super::{
    RequestReaderFailure, RequestSessionDescriptor, RequestSessionPush, RequestSessionPushStatus,
    RequestSessionRead, RequestSessionResult, RequestSessionTerminal,
};
use crate::{CallOptions, CancelToken, Error};
use std::sync::{Condvar, Mutex, MutexGuard};

#[derive(Debug, Default)]
struct State {
    input: Option<RequestSessionDescriptor>,
    finish: Option<Option<RequestReaderFailure>>,
    intake_closed: bool,
    output: Option<RequestSessionResult>,
    terminal: Option<RequestSessionTerminal>,
    settled: bool,
    receiver_closed: bool,
}

#[derive(Debug, Default)]
pub(super) struct Queue {
    state: Mutex<State>,
    wake: Condvar,
    pub(super) cancel: CancelToken,
}

impl Queue {
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|error| error.into_inner())
    }
    fn wait<'a>(&self, state: MutexGuard<'a, State>) -> MutexGuard<'a, State> {
        self.wake
            .wait(state)
            .unwrap_or_else(|error| error.into_inner())
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
    pub(super) fn push(&self, descriptor: RequestSessionDescriptor) -> RequestSessionPush {
        let mut state = self.lock();
        if state.intake_closed || state.finish.is_some() {
            RequestSessionPush::Closed(descriptor)
        } else if state.input.is_some() {
            RequestSessionPush::Full(descriptor)
        } else {
            state.input = Some(descriptor);
            self.wake.notify_all();
            RequestSessionPush::Accepted
        }
    }
    pub(super) fn finish(&self, failure: Option<RequestReaderFailure>) -> Result<(), Error> {
        let mut state = self.lock();
        if let Some(previous) = &state.finish {
            if previous != &failure {
                return Err(Error::usage(
                    "session input was already finished differently",
                ));
            }
        } else {
            state.finish = Some(failure);
        }
        self.wake.notify_all();
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
                return Some(Err(error));
            }
            if let Some(item) = state.input.take() {
                self.wake.notify_all();
                return Some(Ok(item));
            }
            if let Some(failure) = &state.finish {
                let error = failure.as_ref().map(RequestReaderFailure::error);
                state.intake_closed = true;
                return error.map(Err);
            }
            // An idle producer must not hide the native request deadline.
            state = self
                .wake
                .wait_timeout(state, std::time::Duration::from_millis(10))
                .unwrap_or_else(|error| error.into_inner())
                .0;
        }
    }
    pub(super) fn publish(&self, packet: RequestSessionResult) {
        let mut state = self.lock();
        while state.output.is_some() && !state.receiver_closed {
            state = self.wait(state);
        }
        if !state.receiver_closed {
            state.output = Some(packet);
            self.wake.notify_all();
        }
    }
    pub(super) fn settle(&self, terminal: RequestSessionTerminal) {
        let mut state = self.lock();
        state.intake_closed = true;
        state.input = None;
        state.terminal = Some(terminal);
        state.settled = true;
        self.wake.notify_all();
    }
    pub(super) fn read(&self) -> RequestSessionRead {
        let mut state = self.lock();
        if let Some(packet) = state.output.take() {
            self.wake.notify_all();
            RequestSessionRead::Result(packet)
        } else if let Some(terminal) = state.terminal.take() {
            RequestSessionRead::Result(RequestSessionResult::Terminal(terminal))
        } else if state.settled {
            RequestSessionRead::End
        } else {
            RequestSessionRead::Pending
        }
    }
    pub(super) fn close_intake(&self) {
        let mut state = self.lock();
        state.intake_closed = true;
        state.input = None;
        self.wake.notify_all();
    }
    pub(super) fn stop(&self, drop_receiver: bool) {
        let mut state = self.lock();
        self.cancel.cancel();
        state.intake_closed = true;
        state.input = None;
        if drop_receiver {
            state.receiver_closed = true;
            state.output = None;
            state.terminal = None;
        }
        self.wake.notify_all();
    }
}
