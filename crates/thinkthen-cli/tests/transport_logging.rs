#![cfg(test)]
//! Final artifacts cannot trace backend credentials through a host logger.

use std::sync::Mutex;

use loopback::{KEY, Loopback};
use thinkthen::{Backend, Decide, Engine, Key, Outcome, Record};

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
    let backend = Backend::new(server.base(), Key::new(KEY).unwrap(), "model");
    let mut count = 0;
    Engine::new(backend)
        .call(
            &Decide::new("Ready?").unwrap(),
            [Record::text("evidence")],
            |row| {
                count += 1;
                assert!(matches!(row.outcome(), Outcome::Answered { .. }));
            },
        )
        .unwrap();
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
