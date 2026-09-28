//! One encoded request and the production identity of those exact bytes.

use crate::core::adapters::built_in;
use crate::core::recording::{Digest, Exchange as Recorded};
use crate::core::{Backend, BackendProfile, PairPlan, Plan, Reply};

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

/// The smaller request-byte limit a relation request splits under.
pub(crate) fn relation_ceiling(
    backend: &Backend,
    profile: Option<&BackendProfile>,
) -> Option<usize> {
    Some(
        profile
            .and_then(|profile| profile.max_request_bytes)
            .map_or(backend.ceiling(), |limit| limit.min(backend.ceiling())),
    )
}

/// Prepare the ordered pair questions under the shared 400-question bound and
/// the request-size and backend-profile limits.
pub(crate) fn pair_chunks(
    backend: &Backend,
    profile: Option<&BackendProfile>,
    planned: &PairPlan,
) -> Result<Vec<PreparedChunk>, Error> {
    let most = profile
        .and_then(|profile| profile.max_questions)
        .map_or(400, |limit| limit.min(400));
    let ceiling = relation_ceiling(backend, profile);
    let mut chunks = Vec::new();
    for questions in planned.questions.chunks(most) {
        let plan = Plan::new(
            planned.evidence.clone(),
            backend.model().clone(),
            questions.to_vec(),
        )
        .map_err(|_| Error::Defect("relation planned no questions"))?;
        chunks.extend(
            PreparedRequests::with_profile(backend, &plan, profile, ceiling)?.into_chunks(),
        );
    }
    Ok(chunks)
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
