//! Local TLS fixtures shared by the native and command trust boundaries.
#![allow(dead_code, clippy::expect_used, clippy::panic, reason = "each owning test binary uses the shared TLS fixture operations it needs")]
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

pub(crate) const KEY: &str = "sk-local-ca-0211";
pub(crate) const RESPONSE: &str = "{\"model\":\"local-1\",\"answers\":{\"q1\":{\"type\":\"noul\",\"noul\":0.92}},\"usage\":{\"input_tokens\":1,\"output_tokens\":1}}";
pub(crate) const RESPONSE_ALT: &str = "{\"model\":\"local-2\",\"answers\":{\"q1\":{\"type\":\"noul\",\"noul\":0.92}},\"usage\":{\"input_tokens\":1,\"output_tokens\":1}}";

// Fixed test certificates, valid until 2126. The keys guard nothing. The test
// serves TLS itself, so the proof never depends on which `openssl` is on
// `PATH` (ticket 0372). The two CAs share one subject. Made with OpenSSL 3:
//   openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes \
//     -keyout ca.key -out ca.pem -days 36500 -subj /CN=thinkthen-test-ca
//   openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes \
//     -keyout wrong.key -out wrong.pem -days 36500 -subj /CN=thinkthen-test-ca
// then, for each leaf NAME and HOST (localhost and localhost, other-host and
// other.invalid), with NAME.cnf holding `subjectAltName=DNS:HOST`:
//   openssl req -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes \
//     -keyout NAME.key -out NAME.csr -subj /CN=localhost
//   openssl x509 -req -in NAME.csr -CA ca.pem -CAkey ca.key -set_serial 1 \
//     -days 36500 -extfile NAME.cnf -out NAME.pem
pub(crate) const CA: &[u8] = include_bytes!("../library/tls/ca.pem");
const WRONG_CA: &[u8] = include_bytes!("../library/tls/wrong.pem");
/// The named leaf's certificate and key.
fn leaf(name: &str) -> (&'static [u8], &'static [u8]) {
    match name {
        "localhost" => (
            include_bytes!("../library/tls/localhost.pem"),
            include_bytes!("../library/tls/localhost.key"),
        ),
        "other-host" => (
            include_bytes!("../library/tls/other-host.pem"),
            include_bytes!("../library/tls/other-host.key"),
        ),
        _ => panic!("unknown test leaf {name}"),
    }
}

/// A temporary folder holding copies of both test CAs, so a test may remove one.
pub(crate) struct Fixture(pub(crate) PathBuf);

impl Fixture {
    pub(crate) fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "thinkthen-ca-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).expect("fresh certificate fixture folder");
        fs::write(path.join("ca.pem"), CA).expect("test CA copy");
        fs::write(path.join("wrong.pem"), WRONG_CA).expect("wrong CA copy");
        Self(path)
    }

    pub(crate) fn at(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// One HTTPS connection served on a test thread with the named leaf.
pub(crate) struct Server {
    pub(crate) url: String,
    served: thread::JoinHandle<String>,
}

impl Server {
    pub(crate) fn start(name: &str, response: &str) -> Self {
        use rustls::pki_types::pem::PemObject;
        use rustls::pki_types::{CertificateDer, PrivateKeyDer};

        let (cert, key) = leaf(name);
        let config = rustls::ServerConfig::builder_with_provider(Arc::new(
            rustls::crypto::ring::default_provider(),
        ))
        .with_safe_default_protocol_versions()
        .expect("TLS versions")
        .with_no_client_auth()
        .with_single_cert(
            vec![CertificateDer::from_pem_slice(cert).expect("leaf certificate")],
            PrivateKeyDer::from_pem_slice(key).expect("leaf key"),
        )
        .expect("TLS server configuration");
        // The listener is bound before the client learns its port and stays held.
        let listener = TcpListener::bind("127.0.0.1:0").expect("free TLS port");
        let port = listener.local_addr().expect("TLS port").port();
        listener.set_nonblocking(true).expect("polled accept");
        let reply = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response}",
            response.len()
        );
        Self {
            url: format!("https://localhost:{port}"),
            served: thread::spawn(move || serve(&listener, config, &reply)),
        }
    }

    pub(crate) fn finish(self) -> String {
        self.served.join().expect("the TLS responder")
    }
}

/// Accept one connection, reply once the request's head arrives, and return
/// every byte the client sent. A broken client fails the test after 60 s.
fn serve(listener: &TcpListener, config: rustls::ServerConfig, reply: &str) -> String {
    let cap = Instant::now() + Duration::from_secs(60);
    let socket = loop {
        match listener.accept() {
            Ok((socket, _)) => break socket,
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                assert!(Instant::now() < cap, "no TLS connection within 60 s");
                thread::sleep(Duration::from_millis(10));
            }
            Err(error) => panic!("TLS accept failed: {error}"),
        }
    };
    socket.set_nonblocking(false).expect("blocking reads");
    socket
        .set_read_timeout(Some(Duration::from_secs(60)))
        .expect("read limit");
    let connection = rustls::ServerConnection::new(Arc::new(config)).expect("TLS session");
    let mut stream = rustls::StreamOwned::new(connection, socket);
    let (mut sent, mut replied, mut chunk) = (Vec::new(), false, [0_u8; 4096]);
    // A refused handshake, or a client that leaves without a TLS goodbye,
    // ends the read with an error.
    while let Ok(read @ 1..) = stream.read(&mut chunk) {
        sent.extend_from_slice(chunk.get(..read).unwrap_or_default());
        if !replied && sent.windows(4).any(|end| end == b"\r\n\r\n") {
            stream.write_all(reply.as_bytes()).expect("fixed reply");
            stream.flush().expect("fixed reply sent");
            replied = true;
        }
    }
    String::from_utf8_lossy(&sent).into_owned()
}

