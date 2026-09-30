//! The pure packer: the questions a cache lacks, gathered into requests, by
//! ADR 0111 section 4 step 3.
//!
//! A request holds questions of one state. It closes before an input that
//! would pass the request size, a profile limit or the question cap, before a
//! question of another state, and once it holds its inputs cap. One input
//! whose questions pass a limit together splits across requests with its
//! state repeated. A lone question that passes a profile limit, or with a
//! context any limit, is refused before anything is sent.

use std::sync::Arc;

use super::State;
use crate::core::adapters::built_in;
use crate::core::backend_profile::{BackendProfile, LimitKind, ProfileLimit};

/// What closes a request, and what refuses a lone question.
#[derive(Clone, Debug)]
pub(crate) struct PackLimits {
    /// The request size a request closes at.
    pub(crate) ceiling: usize,
    pub(crate) profile: Option<BackendProfile>,
    /// The inputs one request may answer: `--batch N`, or 4,096.
    pub(crate) inputs: usize,
    /// A further cap on questions per request, as relate's 400.
    pub(crate) questions: Option<usize>,
    /// Whether the state is a context, which refuses at any limit.
    pub(crate) context: bool,
}

/// One question to pack and the caller's handle for it.
pub(crate) struct Entry<T> {
    pub(crate) state: State,
    pub(crate) question: Arc<str>,
    /// The options a pick carries, which a profile may limit.
    pub(crate) options: usize,
    pub(crate) item: T,
}

/// One closed request: its state, its questions' handles in wire order, and
/// its body.
pub(crate) struct Packed<T> {
    pub(crate) state: State,
    pub(crate) items: Vec<T>,
    pub(crate) body: Vec<u8>,
}

/// Why a question cannot be sent. No variant holds text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PackError {
    Profile(ProfileLimit),
    Context {
        initial: bool,
        kind: LimitKind,
        limit: usize,
        actual: usize,
    },
}

/// Gathers entries into requests in the order they come.
pub(crate) struct Packer<T> {
    limits: PackLimits,
    model: String,
    open: Option<Open<T>>,
}

struct Open<T> {
    state: State,
    questions: Vec<Arc<str>>,
    items: Vec<T>,
    bytes: usize,
    inputs: usize,
}

impl<T> Packer<T> {
    /// Pack for one backend. `model` is the model's compact JSON string.
    pub(crate) const fn new(limits: PackLimits, model: String) -> Self {
        Self {
            limits,
            model,
            open: None,
        }
    }

    /// Refuse a context whose request with no question passes a limit.
    pub(crate) fn check_state(&self, state: &State) -> Result<(), PackError> {
        match self.over(state, self.base(state), 0, 0) {
            Some((kind, limit, actual)) if self.limits.context => Err(PackError::Context {
                initial: true,
                kind,
                limit,
                actual,
            }),
            _ => Ok(()),
        }
    }

    /// Add one input's missing questions, and push the requests that close.
    ///
    /// # Errors
    ///
    /// Returns [`PackError`] when one of the questions cannot go alone. None
    /// of the input's questions is packed then.
    pub(crate) fn add(
        &mut self,
        entries: Vec<Entry<T>>,
        closed: &mut Vec<Packed<T>>,
    ) -> Result<(), PackError> {
        for entry in &entries {
            self.lone(entry)?;
        }
        let Some(first) = entries.first() else {
            return Ok(());
        };
        if self.open.as_ref().is_some_and(|open| {
            open.state != first.state
                || open.inputs >= self.limits.inputs
                || !self.fits_all(open, &entries)
        }) {
            closed.extend(self.close());
        }
        for entry in entries {
            if self
                .open
                .as_ref()
                .is_some_and(|open| open.state != entry.state || !self.fits(open, &entry))
            {
                closed.extend(self.close());
            }
            let base = self.base(&entry.state);
            let open = self.open.get_or_insert_with(|| Open {
                state: entry.state.clone(),
                questions: Vec::new(),
                items: Vec::new(),
                bytes: base,
                inputs: 0,
            });
            open.bytes = grown(open.bytes, open.questions.len(), entry.question.len());
            open.questions.push(entry.question);
            open.items.push(entry.item);
        }
        if let Some(open) = self.open.as_mut() {
            open.inputs += 1;
            if open.inputs >= self.limits.inputs {
                closed.extend(self.close());
            }
        }
        Ok(())
    }

