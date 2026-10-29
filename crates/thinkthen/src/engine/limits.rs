//! Scheduling bounds shared with the private loopback proof configuration.

/// How many requests may be open at once.
pub(super) const WORKERS: usize = 4;
/// Bound queued and completed chunks independently of worker occupancy.
pub(super) const REORDER_WINDOW: usize = WORKERS * 4;
