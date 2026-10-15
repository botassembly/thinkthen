//! One prepared HTTP attempt, retaining headers only as screened scalar facts.

use super::retry::honored;
use super::{
    Attempt, Exchange, MAX_RESPONSE_BYTES, REPLY_BYTES_PER_REQUEST_BYTE, ResponseInfo, Sent,
};
use crate::core::Json;
use crate::engine::error::{Error, TransportKind};
use std::io;
use std::time::Duration;
use ureq::Agent;

pub(super) fn timed(
    agent: &Agent,
    exchange: &Exchange<'_>,
    limit: Duration,
    (invocation, sdk_request_id): (
        &crate::engine::invocation::Invocation,
        &crate::core::SdkRequestId,
    ),
    controls: (bool, crate::core::adapters::ApiType),
    usage: &crate::engine::usage::Counters,
) -> (Result<Sent, Box<Attempt>>, u64) {
    let interval = usage.http_interval();
    let started = std::time::Instant::now();
    let sent = send(agent, exchange, limit, invocation, sdk_request_id, controls);
    drop(interval);
    let wall_ms =
        u64::try_from(started.elapsed().as_nanos().div_ceil(1_000_000).max(1)).unwrap_or(u64::MAX);
    (sent, wall_ms)
}

fn send(
    agent: &Agent,
    exchange: &Exchange<'_>,
    limit: Duration,
    invocation: &crate::engine::invocation::Invocation,
    sdk_request_id: &crate::core::SdkRequestId,
    controls: (bool, crate::core::adapters::ApiType),
) -> Result<Sent, Box<Attempt>> {
    let (refresh, api) = controls;
    let request = agent
        .post(exchange.url)
        .config()
        .timeout_global(Some(limit))
        .build()
        .header("content-type", "application/json")
        .header("user-agent", invocation.user_agent())
        .header("X-ThinkThen-Call-Id", invocation.call_id.as_str())
        .header("X-ThinkThen-Request-Id", sdk_request_id.as_str());
    let request = if refresh {
        request.header("cache-control", "no-cache")
    } else {
        request
    };
    let request = match exchange.key.as_str() {
        "" => request,
        key => request.header("authorization", &format!("Bearer {key}")),
    };
    let mut response = request.send(exchange.body).map_err(|error| {
        let opening = unopened(&error);
        let kind =
            opening.unwrap_or_else(|| transport(&error, exchange.url.starts_with("https://")));
        let mut attempt = Attempt::from(Error::Transport(kind));
        attempt.unopened = opening.is_some();
        Box::new(attempt)
    })?;
    let status = response.status().as_u16();
    let storable = crate::core::cache_control::permits(
        response
            .headers()
            .get_all("cache-control")
            .iter()
            .map(|value| value.as_bytes()),
    );
    let header =
        |name: &str| unique(response.headers(), name).and_then(|value| value.to_str().ok());
    let info = ResponseInfo::of(
        status,
        header("x-envoy-upstream-service-time"),
        header(api.request_id_header()),
        exchange,
    );
    if !(200..300).contains(&status) {
        return Err(refused(response, exchange, api, info));
    }
    let sent = u64::try_from(exchange.body.len()).unwrap_or(u64::MAX);
    let most = MAX_RESPONSE_BYTES.saturating_add(REPLY_BYTES_PER_REQUEST_BYTE.saturating_mul(sent));
    // `ureq` refuses a body of exactly its limit, so it gets one byte more.
    let body = response
        .body_mut()
        .with_config()
        .limit(most.saturating_add(1))
        .read_to_vec()
        .map_err(|error| {
            Box::new(Attempt {
                failure: match error {
                    ureq::Error::BodyExceedsLimit(_) => Error::ReplyTooLarge(most),
                    error => Error::Transport(transport(&error, false)),
                },
                asked: None,
                info: info.clone(),
                unopened: false,
            })
        })?;
    Ok(Sent {
        body,
        info,
        storable,
    })
}

