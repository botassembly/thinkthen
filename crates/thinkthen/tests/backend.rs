#![cfg(test)]
//! Backend credential ownership and the boundary for unencrypted requests.

use loopback::{KEY, Loopback, Reply};
use serde_json::json;
use thinkthen::{Adapter, Backend, Decide, Engine, Family, Key, Outcome, Record, RetryPolicy};

#[test]
fn redirects_cannot_send_evidence_to_another_address() {
    let destination = Loopback::noul(0.9).unwrap();
    for custom_retry in [false, true] {
        let location = format!("{}/systemone", destination.base());
        let redirect = Loopback::start(move |_| Reply::redirect(303, &location)).unwrap();
        let backend = Backend::new(redirect.base(), Key::new("synthetic-key").unwrap(), "model");
        let engine = Engine::new(backend);
        let engine = if custom_retry {
            engine.retry_policy(RetryPolicy::default())
        } else {
            engine
        };
        let mut count = 0;
        engine
            .call(
                &Decide::new("Is it?").unwrap(),
                [Record::text("text")],
                |row| {
                    count += 1;
                    let Outcome::Failed { error } = row.outcome() else {
                        panic!("a redirect must not forward the request");
                    };
                    assert_eq!(error.kind().code(), "backend.status");
                },
            )
            .unwrap_err();
        assert_eq!(count, 1);
        assert_eq!(redirect.count(), 1);
    }
    assert_eq!(destination.count(), 0);
}

#[test]
fn each_adapter_reads_only_its_own_key_and_address() {
    let system = Loopback::noul(0.9).unwrap();
    let openai = Loopback::start(|_| {
        Reply::json(
            200,
            &json!({"answers":[{"type":"predicate","probability":0.9}]}),
        )
    })
    .unwrap();
    for (adapter, url, key_name, key, server) in [
        (
            Adapter::SystemOne,
            None,
            "THINKTHEN_API_KEY",
            "synthetic-system",
            &system,
        ),
        (
            Adapter::OpenAi,
            Some(openai.base()),
            "OPENAI_API_KEY",
            "synthetic-openai",
            &openai,
        ),
    ] {
        let backend = Backend::resolve_adapter(adapter, url, None, |name| match name {
            "THINKTHEN_API_KEY" => {
                assert_eq!(key_name, name);
                Some("synthetic-system".to_owned())
            }
            "OPENAI_API_KEY" => {
                assert_eq!(key_name, name);
                Some("synthetic-openai".to_owned())
            }
            "THINKTHEN_BASE_URL" => {
                assert_eq!(adapter, Adapter::SystemOne);
                Some(system.base().to_owned())
            }
            _ => panic!("unexpected variable"),
        })
        .unwrap();
        Engine::new(backend)
            .call(
                &Decide::new("Is it?").unwrap(),
                [Record::text("text")],
                |row| assert!(matches!(row.outcome(), Outcome::Answered { .. })),
            )
            .unwrap();
        let requests = server.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(
            requests[0].header("authorization"),
            Some(format!("Bearer {key}").as_str())
        );
    }
    let backend = Backend::resolve_adapter(Adapter::OpenAi, None, None, |name| {
        assert_eq!(name, "OPENAI_API_KEY");
        Some("synthetic-openai".to_owned())
    })
    .unwrap();
    assert!(format!("{backend:?}").contains("https://api.openai.com/v1"));
}

#[test]
fn the_other_adapters_key_cannot_fill_a_missing_key() {
    for (adapter, other_key) in [
        (Adapter::SystemOne, "OPENAI_API_KEY"),
        (Adapter::OpenAi, "THINKTHEN_API_KEY"),
    ] {
        let error = Backend::resolve_adapter(adapter, None, None, |name| {
            (name == other_key).then(|| "synthetic-other".to_owned())
        })
        .unwrap_err();
        assert_eq!(error.kind().code(), "usage.key-missing");
        assert!(!error.to_string().contains("synthetic-other"));
    }
}

