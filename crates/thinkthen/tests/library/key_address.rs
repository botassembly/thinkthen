//! Public and compiled-command boundaries for an exact configured-key URL collision.
#![allow(
    clippy::expect_used,
    reason = "a failed fixture must stop the boundary proof"
)]

use crate::child::ChildEnvironment as _;
use std::process::Command;

use conformance_backend::{Canned, Listener};
use thinkthen::{EngineBuilder, ErrorKind, Question};

#[path = "../support/key_address.rs"]
mod fixture;
use fixture::REFUSAL;

#[test]
fn public_builder_checks_the_final_effective_key_before_local_effects() {
    for (key, base) in [
        ("key-path-0210", "http://localhost/key-path-0210"),
        ("systemone", "http://localhost"),
        ("秘密", "http://localhost/秘密"),
    ] {
        let builder = crate::api::builder(base, key).no_cache();
        let debug = format!("{builder:?}");
        assert!(!debug.contains(base));
        assert!(!debug.contains(key));
        let error = builder.build().expect_err("effective key in final URL");
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(error.to_string(), REFUSAL);
        assert!(!format!("{error:?}").contains(key));
    }

    // The URL is compared literally. Its percent-encoded path is not decoded.
    crate::api::builder("http://localhost/%E7%A7%98%E5%AF%86", "秘密")
        .no_cache()
        .build()
        .expect("a different byte spelling is safe");
}

#[test]
fn captured_key_is_checked_and_an_explicit_key_overrides_it() {
    if std::env::var_os("THINKTHEN_KEY_ADDRESS_CHILD").is_some() {
        let base = "http://localhost/captured-key-0210";
        let refused = EngineBuilder::from_env()
            .expect("captured settings")
            .base_url(base)
            .expect("base")
            .no_cache()
            .build()
            .expect_err("captured key in address");
        assert_eq!(refused.kind(), ErrorKind::Usage);
        assert_eq!(refused.to_string(), REFUSAL);
        for explicit_first in [false, true] {
            let builder = EngineBuilder::from_env().expect("captured settings");
            let builder = if explicit_first {
                builder
                    .api_key("different-safe-key")
                    .expect("override")
                    .base_url(base)
                    .expect("base")
            } else {
                builder
                    .base_url(base)
                    .expect("base")
                    .api_key("different-safe-key")
                    .expect("override")
            };
            builder.no_cache().build().expect("final key is safe");
        }
        return;
    }
    let output = Command::new(std::env::current_exe().expect("test binary"))
        .args([
            "--exact",
            "key_address::captured_key_is_checked_and_an_explicit_key_overrides_it",
        ])
        .clear_environment()
        .home(std::env::temp_dir().join(format!("thinkthen-key-child-{}", std::process::id())))
        .env("THINKTHEN_KEY_ADDRESS_CHILD", "1")
        .env("THINKTHEN_API_KEY", "captured-key-0210")
        .output()
        .expect("isolated child test");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn safe_gateway_url_keeps_result_identity_but_not_debug_text() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"usage":{"input_tokens":1,"output_tokens":1}}"#)
    })
    .expect("loopback listener");
    let base = format!("{}/gateway-marker-0210", listener.base());
    let engine = crate::api::builder(&base, "different-key-0210")
        .no_cache()
        .build()
        .expect("safe gateway");
    let question = Question::decide("a benign question")
        .expect("question")
        .cut();
    let details = engine
        .decide(&question, "benign evidence")
        .expect("details");
    let posting = format!("{base}/systemone");
    assert_eq!(details.results()[0].meta().url(), posting);
    assert!(
        details.results()[0]
            .to_json()
            .expect("JSON")
            .contains(&posting)
    );
    let debug = format!("{details:?}");
    assert!(!debug.contains("gateway-marker-0210"), "{debug}");
    assert_eq!(listener.count(), 1);
}
