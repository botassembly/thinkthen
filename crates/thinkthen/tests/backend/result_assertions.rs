//! Shared assertions over detailed result metadata.

use std::io;
use std::process::Output;

/// Return a detailed result without the members a live run and a replay may
/// differ in, plus `meta.cached` and `meta.requests_sent`. A replay adds no
/// `meta.attempts` event, so that member is dropped too.
pub(crate) fn normalized_details(output: &Output) -> io::Result<(String, bool, u64)> {
    let mut details: serde_json::Value =
        serde_json::from_slice(&output.stdout).map_err(io::Error::other)?;
    let meta = details
        .get_mut("meta")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(|| io::Error::other("details carry no meta"))?;
    let sent = meta
        .remove("requests_sent")
        .and_then(|sent| sent.as_u64())
        .ok_or_else(|| io::Error::other("meta carries no integer requests_sent"))?;
    let cached = meta
        .remove("cached")
        .and_then(|cached| cached.as_bool())
        .ok_or_else(|| io::Error::other("meta carries no boolean cached"))?;
    meta.remove("attempts");
    Ok((details.to_string(), cached, sent))
}
