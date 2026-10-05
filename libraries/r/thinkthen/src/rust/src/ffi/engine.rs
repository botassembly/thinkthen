//! Convert the public R constructor settings without changing positional arguments.

use extendr_api::{Rinternals, Types};

use super::{Crossed, Robj, Rtype, batch_of, text_of, whole_of};

pub(super) fn configure(values: [Robj; 14]) -> Crossed<()> {
    let [
        base_url,
        model,
        throttle,
        max_requests,
        max_requests_total,
        max_request_bytes,
        cache,
        timeout,
        max_retries,
        record,
        replay,
        profile,
        batch,
        backend,
    ] = values;
    let optional =
        |value: &Robj, what: &str| (!value.is_null()).then(|| text_of(value, what)).transpose();
    let cache = match cache.rtype() {
        Rtype::Null => None,
        Rtype::Logicals if cache.as_bool() == Some(false) => Some(None),
        _ => Some(Some(text_of(&cache, "cache")?)),
    };
    crate::choose_engine(crate::Settings {
        backend: optional(&backend, "backend")?,
        base_url: optional(&base_url, "base_url")?,
        model: optional(&model, "model")?,
        throttle: whole_of(&throttle, "throttle")?,
        max_requests: whole_of(&max_requests, "max_requests")?,
        max_requests_total: whole_of(&max_requests_total, "max_requests_total")?,
        max_request_bytes: whole_of(&max_request_bytes, "max_request_bytes")?,
        cache,
        timeout: whole_of(&timeout, "timeout")?,
        max_retries: whole_of(&max_retries, "max_retries")?,
        record: optional(&record, "record")?,
        replay: optional(&replay, "replay")?,
        profile: optional(&profile, "profile")?,
        batch: batch_of(&batch)?,
    })
}
