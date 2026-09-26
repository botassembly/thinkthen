//! One HTTP exchange with a backend, retried as `specification/backends.md` says.
//!
//! A retried status is sent again. A transport failure never is, because the
//! backend may already have the request and may bill it.

use std::fmt;
use std::io;
use std::time::Duration;

use ureq::Agent;

use crate::core::{Json, Withheld};
use crate::engine::Widths;
use crate::engine::error::{Error, TransportKind};

/// The key one request carries. Diagnostics and `Debug` never expose it.
pub(crate) struct Key(String);

impl Key {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    #[cfg(test)]
    pub(crate) fn of(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl fmt::Debug for Key {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Key(<withheld>)")
    }
}

/// The reply bytes every request may earn, whatever its size.
const MAX_RESPONSE_BYTES: u64 = 1024 * 1024;

/// The reply bytes each request byte adds: the worst honest reply runs about
/// five times its request (ticket 0132), and the bound stops an endless one.
const REPLY_BYTES_PER_REQUEST_BYTE: u64 = 8;

/// The most reply bytes one request may earn.
fn reply_limit(request: &[u8]) -> u64 {
    let request = u64::try_from(request.len()).unwrap_or(u64::MAX);
    MAX_RESPONSE_BYTES.saturating_add(REPLY_BYTES_PER_REQUEST_BYTE.saturating_mul(request))
}

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
pub(crate) struct Client {
    agent: Agent,
    timeout: Duration,
    width: &'static Widths,
}

impl fmt::Debug for Client {
    /// Name the pool and show nothing of what has travelled through it.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Client(<pool>)")
    }
}

impl Client {
    /// Build the one pool this process posts through.
    ///
    /// `secure` says whether the resolved address is an `https://` one. A
    /// plain `http://` address is a loopback one, because the address rule
    /// refuses every other host under it, so no proxy carries that request:
    /// a proxy would send the key and the evidence to another machine in
    /// clear text. `ureq` reads `ALL_PROXY`, `HTTPS_PROXY`, `HTTP_PROXY`, and
    /// `NO_PROXY` on its own, and `proxy(None)` cancels all four.
    pub(crate) fn new(timeout: Duration, secure: bool, widths: &'static Widths) -> Self {
        let mut config = Agent::config_builder()
            .timeout_global(Some(timeout))
            .http_status_as_error(false)
            .max_redirects(0);
        if !secure {
            config = config.proxy(None);
        }
        Self {
            agent: config.build().into(),
            timeout,
            width: crate::engine::client_width(widths),
        }
    }

    /// Send through this gate in place of the one `new` chose.
    #[cfg(test)]
    pub(crate) const fn gated(mut self, width: &'static Widths) -> Self {
        self.width = width;
        self
    }

    /// Post the request and hand back the response body the backend answered with.
    ///
    /// No redirect is followed. A key and the evidence go to the resolved URL
    /// and nowhere else, so a redirect comes back as the status it carries and
    /// fails like any other error status.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when the backend cannot be reached, when it answers
    /// with an error status, or when a retried status still holds after the
    /// last retry. A transport failure returns after its one attempt.
    pub(crate) fn post_observed(
        &self,
        exchange: &Exchange<'_>,
        cancel: &crate::engine::Cancel,
        before_attempt: impl Fn(),
    ) -> Result<HttpAnswer, Error> {
        let mut wait = exchange.retry_wait;
        let mut retries = 0;
        loop {
            // The permit covers the attempt alone: never a retry wait, decoding,
            // recording, or output.
            let permit = self.width.acquire(cancel)?;
            cancel.stop_or_remaining()?;
            before_attempt();
            // Accounting may wait on the usage lock, so read the budget after
            // it. Cancellation keeps its one pre-attempt checkpoint.
            let budget = cancel.remaining()?;
            let limit = budget.map_or(self.timeout, |budget| budget.min(self.timeout));
            let sending = cancel.sending();
            let sent = send(&self.agent, exchange, limit);
            drop((sending, permit));
            let attempt = match sent {
                Ok(body) => {
                    return Ok(HttpAnswer {
                        body,
                        requests_sent: u64::from(retries) + 1,
                    });
                }
                Err(attempt) => attempt,
            };
            if matches!(attempt.failure, Error::Transport(TransportKind::Timeout))
                && budget.is_some_and(|budget| budget <= self.timeout)
                && let Some(passed) = cancel.passed()
            {
                return Err(passed);
            }
            if retries >= exchange.max_retries || !is_retried(&attempt.failure) {
                return Err(attempt.failure);
            }
            if let Some(stop) = cancel.wait(bounded_wait(attempt.asked, wait, self.timeout)) {
                return Err(stop);
            }
            wait = wait.saturating_mul(2);
            retries += 1;
        }
    }
}

/// One successful HTTP reply and every attempt that produced it.
pub(crate) struct HttpAnswer {
    pub(crate) body: Vec<u8>,
    pub(crate) requests_sent: u64,
}

