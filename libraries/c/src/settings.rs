//! The closed JSON settings object for the C constructor.

use std::num::NonZeroUsize;
use std::time::Duration;

use serde_json::{Map, Value};
use thinkthen::{BatchSetting, Engine, EngineBuilder};

use crate::failures::Failure;

fn apply(
    mut builder: EngineBuilder,
    values: &Map<String, Value>,
) -> Result<EngineBuilder, Failure> {
    if values.contains_key("usd_per_million_input") || values.contains_key("usd_per_million_output")
    {
        let input = values
            .get("usd_per_million_input")
            .and_then(Value::as_str)
            .ok_or_else(|| Failure::usage("settings prices require both decimal strings"))?;
        let output = values
            .get("usd_per_million_output")
            .and_then(Value::as_str)
            .ok_or_else(|| Failure::usage("settings prices require both decimal strings"))?;
        builder = builder.prices_usd_per_million(input, output)?;
    }
    for (key, value) in values {
        let text = || {
            value
                .as_str()
                .ok_or_else(|| Failure::usage(format!("settings {key} is a string")))
        };
        let whole = || {
            value
                .as_u64()
                .ok_or_else(|| Failure::usage(format!("settings {key} is a whole number")))
        };
        let unknown = || Failure::usage(format!("settings JSON has unknown key {key}"));
        builder = match key.as_str() {
            "usd_per_million_input" | "usd_per_million_output" => builder,
            "backend" => builder.backend(text()?)?,
            "base_url" => builder.base_url(text()?)?,
            "model" => builder.model(text()?)?,
            "throttle" => {
                builder.throttle(u8::try_from(whole()?).map_err(|_| {
                    Failure::usage("a throttle is a whole number from 1 through 32")
                })?)?
            }
            "max_requests" if value.is_null() => builder.max_requests(None)?,
            "max_requests" => {
                builder.max_requests(Some(usize::try_from(whole()?).map_err(|_| {
                    Failure::usage("a request limit is a whole number of 1 or more")
                })?))?
            }
            "max_requests_total" if value.is_null() => builder.max_requests_total(None),
            "max_requests_total" => builder.max_requests_total(Some(whole()?)),
            "max_estimated_input_tokens_total" if value.is_null() => {
                builder.max_estimated_input_tokens_total(None)
            }
            "max_estimated_input_tokens_total" => {
                builder.max_estimated_input_tokens_total(Some(whole()?))
            }
            "max_request_bytes" => builder
                .max_request_bytes(usize::try_from(whole()?).map_err(|_| {
                    Failure::usage("a request size is a whole number of 1 or more")
                })?)?,
            "cache" if value == &Value::Bool(false) => builder.no_cache(),
            "cache" if value.is_string() => builder.cache_at(text()?)?,
            "cache" => return Err(Failure::usage("settings cache is false or a folder path")),
            "refresh_cache" => builder.refresh_cache(
                value
                    .as_bool()
                    .ok_or_else(|| Failure::usage("settings refresh_cache is a boolean"))?,
            ),
            "record" => builder.record(text()?)?,
            "replay" => builder.replay(text()?)?,
            "profile" => builder.profile(text()?)?,
            "timeout" => builder.timeout(Duration::from_secs(whole()?))?,
            "max_retries" => builder.max_retries(
                u32::try_from(whole()?)
                    .map_err(|_| Failure::usage("settings max_retries is a whole number"))?,
            ),
            "batch" => builder.batch(batch(value)?),
            _ => return Err(unknown()),
        };
    }
    Ok(builder)
}

pub(crate) fn batch(value: &Value) -> Result<BatchSetting, Failure> {
    if value.as_str() == Some("max") {
        return Ok(BatchSetting::Max);
    }
    value
        .as_u64()
        .and_then(|count| usize::try_from(count).ok())
        .and_then(NonZeroUsize::new)
        .map(BatchSetting::Records)
        .ok_or_else(|| Failure::usage("batch takes max or a whole number of at least 1"))
}

pub(crate) fn build(text: &str) -> Result<Engine, Failure> {
    EngineBuilder::validate_settings_json(text).map_err(Failure::from)?;
    let value: Value =
        serde_json::from_str(text).map_err(|_| Failure::usage("settings JSON is one object"))?;
    let values = value
        .as_object()
        .ok_or_else(|| Failure::usage("settings JSON is one object"))?;
    apply(EngineBuilder::from_env()?, values)?
        .build()
        .map_err(Failure::from)
}
