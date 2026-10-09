//! Prepare durable bookkeeping before the last stop check, then count one send.

use std::path::Path;
use std::sync::atomic::Ordering;
use std::sync::{Arc, MutexGuard};
use std::thread;

use crate::engine::error::Error;

use super::{Counters, Counts, Queue, Shared, carried, month_now_into, write_behind};

const OVERFLOW: &str = "request attempt count overflow";

/// An uncounted send. Dropping it publishes no count or month row.
pub(crate) struct PreparedAttempt<'a> {
    queue: MutexGuard<'a, Queue>,
    shared: &'a Shared,
    path: Option<&'a Path>,
    month: String,
    writer_failed: bool,
}

impl Counters {
    /// Do the potentially blocking ledger setup before the final stop check.
    pub(crate) fn prepare_attempt(&self) -> Result<PreparedAttempt<'_>, Error> {
        self.check_readable()?;
        let mut queue = self
            .shared
            .queue
            .lock()
            .map_err(|_| Error::Defect("usage counter lock poisoned"))?;
        queue.pending.reserve(1);
        let month = String::with_capacity(32);
        let writer_failed = if let Some(path) = self.path.as_deref()
            && queue.writer.is_none()
            && !self.shared.failed.load(Ordering::Acquire)
        {
            let (path, shared, carried) = (path.to_path_buf(), Arc::clone(&self.shared), carried());
            match thread::Builder::new().spawn(move || write_behind(&path, &shared, carried)) {
                Ok(writer) => {
                    queue.writer = Some(writer);
                    false
                }
                Err(_) => true,
            }
        } else {
            false
        };
        Ok(PreparedAttempt {
            queue,
            shared: &self.shared,
            path: self.path.as_deref(),
            month,
            writer_failed,
        })
    }
}

impl PreparedAttempt<'_> {
    /// Refuse overflow before any mark; otherwise publish one in-flight attempt.
    pub(crate) fn mark(mut self, retry: bool) -> Result<(), Error> {
        let delta = Counts {
            requests_sent: 1,
            retries: u64::from(retry),
            ..Counts::default()
        };
        let total = self
            .queue
            .totals
            .checked_add(delta)
            .ok_or(Error::Defect(OVERFLOW))?;
        if self.path.is_some() {
            month_now_into(&mut self.month);
        }
        let pending = match self.queue.pending.last() {
            Some((month, sum)) if self.path.is_some() && *month == self.month => {
                Some(sum.checked_add(delta).ok_or(Error::Defect(OVERFLOW))?)
            }
            _ => None,
        };
        self.queue.totals = total;
        if self.path.is_some() {
            if !self.shared.failed.load(Ordering::Acquire) {
                match (pending, self.queue.pending.last_mut()) {
                    (Some(next), Some((_, sum))) => *sum = next,
                    _ => self.queue.pending.push((self.month, delta)),
                }
            }
            if self.writer_failed || self.queue.writer.is_none() {
                self.shared.failed.store(true, Ordering::Release);
                self.queue.pending.clear();
            }
            self.shared.changed.notify_all();
        }
        Ok(())
    }
}
