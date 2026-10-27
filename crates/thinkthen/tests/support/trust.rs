#![cfg(test)]
#![allow(
    clippy::disallowed_methods,
    reason = "This loopback TLS and CONNECT fixture owns each listener through its server lifetime."
)]
//! Private, generated authorities and a loopback TLS backend.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

pub(super) struct Certificates {
    pub(super) root: rcgen::Certificate,
    leaf: rcgen::Certificate,
    key: rcgen::KeyPair,
}

impl Certificates {
    pub(super) fn new() -> Self {
        Self::with_leaf(
            rcgen::CertificateParams::new(vec![
                "localhost".to_owned(),
                "127.0.0.1".to_owned(),
                "::1".to_owned(),
            ])
            .unwrap(),
        )
    }

    pub(super) fn with_leaf(mut leaf: rcgen::CertificateParams) -> Self {
        let mut root = rcgen::CertificateParams::default();
        root.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        root.key_usages = vec![
            rcgen::KeyUsagePurpose::KeyCertSign,
            rcgen::KeyUsagePurpose::CrlSign,
        ];
        let root_key = rcgen::KeyPair::generate().unwrap();
        let root = root.self_signed(&root_key).unwrap();
        let key = rcgen::KeyPair::generate().unwrap();
        leaf.extended_key_usages = vec![rcgen::ExtendedKeyUsagePurpose::ServerAuth];
        leaf.use_authority_key_identifier_extension = true;
        let leaf = leaf.signed_by(&key, &root, &root_key).unwrap();
        Self { root, leaf, key }
    }

    pub(super) fn server(&self) -> Server {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        self.server_on(listener)
    }

    pub(super) fn server_ipv6(&self) -> Option<Server> {
        TcpListener::bind("[::1]:0")
            .ok()
            .map(|listener| self.server_on(listener))
    }

    fn server_on(&self, listener: TcpListener) -> Server {
        let url = format!("https://{}/v1", listener.local_addr().unwrap());
        let provider = Arc::new(rustls::crypto::ring::default_provider());
        let config = rustls::ServerConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(
                vec![self.leaf.der().clone()],
                rustls::pki_types::PrivatePkcs8KeyDer::from(self.key.serialize_der()).into(),
            )
            .unwrap();
        let worker = thread::spawn(move || {
            let (socket, _) = listener.accept().unwrap();
            socket
                .set_read_timeout(Some(std::time::Duration::from_secs(10)))
                .unwrap();
            let connection = rustls::ServerConnection::new(Arc::new(config)).unwrap();
            let mut stream = rustls::StreamOwned::new(connection, socket);
            let mut request = Vec::new();
            let mut buffer = [0; 4096];
            loop {
                let Ok(amount) = stream.read(&mut buffer) else {
                    return String::from_utf8(request).unwrap();
                };
                if amount == 0 {
                    return String::from_utf8(request).unwrap();
                }
                request.extend_from_slice(&buffer[..amount]);
                if let Some(end) = request.windows(4).position(|part| part == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&request[..end]);
                    let length = headers
                        .lines()
                        .find_map(|line| {
                            line.to_ascii_lowercase()
                                .strip_prefix("content-length: ")
                                .map(|value| value.parse::<usize>().unwrap())
                        })
                        .unwrap();
                    if request.len() >= end + 4 + length {
                        break;
                    }
                }
            }
            let end = request
                .windows(4)
                .position(|part| part == b"\r\n\r\n")
                .unwrap()
                + 4;
            let sent: serde_json::Value = serde_json::from_slice(&request[end..]).unwrap();
            let answers: serde_json::Map<String, serde_json::Value> = sent["questions"]
                .as_object()
                .unwrap()
                .keys()
                .map(|name| (name.clone(), serde_json::json!({"type":"noul", "noul":0.9})))
                .collect();
            let body = serde_json::json!({"model":"model", "answers": answers}).to_string();
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
            stream.flush().unwrap();
            String::from_utf8(request).unwrap()
        });
        Server { url, worker }
    }
}

pub(super) struct Server {
    pub(super) url: String,
    worker: JoinHandle<String>,
}

impl Server {
    pub(super) fn finish(self) -> String {
        self.worker.join().unwrap()
    }
}

pub(super) struct Proxy {
    pub(super) url: String,
    worker: JoinHandle<String>,
}

impl Proxy {
    pub(super) fn new(url: &str) -> Self {
        let target = url
            .strip_prefix("https://")
            .unwrap()
            .strip_suffix("/v1")
            .unwrap()
            .to_owned();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let worker = thread::spawn(move || {
            let (mut client, _) = listener.accept().unwrap();
            client
                .set_read_timeout(Some(std::time::Duration::from_secs(10)))
                .unwrap();
            let mut headers = Vec::new();
            while !headers.ends_with(b"\r\n\r\n") {
                let mut byte = [0];
                client.read_exact(&mut byte).unwrap();
                headers.push(byte[0]);
            }
            let mut server = std::net::TcpStream::connect(target).unwrap();
            client
                .write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n")
                .unwrap();
            let mut upstream = client.try_clone().unwrap();
            let mut downstream = server.try_clone().unwrap();
            let copy = thread::spawn(move || {
                let _ = std::io::copy(&mut upstream, &mut server);
                let _ = server.shutdown(std::net::Shutdown::Write);
            });
            let _ = std::io::copy(&mut downstream, &mut client);
            let _ = client.shutdown(std::net::Shutdown::Write);
            copy.join().unwrap();
            String::from_utf8(headers).unwrap()
        });
        Self { url, worker }
    }

    pub(super) fn finish(self) -> String {
        self.worker.join().unwrap()
    }
}
