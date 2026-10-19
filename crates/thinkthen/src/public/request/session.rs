//! Owned request execution keeps all native borrows on one worker.
use super::session_queue::Queue;
use super::session_result::{RequestSessionResult, RequestSessionRow, RequestSessionTerminal};
use super::{
    AdmittedRequest, Request, RequestEnvironment, RequestFeed, RequestInput, RequestItem,
    RequestOutcome, RequestValue,
};
use crate::{CallOptions, Engine, Error, SourceLocation, Surface};
use serde::Deserialize;
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
    time::Duration,
};

/// Owned input content and physical provenance, separate from selected evidence.
#[derive(Debug, Clone)]
pub struct RequestSessionDescriptor {
    /// The existing canonical item grammar.
    pub item: RequestItem,
    /// Optional physical source occurrence.
    pub location: Option<SourceLocation>,
}

#[derive(Deserialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "RequestSessionDescriptor")
)]
#[serde(deny_unknown_fields)]
pub(super) struct DescriptorDocument {
    item: RequestItem,
    #[serde(default, deserialize_with = "super::present")]
    #[cfg_attr(test, schemars(with = "LocationDocument"))]
    location: Option<LocationDocument>,
}
#[derive(Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "SessionSourceLocation"), schemars(extend("dependentRequired" = serde_json::json!({"first_line":["last_line"],"last_line":["first_line"]}))))]
#[serde(deny_unknown_fields)]
pub(super) struct LocationDocument {
    file: String,
    #[serde(default, deserialize_with = "super::present")]
    #[cfg_attr(test, schemars(with = "usize", range(min = 1)))]
    first_line: Option<usize>,
    #[serde(default, deserialize_with = "super::present")]
    #[cfg_attr(test, schemars(with = "usize", range(min = 1)))]
    last_line: Option<usize>,
}
impl RequestSessionDescriptor {
    /// Decode the closed descriptor without erasing duplicates or explicit nulls.
    /// # Errors
    /// Refuses invalid items, locations and unknown descriptor controls.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        let document: DescriptorDocument =
            serde_json::from_str(text).map_err(|_| Error::usage("invalid session descriptor"))?;
        let location = document
            .location
            .map(|location| {
                SourceLocation::new(location.file, location.first_line, location.last_line)
            })
            .transpose()?;
        Ok(Self {
            item: document.item,
            location,
        })
    }
}

/// Transport failures cannot carry host diagnostics or impersonate native stops.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RequestReaderFailure {
    /// A host producer raised; its private exception stays with the host.
    Producer,
    /// The caller could not read its input.
    Io {
        /// Physical occurrence, never copied into the safe message.
        location: Option<SourceLocation>,
    },
    /// The caller read invalid UTF-8.
    Utf8 {
        /// Physical occurrence, never copied into the safe message.
        location: Option<SourceLocation>,
    },
    /// The caller could not decode its input grammar.
    InvalidInput {
        /// Physical occurrence, never copied into the safe message.
        location: Option<SourceLocation>,
    },
}
#[derive(Deserialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "RequestReaderFailure")
)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum ReaderFailureDocument {
    Producer,
    Io {
        #[serde(default, deserialize_with = "super::present")]
        #[cfg_attr(test, schemars(with = "LocationDocument"))]
        location: Option<LocationDocument>,
    },
    Utf8 {
        #[serde(default, deserialize_with = "super::present")]
        #[cfg_attr(test, schemars(with = "LocationDocument"))]
        location: Option<LocationDocument>,
    },
    InvalidInput {
        #[serde(default, deserialize_with = "super::present")]
        #[cfg_attr(test, schemars(with = "LocationDocument"))]
        location: Option<LocationDocument>,
    },
}
impl RequestReaderFailure {
    /// Decode the closed host reader failure without carrying host diagnostics.
    /// # Errors
    /// Refuses duplicate or unknown controls, explicit nulls and bad locations.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        let document: ReaderFailureDocument = serde_json::from_str(text)
            .map_err(|_| Error::usage("invalid session reader failure"))?;
        let location = match &document {
            ReaderFailureDocument::Producer => &None,
            ReaderFailureDocument::Io { location }
            | ReaderFailureDocument::Utf8 { location }
            | ReaderFailureDocument::InvalidInput { location } => location,
        }
        .as_ref()
        .map(|location| {
            SourceLocation::new(
                location.file.clone(),
                location.first_line,
                location.last_line,
            )
        })
        .transpose()?;
        Ok(match document {
            ReaderFailureDocument::Producer => Self::Producer,
            ReaderFailureDocument::Io { .. } => Self::Io { location },
            ReaderFailureDocument::Utf8 { .. } => Self::Utf8 { location },
            ReaderFailureDocument::InvalidInput { .. } => Self::InvalidInput { location },
        })
    }
    pub(super) fn error(&self) -> Error {
        match self {
            Self::Producer => Error::local("session producer raised an exception"),
            Self::Io { .. } => Error::local("session input could not be read"),
            Self::Utf8 { .. } => Error::usage("session input is not valid UTF-8"),
            Self::InvalidInput { .. } => Error::usage("session input is invalid"),
        }
    }
}

