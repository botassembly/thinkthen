//! Owned calls keep native settlement alive after R handles an interrupt.
use crate::carry;
use std::sync::{Mutex, MutexGuard, PoisonError};
use thinkthen::{Call, CancelToken, RequestValue};
pub(crate) type Crossed<T> = Result<T, crate::NativeError>;
type Outcome = Result<Call<RequestValue>, thinkthen::Error>;
pub(crate) struct Completion(Mutex<State>);
enum State {
    Unused,
    Claimed,
    Running { cancel: CancelToken, details: Vec<serde_json::Value> },
    Terminal { kind: String, facts: Option<serde_json::Value>, details: Vec<serde_json::Value> },
}
impl std::fmt::Debug for Completion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("Completion { content withheld }") }
}
impl Completion {
    pub(crate) fn new() -> Self { Self(Mutex::new(State::Unused)) }
    fn state(&self) -> MutexGuard<'_, State> { self.0.lock().unwrap_or_else(PoisonError::into_inner) }
    pub(crate) fn claim(&self) -> bool {
        let mut state = self.state();
        if !matches!(*state, State::Unused) { return false; }
        *state = State::Claimed;
        true
    }
    pub(crate) fn settle_early(&self, kind: String) {
        let mut state = self.state();
        if matches!(*state, State::Claimed) { *state = State::Terminal { kind, facts: None, details: Vec::new() }; }
    }
    pub(crate) fn start(&self, cancel: CancelToken) { *self.state() = State::Running { cancel, details: Vec::new() }; }
    pub(crate) fn cancel(&self) {
        if let State::Running { cancel, .. } = &*self.state() { cancel.cancel(); }
    }
    pub(crate) fn observe(&self, event: thinkthen::RecordObservation<'_>, positions: Option<&[usize]>) {
        let event = if let thinkthen::RecordObservation::Question { index, .. } = &event {
            let position = positions.and_then(|p| p.get(*index)).copied();
            match position { Some(index) => event.remap_index(index), None => event }
        } else { return; };
        if let Ok(value) = serde_json::to_value(event)
            && let State::Running { details, .. } = &mut *self.state() { details.push(value); }
    }
    pub(crate) fn finish(&self, outcome: &Outcome) {
        let (kind, facts) = match outcome {
            Ok(call) => ("success", call.facts().complete().and_then(|facts| serde_json::to_value(facts).ok())),
            Err(error) => (error.kind().name(), error.facts().and_then(|facts| facts.complete()).and_then(|facts| serde_json::to_value(facts).ok())),
        };
        let mut state = self.state();
        let details = match &mut *state { State::Running { details, .. } => std::mem::take(details), _ => Vec::new() };
        *state = State::Terminal { kind: kind.into(), facts, details };
    }
    pub(crate) fn read(&self) -> Crossed<extendr_api::Robj> {
        use extendr_api::prelude::*;
        let state = self.state();
        let name = match &*state { State::Unused => "unused", State::Claimed => "claimed", State::Running { .. } => "running", State::Terminal { .. } => "terminal" };
        let mut fields = vec![("state", Robj::from(name))];
        if let State::Terminal { kind, facts, details } = &*state {
            fields.push(("kind", kind.as_str().into()));
            fields.push(("facts", facts.as_ref().map(crate::native_results::facts).transpose()?.unwrap_or_else(|| ().into())));
            fields.push(("details", crate::native_results::plain(&serde_json::json!(details))?));
        }
        Ok(List::from_pairs(fields).into())
    }
}
pub(crate) fn result(call: &Call<RequestValue>, function: thinkthen::RequestFunction, column: Option<&thinkthen::RequestColumn>, scalar: bool) -> Crossed<extendr_api::Robj> {
    let tree = call.value_tree().map_err(|e| carry(&e))?;
    let value = crate::native_results::call(&tree, function)?;
    let view = column.map(|column| column.view(call.value()).map_err(|e| carry(&e))).transpose()?.flatten();
    crate::request::view(value, view, column, scalar)
}
