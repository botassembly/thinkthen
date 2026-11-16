#![cfg(test)]
//! Request status determines whether a batch or the entire call failed.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback, Reply};
use serde_json::json;
use thinkthen::{Decide, Key, Record, Retry, Settings};

#[test]
fn first_batch_client_failures_stop_calls_but_server_failures_remain_rows() {
    for status in [400, 401, 403, 429, 500, 503] {
        let server = Loopback::start(move |_| Reply::json(status, &json!({}))).unwrap();
        let engine = Settings::default()
            .url(server.base())
            .api_key(Key::new(KEY).unwrap())
            .batch("10".parse().unwrap())
            .retry(
                Retry::new(
                    0,
                    std::time::Duration::ZERO,
                    std::time::Duration::from_secs(5),
                )
                .unwrap(),
            )
            .engine(|_| None)
            .unwrap();
        let result = engine
            .rows(
                &Decide::new("Ready?").unwrap(),
                (0..1000).map(|n| Record::text(format!("evidence {n}"))),
            )
            .unwrap()
            .collect::<Result<Vec<_>, _>>();
        if status < 500 {
            let error = result.unwrap_err();
            assert_eq!(error.code().as_str(), "backend.status");
            assert_eq!(error.http_status(), Some(status));
            assert!(
                server.count() <= 16,
                "status {status}: {} requests",
                server.count()
            );
        } else {
            let rows = result.unwrap();
            assert_eq!(rows.len(), 1000);
            assert!(
                rows.iter()
                    .all(|row| row.error().unwrap().http_status() == Some(status))
            );
            assert_eq!(server.count(), 100);
        }
    }
}

#[test]
fn authentication_failure_is_terminal_even_after_a_successful_batch() {
    for status in [400, 401, 403] {
        let server = Loopback::start(move |request| {
            let body = request.json().unwrap();
            if body.to_string().contains("healthy") {
                let answers: serde_json::Map<String, serde_json::Value> = body["questions"]
                    .as_object()
                    .unwrap()
                    .keys()
                    .map(|key| (key.clone(), json!({"type":"noul","noul":0.9})))
                    .collect();
                Reply::json(200, &json!({"answers":answers}))
            } else {
                Reply::json(status, &json!({}))
            }
        })
        .unwrap();
        let engine = Settings::default()
            .url(server.base())
            .api_key(Key::new(KEY).unwrap())
            .batch("1".parse().unwrap())
            .engine(|_| None)
            .unwrap();
        let call = engine.open(&Decide::new("Ready?").unwrap()).unwrap();
        let sender = call.sender();
        let mut rows = call.rows();
        sender.send(Record::text("healthy")).unwrap();
        assert!(rows.next().unwrap().unwrap().error().is_none());
        sender.send(Record::text("broken")).unwrap();
        sender.close();
        let result = rows.collect::<Result<Vec<_>, _>>();
        if status == 400 {
            let rows = result.unwrap();
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].error().unwrap().http_status(), Some(status));
        } else {
            assert_eq!(result.unwrap_err().http_status(), Some(status));
        }
        assert_eq!(server.count(), 2);
    }
}

#[test]
fn a_reply_cut_off_mid_body_fails_its_rows_without_a_resend() {
    for backend in ["systemone", "openai"] {
        let server = Loopback::start(|_| {
            let mut reply = Reply::json(200, &json!({}));
            reply.body = b"{".to_vec();
            reply.announced_length = Some(100);
            reply
        })
        .unwrap();
        let settings = json!({
            "backend": backend,
            "model": "fixture-model",
            "batch": 32,
            "retry": {"max_retries": 2, "initial_backoff_ms": 1, "attempt_timeout_ms": 60000},
        });
        let engine = Settings::from_json(&settings.to_string())
            .unwrap()
            .url(server.base())
            .api_key(Key::new(KEY).unwrap())
            .engine(|_| None)
            .unwrap();
        let rows = engine
            .rows(
                &Decide::new("Ready?").unwrap(),
                (0..32).map(|n| Record::text(format!("record {n}"))),
            )
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(rows.len(), 32, "{backend}");
        for row in &rows {
            let error = row.error().expect("a cut-off reply fails its rows");
            assert_eq!(error.code().as_str(), "backend.transport", "{backend}");
        }
        assert_eq!(
            server.count(),
            1,
            "{backend}: a reply with body bytes is not resent"
        );
    }
}
