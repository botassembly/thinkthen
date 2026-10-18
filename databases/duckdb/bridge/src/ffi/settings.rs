//! Copied caller-session settings at the C++/Rust ABI.

use super::text;
use crate::engines;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(super) struct BridgeSetting {
    name_bytes: *const u8,
    name_len: usize,
    value_bytes: *const u8,
    value_len: usize,
    literal: i32,
}

/// Copied values remain owned by the C++ session during this bridge call.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct BridgeSettings {
    pub(super) settings: *const BridgeSetting,
    pub(super) count: usize,
    pub(super) batch_bytes: *const u8,
    pub(super) batch_len: usize,
    max_requests_total: i64,
    pub(super) model_bytes: *const u8,
    pub(super) model_len: usize,
    cache_allowed: i32,
    record_allowed: i32,
    replay_allowed: i32,
}

pub(crate) fn asked(settings: &BridgeSettings) -> Result<engines::Asked, String> {
    let mut pairs = std::collections::BTreeMap::new();
    let values = super::setting_values(settings);
    for value in values {
        let name = text(value.name_bytes, value.name_len)?;
        let raw = text(value.value_bytes, value.value_len)?;
        let raw = if value.literal != 0 {
            raw.to_owned()
        } else {
            serde_json::Value::String(raw.to_owned()).to_string()
        };
        pairs.insert(name.to_owned(), raw);
    }
    Ok(engines::Asked {
        settings: pairs,
        max_requests_total: (settings.max_requests_total != i64::MIN)
            .then_some(settings.max_requests_total),
    })
}

pub(crate) fn batch(settings: &BridgeSettings) -> Result<Option<String>, String> {
    (!settings.batch_bytes.is_null())
        .then(|| text(settings.batch_bytes, settings.batch_len).map(str::to_owned))
        .transpose()
}

pub(crate) fn probe(settings: &BridgeSettings, path: &str) -> engines::Probe {
    let Ok(asked) = asked(settings) else {
        return engines::Probe::Refused;
    };
    for (name, allowed) in [
        ("cache", settings.cache_allowed),
        ("record", settings.record_allowed),
        ("replay", settings.replay_allowed),
    ] {
        if let Some(raw) = asked.settings.get(name)
            && let Ok(folder) = serde_json::from_str::<String>(raw)
            && path == format!("{}/.probe", folder.trim_end_matches('/'))
            && allowed == 0
        {
            return engines::Probe::Refused;
        }
    }
    engines::Probe::Allowed
}
