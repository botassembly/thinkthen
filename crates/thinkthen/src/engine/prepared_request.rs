//! One encoded request and the production identity of those exact bytes.

use crate::core::adapters::built_in;
use crate::core::recording::{Digest, Exchange as Recorded};
use crate::core::{Backend, Plan, Reply};

use crate::engine::error::Error;

pub(crate) struct PreparedRequest {
    pub(crate) body: Vec<u8>,
    pub(crate) digest: Digest,
}

impl PreparedRequest {
    pub(crate) fn new(backend: &Backend, plan: &Plan) -> Result<Self, Error> {
        let body = built_in::encode(plan)
            .map_err(|_| Error::Defect("a request could not be written as JSON"))?;
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
