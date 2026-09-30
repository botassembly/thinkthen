//! Owned call evidence carried from the worker to R's main thread as JSON.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError};

use serde::Serialize;
use serde_json::value::RawValue;
use thinkthen::{Batch, Call, Error, Facts, RecordObservation, Tally};

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

    /// Run one eager call and count its facts, a failed call's too.
    pub(crate) fn run<T>(
        &self,
        call: impl FnOnce() -> Result<Call<T>, Error>,
    ) -> Result<T, String> {
        match self.tally.run(call) {
            Ok(done) => {
                self.counted.store(true, Ordering::SeqCst);
                Ok(done.into_value())
            }
            Err(error) => {
                if error.facts().is_some() {
                    self.counted.store(true, Ordering::SeqCst);
                }
                Err(carry(&error))
            }
        }
    }

    /// An empty column still reports zero-work facts.
    pub(crate) fn no_work(&self) {
        self.counted.store(true, Ordering::SeqCst);
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

/// Exhaust a lazy batch through its one terminal error, then count its facts.
pub(crate) fn collect<T>(batch: &mut Batch<'_, T>, account: &Account) -> Result<Vec<T>, String> {
    let started = account.tally.start();
    let mut values = Vec::new();
    let mut failure = None;
    for row in batch.by_ref() {
        match row {
            Ok(value) => values.push(value),
            Err(error) => {
                failure = Some(carry(&error));
                break;
            }
        }
    }
    if let Some(facts) = batch.facts() {
        started.finish(facts).map_err(|error| carry(&error))?;
        account.counted.store(true, Ordering::SeqCst);
    }
    failure.map_or(Ok(values), Err)
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
    pub(crate) snapshot: Snapshot,
}
