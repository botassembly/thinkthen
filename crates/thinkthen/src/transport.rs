//! Send attempts, status backoff and safe connection retries.

use crate::core::{Error, ErrorKind};

use crate::{Cancel, RetryPolicy};

mod socket;

pub(crate) fn agent(config: ureq::config::Config, cancel: Cancel) -> ureq::Agent {
    use ureq::unversioned::transport::{ConnectProxyConnector, Connector, RustlsConnector};
    let connector =
        ().chain(ConnectProxyConnector::default())
            .chain(socket::SocketConnector(cancel))
            .chain(RustlsConnector::default());
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
    fn transport() -> Self {
        Self {
            error: Error::new(ErrorKind::Transport, "the backend could not be reached"),
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
    url: &str,
    body: &[u8],
    policy: RetryPolicy,
    cancel: &Cancel,
) -> Result<Vec<u8>, Refused> {
    let uri = crate::backend::validate_url(url).map_err(|error| Refused {
        error,
        too_large: false,
    })?;
    let mut retries = 0;
    let mut connection_retried = false;
    let mut delay = policy.initial_backoff;
    loop {
        if cancel.is_cancelled() {
            return Err(Refused::cancelled());
        }
        let request = agent.post(url);
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
                if !connection_retried && before_send(&error) {
                    connection_retried = true;
                    continue;
                }
                return Err(Refused::transport());
            }
        };
        let status = response.status().as_u16();
        if (200..300).contains(&status) {
            let body = response.body_mut().read_to_vec();
            return if cancel.is_cancelled() {
                Err(Refused::cancelled())
            } else {
                body.map_err(|_| Refused::transport())
            };
        }
        let refused = Refused {
            error: Error::new(
                ErrorKind::Status,
                format!("the backend answered status {status}"),
            ),
            too_large: status == 413
                || (status == 400
                    && response
                        .body_mut()
                        .read_to_vec()
                        .is_ok_and(|body| backend.adapter.request_too_large(&body))),
        };
        if cancel.is_cancelled() {
            return Err(Refused::cancelled());
        }
        if !matches!(status, 429 | 500 | 502 | 503 | 504 | 529) || retries == policy.max_retries {
            return Err(refused);
        }
        if cancel.wait(delay) {
            return Err(Refused::cancelled());
        }
        retries += 1;
        delay = delay.saturating_mul(2);
    }
}

fn before_send(error: &ureq::Error) -> bool {
    matches!(error, ureq::Error::Io(error) if error.kind() == std::io::ErrorKind::ConnectionRefused)
}
