//! One HTTP exchange with a backend, retried as `specification/backends.md` says.
//!
//! A retried status is sent again. A transport failure never is, because the
//! backend may already have the request and may bill it.

use std::fmt;
use std::io;
use std::path::Path;
use std::time::{Duration, Instant};

use ureq::Agent;
use ureq::tls::{PemItem, RootCerts};

use crate::core::{Json, Withheld};
use crate::engine::error::{Error, TransportKind};
use crate::engine::roots::{self, Error as RootsError};
use crate::engine::usage::Counters;
use crate::engine::{Permit, Width, Widths, backoff};

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

/// Parsed replacement trust roots, retained without their source file.
#[derive(Clone)]
pub(crate) struct Roots(RootCerts);

impl Roots {
    pub(crate) fn load(path: &Path) -> Result<Self, RootsError> {
        let bundle = roots::read(path)?;
        let mut certs = Vec::with_capacity(bundle.count);
        for item in ureq::tls::parse_pem(&bundle.bytes) {
            let Ok(PemItem::Certificate(cert)) = item else {
                return Err(RootsError::Usage(
                    "THINKTHEN_CA_BUNDLE has a malformed PEM certificate",
                ));
            };
            certs.push(cert);
        }
        if certs.len() != bundle.count {
            return Err(RootsError::Usage(
                "THINKTHEN_CA_BUNDLE has a malformed PEM certificate",
            ));
        }
        Ok(Self(RootCerts::new_with_certs(&certs)))
    }

    pub(crate) fn configured(&self) -> RootCerts {
        self.0.clone()
    }
}

/// The reply bytes every request may earn, whatever its size.
const MAX_RESPONSE_BYTES: u64 = 1024 * 1024;

/// Reply bytes per request byte. The worst honest reply runs about 5 reply bytes per request byte (ticket 0132).
const REPLY_BYTES_PER_REQUEST_BYTE: u64 = 8;

/// The longest unheaded exponential wait before an attempt is allowed through.
const MAX_RETRY_WAIT: Duration = Duration::from_secs(60);
const MIN_HEADER_WAIT: Duration = Duration::from_secs(1);

