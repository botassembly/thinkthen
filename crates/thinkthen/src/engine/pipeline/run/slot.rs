//! One unemitted input of the coordinator's window.

use super::super::{Answered, Asker, Failed};

/// One unemitted input.
pub(super) struct Slot<A: Asker> {
    pub(super) input: Option<A::Input>,
    pub(super) answers: Vec<Option<Answered>>,
    pub(super) missing: usize,
    pub(super) failed: Option<Failed<A::Error>>,
    /// The ask whose failure `failed` holds, so the earliest ask's failure
    /// wins whatever order the replies arrive in.
    pub(super) failed_at: usize,
}

impl<A: Asker> Slot<A> {
    pub(super) fn failed(failure: Failed<A::Error>) -> Self {
        Self {
            input: None,
            answers: Vec::new(),
            missing: 0,
            failed: Some(failure),
            failed_at: 0,
        }
    }

    /// Done, or failed with every earlier ask answered, so no earlier
    /// failure can still arrive.
    pub(super) fn ready(&self) -> bool {
        self.missing == 0
            || (self.failed.is_some()
                && self
                    .answers
                    .iter()
                    .take(self.failed_at)
                    .all(Option::is_some))
    }

    pub(super) fn answer(&mut self, at: usize, answered: Answered) {
        if let Some(place) = self.answers.get_mut(at) {
            *place = Some(answered);
            self.missing -= 1;
        }
    }

    pub(super) fn fail(&mut self, at: usize, failure: Failed<A::Error>) {
        if self.failed.is_none() || at < self.failed_at {
            self.failed = Some(failure);
            self.failed_at = at;
        }
    }
}
