//! A typed view of the packer: the requests a call would send with an
//! empty store, before any is sent.

use std::collections::HashSet;
use std::fmt;

use serde::Serialize;

use crate::core::PlanSummary;
use crate::core::pack::{self, Entry, Packer};
use crate::engine::pipeline::{self, Asker as _};

use super::asking::{Decisions, Miss, Text, packed};
use super::pull;

use super::bulk::selected_batch;
use super::engine::{DetailQuestion, Engine, Evidence, only};
use super::error::Error;
use super::options::CallOptions;
use super::question::Kind;

/// Prepared request counts before cache answers, refusal splits or retries.
/// The token band is an estimate of the prepared body bytes, not a bill.
/// It serializes as the `plan` definition of the result schema.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "plan"))]
pub struct PlanEstimate {
    records: usize,
    requests: usize,
    estimated_bytes: usize,
    estimated_input_tokens: TokenBand,
    upper_bound: bool,
    #[serde(rename = "first_body_utf8")]
    first_body: Option<String>,
}

/// The low and high input-token estimates of a plan.
#[derive(Clone, Copy, Eq, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "tokenBand"))]
struct TokenBand {
    lower: usize,
    upper: usize,
}

impl fmt::Debug for PlanEstimate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PlanEstimate")
            .field("records", &self.records)
            .field("requests", &self.requests)
            .field("estimated_bytes", &self.estimated_bytes)
            .field("estimated_input_tokens", &self.estimated_input_tokens())
            .field("upper_bound", &self.upper_bound)
            .finish_non_exhaustive()
    }
}

impl PlanEstimate {
    /// Number of validated input records.
    #[must_use]
    pub const fn records(&self) -> usize {
        self.records
    }
    /// Planned requests before cache answers, refusal splits and retries.
    #[must_use]
    pub const fn requests(&self) -> usize {
        self.requests
    }
    /// Exact bytes of bodies known at plan time.
    #[must_use]
    pub const fn estimated_bytes(&self) -> usize {
        self.estimated_bytes
    }
    /// Measured low and high estimates of input tokens.
    #[must_use]
    pub const fn estimated_input_tokens(&self) -> (usize, usize) {
        (
            self.estimated_input_tokens.lower,
            self.estimated_input_tokens.upper,
        )
    }
    /// Whether later staged requests could only be bounded before replies.
    #[must_use]
    pub const fn upper_bound(&self) -> bool {
        self.upper_bound
    }
    /// The first complete prepared request body, if the input was nonempty.
    #[must_use]
    pub fn first_body(&self) -> Option<&[u8]> {
        self.first_body.as_deref().map(str::as_bytes)
    }
}

impl Engine {
    /// Validate all records and preview the same packed requests execution prepares.
    /// This reads no key, cache or network and sends nothing.
    ///
    /// # Errors
    /// Returns [`Error::Usage`] for invalid input or a request that cannot fit.
    pub fn plan<Q, I>(&self, question: &Q, records: I) -> Result<PlanEstimate, Error>
    where
        Q: DetailQuestion + ?Sized,
        I: IntoIterator,
        I::Item: Evidence,
    {
        self.plan_with(question, records, CallOptions::new())
    }

    /// [`Engine::plan`] under the call's batch and context controls.
    ///
    /// # Errors
    /// As [`Engine::plan`].
    pub fn plan_with<Q, I>(
        &self,
        question: &Q,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<PlanEstimate, Error>
    where
        Q: DetailQuestion + ?Sized,
        I: IntoIterator,
        I::Item: Evidence,
    {
        self.try_plan_with(question, records.into_iter().map(Ok), options)
    }

    /// Preview fallible records incrementally under the same admission and packing rules.
    /// No records after the first reader or admission failure are consumed.
    ///
    /// # Errors
    /// As [`Engine::plan_with`], also returning the first input error.
    pub fn try_plan_with<Q, I, T>(
        &self,
        question: &Q,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<PlanEstimate, Error>
    where
        Q: DetailQuestion + ?Sized,
        I: IntoIterator<Item = Result<T, Error>>,
        T: Evidence,
    {
        let question = question.question();
        only(
            question,
            &[
                Kind::Decide,
                Kind::Banded,
                Kind::Choose,
                Kind::Tag,
                Kind::Score,
            ],
            "plan",
        )?;
        let setting = selected_batch(question, &options, self.batch)?;
        let context = options
            .context_text()
            .map(super::engine::evidence)
            .transpose()?;
        let engine = self.asking(question)?;
        let asker = Decisions::new(&engine, question, context.clone());
        let too_large = || Error::usage("the planned input is too large to count");
        let model = pack::model_json(engine.backend().model().as_str())
            .map_err(|_| Error::defect("a model could not be written as JSON"))?;
        let mut packer = Packer::new(
            engine.pack_limits(pull::packing(setting, context.is_some(), false)),
            model,
        );
        if let Some(context) = &context {
            let state = pack::state(context)
                .map_err(|_| Error::defect("a context could not be written as JSON"))?;
            packer.check_state(&state).map_err(packed)?;
        }
        let mut summary = PlanSummary::new(false);
        let mut seen = HashSet::new();
        let mut closed = Vec::new();
        for (at, item) in records.into_iter().enumerate() {
            let item = item?;
            self.check_record_limit(at)?;
            let text = Text {
                at,
                input: crate::public::QuestionInput::Text(item.evidence().to_owned()),
            };
            let asks = asker.asks(&text).map_err(|miss| match miss {
                Miss::Refused(error) => error,
                Miss::Failed(_) => Error::defect("a plan read an answer"),
            })?;
            summary.record().map_err(|_| too_large())?;
            let entries = asks
                .into_iter()
                .filter(|ask| seen.insert(ask.key))
                .map(|ask| Entry {
                    options: pipeline::options(&ask),
                    state: ask.state,
                    question: ask.question,
                    item: (),
                })
                .collect();
            packer.add(entries, &mut closed).map_err(packed)?;
        }
        closed.extend(packer.close());
        for request in &closed {
            summary.request(&request.body).map_err(|_| too_large())?;
        }
        let counts = summary.counts().map_err(|_| too_large())?;
        let first_body = summary
            .first_body()
            .map(|body| String::from_utf8(body.to_vec()))
            .transpose()
            .map_err(|_| Error::defect("a planned body was not UTF-8"))?;
        Ok(PlanEstimate {
            records: counts.records,
            requests: counts.requests,
            estimated_bytes: counts.estimated_bytes,
            estimated_input_tokens: TokenBand {
                lower: counts.estimated_input_tokens.lower,
                upper: counts.estimated_input_tokens.upper,
            },
            upper_bound: counts.upper_bound,
            first_body,
        })
    }
}
