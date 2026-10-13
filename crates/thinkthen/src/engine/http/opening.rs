//! Preserve the failure phase before ureq can flatten a global timeout.
//!
//! The default connector completes TCP, proxy and TLS setup before ureq writes
//! judging-request headers or body bytes. Only that boundary and name lookup
//! may mark a failure safe to retry; no connected transport is wrapped.

use crate::engine::error::TransportKind;
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
        ureq::Error::Other(Box::new(OpeningFailure(super::send::transport(
            &error, false,
        ))))
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
