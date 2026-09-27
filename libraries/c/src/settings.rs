//! The closed JSON settings object for the C constructor.

use std::collections::HashSet;
use std::time::Duration;

use serde_json::{Map, Value};
use thinkthen::{Engine, EngineBuilder};

use crate::failures::Failure;

fn usage(message: impl Into<String>) -> Failure {
    Failure::usage(message)
}

/// Reject repeated top-level keys before `serde_json::Map` can lose them.
#[expect(
    clippy::indexing_slicing,
    reason = "the scan bounds each JSON byte offset before reading it"
)]
fn duplicates(text: &str) -> Result<(), Failure> {
    let bytes = text.as_bytes();
    let mut at = bytes
        .iter()
        .position(|byte| *byte == b'{')
        .ok_or_else(|| usage("settings JSON is one object"))?
        + 1;
    let mut keys = HashSet::new();
    while at < bytes.len() {
        while at < bytes.len() && bytes[at].is_ascii_whitespace() {
            at += 1;
        }
        if bytes.get(at) == Some(&b'}') {
            return Ok(());
        }
        let start = at;
        at += 1;
        while at < bytes.len() {
            if bytes[at] == b'\\' {
                at += 2;
                continue;
            }
            if bytes[at] == b'"' {
                at += 1;
                break;
            }
            at += 1;
        }
        let key: String = serde_json::from_str(&text[start..at])
            .map_err(|_| usage("settings JSON is one object"))?;
        if !keys.insert(key.clone()) {
            return Err(usage(format!("settings JSON repeats key {key}")));
        }
        while at < bytes.len() && bytes[at] != b':' {
            at += 1;
        }
        at += 1;
        let mut depth = 0usize;
        let mut quoted = false;
        while at < bytes.len() {
            match bytes[at] {
                b'\\' if quoted => {
                    at += 2;
                    continue;
                }
                b'"' => quoted = !quoted,
                b'{' | b'[' if !quoted => depth += 1,
                b'}' | b']' if !quoted && depth > 0 => depth -= 1,
                b',' if !quoted && depth == 0 => {
                    at += 1;
                    break;
                }
                b'}' if !quoted && depth == 0 => return Ok(()),
                _ => {}
            }
            at += 1;
        }
    }
    Ok(())
}

fn text<'a>(key: &str, value: &'a Value) -> Result<&'a str, Failure> {
    value
        .as_str()
        .ok_or_else(|| usage(format!("settings {key} is a string")))
}

fn whole(key: &str, value: &Value) -> Result<u64, Failure> {
    value
        .as_u64()
        .ok_or_else(|| usage(format!("settings {key} is a whole number")))
}

fn apply(
    mut builder: EngineBuilder,
    values: &Map<String, Value>,
) -> Result<EngineBuilder, Failure> {
    for (key, value) in values {
        builder = match key.as_str() {
            "base_url" => builder.base_url(text(key, value)?).map_err(Failure::from)?,
            "model" => builder.model(text(key, value)?).map_err(Failure::from)?,
            "throttle" => builder
                .throttle(
                    u8::try_from(whole(key, value)?)
                        .map_err(|_| usage("a throttle is a whole number from 1 through 32"))?,
                )
                .map_err(Failure::from)?,
            "max_requests" if value.is_null() => {
                builder.max_requests(None).map_err(Failure::from)?
            }
            "max_requests" => builder
                .max_requests(Some(usize::try_from(whole(key, value)?).map_err(|_| {
                    usage("a request limit is a whole number of 1 or more")
                })?))
                .map_err(Failure::from)?,
            "cache" if value == &Value::Bool(false) => builder.no_cache(),
            "cache" if value.is_string() => {
                builder.cache_at(text(key, value)?).map_err(Failure::from)?
            }
            "cache" => return Err(usage("settings cache is false or a folder path")),
            "record" => builder.record(text(key, value)?).map_err(Failure::from)?,
            "replay" => builder.replay(text(key, value)?).map_err(Failure::from)?,
            "profile" => builder.profile(text(key, value)?).map_err(Failure::from)?,
            "timeout" => builder
                .timeout(Duration::from_secs(whole(key, value)?))
                .map_err(Failure::from)?,
            "max_retries" => builder.max_retries(
                u32::try_from(whole(key, value)?)
                    .map_err(|_| usage("settings max_retries is a whole number"))?,
            ),
            _ => return Err(usage(format!("settings JSON has unknown key {key}"))),
        };
    }
    Ok(builder)
}

pub(crate) fn build(text: &str) -> Result<Engine, Failure> {
    let value: Value =
        serde_json::from_str(text).map_err(|_| usage("settings JSON is one object"))?;
    let values = value
        .as_object()
        .ok_or_else(|| usage("settings JSON is one object"))?;
    duplicates(text)?;
    apply(EngineBuilder::from_env().map_err(Failure::from)?, values)?
        .build()
        .map_err(Failure::from)
}
