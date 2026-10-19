//! Generic subprocess support shared with library tests, without native state access.
#[path = "../../thinkthen/src/test_deadline/child.rs"]
pub(crate) mod child;
#[path = "../../thinkthen/src/test_deadline/support.rs"]
mod support;
pub(crate) use child::{run::output, wait::finish};
#[cfg(unix)]
pub(crate) use support::park_for_signal;
