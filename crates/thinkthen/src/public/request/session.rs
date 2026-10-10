//! Owned request execution keeps all native borrows on one worker.
use super::session_queue::Queue;
use super::session_result::{RequestSessionResult, RequestSessionRow, RequestSessionTerminal};
use super::{
    AdmittedRequest, Request, RequestEnvironment, RequestFeed, RequestInput, RequestItem,
    RequestOutcome, RequestValue,
};
use crate::{CallOptions, Engine, Error, SourceLocation, Surface};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

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
            ReaderFailureDocument::Io { .. } => Self::Io { location },
            ReaderFailureDocument::Utf8 { .. } => Self::Utf8 { location },
            ReaderFailureDocument::InvalidInput { .. } => Self::InvalidInput { location },
        })
    }
    pub(super) fn error(&self) -> Error {
        match self {
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

/// Owned controls for the native session feed, separate from canonical Request JSON.
#[derive(Debug, Default, Clone)]
pub struct RequestSessionFeedOptions {
    /// Validate the supported input before any provider sends.
    pub eager: bool,
    /// Admit the image route before the caller reads content.
    pub image_inputs: bool,
    /// Preserve rejected as well as passing filter rows.
    pub all_filter_results: bool,
    /// Replace the prepared record projection with an owned native reading.
    pub record_reading: Option<crate::RecordReading>,
}

/// One bounded session. Drop signals stop and never joins a provider.
#[derive(Debug)]
pub struct RequestSession {
    queue: Arc<Queue>,
    feed: bool,
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
        if !self.feed {
            return Err(Error::usage("this session has no caller-supplied feed"));
        }
        match self.queue.capacity() {
            RequestSessionPushStatus::Accepted => {}
            status => return Ok(status),
        }
        let descriptor = RequestSessionDescriptor::from_json(text)?;
        Ok(match self.queue.push(descriptor) {
            RequestSessionPush::Accepted => RequestSessionPushStatus::Accepted,
            RequestSessionPush::Full(_) => RequestSessionPushStatus::Full,
            RequestSessionPush::Closed(_) => RequestSessionPushStatus::Closed,
        })
    }
    /// Read one packet without waiting for native work.
    #[must_use]
    pub fn try_read(&self) -> RequestSessionRead {
        self.queue.read()
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
        let admitted = super::admission::admit_session(request)?;
        let feed = matches!(
            admitted.request.call.arguments().input,
            RequestInput::Feed { .. }
        );
        let queue = Arc::new(Queue::default());
        let configured_feed = match &admitted.request.call.arguments().input {
            RequestInput::Feed { name, .. } => {
                Some(configured_feed(name.clone(), Arc::clone(&queue), &options))
            }
            _ if options.eager
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
            .spawn(move || {
                run(engine, admitted, &worker_queue, surface, options);
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
    let sink = |value| publish_rows(queue, value, files_only, &files, options.all_filter_results);
    let feed = match &request.request.call.arguments().input {
        RequestInput::Feed { name, .. } => {
            Some(configured_feed(name.clone(), Arc::clone(queue), &options))
        }
        _ => None,
    };
    let environment = RequestEnvironment {
        controls: CallOptions::new()
            .surface(surface)
            .cancel(&queue.cancel)
            .observe(&observer),
        feed,
    };
    let terminal = match engine.execute_request_sink(&request, environment, Some(&sink), None, None)
    {
        Ok(RequestOutcome::Complete(call)) => {
            let facts = call.facts().clone();
            queue.close_intake();
            publish_rows(
                queue,
                call.into_value(),
                files_only,
                &files,
                options.all_filter_results,
            );
            RequestSessionTerminal {
                facts: Some(facts),
                error: None,
            }
        }
        Ok(RequestOutcome::Failed { completed, error }) => {
            queue.close_intake();
            publish_rows(
                queue,
                completed,
                files_only,
                &files,
                options.all_filter_results,
            );
            RequestSessionTerminal::failed(error)
        }
        Err(error) => {
            queue.close_intake();
            RequestSessionTerminal::failed(error)
        }
    };
    queue.settle(terminal);
}

fn publish_rows(
    queue: &Queue,
    value: RequestValue,
    files_only: bool,
    files: &Mutex<std::collections::BTreeSet<String>>,
    all_filter_results: bool,
) {
    macro_rules! rows {
        ($rows:expr, $variant:ident) => {
            for row in $rows {
                queue.publish(RequestSessionResult::Row(RequestSessionRow::$variant(row)));
            }
        };
    }
    match value {
        RequestValue::Decisions(rows) => rows!(rows, Decision),
        RequestValue::Choices(rows) => rows!(rows, Choice),
        RequestValue::Tags(rows) => rows!(rows, Tags),
        RequestValue::Scores(rows) => rows!(rows, Score),
        RequestValue::Annotations(rows) => rows!(rows, Annotation),
        RequestValue::Filtered(rows) => {
            for row in rows {
                let selected = super::execution::selected_filter(
                    &row,
                    files_only,
                    &mut files.lock().unwrap_or_else(|e| e.into_inner()),
                );
                if selected || all_filter_results {
                    queue.publish(RequestSessionResult::Row(RequestSessionRow::Filter(row)));
                }
            }
        }
        aggregate => queue.publish(RequestSessionResult::Aggregate(aggregate)),
    }
}
