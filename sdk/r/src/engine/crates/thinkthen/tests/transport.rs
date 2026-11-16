#![cfg(test)]
//! Failed rows explain transport causes without exposing request credentials.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{HeldReply, KEY, Loopback, UnavailableBackend};
use serde_json::json;
use thinkthen::{Decide, Outcome, Record, Settings};

#[test]
fn transport_causes_name_the_backend_without_credentials() {
    let refused = UnavailableBackend::new().unwrap();
    let (reset, _, _) = Loopback::holding(HeldReply::Reset).unwrap();
    let (timeout, _, _) = Loopback::holding(HeldReply::Headers).unwrap();
    for (url, cause) in [
        (refused.base(), "connection refused"),
        (reset.base(), "connection reset"),
        (timeout.base(), "timeout"),
    ] {
        // Darwin keeps a bound non-listening TCP port silent instead of refusing it.
        if cfg!(target_os = "macos") && cause == "connection refused" {
            continue;
        }
        let client = Settings::from_json(
            &json!({
                "backend":"openai", "url":format!("{url}?private-query={KEY}"),
                "retry":{"max_retries":0,"attempt_timeout_ms":200}
            })
            .to_string(),
        )
        .unwrap()
        .engine(|name| (name == "OPENAI_API_KEY").then(|| KEY.to_owned()))
        .unwrap();
        let question = Decide::new("Ready?").unwrap();
        let rows: Vec<_> = client
            .rows(&question, [Record::text("evidence")])
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(rows.len(), 1);
        let Outcome::Failed { error } = rows[0].outcome() else {
            panic!("expected failed row")
        };
        assert_eq!(error.code().as_str(), "backend.transport");
        let expected = format!("the openai backend could not be reached: {cause}");
        assert_eq!(error.message(), expected);
        assert!(!json!(rows).to_string().contains(KEY));
    }
}

#[test]
fn tls_handshake_failure_is_named_without_raw_peer_data() {
    if std::env::var_os("THINKTHEN_TEST_TLS_CHILD").is_none() {
        let mut command = std::process::Command::new(std::env::current_exe().unwrap());
        for name in [
            "HTTP_PROXY",
            "HTTPS_PROXY",
            "ALL_PROXY",
            "http_proxy",
            "https_proxy",
            "all_proxy",
        ] {
            command.env_remove(name);
        }
        let output = command
            .args([
                "--exact",
                "tls_handshake_failure_is_named_without_raw_peer_data",
            ])
            .env("THINKTHEN_TEST_TLS_CHILD", "1")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        assert!(!String::from_utf8_lossy(&output.stderr).contains(KEY));
        return;
    }
    let (base, peer) = loopback::invalid_tls_reply().unwrap();
    let url = format!("{base}?private={KEY}");
    let client =
        Settings::from_json(&json!({"url":url,"retry":{"attempt_timeout_ms":2000}}).to_string())
            .unwrap()
            .engine(|name| (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned()))
            .unwrap();
    let rows: Vec<_> = client
        .rows(&Decide::new("Ready?").unwrap(), [Record::text("evidence")])
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(rows.len(), 1);
    let Outcome::Failed { error } = rows[0].outcome() else {
        panic!("expected failed row")
    };
    assert_eq!(error.code().as_str(), "backend.transport");
    assert_eq!(
        error.message(),
        "the systemone backend could not be reached: TLS failure; check the operating system certificate store or THINKTHEN_CA_FILE"
    );
    assert!(!json!(rows).to_string().contains(KEY));
    peer.join().unwrap().unwrap();
}
