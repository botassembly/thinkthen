//! Send attempts, status backoff and safe connection retries.

use crate::core::{Error, ErrorKind};

use crate::{Cancel, RetryPolicy};

mod excerpt;
mod response;
mod scope;
mod socket;
#[cfg(test)]
mod socket_tests;
mod tls;

#[derive(Debug)]
pub(crate) struct Agent {
    inner: ureq::Agent,
    unsupported_proxy: Option<&'static str>,
}

pub(crate) fn agent(config: ureq::config::Config) -> Agent {
    let unsupported_proxy = config.proxy().and_then(|proxy| {
        use ureq::ProxyProtocol::{Socks4, Socks4A, Socks5, Socks5h};
        if !matches!(proxy.protocol(), Socks4 | Socks4A | Socks5 | Socks5h) {
            return None;
        }
        // Identify the selected environment setting without retaining its secrets.
        [
            "ALL_PROXY",
            "all_proxy",
            "HTTPS_PROXY",
            "https_proxy",
            "HTTP_PROXY",
            "http_proxy",
        ]
        .into_iter()
        .find(|variable| {
            std::env::var(variable)
                .ok()
                .and_then(|value| ureq::Proxy::new(&value).ok())
                .is_some_and(|candidate| candidate.uri() == proxy.uri())
        })
    });
    Agent {
        inner: connect_agent(config),
        unsupported_proxy,
    }
}

fn connect_agent(config: ureq::config::Config) -> ureq::Agent {
    use ureq::unversioned::transport::{ConnectProxyConnector, Connector};
    let connector =
        ().chain(ConnectProxyConnector::default())
            .chain(socket::SocketConnector)
            .chain(tls::TrustConnector::default())
            .chain(response::ResponseConnector);
    ureq::Agent::with_parts(
        config,
        connector,
        ureq::unversioned::resolver::DefaultResolver::default(),
    )
}

#[derive(Debug)]
pub(crate) struct Refused {
    pub(crate) error: Error,
    pub(crate) too_large: bool,
}

impl Refused {
    fn transport(cause: ureq::Error, backend: crate::BackendName) -> Self {
        let mut error = Error::new(
            ErrorKind::Transport,
            format!(
                "the {} backend could not be reached: {}",
                backend.as_str(),
                transport_cause(&cause)
            ),
        );
        error.source = Some(std::sync::Arc::new(cause));
        Self {
            error,
            too_large: false,
        }
    }

