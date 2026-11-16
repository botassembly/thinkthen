#![cfg(test)]
//! Final artifacts cannot trace backend credentials through a host logger.

#[path = "../test-support/server/mod.rs"]
#[allow(
    dead_code,
    reason = "This fixture subset retains server lifecycle ownership fields."
)]
pub mod loopback;

use std::sync::Mutex;

use loopback::{KEY, Loopback};
use thinkthen::{Decide, Key, Outcome, Record, Settings};

struct HostLogger(Mutex<Vec<(String, String)>>);

impl log::Log for HostLogger {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        self.0
            .lock()
            .unwrap()
            .push((record.target().to_owned(), record.args().to_string()));
    }

    fn flush(&self) {}
}

static LOGGER: HostLogger = HostLogger(Mutex::new(Vec::new()));

#[test]
fn final_artifacts_do_not_trace_backend_credentials() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    let server = Loopback::noul(0.9).unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .model("model")
        .engine(|_| None)
        .unwrap();
    let mut count = 0;
    engine
        .rows(&Decide::new("Ready?").unwrap(), [Record::text("evidence")])
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| {
            count += 1;
            assert!(matches!(row.outcome(), Outcome::Answered { .. }));
        });
    assert_eq!(count, 1);
    assert_eq!(server.count(), 1);
    assert!(server.requests()[0].header("authorization") == Some(format!("Bearer {KEY}").as_str()));
    log::debug!(target: "embedding_host", "host debug logging remains enabled");
    let records = LOGGER.0.lock().unwrap();
    assert!(records.iter().any(|(target, _)| target == "embedding_host"));
    assert!(!records.iter().any(|(_, message)| message.contains(KEY)));
    // The dependency splits wire bytes into hex/ASCII rows. Reassemble the
    // printable columns so a key crossing a row boundary cannot escape proof.
    let wire: String = records
        .iter()
        .filter(|(target, _)| target == "ureq_proto::util")
        .filter_map(|(_, message)| message.rsplit_once(' ').map(|(_, ascii)| ascii))
        .collect();
    assert!(
        !wire.contains(KEY),
        "transport tracing exposed a backend key"
    );
}
