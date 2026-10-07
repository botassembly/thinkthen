//! Native structured stops never parse diagnostic text.
use super::{EngineError, ErrorKind};
use serde::Serialize;

/// The native cause behind one of the six stable error kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum StopCause {
    /// Admission or caller-budget refusal.
    Usage,
    /// Local storage or reader failure.
    Local,
    /// Configured key unavailable when a live send needed it.
    NoKey,
    /// Transport failed; the request may have arrived and is not retried.
    Transport,
    /// HTTP status stopped execution.
    Status,
    /// Oversized request or response refused.
    TooLarge,
    /// Known reply field or required logical answer was invalid.
    Reply,
    /// Another backend failure, including incompatible reported models.
    Backend,
    /// Caller cancellation.
    Cancelled,
    /// Caller deadline.
    Deadline,
    /// Internal defect.
    Defect,
}

/// Owned structured stop information retained with an error snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Stopped {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) at: Option<usize>,
    pub(super) cause: StopCause,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<u16>,
    retryable: bool,
}
impl Stopped {
    /// Original one-based stopping record, only when the native runner knows it.
    #[must_use]
    pub const fn at(&self) -> Option<usize> {
        self.at
    }
    /// Actual structured native cause.
    #[must_use]
    pub const fn cause(&self) -> StopCause {
        self.cause
    }
    /// Actual stopping status, including the last status before retry-budget refusal.
    #[must_use]
    pub const fn status(&self) -> Option<u16> {
        self.status
    }
    /// Whether the same call may succeed later under the existing retry boundary.
    #[must_use]
    pub const fn retryable(&self) -> bool {
        self.retryable
    }

    pub(super) const fn of_kind(kind: ErrorKind) -> Self {
        let cause = match kind {
            ErrorKind::Usage => StopCause::Usage,
            ErrorKind::Local => StopCause::Local,
            ErrorKind::Backend => StopCause::Backend,
            ErrorKind::Cancelled => StopCause::Cancelled,
            ErrorKind::Deadline => StopCause::Deadline,
            ErrorKind::Defect => StopCause::Defect,
        };
        Self {
            at: None,
            cause,
            status: None,
            retryable: false,
        }
    }
    pub(super) fn of_engine(error: &EngineError) -> Self {
        let mut stopped = Self::of_kind(super::kind_of(error.kind()));
        stopped.retryable = error.retryable();
        match error {
            EngineError::NoKey(_) => stopped.cause = StopCause::NoKey,
            EngineError::Transport(_) => stopped.cause = StopCause::Transport,
            EngineError::Status(status) => {
                stopped.cause = StopCause::Status;
                stopped.status = Some(*status);
            }
            EngineError::TokenLimit => {
                stopped.cause = StopCause::TooLarge;
                stopped.status = Some(400);
            }
            EngineError::ReplyTooLarge(_) => stopped.cause = StopCause::TooLarge,
            EngineError::Reply(_) | EngineError::RecognizeLogical => {
                stopped.cause = StopCause::Reply
            }
            EngineError::SendBudgetRetry(status) => stopped.status = Some(*status),
            _ => {}
        }
        stopped
    }
}