/// Intake transfers ownership only when it accepts the descriptor.
#[derive(Debug)]
pub enum RequestSessionPush {
    /// The worker owns this descriptor now.
    Accepted,
    /// Retry this same owned descriptor after backpressure clears.
    Full(RequestSessionDescriptor),
    /// Stop reading and dispose of this unaccepted descriptor.
    Closed(RequestSessionDescriptor),
}
/// JSON admission retains no caller bytes on any status.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RequestSessionPushStatus {
    /// The decoded descriptor transferred to the session.
    Accepted,
    /// Retry the same bytes without advancing the producer.
    Full,
    /// Stop advancing the producer and dispose of the unaccepted bytes.
    Closed,
}
/// A nonblocking read distinguishes unsettled work from final exhaustion.
#[derive(Debug)]
#[expect(
    clippy::large_enum_variant,
    reason = "the admitted API returns an independent owned packet directly"
)]
pub enum RequestSessionRead {
    /// One independent owned packet.
    Result(RequestSessionResult),
    /// No packet is ready; native work has not settled.
    Pending,
    /// Terminal was read and no more output will arrive.
    End,
}

/// Events that a caller wants a timed native wait to observe.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RequestSessionWaitInterest {
    /// Wait for output, exhaustion or cancellation; ignore available input room.
    Result,
    /// Wait for output, input room/closure, exhaustion or cancellation.
    Input,
    /// Drain final output after cancellation without repeatedly waking for cancel.
    Settlement,
    /// Arm one exclusive settlement reader; return readiness without waiting.
    /// Discard earlier cancellation notifications. The timeout is ignored.
    SettlementArm,
    /// Wait for final output or cancellation requested after SettlementArm.
    /// Arm before checking the caller's cancellation/disposal state, then wait.
    InterruptibleSettlement,
}

/// Readiness does not consume a packet or transfer an input descriptor.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RequestSessionReady {
    /// A result or terminal packet can be read.
    Result,
    /// The input cell has room.
    Input,
    /// Intake has closed; the producer must stop.
    Closed,
    /// All output has been consumed.
    End,
    /// Cancellation stopped intake; final output may still be pending.
    Cancelled,
    /// The caller's timeout passed without requested readiness.
    Timeout,
}

