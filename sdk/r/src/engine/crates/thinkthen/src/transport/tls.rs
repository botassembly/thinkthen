//! Platform certificate trust, augmented by the engine's optional PEM file.

use std::io::{Read, Write};
use std::sync::{Arc, OnceLock};

use rustls::pki_types::{CertificateDer, ServerName, pem::PemObject};
use ureq::unversioned::transport::{
    Buffers, ConnectionDetails, Connector, Either, LazyBuffers, NextTimeout, Transport,
    TransportAdapter,
};

pub(super) const FAILURE: &str =
    "TLS failure; check the operating system certificate store or THINKTHEN_CA_FILE";

#[derive(Debug, Default)]
pub(super) struct TrustConnector {
    config: OnceLock<Result<Arc<rustls::ClientConfig>, ureq::Error>>,
}

impl<In: Transport> Connector<In> for TrustConnector {
    type Out = Either<In, TlsTransport>;

    fn connect(
        &self,
        details: &ConnectionDetails,
        chained: Option<In>,
    ) -> Result<Option<Self::Out>, ureq::Error> {
        let Some(transport) = chained else {
            return Ok(None);
        };
        if !details.needs_tls() || transport.is_tls() {
            return Ok(Some(Either::A(transport)));
        }
        let config = self
            .config
            .get_or_init(configuration)
            .as_ref()
            .map_err(|_| ureq::Error::Tls(FAILURE))?
            .clone();
        let host = details.uri.host().unwrap_or_default();
        let host = host
            .strip_prefix('[')
            .and_then(|host| host.strip_suffix(']'))
            .unwrap_or(host);
        let name = ServerName::try_from(host.to_owned()).map_err(|_| ureq::Error::Tls(FAILURE))?;
        let mut connection = rustls::ClientConnection::new(config, name)?;
        let mut socket = TransportAdapter::new(transport.boxed());
        socket.set_timeout(details.timeout);
        // Finish certificate verification before accepting HTTP headers or the key.
        connection.complete_io(&mut socket)?;
        Ok(Some(Either::B(TlsTransport {
            stream: rustls::StreamOwned::new(connection, socket),
            buffers: LazyBuffers::new(
                details.config.input_buffer_size(),
                details.config.output_buffer_size(),
            ),
        })))
    }
}

fn configuration() -> Result<Arc<rustls::ClientConfig>, ureq::Error> {
    let roots = extra_roots()?;
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    // ureq 3.4's RootCerts::Specific replaces platform roots; PlatformVerifier
    // cannot take extra roots or a custom ClientConfig. Keep HTTP and CONNECT in
    // ureq and use the platform verifier's additive API at this single TLS edge.
    // https://docs.rs/ureq/3.4.2/ureq/tls/enum.RootCerts.html
    // https://docs.rs/rustls-platform-verifier/0.7.1/rustls_platform_verifier/struct.Verifier.html
    let verifier =
        rustls_platform_verifier::Verifier::new_with_extra_roots(roots, provider.clone())
            .map_err(|_| ureq::Error::Tls(FAILURE))?;
    let config = rustls::ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(verifier))
        .with_no_client_auth();
    Ok(Arc::new(config))
}

fn extra_roots() -> Result<Vec<CertificateDer<'static>>, ureq::Error> {
    let Some(path) = std::env::var_os("THINKTHEN_CA_FILE") else {
        return Ok(Vec::new());
    };
    let pem = std::fs::read(path).map_err(|_| ureq::Error::Tls(FAILURE))?;
    let roots = CertificateDer::pem_slice_iter(&pem)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ureq::Error::Tls(FAILURE))?;
    if roots.is_empty() {
        return Err(ureq::Error::Tls(FAILURE));
    }
    // Reject every invalid DER on every OS, including mixed valid/invalid files.
    let mut admitted = rustls::RootCertStore::empty();
    for root in &roots {
        admitted
            .add(root.clone())
            .map_err(|_| ureq::Error::Tls(FAILURE))?;
    }
    Ok(roots)
}

pub(super) struct TlsTransport {
    buffers: LazyBuffers,
    stream: rustls::StreamOwned<rustls::ClientConnection, TransportAdapter>,
}

impl std::fmt::Debug for TlsTransport {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TlsTransport")
            .finish_non_exhaustive()
    }
}

impl Transport for TlsTransport {
    fn buffers(&mut self) -> &mut dyn Buffers {
        &mut self.buffers
    }

    fn transmit_output(&mut self, amount: usize, timeout: NextTimeout) -> Result<(), ureq::Error> {
        self.stream.get_mut().set_timeout(timeout);
        let output = self
            .buffers
            .output()
            .get(..amount)
            .ok_or(ureq::Error::Tls(FAILURE))?;
        self.stream.write_all(output)?;
        Ok(())
    }

    fn await_input(&mut self, timeout: NextTimeout) -> Result<bool, ureq::Error> {
        self.stream.get_mut().set_timeout(timeout);
        let amount = self.stream.read(self.buffers.input_append_buf())?;
        self.buffers.input_appended(amount);
        Ok(amount > 0)
    }

    fn is_open(&mut self) -> bool {
        self.stream.get_mut().get_mut().is_open()
    }

    fn is_tls(&self) -> bool {
        true
    }
}
