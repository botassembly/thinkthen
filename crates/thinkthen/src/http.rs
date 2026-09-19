//! One HTTP exchange with a backend, retried as `specification/backends.md` says.

use std::fmt;
use std::thread;
use std::time::Duration;

use ureq::Agent;

use crate::edge::Key;
use crate::failure::Failure;

/// The statuses a backend is asked again after.
const RETRIED: [u16; 6] = [429, 500, 502, 503, 504, 529];

/// What one exchange needs, gathered at the edge before anything opens.
pub(crate) struct Exchange<'a> {
    /// Where the body is posted.
    pub(crate) url: &'a str,
    /// The request body the adapter wrote, which carries the evidence.
    pub(crate) body: &'a [u8],
    /// The key, when the backend names a variable that holds one.
    pub(crate) key: Option<&'a Key>,
    /// How long one attempt may take, from connect to the last byte.
    pub(crate) timeout: Duration,
    /// How many times a retried failure is sent again.
    pub(crate) max_retries: u32,
    /// The first wait, which doubles on every retry after it.
    pub(crate) retry_wait: Duration,
}

impl fmt::Debug for Exchange<'_> {
    /// Show what an exchange does and never the evidence or the key it carries.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Exchange")
            .field("url", &self.url)
            .field(
                "body",
                &format_args!("<{} bytes withheld>", self.body.len()),
            )
            .field("key", &self.key.map(|_| "<withheld>"))
            .field("timeout", &self.timeout)
            .field("max_retries", &self.max_retries)
            .field("retry_wait", &self.retry_wait)
            .finish()
    }
}

/// Post the request and hand back the response body the backend answered with.
///
/// # Errors
///
/// Returns [`Failure`] when the backend cannot be reached, when it answers with
/// an error status, or when both still hold after the last retry.
pub(crate) fn post(exchange: &Exchange<'_>) -> Result<Vec<u8>, Failure> {
    let agent: Agent = Agent::config_builder()
        .timeout_global(Some(exchange.timeout))
        .http_status_as_error(false)
        .build()
        .into();
    let mut wait = exchange.retry_wait;
    let mut retries = 0;
    loop {
        let failure = match send(&agent, exchange) {
            Ok(body) => return Ok(body),
            Err(failure) => failure,
        };
        if retries >= exchange.max_retries || !is_retried(&failure) {
            return Err(failure);
        }
        thread::sleep(wait);
        wait = wait.saturating_mul(2);
        retries += 1;
    }
}

/// Post the request once.
fn send(agent: &Agent, exchange: &Exchange<'_>) -> Result<Vec<u8>, Failure> {
    let mut request = agent
        .post(exchange.url)
        .header("content-type", "application/json");
    if let Some(key) = exchange.key {
        request = request.header("authorization", &format!("Bearer {}", key.as_str()));
    }
    let mut response = request
        .send(exchange.body)
        .map_err(|error| Failure::Transport(error.to_string()))?;
    let status = response.status().as_u16();
    if !(200..300).contains(&status) {
        return Err(Failure::Status(status));
    }
    response
        .body_mut()
        .read_to_vec()
        .map_err(|error| Failure::Transport(error.to_string()))
}

/// Say whether this failure earns another attempt.
fn is_retried(failure: &Failure) -> bool {
    match failure {
        Failure::Transport(_) => true,
        Failure::Status(status) => RETRIED.contains(status),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::Exchange;
    use crate::edge::Key;
    use std::time::Duration;

    #[test]
    fn an_exchange_shows_neither_the_key_nor_the_evidence_it_carries() {
        let key = Key::of("sk-secret-value");
        let exchange = Exchange {
            url: "http://127.0.0.1:1/v1",
            body: br#"{"state":"something private"}"#,
            key: Some(&key),
            timeout: Duration::from_secs(30),
            max_retries: 2,
            retry_wait: Duration::from_secs(1),
        };

        let shown = format!("{exchange:?} {key:?}");
        assert!(!shown.contains("sk-secret-value"), "{shown}");
        assert!(!shown.contains("something private"), "{shown}");
        assert!(shown.contains("withheld"), "{shown}");
    }
}
