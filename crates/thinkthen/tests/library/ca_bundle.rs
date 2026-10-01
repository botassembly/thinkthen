//! Genuine localhost TLS trust and bounded named-bundle refusals (ticket 0211).
#![allow(
    clippy::expect_used,
    clippy::panic,
    reason = "a failed local certificate fixture must stop the boundary proof"
)]

use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
#[cfg(feature = "cli")]
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
#[cfg(feature = "cli")]
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(feature = "cli")]
use thinkthen::{Engine, ErrorKind};
use thinkthen::{EngineBuilder, Question};

const KEY: &str = "sk-local-ca-0211";
const RESPONSE: &str = "{\"model\":\"local-1\",\"answers\":{\"q1\":{\"type\":\"noul\",\"noul\":0.92}},\"usage\":{\"input_tokens\":1,\"output_tokens\":1}}";
const RESPONSE_ALT: &str = "{\"model\":\"local-2\",\"answers\":{\"q1\":{\"type\":\"noul\",\"noul\":0.92}},\"usage\":{\"input_tokens\":1,\"output_tokens\":1}}";

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
const CA: &[u8] = include_bytes!("tls/ca.pem");
const WRONG_CA: &[u8] = include_bytes!("tls/wrong.pem");
/// The named leaf's certificate and key.
fn leaf(name: &str) -> (&'static [u8], &'static [u8]) {
    match name {
        "localhost" => (
            include_bytes!("tls/localhost.pem"),
            include_bytes!("tls/localhost.key"),
        ),
        "other-host" => (
            include_bytes!("tls/other-host.pem"),
            include_bytes!("tls/other-host.key"),
        ),
        _ => panic!("unknown test leaf {name}"),
    }
}