fn unique<'a>(
    headers: &'a ureq::http::HeaderMap,
    name: &str,
) -> Option<&'a ureq::http::HeaderValue> {
    let mut values = headers.get_all(name).iter();
    let first = values.next()?;
    values.next().is_none().then_some(first)
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
pub(super) fn transport(error: &ureq::Error, may_be_handshake: bool) -> TransportKind {
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
pub(super) fn io_transport(error: &io::Error) -> TransportKind {
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

// Opening phase markers delegate to the default connector and resolver.
use ureq::unversioned::resolver::{DefaultResolver, ResolvedSocketAddrs, Resolver};
use ureq::unversioned::transport::{
    ConnectionDetails, Connector, DefaultConnector, NextTimeout, Transport,
};

#[derive(Debug, thiserror::Error)]
#[error("connection opening failed")]
struct OpeningFailure(TransportKind);

pub(super) fn unopened(error: &ureq::Error) -> Option<TransportKind> {
    match error {
        ureq::Error::Other(error) => error
            .downcast_ref::<OpeningFailure>()
            .map(|failure| failure.0),
        _ => None,
    }
}

fn marked(error: ureq::Error) -> ureq::Error {
    let eligible = matches!(
        &error,
        ureq::Error::Timeout(_) | ureq::Error::HostNotFound | ureq::Error::ConnectionFailed
    ) || matches!(&error, ureq::Error::Io(error) if error.kind() == std::io::ErrorKind::ConnectionRefused);
    if eligible {
        ureq::Error::Other(Box::new(OpeningFailure(transport(&error, false))))
    } else {
        error
    }
}

#[derive(Debug, Default)]
pub(super) struct OpeningConnector(DefaultConnector);

impl Connector for OpeningConnector {
    type Out = Box<dyn Transport>;

    fn connect(
        &self,
        details: &ConnectionDetails,
        chained: Option<()>,
    ) -> Result<Option<Self::Out>, ureq::Error> {
        self.0.connect(details, chained).map_err(marked)
    }
}

#[derive(Debug, Default)]
pub(super) struct OpeningResolver(DefaultResolver);

impl Resolver for OpeningResolver {
    fn resolve(
        &self,
        uri: &ureq::http::Uri,
        config: &ureq::config::Config,
        timeout: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        self.0.resolve(uri, config, timeout).map_err(marked)
    }
}

/// Read one refused response and retain only screened diagnostics and retry scalars.
fn refused(
    mut response: ureq::http::Response<ureq::Body>,
    exchange: &Exchange<'_>,
    api: crate::core::adapters::ApiType,
    info: ResponseInfo,
) -> Box<Attempt> {
    let status = response.status().as_u16();
    let header =
        |name: &str| unique(response.headers(), name).and_then(|value| value.to_str().ok());
    let plain = header("content-type")
        .and_then(|value| value.split(';').next())
        .is_some_and(|value| value.trim().eq_ignore_ascii_case("text/plain"));
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
    let failure = super::diagnostic::decorate(failure, body.as_deref(), exchange, api, plain);
    Box::new(Attempt {
        failure,
        asked,
        info,
        unopened: false,
    })
}

/// Classify the posting address solely for optional refusal advice.
pub(super) fn ollama_address(url: &str) -> bool {
    url.parse::<ureq::http::Uri>().is_ok_and(|uri| {
        uri.port_u16() == Some(11434)
            || uri
                .host()
                .is_some_and(|host| host.eq_ignore_ascii_case("ollama"))
    })
}

#[cfg(test)]
mod tests {
    //! Stays: transport retry timing, cancellation and address diagnostics need direct fault and secrecy checks.
    #[test]
    fn ollama_address_matches_only_the_host_or_port() {
        for (url, expected) in [
            ("http://127.0.0.1:11434/v1/systemone", true),
            ("http://[::1]:11434/v1/systemone", true),
            ("https://OLLAMA:443/v1/systemone", true),
            ("https://ollama/v1/systemone", true),
            ("http://127.0.0.1:11435/v1/ollama", false),
            ("https://ollama.example.org/v1", false),
            ("https://notollama/v1", false),
            ("https://example.org/ollama:11434", false),
        ] {
            assert_eq!(super::ollama_address(url), expected, "{url}");
        }
    }
}
