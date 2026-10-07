//! Installed local MCP stdio adapter over one native engine.

mod admission;
mod composition;
mod dispatch;
mod executor;
mod input;
mod inputs;
mod output;
mod protocol;
mod runtime;
pub(crate) mod startup;
mod tools;

#[cfg(test)]
mod tests;