/// Owned session controls, separate from canonical Request JSON.
#[derive(Default, Clone)]
pub struct RequestSessionFeedOptions {
    /// Owned ordinary annotation pointer policy, defaulting to no continuation.
    /// The worker calls this only for an absent question-set `on` pointer, with
    /// zero-based original ordinal and pointer borrowed for the callback. True
    /// skips that original; false or panic retains its native failure. It runs
    /// without queue/scheduler locks. Never wait on this same session, and release
    /// caller waits on cancellation or receiver shutdown. General recovering
    /// sessions and other functions refuse this policy before reading input.
    #[expect(clippy::type_complexity, reason = "the owned narrow callback retains the ordinary annotation policy without a result or sink facade")]
    pub on_missing_annotation_pointer: Option<Arc<dyn Fn(usize, &str) -> bool + Send + Sync>>,
    /// Retain an otherwise successful native result when cancellation arrives
    /// during final settlement. Active work still stops on cancellation.
    /// Defaults to false: ordinary caller cancellation takes precedence over success.
    /// Applies to inline, source and feed requests in either session mode.
    pub retain_success_on_cancel: bool,
    /// Publish each completed recognition collection immediately as an Aggregate
    /// packet, before input EOF. Defaults to false, preserving one final whole-
    /// function Aggregate, including empty input and a failed completed prefix.
    /// With this ordinary recognition-only option, empty collections publish
    /// nothing and the terminal answer still retains the complete ordered prefix.
    /// Bounded publication applies backpressure and never replays the final prefix.
    pub incremental_recognition: bool,
    /// Override the native open-batch input pause for a caller-supplied feed.
    /// None retains the native default; zero retains immediate native closure.
    /// This controls the existing host pause, without another timer. Headers,
    /// blanks and buffered logical input do not count as an input pause.
    pub input_pause: Option<Duration>,
    /// Validate the supported input before any provider sends.
    pub eager: bool,
    /// Admit the image route before the caller reads content.
    pub image_inputs: bool,
    /// Preserve rejected as well as passing filter rows.
    pub all_filter_results: bool,
    /// Replace the prepared record projection with an owned native reading.
    pub record_reading: Option<crate::RecordReading>,
}

impl std::fmt::Debug for RequestSessionFeedOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestSessionFeedOptions")
            .field("retain_success_on_cancel", &self.retain_success_on_cancel)
            .field("incremental_recognition", &self.incremental_recognition)
            .field("input_pause", &self.input_pause)
            .field("eager", &self.eager).field("image_inputs", &self.image_inputs)
            .field("all_filter_results", &self.all_filter_results)
            .field("record_reading", &self.record_reading).finish()
    }
}

