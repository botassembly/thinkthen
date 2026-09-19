//! One HTTP exchange with a backend, retried as `specification/backends.md` says.

use std::fmt;
use std::thread;
use std::time::Duration;

use ureq::Agent;

use crate::edge::Key;
use crate::failure::Failure;

/// The most of one response body an attempt reads before it gives up.
///
/// A judgment answers in well under a kilobyte, so a megabyte is generous by a
/// thousandfold. The bound is here so a server that never stops writing cannot
/// fill this process's memory.
const MAX_RESPONSE_BYTES: u64 = 1024 * 1024;

/// The statuses a backend is asked again after.
const RETRIED: [u16; 6] = [429, 500, 502, 503, 504, 529];

/// The longest a `Retry-After` header moves the wait to.
///
/// The header is the backend's own number and this process trusts it only so
/// far. A backend asking for an hour would hang a script with no way out but
/// a signal, so the wait stops here and the attempt goes out.
const MAX_RETRY_WAIT: Duration = Duration::from_secs(60);

/// One connection pool, built once and shared by every worker.
///
/// A pool keeps a connection open between requests, so a run over many records
/// pays for one handshake rather than one per record. `ureq` shares an agent
/// across threads, so the workers hold one of these between them.
pub(crate) struct Client(Agent);

impl fmt::Debug for Client {
    /// Name the pool and show nothing of what has travelled through it.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Client(<pool>)")
    }
}

impl Client {
    /// Build the one pool this process posts through.
    pub(crate) fn new(timeout: Duration) -> Self {
        Self(
            Agent::config_builder()
                .timeout_global(Some(timeout))
                .http_status_as_error(false)
                .max_redirects(0)
                .build()
                .into(),
        )
    }

    /// Post the request and hand back the response body the backend answered with.
    ///
    /// No redirect is followed. A key and the evidence go to the resolved URL
    /// and nowhere else, so a redirect comes back as the status it carries and
    /// fails like any other error status.
    ///
    /// # Errors
    ///
    /// Returns [`Failure`] when the backend cannot be reached, when it answers
    /// with an error status, or when both still hold after the last retry.
    pub(crate) fn post(&self, exchange: &Exchange<'_>) -> Result<Vec<u8>, Failure> {
        let mut wait = exchange.retry_wait;
        let mut retries = 0;
        loop {
            let attempt = match send(&self.0, exchange) {
                Ok(body) => return Ok(body),
                Err(attempt) => attempt,
            };
            if retries >= exchange.max_retries || !is_retried(&attempt.failure) {
                return Err(attempt.failure);
            }
            thread::sleep(attempt.asked.unwrap_or(wait));
            wait = wait.saturating_mul(2);
            retries += 1;
        }
    }
}

/// What one exchange needs, gathered at the edge before anything opens.
pub(crate) struct Exchange<'a> {
    /// Where the body is posted.
    pub(crate) url: &'a str,
    /// The request body the adapter wrote, which carries the evidence.
    pub(crate) body: &'a [u8],
    /// The key the one authorization header carries.
    pub(crate) key: &'a Key,
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
            .field("key", &self.key)
            .field("max_retries", &self.max_retries)
            .field("retry_wait", &self.retry_wait)
            .finish()
    }
}

/// What one attempt failed with, and how long the backend asked this one to wait.
#[derive(Debug)]
struct Attempt {
    failure: Failure,
    asked: Option<Duration>,
}

impl From<Failure> for Attempt {
    /// A failure with no header behind it waits the doubling wait.
    fn from(failure: Failure) -> Self {
        Self {
            failure,
            asked: None,
        }
    }
}

/// The wait the two retry headers ask for, capped at [`MAX_RETRY_WAIT`].
///
/// The backend sends `retry-after-ms` in whole milliseconds beside the standard
/// `retry-after` in whole seconds, and the finer one is read first.
/// `specification/backends.md` takes those two forms alone. The HTTP-date form
/// of `retry-after` needs a clock and a date reader, and neither belongs here.
fn honored(millis: Option<&str>, seconds: Option<&str>) -> Option<Duration> {
    let asked = |header: Option<&str>| header?.trim().parse::<u64>().ok();
    let wait = match asked(millis) {
        Some(number) => Duration::from_millis(number),
        None => Duration::from_secs(asked(seconds)?),
    };
    Some(wait.min(MAX_RETRY_WAIT))
}

/// Post the request once.
fn send(agent: &Agent, exchange: &Exchange<'_>) -> Result<Vec<u8>, Attempt> {
    let request = agent
        .post(exchange.url)
        .header("content-type", "application/json")
        .header(
            "authorization",
            &format!("Bearer {}", exchange.key.as_str()),
        );
    let mut response = request
        .send(exchange.body)
        .map_err(|error| Attempt::from(Failure::Transport(error.to_string())))?;
    let status = response.status().as_u16();
    if !(200..300).contains(&status) {
        let header = |name: &str| {
            response
                .headers()
                .get(name)
                .and_then(|value| value.to_str().ok())
        };
        let asked = honored(header("retry-after-ms"), header("retry-after"));
        return Err(Attempt {
            failure: Failure::Status(status),
            asked,
        });
    }
    response
        .body_mut()
        .with_config()
        .limit(MAX_RESPONSE_BYTES)
        .read_to_vec()
        .map_err(|error| Attempt::from(Failure::Transport(error.to_string())))
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
    use super::{Exchange, honored};
    use crate::edge::Key;
    use std::time::Duration;

    #[test]
    fn an_exchange_shows_neither_the_key_nor_the_evidence_it_carries() {
        let key = Key::of("sk-secret-value");
        let exchange = Exchange {
            url: "http://127.0.0.1:1/v1",
            body: br#"{"state":"something private"}"#,
            key: &key,
            max_retries: 2,
            retry_wait: Duration::from_secs(1),
        };

        let shown = format!("{exchange:?} {key:?}");
        assert!(!shown.contains("sk-secret-value"), "{shown}");
        assert!(!shown.contains("something private"), "{shown}");
        assert!(shown.contains("withheld"), "{shown}");
    }

    #[test]
    fn a_retry_after_header_is_read_in_seconds_and_stops_at_the_ceiling() {
        let cases = [
            (Some("2"), Some(Duration::from_secs(2))),
            (Some("  7 "), Some(Duration::from_secs(7))),
            (Some("0"), Some(Duration::ZERO)),
            (Some("99999"), Some(Duration::from_secs(60))),
            (Some("Wed, 21 Oct 2026 07:28:00 GMT"), None),
            (Some("-1"), None),
            (Some(""), None),
            (None, None),
        ];
        for (header, expected) in cases {
            assert_eq!(honored(None, header), expected, "{header:?}");
        }
    }

    #[test]
    fn the_milliseconds_header_is_read_first_and_the_seconds_header_follows_it() {
        let cases = [
            (Some("250"), None, Some(Duration::from_millis(250))),
            (Some("1500"), Some("9"), Some(Duration::from_millis(1500))),
            (Some("600000"), None, Some(Duration::from_secs(60))),
            // A milliseconds header nobody can read leaves the seconds one.
            (Some("soon"), Some("3"), Some(Duration::from_secs(3))),
            (Some(""), Some("3"), Some(Duration::from_secs(3))),
            (Some("-5"), None, None),
            (None, None, None),
        ];
        for (millis, seconds, expected) in cases {
            assert_eq!(honored(millis, seconds), expected, "{millis:?} {seconds:?}");
        }
    }
}
