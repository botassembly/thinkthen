//! The packer: it groups records into requests and reads each reply into rows.

use std::collections::HashMap;
use std::num::NonZeroUsize;

use serde_json::value::RawValue;
use sha2::{Digest, Sha256};

use crate::core::adapter::{Adapter, Reply};
use crate::core::ask::{Ask, AskAnswer};
use crate::core::error::{Error, ErrorKind};
use crate::core::record::{AdmittedRecord, Id, Record};
use crate::core::row::{Outcome, Row};
use crate::core::{Function, FunctionAnswers};

/// One ask's identity: SHA-256 over the length-prefixed adapter, address,
/// model, state and wire question. Its neighbors never change it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AskKey([u8; 32]);

/// Writes `field` after its length, so no two field lists give the same bytes.
fn prefixed(field: &[u8], mut write: impl FnMut(&[u8])) {
    write(&(field.len() as u64).to_be_bytes());
    write(field);
}

/// Groups records into bounded batches whose rounds can span requests.
#[derive(Debug)]
pub(crate) struct Packer<'a, F: Function> {
    question: &'a F,
    adapter: Adapter,
    batch: NonZeroUsize,
    limits: crate::core::Limits,
    empty_len: usize,
    seed: Vec<u8>,
    position: usize,
    open: Chunk<'a, F>,
}

impl<'a, F: Function> Packer<'a, F> {
    /// A packer for `adapter` at `address` asking `model`, closing each request
    /// at `batch` records or at the selected backend model limits.
    #[must_use]
    pub(crate) fn new(
        question: &'a F,
        adapter: Adapter,
        address: &str,
        model: &str,
        batch: NonZeroUsize,
        limits: crate::core::Limits,
        identity: String,
    ) -> Self {
        let mut seed = Vec::new();
        for field in [adapter.path(), address, model] {
            prefixed(field.as_bytes(), |part| seed.extend_from_slice(part));
        }
        let empty_len = adapter
            .encode(crate::core::ask::DEFAULT_STATE, model, &[])
            .len();
        Self {
            question,
            adapter,
            batch,
            limits,
            empty_len,
            seed,
            position: 0,
            open: Chunk::new(
                question,
                adapter,
                limits,
                model.to_owned(),
                empty_len,
                identity,
            ),
        }
    }

    /// Add `record`, asked by `ask`. Returns the request it closes, if any.
    ///
    /// A record whose question alone passes the byte limit fails with
    /// `backend.limit` and sends nothing.
    pub(crate) fn push(&mut self, record: Record) -> Option<Chunk<'a, F>> {
        self.position += 1;
        let id = record.id.clone().unwrap_or(Id::position(self.position));
        let record = AdmittedRecord::new(record);
        let question = self.question;
        let limits = self.limits;
        self.push_round(id, record, Vec::new(), |record| {
            question.try_asks_record(record, limits)
        })
    }

    fn push_round(
        &mut self,
        id: Id,
        record: Result<AdmittedRecord, Error>,
        history: Vec<Result<AskAnswer, Error>>,
        asks: impl FnOnce(&AdmittedRecord) -> Result<Vec<Ask>, Error>,
    ) -> Option<Chunk<'a, F>> {
        let entry = record.as_ref().map_err(Clone::clone).and_then(|record| {
            let prepared = asks(record)?
                .into_iter()
                .enumerate()
                .map(|(position, ask)| self.prepare(ask, record, &history, position))
                .collect::<Vec<_>>();
            let mut state_lengths = HashMap::new();
            // A byte-oversized ask prevents the record's whole round from sending.
            for (ask, question, _, _) in prepared.iter().flatten() {
                let empty_len = *state_lengths.entry(ask.state()).or_insert_with(|| {
                    self.adapter
                        .encode(ask.state(), &self.open.model, &[])
                        .len()
                });
                self.validate_size(empty_len, question)?;
            }
            Ok(prepared)
        });
        let mut extra = 0;
        let mut added = std::collections::HashSet::new();
        if let Ok(questions) = &entry {
            for (_, question, key, _) in questions.iter().flatten() {
                if !self.open.slots.contains_key(key) && added.insert(*key) {
                    extra += self
                        .adapter
                        .entry_len(self.open.questions.len() + added.len() - 1, question);
                }
            }
        }
        let state_changed = entry
            .as_ref()
            .ok()
            .and_then(|asks| {
                asks.iter()
                    .flatten()
                    .next()
                    .map(|(ask, _, _, _)| ask.state())
            })
            .zip(
                self.open
                    .questions
                    .last()
                    .map(|question| question.state.as_ref()),
            )
            .is_some_and(|(next, held)| next != held);
        let full = state_changed
            || self.open.entries.len() == self.batch.get()
            || self.open.len + extra > self.limits.request_bytes.get()
            || self
                .limits
                .questions
                .is_some_and(|limit| self.open.questions.len() + added.len() > limit.get());
        let closed = (full && !self.open.entries.is_empty()).then(|| self.take());
        self.open.add(id, record, entry);
        if let Some(entry) = self.open.entries.last_mut() {
            entry.3 = history;
        }
        closed
    }

    /// The last open request, if it holds any record.
    #[must_use]
    pub(crate) fn finish(mut self) -> Option<Chunk<'a, F>> {
        self.flush()
    }
    pub(crate) fn flush(&mut self) -> Option<Chunk<'a, F>> {
        (!self.open.entries.is_empty()).then(|| self.take())
    }
    pub(crate) fn flush_full(&mut self) -> Option<Chunk<'a, F>> {
        (self.open.entries.len() == self.batch.get()).then(|| self.take())
    }

    fn prepare(
        &self,
        ask: Ask,
        record: &AdmittedRecord,
        history: &[Result<AskAnswer, Error>],
        position: usize,
    ) -> Prepared {
        let question = self.adapter.question(&ask, self.limits)?;
        let mut hasher = Sha256::new();
        hasher.update(&self.seed);
        prefixed(ask.state().as_bytes(), |part| hasher.update(part));
        prefixed(question.get().as_bytes(), |part| hasher.update(part));
        let stub_key = F::stub_key(
            FunctionAnswers::new(self.question, record, history),
            &self.open.identity,
            self.limits,
            position,
        );
        Ok((ask, question, AskKey(hasher.finalize().into()), stub_key))
    }

    fn validate_size(&self, empty_len: usize, question: &RawValue) -> Result<(), Error> {
        if empty_len + self.adapter.entry_len(0, question) > self.limits.request_bytes.get() {
            return Err(Error::new(
                ErrorKind::BackendLimit,
                format!(
                    "the record's question is larger than the backend request limit of {} bytes",
                    self.limits.request_bytes
                ),
            ));
        }
        Ok(())
    }

    fn take(&mut self) -> Chunk<'a, F> {
        let fresh = Chunk::new(
            self.question,
            self.adapter,
            self.limits,
            self.open.model.clone(),
            self.empty_len,
            self.open.identity.clone(),
        );
        std::mem::replace(&mut self.open, fresh)
    }
}

