//! Planned questions asked one input each on the question pipeline: `find`,
//! the three `recognize` steps and `relate`, by ADR 0111 section 5.
//!
//! Each keeps its own state and wire form, so its answers are cached by
//! address, model, state and question, as the record functions' are.

use crate::core::adapters::built_in::DecodeError;
use crate::core::pack::{self, Ask, Entry, PackError, PackLimits, Packer};
use crate::core::recording::{Digest, Exchange as Recorded};
use crate::core::{Backend, BackendProfile, ModelName, Plan, Question, Reply, Usage};
use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::engine::pipeline::{self, Asker, Failed, Flow, Packing};

use super::{Answered, Engine};

/// The most questions one `relate` or `recognize` step 3 request holds,
/// from ADR 0057 item 4.
const MOST_PAIR_QUESTIONS: usize = 400;

/// What closes one step's requests besides a profile.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Bound {
    questions: Option<usize>,
    /// Whether a request closes at the backend's request size.
    sized: bool,
}

impl Bound {
    /// `find` and the first two `recognize` steps: one request per state,
    /// split only by a profile.
    pub(crate) const WHOLE: Self = Self {
        questions: None,
        sized: false,
    };

    /// `relate` and `recognize` step 3: at most 400 questions, or a
    /// profile's smaller question limit, within the request size.
    pub(crate) fn pairs(profile: Option<&BackendProfile>) -> Self {
        Self {
            questions: Some(
                profile
                    .and_then(|profile| profile.max_questions)
                    .map_or(MOST_PAIR_QUESTIONS, |limit| limit.min(MOST_PAIR_QUESTIONS)),
            ),
            sized: true,
        }
    }
}

/// The wire questions of some plans, in order, beside the logical question
/// that reads them. A tag question asks once per label.
#[derive(Default)]
pub(crate) struct Asks {
    asks: Vec<Ask>,
    questions: Vec<Question>,
    /// The first wire question of each logical question, and its count.
    spans: Vec<(usize, usize)>,
}

/// One request a plan shows before any send, as the pipeline packs it with
/// nothing cached.
pub(crate) struct Request {
    pub(crate) body: Vec<u8>,
    pub(crate) digest: Digest,
    /// The place of each logical question it holds, once per wire question.
    pub(crate) places: Vec<usize>,
}

impl Asks {
    /// Add every question of `plan`, each with its wire questions.
    pub(crate) fn add(&mut self, backend: &Backend, plan: &Plan) -> Result<(), Error> {
        let asks = pack::asks(backend.url(), plan)
            .map_err(|_| Error::Defect("a request could not be written as JSON"))?;
        let mut start = self.asks.len();
        for question in plan.questions() {
            let count = match question {
                Question::Tag { labels, .. } => labels.count(),
                _ => 1,
            };
            self.spans.push((start, count));
            start += count;
        }
        if start != self.asks.len() + asks.len() {
            return Err(Error::Defect(
                "a plan's wire questions do not match its questions",
            ));
        }
        self.asks.extend(asks);
        self.questions.extend(plan.questions().iter().cloned());
        Ok(())
    }

    /// The number of logical questions.
    pub(crate) const fn len(&self) -> usize {
        self.questions.len()
    }

    pub(crate) const fn is_empty(&self) -> bool {
        self.questions.is_empty()
    }

    /// The wire questions of the logical question at `place`.
    fn wire(&self, place: usize) -> Option<&[Ask]> {
        let (start, count) = *self.spans.get(place)?;
        self.asks.get(start..start + count)
    }

    pub(crate) fn questions(&self) -> &[Question] {
        &self.questions
    }

    /// The most questions one request may hold beside the step's bound. It
    /// is every question, so the pipeline's window, a multiple of it, holds
    /// them all and never closes a request early.
    fn inputs(&self) -> usize {
        self.asks.len().max(1)
    }

    /// The requests these questions make with nothing cached. A question
    /// that cannot go alone refuses them all, so nothing is sent.
    pub(crate) fn requests(
        &self,
        backend: &Backend,
        profile: Option<&BackendProfile>,
        bound: Bound,
    ) -> Result<Vec<Request>, Error> {
        let mut packer = packer(backend, profile, bound, self.inputs())?;
        let mut closed = Vec::new();
        for place in 0..self.len() {
            let wire = self
                .wire(place)
                .ok_or(Error::Defect("a place asks nothing"))?;
            let entries = wire.iter().map(|ask| entry(ask, place)).collect();
            packer.add(entries, &mut closed).map_err(packed)?;
        }
        closed.extend(packer.close());
        Ok(closed
            .into_iter()
            .map(|request| Request {
                digest: Recorded::new(backend.url(), &request.body).digest(),
                body: request.body,
                places: request.items,
            })
            .collect())
    }
}

fn packer(
    backend: &Backend,
    profile: Option<&BackendProfile>,
    bound: Bound,
    inputs: usize,
) -> Result<Packer<usize>, Error> {
    let model = pack::model_json(backend.model().as_str())
        .map_err(|_| Error::Defect("a model could not be written as JSON"))?;
    let limits = PackLimits {
        ceiling: if bound.sized {
            backend.ceiling()
        } else {
            usize::MAX
        },
        profile: profile.cloned(),
        inputs,
        questions: bound.questions,
        context: false,
    };
    Ok(Packer::new(limits, model))
}

