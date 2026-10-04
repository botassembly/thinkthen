//! Named backends (ticket 0334, ADR 0114) through the real command and the
//! real Rust builder.
//!
//! Every run starts from a cleared environment in a scratch home. A distinct
//! marker key sits in each key variable. Loopback listeners count requests by
//! the bearer marker they receive, and the tests assert the counts, never the
//! markers. A loopback proxy named in `HTTPS_PROXY` counts every connection a
//! run tries to open to a provider's host.
#![cfg(feature = "cli")]
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a failed fixture or child stops the proof"
)]

#[path = "named_backends/builder.rs"]
mod builder;
#[path = "named_backends/command.rs"]
mod command;
#[path = "named_backends/ollama.rs"]
mod ollama;
#[path = "named_backends/paths_0399.rs"]
mod paths_0399;
#[path = "named_backends/precedence.rs"]
mod precedence;
#[path = "named_backends/rate.rs"]
mod rate;
#[path = "named_backends/replay_0399.rs"]
mod replay_0399;
#[path = "named_backends/setups_0400.rs"]
mod setups_0400;
#[path = "named_backends/support.rs"]
mod support;
