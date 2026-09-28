//! One caller-owned completion state, with no R object on a worker.

use std::sync::{Mutex, PoisonError};

use super::account::Snapshot;

#[derive(Clone, Debug)]
pub(crate) enum State {
    Unused,
    Claimed,
    Running,
    Terminal { kind: String, snapshot: Snapshot },
}

#[derive(Debug)]
pub(crate) struct Receipt(Mutex<State>);

impl Receipt {
    pub(crate) fn new() -> Self {
        Self(Mutex::new(State::Unused))
    }

    pub(crate) fn read(&self) -> State {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    pub(crate) fn claim(&self) -> bool {
        let mut state = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if !matches!(*state, State::Unused) {
            return false;
        }
        *state = State::Claimed;
        true
    }

    pub(crate) fn running(&self) {
        let mut state = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if matches!(*state, State::Claimed) {
            *state = State::Running;
        }
    }

    pub(crate) fn settle_early(&self, kind: String) {
        let mut state = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if matches!(*state, State::Claimed) {
            *state = State::Terminal {
                kind,
                snapshot: Snapshot {
                    facts: None,
                    details: Vec::new(),
                },
            };
        }
    }

    pub(crate) fn settle(&self, kind: String, snapshot: Snapshot) {
        let mut state = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if matches!(*state, State::Claimed | State::Running) {
            *state = State::Terminal { kind, snapshot };
        }
    }
}