/// What a record asks: the slot of its question and its ask, or why it asks nothing.
type Asked = Result<Vec<Result<(AskKey, usize, Ask, [u8; 32]), Error>>, Error>;
type Prepared = Result<(Ask, Box<RawValue>, AskKey, [u8; 32]), Error>;
type Entry = (
    Id,
    Result<AdmittedRecord, Error>,
    Asked,
    Vec<Result<AskAnswer, Error>>,
);

#[derive(Debug)]
struct EncodedAsk {
    state: std::sync::Arc<str>,
    body: Box<RawValue>,
}

/// An ordered record group and the distinct questions in its current round.
#[derive(Debug)]
pub(crate) struct Chunk<'a, F: Function> {
    question: &'a F,
    identity: String,
    adapter: Adapter,
    limits: crate::core::Limits,
    model: String,
    entries: Vec<Entry>,
    questions: Vec<EncodedAsk>,
    slots: HashMap<AskKey, usize>,
    len: usize,
}

impl<'a, F: Function> Chunk<'a, F> {
    fn new(
        question: &'a F,
        adapter: Adapter,
        limits: crate::core::Limits,
        model: String,
        len: usize,
        identity: String,
    ) -> Self {
        Self {
            question,
            identity,
            adapter,
            limits,
            model,
            entries: Vec::new(),
            questions: Vec::new(),
            slots: HashMap::new(),
            len,
        }
    }

    /// Add one record. A question this request already asks is asked once.
    fn add(
        &mut self,
        id: Id,
        evidence: Result<AdmittedRecord, Error>,
        entry: Result<Vec<Prepared>, Error>,
    ) {
        let entry = entry.map(|asks| {
            asks.into_iter()
                .map(|ask| {
                    ask.map(|(ask, question, key, stub_key)| {
                        let slot = *self.slots.entry(key).or_insert_with(|| {
                            self.len += self.adapter.entry_len(self.questions.len(), &question);
                            self.questions.push(EncodedAsk {
                                state: ask.shared_state(),
                                body: question,
                            });
                            self.questions.len() - 1
                        });
                        (key, slot, ask, stub_key)
                    })
                })
                .collect()
        });
        self.entries.push((id, evidence, entry, Vec::new()));
    }

