//! A typed view of the pure full-input request planner.

use std::fmt;

use crate::core::{self, Batcher, PlanSummary};

use super::bulk::selected_batch;
use super::engine::{DetailQuestion, Engine, Evidence};
use super::error::Error;
use super::options::CallOptions;

/// Prepared request counts before cache answers, refusal splits or retries.
/// The token band is an estimate of the prepared body bytes, not a bill.
#[derive(Clone, Eq, PartialEq)]
pub struct PlanEstimate {
    records: usize,
    requests: usize,
    estimated_bytes: usize,
    lower_tokens: usize,
    upper_tokens: usize,
    upper_bound: bool,
    first_body: Option<Vec<u8>>,
}

impl fmt::Debug for PlanEstimate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PlanEstimate")
            .field("records", &self.records)
            .field("requests", &self.requests)
            .field("estimated_bytes", &self.estimated_bytes)
            .field(
                "estimated_input_tokens",
                &(self.lower_tokens, self.upper_tokens),
            )
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
        (self.lower_tokens, self.upper_tokens)
    }
    /// Whether later staged requests could only be bounded before replies.
    #[must_use]
    pub const fn upper_bound(&self) -> bool {
        self.upper_bound
    }
    /// The first complete prepared request body, if the input was nonempty.
    #[must_use]
    pub fn first_body(&self) -> Option<&[u8]> {
        self.first_body.as_deref()
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
        let question = question.question();
        let setting = selected_batch(question, &options, self.batch)?;
        let context = options
            .context_text()
            .map(super::engine::evidence)
            .transpose()?;
        let engine = self.asking(question)?;
        let mut batcher = Batcher::new(
            engine.backend().clone(),
            engine.profile().cloned(),
            question.core.clone(),
            setting,
            context,
        )
        .map_err(Error::refused)?;
        let mut summary = PlanSummary::new(false);
        for item in records {
            if let Some(most) = self.most.filter(|most| summary.records() >= *most) {
                return Err(Error::usage(format!(
                    "this engine answers at most {most} records in one call"
                )));
            }
            let text = item.evidence();
            let record = core::BatchRecord {
                evidence: super::engine::evidence(text)?,
                value: core::Json::String(text.to_owned()),
            };
            let mut closed = Vec::new();
            batcher.push(record, &mut closed).map_err(Error::refused)?;
            summary
                .record()
                .map_err(|_| Error::usage("the planned input is too large to count"))?;
            for batch in closed {
                summary
                    .batch(&batch)
                    .map_err(|_| Error::usage("the planned input is too large to count"))?;
            }
        }
        if let Some(batch) = batcher.finish().map_err(Error::refused)? {
            summary
                .batch(&batch)
                .map_err(|_| Error::usage("the planned input is too large to count"))?;
        }
        let counts = summary
            .counts()
            .map_err(|_| Error::usage("the planned input is too large to count"))?;
        Ok(PlanEstimate {
            records: counts.records,
            requests: counts.requests,
            estimated_bytes: counts.estimated_bytes,
            lower_tokens: counts.estimated_input_tokens.lower,
            upper_tokens: counts.estimated_input_tokens.upper,
            upper_bound: counts.upper_bound,
            first_body: summary.first_body().map(<[u8]>::to_vec),
        })
    }
}
