#![cfg(test)]
//! Backend credential ownership and the boundary for unencrypted requests.

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{Loopback, Reply};
use serde_json::json;
use thinkthen::{BackendName, Code, Decide, Family, Key, Outcome, Record, Retry, Settings};

#[test]
fn redirects_cannot_send_evidence_to_another_address() {
    let destination = Loopback::noul(0.9).unwrap();
    for custom_retry in [false, true] {
        let location = format!("{}/systemone", destination.base());
        let redirect = Loopback::start(move |_| Reply::redirect(303, &location)).unwrap();
        let settings = Settings::default()
            .url(redirect.base())
            .api_key(Key::new("synthetic-key").unwrap())
            .model("model");
        let settings = if custom_retry {
            settings.retry(Retry::default())
        } else {
            settings
        };
        let engine = settings.engine(|_| None).unwrap();
        let mut count = 0;
        engine
            .rows(&Decide::new("Is it?").unwrap(), [Record::text("text")])
            .unwrap()
            .map(|row| row.unwrap())
            .for_each(|row| {
                count += 1;
                let Outcome::Failed { error } = row.outcome() else {
                    panic!("a redirect must not forward the request");
                };
                assert_eq!(error.code().as_str(), "backend.status");
            });
        assert_eq!(count, 1);
        assert_eq!(redirect.count(), 1);
    }
    assert_eq!(destination.count(), 0);
}

#[test]
fn each_adapter_reads_only_its_own_key_and_address() {
    for adapter in [BackendName::SystemOne, BackendName::OpenAi] {
        for address_source in ["primary", "fallback", "explicit"] {
            if adapter == BackendName::OpenAi && address_source == "fallback" {
                continue;
            }
            let system = Loopback::noul(0.9).unwrap();
            let openai = Loopback::start(|_| {
                Reply::json(
                    200,
                    &json!({"answers":[{"type":"predicate","probability":0.9}]}),
                )
            })
            .unwrap();
            let (server, other, key) = if adapter == BackendName::SystemOne {
                (&system, &openai, "synthetic-system")
            } else {
                (&openai, &system, "synthetic-openai")
            };
            let mut settings = Settings::default().batch("50".parse().unwrap());
            if address_source == "explicit" {
                settings = settings.url(server.base());
            }
            let engine = settings
                .engine(|name| match name {
                    "THINKTHEN_BACKEND" => Some(adapter.as_str().to_owned()),
                    "THINKTHEN_API_KEY" => Some("synthetic-system".to_owned()),
                    "OPENAI_API_KEY" => Some("synthetic-openai".to_owned()),
                    "THINKTHEN_URL" if address_source == "fallback" => None,
                    "THINKTHEN_URL" if address_source == "explicit" => {
                        Some(other.base().to_owned())
                    }
                    "THINKTHEN_URL" => Some(system.base().to_owned()),
                    "THINKTHEN_BASE_URL" if address_source == "fallback" => {
                        Some(system.base().to_owned())
                    }
                    "THINKTHEN_BASE_URL" => Some(other.base().to_owned()),
                    "OPENAI_BASE_URL" if address_source == "explicit" => {
                        Some(other.base().to_owned())
                    }
                    "OPENAI_BASE_URL" => Some(openai.base().to_owned()),
                    _ => panic!("unexpected variable"),
                })
                .unwrap();
            let rows: Vec<_> = engine
                .rows(&Decide::new("Is it?").unwrap(), [Record::text("text")])
                .unwrap()
                .map(|row| row.unwrap())
                .collect();
            assert_eq!(rows.len(), 1);
            assert!(matches!(rows[0].outcome(), Outcome::Answered { .. }));
            assert_eq!(other.count(), 0);
            let requests = server.requests();
            assert_eq!(requests.len(), 1);
            assert!(requests[0].header("authorization") == Some(format!("Bearer {key}").as_str()));
        }
    }
}

#[test]
fn the_other_adapters_key_cannot_fill_a_missing_key() {
    for (adapter, other_key) in [
        (BackendName::SystemOne, "OPENAI_API_KEY"),
        (BackendName::OpenAi, "THINKTHEN_API_KEY"),
    ] {
        let error = Settings::default()
            .with_backend(adapter)
            .engine(|name| (name == other_key).then(|| "synthetic-other".to_owned()))
            .unwrap_err();
        assert_eq!(error.code().as_str(), "usage.key-missing");
        assert!(!error.to_string().contains("synthetic-other"));
    }
}

#[test]
fn only_https_and_named_loopback_http_are_admitted_and_sent() {
    let server = Loopback::noul(0.9).unwrap();
    for base in [
        server.base().replace("127.0.0.1", "127.0.0.2"),
        "http://example.invalid/v1".to_owned(),
        "http://localhost.example.invalid/v1".to_owned(),
        "http://127.0.0.1.example.invalid/v1".to_owned(),
        "http://[::2]/v1".to_owned(),
        "http://[broken/v1".to_owned(),
        "HTTP://example.invalid/v1".to_owned(),
        "http://localhost@localhost.example.invalid/v1".to_owned(),
        server.base().replacen("http:", "socks:", 1),
        server.base().replacen("http:", "socks4:", 1),
        server.base().replacen("http:", "socks4a:", 1),
        server.base().replacen("http:", "socks5:", 1),
        server.base().replacen("http:", "socks5h:", 1),
        server.base().replacen("http:", "ftp:", 1),
        server.base().replacen("http:", "file:", 1),
        server.base().replacen("http:", "ws:", 1),
        "/v1".to_owned(),
    ] {
        let error = Settings::default()
            .url(&base)
            .api_key(Key::new("synthetic-key").unwrap())
            .engine(|_| None)
            .unwrap_err();
        assert_eq!(error.code().as_str(), "usage.url", "{base}");
        assert_eq!(error.code().family(), Family::Usage);
    }
    assert_eq!(server.count(), 0);
    for base in [
        "http://localhost:1234/v1",
        "http://127.0.0.1:1234/v1",
        "http://[::1]:1234/v1",
        "https://example.invalid/v1",
    ] {
        Settings::default()
            .url(base)
            .api_key(Key::new("synthetic-key").unwrap())
            .engine(|_| None)
            .unwrap();
    }
}

#[test]
fn backend_names_reject_unknown_values_before_credentials() {
    let error = Settings::from_json(r#"{"backend":"a-new-name"}"#).unwrap_err();
    assert_eq!(error.code().as_str(), "usage.backend");
    assert!(!error.message().contains("stub"));
    for (code, family) in [
        (Code::UsageQuestion, "usage"),
        (Code::BackendStatus, "backend"),
        (Code::LocalIo, "local"),
    ] {
        assert_eq!(code.family().as_str(), family);
    }
}
