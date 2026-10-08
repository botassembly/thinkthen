//! Original versioned bytes decode strictly and execute through the native request.
use crate::failures::Failure;
use thinkthen::{CallOptions, Engine, Request, RequestEnvironment, RequestOutcome};
pub(super) fn call(
    engine: &Engine,
    text: &str,
    controls: CallOptions<'_>,
) -> Result<String, Failure> {
    let admitted = Request::from_json(text)?.admit()?;
    match engine.execute_request(
        &admitted,
        RequestEnvironment {
            controls,
            feed: None,
        },
    )? {
        RequestOutcome::Complete(call) => {
            let complete = call
                .complete()
                .ok_or_else(|| Failure::defect("a completed native request has no identity"))?;
            super::written(serde_json::to_string(&complete))
        }
        RequestOutcome::Failed { error, .. } => Err(error.into()),
    }
}
