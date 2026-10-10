//! Owned call evidence carried from the worker to R's main thread as JSON.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError};

use serde::Serialize;
use serde_json::value::RawValue;
use thinkthen::{Facts, RecordObservation, Tally};

use crate::carry;

/// One call's facts, summed by the crate's `Tally`, and its question events.
#[derive(Debug, Default)]
pub(crate) struct Account {
    positions: Option<Vec<usize>>,
    tally: Tally,
    counted: AtomicBool,
    unwritten: AtomicBool,
    details: Mutex<Vec<Box<RawValue>>>,
}

impl Account {
    pub(crate) fn new(positions: Option<Vec<usize>>) -> Self {
        Self {
            positions,
            ..Self::default()
        }
    }

    /// Keep one question event, with its index moved to the caller's
    /// original position when the call asked about only some rows.
    pub(crate) fn observe(&self, event: RecordObservation<'_>, original: Option<usize>) {
        let RecordObservation::Question {
            index,
            member,
            stage,
            position,
            detail,
        } = event
        else {
            return;
        };
        let index = original
            .or_else(|| self.positions.as_ref()?.get(index).copied())
            .unwrap_or(index);
        let event = RecordObservation::Question {
            index,
            member,
            stage,
            position,
            detail,
        };
        match serde_json::value::to_raw_value(&event) {
            Ok(json) => self
                .details
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push(json),
            Err(_) => self.unwritten.store(true, Ordering::SeqCst),
        }
    }

    /// Fail the call when a question event could not be written.
    pub(crate) fn written(&self) -> Result<(), String> {
        if self.unwritten.load(Ordering::SeqCst) {
            return Err(crate::defect("a question event could not be written"));
        }
        Ok(())
    }

    pub(crate) fn start(&self) -> thinkthen::TallyStart<'_> {
        self.tally.start()
    }

    pub(crate) fn include(
        &self,
        started: thinkthen::TallyStart<'_>,
        facts: &Facts,
    ) -> Result<(), String> {
        started.finish(facts).map_err(|e| carry(&e))?;
        self.counted.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub(crate) fn finish(self) -> Snapshot {
        Snapshot {
            facts: self
                .counted
                .load(Ordering::SeqCst)
                .then(|| self.tally.facts()),
            details: self
                .details
                .into_inner()
                .unwrap_or_else(PoisonError::into_inner),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct Snapshot {
    pub(crate) facts: Option<Facts>,
    pub(crate) details: Vec<Box<RawValue>>,
}

impl Snapshot {
    pub(crate) const fn empty() -> Self {
        Self {
            facts: None,
            details: Vec::new(),
        }
    }
}

pub(crate) struct Completed<T> {
    pub(crate) result: Result<T, String>,
}
