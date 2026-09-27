//! One encoded request and the production identity of those exact bytes.

use crate::core::adapters::built_in;
use crate::core::recording::{Digest, Exchange as Recorded};
use crate::core::{
    Backend, BackendProfile, LimitKind, Plan, Question, RelationEntityView, RelationPlan, Reply,
    plan_pairs, relation_evidence,
};

use crate::engine::error::Error;

pub(crate) struct PreparedRequest {
    pub(crate) body: Vec<u8>,
    pub(crate) digest: Digest,
}

/// One contiguous plan chunk and the exact request prepared from it.
pub(crate) struct PreparedChunk {
    pub(crate) plan: Plan,
    pub(crate) request: PreparedRequest,
}

#[cfg(test)]
thread_local! {
    /// How many plans this thread has prepared, so a test can catch a second preparation.
    pub(crate) static PREPARATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Every chunk prepared and checked before execution starts.
pub(crate) struct PreparedRequests {
    chunks: Vec<PreparedChunk>,
}

impl PreparedRequests {
    /// Split `plan` into the fewest contiguous chunks that pass `profile`,
    /// each at most `ceiling` bytes unless it holds one question.
    pub(crate) fn with_profile(
        backend: &Backend,
        plan: &Plan,
        profile: Option<&BackendProfile>,
        ceiling: Option<usize>,
    ) -> Result<Self, Error> {
        #[cfg(test)]
        PREPARATIONS.with(|count| count.set(count.get() + 1));
        let mut chunks = Vec::new();
        let mut consumed = 0;
        while consumed < plan.questions().len() {
            let (count, (chunk, body)) = longest(plan, profile, ceiling, consumed)?;
            let digest = Recorded::new(backend.url(), &body).digest();
            chunks.push(PreparedChunk {
                plan: chunk,
                request: PreparedRequest { body, digest },
            });
            consumed += count;
        }
        Ok(Self { chunks })
    }

    pub(crate) fn into_chunks(self) -> Vec<PreparedChunk> {
        self.chunks
    }
}

/// The request-byte ceiling a relation request splits at, unless the profile limits bytes.
pub(crate) fn relation_ceiling(
    backend: &Backend,
    profile: Option<&BackendProfile>,
) -> Option<usize> {
    backend
        .ceiling()
        .filter(|_| profile.is_none_or(|profile| !profile.limits_request_bytes()))
}

/// One concrete relation after the only relation fallback decision.
pub(crate) struct SettledRelation {
    pub(crate) planned: RelationPlan,
    pub(crate) requests: PreparedRequests,
    /// The backend-profile limit that turned a choice into yes/no questions.
    pub(crate) fallback: Option<LimitKind>,
}

impl SettledRelation {
    /// Prepare one planned concrete relation for `relate`.
    ///
    /// A choice refused by a backend-profile option or request-byte limit
    /// becomes yes/no questions for this concrete relation alone.
    pub(crate) fn settle<E: RelationEntityView>(
        backend: &Backend,
        profile: Option<&BackendProfile>,
        source: Option<&str>,
        entities: &[E],
        planned: RelationPlan,
    ) -> Result<Self, Error> {
        let ceiling = relation_ceiling(backend, profile);
        let plan = relation_request(backend, source, entities, &planned)?;
        match PreparedRequests::with_profile(backend, &plan, profile, ceiling) {
            Ok(requests) => Ok(Self {
                planned,
                requests,
                fallback: None,
            }),
            Err(Error::ProfileLimit(limit))
                if matches!(planned.questions.first(), Some(Question::Choose { .. }))
                    && limit.permits_relation_fallback() =>
            {
                let planned = plan_pairs(entities, &planned.relation)
                    .map_err(|_| Error::Defect("relation fallback planning failed"))?;
                let plan = relation_request(backend, source, entities, &planned)?;
                let requests = PreparedRequests::with_profile(backend, &plan, profile, ceiling)?;
                Ok(Self {
                    planned,
                    requests,
                    fallback: Some(limit.kind),
                })
            }
            Err(error) => Err(error),
        }
    }
}

fn relation_request<E: RelationEntityView>(
    backend: &Backend,
    source: Option<&str>,
    entities: &[E],
    planned: &RelationPlan,
) -> Result<Plan, Error> {
    let evidence = relation_evidence(source, entities, &planned.relation)
        .map_err(|_| Error::Defect("relation state could not be built"))?;
    Plan::new(evidence, backend.model().clone(), planned.questions.clone())
        .map_err(|_| Error::Defect("relation planned no questions"))
}

type Candidate = (Plan, Vec<u8>);

/// The longest fitting chunk after `consumed`. Every limit tightens as a chunk
/// grows, so the count doubles up to the remainder, then the gap halves.
fn longest(
    plan: &Plan,
    profile: Option<&BackendProfile>,
    ceiling: Option<usize>,
    consumed: usize,
) -> Result<(usize, Candidate), Error> {
    let remaining = plan.questions().len() - consumed;
    let fits = |count| {
        candidate(plan, profile, consumed, count)
            .ok()
            .filter(|(_, body)| ceiling.is_none_or(|most| body.len() <= most))
    };
    let mut best = (1, candidate(plan, profile, consumed, 1)?);
    let mut over = remaining + 1;
    while best.0 < remaining && over == remaining + 1 {
        let count = (best.0 * 2).min(remaining);
        match fits(count) {
            Some(found) => best = (count, found),
            None => over = count,
        }
    }
    while over <= remaining && over - best.0 > 1 {
        let count = best.0 + (over - best.0) / 2;
        match fits(count) {
            Some(found) => best = (count, found),
            None => over = count,
        }
    }
    Ok(best)
}

fn candidate(
    plan: &Plan,
    profile: Option<&BackendProfile>,
    consumed: usize,
    count: usize,
) -> Result<Candidate, Error> {
    let questions = plan
        .questions()
        .iter()
        .skip(consumed)
        .take(count)
        .cloned()
        .collect();
    let chunk = Plan::new(plan.evidence().clone(), plan.model().clone(), questions)
        .map_err(|_| Error::Defect("a request chunk asks nothing"))?;
    let body = checked_body(&chunk, profile)?;
    Ok((chunk, body))
}

impl PreparedRequest {
    #[cfg(test)]
    pub(crate) fn new(backend: &Backend, plan: &Plan) -> Result<Self, Error> {
        Self::with_profile(backend, plan, None)
    }

    pub(crate) fn with_profile(
        backend: &Backend,
        plan: &Plan,
        profile: Option<&BackendProfile>,
    ) -> Result<Self, Error> {
        let body = checked_body(plan, profile)?;
        let digest = Recorded::new(backend.url(), &body).digest();
        Ok(Self { body, digest })
    }

    pub(crate) fn recorded<'a>(&'a self, backend: &'a Backend) -> Recorded<'a> {
        Recorded::new(backend.url(), &self.body)
    }
}

/// Encode `plan` and check the exact body against `profile`.
fn checked_body(plan: &Plan, profile: Option<&BackendProfile>) -> Result<Vec<u8>, Error> {
    let body = built_in::encode(plan)
        .map_err(|_| Error::Defect("a request could not be written as JSON"))?;
    if let Some(profile) = profile {
        let evidence = plan
            .evidence()
            .as_text()
            .map_err(|_| Error::Defect("the evidence could not be written as JSON"))?;
        profile
            .check(plan, evidence.as_ref(), &body)
            .map_err(Error::ProfileLimit)?;
    }
    Ok(body)
}

pub(crate) struct Answered {
    pub(crate) reply: Reply,
    pub(crate) replayed: bool,
    pub(crate) request: Digest,
    pub(crate) requests_sent: u64,
}
