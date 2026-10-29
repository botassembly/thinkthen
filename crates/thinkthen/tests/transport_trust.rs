#![cfg(test)]
//! The engine admits operator roots and verifies TLS before sending a key.

#[path = "support/trust.rs"]
mod trust;

use std::process::Command;
use thinkthen::{Decide, Key, Outcome, Record, Settings};

const KEY: &str = "synthetic-loopback-key";

#[test]
fn private_roots_add_trust_and_invalid_files_fail_closed() {
    let fixture = trust::Certificates::new();
    let directory = tempfile::tempdir().unwrap();
    let ca = directory.path().join("ca.pem");
    std::fs::write(&ca, fixture.root.pem()).unwrap();
    let mixed = fixture.root.pem() + "-----BEGIN CERTIFICATE-----\nAQID\n-----END CERTIFICATE-----";
    for (name, contents, answered) in [
        ("valid", None, true),
        ("empty", Some(""), false),
        (
            "malformed",
            Some("-----BEGIN CERTIFICATE-----\ninvalid\n-----END CERTIFICATE-----"),
            false,
        ),
        (
            "invalid-der",
            Some("-----BEGIN CERTIFICATE-----\nAQID\n-----END CERTIFICATE-----"),
            false,
        ),
        ("mixed", Some(mixed.as_str()), false),
        ("missing", None, false),
        ("unset", None, false),
    ] {
        let file = directory.path().join(name);
        if let Some(contents) = contents {
            std::fs::write(&file, contents).unwrap();
        }
        let server = fixture.server();
        let mut child = isolated_child();
        child
            .env("THINKTHEN_TEST_URL", &server.url)
            .env("THINKTHEN_TEST_ANSWERED", answered.to_string());
        if name != "unset" {
            child.env(
                "THINKTHEN_CA_FILE",
                if name == "valid" { &ca } else { &file },
            );
        }
        let output = child.output().unwrap();
        assert!(
            output.status.success(),
            "{name}: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let request = server.finish();
        assert_eq!(!request.is_empty(), answered, "{name}");
        if answered {
            assert!(request.contains(KEY));
        }
    }
}

#[test]
fn private_root_tls_supports_ipv6_loopback() {
    let fixture = trust::Certificates::new();
    let directory = tempfile::tempdir().unwrap();
    let ca = directory.path().join("ca.pem");
    std::fs::write(&ca, fixture.root.pem()).unwrap();
    let Some(server) = fixture.server_ipv6() else {
        return;
    };
    let output = isolated_child()
        .env("THINKTHEN_TEST_URL", &server.url)
        .env("THINKTHEN_TEST_ANSWERED", "true")
        .env("THINKTHEN_CA_FILE", ca)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(server.finish().contains(KEY));
}

#[test]
fn connect_tunnels_tls_without_exposing_the_backend_key() {
    let fixture = trust::Certificates::new();
    let directory = tempfile::tempdir().unwrap();
    let ca = directory.path().join("ca.pem");
    std::fs::write(&ca, fixture.root.pem()).unwrap();
    let server = fixture.server();
    let proxy = trust::Proxy::new(&server.url);
    let output = isolated_child()
        .env("THINKTHEN_TEST_URL", &server.url)
        .env("THINKTHEN_TEST_ANSWERED", "true")
        .env("THINKTHEN_CA_FILE", ca)
        .env("HTTPS_PROXY", &proxy.url)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(server.finish().contains(KEY));
    let headers = proxy.finish();
    assert!(headers.starts_with("CONNECT 127.0.0.1:"));
    assert!(!headers.contains(KEY));
}

#[test]
fn trusted_roots_still_require_the_server_name_and_validity() {
    let wrong_name = rcgen::CertificateParams::new(vec!["another.invalid".to_owned()]).unwrap();
    let mut expired = rcgen::CertificateParams::new(vec!["127.0.0.1".to_owned()]).unwrap();
    expired.not_after = rcgen::date_time_ymd(2000, 1, 1);
    for leaf in [wrong_name, expired] {
        let fixture = trust::Certificates::with_leaf(leaf);
        let directory = tempfile::tempdir().unwrap();
        let ca = directory.path().join("ca.pem");
        std::fs::write(&ca, fixture.root.pem()).unwrap();
        let server = fixture.server();
        let output = isolated_child()
            .env("THINKTHEN_TEST_URL", &server.url)
            .env("THINKTHEN_TEST_ANSWERED", "false")
            .env("THINKTHEN_CA_FILE", ca)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(server.finish().is_empty());
    }
}

#[cfg(target_os = "linux")]
#[test]
fn container_system_store_is_used_and_extra_roots_preserve_it() {
    if std::env::var_os("THINKTHEN_TEST_CONTAINER_STORE").is_none() {
        assert!(
            std::env::var_os("THINKTHEN_EXPECT_CONTAINER_STORE").is_none(),
            "qualification expected the system-store test but omitted its activation"
        );
        return;
    }
    let fixture = trust::Certificates::new();
    let extra = trust::Certificates::new();
    let directory = tempfile::tempdir().unwrap();
    let ca = directory.path().join("extra.pem");
    std::fs::write(&ca, extra.root.pem()).unwrap();
    let store = std::path::Path::new("/etc/ssl/certs/ca-certificates.crt");
    for (installed, file, answered) in [
        (false, false, false),
        (true, false, true),
        (true, true, true),
        (false, true, false),
    ] {
        if installed {
            std::fs::create_dir_all(store.parent().unwrap()).unwrap();
            std::fs::write(store, fixture.root.pem()).unwrap();
        } else if store.exists() {
            std::fs::remove_file(store).unwrap();
        }
        let server = fixture.server();
        let mut child = isolated_child();
        child
            .env("THINKTHEN_TEST_URL", &server.url)
            .env("THINKTHEN_TEST_ANSWERED", answered.to_string());
        if file {
            child.env("THINKTHEN_CA_FILE", &ca);
        }
        let output = child.output().unwrap();
        assert!(
            output.status.success(),
            "store={installed}, file={file}: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(!server.finish().is_empty(), answered);
    }
}

fn isolated_child() -> Command {
    let mut child = Command::new(std::env::current_exe().unwrap());
    child.env_clear();
    for name in [
        "PATH",
        "SystemRoot",
        "WINDIR",
        "TEMP",
        "TMP",
        "LLVM_PROFILE_FILE",
    ] {
        if let Some(value) = std::env::var_os(name) {
            child.env(name, value);
        }
    }
    child.args(["--exact", "trust_child", "--nocapture"]);
    child
}

#[test]
fn trust_child() {
    let Ok(url) = std::env::var("THINKTHEN_TEST_URL") else {
        return;
    };
    let answered = std::env::var("THINKTHEN_TEST_ANSWERED").unwrap() == "true";
    let engine = Settings::default()
        .url(url)
        .api_key(Key::new(KEY).unwrap())
        .model("model")
        .engine(|_| None)
        .unwrap();
    let rows = engine
        .rows(&Decide::new("Ready?").unwrap(), [Record::text("evidence")])
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(rows.len(), 1);
    if answered {
        assert!(
            matches!(rows[0].outcome(), Outcome::Answered { .. }),
            "{rows:?}"
        );
    } else {
        let error = rows[0].error().unwrap();
        assert_eq!(error.code().as_str(), "backend.transport");
        assert!(error.message().contains("TLS failure"), "{error}");
        assert!(error.message().contains("THINKTHEN_CA_FILE"), "{error}");
        assert!(!error.message().contains(KEY));
    }
}

#[cfg(target_os = "linux")]
#[test]
fn an_expected_container_store_proof_cannot_silently_skip() {
    let output = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "container_system_store_is_used_and_extra_roots_preserve_it",
            "--nocapture",
        ])
        .env_remove("THINKTHEN_TEST_CONTAINER_STORE")
        .env("THINKTHEN_EXPECT_CONTAINER_STORE", "1")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("omitted its activation")
            || String::from_utf8_lossy(&output.stderr).contains("omitted its activation")
    );
}
