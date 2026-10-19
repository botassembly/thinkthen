//! One bounded missing-pointer event and its caller-owned acknowledgement.
use crate::cli::interrupt::Signal;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc::{self, Receiver, SyncSender},
};
use std::time::Duration;

type Callback = Arc<dyn Fn(usize, &str) -> bool + Send + Sync>;
pub(super) struct Missing {
    pub(super) ordinal: usize,
    pub(super) pointer: String,
    acknowledge: SyncSender<bool>,
}
impl Missing {
    pub(super) fn acknowledge(self, accepted: bool) {
        let _sent = self.acknowledge.try_send(accepted);
    }
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
            if stopped.load(Ordering::Acquire) || signal.fired() {
                return false;
            }
            let (acknowledge, replies) = mpsc::sync_channel(1);
            if sender
                .try_send(Missing {
                    ordinal,
                    pointer: pointer.to_owned(),
                    acknowledge,
                })
                .is_err()
            {
                return false;
            }
            await_acknowledgement(&replies, &stopped, &signal)

        });
        (Self { events, closed }, callback)
    }
    pub(super) fn try_next(&self) -> Option<Missing> {
        self.events.try_recv().ok()
    }
    pub(super) fn close(&self) {
        self.closed.store(true, Ordering::Release);
    }
}
fn await_acknowledgement(replies: &Receiver<bool>, stopped: &AtomicBool, signal: &Signal) -> bool {
    loop {
        match replies.recv_timeout(Duration::from_millis(50)) {
            Ok(accepted) => return accepted,
            Err(mpsc::RecvTimeoutError::Disconnected) => return false,
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if stopped.load(Ordering::Acquire) || signal.fired() {
                    // A rendered closed-pipe row may already have been acknowledged.
                    return replies.try_recv().unwrap_or(false);
                }
            }
        }
    }
}

impl Drop for Continuation {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests {
    //! Stays: the private bounded continuation acknowledgment must release on cancel and receiver drop.
    use super::*;

    #[test]
    fn acknowledgement_holds_the_worker_until_the_command_finishes_its_row() {
        let (continuation, callback) = Continuation::new(Signal::default());
        let (settled, result) = mpsc::sync_channel(1);
        let worker = std::thread::spawn(move || {
            settled.send(callback(2, "/selected")).unwrap();
        });
        let missing = continuation
            .events
            .recv_timeout(Duration::from_secs(5))
            .unwrap();
        assert_eq!(
            (missing.ordinal, missing.pointer.as_str()),
            (2, "/selected")
        );
        assert_eq!(result.try_recv(), Err(mpsc::TryRecvError::Empty));
        missing.acknowledge(true);
        assert!(result.recv_timeout(Duration::from_secs(5)).unwrap());
        worker.join().unwrap();
    }

    fn invoke(callback: Callback) -> (std::thread::JoinHandle<()>, Receiver<bool>) {
        let (sender, result) = mpsc::sync_channel(1);
        let worker = std::thread::spawn(move || {
            sender.send(callback(0, "/missing")).unwrap();
        });
        (worker, result)
    }

    #[test]
    fn failed_or_dropped_output_acknowledgement_releases_the_worker() {
        for explicit_refusal in [false, true] {
            let (continuation, callback) = Continuation::new(Signal::default());
            let (worker, result) = invoke(callback);
            let missing = continuation
                .events
                .recv_timeout(Duration::from_secs(5))
                .unwrap();
            if explicit_refusal {
                missing.acknowledge(false);
            } else {
                drop(missing);
            }
            assert!(!result.recv_timeout(Duration::from_secs(5)).unwrap());
            worker.join().unwrap();
        }
    }

    #[test]
    fn shutdown_and_dropped_receiver_release_an_outstanding_acknowledgement() {
        for drop_receiver in [false, true] {
            let (continuation, callback) = Continuation::new(Signal::default());
            let (worker, result) = invoke(callback);
            let missing = continuation
                .events
                .recv_timeout(Duration::from_secs(5))
                .unwrap();
            if drop_receiver {
                drop(continuation);
            } else {
                continuation.close();
            }
            // Keep the sender alive: closure must release the callback even
            // when command output has stalled while holding this event.
            assert!(!result.recv_timeout(Duration::from_secs(5)).unwrap());
            worker.join().unwrap();
            drop(missing);
        }
    }

    #[test]
    fn rendered_closed_pipe_acknowledgement_wins_before_shutdown() {
        let (continuation, callback) = Continuation::new(Signal::default());
        let (worker, result) = invoke(callback);
        let missing = continuation
            .events
            .recv_timeout(Duration::from_secs(5))
            .unwrap();
        missing.acknowledge(true);
        continuation.close();
        assert!(result.recv_timeout(Duration::from_secs(5)).unwrap());
        worker.join().unwrap();
    }
}