/// One session with bounded intake and owned unread output.
/// Drop signals stop and never joins a provider.
#[derive(Debug)]
pub struct RequestSession {
    pub(super) queue: Arc<Queue>,
    pub(super) feed: bool,
}
impl RequestSession {
    /// Transfer one descriptor without waiting for native work or queue space.
    /// # Errors
    /// Refuses feed controls on a declared inline/source request.
    pub fn try_push(
        &self,
        descriptor: RequestSessionDescriptor,
    ) -> Result<RequestSessionPush, Error> {
        if !self.feed {
            return Err(Error::usage("this session has no caller-supplied feed"));
        }
        Ok(self.queue.push(descriptor))
    }
    /// Admit length-delimited host JSON through the canonical descriptor decoder.
    /// Capacity and closure are checked before decoding; decoding never holds
    /// the queue lock. A concurrent full or closure race discards the temporary
    /// owned value without publishing it. No caller bytes survive this return.
    /// # Errors
    /// Refuses non-feed sessions and malformed descriptors when intake has room.
    pub fn try_push_json(&self, text: &str) -> Result<RequestSessionPushStatus, Error> {
        self.try_push_with(|| RequestSessionDescriptor::from_json(text))
    }
    /// Convert and transfer one host descriptor when the feed has room.
    /// Native adapters use this operation to avoid inspecting borrowed payloads
    /// on full or closed intake. Conversion runs without the queue lock; a
    /// concurrent push or closure can discard the converted owned descriptor.
    /// The callback runs at most once and no borrowed payload is retained.
    /// # Errors
    /// Refuses non-feed sessions and propagates conversion failures only when
    /// intake has room. Adapter errors retain their own representation limits.
    pub fn try_push_with<E: From<Error>>(
        &self,
        convert: impl FnOnce() -> Result<RequestSessionDescriptor, E>,
    ) -> Result<RequestSessionPushStatus, E> {
        if !self.feed {
            return Err(Error::usage("this session has no caller-supplied feed").into());
        }
        match self.queue.capacity() {
            RequestSessionPushStatus::Accepted => {}
            status => return Ok(status),
        }
        Ok(match self.queue.push(convert()?) {
            RequestSessionPush::Accepted => RequestSessionPushStatus::Accepted,
            RequestSessionPush::Full(_) => RequestSessionPushStatus::Full,
            RequestSessionPush::Closed(_) => RequestSessionPushStatus::Closed,
        })
    }
    /// Wait for input room, parking unread output in session order as needed.
    /// Unread answers consume memory; they never block this producer.
    /// Convert at most once, outside the queue lock, only after intake has room.
    /// Closure or cancellation wakes the producer and retains nothing.
    /// # Errors
    /// Refuses non-feed sessions and propagates conversion failures on this caller.
    pub fn push_with<E: From<Error>>(
        &self,
        convert: impl FnOnce() -> Result<RequestSessionDescriptor, E>,
    ) -> Result<RequestSessionPushStatus, E> {
        if !self.feed {
            return Err(Error::usage("this session has no caller-supplied feed").into());
        }
        if !self.queue.input_room() {
            return Ok(RequestSessionPushStatus::Closed);
        }
        Ok(self.queue.push_wait(convert()?))
    }
    /// Read one packet without waiting for native work.
    #[must_use]
    pub fn try_read(&self) -> RequestSessionRead {
        ordinary_read(self.queue.read())
    }
    /// Block and transfer the next independent packet, or return End after drain.
    /// Result interest wakes promptly on cancellation; Settlement drains final facts.
    /// Queued output takes priority over cancellation. No provider work is joined.
    /// # Errors
    /// Refuses all interests except Result/Settlement; Result can return cancellation.
    pub fn next_packet(
        &self,
        interest: RequestSessionWaitInterest,
    ) -> Result<RequestSessionRead, Error> {
        self.queue.next_packet(interest).map(ordinary_read)
    }
    /// Block without consuming output until requested readiness or timeout.
    /// Result waits ignore input room. Settlement waits ignore cancellation so
    /// callers can drain final facts after a cancellation wake. Cancellation
    /// wakes Result/Input waits immediately and never joins provider work.
    /// An exclusive settlement reader arms before checking caller cancellation,
    /// then waits with InterruptibleSettlement; a later cancel interrupts that wait.
    #[must_use]
    pub fn wait(
        &self,
        interest: RequestSessionWaitInterest,
        timeout: std::time::Duration,
    ) -> RequestSessionReady {
        self.queue.ready(interest, timeout, self.feed)
    }
    /// Fix EOF or transport failure independently of a full input cell.
    /// # Errors
    /// Refuses a changed terminal input or a failure on a non-feed request.
    pub fn finish(&self, failure: Option<RequestReaderFailure>) -> Result<(), Error> {
        if !self.feed {
            return if failure.is_none() {
                Ok(())
            } else {
                Err(Error::usage("this session has no caller-supplied feed"))
            };
        }
        self.queue.finish(failure)
    }
    /// Finish with an owned native reader diagnostic after accepted descriptors.
    /// # Errors
    /// Refuses non-reader error kinds, invocation facts and any prior finish.
    /// Native errors transfer once; transport EOF and equal transport failures remain idempotent.
    pub fn finish_native_reader_error(&self, error: Error) -> Result<(), Error> {
        if !self.feed {
            return Err(Error::usage("this session has no caller-supplied feed"));
        }
        if !matches!(
            error.kind(),
            crate::ErrorKind::Usage | crate::ErrorKind::Local
        ) || error.facts().is_some()
        {
            return Err(Error::usage(
                "session native reader failure requires Usage or Local without facts",
            ));
        }
        self.queue.finish_native(error)
    }
    /// Stop intake promptly. Final facts remain pending until native settlement.
    pub fn cancel(&self) {
        self.queue.stop(false);
    }
}
impl Drop for RequestSession {
    fn drop(&mut self) {
        self.queue.stop(true);
    }
}

