//! The command edge around one prepared engine request.

use crate::args::Common;
use crate::core::{Backend, BackendProfile, Plan};
use crate::edge::{self, Environment};
use crate::failure::Failure;
use crate::http::{Client, Exchange};
use crate::prepared_request::{Answered, PreparedChunk, PreparedRequest};
use crate::recorder::Recorder;

#[expect(
    clippy::too_many_arguments,
    reason = "one command request passes the existing transport and storage boundaries explicitly"
)]
pub(crate) fn ask(
    backend: &Backend,
    plan: &Plan,
    common: &Common,
    environment: &Environment,
    recorder: &Recorder,
    client: &Client,
    profile: Option<&BackendProfile>,
) -> Result<Answered, Failure> {
    crate::engine::request::ask_profile(
        backend,
        plan,
        profile,
        recorder,
        environment.cancel(),
        crate::engine::request::Transport {
            client,
            max_retries: common.max_retries,
            retry_wait: environment.retry_wait(),
            usage: environment.usage(),
        },
        edge::key,
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "a prepared grouped request shares the same explicit transport and storage boundary"
)]
pub(crate) fn ask_prepared(
    backend: &Backend,
    plan: &Plan,
    prepared: PreparedRequest,
    common: &Common,
    environment: &Environment,
    recorder: &Recorder,
    client: &Client,
) -> Result<Answered, Failure> {
    crate::engine::request::ask_prepared(
        backend,
        plan,
        prepared,
        recorder,
        environment.cancel(),
        environment.usage(),
        edge::key,
        |prepared, key| {
            client
                .post_observed(
                    &Exchange {
                        url: backend.url().as_str(),
                        body: &prepared.body,
                        key,
                        max_retries: common.max_retries,
                        retry_wait: environment.retry_wait(),
                    },
                    environment.cancel(),
                    || environment.usage().request_sent(),
                )
                .map_err(Failure::from)
        },
    )
}

/// The boundaries every prepared request of one command passes through.
pub(crate) struct Asking<'a> {
    pub(crate) backend: &'a Backend,
    pub(crate) common: &'a Common,
    pub(crate) environment: &'a Environment,
    pub(crate) recorder: &'a Recorder,
    pub(crate) client: &'a Client,
}

impl Asking<'_> {
    /// Send chunks prepared earlier, in order, and hand each reply on.
    pub(crate) fn chunks(
        &self,
        chunks: Vec<PreparedChunk>,
        mut each: impl FnMut(Answered) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        for chunk in chunks {
            each(ask_prepared(
                self.backend,
                &chunk.plan,
                chunk.request,
                self.common,
                self.environment,
                self.recorder,
                self.client,
            )?)?;
        }
        Ok(())
    }
}
