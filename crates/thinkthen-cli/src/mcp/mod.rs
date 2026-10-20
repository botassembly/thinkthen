//! Installed local MCP stdio adapter over one native engine.

mod admission;
mod executor;
mod input;
mod inputs;
mod output;
mod protocol;
mod request;
mod runtime;
pub(crate) mod startup;
mod tools;

#[cfg(test)]
mod tests;
