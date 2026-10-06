//! Local MCP transport. The complete native execution adapter is not adopted yet.
//!
//! No CLI command enters this module until the native result/2 owner supplies
//! complete execution and its output schema. The transport never constructs facts.

#![allow(
    dead_code,
    reason = "0455 WIP awaits the complete native execution adapter before CLI exposure"
)]

mod admission;
mod dispatch;
mod input;
mod output;
mod protocol;
mod runtime;
mod tools;

#[cfg(test)]
mod tests;
