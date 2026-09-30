//! Genuine localhost TLS trust and bounded named-bundle refusals (ticket 0211).
#![cfg(target_os = "linux")]
#![allow(
    clippy::expect_used,
    clippy::panic,
    reason = "a failed local certificate fixture must stop the boundary proof"
)]

use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use thinkthen::{Engine, EngineBuilder, ErrorKind, Question};

use crate::child;

const KEY: &str = "sk-local-ca-0211";
const RESPONSE: &str = "{\"model\":\"local-1\",\"answers\":{\"q1\":{\"type\":\"noul\",\"noul\":0.92}},\"usage\":{\"input_tokens\":1,\"output_tokens\":1}}";
const RESPONSE_ALT: &str = "{\"model\":\"local-2\",\"answers\":{\"q1\":{\"type\":\"noul\",\"noul\":0.92}},\"usage\":{\"input_tokens\":1,\"output_tokens\":1}}";

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
        Self(path)
    }

    fn at(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }

    fn ca(&self, name: &str) {
        let key = self.at(&format!("{name}.key"));
        let pem = self.at(&format!("{name}.pem"));
        openssl(&[
            "req",
            "-x509",
            "-newkey",
            "rsa:2048",
            "-nodes",
            "-keyout",
            key.to_str().expect("fixture path"),
            "-out",
            pem.to_str().expect("fixture path"),
            "-days",
            "1",
            "-subj",
            "/CN=thinkthen-test-ca",
        ]);
    }

    fn leaf(&self, name: &str, host: &str) {
        let key = self.at(&format!("{name}.key"));
        let csr = self.at(&format!("{name}.csr"));
        let pem = self.at(&format!("{name}.pem"));
        let san = self.at(&format!("{name}.cnf"));
        fs::write(&san, format!("subjectAltName=DNS:{host}\n")).expect("SAN file");
        openssl(&[
            "req",
            "-newkey",
            "rsa:2048",
            "-nodes",
            "-keyout",
            key.to_str().expect("fixture path"),
            "-out",
            csr.to_str().expect("fixture path"),
            "-subj",
            "/CN=localhost",
        ]);
        openssl(&[
            "x509",
            "-req",
            "-in",
            csr.to_str().expect("fixture path"),
            "-CA",
            self.at("ca.pem").to_str().expect("fixture path"),
            "-CAkey",
            self.at("ca.key").to_str().expect("fixture path"),
            "-set_serial",
            "1",
            "-out",
            pem.to_str().expect("fixture path"),
            "-days",
            "1",
            "-extfile",
            san.to_str().expect("fixture path"),
        ]);
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn openssl(args: &[&str]) {
    let output = child::command("openssl", &[])
        .args(args)
        .output()
        .expect("OpenSSL 3 available");
    assert!(output.status.success(), "OpenSSL fixture command failed");
}

struct Server {
    child: Child,
    url: String,
}

impl Server {
    fn start(fixture: &Fixture, leaf: &str, response: &str) -> Self {
        let address = TcpListener::bind("127.0.0.1:0").expect("reserve port");
        let port = address.local_addr().expect("bound address").port();
        drop(address);
        let accept = format!("127.0.0.1:{port}");
        let mut child = child::command("openssl", &[])
            .args([
                "s_server", "-quiet", "-ign_eof", "-naccept", "1", "-accept", &accept,
            ])
            .arg("-cert")
            .arg(fixture.at(&format!("{leaf}.pem")))
            .arg("-key")
            .arg(fixture.at(&format!("{leaf}.key")))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("local TLS responder");
        let payload = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response}",
            response.len()
        );
        child
            .stdin
            .as_mut()
            .expect("responder input")
            .write_all(payload.as_bytes())
            .expect("canned response");
        for _ in 0..100 {
            if let Some(status) = child.try_wait().expect("responder state") {
                panic!("local TLS responder exited before binding: {status}");
            }
            match TcpListener::bind(&accept) {
                Err(error) if error.kind() == std::io::ErrorKind::AddrInUse => break,
                Ok(probe) => drop(probe),
                Err(error) => panic!("local TLS bind check failed: {error}"),
            }
            thread::sleep(Duration::from_millis(10));
        }
        Self {
            child,
            url: format!("https://localhost:{port}"),
        }
    }

    fn finish(mut self) -> String {
        for _ in 0..100 {
            if self.child.try_wait().expect("responder state").is_some() {
                let mut sent = String::new();
                self.child
                    .stdout
                    .take()
                    .expect("captured requests")
                    .read_to_string(&mut sent)
                    .expect("request text");
                return sent;
            }
            thread::sleep(Duration::from_millis(10));
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
        panic!("local TLS responder did not finish");
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn command(url: &str, ca: Option<&Path>, home: &Path, key: Option<&str>) -> Output {
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

#[test]
fn genuine_tls_trust_replaces_default_and_keeps_hostname_verification() {
    let fixture = Fixture::new();
    fixture.ca("ca");
    fixture.ca("wrong");
    fixture.leaf("localhost", "localhost");
    fixture.leaf("other-host", "other.invalid");
    for (leaf, bundle, success) in [
        ("localhost", None, false),
        ("localhost", Some("ca.pem"), true),
        ("localhost", Some("wrong.pem"), false),
        ("other-host", Some("ca.pem"), false),
    ] {
        let server = Server::start(&fixture, leaf, RESPONSE);
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

#[test]
fn bounded_bundle_refusals_precede_send_and_replay_folder_work() {
    let fixture = Fixture::new();
    fixture.ca("ca");
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
        (
            "too-many.pem",
            fs::read(fixture.at("ca.pem"))
                .expect("valid CA")
                .repeat(257),
        ),
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
    fixture.ca("ca");
    fixture.leaf("localhost", "localhost");
    for mode in ["env", "explicit"] {
        let response = if mode == "explicit" {
            RESPONSE_ALT
        } else {
            RESPONSE
        };
        let server = Server::start(&fixture, "localhost", response);
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
