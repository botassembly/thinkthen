//! Opt-in recovered rows keep ordinary packets and source ownership independent.
use super::{
    AdmittedRequest, Request, RequestEnvironment, RequestFeed, RequestFunction, RequestInput,
    RequestRecoveredRow, RequestSession, RequestSessionFeedOptions, RequestSessionPushStatus,
    RequestSessionTerminal, RequestSessionWaitInterest,
};
use crate::{Call, CallOptions, Engine, Error, OwnedRecordObservation, Surface};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

/// A recovered native row, observation, or joined invocation terminal.
#[derive(Debug)]
#[expect(
    clippy::large_enum_variant,
    reason = "owned recovered packets follow the existing independent native packet API"
)]
pub enum RequestRecoveringResult {
    /// Original occurrence and its recoverable result.
    Row(RequestRecoveredRow),
    /// Actual native observation.
    Observation {
        /// Declared function.
        function: RequestFunction,
        /// Owned observer snapshot.
        value: OwnedRecordObservation,
    },
    /// Final facts after native workers join.
    Terminal(RequestSessionTerminal),
}
/// A nonblocking typed recovered output read.
#[derive(Debug)]
#[expect(
    clippy::large_enum_variant,
    reason = "the nonblocking read transfers one independent native packet directly"
)]
pub enum RequestRecoveringRead {
    /// One independent owned output.
    Result(RequestRecoveringResult),
    /// Native work has not settled.
    Pending,
    /// Final output was consumed.
    End,
}
#[derive(Debug, Default)]
struct OutputState {
    packet: Option<RequestRecoveringResult>,
    terminal: Option<RequestSessionTerminal>,
    settled: bool,
    closed: bool,
}
#[derive(Debug, Default)]
struct Output {
    state: Mutex<OutputState>,
    wake: Condvar,
    changed: Arc<super::session_queue::Change>,
}
impl Output {
    fn lock(&self) -> MutexGuard<'_, OutputState> {
        self.state.lock().unwrap_or_else(|error| error.into_inner())
    }
    fn publish(&self, packet: RequestRecoveringResult) {
        let mut state = self.lock();
        while state.packet.is_some() && !state.closed {
            state = self
                .wake
                .wait(state)
                .unwrap_or_else(|error| error.into_inner());
        }
        if !state.closed {
            state.packet = Some(packet);
            self.wake.notify_all();
            self.changed.notify();
        }
    }
    fn settle(&self, terminal: RequestSessionTerminal) {
        let mut state = self.lock();
        state.terminal = Some(terminal);
        state.settled = true;
        self.wake.notify_all();
        self.changed.notify();
    }
    fn read(&self, state: &mut OutputState) -> RequestRecoveringRead {
        if let Some(packet) = state.packet.take() {
            self.wake.notify_all();
            self.changed.notify();
            RequestRecoveringRead::Result(packet)
        } else if let Some(terminal) = state.terminal.take() {
            RequestRecoveringRead::Result(RequestRecoveringResult::Terminal(terminal))
        } else if state.settled || state.closed {
            RequestRecoveringRead::End
        } else {
            RequestRecoveringRead::Pending
        }
    }
}
/// One bounded recovering request with owned descriptor intake.
#[derive(Debug)]
pub struct RequestRecoveringSession {
    input: RequestSession,
    output: Arc<Output>,
}
impl RequestRecoveringSession {
    /// Admit an owned descriptor only while intake has room.
    /// # Errors
    /// Returns the canonical descriptor refusal.
    pub fn try_push_json(&self, text: &str) -> Result<RequestSessionPushStatus, Error> {
        self.input.try_push_json(text)
    }
    /// Transfer a descriptor without waiting for intake space.
    /// # Errors
    /// Refuses descriptors on a request without a feed.
    pub fn try_push(
        &self,
        descriptor: super::RequestSessionDescriptor,
    ) -> Result<super::RequestSessionPush, Error> {
        self.input.try_push(descriptor)
    }
    /// Convert a descriptor only when intake has room.
    /// # Errors
    /// Returns the canonical feed or conversion refusal.
    pub fn try_push_with<E: From<Error>>(
        &self,
        convert: impl FnOnce() -> Result<super::RequestSessionDescriptor, E>,
    ) -> Result<RequestSessionPushStatus, E> {
        self.input.try_push_with(convert)
    }
    /// Convert and transfer a descriptor once intake has room.
    /// # Errors
    /// Returns the canonical feed or conversion refusal.
    pub fn push_with<E: From<Error>>(
        &self,
        convert: impl FnOnce() -> Result<super::RequestSessionDescriptor, E>,
    ) -> Result<RequestSessionPushStatus, E> {
        self.input.push_with(convert)
    }
    /// Finish the reader independently of occupied descriptor intake.
    /// # Errors
    /// Refuses changed terminal controls.
    pub fn finish(&self, failure: Option<super::RequestReaderFailure>) -> Result<(), Error> {
        self.input.finish(failure)
    }
    /// Retain a typed reader diagnostic as a terminal failure.
    /// # Errors
    /// Refuses unsupported native reader diagnostics.
    pub fn finish_native_reader_error(&self, error: Error) -> Result<(), Error> {
        self.input.finish_native_reader_error(error)
    }
    /// Read a recovered output without blocking.
    #[must_use]
    pub fn try_read(&self) -> RequestRecoveringRead {
        self.output.read(&mut self.output.lock())
    }
    /// Wait for the next recovered output or joined terminal.
    /// # Errors
    /// Refuses unsupported interests; Result observes cancellation promptly.
    pub fn next_packet(
        &self,
        interest: RequestSessionWaitInterest,
    ) -> Result<RequestRecoveringRead, Error> {
        if !matches!(
            interest,
            RequestSessionWaitInterest::Result | RequestSessionWaitInterest::Settlement
        ) {
            return Err(Error::usage(
                "packet reads require result or settlement interest",
            ));
        }
        let mut state = self.output.lock();
        loop {
            let read = self.output.read(&mut state);
            if !matches!(read, RequestRecoveringRead::Pending) {
                return Ok(read);
            }
            if interest == RequestSessionWaitInterest::Result
                && self.input.queue.cancel.is_cancelled()
            {
                return Err(Error::cancelled());
            }
            state = self
                .output
                .wake
                .wait(state)
                .unwrap_or_else(|error| error.into_inner());
        }
    }
    /// Wait for recovered output, descriptor room, or caller cancellation.
    /// Readiness transfers no descriptor and consumes no recovered output.
    #[must_use]
    pub fn wait(
        &self,
        interest: RequestSessionWaitInterest,
        timeout: std::time::Duration,
    ) -> super::RequestSessionReady {
        use super::RequestSessionReady as Ready;
        if interest == RequestSessionWaitInterest::SettlementArm {
            let _armed =
                self.input
                    .queue
                    .ready(interest, std::time::Duration::ZERO, self.input.feed);
            let state = self.output.lock();
            return if state.packet.is_some() || state.terminal.is_some() {
                Ready::Result
            } else if state.settled || state.closed {
                Ready::End
            } else {
                Ready::Timeout
            };
        }
        let started = std::time::Instant::now();
        loop {
            let observed = self.output.changed.observed();
            let state = self.output.lock();
            if state.packet.is_some() || state.terminal.is_some() {
                return Ready::Result;
            }
            if state.settled || state.closed {
                return Ready::End;
            }
            drop(state);
            let readiness =
                self.input
                    .queue
                    .ready(interest, std::time::Duration::ZERO, self.input.feed);
            if readiness != Ready::Timeout {
                return readiness;
            }
            let Some(remaining) = timeout.checked_sub(started.elapsed()) else {
                return Ready::Timeout;
            };
            if remaining.is_zero() {
                return Ready::Timeout;
            }
            self.output.changed.wait(observed, remaining);
        }
    }
    /// Stop descriptor intake while retaining final output.
    pub fn cancel(&self) {
        self.input.cancel();
        let _state = self.output.lock();
        self.output.wake.notify_all();
        self.output.changed.notify();
    }
}
impl Drop for RequestRecoveringSession {
    fn drop(&mut self) {
        self.input.cancel();
        let mut state = self.output.lock();
        state.closed = true;
        state.packet = None;
        state.terminal = None;
        self.output.wake.notify_all();
        self.output.changed.notify();
    }
}
impl Engine {
    /// Start a bounded recovering source or inline request.
    /// # Errors
    /// Returns declaration refusal or failure to start the worker.
    pub fn start_request_recovering_session(
        &self,
        request: Request,
        surface: Surface,
    ) -> Result<RequestRecoveringSession, Error> {
        self.request_recovering_session_with_feed_options(
            request,
            surface,
            RequestSessionFeedOptions::default(),
        )
    }
    /// Start a bounded recovering descriptor feed with native reader controls.
    /// # Errors
    /// Refuses incompatible controls before descriptor intake.
    pub fn request_recovering_session_with_feed_options(
        &self,
        request: Request,
        surface: Surface,
        options: RequestSessionFeedOptions,
    ) -> Result<RequestRecoveringSession, Error> {
        let admitted = super::admission::admit_session(request)?;
        let feed = matches!(
            admitted.request.call.arguments().input,
            RequestInput::Feed { .. }
        );
        if !feed
            && (options.eager
                || options.image_inputs
                || options.all_filter_results
                || options.record_reading.is_some())
        {
            return Err(Error::usage(
                "owned feed controls require a caller-supplied feed",
            ));
        }
        let output = Arc::new(Output::default());
        let queue = Arc::new(super::session_queue::Queue::recovering(Arc::clone(
            &output.changed,
        )));
        let runtime_feed = make_feed(&admitted, Arc::clone(&queue), &options);
        super::execution::feed_projection(&admitted, runtime_feed.as_ref())?;
        drop(runtime_feed);
        if options.image_inputs {
            super::execution::prepare(self, &admitted, CallOptions::new().surface(surface), true)?;
        }
        let worker_output = Arc::clone(&output);
        let worker_queue = Arc::clone(&queue);
        let engine = self.clone();
        std::thread::Builder::new()
            .name("recovering-request-session".into())
            .spawn(move || {
                let function = admitted.request.call.function();
                let observe = |event: crate::RecordObservation<'_>| {
                    worker_output.publish(RequestRecoveringResult::Observation {
                        function,
                        value: event.to_owned(),
                    })
                };
                let sink = |row| worker_output.publish(RequestRecoveringResult::Row(row));
                let environment = RequestEnvironment {
                    controls: CallOptions::new()
                        .surface(surface)
                        .cancel(&worker_queue.cancel)
                        .observe(&observe),
                    feed: make_feed(&admitted, Arc::clone(&worker_queue), &options),
                };
                let terminal =
                    match engine.execute_request_recovering_sink(&admitted, environment, &sink) {
                        Ok(call) => RequestSessionTerminal {
                            facts: Some(call.facts().clone()),
                            error: None,
                        },
                        Err(error) => RequestSessionTerminal::failed(error),
                    };
                worker_queue.close_intake();
                worker_output.settle(terminal);
            })
            .map_err(|_| Error::local("session worker could not be started"))?;
        Ok(RequestRecoveringSession {
            input: RequestSession { queue, feed },
            output,
        })
    }
    /// Deliver recovering rows incrementally through the canonical request scheduler.
    /// # Errors
    /// Producer, reader and admission failures terminate; native stops join workers.
    pub fn execute_request_recovering_sink<'a>(
        &self,
        request: &'a AdmittedRequest,
        mut environment: RequestEnvironment<'a>,
        sink: &dyn Fn(RequestRecoveredRow),
    ) -> Result<Call<()>, Error> {
        let function = request.request.call.function();
        let eager = environment.feed.as_ref().is_some_and(|feed| feed.eager)
            || !matches!(
                request.request.call.arguments().input,
                RequestInput::Source { .. } | RequestInput::Feed { .. }
            );
        if eager
            || matches!(
                function,
                RequestFunction::Rank | RequestFunction::Find | RequestFunction::Relate
            )
        {
            let call = self.execute_request_recovering(request, environment)?;
            return Ok(call.map(|rows| rows.into_iter().for_each(sink)));
        }
        super::execution::feed_projection(request, environment.feed.as_ref())?;
        environment.controls.recovery_rows = true;
        super::execution::execute(
            self,
            request,
            environment.controls,
            environment
                .feed
                .as_ref()
                .is_some_and(|feed| feed.image_inputs),
            |prepared| {
                request.records(
                    &prepared.reading,
                    environment,
                    prepared.controls,
                    prepared.image_refusal.clone(),
                )
            },
            |prepared, records| {
                let dynamic = match &prepared.definition {
                    super::RequestDefinition::DynamicChoose(question) => Some(question.clone()),
                    _ => None,
                };
                let batch = match (function, dynamic) {
                    (RequestFunction::Choose, Some(question)) => {
                        prepared.engine.recover_dynamic_choose_stream(
                            question,
                            records,
                            prepared.controls,
                            prepared.context,
                        )?
                    }
                    (RequestFunction::Annotate, _) => prepared.engine.recover_annotations_stream(
                        prepared.definition.annotation()?,
                        records,
                        prepared.controls,
                        prepared.context,
                    )?,
                    (RequestFunction::Recognize, _) => {
                        let ask = prepared.definition.recognition()?;
                        let controls = match &prepared.context {
                            Some(context) => prepared.controls.context(context),
                            None => prepared.controls,
                        };
                        return prepared
                            .engine
                            .recover_recognition_stream(&ask, records, controls, sink);
                    }
                    _ => prepared.engine.recover_atomic_stream(
                        function.input_function(),
                        prepared.definition.atomic()?,
                        records,
                        prepared.controls,
                        prepared.context,
                    )?,
                };
                let mut batch = batch;
                for row in batch.by_ref() {
                    sink(row?);
                }
                let facts = batch
                    .facts()
                    .cloned()
                    .ok_or_else(|| Error::defect("a recovered stream has no final facts"))?;
                Ok(Call::new((), facts))
            },
        )
    }
}
fn make_feed(
    request: &AdmittedRequest,
    queue: Arc<super::session_queue::Queue>,
    options: &RequestSessionFeedOptions,
) -> Option<RequestFeed<'static>> {
    let RequestInput::Feed { name, .. } = &request.request.call.arguments().input else {
        return None;
    };
    let mut feed = RequestFeed::session(name.clone(), queue);
    feed.eager = options.eager;
    feed.image_inputs = options.image_inputs;
    feed.all_filter_results = options.all_filter_results;
    feed.record_reading = options.record_reading.clone();
    Some(feed)
}

