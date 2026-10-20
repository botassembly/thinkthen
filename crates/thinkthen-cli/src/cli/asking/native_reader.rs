//! One framed item per outstanding native input demand.
use crate::schedule::Placed;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct State<T> {
    demand: bool,
    busy: bool,
    stopped: bool,
    item: Option<Option<Result<T, Placed>>>,
}
struct Shared<T> {
    state: Mutex<State<T>>,
    changed: Condvar,
}
/// One nonblocking demand result; no second item is requested until this one is taken.
pub(crate) enum Read<T> {
    Pending,
    Item(Result<T, Placed>),
    End,
}
pub(crate) struct Reader<T>(Arc<Shared<T>>);
impl<T: Send + 'static> Reader<T> {
    pub(crate) fn new(records: impl Iterator<Item = Result<T, Placed>> + Send + 'static) -> Self {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                demand: false,
                busy: false,
                stopped: false,
                item: None,
            }),
            changed: Condvar::new(),
        });
        let worker = shared.clone();
        thread::spawn(move || feed(records, worker));
        Self(shared)
    }
    /// Wait only for an already demanded item. Arrival, EOF and failure wake
    /// the pump immediately; the existing poll bound still admits cancellation.
    pub(crate) fn wait(&self, timeout: std::time::Duration) {
        if let Ok(state) = self.0.state.lock() {
            let _ready = self.0.changed.wait_timeout_while(state, timeout, |state| {
                state.item.is_none() && !state.stopped
            });
        }
    }
    /// Demand one item without blocking the command's event pump.
    pub(crate) fn try_next(&self) -> Read<T> {
        let Ok(mut state) = self.0.state.lock() else {
            return Read::Item(Err(lock_error()));
        };
        if let Some(item) = state.item.take() {
            state.busy = false;
            return item.map_or(Read::End, Read::Item);
        }
        if state.stopped {
            return Read::End;
        }
        if !state.busy {
            state.demand = true;
            state.busy = true;
            self.0.changed.notify_one();
        }
        Read::Pending
    }
}
impl<T> Drop for Reader<T> {
    fn drop(&mut self) {
        if let Ok(mut state) = self.0.state.lock() {
            state.stopped = true;
            state.item = None;
        }
        self.0.changed.notify_one();
    }
}

fn lock_error() -> Placed {
    crate::failure::Failure::Defect("the input reader lock failed").into()
}

fn feed<T>(mut records: impl Iterator<Item = Result<T, Placed>>, worker: Arc<Shared<T>>) {
    loop {
        let Ok(state) = worker.state.lock() else {
            return;
        };
        let Ok(mut state) = worker
            .changed
            .wait_while(state, |s| !s.demand && !s.stopped)
        else {
            return;
        };
        if state.stopped {
            return;
        }
        state.demand = false;
        drop(state);
        let item = records.next();
        let last = item.as_ref().is_none_or(Result::is_err);
        let Ok(mut state) = worker.state.lock() else {
            return;
        };
        if state.stopped {
            return;
        }
        state.item = Some(item);
        state.stopped = last;
        worker.changed.notify_all();
        if last {
            return;
        }
    }
}

/// Wait for the next native boundary while retaining command signal polling.
pub(crate) fn session_ended(
    session: &thinkthen::RequestSession,
    cancelled: bool,
    input_done: bool,
) -> bool {
    let interest = if cancelled {
        thinkthen::RequestSessionWaitInterest::Settlement
    } else if input_done {
        thinkthen::RequestSessionWaitInterest::Result
    } else {
        thinkthen::RequestSessionWaitInterest::Input
    };
    session.wait(interest, std::time::Duration::from_millis(50))
        == thinkthen::RequestSessionReady::End
}

#[cfg(test)]
#[allow(
    clippy::expect_used,
    clippy::panic,
    reason = "poison a real reader lock and require its observable failure"
)]
mod tests {
    //! Stays: fault injection proves poisoned reader state becomes failure rather than fabricated EOF.
    #[test]
    fn a_poisoned_reader_reports_failure_instead_of_end_of_input() {
        let reader = super::Reader::<()>::new(std::iter::empty());
        let shared = reader.0.clone();
        assert!(
            std::thread::spawn(move || {
                let _state = shared.state.lock().expect("input state");
                panic!("poison input state");
            })
            .join()
            .is_err()
        );
        let super::Read::Item(Err(error)) = reader.try_next() else {
            panic!("the poisoned reader must report failure");
        };
        assert!(matches!(
            error.cause,
            crate::failure::Failure::Defect("the input reader lock failed")
        ));
    }
}
