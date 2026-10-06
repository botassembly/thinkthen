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

pub(super) fn send(
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
        Box::new(Attempt::from(Error::Transport(transport(
            &error,
            exchange.url.starts_with("https://"),
        ))))
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
        return Err(Box::new(Attempt {
            failure,
            asked,
            info,
        }));
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
