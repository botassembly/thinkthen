//! The command process owns its console signal disposition.
#[path = "windows/console/ffi.rs"]
mod console;
pub(crate) use console::enable_interrupts;

#[cfg(test)]
#[path = "../../thinkthen/src/windows/test_support.rs"]
pub(crate) mod test_support;
