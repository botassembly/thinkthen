//! Canonical backend checks through the compiled command.

const COMMAND: &[&str] = &["backends", "check"];

#[path = "check/cases.rs"]
mod cases;
