//! Machine-readable cause beside the unchanged human diagnostic and exit code.

use std::io::Write;

use serde::Serialize;

use super::Failure;

#[derive(Serialize)]
pub(crate) struct Stopped {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) at: Option<usize>,
    pub(crate) cause: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) status: Option<u16>,
    pub(crate) retryable: bool,
}

/// Keep the existing diagnostic and expose the same exit code to run facts.
pub(crate) fn report(failure: &Failure, mut writer: impl Write) -> u8 {
    super::say(failure, &mut writer)
}

impl Stopped {
    pub(crate) fn of(failure: &Failure, exit: u8) -> Self {
        let (at, cause) = match failure {
            Failure::Stopped { at, cause, .. } => (Some(*at), cause.as_ref()),
            _ => (Some(1), failure),
        };
        let cause = match cause {
            Failure::BatchFailed { cause, .. } => cause.as_ref(),
            other => other,
        };
        let (name, status, retryable) = match cause {
            Failure::Cancelled => ("cancelled", None, false),
            Failure::NoKey(_) => ("no_key", None, false),
            Failure::Transport(_) => ("transport", None, false),
            Failure::Status(413) | Failure::TokenLimit => ("too_large", None, false),
            Failure::Status(status) => (
                "status",
                Some(*status),
                crate::engine::error::retried_status(*status),
            ),
            Failure::Reply(_) | Failure::ReplyTooLarge(_) | Failure::PartialReply { .. } => {
                ("reply", None, false)
            }
            _ => (
                match exit {
                    2 => "usage",
                    5 => "local",
                    4 => "backend",
                    130 | 143 => "cancelled",
                    _ => "defect",
                },
                None,
                false,
            ),
        };
        Self {
            at: (name != "cancelled").then_some(at).flatten(),
            cause: name,
            status,
            retryable,
        }
    }
}
