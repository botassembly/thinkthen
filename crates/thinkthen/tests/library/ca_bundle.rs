//! Genuine public TLS trust and bounded named-bundle refusals.
#![allow(clippy::expect_used, clippy::panic, reason = "a failed local certificate fixture stops the boundary proof")]
use crate::child::ChildEnvironment as _;
use std::fs;
use std::process::Command;
use std::path::PathBuf;
use thinkthen::{Engine, EngineBuilder, ErrorKind, Question};
#[path = "../support/tls.rs"]
mod tls;
use tls::{CA, KEY, RESPONSE, RESPONSE_ALT, Fixture, Server};

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
        let preloaded = Engine::builder().base_url(base).unwrap()
            .ca_bundle(&path).unwrap().no_cache().preload_ca_bundle().unwrap_err();
        assert_eq!(preloaded.kind(), refused.kind(), "{name}");
        assert_eq!(preloaded.to_string(), refused.to_string(), "{name}");
        assert_eq!(refused.kind(), ErrorKind::Usage, "{name}");
        assert!(
            !refused.to_string().contains(path.to_str().expect("path")),
            "{name}"
        );
    }
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
            .clear_environment()
            .home(&fixture.0)
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

#[test]
fn explicit_preload_retains_roots_and_new_selection_invalidates_them() {
    let fixture = Fixture::new();
    let path = fixture.at("preloaded.pem");
    let builder = Engine::builder().no_cache().ca_bundle(&path).unwrap();
    // Selecting a bundle is lazy: it may be created before ordinary build.
    fs::write(&path, CA).unwrap();
    builder.build().unwrap();
    let builder = Engine::builder().no_cache().ca_bundle(&path).unwrap()
        .preload_ca_bundle().unwrap();
    fs::remove_file(&path).unwrap();
    // Both a repeated preload and build reuse the admitted roots without reopening.
    let builder = builder.preload_ca_bundle().unwrap();
    builder.build().unwrap();

    fs::write(&path, CA).unwrap();
    let builder = Engine::builder().no_cache().ca_bundle(&path).unwrap()
        .preload_ca_bundle().unwrap();
    let missing = fixture.at("replacement-missing.pem");
    let error = builder.ca_bundle(&missing).unwrap().build().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Local);
    let expected = Engine::builder().no_cache().ca_bundle(&missing).unwrap()
        .preload_ca_bundle().unwrap_err();
    assert_eq!(error.to_string(), expected.to_string());
    assert!(!error.to_string().contains(missing.to_str().unwrap()));
}

#[test]
fn named_storage_metadata_is_a_warning_signal_only() {
    let fixture = Fixture::new();
    let path = fixture.at("named");
    assert!(!thinkthen::storage_writable_by_another(&path));
    fs::write(&path, b"unchanged contents").unwrap();
    assert!(!thinkthen::storage_writable_by_another(&path));
    assert_eq!(fs::read(&path).unwrap(), b"unchanged contents");
    let folder = fixture.at("folder");
    fs::create_dir(&folder).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        for (mode, warned) in [(0o700, false), (0o770, false), (0o707, true)] {
            fs::set_permissions(&folder, fs::Permissions::from_mode(mode)).unwrap();
            assert_eq!(thinkthen::storage_writable_by_another(&folder), warned);
            assert_eq!(fs::metadata(&folder).unwrap().permissions().mode() & 0o777, mode);
        }
    }
    #[cfg(not(unix))]
    assert!(!thinkthen::storage_writable_by_another(&folder));
    assert_eq!(fs::read_dir(folder).unwrap().count(), 0);
}