/// What one exchange needs, gathered at the edge before anything opens.
pub(crate) struct Exchange<'a> {
    /// Where the body is posted.
    pub(crate) url: &'a str,
    /// The request body the adapter wrote, which carries the evidence.
    pub(crate) body: &'a [u8],
    /// The key the one authorization header carries.
    pub(crate) key: &'a Key,
    /// How many times a retried status is sent again.
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
            .field("body", &Withheld(self.body.len()))
            .field("key", &self.key)
            .field("max_retries", &self.max_retries)
            .field("retry_wait", &self.retry_wait)
            .finish()
    }
}

/// What one attempt failed with, and how long the backend asked this one to wait.
#[derive(Debug)]
struct Attempt {
    failure: Error,
    asked: Option<Duration>,
}

impl From<Error> for Attempt {
    /// A failure with no header behind it waits the doubling wait.
    fn from(failure: Error) -> Self {
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

/// Bound either retry-wait source by the public per-attempt timeout.
fn bounded_wait(asked: Option<Duration>, exponential: Duration, timeout: Duration) -> Duration {
    asked.unwrap_or(exponential).min(timeout)
}

/// Post the request once, blocking for at most `limit`.
fn send(agent: &Agent, exchange: &Exchange<'_>, limit: Duration) -> Result<Vec<u8>, Attempt> {
    let request = agent
        .post(exchange.url)
        .config()
        .timeout_global(Some(limit))
        .build()
        .header("content-type", "application/json")
        .header(
            "authorization",
            &format!("Bearer {}", exchange.key.as_str()),
        );
    let mut response = request
        .send(exchange.body)
        .map_err(|error| Attempt::from(Error::Transport(transport(&error))))?;
    let status = response.status().as_u16();
    if !(200..300).contains(&status) {
        let header = |name: &str| {
            response
                .headers()
                .get(name)
                .and_then(|value| value.to_str().ok())
        };
        let asked = honored(header("retry-after-ms"), header("retry-after"));
        let failure = if status == 400 && names_token_limit(&mut response) {
            Error::TokenLimit
        } else {
            Error::Status(status)
        };
        return Err(Attempt { failure, asked });
    }
    // `ureq` refuses a body of exactly its limit, so it gets one byte more.
    let most = reply_limit(exchange.body);
    response
        .body_mut()
        .with_config()
        .limit(most.saturating_add(1))
        .read_to_vec()
        .map_err(|error| match error {
            ureq::Error::BodyExceedsLimit(_) => Attempt::from(Error::ReplyTooLarge(most)),
            error => Attempt::from(Error::Transport(transport(&error))),
        })
}

/// Whether a 400 reply's body names `max_tokens_exceeded` as its
/// `detail.error_type`. The body is read up to 4 KiB and never kept: an
/// unreadable, longer, or other body answers no, and the caller keeps status 400.
fn names_token_limit(response: &mut ureq::http::Response<ureq::Body>) -> bool {
    let Ok(body) = response
        .body_mut()
        .with_config()
        .limit(BODY_REASON_BYTES)
        .read_to_vec()
    else {
        return false;
    };
    let Ok(text) = std::str::from_utf8(&body) else {
        return false;
    };
    Json::parse(text).ok().is_some_and(|value| {
        value
            .member("detail")
            .and_then(|detail| detail.member("error_type"))
            .and_then(Json::as_str)
            == Some("max_tokens_exceeded")
    })
}

/// How much of a 400 reply's body is read for its reason.
const BODY_REASON_BYTES: u64 = 4096;

/// Reduce an HTTP-library error to the safe class the command contract knows.
fn transport(error: &ureq::Error) -> TransportKind {
    match error {
        ureq::Error::Timeout(_) => TransportKind::Timeout,
        ureq::Error::HostNotFound => TransportKind::NameLookup,
        ureq::Error::Io(error) => io_transport(error),
        _ => TransportKind::Other,
    }
}

/// Reduce an operating-system I/O error without preserving its text.
fn io_transport(error: &io::Error) -> TransportKind {
    match error.kind() {
        io::ErrorKind::ConnectionRefused => TransportKind::Refused,
        io::ErrorKind::UnexpectedEof
        | io::ErrorKind::ConnectionReset
        | io::ErrorKind::ConnectionAborted
        | io::ErrorKind::BrokenPipe => TransportKind::PrematureClose,
        io::ErrorKind::TimedOut => TransportKind::Timeout,
        _ => TransportKind::Other,
    }
}

/// Say whether this failure earns another attempt: only a retried status does.
fn is_retried(failure: &Error) -> bool {
    failure.retryable()
}

#[cfg(test)]
mod tests {
    use super::{
        Client, Exchange, Key, bounded_wait, honored, io_transport, is_retried, transport,
    };
    use crate::engine::error::{Error, TransportKind};
    use std::cell::Cell;
    use std::io::{self, Read as _, Write as _};
    use std::net::TcpListener;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;
    use std::time::Duration;

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

    #[test]
    fn either_retry_wait_source_stops_at_the_attempt_timeout() {
        let timeout = Duration::from_secs(2);
        assert_eq!(
            bounded_wait(
                Some(Duration::from_secs(30)),
                Duration::from_secs(1),
                timeout
            ),
            timeout
        );
        assert_eq!(bounded_wait(None, Duration::from_secs(4), timeout), timeout);
        assert_eq!(
            bounded_wait(
                Some(Duration::from_millis(250)),
                Duration::from_secs(4),
                timeout
            ),
            Duration::from_millis(250)
        );
    }

    #[test]
    fn structured_transport_errors_map_without_reading_their_display_text() {
        let cases = [
            (
                ureq::Error::Timeout(ureq::Timeout::Global),
                TransportKind::Timeout,
            ),
            (ureq::Error::HostNotFound, TransportKind::NameLookup),
            (
                ureq::Error::Io(io::Error::new(
                    io::ErrorKind::ConnectionRefused,
                    "hostile refused text",
                )),
                TransportKind::Refused,
            ),
            (
                ureq::Error::Io(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "hostile close text",
                )),
                TransportKind::PrematureClose,
            ),
            (ureq::Error::ConnectionFailed, TransportKind::Other),
        ];
        for (error, expected) in cases {
            assert_eq!(transport(&error), expected, "{error:?}");
        }
        for kind in [
            io::ErrorKind::ConnectionReset,
            io::ErrorKind::ConnectionAborted,
            io::ErrorKind::BrokenPipe,
        ] {
            assert_eq!(
                io_transport(&io::Error::new(kind, "hostile close text")),
                TransportKind::PrematureClose
            );
        }
    }

    #[test]
    fn no_transport_failure_is_sent_again() {
        let cases = [
            (Error::Transport(TransportKind::Refused), false),
            (Error::Transport(TransportKind::Timeout), false),
            (Error::Transport(TransportKind::NameLookup), false),
            (Error::Transport(TransportKind::PrematureClose), false),
            (Error::Transport(TransportKind::Other), false),
            (Error::Status(429), true),
            (Error::Status(500), true),
            (Error::Status(502), true),
            (Error::Status(503), true),
            (Error::Status(504), true),
            (Error::Status(529), true),
            (Error::Status(401), false),
        ];
        for (failure, expected) in cases {
            assert_eq!(is_retried(&failure), expected, "{failure:?}");
        }
    }

    #[test]
    #[allow(clippy::excessive_nesting, reason = "synchronized server fixture")]
    fn cancellation_during_a_retry_wait_starts_no_second_attempt() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
        let address = listener.local_addr().expect("address");
        let received = Arc::new(AtomicUsize::new(0));
        let counted = Arc::clone(&received);
        let cancel = crate::engine::Cancel::default();
        let server_cancel = cancel.clone();
        let server = thread::spawn(move || {
            for stream in listener.incoming().take(2) {
                let mut stream = stream.expect("request");
                let mut request = [0_u8; 1024];
                let _read = stream.read(&mut request).expect("request bytes");
                counted.fetch_add(1, Ordering::SeqCst);
                stream
                    .write_all(b"HTTP/1.1 503 Unavailable\r\nContent-Length: 0\r\n\r\n")
                    .expect("response");
                if server_cancel.fired() {
                    break;
                }
            }
        });
        let url = format!("http://{address}/v1/systemone");
        let key = Key::of("sk-test-value");
        let client = Client::new(
            Duration::from_secs(2),
            false,
            crate::engine::process_width(),
        );
        let attempts = Cell::new(0_u32);
        let exchange = Exchange {
            url: &url,
            body: b"{}",
            key: &key,
            max_retries: 1,
            retry_wait: Duration::from_secs(1),
        };

        let result = client.post_observed(&exchange, &cancel, || {
            attempts.set(attempts.get() + 1);
            cancel.fire();
        });
        server.join().expect("server thread");

        assert!(matches!(result, Err(Error::Cancelled)));
        assert_eq!(attempts.get(), 1);
        assert_eq!(received.load(Ordering::SeqCst), 1);
    }

    /// Port zero can never listen, so the refusal is deterministic.
    #[test]
    fn a_refused_attempt_is_observed_once_and_returned_without_a_retry() {
        let key = Key::of("sk-test-value");
        let client = Client::new(
            Duration::from_secs(4),
            false,
            crate::engine::process_width(),
        );
        let observed = Cell::new(0_u32);
        let exchange = Exchange {
            url: "http://127.0.0.1:0/v1/systemone",
            body: b"{}",
            key: &key,
            max_retries: 2,
            retry_wait: Duration::from_millis(1),
        };

        let result = client.post_observed(&exchange, &crate::engine::Cancel::default(), || {
            observed.set(observed.get() + 1)
        });

        assert_eq!(observed.get(), 1);
        assert!(matches!(
            result,
            Err(Error::Transport(TransportKind::Refused))
        ));
    }
}
