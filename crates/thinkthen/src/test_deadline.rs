//! Deadlines for test subprocesses, on both sides of the pipe.

pub(crate) mod child;
pub(crate) mod isolated;
#[cfg(test)]
mod child_tests;
use child::{run, wait};

pub(crate) use run::output;
pub(crate) use wait::finish;

mod support;
#[cfg(unix)]
pub(crate) use support::park_for_signal;
pub(crate) use support::whole_request;
