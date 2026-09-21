//! One encoded request and the production identity of those exact bytes.

use thinkthen_core::adapters::built_in;
use thinkthen_core::recording::{Digest, Exchange as Recorded};
use thinkthen_core::{Backend, Plan, Reply};

use crate::failure::Failure;

pub(crate) struct PreparedRequest {
    pub(crate) body: Vec<u8>,
    pub(crate) digest: Digest,
}

impl PreparedRequest {
    pub(crate) fn new(backend: &Backend, plan: &Plan) -> Result<Self, Failure> {
        let body = built_in::encode(plan)
            .map_err(|_| Failure::Defect("a request could not be written as JSON"))?;
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
}