impl Engine {
    /// Admit a request synchronously, then execute it on its owning worker.
    /// # Errors
    /// Returns declaration refusal or failure to create the native worker.
    pub fn request_session(&self, request: Request) -> Result<RequestSession, Error> {
        self.request_session_with_surface(request, Surface::Rust)
    }
    /// Admit a request with the explicit host identity owned by its worker.
    /// # Errors
    /// Returns declaration refusal or failure to create the native worker.
    pub fn request_session_with_surface(
        &self,
        request: Request,
        surface: Surface,
    ) -> Result<RequestSession, Error> {
        self.request_session_with_feed_options(
            request,
            surface,
            RequestSessionFeedOptions::default(),
        )
    }
    /// Admit owned native feed controls before starting the session worker.
    /// # Errors
    /// Refuses incompatible feed projections, image routes and declarations.
    pub fn request_session_with_feed_options(
        &self,
        request: Request,
        surface: Surface,
        options: RequestSessionFeedOptions,
    ) -> Result<RequestSession, Error> {
        self.start_session(request, surface, options, Mode::Ordinary)
    }
    pub(super) fn start_session(
        &self,
        request: Request,
        surface: Surface,
        options: RequestSessionFeedOptions,
        mode: Mode,
    ) -> Result<RequestSession, Error> {
        if options.on_missing_annotation_pointer.is_some() {
            if request.call.function() != super::RequestFunction::Annotate {
                return Err(Error::usage("missing annotation pointer continuation requires an annotation request"));
            }
            if matches!(mode, Mode::Recovering) {
                return Err(Error::usage("annotation pointer continuation cannot accompany general row recovery"));
            }
        }
        if options.incremental_recognition && (request.call.function() != super::RequestFunction::Recognize
            || matches!(mode, Mode::Recovering)) {
            return Err(Error::usage("incremental recognition requires an ordinary recognition session"));
        }
        let admitted = super::admission::admit_session(request)?;
        let feed = matches!(
            admitted.request.call.arguments().input,
            RequestInput::Feed { .. }
        );
        let queue = Arc::new(Queue::default());
        if let RequestInput::Feed { framing, .. } = admitted.request.call.arguments().input {
            queue.configure_input(framing);
        }
        let configured_feed = match &admitted.request.call.arguments().input {
            RequestInput::Feed { name, .. } => {
                Some(configured_feed(name.clone(), Arc::clone(&queue), &options))
            }
            _ if options.input_pause.is_some() || options.eager
                || options.image_inputs
                || options.all_filter_results
                || options.record_reading.is_some() =>
            {
                return Err(Error::usage(
                    "owned feed controls require a caller-supplied feed",
                ));
            }
            _ => None,
        };
        super::execution::feed_projection(&admitted, configured_feed.as_ref())?;
        // Existing constructors retain asynchronous runtime preparation. Explicit
        // image declaration must complete the same native route check before intake.
        if options.image_inputs {
            super::execution::prepare(self, &admitted, CallOptions::new().surface(surface), true)?;
        }
        let worker_queue = Arc::clone(&queue);
        let engine = self.clone();
        std::thread::Builder::new()
            .name("request-session".into())
            .spawn(move || match mode {
                Mode::Ordinary => run(engine, admitted, &worker_queue, surface, options),
                Mode::Recovering => {
                    run_recovering(engine, admitted, &worker_queue, surface, options)
                }
            })
            .map_err(|_| Error::local("session worker could not be started"))?;
        Ok(RequestSession { queue, feed })
    }
}

