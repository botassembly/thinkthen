//! Owned request execution keeps all native borrows on one worker.
use super::session_queue::Queue;
use super::session_result::{RequestSessionResult, RequestSessionRow, RequestSessionTerminal};
use super::{
    AdmittedRequest, Request, RequestEnvironment, RequestFeed, RequestInput, RequestItem,
    RequestOutcome, RequestValue,
};
use crate::{CallOptions, Engine, Error, SourceLocation};
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
struct LocationDocument {
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
impl RequestReaderFailure {
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
        let admitted = super::admission::admit_session(request)?;
        let feed = matches!(
            admitted.request.call.arguments().input,
            RequestInput::Feed { .. }
        );
        let queue = Arc::new(Queue::default());
        let worker_queue = Arc::clone(&queue);
        let engine = self.clone();
        std::thread::Builder::new()
            .name("request-session".into())
            .spawn(move || {
                run(engine, admitted, &worker_queue);
            })
            .map_err(|_| Error::local("session worker could not be started"))?;
        Ok(RequestSession { queue, feed })
    }
}

fn run(engine: Engine, request: AdmittedRequest, queue: &Arc<Queue>) {
    let observer = |event: crate::RecordObservation<'_>| {
        queue.publish(RequestSessionResult::Observation(event.to_owned()));
    };
    let files = Mutex::new(std::collections::BTreeSet::new());
    let files_only = request.request.call.arguments().options.files_only;
    let sink = |value| publish_rows(queue, value, files_only, &files);
    let feed = match &request.request.call.arguments().input {
        RequestInput::Feed { name, .. } => {
            Some(RequestFeed::session(name.clone(), Arc::clone(queue)))
        }
        _ => None,
    };
    let environment = RequestEnvironment {
        controls: CallOptions::new().cancel(&queue.cancel).observe(&observer),
        feed,
    };
    let terminal = match engine.execute_request_sink(&request, environment, Some(&sink)) {
        Ok(RequestOutcome::Complete(call)) => {
            let facts = call.facts().clone();
            publish_rows(queue, call.into_value(), files_only, &files);
            RequestSessionTerminal {
                facts: Some(facts),
                error: None,
            }
        }
        Ok(RequestOutcome::Failed { completed, error }) => {
            publish_rows(queue, completed, files_only, &files);
            RequestSessionTerminal::failed(error)
        }
        Err(error) => RequestSessionTerminal::failed(error),
    };
    queue.settle(terminal);
}

fn publish_rows(
    queue: &Queue,
    value: RequestValue,
    files_only: bool,
    files: &Mutex<std::collections::BTreeSet<String>>,
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
                if selected {
                    queue.publish(RequestSessionResult::Row(RequestSessionRow::Filter(row)));
                }
            }
        }
        aggregate => queue.publish(RequestSessionResult::Aggregate(aggregate)),
    }
}