#[test]
fn only_https_and_named_loopback_http_are_admitted_and_sent() {
    let server = Loopback::noul(0.9).unwrap();
    let question = Decide::new("Is it?").unwrap();
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
        let error =
            Backend::resolve(Some(&base), None, |_| Some("synthetic-key".to_owned())).unwrap_err();
        assert_eq!(error.kind().code(), "usage.url", "{base}");
        assert_eq!(error.kind().family(), Family::Usage);
        let backend = Backend::new(&base, Key::new("synthetic-key").unwrap(), "model");
        let mut count = 0;
        Engine::new(backend)
            .call(&question, [Record::text("text")], |row| {
                count += 1;
                let Outcome::Failed { error } = row.outcome() else {
                    panic!("an unsafe address must fail");
                };
                assert_eq!(error.kind().code(), "usage.url", "{base}");
            })
            .unwrap_err();
        assert_eq!(count, 1);
    }
    assert_eq!(server.count(), 0);
    for base in [
        "http://localhost:1234/v1",
        "http://127.0.0.1:1234/v1",
        "http://[::1]:1234/v1",
        "https://example.invalid/v1",
    ] {
        Backend::resolve(Some(base), None, |_| Some("synthetic-key".to_owned())).unwrap();
    }
}

#[test]
fn selected_model_bounds_close_requests_and_refuse_oversized_asks_as_backend_errors() {
    use std::num::NonZeroUsize;
    use thinkthen::{BackendEntry, BackendTable, Limits};
    let limits = |bytes, questions, options| Limits {
        request_bytes: NonZeroUsize::new(bytes).unwrap(),
        questions: NonZeroUsize::new(questions),
        options: NonZeroUsize::new(options),
    };
    for (model, expected, failed) in [
        ("default-model", vec![5], false),
        ("two-questions", vec![2, 2, 1], false),
        ("small-body", vec![1, 1, 1, 1, 1], false),
        ("too-small", vec![], true),
    ] {
        let server = Loopback::noul(0.9).unwrap();
        let mut table = BackendTable::default();
        table.insert(BackendEntry::new(
            "new-system-one",
            Adapter::SystemOne,
            "https://unused.invalid",
            "WRONG_KEY",
            "wrong-model",
            limits(1, 1, 1),
        ));
        table.insert(
            BackendEntry::new(
                "new-system-one",
                Adapter::SystemOne,
                server.base(),
                "NAMED_KEY",
                "default-model",
                limits(96_000, 0, 255),
            )
            .model_limits("two-questions", limits(96_000, 2, 255))
            .model_limits("small-body", limits(400, 0, 255))
            .model_limits("too-small", limits(30, 0, 255)),
        );
        let backend = table
            .resolve("new-system-one", None, Some(model), |name| {
                assert_eq!(name, "NAMED_KEY");
                Some(KEY.to_owned())
            })
            .unwrap();
        let mut rows = Vec::new();
        let result = Engine::new(backend).call(
            &Decide::new("Ready?").unwrap(),
            (0..5).map(|n| Record::text(format!("{} {n}", "x".repeat(150)))),
            |row| rows.push(serde_json::to_value(row).unwrap()),
        );
        assert_eq!(result.is_err(), failed);
        assert_eq!(rows.len(), 5, "{model}");
        for row in rows {
            if failed {
                assert_eq!(row["error"]["code"], "backend.limit");
                assert!(
                    row["error"]["message"]
                        .as_str()
                        .unwrap()
                        .contains("30 bytes")
                );
            } else {
                assert_eq!(row["answer"], true);
                assert_eq!(row["detail"]["source"]["backend"], "new-system-one");
                assert_eq!(row["detail"]["source"]["model"], model);
            }
        }
        let mut sizes: Vec<_> = server
            .requests()
            .iter()
            .map(|request| {
                request.json().unwrap()["questions"]
                    .as_object()
                    .unwrap()
                    .len()
            })
            .collect();
        sizes.sort_unstable();
        let mut expected = expected;
        expected.sort_unstable();
        assert_eq!(sizes, expected, "{model}");
        if model == "small-body" {
            assert!(
                server
                    .requests()
                    .iter()
                    .all(|request| request.body.len() <= 400)
            );
        }
    }
}

#[test]
fn backend_names_admit_as_strings_and_unknown_names_fail_before_credentials() {
    use thinkthen::{Client, ErrorKind, SdkSettings};
    SdkSettings::from_json(r#"{"adapter":"a-new-name"}"#).unwrap();
    let error = Client::from_json(r#"{"adapter":"a-new-name"}"#, |name| {
        panic!("unknown backend read {name}")
    })
    .unwrap_err();
    assert_eq!(error.kind().code(), "usage.backend");
    for (kind, family) in [
        (ErrorKind::Question, "usage"),
        (ErrorKind::BackendLimit, "backend"),
        (ErrorKind::Io, "local"),
    ] {
        assert_eq!(kind.family().as_str(), family);
    }
}
