#![cfg(test)]
//! Shared local server and client construction for response-store behavior tests.
use crate::loopback::{KEY, Reply};
use serde_json::{Value, json};
use std::path::Path;
use std::time::Duration;
use thinkthen::{BackendName, Engine, Key, ResponseMode, ResponseStore, Retry, Settings};
pub(crate) fn config(path: &Path, mode: ResponseMode) -> ResponseStore {
    ResponseStore::new(path, mode, "test", "v1").unwrap()
}
pub(crate) fn settings(
    path: &Path,
    mode: ResponseMode,
    backend: BackendName,
    url: &str,
) -> Settings {
    Settings::default()
        .with_backend(backend)
        .url(url)
        .api_key(Key::new(KEY).unwrap())
        .retry(Retry::new(0, Duration::from_millis(1), Duration::from_secs(5)).unwrap())
        .response_store(Some(config(path, mode)))
}
pub(crate) fn replay(path: &Path, url: &str) -> Engine {
    Settings::default()
        .url(url)
        .response_store(Some(config(path, ResponseMode::Replay)))
        .engine(|name| {
            assert!(!name.ends_with("KEY"), "replay looked up credentials");
            None
        })
        .unwrap()
}
pub(crate) fn replay_fresh(path: &Path, url: &str) -> Engine {
    replay(path, url)
}
pub(crate) fn database(path: &Path) -> rusqlite::Connection {
    rusqlite::Connection::open(path.join("responses.sqlite3")).unwrap()
}
pub(crate) fn asks(body: &Value, backend: BackendName) -> usize {
    if backend == BackendName::SystemOne {
        body["questions"].as_object().unwrap().len()
    } else {
        body["questions"].as_array().unwrap().len()
    }
}
pub(crate) fn response(body: &Value, backend: BackendName, probability: f64) -> Reply {
    let answers = if backend == BackendName::SystemOne {
        Value::Object(
            body["questions"]
                .as_object()
                .unwrap()
                .keys()
                .map(|key| (key.clone(), json!({"type":"noul","noul":probability})))
                .collect(),
        )
    } else {
        json!(
            (0..asks(body, backend))
                .map(|_| json!({"type":"predicate","probability":probability}))
                .collect::<Vec<_>>()
        )
    };
    Reply::json(
        200,
        &json!({"answers":answers,"usage":{"input_tokens":13,"output_tokens":2}}),
    )
}