fn configured_feed(
    name: String,
    queue: Arc<Queue>,
    options: &RequestSessionFeedOptions,
) -> RequestFeed<'static> {
    let mut feed = RequestFeed::session(name, queue);
    feed.eager = options.eager;
    feed.image_inputs = options.image_inputs;
    feed.all_filter_results = options.all_filter_results;
    feed.record_reading = options.record_reading.clone();
    feed
}

fn run(
    engine: Engine,
    request: AdmittedRequest,
    queue: &Arc<Queue>,
    surface: Surface,
    options: RequestSessionFeedOptions,
) {
    let function = request.request.call.function();
    let observer = |event: crate::RecordObservation<'_>| {
        queue.publish(RequestSessionResult::Observation {
            function,
            value: event.to_owned(),
        });
    };
    let files = Mutex::new(std::collections::BTreeSet::new());
    let files_only = request.request.call.arguments().options.files_only;
    // Recognition completes records incrementally, but exposes one whole-request packet.
    // Keep its completed prefix until the native invocation settles, including a failure.
    let recognized = RefCell::new(Vec::new());
    let answers = RefCell::new(super::settlement::Settlement::new(
        &request.request.call.arguments().input,
    ));
    let sink = |value| match value {
        RequestValue::Recognized(rows) => {
            answers
                .borrow_mut()
                .record(&RequestValue::Recognized(rows.clone()));
            if options.incremental_recognition {
                if !rows.is_empty() {
                    queue.publish(RequestSessionResult::Aggregate(RequestValue::Recognized(rows)));
                }
            } else { recognized.borrow_mut().extend(rows); }
        }
        value => publish_rows(
            queue,
            value,
            files_only,
            &files,
            options.all_filter_results,
            &answers,
        ),
    };
    let feed = match &request.request.call.arguments().input {
        RequestInput::Feed { name, .. } => {
            Some(configured_feed(name.clone(), Arc::clone(queue), &options))
        }
        _ => None,
    };
    let mut controls = CallOptions::new().surface(surface).cancel(&queue.cancel)
        .retain_success_on_cancel(options.retain_success_on_cancel).observe(&observer);
    if let Some(callback) = options.on_missing_annotation_pointer.as_deref() {
        controls = controls.on_missing_annotation_pointer(callback);
    }
    let input_ready = || queue.logical_ready(CallOptions::new().cancel(&queue.cancel));
    let readiness = crate::public::readiness::Readiness::new(&input_ready, options.input_pause);
    let registration = feed.as_ref().map(|_| super::session_input::Registration::new(queue, readiness.wake()));
    if feed.is_some() { controls = controls.readiness(&readiness); }
    let environment = RequestEnvironment { controls, feed };
    let mut terminal =
        match engine.execute_request_sink(&request, environment, Some(&sink), None, None) {
            Ok(RequestOutcome::Complete(call)) => {
                let facts = call.facts().clone();
                queue.close_intake();
                sink(call.into_value());
                RequestSessionTerminal {
                    facts: Some(facts),
                    answer: None,
                    resolution: crate::AnswerResolution::Resolved,
                    error: None,
                }
            }
            Ok(RequestOutcome::Failed { completed, error }) => {
                queue.close_intake();
                sink(completed);
                RequestSessionTerminal::failed(error)
            }
            Err(error) => {
                queue.close_intake();
                RequestSessionTerminal::failed(error)
            }
        };
    if function == super::RequestFunction::Recognize && !options.incremental_recognition {
        queue.publish(RequestSessionResult::Aggregate(RequestValue::Recognized(
            recognized.into_inner(),
        )));
    }
    terminal.answer = answers.into_inner().finish();
    drop(registration);
    queue.settle(terminal);
}

