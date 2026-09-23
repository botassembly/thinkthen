//! One encoded request and the production identity of those exact bytes.

use crate::core::adapters::built_in;
use crate::core::recording::{Digest, Exchange as Recorded};
use crate::core::{Backend, BackendProfile, Plan, Reply};

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

/// Every chunk prepared and checked before execution starts.
pub(crate) struct PreparedRequests {
    chunks: Vec<PreparedChunk>,
}

impl PreparedRequests {
    pub(crate) fn with_profile(
        backend: &Backend,
        plan: &Plan,
        profile: Option<&BackendProfile>,
    ) -> Result<Self, Error> {
        let mut chunks = Vec::new();
        let mut consumed = 0;
        while consumed < plan.questions().len() {
            let remaining = plan.questions().len() - consumed;
            let mut longest = None;
            for count in 1..=remaining {
                match prepare_chunk(backend, plan, profile, consumed, count) {
                    Ok(chunk) => longest = Some((count, chunk)),
                    Err(Error::ProfileLimit(limit)) if limit.permits_split() => break,
                    Err(error) => return Err(error),
                }
            }
            let Some((count, chunk)) = longest else {
                let _impossible = prepare_chunk(backend, plan, profile, consumed, 1)?;
                return Err(Error::Defect(
                    "an impossible request chunk passed preflight",
                ));
            };
            chunks.push(chunk);
            consumed += count;
        }
        Ok(Self { chunks })
    }

    pub(crate) fn into_chunks(self) -> Vec<PreparedChunk> {
        self.chunks
    }
}

fn prepare_chunk(
    backend: &Backend,
    plan: &Plan,
    profile: Option<&BackendProfile>,
    consumed: usize,
    count: usize,
) -> Result<PreparedChunk, Error> {
    let questions = plan
        .questions()
        .iter()
        .skip(consumed)
        .take(count)
        .cloned()
        .collect();
    let chunk = Plan::new(plan.evidence().clone(), plan.model().clone(), questions)
        .map_err(|_| Error::Defect("a request chunk asks nothing"))?;
    let request = PreparedRequest::with_profile(backend, &chunk, profile)?;
    Ok(PreparedChunk {
        plan: chunk,
        request,
    })
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
        let digest = Recorded::new(backend.url(), &body).digest();
        Ok(Self { body, digest })
    }

    pub(crate) fn recorded<'a>(&'a self, backend: &'a Backend) -> Recorded<'a> {
        Recorded::new(backend.url(), &self.body)
    }
}

pub(crate) struct Answered {
    pub(crate) reply: Reply,
    pub(crate) replayed: bool,
    pub(crate) request: Digest,
    pub(crate) requests_sent: u64,
}
