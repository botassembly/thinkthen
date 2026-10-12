//! Original versioned bytes decode strictly and execute through the native request.
use crate::failures::Failure;
use thinkthen::{CallOptions, Engine, Request, RequestEnvironment};
pub(super) fn call(
    engine: &Engine,
    text: &str,
    controls: CallOptions<'_>,
) -> Result<String, Failure> {
    let admitted = Request::from_json(text)?.admit()?;
    let call = crate::typed::executed(
        engine,
        &admitted,
        RequestEnvironment {
            controls,
            feed: None,
        },
    )?;
    let complete = call
        .complete()
        .ok_or_else(|| Failure::defect("a completed native request has no identity"))?;
    super::written(serde_json::to_string(&complete))
}
