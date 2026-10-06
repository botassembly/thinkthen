//! One HTTP exchange with a backend, retried as `specification/backends.md` says.
//!
//! A retried status is sent again. A transport failure never is, because the
//! backend may already have the request and may bill it.

use std::fmt;
use std::path::Path;
use std::time::{Duration, Instant};

use ureq::Agent;
use ureq::tls::{PemItem, RootCerts};

use crate::core::Withheld;
use crate::engine::error::{Error, TransportKind};
use crate::engine::roots::{self, Error as RootsError};
use crate::engine::usage::Counters;
use crate::engine::{Permit, Width, Widths, backoff};

mod observation;
mod retry;
mod send;
use observation::{ResponseInfo, observed_result};
#[cfg(test)]
use retry::honored;
use retry::{bounded_wait, draw, floor, longest, too_long};
use send::send;
#[cfg(test)]
use send::{io_transport, transport};

/// The key one request carries. Diagnostics and `Debug` never expose it.
pub(crate) struct Key(String);

impl Key {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn check_control(&self) -> Result<(), Error> {
        if self.0.chars().any(char::is_control) {
            Err(Error::Usage("the API key contains a control character"))
        } else {
            Ok(())
        }
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

/// One connection pool, built once and shared by every worker.
///
/// It keeps an idle connection for each request the widest throttle allows, so
/// a run pays for one handshake per job rather than one per record (ticket 0142).
/// It reuses no connection idle for a second or more. Common keep-alive waits run longer (0341).
pub(crate) struct Client {
    api: crate::core::adapters::ApiType,
    agent: Agent,
    timeout: Duration,
    width: &'static Widths,
    /// The spacing between attempt starts to one address, or none.
    every: Option<Duration>,
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
            let _open = gates.wait_open(url, (cap, longest(self.timeout)), cancel)?;
            if let Some(every) = self.every {
                gates.pace(url, every, cancel)?;
            }
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
            .max_idle_connections_per_host(Width::MOST.get())
            .max_idle_age(Duration::from_secs(1));
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
            api: Default::default(),
            agent: config.build().into(),
            timeout,
            width: crate::engine::limits::client_width(widths),
            every: None,
        }
    }

    pub(crate) const fn with_api(mut self, api: crate::core::adapters::ApiType) -> Self {
        self.api = api;
        self
    }

    /// Space attempt starts to each address by `every`.
    pub(crate) const fn paced(mut self, every: Option<Duration>) -> Self {
        self.every = every;
        self
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
        self.post_with_reservation_check(exchange, cancel, usage, (before_attempt, || (), marked))
    }

    #[cfg(test)]
    pub(crate) fn post_observed_after_reservation(
        &self,
        exchange: &Exchange<'_>,
        cancel: &crate::engine::Cancel,
        usage: &Counters,
        after_reservation: impl Fn(),
    ) -> Result<HttpAnswer, Error> {
        self.post_with_reservation_check(
            exchange,
            cancel,
            usage,
            (|_| (), after_reservation, || ()),
        )
    }

    fn post_with_reservation_check(
        &self,
        exchange: &Exchange<'_>,
        cancel: &crate::engine::Cancel,
        usage: &Counters,
        hooks: (impl Fn(bool), impl Fn(), impl Fn()),
    ) -> Result<HttpAnswer, Error> {
        let (before_attempt, after_reservation, marked) = hooks;
        exchange.key.check_control()?;
        let invocation = cancel.invocation()?;
        let sdk_request_id = crate::engine::invocation::request_id()?;
        let gates = &crate::engine::limits::of(std::process::id(), cancel)?.gates;
        let mut wait = exchange.retry_wait;
        let mut retries = 0;
        let mut last_status = None;
        loop {
            let now = Instant::now();
            let cap = now + longest(self.timeout);
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
            let reservation = cancel.reserve_send(last_status, exchange.body.len())?;
            after_reservation();
            cancel.remaining_without_check()?;
            prepared.mark(retries > 0)?;
            if let Some(reservation) = reservation {
                reservation.commit();
            }
            cancel.sent();
            let ordinal = cancel.attempt_started();
            marked();
            let sending = cancel.sending();
            let started = Instant::now();
            let sent = send(
                &self.agent,
                exchange,
                limit,
                invocation,
                &sdk_request_id,
                (cancel.cache_refresh, self.api),
            );
            let wall_ms = u64::try_from(started.elapsed().as_nanos().div_ceil(1_000_000).max(1))
                .unwrap_or(u64::MAX);
            drop(sending);
            if let (Some(ordinal), Some(digest)) = (ordinal, cancel.attempt_digest()) {
                let (info, outcome) = observed_result(&sent);
                cancel.attempt_completed(info.clone().observation(
                    ordinal,
                    digest,
                    &sdk_request_id,
                    wall_ms,
                    outcome,
                ));
            }
            if let Err(attempt) = &sent
                && is_retried(&attempt.failure)
            {
                permit.release_closing(
                    gates,
                    exchange.url,
                    bounded_wait(attempt.asked, wait, self.timeout, draw()),
                    floor(attempt.asked, &attempt.failure),
                );
            } else {
                drop(permit);
            }
            let attempt = match sent {
                Ok(answer) => {
                    return Ok(HttpAnswer {
                        body: answer.body,
                        storable: answer.storable,
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
            if retries >= exchange.max_retries
                || !is_retried(&attempt.failure)
                || too_long(attempt.asked, self.timeout)
            {
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
    pub(crate) storable: bool,
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
    info: ResponseInfo,
}

impl From<Error> for Attempt {
    /// A failure with no header behind it waits the doubling wait.
    fn from(failure: Error) -> Self {
        Self {
            failure,
            asked: None,
            info: ResponseInfo::default(),
        }
    }
}

/// Post the request once, blocking for at most `limit`.
struct Sent {
    body: Vec<u8>,
    info: ResponseInfo,
    storable: bool,
}

/// Say whether this failure earns another attempt: only a retried status does.
fn is_retried(failure: &Error) -> bool {
    failure.retryable()
}

#[cfg(test)]
mod tests;
