//! One framed item per outstanding native input demand.
use crate::public::cli_reader::Wake;
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
    pub(crate) fn next(&self) -> Option<Result<T, Placed>> {
        let mut state = self.0.state.lock().ok()?;
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
            self.0.changed.notify_one();
        }
    }
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
