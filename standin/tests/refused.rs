//! A refused connection fails at once, and the counter counts nothing for
//! it (ruling 5 of the adversarial review, 2026-09-21; review finding,
//! 2026-09-22: the backoff waits were pointless for a refusal).
//!
//! One test per file, because the engine reads the environment once per
//! process; this binary owns its process.

use std::time::Duration;
use std::time::Instant;

use thinkthen_contract::{Connector, EngineConfig, ErrorKind, Question};
use thinkthen_standin::StandinConnector;

/// A refused connection is not retryable and does not burn the backoff: the
/// call fails in well under the first one-second wait.
#[test]
fn a_refused_send_fails_at_once() {
    let engine = StandinConnector
        .connect(&EngineConfig {
            address: Some("http://127.0.0.1:1/v1".into()),
            timeout: Some(Duration::from_millis(250)),
            max_retries: Some(3),
            width: Some(1),
            ..EngineConfig::default()
        })
        .expect("builds");
    let before = engine.usage().requests;
    let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("parses");
    let started = Instant::now();
    let failed = engine
        .decide(&question, "i want a refund")
        .expect_err("port 1 refuses");
    let wall = started.elapsed();
    assert_eq!(failed.kind, ErrorKind::Backend, "{failed}");
    assert!(!failed.retryable, "a refusal is not retryable: {failed}");
    assert!(failed.message.contains("refused"), "{failed}");
    assert!(
        wall < Duration::from_millis(900),
        "no backoff waits for a refusal, wall was {wall:?}"
    );
    let after = engine.usage().requests;
    assert_eq!(
        after, before,
        "a refused connection left nothing, so nothing counts (was {before}, now {after})"
    );
}
