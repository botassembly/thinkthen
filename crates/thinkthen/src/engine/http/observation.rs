//! Allowlisted response metadata for one live attempt.

use crate::core::{AttemptObservation, AttemptOutcome};

use super::{Attempt, Error, Exchange, Sent};

#[derive(Clone, Default)]
pub(super) struct ResponseInfo {
    pub(super) status: Option<u16>,
    pub(super) server_ms: Option<u64>,
    pub(super) request_id: Option<String>,
}

impl std::fmt::Debug for ResponseInfo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ResponseInfo")
            .field("status", &self.status)
            .field("server_ms", &self.server_ms)
            .field(
                "request_id",
                &self.request_id.as_ref().map(|_| "<withheld>"),
            )
            .finish()
    }
}

impl ResponseInfo {
    pub(super) fn of(
        status: u16,
        server: Option<&str>,
        id: Option<&str>,
        exchange: &Exchange<'_>,
    ) -> Self {
        let server_ms = server
            .filter(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
            .and_then(|value| value.parse().ok());
        let request_id = id
            .filter(|value| (1..=128).contains(&value.len()))
            .filter(|value| {
                value
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
            })
            .filter(|value| {
                let key = exchange.key.as_str();
                (key.is_empty() || !value.contains(key))
                    && !exchange.url.contains(value)
                    && !exchange
                        .body
                        .windows(value.len())
                        .any(|part| part == value.as_bytes())
            })
            .map(str::to_owned);
        Self {
            status: Some(status),
            server_ms,
            request_id,
        }
    }

    pub(super) fn observation(
        self,
        ordinal: u64,
        digest: &str,
        wall_ms: u64,
        outcome: AttemptOutcome,
    ) -> AttemptObservation {
        AttemptObservation {
            ordinal,
            request_sha256: digest.to_owned(),
            wall_ms,
            outcome,
            status: self.status,
            server_ms: self.server_ms,
            request_id: self.request_id,
        }
    }
}

pub(super) fn observed_result(
    sent: &Result<Sent, Box<Attempt>>,
) -> (&ResponseInfo, AttemptOutcome) {
    match sent {
        Ok(answer) => (&answer.info, AttemptOutcome::Ok),
        Err(attempt) => {
            let outcome = if matches!(
                attempt.failure,
                Error::Transport(_) | Error::ReplyTooLarge(_)
            ) {
                AttemptOutcome::Transport
            } else {
                AttemptOutcome::Status
            };
            (&attempt.info, outcome)
        }
    }
}
