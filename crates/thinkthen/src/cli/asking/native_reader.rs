//! One framed item per outstanding native input demand.
use crate::public::readiness::Wake;
use crate::schedule::Placed;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct State<T> {
    demand: bool,
    busy: bool,
    stopped: bool,
    item: Option<Option<Result<T, Placed>>>,
    wake: Option<Wake>,
}
struct Shared<T> {
    state: Mutex<State<T>>,
    changed: Condvar,
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
                wake: None,
            }),
            changed: Condvar::new(),
        });
        let worker = shared.clone();
        thread::spawn(move || feed(records, worker));
        Self(shared)
    }
    pub(crate) fn wake(&self, wake: Wake) {
        if let Ok(mut state) = self.0.state.lock() {
            state.wake = Some(wake);
        }
    }
    pub(crate) fn ready(&self) -> bool {
        let Ok(mut state) = self.0.state.lock() else {
            return true;
        };
        if state.item.is_some() {
            return true;
        }
        if !state.stopped && !state.busy {
            state.demand = true;
            state.busy = true;
            self.0.changed.notify_one();
        }
        false
    }
    pub(crate) fn next(&self, cancel: &crate::engine::Cancel<'_>) -> Option<Result<T, Placed>> {
        let Ok(mut state) = self.0.state.lock() else {
            return Some(Err(lock_error()));
        };
        while state.item.is_none() && !state.stopped {
            if let Err(error) = cancel.remaining_without_check() {
                return Some(Err(error.into()));
            }
            if !state.busy {
                state.demand = true;
                state.busy = true;
                self.0.changed.notify_one();
            }
            let Ok((next, _)) = self
                .0
                .changed
                .wait_timeout(state, std::time::Duration::from_millis(50))
            else {
                return Some(Err(lock_error()));
            };
            state = next;
        }
        let item = state.item.take()?;
        state.busy = false;
        item
    }
}
impl<T> Drop for Reader<T> {
    fn drop(&mut self) {
        if let Ok(mut state) = self.0.state.lock() {
            state.stopped = true;
            state.item = None;
            state.wake = None;
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
        let wake = state.wake.clone();
        drop(state);
        if let Some(wake) = wake {
            wake.notify();
        }
        if last {
            return;
        }
    }
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
        assert!(
            reader.ready(),
            "readiness must let the caller observe the failure"
        );
        let error = reader
            .next(&crate::engine::Cancel::default())
            .expect("an item")
            .expect_err("a failure");
        assert!(matches!(
            error.cause,
            crate::failure::Failure::Defect("the input reader lock failed")
        ));
    }
}
