//! One framed item per outstanding native input demand.
use super::judged::{Held, Records};
use crate::public::cli_reader::Wake;
use crate::schedule::Placed;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(Default)]
struct State {
    demand: bool,
    busy: bool,
    stopped: bool,
    item: Option<Option<Result<Held, Placed>>>,
    wake: Option<Wake>,
}
struct Shared {
    state: Mutex<State>,
    changed: Condvar,
}
pub(super) struct Reader(Arc<Shared>);
impl Reader {
    pub(super) fn new(records: Records) -> Self {
        let shared = Arc::new(Shared {
            state: Mutex::new(State::default()),
            changed: Condvar::new(),
        });
        let worker = shared.clone();
        thread::spawn(move || feed(records, worker));
        Self(shared)
    }
    pub(super) fn wake(&self, wake: Wake) {
        if let Ok(mut state) = self.0.state.lock() {
            state.wake = Some(wake);
        }
    }
    pub(super) fn ready(&self) -> bool {
        let Ok(mut state) = self.0.state.lock() else {
            return false;
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
    pub(super) fn next(&self) -> Option<Result<Held, Placed>> {
        let mut state = self.0.state.lock().ok()?;
        let item = state.item.take()?;
        state.busy = false;
        item
    }
}
impl Drop for Reader {
    fn drop(&mut self) {
        if let Ok(mut state) = self.0.state.lock() {
            state.stopped = true;
            state.item = None;
            state.wake = None;
            self.0.changed.notify_one();
        }
    }
}

fn feed(mut records: Records, worker: Arc<Shared>) {
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
