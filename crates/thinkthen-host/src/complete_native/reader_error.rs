//! Explicit client/authorized reader failures carry no caller-supplied call facts.
use super::usage;
use serde_json::Value;
use thinkthen::{Error, ErrorKind};
pub(super) fn decode(source: &str) -> Result<Error, Error> {
    let document: Value =
        serde_json::from_str(source).map_err(|_| usage("invalid reader error"))?;
    let outer = document
        .as_object()
        .ok_or_else(|| usage("reader error is a native envelope"))?;
    if outer.len() != 1 {
        return Err(usage("reader error cannot supply call facts"));
    }
    let detail = outer
        .get("error")
        .and_then(Value::as_object)
        .ok_or_else(|| usage("reader error requires a native error"))?;
    if detail.len() != 4 || detail.get("retryable") != Some(&Value::Bool(false)) {
        return Err(usage("reader error is a pre-start nonretryable error"));
    }
    let kind = match detail.get("kind").and_then(Value::as_str) {
        Some("local") => ErrorKind::Local,
        Some("usage") => ErrorKind::Usage,
        _ => return Err(usage("reader errors are local or usage failures")),
    };
    let message = detail
        .get("message")
        .and_then(Value::as_str)
        .ok_or_else(|| usage("reader error message is text"))?;
    let error = Error::new(kind, message);
    if serde_json::to_value(error.complete()).map_err(|_| super::defect())? != document {
        return Err(usage("reader error is an unstarted native reader failure"));
    }
    Ok(error)
}
