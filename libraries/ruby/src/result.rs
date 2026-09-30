//! Owned call accounting copied before the Ruby worker hands its result over.

use thinkthen::Facts;

use crate::call::Output;

/// A finished call: its value, its facts, and each question event as JSON.
#[derive(Debug)]
pub(crate) struct Completed {
    pub(crate) value: Output,
    pub(crate) facts: Facts,
    pub(crate) details: Vec<String>,
}
