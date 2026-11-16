//! Security checks use injected DNS because changing the host resolver is unsafe.
use super::{scope, socket::SocketConnector};
use std::net::SocketAddr;
use std::time::Duration;
use ureq::unversioned::resolver::{ResolvedSocketAddrs, Resolver};
use ureq::unversioned::transport::NextTimeout;

#[derive(Debug)]
struct Addresses(Vec<SocketAddr>);
impl Resolver for Addresses {
    fn resolve(
        &self,
        _: &ureq::http::Uri,
        _: &ureq::config::Config,
        _: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        let mut resolved = self.empty();
        for address in &self.0 {
            resolved.push(*address);
        }
        Ok(resolved)
    }
}

#[test]
fn plaintext_refuses_the_entire_dns_set_before_connecting() {
    for addresses in [
        vec![
            "127.0.0.1:1".parse().unwrap(),
            "192.0.2.1:80".parse().unwrap(),
        ],
        vec![
            "[::1]:80".parse().unwrap(),
            "[2001:db8::1]:80".parse().unwrap(),
        ],
    ] {
        let config = ureq::Agent::config_builder()
            .proxy(None)
            .timeout_global(Some(Duration::from_secs(1)))
            .build();
        let agent = ureq::Agent::with_parts(config, SocketConnector, Addresses(addresses));
        let _exchange = scope::enter(&crate::Cancel::default(), scope::Security::Loopback);
        let error = agent.get("http://localhost/").call().unwrap_err();
        assert!(
            matches!(error, ureq::Error::Io(ref error) if error.kind() == std::io::ErrorKind::PermissionDenied),
            "{error}"
        );
    }
}