/// One connection pool, built once and shared by every worker.
///
/// It keeps an idle connection for each request the widest throttle allows, so
/// a run pays for one handshake per job rather than one per record (ticket 0142).
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
    fn acquire_open<'a>(
        &'a self,
        gates: &backoff::Gates,
        url: &str,
        cap: Instant,
        cancel: &crate::engine::Cancel,
    ) -> Result<Permit<'a>, Error> {
        loop {
            let _open = gates.wait_open(url, cap, cancel)?;
            let permit = self.width.acquire(cancel)?;
            if gates.may_send(url, cap) {
                return Ok(permit);
            }
            drop(permit);
        }
    }

    /// Build the one pool this process posts through.
    ///
    /// `secure` says whether the resolved address is an `https://` one. A
    /// plain `http://` address is a loopback one, because the address rule
    /// refuses every other host under it, so no proxy carries that request:
    /// a proxy would send the key and the evidence to another machine in
    /// clear text. `ureq` reads `ALL_PROXY`, `HTTPS_PROXY`, `HTTP_PROXY`, and
    /// `NO_PROXY` on its own, and `proxy(None)` cancels all four.
    pub(crate) fn new(timeout: Duration, secure: bool, widths: &'static Widths) -> Self {
        Self::with_roots(timeout, secure, widths, None)
    }

    /// Build a pool with one parsed replacement trust snapshot when selected.
    pub(crate) fn with_roots(
        timeout: Duration,
        secure: bool,
        widths: &'static Widths,
        roots: Option<&Roots>,
    ) -> Self {
        let mut config = Agent::config_builder()
            .timeout_global(Some(timeout))
            .http_status_as_error(false)
            .max_redirects(0)
            .max_idle_connections(Width::MOST.get())
            .max_idle_connections_per_host(Width::MOST.get());
        if let Some(roots) = roots {
            config = config.tls_config(
                ureq::tls::TlsConfig::builder()
                    .root_certs(roots.configured())
                    .build(),
            );
        }
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
    #[cfg(test)]
    pub(crate) fn post_observed(
        &self,
        exchange: &Exchange<'_>,
        cancel: &crate::engine::Cancel,
        before_attempt: impl Fn(),
    ) -> Result<HttpAnswer, Error> {
        let usage = Counters::new(None);
        self.post_observed_with_retry(exchange, cancel, &usage, |_| before_attempt())
    }

    #[cfg(test)]
    pub(crate) fn post_observed_with_retry(
        &self,
        exchange: &Exchange<'_>,
        cancel: &crate::engine::Cancel,
        usage: &Counters,
        before_attempt: impl Fn(bool),
    ) -> Result<HttpAnswer, Error> {
        self.post_marked_with_retry(exchange, cancel, usage, before_attempt, || ())
    }

    /// Also report each actual transport start to one private request owner.
    pub(crate) fn post_marked_with_retry(
        &self,
        exchange: &Exchange<'_>,
        cancel: &crate::engine::Cancel,
        usage: &Counters,
        before_attempt: impl Fn(bool),
        marked: impl Fn(),
    ) -> Result<HttpAnswer, Error> {
        if exchange
            .key
            .as_str()
            .bytes()
            .any(|byte| byte == b'\n' || byte == b'\r')
        {
            return Err(Error::Usage("the API key contains a line break"));
        }
        let gates = backoff::process_gates(cancel)?;
        let mut wait = exchange.retry_wait;
        let mut retries = 0;
        let mut last_status = None;
        loop {
            let now = Instant::now();
            let cap = now + self.timeout.min(MAX_RETRY_WAIT);
            // A gate wait owns no send slot. Recheck after acquiring one, since
            // another in-flight reply could have closed the gate meanwhile.
            let permit = self.acquire_open(gates, exchange.url, cap, cancel)?;
            cancel.stop_or_remaining()?;
            before_attempt(retries > 0);
            let prepared = usage.prepare_attempt()?;
            // Bookkeeping and the test hook may wait. The final check reads no
            // host callback while the usage lock is held.
            let budget = cancel.remaining_without_check()?;
            let limit = budget.map_or(self.timeout, |budget| budget.min(self.timeout));
            let reservation = cancel.reserve_send(last_status)?;
            prepared.mark(retries > 0)?;
            if let Some(reservation) = reservation {
                reservation.commit();
            }
            cancel.sent();
            marked();
            let sending = cancel.sending();
            let sent = send(&self.agent, exchange, limit);
            drop(sending);
            if let Err(attempt) = &sent
                && is_retried(&attempt.failure)
            {
                permit.release_closing(
                    gates,
                    exchange.url,
                    bounded_wait(attempt.asked, wait, self.timeout),
                    attempt.asked.is_some(),
                );
            } else {
                drop(permit);
            }
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
            last_status = match attempt.failure {
                Error::Status(status) => Some(status),
                _ => None,
            };
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
    /// Name the exchange and withhold its address, evidence, and key.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Exchange")
            .field("url", &"<withheld>")
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

/// The wait the two retry headers ask for, with zero given a one-second floor.
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
    Some(if wait.is_zero() {
        MIN_HEADER_WAIT
    } else {
        wait
    })
}

/// The server's valid delay is a floor; cap only an unheaded local wait.
fn bounded_wait(asked: Option<Duration>, exponential: Duration, timeout: Duration) -> Duration {
    asked.unwrap_or_else(|| exponential.min(timeout).min(MAX_RETRY_WAIT))
}

/// Post the request once, blocking for at most `limit`.
fn send(agent: &Agent, exchange: &Exchange<'_>, limit: Duration) -> Result<Vec<u8>, Attempt> {
    let request = agent
        .post(exchange.url)
        .config()
        .timeout_global(Some(limit))
        .build()
        .header("content-type", "application/json");
    let request = match exchange.key.as_str() {
        "" => request,
        key => request.header("authorization", &format!("Bearer {key}")),
    };
    let mut response = request.send(exchange.body).map_err(|error| {
        Attempt::from(Error::Transport(transport(
            &error,
            exchange.url.starts_with("https://"),
        )))
    })?;
    let status = response.status().as_u16();
    if !(200..300).contains(&status) {
        let header = |name: &str| {
            response
                .headers()
                .get(name)
                .and_then(|value| value.to_str().ok())
        };
        let asked = honored(header("retry-after-ms"), header("retry-after"));
        let body = response
            .body_mut()
            .with_config()
            .limit(MAX_RESPONSE_BYTES)
            .read_to_vec()
            .ok();
        let failure = if status == 400 && body.as_deref().is_some_and(names_token_limit) {
            Error::TokenLimit
        } else {
            Error::Status(status)
        };
        return Err(Attempt { failure, asked });
    }
    let sent = u64::try_from(exchange.body.len()).unwrap_or(u64::MAX);
    let most = MAX_RESPONSE_BYTES.saturating_add(REPLY_BYTES_PER_REQUEST_BYTE.saturating_mul(sent));
    // `ureq` refuses a body of exactly its limit, so it gets one byte more.
    response
        .body_mut()
        .with_config()
        .limit(most.saturating_add(1))
        .read_to_vec()
        .map_err(|error| match error {
            ureq::Error::BodyExceedsLimit(_) => Attempt::from(Error::ReplyTooLarge(most)),
            error => Attempt::from(Error::Transport(transport(&error, false))),
        })
}

/// Whether a 400 reply's body names `max_tokens_exceeded` as its
/// `detail.error_type`. The body is read up to 4 KiB and never kept: an
/// unreadable, longer, or other body answers no, and the caller keeps status 400.
fn names_token_limit(body: &[u8]) -> bool {
    if body.len() > BODY_REASON_BYTES as usize {
        return false;
    }
    let Ok(text) = std::str::from_utf8(body) else {
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
/// Only a secure request still opening its response can wrap a rustls handshake
/// failure as `Io(InvalidData)`. A body read has already passed the handshake.
fn transport(error: &ureq::Error, may_be_handshake: bool) -> TransportKind {
    match error {
        ureq::Error::Timeout(_) => TransportKind::Timeout,
        ureq::Error::HostNotFound => TransportKind::NameLookup,
        ureq::Error::Tls(_) | ureq::Error::Rustls(_) => TransportKind::Tls,
        ureq::Error::Io(error)
            if may_be_handshake && error.kind() == io::ErrorKind::InvalidData =>
        {
            TransportKind::Tls
        }
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
mod tests;
