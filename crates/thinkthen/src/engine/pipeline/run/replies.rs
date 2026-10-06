//! Accept wire observations once, persist their facts, and share them with waiters.

use super::Run;
use crate::core::{
    ModelName,
    pack::{self, Ask, QuestionKey},
};
use crate::engine::pipeline::{Answered, Asker, send::Done};
use crate::engine::store::{Found, JSONL, Row};
use crate::engine::{Cancel, error::Error};
use std::sync::Arc;

impl<A: Asker> Run<'_, A> {
    /// A stored answer as an answer; damaged saved evidence always refuses.
    pub(super) fn stored(
        &self,
        ask: &Ask,
        found: Found,
        label: usize,
        cancel: &Cancel,
    ) -> Result<Answered, Option<Error>> {
        let decodes = pack::read(
            std::slice::from_ref(&ask.decoder),
            &[Ok(found.answer.as_str())],
            &found.answered_by,
        )
        .is_ok_and(|outcomes| {
            outcomes
                .iter()
                .all(|outcome| matches!(outcome, crate::core::AnswerOutcome::Answered(_)))
        });
        if !decodes {
            return Err(Some(Error::Entry(
                JSONL.to_owned(),
                "holds an answer that no longer decodes".to_owned(),
            )));
        }
        if self.counts.cache_answers {
            self.counts.usage.cache_answer();
            cancel.cache_answer();
        }
        if let Ok(model) = ModelName::new(found.answered_by.clone()) {
            self.counts.usage.answered_by(&model);
        }
        cancel.answered_by(&found.answered_by);
        Ok(Answered {
            key: found.key,
            observation_id: Some(found.observation_id),
            batch_size: found.batch_size,
            origin: if self.store.as_ref().is_some_and(|store| store.replays()) {
                crate::core::Origin::Replay
            } else {
                crate::core::Origin::Cache
            },
            answer: Ok(Arc::from(found.answer)),
            answered_by: Arc::from(found.answered_by),
            usage: found.usage,
            cached: true,
            requests_sent: 0,
            attempts: Arc::from([]),
            span: (label, label),
        })
    }

    /// Store a reply's good answers and hand every answer to the inputs that wait on it.
    pub(super) fn answered(&mut self, done: Done, cancel: &Cancel) {
        let span = done
            .asks
            .iter()
            .fold((usize::MAX, 0), |(low, high), (_, label)| {
                (low.min(*label), high.max(*label))
            });
        let split = match done.result {
            Ok(split) => split,
            Err(error) => {
                // Every earlier input's questions went out before this
                // request, so their rows and this failure still print.
                if !self.continues && self.stopping.is_none() {
                    self.stopping = Some(error.clone());
                    self.closed.clear();
                }
                return self.fail_all(&done.asks, &error, span);
            }
        };
        let accepted = match Accepted::new(&self.call, &done.asks, &split) {
            Ok(accepted) => accepted,
            Err(error) => return self.fail_all(&done.asks, &error, span),
        };
        let rows = accepted.rows(&self.call, &done.asks, done.taken_at, &split);
        let written = match self.store.as_mut() {
            Some(store) => store.accept(&rows, done.storable, done.original.as_ref(), cancel),
            None => Ok(()),
        };
        drop(rows);
        if let Err(error) = written {
            return self.fail_all(&done.asks, &error, span);
        }
        let answered_by: Arc<str> = Arc::from(split.model.as_str());
        for (position, (((((ask, _), answer), usage), key), observation_id)) in done
            .asks
            .iter()
            .zip(split.answers)
            .zip(accepted.shares)
            .zip(accepted.keys)
            .zip(accepted.identities)
            .enumerate()
        {
            let answered = Answered {
                key,
                observation_id,
                batch_size: Some(accepted.batch_size),
                origin: crate::core::Origin::Live,
                answer: answer.map(Arc::from),
                answered_by: Arc::clone(&answered_by),
                usage,
                cached: false,
                requests_sent: crate::core::share(done.requests_sent, done.asks.len(), position),
                attempts: Arc::clone(&done.attempts),
                span,
            };
            self.deliver(&ask.key, answered);
        }
    }
}

struct Accepted {
    url: crate::core::Url,
    keys: Vec<QuestionKey>,
    identities: Vec<Option<crate::core::ObservationId>>,
    batch_size: std::num::NonZeroU32,
    shares: Vec<Option<crate::core::ReportedUsage>>,
}

impl Accepted {
    fn new(call: &super::Call, asks: &[(Ask, usize)], split: &pack::Split) -> Result<Self, Error> {
        if split.answers.len() != asks.len() {
            return Err(Error::Defect(
                "a reply answered the wrong number of questions",
            ));
        }
        let batch_size = u32::try_from(asks.len())
            .ok()
            .and_then(std::num::NonZeroU32::new)
            .ok_or(Error::Defect(
                "a successful request has no representable wire-question count",
            ))?;
        let identities = split
            .answers
            .iter()
            .map(|answer| {
                if answer.is_ok() {
                    crate::engine::invocation::observation_id().map(Some)
                } else {
                    Ok(None)
                }
            })
            .collect::<Result<_, _>>()?;
        let model_error = |_| Error::Defect("a model cannot be serialized");
        let requested = pack::model_json(&call.model).map_err(model_error)?;
        let reported = pack::model_json(split.model.as_str()).map_err(model_error)?;
        let url = crate::core::posting_address(&call.url)
            .map_err(|_| Error::Defect("the posting address cannot be normalized"))?;
        let keys = asks
            .iter()
            .map(|(ask, _)| {
                QuestionKey::complete(&url, &requested, &reported, ask.state.json(), &ask.question)
            })
            .collect();
        Ok(Self {
            url,
            keys,
            identities,
            batch_size,
            shares: pack::shares(split.usage, asks.len()),
        })
    }

    fn rows<'a>(
        &'a self,
        call: &'a super::Call,
        asks: &'a [(Ask, usize)],
        taken_at: i64,
        split: &'a pack::Split,
    ) -> Vec<Row<'a>> {
        asks.iter()
            .zip(&split.answers)
            .zip(&self.shares)
            .zip(&self.identities)
            .zip(&self.keys)
            .filter_map(|(((((ask, _), answer), usage), observation_id), key)| {
                Some(Row {
                    key: *key,
                    observation_id: observation_id.clone()?,
                    batch_size: Some(self.batch_size),
                    url: self.url.as_str(),
                    model: &call.model,
                    state: &ask.state,
                    question: &ask.question,
                    answer: answer.as_deref().ok()?,
                    answered_by: split.model.as_str(),
                    usage: *usage,
                    taken_at,
                    origin: "live",
                })
            })
            .collect()
    }
}
