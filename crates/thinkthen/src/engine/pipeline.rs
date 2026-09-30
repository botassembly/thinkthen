//! One path from inputs to rows, by ADR 0111 section 4.
//!
//! The coordinator runs on the calling thread. It asks the host's reader for
//! one input at a time, looks each question up in the call's in-flight map
//! and then the store, packs the misses, hands closed requests to `jobs` send
//! workers, stores each reply's good answers, and emits rows in input order.

use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};

use crate::core::Usage;
use crate::core::adapters::built_in::DecodeError;
use crate::core::pack::{self, Ask, PackError, PackLimits, Packer, QuestionKey};
use crate::engine::error::Error;
use crate::engine::facade::Engine;
pub(crate) use crate::engine::schedule::Input;
use crate::engine::store::{Mode, Store};
use crate::engine::{Cancel, workers};
use crate::public::AttemptObservation;

mod run;
mod send;

/// The inputs one request answers when no `--batch N` is set.
pub(crate) const MOST_INPUTS: usize = 4096;

/// What one function asks of each input and how it reads the answers.
pub(crate) trait Asker: Sync {
    type Input: Send;
    type Row;
    type Error: Send;

    /// The number messages give this input, such as its line.
    fn label(&self, input: &Self::Input) -> usize;

    /// The wire questions one input needs, in order. Pure.
    fn asks(&self, input: &Self::Input) -> Result<Vec<Ask>, Self::Error>;

    /// The row from the answers, in the same order. Pure.
    fn row(&self, input: Self::Input, answers: Vec<Answered>) -> Result<Self::Row, Self::Error>;

    /// Whether the host no longer takes rows, such as a closed output pipe.
    /// The call then stops quietly, even while the reader waits for input.
    fn gone(&self) -> bool {
        false
    }
}

/// One wire question's answer, live or stored.
#[derive(Clone, Debug)]
pub(crate) struct Answered {
    pub(crate) key: QuestionKey,
    /// The wire answer's JSON as received, or the error that failed it.
    pub(crate) answer: Result<Arc<str>, DecodeError>,
    /// The model the reply named.
    pub(crate) answered_by: Arc<str>,
    /// This question's share of its request's usage.
    pub(crate) usage: Option<Usage>,
    /// Whether the store answered it.
    pub(crate) cached: bool,
    /// This question's share of its request's HTTP attempts.
    pub(crate) requests_sent: u64,
    /// Every attempt its request made, when the call collects them.
    pub(crate) attempts: Arc<[AttemptObservation]>,
    /// The labels of the first and last input its request served.
    pub(crate) span: (usize, usize),
}

/// Why one input has no row.
#[derive(Debug)]
pub(crate) enum Failed<E> {
    /// The function refused the input.
    Asker(E),
    /// One of its questions cannot be sent even alone.
    Pack { error: PackError, at: usize },
    /// Its request or its lookup failed. `first` and `last` label the
    /// inputs the failed request served.
    Engine {
        error: Error,
        first: usize,
        last: usize,
    },
    /// The call stopped before this input finished.
    Stopped(Error),
}

/// What the host wants after one emitted input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Flow {
    Continue,
    Stop,
}

/// What closes a request beyond the backend's own limits.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Packing {
    /// `--batch N`, or `None` for the 4,096-input cap.
    pub(crate) inputs: Option<usize>,
    /// Whether the state is a context.
    pub(crate) context: bool,
    /// Whether rows show each request's attempts.
    pub(crate) detailed: bool,
}

/// The host side of the input bridge: one input, failure or end per ask.
pub(crate) struct Port<I, E>(Sender<Event<I, E>>);

impl<I, E> Port<I, E> {
    pub(crate) fn send(&self, input: Input<I, E>) -> Result<(), ()> {
        self.0.send(Event::Input(input)).map_err(|_| ())
    }
}

enum Event<I, E> {
    Input(Input<I, E>),
    Done(Vec<send::Done>),
}

impl Engine {
    /// Answer every input the reader hands over and emit each row, or why it
    /// has none, in input order. The reader runs on the host's own thread
    /// and answers each ask with one input. Every worker has joined on return.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when the store cannot be opened; every later failure
    /// reaches the host as an input's [`Failed`].
    pub(crate) fn ask_all<A: Asker>(
        &self,
        asker: &A,
        packing: Packing,
        start_reader: impl FnOnce(Receiver<()>, Port<A::Input, A::Error>),
        emit: impl FnMut(usize, Result<A::Row, Failed<A::Error>>) -> Flow,
        cancel: &Cancel,
    ) -> Result<(), Error> {
        let state = self.state(cancel)?;
        let store = self.store()?;
        let model = pack::model_json(self.backend().model().as_str())
            .map_err(|_| Error::Defect("a model could not be written as JSON"))?;
        let limits = self.pack_limits(packing);
        let window = (state.width + 1).saturating_mul(limits.inputs);
        let sender = send::Sender::new(self, &state, model.clone(), packing.detailed);
        let (events, received) = channel();
        let (ask, asked) = channel();
        start_reader(asked, Port(events.clone()));
        let counts = run::Counts {
            usage: &state.usage,
            cache_answers: self.storage().cache_answers,
        };
        workers::scoped(
            state.width,
            events,
            &|job| Event::Done(sender.send(job, cancel)),
            |work| {
                let call = run::Call {
                    url: self.backend().url().as_str().to_owned(),
                    model: self.backend().model().as_str().to_owned(),
                };
                let bounds = run::Bounds {
                    window,
                    jobs: state.width,
                };
                let packer = Packer::new(limits, model);
                run::Run::new(asker, call, store, packer, bounds, counts)
                    .drive(&ask, &received, &work, emit, cancel)
            },
        );
        Ok(())
    }

    /// The limits a request of this call closes at.
    pub(crate) fn pack_limits(&self, packing: Packing) -> PackLimits {
        PackLimits {
            ceiling: self.backend().ceiling(),
            profile: self.profile().cloned(),
            inputs: packing.inputs.unwrap_or(MOST_INPUTS).max(1),
            questions: None,
            context: packing.context,
        }
    }

    /// The call's store, by the modes table of ADR 0111 section 3, or `None`
    /// under `--no-cache`.
    fn store(&self) -> Result<Option<Store>, Error> {
        let storage = self.storage();
        let refresh = storage.refresh_cache
            || crate::core::adapters::built_in::is_mutable_alias(self.backend().model());
        let (folder, mode) = match (&storage.record, &storage.replay) {
            (Some(folder), Some(_)) if refresh => (folder, Mode::Refresh),
            (Some(folder), Some(_)) => (folder, Mode::Cache),
            (Some(folder), None) => (folder, Mode::Record),
            (None, Some(folder)) => (folder, Mode::Replay),
            (None, None) => return Ok(None),
        };
        Store::open(folder, mode, storage.private_default).map(Some)
    }
}

#[cfg(test)]
mod tests;
