//! One synchronous owner of a named call, its pull producer and final result.
use super::{Request, RequestArguments, RequestEnvironment, RequestOutcome, RequestValue};
use crate::{Call, Engine, Error};

impl Engine {
    /// Run a named function until its native call settles.
    ///
    /// Arguments carry the question, input and options. The environment owns at
    /// most one pull producer and borrows cancellation and deadline controls.
    /// The producer runs only on this caller; no result reader escapes the call.
    /// Rust borrowing keeps the engine alive for the entire invocation.
    ///
    /// # Errors
    /// Returns native admission, producer, provider and stop errors with final
    /// facts when execution started. A declared feed requires its named producer;
    /// a producer on an inline or source input is refused before it advances.
    pub fn call(
        &self,
        function: &str,
        arguments: RequestArguments,
        mut environment: RequestEnvironment<'_>,
    ) -> Result<Call<RequestValue>, Error> {
        environment.controls = environment.controls.started()?;
        environment.controls.admission()?;
        let request = super::admission::admit(
            Request::named(function, arguments)?,
            environment.controls.attachment_limit,
        )?;
        environment
            .controls
            .admit_preparation(request.request.call.function())?;
        if let Some(feed) = &environment.feed
            && let super::execution::FeedContents::Descriptors(descriptors) = &feed.contents
        {
            request.validate_descriptor_feed(&feed.name, descriptors, environment.controls)?;
        }
        if let Some(feed) = &mut environment.feed {
            use super::execution::FeedContents;
            let contents = std::mem::replace(
                &mut feed.contents,
                FeedContents::Items(Box::new(std::iter::empty())),
            );
            feed.contents = match contents {
                FeedContents::Items(items) => {
                    FeedContents::Items(Box::new(items.map(|item| item.map_err(Error::producer))))
                }
                FeedContents::Records(records) => {
                    FeedContents::Records(Box::new(records.map(|row| row.map_err(Error::producer))))
                }
                other => other,
            };
        }
        match self.execute_request(&request, environment)? {
            RequestOutcome::Complete(call) => Ok(call),
            RequestOutcome::Failed { completed, error } => Err(error.with_completed(completed)),
        }
    }
}
