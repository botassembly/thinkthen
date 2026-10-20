//! One lock owns bounded intake, ordered output, cancellation and settlement.
use super::{
    RequestReaderFailure, RequestSessionDescriptor, RequestSessionPush, RequestSessionPushStatus,
    RequestSessionResult, RequestSessionTerminal,
};
use crate::{CallOptions, CancelToken, Error};
use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex, MutexGuard},
};

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
    output: Option<Packet>,
    parked: VecDeque<Packet>,
    terminal: Option<RequestSessionTerminal>,
    settled: bool,
    receiver_closed: bool,
    settlement_interrupted: bool,
}

/// Execution mode chooses packet contents; synchronization remains shared.
#[derive(Debug)]
#[expect(
    clippy::large_enum_variant,
    reason = "both execution modes move the existing owned packets without another allocation"
)]
pub(super) enum Packet {
    Ordinary(RequestSessionResult),
    Recovering(super::RequestRecoveringResult),
}
#[derive(Debug)]
#[expect(
    clippy::large_enum_variant,
    reason = "queue reads transfer the owned packet directly into its public typed view"
)]
pub(super) enum Read {
    Packet(Packet),
    Terminal(RequestSessionTerminal),
    Pending,
    End,
}
impl From<RequestSessionResult> for Packet {
    fn from(value: RequestSessionResult) -> Self {
        Self::Ordinary(value)
    }
}
impl From<super::RequestRecoveringResult> for Packet {
    fn from(value: super::RequestRecoveringResult) -> Self {
        Self::Recovering(value)
    }
}

#[derive(Debug, Default)]
pub(super) struct Queue {
    state: Mutex<State>,
    wake: Condvar,
    pub(super) cancel: CancelToken,
    pub(super) logical: Mutex<Option<super::session_input::Logical>>,
    input_wake: Mutex<Option<crate::public::readiness::Wake>>,
}

