//! Safe failure envelopes retain actual invocation facts.
use super::CompleteFacts;
use crate::public::{Error, ErrorKind, EstimatedInputDenial, SendBudgetDenial, Stopped};
use serde::Serialize;
use std::fmt;

/// Safe failure fields; no successful result or answer identity is fabricated.
#[derive(Clone, Copy, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ErrorSnapshot<'a> {
    kind: ErrorKind,
    message: &'a str,
    retryable: bool,
    stopped: Stopped,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_budget_denial: Option<SendBudgetDenial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_input_denial: Option<EstimatedInputDenial>,
}
impl ErrorSnapshot<'_> {
    /// Stable public kind.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }
    /// Native safe diagnostic.
    #[must_use]
    pub const fn message(&self) -> &str {
        self.message
    }
    /// Typed stopping cause and known original record position.
    #[must_use]
    pub const fn stopped(&self) -> Stopped {
        self.stopped
    }
}

/// Failure envelope with final started-call facts, absent for admission refusal.
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CompleteError<'a> {
    error: ErrorSnapshot<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facts: Option<CompleteFacts<'a>>,
}
impl Error {
    /// Borrow safe complete failure fields and actual final facts.
    #[must_use]
    pub fn complete(&self) -> CompleteError<'_> {
        CompleteError {
            error: ErrorSnapshot {
                kind: self.kind(),
                message: self.detail().message(),
                retryable: self.retryable(),
                stopped: self.stopped(),
                send_budget_denial: self.send_budget_denial(),
                estimated_input_denial: self.estimated_input_denial(),
            },
            facts: self.facts().and_then(super::Facts::complete),
        }
    }
}
impl CompleteError<'_> {
    /// The safe native failure fields.
    #[must_use]
    pub const fn error(&self) -> &ErrorSnapshot<'_> {
        &self.error
    }
    /// Final joined facts, absent on pre-start refusals.
    #[must_use]
    pub const fn facts(&self) -> Option<&CompleteFacts<'_>> {
        self.facts.as_ref()
    }
}
impl fmt::Debug for CompleteError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompleteError")
            .field("error", &self.error)
            .field("call_id", &self.facts.as_ref().map(CompleteFacts::call_id))
            .finish()
    }
}
