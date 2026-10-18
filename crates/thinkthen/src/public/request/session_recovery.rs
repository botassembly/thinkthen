//! Typed recovering view of the shared session owner; no independent state or wake.
use super::{Request, RequestFunction, RequestRecoveredRow, RequestSession, RequestSessionFeedOptions, RequestSessionPushStatus, RequestSessionTerminal, RequestSessionWaitInterest};
use crate::{Engine, Error, OwnedRecordObservation, Surface};
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
/// One recovering request with owned descriptor intake and parked unread output.
#[derive(Debug)]
pub struct RequestRecoveringSession {
    input: RequestSession,
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
        recovered_read(self.input.queue.read())
    }
    /// Wait for the next recovered output or joined terminal.
    /// # Errors
    /// Refuses unsupported interests; Result observes cancellation promptly.
    pub fn next_packet(&self, interest: RequestSessionWaitInterest) -> Result<RequestRecoveringRead, Error> {
        self.input.queue.next_packet(interest).map(recovered_read)
    }
    /// Wait without consuming output; arm before checking caller cancellation.
    #[must_use]
    pub fn wait(&self, interest: RequestSessionWaitInterest, timeout: std::time::Duration) -> super::RequestSessionReady {
        self.input.wait(interest, timeout)
    }
    /// Stop descriptor intake while retaining final output.
    pub fn cancel(&self) { self.input.cancel(); }
}
fn recovered_read(read: super::session_queue::Read) -> RequestRecoveringRead {
    use super::session_queue::{Packet, Read};
    match read {
        Read::Packet(Packet::Recovering(packet)) => RequestRecoveringRead::Result(packet),
        Read::Terminal(terminal) => RequestRecoveringRead::Result(RequestRecoveringResult::Terminal(terminal)),
        Read::Pending => RequestRecoveringRead::Pending,
        Read::End => RequestRecoveringRead::End,
        Read::Packet(Packet::Ordinary(_)) => RequestRecoveringRead::Result(RequestRecoveringResult::Terminal(RequestSessionTerminal::failed(Error::defect("ordinary packet in recovering session")))),
    }
}
impl Engine {
    /// Start a recovering source or inline request with owned unread output.
    /// # Errors
    /// Returns declaration refusal or failure to start the worker.
    pub fn start_request_recovering_session(&self, request: Request, surface: Surface) -> Result<RequestRecoveringSession, Error> {
        self.request_recovering_session_with_feed_options(request, surface, RequestSessionFeedOptions::default())
    }
    /// Start a recovering feed with bounded intake and native reader controls.
    /// # Errors
    /// Refuses incompatible controls before descriptor intake.
    pub fn request_recovering_session_with_feed_options(&self, request: Request, surface: Surface, options: RequestSessionFeedOptions) -> Result<RequestRecoveringSession, Error> {
        self.start_session(request, surface, options, super::session::Mode::Recovering)
            .map(|input| RequestRecoveringSession { input })
    }
}
