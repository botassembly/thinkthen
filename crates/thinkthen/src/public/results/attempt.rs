//! Owned facts about one live HTTP attempt, never a recording or a call total.

use std::fmt;

use serde::Serialize;

/// The transport result of one sent request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(test, schemars(rename = "attemptOutcome"))]
pub enum AttemptOutcome {
    /// The HTTP request and bounded response body completed successfully.
    Ok,
    /// The backend returned a non-success HTTP status.
    Status,
    /// The transport or bounded response-body read failed.
    Transport,
}

/// One scoped live send. Completion order can differ from ordinal order.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "attempt"))]
pub struct AttemptObservation {
    ordinal: u64,
    request_sha256: String,
    wall_ms: u64,
    outcome: AttemptOutcome,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_id: Option<String>,
}

impl AttemptObservation {
    pub(crate) fn new(
        ordinal: u64,
        request_sha256: String,
        wall_ms: u64,
        outcome: AttemptOutcome,
        response: (Option<u16>, Option<u64>, Option<String>),
    ) -> Self {
        Self {
            ordinal,
            request_sha256,
            wall_ms,
            outcome,
            status: response.0,
            server_ms: response.1,
            request_id: response.2,
        }
    }

    /// One-based send-mark order within this call or command.
    #[must_use]
    pub const fn ordinal(&self) -> u64 {
        self.ordinal
    }

    /// SHA-256 of the prepared request's existing recording identity.
    #[must_use]
    pub fn request_sha256(&self) -> &str {
        &self.request_sha256
    }

    /// Monotone transport and bounded body-read time, rounded up to milliseconds.
    #[must_use]
    pub const fn wall_ms(&self) -> u64 {
        self.wall_ms
    }

    /// Whether HTTP succeeded, returned a status, or failed in transport.
    #[must_use]
    pub const fn outcome(&self) -> AttemptOutcome {
        self.outcome
    }

    /// HTTP status, when a response was received.
    #[must_use]
    pub const fn status(&self) -> Option<u16> {
        self.status
    }

    /// Backend-reported interval, when one safe header supplied it.
    #[must_use]
    pub const fn server_ms(&self) -> Option<u64> {
        self.server_ms
    }

    /// Screened backend request ID, when one safe header supplied it.
    #[must_use]
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
}

impl fmt::Debug for AttemptObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AttemptObservation")
            .field("ordinal", &self.ordinal)
            .field("request_sha256", &self.request_sha256)
            .field("wall_ms", &self.wall_ms)
            .field("outcome", &self.outcome)
            .field("status", &self.status)
            .field("server_ms", &self.server_ms)
            .field(
                "request_id",
                &self.request_id.as_ref().map(|_| "<withheld>"),
            )
            .finish()
    }
}
