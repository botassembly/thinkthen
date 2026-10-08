//! The same aggregate state is admitted on probes and actual staged sends.
use super::{Asks, Bound, Request};
use crate::core::{Backend, BackendProfile};
use crate::engine::error::Error;
pub(super) fn contextual_requests(
    asks: &Asks,
    backend: &Backend,
    profile: Option<&BackendProfile>,
    context: Option<&crate::core::Json>,
    bound: Bound,
) -> Result<Vec<Request>, Error> {
    match context {
        Some(context) => asks
            .clone()
            .with_context_value(backend, context)?
            .requests(backend, profile, bound),
        None => asks.requests(backend, profile, bound),
    }
}