fn entry(ask: &Ask, place: usize) -> Entry<usize> {
    Entry {
        state: ask.state.clone(),
        question: std::sync::Arc::clone(&ask.question),
        options: pipeline::options(ask),
        item: place,
    }
}

/// A question the packer cannot send even alone.
fn packed(error: PackError) -> Error {
    match error {
        PackError::Profile(limit) => Error::ProfileLimit(limit),
        PackError::Context { .. } => Error::Defect("a planned state was read as a context"),
    }
}

/// Each place's one question, read back into a one-answer reply whose
/// request names the question key.
struct Each<'a>(&'a Asks);

impl Asker for Each<'_> {
    type Input = usize;
    type Row = Answered;
    type Error = Error;

    fn label(&self, place: &usize) -> usize {
        place + 1
    }

    fn asks(&self, place: &usize) -> Result<Vec<Ask>, Error> {
        let wire = self.0.wire(*place);
        Ok(wire.ok_or(Error::Defect("a place asks nothing"))?.to_vec())
    }

    fn row(&self, place: usize, answers: Vec<pipeline::Answered>) -> Result<Answered, Error> {
        let question = self.0.questions.get(place);
        let (Some(question), Some(first)) = (question, answers.first()) else {
            return Err(Error::Defect("a place has no answer"));
        };
        let stored: Vec<_> = answers
            .iter()
            .map(|answered| answered.answer.as_deref().map_err(DecodeError::cause))
            .collect();
        let model = &*first.answered_by;
        let outcomes = pack::read(std::slice::from_ref(question), &stored, model)?;
        let model =
            ModelName::reported(model).map_err(|_| Error::Defect("a reply named no model"))?;
        let usage = answers
            .iter()
            .try_fold(None, |total, answered| summed(total, answered.usage))?;
        Ok(Answered {
            reply: Reply::new(model, outcomes, usage),
            replayed: answers.iter().all(|answered| answered.cached),
            request: Digest::named(first.key.hex()),
            requests_sent: answers.iter().map(|answered| answered.requests_sent).sum(),
        })
    }
}

impl Engine {
    /// Ask every question, packing requests within `bound`, and hand each
    /// one-answer reply on in question order. A question that cannot go
    /// alone refuses the call before any send. The first failure in question
    /// order stops the call and returns.
    pub(crate) fn ask_each(
        &self,
        asks: &Asks,
        bound: Bound,
        cancel: &Cancel,
        mut each: impl FnMut(usize, Answered) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if asks.is_empty() {
            return Ok(());
        }
        let planned = asks.requests(&self.backend, self.profile(), bound)?.len();
        let packing = Packing {
            inputs: Some(asks.inputs()),
            questions: bound.questions,
            sized: bound.sized,
            context: false,
            detailed: false,
            continues: false,
        };
        let mut failure = None;
        let mut place = 0;
        let host = pipeline::eager((0..asks.len()).collect(), |row| {
            let taken = match row {
                Ok(answered) => each(place, answered),
                Err(failed) => Err(failure_of(failed)),
            };
            place += 1;
            match taken {
                Ok(()) => Flow::Continue,
                Err(error) => {
                    failure = Some(error);
                    Flow::Stop
                }
            }
        });
        self.ask_all(&Each(asks), packing, host, cancel)?;
        match failure {
            Some(error) => Err(error),
            // A stop seen while several requests were out still stops the
            // call once they finish, as the chunked sender it replaced did.
            // One sent request finishes and returns its answer.
            None if planned > 1
                && matches!(cancel.remaining_without_check(), Err(Error::Cancelled)) =>
            {
                Err(Error::Cancelled)
            }
            None if place == asks.len() => Ok(()),
            None => Err(Error::Defect("a planned question had no answer")),
        }
    }
}

fn failure_of(failed: Failed<Error>) -> Error {
    match failed {
        Failed::Asker(error) | Failed::Engine { error, .. } | Failed::Stopped(error) => error,
        Failed::Pack { error, .. } => packed(error),
    }
}

/// The sum of two usages, absent when either is.
pub(crate) fn summed(total: Option<Usage>, more: Option<Usage>) -> Result<Option<Usage>, Error> {
    match (total, more) {
        (Some(left), Some(right)) => left
            .checked_plus(right)
            .ok_or(Error::UsageOverflow)
            .map(Some),
        (None, held) | (held, None) => Ok(held),
    }
}

/// The model a step's rows report: the first live reply's, or with none
/// live, the first stored answer's. Live replies must agree, by ADR 0111
/// section 4; a stored answer's model takes no part in the check.
#[derive(Debug, Default)]
pub(crate) struct Models {
    live: Option<ModelName>,
    stored: Option<ModelName>,
}

impl Models {
    /// Take one reply's model. A live one goes through `check` against the
    /// live model held so far.
    pub(crate) fn take(
        &mut self,
        answered: &Answered,
        check: impl FnOnce(&mut Option<ModelName>, &ModelName) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let model = answered.reply.model();
        if answered.replayed {
            self.stored.get_or_insert_with(|| model.clone());
            return Ok(());
        }
        check(&mut self.live, model)
    }

    pub(crate) fn model(&self) -> Option<&ModelName> {
        self.live.as_ref().or(self.stored.as_ref())
    }
}