/// A temporary folder holding copies of both test CAs, so a test may remove one.
struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
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

    fn at(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// One HTTPS connection served on a test thread with the named leaf.
struct Server {
    url: String,
    served: thread::JoinHandle<String>,
}

impl Server {
    fn start(name: &str, response: &str) -> Self {
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

    fn finish(self) -> String {
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

#[cfg(feature = "cli")]
fn command(url: &str, ca: Option<&Path>, home: &Path, key: Option<&str>) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    child
        .args([
            "decide",
            "Does this ask for a refund?",
            "--url",
            url,
            "--model",
            "local-1",
            "--no-cache",
        ])
        .env_clear()
        .env("HOME", home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(key) = key {
        child.env("THINKTHEN_API_KEY", key);
    }
    if let Some(ca) = ca {
        child.env("THINKTHEN_CA_BUNDLE", ca);
    }
    let mut child = child.spawn().expect("compiled command");
    child
        .stdin
        .take()
        .expect("command input")
        .write_all(b"Please refund this.\n")
        .expect("evidence input");
    child.wait_with_output().expect("command result")
}

#[cfg(feature = "cli")]
#[test]
fn genuine_tls_trust_replaces_default_and_keeps_hostname_verification() {
    let fixture = Fixture::new();
    for (leaf, bundle, success) in [
        ("localhost", None, false),
        ("localhost", Some("ca.pem"), true),
        ("localhost", Some("wrong.pem"), false),
        ("other-host", Some("ca.pem"), false),
    ] {
        let server = Server::start(leaf, RESPONSE);
        let output = command(
            &server.url,
            bundle.map(|name| fixture.at(name)).as_deref(),
            &fixture.0,
            Some(KEY),
        );
        let requests = server.finish();
        let said = String::from_utf8_lossy(&output.stderr);
        if success {
            assert_eq!(output.status.code(), Some(0), "{said}");
            assert_eq!(output.stdout, b"true\n");
            assert_eq!(
                requests.matches("POST /systemone").count(),
                1,
                "{requests:?}"
            );
        } else {
            assert_eq!(output.status.code(), Some(4), "{said}");
            assert!(
                said.contains("the TLS connection or certificate check failed"),
                "{said}"
            );
            assert!(!requests.contains("POST /systemone"), "{requests}");
        }
        assert!(!said.contains(KEY), "{said}");
    }
}

#[cfg(feature = "cli")]
#[test]
fn bounded_bundle_refusals_precede_send_and_replay_folder_work() {
    let fixture = Fixture::new();
    let base = "https://localhost:1";
    let missing = fixture.at("missing.pem");
    let replay = fixture.at("absent-recording");
    let refused = Engine::builder()
        .base_url(base)
        .expect("base")
        .replay(&replay)
        .expect("replay path")
        .ca_bundle(&missing)
        .expect("absolute CA path")
        .build()
        .expect_err("named missing CA before replay");
    assert_eq!(refused.kind(), ErrorKind::Local);
    assert!(refused.to_string().contains("THINKTHEN_CA_BUNDLE"));
    assert!(!replay.exists());
    let relative = Engine::builder()
        .ca_bundle("relative.pem")
        .expect_err("absolute path needed");
    assert_eq!(relative.kind(), ErrorKind::Usage);
    for (name, contents) in [
        ("empty.pem", Vec::new()),
        ("text.pem", b"not a certificate".to_vec()),
        (
            "private.pem",
            b"-----BEGIN PRIVATE KEY-----\nAA==\n-----END PRIVATE KEY-----\n".to_vec(),
        ),
        (
            "unsupported.pem",
            b"-----BEGIN CERTIFICATE REQUEST-----\nAA==\n-----END CERTIFICATE REQUEST-----\n"
                .to_vec(),
        ),
        (
            "malformed.pem",
            b"-----BEGIN CERTIFICATE-----\n%%%\n-----END CERTIFICATE-----\n".to_vec(),
        ),
        (
            "unmatched.pem",
            b"-----BEGIN CERTIFICATE-----\nAA==\n".to_vec(),
        ),
        ("oversize.pem", vec![b'A'; 2 * 1024 * 1024 + 1]),
        ("too-many.pem", CA.repeat(257)),
    ] {
        let path = fixture.at(name);
        fs::write(&path, contents).expect("bad CA fixture");
        let refused = Engine::builder()
            .base_url(base)
            .expect("base")
            .ca_bundle(&path)
            .expect("absolute CA path")
            .no_cache()
            .build()
            .expect_err("bad CA before transport");
        assert_eq!(refused.kind(), ErrorKind::Usage, "{name}");
        assert!(
            !refused.to_string().contains(path.to_str().expect("path")),
            "{name}"
        );
    }
    let output = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .args(["check", "--url", base, "--plan"])
        .env_clear()
        .env("HOME", &fixture.0)
        .env("THINKTHEN_CA_BUNDLE", &missing)
        .output()
        .expect("compiled check");
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("THINKTHEN_CA_BUNDLE"));

    // This is an ordinary live decide path with no key, not a dry-run path.
    // The invalid bundle must win before either the key reader or a socket.
    let listener = TcpListener::bind("127.0.0.1:0").expect("counted local socket");
    listener
        .set_nonblocking(true)
        .expect("nonblocking listener");
    let url = format!(
        "https://127.0.0.1:{}",
        listener.local_addr().expect("address").port()
    );
    let output = command(&url, Some(&missing), &fixture.0, None);
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("THINKTHEN_CA_BUNDLE"));
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
    );
}

#[test]
fn public_environment_and_explicit_builder_share_the_loaded_roots() {
    let fixture = Fixture::new();
    for mode in ["env", "explicit"] {
        let response = if mode == "explicit" {
            RESPONSE_ALT
        } else {
            RESPONSE
        };
        let server = Server::start("localhost", response);
        let output = Command::new(std::env::current_exe().expect("test executable"))
            .args(["--exact", "ca_bundle::public_tls_child", "--nocapture"])
            .env_clear()
            .env("HOME", &fixture.0)
            .env("THINKTHEN_TEST_CA_MODE", mode)
            .env(
                "THINKTHEN_CA_BUNDLE",
                if mode == "explicit" {
                    PathBuf::from("invalid-relative-ca.pem")
                } else {
                    fixture.at("ca.pem")
                },
            )
            .env("THINKTHEN_TEST_SELECTED_CA", fixture.at("ca.pem"))
            .env("THINKTHEN_BASE_URL", &server.url)
            .env("THINKTHEN_API_KEY", KEY)
            .output()
            .expect("isolated public caller");
        let requests = server.finish();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        assert_eq!(
            requests.matches("POST /systemone").count(),
            1,
            "{requests:?}"
        );
        if mode == "explicit" {
            assert!(requests.contains("\"model\":\"local-2\""), "{requests:?}");
        }
    }
}

#[test]
fn public_tls_child() {
    let Ok(mode) = std::env::var("THINKTHEN_TEST_CA_MODE") else {
        return;
    };
    let ca = PathBuf::from(std::env::var("THINKTHEN_TEST_SELECTED_CA").expect("test CA"));
    let builder = if mode == "env" {
        EngineBuilder::from_env().expect("env settings")
    } else {
        EngineBuilder::from_env()
            .expect("invalid environment CA remains overridable")
            .ca_bundle(&ca)
            .expect("explicit CA path")
    };
    let engine = builder.no_cache().build().expect("private CA engine");
    let mut question = Question::decide("Does this ask for a refund?").expect("question");
    if mode == "explicit" {
        fs::remove_file(&ca).expect("remove selected CA after engine build");
        question = question.model("local-2").expect("model override");
    }
    let question = question.cut();
    engine
        .decide(&question, "Please refund this.")
        .expect("trusted answer");
}
