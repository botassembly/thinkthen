#![cfg(test)]
//! The owned test route changes only transport destination and disappears on drop.
#[cfg(feature = "fixture-routing")]
mod tests {
    use loopback::{KEY, Loopback};
    use serde_json::json;
    use thinkthen::{
        BackendName, Code, Settings, questions, records::Record, test_fixture::RouteGuard,
    };
    #[test]
    fn both_owned_routes_are_loopback_only_and_preserve_backend_model_and_key_identity() {
        assert_eq!(
            RouteGuard::install("https://remote.invalid/v1", "http://127.0.0.1:1/v1")
                .unwrap_err()
                .code(),
            Code::UsageUrl
        );
        let case = json!({"replies":[{"probability":0.8123456789012345}]});
        let systemone = Loopback::case(&case).unwrap();
        let openai = Loopback::case(&case).unwrap();
        let routes = RouteGuard::install(systemone.base(), openai.base()).unwrap();
        assert_eq!(
            RouteGuard::install(systemone.base(), openai.base())
                .unwrap_err()
                .code(),
            Code::UsageSettings
        );
        for (backend, key, path, server) in [
            (
                BackendName::SystemOne,
                "THINKTHEN_API_KEY",
                "/v1/systemone",
                &systemone,
            ),
            (
                BackendName::OpenAi,
                "OPENAI_API_KEY",
                "/v1/decisions",
                &openai,
            ),
        ] {
            let engine = Settings::default()
                .with_backend(backend)
                .model("fixture-model")
                .batch(1_u64.try_into().unwrap())
                .engine(|name| {
                    assert_eq!(name, key);
                    Some(KEY.to_owned())
                })
                .unwrap();
            let row = engine
                .run(
                    &questions::Decide::new("Ready?").unwrap(),
                    Record::text("evidence"),
                )
                .unwrap();
            assert_eq!(row.source().backend(), backend);
            assert_eq!(row.source().model(), "fixture-model");
            let sent = server.requests();
            assert_eq!(sent.len(), 1);
            assert_eq!(sent[0].path, path);
            assert_eq!(
                sent[0].header("authorization"),
                Some(format!("Bearer {KEY}").as_str())
            );
        }
        drop(routes);
        assert!(
            Settings::default()
                .with_backend(BackendName::OpenAi)
                .url("http://remote.invalid/v1")
                .engine(|_| panic!("URL admission must precede credentials"))
                .is_err()
        );
    }
}