    pub(crate) fn cancelled() -> Self {
        Self {
            error: crate::retry::cancelled(),
            too_large: false,
        }
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "the send boundary receives shared accounting alongside transport policy and cancellation"
)]
pub(crate) fn post(
    agent: &Agent,
    backend: &crate::backend::Remote,
    name: crate::BackendName,
    url: &str,
    body: &[u8],
    policy: RetryPolicy,
    cancel: &Cancel,
    account: &crate::usage::Account,
    response_limit: Option<u64>,
) -> Result<Vec<u8>, Refused> {
    let key = backend.key.as_ref().ok_or_else(|| Refused {
        error: Error::new(
            ErrorKind::KeyMissing,
            "live transport requires an admitted credential",
        ),
        too_large: false,
    })?;
    let uri = crate::backend::validate_url(url).map_err(|error| Refused {
        error,
        too_large: false,
    })?;
    if uri.scheme_str() != Some("http")
        && let Some(variable) = agent.unsupported_proxy
        && agent
            .inner
            .config()
            .proxy()
            .is_some_and(|proxy| !proxy.is_no_proxy(&uri))
    {
        return Err(Refused {
            error: Error::new(
                ErrorKind::Transport,
                format!("{variable} selects an unsupported SOCKS proxy; use an HTTP CONNECT proxy"),
            ),
            too_large: false,
        });
    }
    let security = if uri.scheme_str() == Some("http") {
        scope::Security::Loopback
    } else {
        scope::Security::Encrypted
    };
    let _exchange = scope::enter(cancel, security);
    let mut retries = 0;
    let mut connection_retried = false;
    let mut fresh = None;
    let mut delay = policy.initial_backoff;
    let mut reason = crate::usage::Reason::Initial;
    loop {
        if cancel.is_cancelled() {
            return Err(Refused::cancelled());
        }
        response::begin();
        let request = fresh.as_ref().unwrap_or(&agent.inner).post(url);
        // Admission permits plaintext only on loopback. Its key must stay local.
        let request = if uri.scheme_str() == Some("http") {
            request.config().proxy(None).build()
        } else {
            request
        };
        let mut attempt = account.begin(body.len(), reason).map_err(|error| Refused {
            error,
            too_large: false,
        })?;
        let sent = request
            .header("content-type", "application/json")
            .header("authorization", &format!("Bearer {}", key.as_str()))
            .send(body);
        if cancel.is_cancelled() {
            return Err(Refused::cancelled());
        }
        let mut response = match sent {
            Ok(response) => response,
            Err(error) => {
                if !connection_retried && (before_send(&error) || response::stale(&error)) {
                    connection_retried = true;
                    reason = crate::usage::Reason::Connection;
                    fresh = Some(connect_agent(agent.inner.config().clone()));
                    continue;
                }
                return Err(Refused::transport(error, name));
            }
        };
        let status = response.status().as_u16();
        if (200..300).contains(&status) {
            let body = match response_limit {
                Some(limit) => response.body_mut().with_config().limit(limit).read_to_vec(),
                None => response.body_mut().read_to_vec(),
            };
            if let Ok(bytes) = &body {
                attempt.response(bytes, true);
            }
            return if cancel.is_cancelled() {
                Err(Refused::cancelled())
            } else {
                body.map_err(|error| {
                    if response_limit.is_some() && matches!(error, ureq::Error::BodyExceedsLimit(_))
                    {
                        Refused {
                            error: crate::response_store::error(crate::Code::LocalStoreQuota),
                            too_large: false,
                        }
                    } else {
                        Refused::transport(error, name)
                    }
                })
            };
        }
        let (body, complete) = excerpt::capture(response.body_mut());
        attempt.response(&body, complete);
        drop(attempt);
        let excerpt = excerpt::clean(&body, key.as_str());
        let mut message = format!("the backend answered status {status}");
        if !excerpt.is_empty() {
            message.push_str(": ");
            message.push_str(&excerpt);
        }
        let mut error = Error::new(ErrorKind::Status, message);
        error.http_status = Some(status);
        let refused = Refused {
            error,
            too_large: status == 413 || (status == 400 && backend.adapter.request_too_large(&body)),
        };
        if cancel.is_cancelled() {
            return Err(Refused::cancelled());
        }
        if !matches!(status, 429 | 500 | 502 | 503 | 504 | 529) || retries == policy.max_retries {
            return Err(refused);
        }
        let retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|value| value.to_str().ok());
        let retry_delay = crate::retry::retry_delay(delay, retry_after);
        if cancel.wait(retry_delay) {
            return Err(Refused::cancelled());
        }
        retries += 1;
        reason = crate::usage::Reason::Status;
        delay = delay.saturating_mul(2);
    }
}

fn before_send(error: &ureq::Error) -> bool {
    matches!(error, ureq::Error::Io(error) if error.kind() == std::io::ErrorKind::ConnectionRefused)
}

// Classify typed failures only: error text can contain URLs or credentials.
fn transport_cause(error: &ureq::Error) -> String {
    match error {
        ureq::Error::Timeout(_) => "timeout".to_owned(),
        ureq::Error::HostNotFound => "hostname resolution failed".to_owned(),
        ureq::Error::Tls(_) | ureq::Error::Rustls(_) | ureq::Error::TlsRequired => {
            tls::FAILURE.to_owned()
        }
        ureq::Error::Io(error) => {
            if error
                .get_ref()
                .is_some_and(|inner| inner.is::<rustls::Error>())
            {
                return tls::FAILURE.to_owned();
            }
            if let Some(inner) = error
                .get_ref()
                .and_then(|inner| inner.downcast_ref::<ureq::Error>())
            {
                return transport_cause(inner);
            }
            match error.kind() {
                std::io::ErrorKind::ConnectionRefused => "connection refused".to_owned(),
                std::io::ErrorKind::ConnectionReset => "connection reset".to_owned(),
                std::io::ErrorKind::ConnectionAborted => "connection aborted".to_owned(),
                std::io::ErrorKind::TimedOut => "timeout".to_owned(),
                std::io::ErrorKind::UnexpectedEof => "connection closed unexpectedly".to_owned(),
                std::io::ErrorKind::BrokenPipe => "connection broken".to_owned(),
                kind => format!("I/O failure ({kind})"),
            }
        }
        ureq::Error::BodyExceedsLimit(limit) => format!("reply body exceeds {limit} bytes"),
        ureq::Error::Protocol(_) => "invalid HTTP response".to_owned(),
        ureq::Error::ConnectProxyFailed(_) => "proxy connection failed".to_owned(),
        _ => "HTTP transport failure".to_owned(),
    }
}
