#![cfg(test)]
//! Failed HTTP rows retain a bounded reason without disclosing credentials.
use loopback::{Loopback, Reply};
use serde_json::json;
use thinkthen::{Decide, Key, Record, Settings};

#[test]
fn failed_replies_keep_clean_reasons_and_stable_status() {
    let key = "synthetic-private-key";
    for (status, body, excerpt) in [
        (
            401,
            format!("authorization denied\n{key} sk-other-secret"),
            "authorization denied[redacted] [redacted]".to_owned(),
        ),
        (
            402,
            "quota\0 exhausted".to_owned(),
            "quota exhausted".to_owned(),
        ),
        (
            429,
            "rate limit reached".to_owned(),
            "rate limit reached".to_owned(),
        ),
        (500, "é".repeat(300), "é".repeat(200)),
        (500, "".to_owned(), "".to_owned()),
        (
            401,
            r#"{"api_key":"tiny","error":"authorization denied"}"#.to_owned(),
            r#"{"api_key":"[redacted]","error":"authorization denied"}"#.to_owned(),
        ),
        (
            401,
            "Bearer tiny".to_owned(),
            "Bearer [redacted]".to_owned(),
        ),
        (
            401,
            r#"{"authorization":"tiny"}"#.to_owned(),
            r#"{"authorization":"[redacted]"}"#.to_owned(),
        ),
        (
            402,
            "quota\u{202e} exhausted".to_owned(),
            "quota exhausted".to_owned(),
        ),
        (
            400,
            format!("{}!!!!!", "a".repeat(16380)),
            "[redacted]!!!!".to_owned(),
        ),
        (
            401,
            format!("{}{key}", " ".repeat(16380)),
            "[redacted]".to_owned(),
        ),
    ] {
        let server = Loopback::start(move |_| {
            let mut reply = Reply::json(status, &json!(null));
            reply.body = body.as_bytes().to_vec();
            reply
        })
        .unwrap();
        let engine = Settings::from_json(
            &json!({"url":server.base(),"retry":{"max_retries":0}}).to_string(),
        )
        .unwrap()
        .api_key(Key::new(key).unwrap())
        .engine(|_| None)
        .unwrap();
        let rows = engine
            .rows(&Decide::new("Ready?").unwrap(), [Record::text("evidence")])
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(rows.len(), 1);
        let error = rows[0].error().unwrap();
        assert_eq!(error.code().as_str(), "backend.status");
        assert_eq!(json!(error)["http_status"], status);
        let expected = if excerpt.is_empty() {
            format!("the backend answered status {status}")
        } else {
            format!("the backend answered status {status}: {excerpt}")
        };
        assert_eq!(error.message(), expected);
        assert!(!json!(rows).to_string().contains(key));
        assert_eq!(server.count(), 1);
    }
}
