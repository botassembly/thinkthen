#![cfg(test)]
//! An explicit private native artifact owns both loopback routes.
mod support;
#[cfg(feature = "fixture-routing")]
mod tests {
    use super::support;
    use loopback::{KEY, Loopback};
    use serde_json::json;
    #[test]
    fn private_native_routes_preserve_backend_model_and_key_identity() {
        let case = json!({"replies":[{"probability":0.8123456789012345}]});
        let systemone = Loopback::case(&case).unwrap();
        let openai = Loopback::case(&case).unwrap();
        let output = support::command("thinkthen-fixture-consumer")
            .args([systemone.base(), openai.base()])
            .env("THINKTHEN_API_KEY", KEY)
            .env("OPENAI_API_KEY", KEY)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        for (server, path) in [(&systemone, "/v1/systemone"), (&openai, "/v1/decisions")] {
            let requests = server.requests();
            assert_eq!(requests.len(), 1);
            assert_eq!(requests[0].path, path);
            assert_eq!(
                requests[0].header("authorization"),
                Some(format!("Bearer {KEY}").as_str())
            );
        }
    }
}