    /// Bounded wire requests for this round, retaining their original answer slots.
    #[must_use]
    pub(crate) fn requests(&self) -> Vec<Request<'_>> {
        let mut requests = Vec::new();
        let mut start = 0;
        let mut state = self
            .questions
            .first()
            .map_or(crate::core::ask::DEFAULT_STATE, |question| {
                question.state.as_ref()
            });
        let mut len = self.adapter.encode(state, &self.model, &[]).len();
        for (slot, question) in self.questions.iter().enumerate() {
            let next_state = question.state.as_ref();
            let extra = self.adapter.entry_len(slot - start, &question.body);
            if slot > start
                && (state != next_state
                    || len + extra > self.limits.request_bytes.get()
                    || self
                        .limits
                        .questions
                        .is_some_and(|max| slot - start == max.get()))
            {
                requests.push(self.request(start, slot));
                start = slot;
                state = next_state;
                len = self.adapter.encode(state, &self.model, &[]).len();
            }
            len += self.adapter.entry_len(slot - start, &question.body);
        }
        if start < self.questions.len() {
            requests.push(self.request(start, self.questions.len()));
        }
        requests
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "ranges come from this question vector"
    )]
    fn request(&self, start: usize, end: usize) -> Request<'_> {
        Request {
            adapter: self.adapter,
            model: &self.model,
            state: &self.questions[start].state,
            start,
            questions: &self.questions[start..end],
        }
    }

    /// Finish a round whose next asks are empty. Only this module can finish rows.
    fn read(self) -> Vec<Row> {
        self.entries
            .into_iter()
            .map(|(id, record, entry, history)| {
                let outcome = entry
                    .and(record)
                    .map(|record| {
                        F::read_record(
                            FunctionAnswers::new(self.question, &record, &history),
                            self.limits,
                        )
                    })
                    .unwrap_or_else(|error| Outcome::Failed { error });
                Row::new(id, outcome)
            })
            .collect()
    }

    /// Match every request reply, then return rows or prepare the next round.
    pub(crate) fn advance(
        self,
        replies: Vec<RequestAnswers>,
        address: &str,
        batch: NonZeroUsize,
    ) -> Vec<NextRound<'a, F>> {
        let mut packer = Packer::new(
            self.question,
            self.adapter,
            address,
            &self.model,
            batch,
            self.limits,
            self.identity.clone(),
        );
        let mut chunks = Vec::new();
        for (id, record, entry, mut history) in self.entries {
            let asks = entry.and_then(|asks| {
                history.extend(asks.into_iter().map(|ask| {
                    ask.and_then(|(_, slot, ask, stub_key)| {
                        let reply = replies
                            .iter()
                            .find(|reply| reply.slots.contains(&slot))
                            .ok_or_else(|| {
                                Error::new(
                                    ErrorKind::Reply,
                                    "the round is missing a request answer",
                                )
                            })?;
                        match &reply.reply {
                            Ok(answer) => answer.read(slot - reply.slots.start, &ask, &stub_key),
                            Err(error) => Err(error.clone()),
                        }
                    })
                }));
                let record = record.as_ref().map_err(Clone::clone)?;
                F::followup_record(
                    FunctionAnswers::new(self.question, record, &history),
                    self.limits,
                )
            });
            if let Some(chunk) = packer.push_round(id, record, history, |_| asks) {
                chunks.push(chunk);
            }
        }
        if let Some(chunk) = packer.finish() {
            chunks.push(chunk);
        }
        chunks
            .into_iter()
            .map(|chunk| {
                if chunk.questions.is_empty() {
                    NextRound::Ready(chunk.read())
                } else {
                    NextRound::Pending(chunk)
                }
            })
            .collect()
    }
}

/// One bounded request whose answer slots belong to its original round.
#[derive(Debug)]
pub(crate) struct Request<'a> {
    adapter: Adapter,
    model: &'a str,
    state: &'a str,
    start: usize,
    questions: &'a [EncodedAsk],
}

impl<'a> Request<'a> {
    /// Encode this request for its adapter.
    #[must_use]
    pub(crate) fn body(&self) -> Vec<u8> {
        let questions: Vec<_> = self
            .questions
            .iter()
            .map(|question| question.body.as_ref())
            .collect();
        self.adapter.encode(self.state, self.model, &questions)
    }

    /// Split a refused request once when it holds more than one question.
    #[must_use]
    pub(crate) fn split(&self) -> Option<(Self, Self)> {
        if self.questions.len() < 2 {
            return None;
        }
        let middle = self.questions.len() / 2;
        let (left, right) = self.questions.split_at(middle);
        Some((
            Self {
                questions: left,
                ..*self
            },
            Self {
                questions: right,
                start: self.start + middle,
                ..*self
            },
        ))
    }

    /// Answer from the built-in deterministic provider without I/O.
    #[must_use]
    pub(crate) fn stub_answer(&self) -> RequestAnswers {
        RequestAnswers {
            slots: self.start..self.start + self.questions.len(),
            reply: Ok(Reply::Stub),
        }
    }

    /// Decode a reply while preserving the original round's answer slots.
    pub(crate) fn answered(&self, reply: Result<Vec<u8>, Error>) -> RequestAnswers {
        RequestAnswers {
            slots: self.start..self.start + self.questions.len(),
            reply: reply.and_then(|body| self.adapter.decode(&body)),
        }
    }
}

/// A decoded request, bound to the slots that produced it.
#[derive(Debug)]
pub(crate) struct RequestAnswers {
    slots: std::ops::Range<usize>,
    reply: Result<Reply, Error>,
}

/// The original question's next request, or its finished rows.
#[derive(Debug)]
pub(crate) enum NextRound<'a, F: Function> {
    /// This record group has finished every dependent round.
    Ready(Vec<Row>),
    /// This record group has another request to send.
    Pending(Chunk<'a, F>),
}
