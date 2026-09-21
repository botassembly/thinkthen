//! Private request execution, recording, locking, and bounded scheduling.

pub(crate) mod annotate_schedule;
pub(crate) mod cache_lock;
pub(crate) mod error;
pub(crate) mod http;
pub(crate) mod prepared_request;
pub(crate) mod recorder;
pub(crate) mod request;
pub(crate) mod schedule;
pub(crate) mod workers;
