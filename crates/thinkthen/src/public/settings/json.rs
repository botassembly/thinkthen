//! The admitted engine-settings document and its sole builder application.
use super::EngineBuilder;
use crate::{Error, RequestBatch};
use serde::{Deserialize, Deserializer};
use std::time::Duration;

// Each field declares its decoding type once. Named decoding errors never
// include caller values; the schema derives from these same admitted fields.
macro_rules! engine_fields {
    ($($field:ident : $ty:ty => $name:literal, $schema:literal;)*) => {
        #[derive(Deserialize)]
        #[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "EngineSettings"))]
        #[serde(deny_unknown_fields)]
        struct Fields {
            $(#[serde(default, deserialize_with = $name)]
              #[cfg_attr(test, schemars(with = $schema))]
              $field: Option<$ty>,)*
        }
        $(fn $field<'de, D: Deserializer<'de>>(de: D) -> Result<Option<$ty>, D::Error> {
            <$ty>::deserialize(de).map(Some).map_err(|_| {
                serde::de::Error::custom(concat!("settings ", $name, " has an invalid value"))
            })
        })*
    };
}
engine_fields! {
    backend: String => "backend", "String";
    base_url: String => "base_url", "String";
    model: String => "model", "String";
    throttle: u8 => "throttle", "u8";
    max_requests: Option<usize> => "max_requests", "Option<usize>";
    max_requests_total: Option<u64> => "max_requests_total", "Option<u64>";
    max_estimated_input_tokens_total: Option<u64> => "max_estimated_input_tokens_total", "Option<u64>";
    max_request_bytes: usize => "max_request_bytes", "usize";
    cache: CacheDocument => "cache", "CacheDocument";
    refresh_cache: bool => "refresh_cache", "bool";
    record: String => "record", "String";
    replay: String => "replay", "String";
    profile: String => "profile", "String";
    proxy: serde::de::IgnoredAny => "proxy", "serde_json::Value";
    timeout: u64 => "timeout", "u64";
    max_retries: u32 => "max_retries", "u32";
    batch: RequestBatch => "batch", "RequestBatch";
    usd_per_million_input: String => "usd_per_million_input", "String";
    usd_per_million_output: String => "usd_per_million_output", "String";
}

// Serde structs also accept positional arrays. This map-only entry preserves
// the engine constructor's object boundary while deriving all fields once.
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(with = "Fields"))]
pub(in crate::public) struct Document(Fields);
impl<'de> Deserialize<'de> for Document {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        struct Object;
        impl<'de> serde::de::Visitor<'de> for Object {
            type Value = Document;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("one settings object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<Document, A::Error> {
                Fields::deserialize(serde::de::value::MapAccessDeserializer::new(map)).map(Document)
            }
        }
        de.deserialize_map(Object)
    }
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
enum CacheDocument {
    Folder(String),
    Off(DisabledCache),
}
#[derive(Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(with = "bool", extend("const" = false)))]
#[serde(try_from = "bool")]
struct DisabledCache;
impl TryFrom<bool> for DisabledCache {
    type Error = &'static str;
    fn try_from(value: bool) -> Result<Self, Self::Error> {
        if value {
            Err("cache accepts false or a folder path")
        } else {
            Ok(Self)
        }
    }
}

impl Document {
    pub(super) fn parse(text: &str) -> Result<Self, Error> {
        serde_json::from_str(text).map_err(|error| {
            let message = error.to_string();
            if message.starts_with("duplicate field") {
                Error::usage("settings JSON repeats a member name")
            } else if let Some(key) = message
                .strip_prefix("unknown field `")
                .and_then(|s| s.split_once('`').map(|(key, _)| key))
            {
                Error::usage(format!(
                    "settings JSON has unknown key {}",
                    crate::core::safe_key(key)
                ))
            } else if let Some(message) = message.strip_prefix("settings ") {
                // Our field decoder supplies only a fixed field name and text.
                Error::usage(format!(
                    "settings {}",
                    message
                        .split(" at line ")
                        .next()
                        .unwrap_or("has an invalid value")
                ))
            } else {
                Error::usage("settings JSON is one object")
            }
        })
    }
    pub(super) fn apply(
        &self,
        mut builder: EngineBuilder,
        captured: bool,
    ) -> Result<EngineBuilder, Error> {
        let values = &self.0;
        // Reserved activation has no executable protocol; presence alone refuses.
        if values.proxy.is_some() {
            builder = builder.proxy(&crate::public::ProxyActivation::Null);
        }
        builder.check_proxy()?;
        match (
            &values.usd_per_million_input,
            &values.usd_per_million_output,
        ) {
            (Some(input), Some(output)) => {
                builder = builder.prices_usd_per_million(input, output)?
            }
            (None, None) => {}
            _ => {
                return Err(Error::usage(
                    "settings prices require both input and output fields",
                ));
            }
        }
        // Configured backend names belong to the captured configuration. All
        // other owner checks run before capture against a bare builder too.
        if captured && let Some(value) = &values.backend {
            builder = builder.backend(value)?;
        }
        if let Some(value) = &values.base_url {
            builder = builder.base_url(value)?;
        }
        if let Some(value) = &values.batch {
            builder = builder.batch(value.native()?);
        }
        if let Some(value) = &values.cache {
            builder = match value {
                CacheDocument::Folder(path) => builder.cache_at(path)?,
                CacheDocument::Off(_) => builder.no_cache(),
            };
        }
        if let Some(value) = values.max_estimated_input_tokens_total {
            builder = builder.max_estimated_input_tokens_total(value);
        }
        if let Some(value) = values.max_request_bytes {
            builder = builder.max_request_bytes(value)?;
        }
        if let Some(value) = values.max_requests {
            builder = builder.max_requests(value)?;
        }
        if let Some(value) = values.max_requests_total {
            builder = builder.max_requests_total(value);
        }
        if let Some(value) = values.max_retries {
            builder = builder.max_retries(value);
        }
        if let Some(value) = &values.model {
            builder = builder.model(value)?;
        }
        if let Some(value) = &values.profile {
            builder = builder.profile(value)?;
        }
        if let Some(value) = &values.record {
            builder = builder.record(value)?;
        }
        if let Some(value) = values.refresh_cache {
            builder = builder.refresh_cache(value);
        }
        if let Some(value) = &values.replay {
            builder = builder.replay(value)?;
        }
        if let Some(value) = values.throttle {
            builder = builder.throttle(value)?;
        }
        if let Some(value) = values.timeout {
            builder = builder.timeout(Duration::from_secs(value))?;
        }
        Ok(builder)
    }
}
impl EngineBuilder {
    /// Admit engine settings before capturing the environment and configuration.
    /// Configured backend names resolve against that captured configuration.
    /// # Errors
    /// Refuses duplicate, unknown or invalid settings and environment failures.
    pub fn from_settings_json(text: &str) -> Result<Self, Error> {
        let document = Document::parse(text)?;
        document.apply(Self::new(), false)?;
        document.apply(Self::from_env()?, true)
    }
}
