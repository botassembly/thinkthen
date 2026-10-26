//! Shared, bounded fault fixtures for installed surface adapters.
use crate::{Loopback, Reply, UnavailableBackend, lock};
use serde_json::{Value, json};
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

/// The shared workload sizes, client settings, reply rules and fault scenarios.
/// # Errors
/// Returns a JSON error if the bundled table is invalid.
pub fn stress_table() -> serde_json::Result<Value> {
    serde_json::from_str(include_str!("../../../conformance/stress.json"))
}
/// A bounded answering server or a reserved address that refuses connections.
#[derive(Debug)]
pub enum StressFixture {
    /// Answer requests using the shared fault sequence and reply rules.
    Server(Loopback),
    /// Refuse connections without a port reservation race.
    Refused(UnavailableBackend),
}
impl StressFixture {
    /// Open one named scenario from the shared table.
    /// Faults are consumed in request arrival order, then normal replies resume.
    /// # Errors
    /// Returns invalid scenario or socket setup errors.
    pub fn open(name: &str) -> io::Result<Self> {
        let table = stress_table().map_err(io::Error::other)?;
        let scenario = table
            .get("scenarios")
            .and_then(Value::as_array)
            .and_then(|cases| {
                cases
                    .iter()
                    .find(|case| case.get("name").is_some_and(|value| value == name))
            })
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "unknown stress scenario")
            })?;
        if name == "refused" {
            return UnavailableBackend::new().map(Self::Refused);
        }
        let faults = scenario
            .get("faults")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let replies = table
            .get("replies")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let attempt = AtomicUsize::new(0);
        let server = Loopback::start(move |request| {
            let mut index = attempt.fetch_add(1, Ordering::Relaxed);
            for fault in &faults {
                let repeat = fault.get("repeat").and_then(Value::as_u64).unwrap_or(1) as usize;
                if index >= repeat {
                    index -= repeat;
                    continue;
                }
                return fault_reply(fault, crate::case::reply(&replies, request));
            }
            crate::case::reply(&replies, request)
        })?;
        lock(&server.state).recording_limit = Some(8);
        Ok(Self::Server(server))
    }
    /// The backend base URL, valid until the fixture is dropped.
    #[must_use]
    pub fn base(&self) -> &str {
        match self {
            Self::Server(server) => server.base(),
            Self::Refused(server) => server.base(),
        }
    }
    /// Total requests and connections, active connections and bounded recording size.
    #[must_use]
    pub fn stats(&self) -> Value {
        match self {
            Self::Server(server) => {
                let state = lock(&server.state);
                json!({"requests":state.requests,"connections":state.connections,
                    "active_connections":server.owner.active(),"recorded_requests":state.seen.len()})
            }
            Self::Refused(_) => {
                json!({"requests":0,"connections":0,"active_connections":0,"recorded_requests":0})
            }
        }
    }
}
fn fault_reply(fault: &Value, mut reply: Reply) -> Reply {
    let number = |name, default| fault.get(name).and_then(Value::as_u64).unwrap_or(default);
    match fault.get("kind").and_then(Value::as_str) {
        Some("status") => {
            reply = Reply::json(
                number("status", 503) as u16,
                &json!({"error":"injected fault"}),
            );
            if let Some(value) = fault.get("retry_after").and_then(Value::as_str) {
                reply
                    .headers
                    .push(("retry-after".to_owned(), value.to_owned()));
            }
        }
        Some("slow") => reply.delay = Duration::from_millis(number("delay_ms", 20)),
        Some("truncated") => {
            reply.body = b"{".to_vec();
            reply.announced_length = Some(100);
        }
        Some("closed") => reply.status = 0,
        _ => {}
    }
    reply
}
