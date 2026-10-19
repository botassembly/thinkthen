//! One bounded missing-pointer event and its caller-owned acknowledgement.
use crate::cli::interrupt::Signal;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, SyncSender}};
use std::time::Duration;

type Callback = Arc<dyn Fn(usize, &str) -> bool + Send + Sync>;
pub(super) struct Missing {
    pub(super) ordinal: usize,
    pub(super) pointer: String,
    acknowledge: SyncSender<bool>,
}
impl Missing {
    pub(super) fn acknowledge(self, accepted: bool) { let _sent = self.acknowledge.try_send(accepted); }
}
pub(super) struct Continuation {
    events: Receiver<Missing>,
    closed: Arc<AtomicBool>,
}
impl Continuation {
    pub(super) fn new(signal: Signal) -> (Self, Callback) {
        let (sender, events) = mpsc::sync_channel(1);
        let closed = Arc::new(AtomicBool::new(false));
        let stopped = closed.clone();
        let callback = Arc::new(move |ordinal: usize, pointer: &str| {
            if stopped.load(Ordering::Acquire) || signal.fired() { return false; }
            let (acknowledge, replies) = mpsc::sync_channel(1);
            if sender.try_send(Missing { ordinal, pointer: pointer.to_owned(), acknowledge }).is_err() { return false; }
            loop {
                match replies.recv_timeout(Duration::from_millis(50)) {
                    Ok(accepted) => return accepted,
                    Err(mpsc::RecvTimeoutError::Disconnected) => return false,
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        if stopped.load(Ordering::Acquire) || signal.fired() {
                            // A rendered closed-pipe row may already have been
                            // acknowledged before cancellation. Preserve that fact.
                            return replies.try_recv().unwrap_or(false);
                        }
                    }
                }
            }
        });
        (Self { events, closed }, callback)
    }
    pub(super) fn try_next(&self) -> Option<Missing> { self.events.try_recv().ok() }
    pub(super) fn close(&self) { self.closed.store(true, Ordering::Release); }
}
impl Drop for Continuation {
    fn drop(&mut self) { self.close(); }
}