fn publish_rows(
    queue: &Queue,
    value: RequestValue,
    files_only: bool,
    files: &Mutex<std::collections::BTreeSet<String>>,
    all_filter_results: bool,
    answers: &RefCell<super::settlement::Settlement>,
) {
    macro_rules! rows {
        ($rows:expr, $variant:ident) => {
            for row in $rows {
                queue.publish(RequestSessionResult::Row(RequestSessionRow::$variant(row)));
            }
        };
    }
    if !matches!(&value, RequestValue::Filtered(_)) {
        answers.borrow_mut().record(&value);
    }
    match value {
        RequestValue::Decisions(rows) => rows!(rows, Decision),
        RequestValue::Choices(rows) => rows!(rows, Choice),
        RequestValue::Tags(rows) => rows!(rows, Tags),
        RequestValue::Scores(rows) => rows!(rows, Score),
        RequestValue::Annotations(rows) => rows!(rows, Annotation),
        RequestValue::Filtered(rows) => {
            let mut retained = Vec::new();
            for row in rows {
                let selected = super::execution::selected_filter(
                    &row,
                    files_only,
                    &mut files.lock().unwrap_or_else(|e| e.into_inner()),
                );
                if selected || all_filter_results {
                    retained.push(row.clone());
                    queue.publish(RequestSessionResult::Row(RequestSessionRow::Filter(row)));
                }
            }
            answers
                .borrow_mut()
                .record(&RequestValue::Filtered(retained));
        }
        aggregate => queue.publish(RequestSessionResult::Aggregate(aggregate)),
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Mode {
    Ordinary,
    Recovering,
}
fn ordinary_read(read: super::session_queue::Read) -> RequestSessionRead {
    use super::session_queue::{Packet, Read};
    match read {
        Read::Packet(Packet::Ordinary(packet)) => RequestSessionRead::Result(packet),
        Read::Terminal(terminal) => {
            RequestSessionRead::Result(RequestSessionResult::Terminal(terminal))
        }
        Read::Pending => RequestSessionRead::Pending,
        Read::End => RequestSessionRead::End,
        Read::Packet(Packet::Recovering(_)) => RequestSessionRead::Result(
            RequestSessionResult::Terminal(RequestSessionTerminal::failed(Error::defect(
                "recovering packet in ordinary session",
            ))),
        ),
    }
}
fn run_recovering(
    engine: Engine,
    request: AdmittedRequest,
    queue: &Arc<Queue>,
    surface: Surface,
    options: RequestSessionFeedOptions,
) {
    use super::RequestRecoveringResult;
    let function = request.request.call.function();
    let observe = |event: crate::RecordObservation<'_>| {
        queue.publish(RequestRecoveringResult::Observation {
            function,
            value: event.to_owned(),
        })
    };
    let answers = RefCell::new(super::settlement::Settlement::new(
        &request.request.call.arguments().input,
    ));
    let sink = |row: super::RequestRecoveredRow| {
        answers.borrow_mut().recovered(function, &row);
        queue.publish(RequestRecoveringResult::Row(row));
    };
    let input_ready = || queue.logical_ready(CallOptions::new().cancel(&queue.cancel));
    let readiness = crate::public::readiness::Readiness::new(&input_ready, options.input_pause);
    let feed_request = matches!(request.request.call.arguments().input, RequestInput::Feed { .. });
    let registration = feed_request.then(|| super::session_input::Registration::new(queue, readiness.wake()));
    let mut controls = CallOptions::new().surface(surface).cancel(&queue.cancel)
        .retain_success_on_cancel(options.retain_success_on_cancel).observe(&observe);
    if feed_request { controls = controls.readiness(&readiness); }
    let environment = RequestEnvironment {
        controls,
        feed: match &request.request.call.arguments().input {
            RequestInput::Feed { name, .. } => {
                Some(configured_feed(name.clone(), Arc::clone(queue), &options))
            }
            _ => None,
        },
    };
    let mut terminal = match engine.execute_request_recovering_sink(&request, environment, &sink) {
        Ok(call) => RequestSessionTerminal {
            facts: Some(call.facts().clone()),
            answer: None,
            resolution: crate::AnswerResolution::Resolved,
            error: None,
        },
        Err(error) => RequestSessionTerminal::failed(error),
    };
    terminal.answer = answers.into_inner().finish();
    drop(registration);
    queue.settle(terminal);
}