impl Queue {
    fn notify(&self) {
        self.wake.notify_all();
    }
    pub(super) fn register_input_wake(&self, wake: Option<crate::public::readiness::Wake>) {
        let mut previous = self
            .input_wake
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(previous) = previous.take() {
            previous.clear();
        }
        *previous = wake;
    }
    fn notify_input(&self) {
        let wake = self
            .input_wake
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        if let Some(wake) = wake {
            wake.notify();
        }
    }
    pub(super) fn reschedule_input(&self) {
        let ready = {
            let state = self.lock();
            state.input.is_some()
                || state.finish.is_some()
                || state.intake_closed
                || self.cancel.is_cancelled()
        };
        if ready {
            self.notify_input();
        }
    }
    pub(super) fn try_input(&self, controls: CallOptions<'_>) -> super::session_input::Read {
        let mut state = self.lock();
        let value = self.take_input(&mut state, controls);
        self.notify();
        value
    }
    fn take_input(
        &self,
        state: &mut State,
        controls: CallOptions<'_>,
    ) -> super::session_input::Read {
        use super::session_input::Read;
        if state.intake_closed || self.cancel.is_cancelled() {
            return Read::End;
        }
        if let Err(error) = controls.reader_admission() {
            state.intake_closed = true;
            return Read::Item(Box::new(Err(error)));
        }
        if let Some(item) = state.input.take() {
            return Read::Item(Box::new(Ok(item)));
        }
        if let Some(failure) = &mut state.finish {
            let error = match failure {
                Finish::Transport(failure) => failure.as_ref().map(RequestReaderFailure::error),
                Finish::Native(error) => error.take(),
            };
            state.intake_closed = true;
            return error.map_or(Read::End, |error| Read::Item(Box::new(Err(error))));
        }
        Read::Pending
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
            if !state.parked.is_empty() || state.output.is_some() || state.terminal.is_some() {
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
        let mut state = self.lock();
        if state.input.is_some() {
            self.park(&mut state);
        }
        if state.intake_closed || state.finish.is_some() {
            RequestSessionPushStatus::Closed
        } else if state.input.is_some() {
            RequestSessionPushStatus::Full
        } else {
            RequestSessionPushStatus::Accepted
        }
    }
    fn park(&self, state: &mut State) {
        if let Some(packet) = state.output.take() {
            state.parked.push_back(packet);
            self.notify();
        }
    }
    fn wait_for_room(&self) -> MutexGuard<'_, State> {
        let mut state = self.lock();
        while state.input.is_some() && !state.intake_closed && state.finish.is_none() {
            self.park(&mut state);
            // Publication, intake and this check share one lock. Condvar::wait
            // atomically releases it, so neither queue can lose a notification.
            state = self.wait(state);
        }
        state
    }
    pub(super) fn input_room(&self) -> bool {
        let state = self.wait_for_room();
        !state.intake_closed && state.finish.is_none()
    }
    pub(super) fn push_wait(
        &self,
        descriptor: RequestSessionDescriptor,
    ) -> RequestSessionPushStatus {
        let mut state = self.wait_for_room();
        if state.intake_closed || state.finish.is_some() {
            RequestSessionPushStatus::Closed
        } else {
            state.input = Some(descriptor);
            self.notify();
            drop(state);
            self.notify_input();
            RequestSessionPushStatus::Accepted
        }
    }
    pub(super) fn push(&self, descriptor: RequestSessionDescriptor) -> RequestSessionPush {
        let mut state = self.lock();
        if state.input.is_some() {
            self.park(&mut state);
        }
        if state.intake_closed || state.finish.is_some() {
            RequestSessionPush::Closed(descriptor)
        } else if state.input.is_some() {
            RequestSessionPush::Full(descriptor)
        } else {
            state.input = Some(descriptor);
            self.notify();
            drop(state);
            self.notify_input();
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
        drop(state);
        self.notify_input();
        Ok(())
    }
    pub(super) fn next(
        &self,
        controls: CallOptions<'_>,
    ) -> Option<Result<RequestSessionDescriptor, Error>> {
        let mut state = self.lock();
        loop {
            match self.take_input(&mut state, controls) {
                super::session_input::Read::Item(item) => {
                    self.notify();
                    return Some(*item);
                }
                super::session_input::Read::End => {
                    self.notify();
                    return None;
                }
                super::session_input::Read::Pending => {}
            }
            state = match controls.reader_remaining() {
                Ok(Some(remaining)) => {
                    self.wake
                        .wait_timeout(state, remaining)
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
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
    pub(super) fn publish(&self, packet: impl Into<Packet>) {
        let mut state = self.lock();
        while state.output.is_some() && !state.receiver_closed {
            state = self.wait(state);
        }
        if !state.receiver_closed {
            state.output = Some(packet.into());
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
        drop(state);
        self.logical
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
        self.notify_input();
    }
    pub(super) fn read(&self) -> Read {
        self.read_locked(&mut self.lock())
    }
    pub(super) fn next_packet(
        &self,
        interest: super::RequestSessionWaitInterest,
    ) -> Result<Read, crate::Error> {
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
            if !matches!(packet, Read::Pending) {
                return Ok(packet);
            }
            if interest != Interest::Settlement && self.cancel.is_cancelled() {
                return Err(crate::Error::cancelled());
            }
            state = self.wait(state);
        }
    }
    fn read_locked(&self, state: &mut State) -> Read {
        if let Some(packet) = state.parked.pop_front() {
            Read::Packet(packet)
        } else if let Some(packet) = state.output.take() {
            self.notify();
            Read::Packet(packet)
        } else if let Some(terminal) = state.terminal.take() {
            Read::Terminal(terminal)
        } else if state.settled || state.receiver_closed {
            Read::End
        } else {
            Read::Pending
        }
    }
    pub(super) fn close_intake(&self) {
        let mut state = self.lock();
        state.intake_closed = true;
        state.input = None;
        self.notify();
        drop(state);
        self.notify_input();
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
            state.parked.clear();
            state.terminal = None;
        }
        self.notify();
        drop(state);
        self.notify_input();
    }
}

#[cfg(test)]
mod input_tests;