    /// Whether a request is open.
    pub(crate) const fn is_open(&self) -> bool {
        self.open.is_some()
    }

    /// Close the open request, for a pause, a full window or the end.
    pub(crate) fn close(&mut self) -> Option<Packed<T>> {
        let open = self.open.take()?;
        let body = built_in::join(
            open.state.json(),
            &self.model,
            open.questions.iter().map(|question| &**question),
        );
        Some(Packed {
            state: open.state,
            items: open.items,
            body,
        })
    }

    /// The body bytes of a request of this state with no question.
    fn base(&self, state: &State) -> usize {
        // `{"state":` S `,"model":` M `,"questions":{` `}}`
        state.json().len() + self.model.len() + 34
    }

    fn fits(&self, open: &Open<T>, entry: &Entry<T>) -> bool {
        let bytes = grown(open.bytes, open.questions.len(), entry.question.len());
        self.over(&open.state, bytes, open.questions.len() + 1, 0)
            .is_none()
    }

    fn fits_all(&self, open: &Open<T>, entries: &[Entry<T>]) -> bool {
        let mut bytes = open.bytes;
        let mut count = open.questions.len();
        for entry in entries {
            if entry.state != open.state {
                return false;
            }
            bytes = grown(bytes, count, entry.question.len());
            count += 1;
        }
        self.over(&open.state, bytes, count, 0).is_none()
    }

    /// Refuse a question that passes a limit in a request of its own.
    fn lone(&self, entry: &Entry<T>) -> Result<(), PackError> {
        let bytes = grown(self.base(&entry.state), 0, entry.question.len());
        let profile = self.limits.profile.as_ref();
        if self.limits.context
            && let Some((kind, limit, actual)) = self
                .over(&entry.state, bytes, 1, entry.options)
                .filter(|(kind, ..)| *kind != LimitKind::Options)
        {
            return Err(PackError::Context {
                initial: false,
                kind,
                limit,
                actual,
            });
        }
        let Some(profile) = profile else {
            return Ok(());
        };
        let checks = [
            (
                LimitKind::EvidenceBytes,
                profile.max_evidence_bytes,
                entry.state.evidence_bytes(),
            ),
            (LimitKind::Options, profile.max_options, entry.options),
            (LimitKind::RequestBytes, profile.max_request_bytes, bytes),
            (LimitKind::Questions, profile.max_questions, 1),
        ];
        checks
            .into_iter()
            .find_map(|(kind, most, actual)| {
                most.filter(|&most| actual > most)
                    .map(|limit| ProfileLimit {
                        name: profile.name().clone(),
                        kind,
                        limit,
                        actual,
                    })
            })
            .map_or(Ok(()), |limit| Err(PackError::Profile(limit)))
    }

    /// The first limit these counts pass, with the limit and the count.
    fn over(
        &self,
        state: &State,
        bytes: usize,
        questions: usize,
        options: usize,
    ) -> Option<(LimitKind, usize, usize)> {
        let profile = self.limits.profile.as_ref();
        let request = profile
            .and_then(|held| held.max_request_bytes)
            .map_or(self.limits.ceiling, |most| most.min(self.limits.ceiling));
        let questions_most = match (
            profile.and_then(|held| held.max_questions),
            self.limits.questions,
        ) {
            (Some(one), Some(other)) => Some(one.min(other)),
            (one, other) => one.or(other),
        };
        [
            (
                LimitKind::EvidenceBytes,
                profile.and_then(|held| held.max_evidence_bytes),
                state.evidence_bytes(),
            ),
            (LimitKind::RequestBytes, Some(request), bytes),
            (LimitKind::Questions, questions_most, questions),
            (
                LimitKind::Options,
                profile.and_then(|held| held.max_options),
                options,
            ),
        ]
        .into_iter()
        .find_map(|(kind, most, actual)| {
            most.filter(|&most| actual > most)
                .map(|most| (kind, most, actual))
        })
    }
}

/// The body bytes after adding a question of `length` bytes to a request of
/// `count` questions: its `"qN":` name, its bytes, and a comma after the first.
fn grown(bytes: usize, count: usize, length: usize) -> usize {
    let name = 4 + (count + 1).to_string().len();
    bytes + name + length + usize::from(count > 0)
}

#[cfg(test)]
#[path = "packer_tests.rs"]
mod tests;
