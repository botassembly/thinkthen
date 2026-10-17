//! MCP contracts through the compiled command and its actual stdio pipes.
#![cfg(feature = "cli")]
#[path = "../src/test_deadline/child.rs"]
mod child;
#[path = "mcp_stdio/execution.rs"]
mod execution;
#[path = "mcp_stdio/protocol.rs"]
mod protocol;
#[path = "mcp_stdio/refusal.rs"]
mod refusal;
#[path = "mcp_stdio/session.rs"]
mod session;
