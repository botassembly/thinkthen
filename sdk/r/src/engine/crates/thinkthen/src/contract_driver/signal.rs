//! One wake source covers output publication and input-capacity changes.
use std::sync::{Condvar, Mutex, PoisonError};
use std::time::Duration;
#[derive(Debug, Default)]
pub(super) struct Signal {
    version: Mutex<u64>,
    changed: Condvar,
}
impl Signal {
    pub(super) fn version(&self) -> u64 {
        *self.version.lock().unwrap_or_else(PoisonError::into_inner)
    }
    pub(super) fn notify(&self) {
        let mut version = self.version.lock().unwrap_or_else(PoisonError::into_inner);
        *version = version.wrapping_add(1);
        drop(version);
        self.changed.notify_all();
    }
    pub(super) fn wait(&self, version: u64, timeout: Option<Duration>) {
        let current = self.version.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(timeout) = timeout {
            drop(
                self.changed
                    .wait_timeout_while(current, timeout, |current| *current == version)
                    .unwrap_or_else(PoisonError::into_inner),
            );
        } else {
            drop(
                self.changed
                    .wait_while(current, |current| *current == version)
                    .unwrap_or_else(PoisonError::into_inner),
            );
        }
    }
}
