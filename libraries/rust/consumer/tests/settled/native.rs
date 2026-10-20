//! Reuse canonical fixture admission without changing native packet execution.
#[path = "../../../../../crates/thinkthen/tests/packet_oracle/dump.rs"]
mod shared;
pub(crate) use shared::{Document, fixture_engine, request};
