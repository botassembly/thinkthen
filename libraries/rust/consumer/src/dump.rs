//! Use the same public-API dumper as the core currency test.
#[path = "../../../../crates/thinkthen/tests/packet_oracle/dump.rs"]
mod native;
pub(crate) use native::run;