#[cfg(test)]
mod tests {
    //! Stays: independent intake and output queues need direct blocked-wait control for input, cancellation and settlement wakeups.
    use super::super::RequestSessionReady as Ready;
    use super::*;
    use std::{sync::mpsc, time::Duration};
    #[test]
    fn recovered_wait_wakes_for_input_closure_cancel_and_output() {
        for (interest, expected) in [
            (RequestSessionWaitInterest::Input, Ready::Closed),
            (RequestSessionWaitInterest::Result, Ready::Cancelled),
            (RequestSessionWaitInterest::Settlement, Ready::Result),
        ] {
            wake_case(interest, expected);
        }
    }
    fn wake_case(interest: RequestSessionWaitInterest, expected: Ready) {
        let output = Arc::new(Output::default());
        let queue = Arc::new(super::super::session_queue::Queue::recovering(Arc::clone(
            &output.changed,
        )));
        let descriptor = super::super::RequestSessionDescriptor::from_json(
            r#"{"item":{"original":{"kind":"text","text":"Held."}}}"#,
        )
        .unwrap();
        assert_eq!(
            queue.push_wait(descriptor),
            RequestSessionPushStatus::Accepted
        );
        let session = RequestRecoveringSession {
            input: RequestSession {
                queue: Arc::clone(&queue),
                feed: true,
            },
            output,
        };
        std::thread::scope(|scope| {
            let (started, starting) = mpsc::sync_channel(0);
            let (ready, readiness) = mpsc::sync_channel(1);
            let session = &session;
            let waiting = scope.spawn(move || {
                started.send(()).unwrap();
                ready
                    .send(session.wait(interest, Duration::from_secs(2)))
                    .unwrap();
            });
            // The start latch and empty read prove the call has no earlier readiness.
            // The generous response bound detects a missing cross-queue wake without
            // making the fixture depend on the retired 50 ms polling interval.
            starting.recv().unwrap();
            assert!(matches!(
                readiness.recv_timeout(Duration::from_millis(20)),
                Err(mpsc::RecvTimeoutError::Timeout)
            ));
            match expected {
                Ready::Closed => queue.close_intake(),
                Ready::Cancelled => session.cancel(),
                _ => session.output.settle(RequestSessionTerminal {
                    facts: None,
                    error: None,
                }),
            }
            let result = readiness.recv_timeout(Duration::from_secs(1));
            waiting.join().unwrap();
            assert_eq!(result.unwrap(), expected);
        });
    }
}
