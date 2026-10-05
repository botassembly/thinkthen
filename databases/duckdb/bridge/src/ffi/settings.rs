//! Copied caller-session settings at the C++/Rust ABI.

use super::text;
use crate::engines;

/// Unset numeric SQL settings use `i64::MIN`; every valid setting is larger.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct BridgeSettings {
    pub(super) batch_bytes: *const u8,
    pub(super) batch_len: usize,
    throttle: i64,
    max_requests: i64,
    max_request_bytes: i64,
    max_requests_total: i64,
    backend_bytes: *const u8,
    backend_len: usize,
    cache_bytes: *const u8,
    cache_len: usize,
    cache_allowed: i32,
    pub(super) model_bytes: *const u8,
    pub(super) model_len: usize,
    timeout: i64,
    max_retries: i64,
    profile_bytes: *const u8,
    profile_len: usize,
    record_bytes: *const u8,
    record_len: usize,
    replay_bytes: *const u8,
    replay_len: usize,
    record_allowed: i32,
    replay_allowed: i32,
}

pub(crate) fn asked(settings: &BridgeSettings) -> Result<engines::Asked, String> {
    let present = |value| (value != i64::MIN).then_some(value);
    let optional = |bytes: *const u8, len: usize| {
        (!bytes.is_null())
            .then(|| text(bytes, len).map(str::to_owned))
            .transpose()
    };
    let cache = optional(settings.cache_bytes, settings.cache_len)?;
    Ok(engines::Asked {
        backend: optional(settings.backend_bytes, settings.backend_len)?,
        throttle: present(settings.throttle),
        max_requests: present(settings.max_requests),
        max_request_bytes: present(settings.max_request_bytes),
        max_requests_total: present(settings.max_requests_total),
        cache,
        model: optional(settings.model_bytes, settings.model_len)?,
        timeout: present(settings.timeout),
        max_retries: present(settings.max_retries),
        profile: optional(settings.profile_bytes, settings.profile_len)?,
        record: optional(settings.record_bytes, settings.record_len)?,
        replay: optional(settings.replay_bytes, settings.replay_len)?,
    })
}

pub(crate) fn batch(settings: &BridgeSettings) -> Result<Option<String>, String> {
    (!settings.batch_bytes.is_null())
        .then(|| text(settings.batch_bytes, settings.batch_len).map(str::to_owned))
        .transpose()
}

pub(crate) fn probe(settings: &BridgeSettings, path: &str) -> engines::Probe {
    for (bytes, len, allowed) in [
        (
            settings.cache_bytes,
            settings.cache_len,
            settings.cache_allowed,
        ),
        (
            settings.record_bytes,
            settings.record_len,
            settings.record_allowed,
        ),
        (
            settings.replay_bytes,
            settings.replay_len,
            settings.replay_allowed,
        ),
    ] {
        if let Ok(folder) = text(bytes, len)
            && path == format!("{}/.probe", folder.trim_end_matches('/'))
            && allowed == 0
        {
            return engines::Probe::Refused;
        }
    }
    engines::Probe::Allowed
}
