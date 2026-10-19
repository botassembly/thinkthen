//! Read-only projections of the existing count-only usage store.
use crate::{Counters, Error, ErrorKind, engine::usage};
use std::path::Path;

/// Persisted counts for one month and all recorded months.
/// These counts contain no questions, evidence, credentials or answer keys.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UsageTotals {
    /// The selected month's counts.
    pub month: Counters,
    /// Counts across every recorded month.
    pub total: Counters,
}

/// The current UTC usage month, using the store's existing calendar rules.
#[must_use]
pub fn current_usage_month() -> String {
    usage::month_now()
}

/// Read the caller's named count-only usage store without changing it.
/// Missing state returns zero counts. The existing shared lock and ownership
/// checks apply; failure messages name the explicit caller-owned path.
/// # Errors
/// Refuses an invalid month or unsafe, corrupt or unreadable usage state.
pub fn read_usage(path: &Path, month: &str) -> Result<UsageTotals, Error> {
    if !usage::recognized_month(&format!("{month}.json")) {
        return Err(Error::new(ErrorKind::Usage, "usage month must be YYYY-MM"));
    }
    let totals = usage::read(path, month)
        .map_err(|error| Error::new(ErrorKind::Local, error.sentence(Some(path))))?;
    Ok(UsageTotals {
        month: Counters::of(&totals.month),
        total: Counters::of(&totals.total),
    })
}
