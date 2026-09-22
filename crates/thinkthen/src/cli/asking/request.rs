//! The command edge around one prepared engine request.

use crate::args::Common;
use crate::core::{Backend, BackendProfile, Plan};
use crate::edge::{self, Environment};
use crate::failure::Failure;
use crate::http::{Client, Exchange};
use crate::prepared_request::{Answered, PreparedRequest};
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
        crate::engine::request::Transport {
            client,
            max_retries: common.max_retries,
            retry_wait: environment.retry_wait(),
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
        edge::key,
        |prepared, key| {
            client
                .post(&Exchange {
                    url: backend.url().as_str(),
                    body: &prepared.body,
                    key,
                    max_retries: common.max_retries,
                    retry_wait: environment.retry_wait(),
                })
                .map_err(Failure::from)
        },
    )
}
