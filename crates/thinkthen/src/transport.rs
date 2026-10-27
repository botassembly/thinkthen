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

pub(crate) fn agent(config: ureq::config::Config) -> ureq::Agent {
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

pub(crate) fn post(
    agent: &ureq::Agent,
    backend: &crate::backend::Remote,
    name: crate::BackendName,
    url: &str,
    body: &[u8],
    policy: RetryPolicy,
    cancel: &Cancel,
) -> Result<Vec<u8>, Refused> {
    let uri = crate::backend::validate_url(url).map_err(|error| Refused {
        error,
        too_large: false,
    })?;
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
    loop {
        if cancel.is_cancelled() {
            return Err(Refused::cancelled());
        }
        response::begin();
        let request = fresh.as_ref().unwrap_or(agent).post(url);
        // Admission permits plaintext only on loopback. Its key must stay local.
        let request = if uri.scheme_str() == Some("http") {
            request.config().proxy(None).build()
        } else {
            request
        };
        let sent = request
            .header("content-type", "application/json")
            .header("authorization", &format!("Bearer {}", backend.key.as_str()))
            .send(body);
        if cancel.is_cancelled() {
            return Err(Refused::cancelled());
        }
        let mut response = match sent {
            Ok(response) => response,
            Err(error) => {
                if !connection_retried && (before_send(&error) || response::stale(&error)) {
                    connection_retried = true;
                    fresh = Some(self::agent(agent.config().clone()));
                    continue;
                }
                return Err(Refused::transport(error, name));
            }
        };
        let status = response.status().as_u16();
        if (200..300).contains(&status) {
            let body = response.body_mut().read_to_vec();
            return if cancel.is_cancelled() {
                Err(Refused::cancelled())
            } else {
                body.map_err(|error| Refused::transport(error, name))
            };
        }
        let body = excerpt::capture(response.body_mut());
        let excerpt = excerpt::clean(&body, backend.key.as_str());
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
        ureq::Error::Protocol(_) => "invalid HTTP response".to_owned(),
        ureq::Error::ConnectProxyFailed(_) => "proxy connection failed".to_owned(),
        _ => "HTTP transport failure".to_owned(),
    }
}
