#![cfg(test)]
//! Public-client cancellation after observable SQLite and client-mutex contention.
use std::sync::{
    Mutex,
    mpsc::{Sender, channel},
};
use std::time::Duration;
use thinkthen::{Cancel, Code, Decide, Key, ResponseMode, ResponseStore, Settings};

struct Observer(Mutex<Option<Sender<&'static str>>>);
static OBSERVER: Observer = Observer(Mutex::new(None));
impl log::Log for Observer {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.target() == "thinkthen::response_store"
    }
    fn log(&self, record: &log::Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let message = record.args().to_string();
        let kind = if message.contains("SQLite") {
            "sqlite"
        } else if message.contains("client connection") {
            "mutex"
        } else {
            return;
        };
        if let Some(sender) = self.0.lock().unwrap().as_ref() {
            let _ = sender.send(kind);
        }
    }
    fn flush(&self) {}
}
#[test]
fn cancellation_interrupts_sqlite_and_connection_mutex_waits() {
    log::set_logger(&OBSERVER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    let (observed, waiting) = channel();
    *OBSERVER.0.lock().unwrap() = Some(observed);
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("store");
    let config = ResponseStore::new(&path, ResponseMode::Cache, "test", "v1")
        .unwrap()
        .busy_timeout(Duration::from_secs(86400))
        .unwrap();
    let engine = Settings::default()
        .url("http://127.0.0.1:1")
        .api_key(Key::new("test-credential").unwrap())
        .max_requests(Some(0))
        .response_store(Some(config))
        .engine(|_| None)
        .unwrap();
    let external = rusqlite::Connection::open(path.join("responses.sqlite3")).unwrap();
    external.execute_batch("BEGIN IMMEDIATE").unwrap();
    let run = |cancel: Cancel| {
        let client = engine.clone().cancel(cancel);
        let (done, result) = channel();
        let worker = std::thread::spawn(move || {
            done.send(
                client
                    .run(&Decide::new("Relevant?").unwrap(), "record")
                    .map(|_| ()),
            )
            .unwrap();
        });
        (worker, result)
    };
    let first_cancel = Cancel::default();
    let (first, first_result) = run(first_cancel.clone());
    assert_eq!(
        waiting.recv_timeout(Duration::from_secs(30)).unwrap(),
        "sqlite"
    );
    let second_cancel = Cancel::default();
    let (second, second_result) = run(second_cancel.clone());
    assert_eq!(
        waiting.recv_timeout(Duration::from_secs(30)).unwrap(),
        "mutex"
    );
    second_cancel.cancel();
    let second_outcome = second_result.recv_timeout(Duration::from_secs(30));
    first_cancel.cancel();
    let first_outcome = first_result.recv_timeout(Duration::from_secs(30));
    // A timeout must not leave the test holding either contender indefinitely.
    external.execute_batch("ROLLBACK").unwrap();
    first.join().unwrap();
    second.join().unwrap();
    assert_eq!(
        second_outcome.unwrap().unwrap_err().code(),
        Code::LocalCancelled
    );
    assert_eq!(
        first_outcome.unwrap().unwrap_err().code(),
        Code::LocalCancelled
    );
    assert_eq!(engine.usage().attempts, 0);
    assert!(
        waiting.try_recv().is_err(),
        "one diagnostic per contention cause"
    );
}
