//! The configuration file's `backend` and `backends` fields (ADR 0114 section 2).
//!
//! Each added entry names a base, the name of the variable that holds its key,
//! and a model. Any entry, a built-in's included, may set `requests_per_minute`
//! (ticket 0343). The file never holds a key, and no refusal repeats a value.

use serde_json::value::RawValue;
use std::collections::BTreeMap;
use std::num::NonZeroU32;

use serde_json::{Map, Value};

use super::{ConfigError, refused};
use crate::core::{
    Backend, BackendProfile, DEFAULT_MODEL, MAX_PER_MINUTE, ModelName, Named, Prices, named,
};

const BUILT_IN: &str = "a configuration entry for a built-in backend holds only `requests_per_minute`, `usd_per_million_input`, `usd_per_million_output`, and `profile`";
const NAME: &str = "configuration field `backends` holds a name that is not 1 to 32 lowercase letters, digits, and hyphens";
const EXTRA: &str = "configuration backend entries hold only `url`, `path`, `key_env`, `model`, `requests_per_minute`, `both_sides`, `usd_per_million_input`, `usd_per_million_output`, and `profile`";
const MISSING: &str = "configuration backend entries need `url`, `key_env`, and `model`";
const STRINGS: &str = "configuration backend entries are objects whose `url`, `path`, `key_env`, and `model` are strings";
const URL: &str = "configuration backend field `url` must be a safe backend base";
const KEY_ENV: &str = "configuration backend field `key_env` names an environment variable: a capital letter or underscore, then capital letters, digits, and underscores";
const MODEL: &str = "configuration backend field `model` must be a model name, not blank";
const RATE: &str =
    "configuration backend field `requests_per_minute` must be a whole number from 1 to 60000";
const BACKEND: &str =
    "configuration field `backend` must name a built-in backend or an entry of `backends`";

/// Check every entry, in name order, and the `backend` field against them.
pub(super) fn read(
    backend: Option<&str>,
    entries: Option<&BTreeMap<String, Box<RawValue>>>,
) -> Result<Vec<Named>, ConfigError> {
    let mut named = Vec::new();
    for (name, value) in entries.into_iter().flatten() {
        named.push(entry(name, value)?);
    }
    if let Some(backend) = backend
        && named::find(backend, &named).is_err()
    {
        return Err(refused(BACKEND));
    }
    Ok(named)
}

fn entry(name: &str, raw: &RawValue) -> Result<Named, ConfigError> {
    let value: Value = serde_json::from_str(raw.get()).map_err(|_| refused(STRINGS))?;
    let profile: BTreeMap<String, Box<RawValue>> =
        serde_json::from_str(raw.get()).unwrap_or_default();
    let profile = profile.get("profile").map(|raw| raw.get());
    if let Some(built_in) = Named::built_in(name) {
        return built_in_entry(built_in, &value, profile);
    }

    if !named::valid_name(name) {
        return Err(refused(NAME));
    }
    let Value::Object(fields) = &value else {
        return Err(refused(STRINGS));
    };
    if fields.keys().any(|field| {
        !matches!(
            field.as_str(),
            "url"
                | "path"
                | "key_env"
                | "model"
                | "requests_per_minute"
                | "both_sides"
                | "usd_per_million_input"
                | "usd_per_million_output"
                | "profile"
        )
    }) {
        return Err(refused(EXTRA));
    }
    let text = |field: &str| match fields.get(field) {
        None => Err(refused(MISSING)),
        Some(Value::String(text)) => Ok(text.as_str()),
        Some(_) => Err(refused(STRINGS)),
    };
    let (url, key_env, model) = (text("url")?, text("key_env")?, text("model")?);
    if Backend::resolve(Some(url), None, DEFAULT_MODEL).is_err() {
        return Err(refused(URL));
    }
    if !variable_name(key_env) {
        return Err(refused(KEY_ENV));
    }
    if ModelName::new(model).is_err() {
        return Err(refused(MODEL));
    }
    let path = match fields.get("path") {
        None => None,
        Some(Value::String(path)) if named::valid_path(path) => Some(path.as_str()),
        Some(Value::String(_)) => {
            return Err(refused(
                "configuration backend field `path` must be one or more segments of letters, digits, and `-._~@`, joined by `/`",
            ));
        }
        Some(_) => return Err(refused(STRINGS)),
    };
    let entry = Named::new(name, url, key_env, model);
    setup(
        match path {
            Some(path) => entry.with_path(path),
            None => entry,
        },
        fields,
        profile,
    )
}

/// Validate built-in settings without accepting transport or model overrides.
fn built_in_entry(
    entry: Named,
    value: &Value,
    profile: Option<&str>,
) -> Result<Named, ConfigError> {
    let Value::Object(fields) = value else {
        return Err(refused(BUILT_IN));
    };
    if let Some(field) = ["url", "path", "key_env", "model", "both_sides"]
        .into_iter()
        .find(|field| fields.contains_key(*field))
    {
        return Err(ConfigError {
            message: format!(
                "configuration entry for built-in backend `{}` cannot set `{field}`; to use the built-in backend, remove `url`, `path`, `key_env`, `model`, and `both_sides` and delete the entry if it becomes empty; to keep custom routing, rename both the custom entry in `backends` and the selected `backend`",
                entry.name()
            ).into(),
            unreadable: false,
            price: None,
        });
    }
    if fields.is_empty()
        || fields.keys().any(|field| {
            !matches!(
                field.as_str(),
                "requests_per_minute"
                    | "usd_per_million_input"
                    | "usd_per_million_output"
                    | "profile"
            )
        })
    {
        return Err(refused(BUILT_IN));
    }
    setup(entry, fields, profile)
}

/// Parse the shared settings once; built-in transport overrides were already refused.
fn setup(
    entry: Named,
    fields: &Map<String, Value>,
    profile: Option<&str>,
) -> Result<Named, ConfigError> {
    let both_sides = match fields.get("both_sides") {
        None => false,
        Some(Value::Bool(value)) => *value,
        Some(_) => {
            return Err(refused(
                "configuration backend field `both_sides` must be true or false",
            ));
        }
    };
    let prices = match (
        fields.get("usd_per_million_input"),
        fields.get("usd_per_million_output"),
    ) {
        (None, None) => None,
        (Some(Value::String(input)), Some(Value::String(output))) => Prices::parse(input, output),
        _ => None,
    };
    if prices.is_none()
        && (fields.contains_key("usd_per_million_input")
            || fields.contains_key("usd_per_million_output"))
    {
        return Err(refused(
            "configuration backend fields `usd_per_million_input` and `usd_per_million_output` come together as decimal strings from 0 through 1000000 with at most six fractional digits",
        ));
    }
    let profile = profile
        .map(|text| {
            BackendProfile::parse(text).map_err(|error| ConfigError {
                message: format!("configuration backend field `profile` {error}").into(),
                unreadable: false,
                price: None,
            })
        })
        .transpose()?;
    Ok(entry
        .with_per_minute(fields.get("requests_per_minute").map(rate).transpose()?)
        .with_setup(prices, profile, both_sides))
}

/// A JSON whole number from 1 to 60,000, the variable's range.
fn rate(value: &Value) -> Result<NonZeroU32, ConfigError> {
    value
        .as_u64()
        .filter(|rate| (1..=u64::from(MAX_PER_MINUTE)).contains(rate))
        .and_then(|rate| u32::try_from(rate).ok())
        .and_then(NonZeroU32::new)
        .ok_or_else(|| refused(RATE))
}

/// Whether the text matches `[A-Z_][A-Z0-9_]*`, which refuses most pasted keys.
fn variable_name(text: &str) -> bool {
    let mut bytes = text.bytes();
    bytes
        .next()
        .is_some_and(|first| first.is_ascii_uppercase() || first == b'_')
        && bytes.all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
}

#[cfg(test)]
mod tests {
    use super::super::Config;

    #[test]
    fn each_backend_refusal_names_its_field_and_never_its_value() {
        let marker = "Sk_config_marker_0334";
        let entry = |body: &str| {
            format!(r#"{{"schema":"thinkthen.config/1","backends":{{"local-d1":{body}}}}}"#)
        };
        let good =
            r#"{"url":"http://127.0.0.1:8080/v1","key_env":"LOCAL_D1_KEY","model":"d1:free"}"#;
        let built_in = "a configuration entry for a built-in backend holds only `requests_per_minute`, `usd_per_million_input`, `usd_per_million_output`, and `profile`";
        let parsed = Config::parse(entry(good).as_bytes()).expect("a valid entry");
        assert_eq!(parsed.named().len(), 1);
        assert!(
            !format!("{parsed:?}").contains("8080"),
            "Debug withholds the base"
        );
        for (text, sentence) in [
            (
                r#"{"schema":"thinkthen.config/1","backends":{"liquid":{"url":"http://127.0.0.1/v1","key_env":"K","model":"m"}}}"#.to_owned(),
                "configuration entry for built-in backend `liquid` cannot set `url`; to use the built-in backend, remove `url`, `path`, `key_env`, `model`, and `both_sides` and delete the entry if it becomes empty; to keep custom routing, rename both the custom entry in `backends` and the selected `backend`",
            ),
            (
                format!(r#"{{"schema":"thinkthen.config/1","backends":{{"ollama":{{"requests_per_minute":60,"model":"{marker}"}}}}}}"#),
                "configuration entry for built-in backend `ollama` cannot set `model`; to use the built-in backend, remove `url`, `path`, `key_env`, `model`, and `both_sides` and delete the entry if it becomes empty; to keep custom routing, rename both the custom entry in `backends` and the selected `backend`",
            ),
            (
                r#"{"schema":"thinkthen.config/1","backends":{"typesafe":{}}}"#.to_owned(),
                built_in,
            ),
            (
                format!(r#"{{"schema":"thinkthen.config/1","backends":{{"typesafe":{{"url":"{marker}"}}}}}}"#),
                "configuration entry for built-in backend `typesafe` cannot set `url`; to use the built-in backend, remove `url`, `path`, `key_env`, `model`, and `both_sides` and delete the entry if it becomes empty; to keep custom routing, rename both the custom entry in `backends` and the selected `backend`",
            ),
            (
                r#"{"schema":"thinkthen.config/1","backends":{"liquid":600}}"#.to_owned(),
                built_in,
            ),
            (
                format!(r#"{{"schema":"thinkthen.config/1","backends":{{"{marker}":{good}}}}}"#),
                "configuration field `backends` holds a name that is not 1 to 32 lowercase letters, digits, and hyphens",
            ),
            (
                entry(&format!(r#"{{"url":"http://127.0.0.1/v1","key_env":"{marker}","model":"m"}}"#)),
                "configuration backend field `key_env` names an environment variable: a capital letter or underscore, then capital letters, digits, and underscores",
            ),
            (
                entry(r#"{"url":"http://127.0.0.1/v1","model":"m"}"#),
                "configuration backend entries need `url`, `key_env`, and `model`",
            ),
            (
                entry(&format!(r#"{{"url":"http://127.0.0.1/v1","key_env":"K","model":"m","key":"{marker}"}}"#)),
                "configuration backend entries hold only `url`, `path`, `key_env`, `model`, `requests_per_minute`, `both_sides`, `usd_per_million_input`, `usd_per_million_output`, and `profile`",
            ),
            (
                entry(r#"{"url":"http://127.0.0.1/v1","key_env":7,"model":"m"}"#),
                "configuration backend entries are objects whose `url`, `path`, `key_env`, and `model` are strings",
            ),
            (
                entry(&format!(r#"{{"url":"http://example.com/{marker}","key_env":"K","model":"m"}}"#)),
                "configuration backend field `url` must be a safe backend base",
            ),
            (
                entry(r#"{"url":"http://127.0.0.1/v1","key_env":"K","model":"  "}"#),
                "configuration backend field `model` must be a model name, not blank",
            ),
            (
                format!(r#"{{"schema":"thinkthen.config/1","backend":"{marker}"}}"#),
                "configuration field `backend` must name a built-in backend or an entry of `backends`",
            ),
            (
                r#"{"schema":"thinkthen.config/1","backend":"nowhere"}"#.to_owned(),
                "configuration field `backend` must name a built-in backend or an entry of `backends`",
            ),
            (
                r#"{"schema":"thinkthen.config/1","backend":7}"#.to_owned(),
                "configuration field `backend` must be a string",
            ),
            (
                r#"{"schema":"thinkthen.config/1","backends":[]}"#.to_owned(),
                "configuration field `backends` must be an object",
            ),
        ] {
            let refused = Config::parse(text.as_bytes()).expect_err(&text);
            assert_eq!(refused.message, sentence, "{text}");
            assert!(!refused.message.contains(marker));
        }
        for backend in ["liquid", "typesafe", "local-d1"] {
            let text = format!(
                r#"{{"schema":"thinkthen.config/1","backend":"{backend}","backends":{{"local-d1":{good}}}}}"#
            );
            assert!(Config::parse(text.as_bytes()).is_ok(), "{backend}");
        }
    }

    #[test]
    fn a_rate_is_a_whole_number_from_1_to_60000_on_any_entry_and_never_echoed() {
        let rate = "configuration backend field `requests_per_minute` must be a whole number from 1 to 60000";
        let added = |value: &str| {
            format!(
                r#"{{"schema":"thinkthen.config/1","backends":{{"local-d1":{{"url":"http://127.0.0.1/v1","key_env":"K","model":"m","requests_per_minute":{value}}}}}}}"#
            )
        };
        let built_in = |value: &str| {
            format!(
                r#"{{"schema":"thinkthen.config/1","backends":{{"liquid":{{"requests_per_minute":{value}}}}}}}"#
            )
        };
        for value in [
            "0",
            "60001",
            "-5",
            "1.5",
            "6e1",
            r#""600""#,
            "null",
            "true",
            "[600]",
            "4294967297",
            "\"Sk_rate_marker_0343\"",
        ] {
            for text in [added(value), built_in(value)] {
                let refused = Config::parse(text.as_bytes()).expect_err(&text);
                assert_eq!(refused.message, rate, "{text}");
                assert!(!refused.message.contains("marker"), "{text}");
            }
        }
        for (value, expected) in [("1", 1), ("600", 600), ("60000", 60_000)] {
            accepted(&added(value), expected);
            accepted(&built_in(value), expected);
        }
    }

    /// The only entry of `text` carries `rate`, and a built-in keeps its base,
    /// key variables, and model: the rate is the only change.
    fn accepted(text: &str, rate: u32) {
        use crate::core::{Named, named};
        let parsed = Config::parse(text.as_bytes()).expect(text);
        let [entry] = parsed.named() else {
            panic!("one entry: {text}");
        };
        let backend = |configured: &[Named]| {
            named::choose(&[(Some(entry.name()), None)], configured)
                .expect("a choice")
                .backend(None, "unused")
                .expect("a backend")
        };
        let chosen = backend(parsed.named());
        assert_eq!(chosen.per_minute().map(u32::from), Some(rate), "{text}");
        let Some(plain) = Named::built_in(entry.name()) else {
            return;
        };
        assert_eq!(entry.keys(), plain.keys());
        let plain = backend(&[]);
        assert_eq!(plain.per_minute(), None, "a built-in carries no rate");
        assert_eq!(chosen.with_per_minute(None), plain);
    }

    #[cfg(unix)]
    #[test]
    fn a_file_another_user_can_write_may_not_name_backends() {
        use std::os::unix::fs::PermissionsExt as _;

        let path =
            std::env::temp_dir().join(format!("thinkthen-shared-backends-{}", std::process::id()));
        for (text, refused) in [
            (
                r#"{"schema":"thinkthen.config/1","backends":{"x":{"url":"http://127.0.0.1/v1","key_env":"AWS_SECRET_ACCESS_KEY","model":"m"}}}"#,
                true,
            ),
            (
                r#"{"schema":"thinkthen.config/1","backend":"liquid"}"#,
                false,
            ),
        ] {
            std::fs::write(&path, text).expect("a configuration file");
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o646))
                .expect("other-write");
            let read = Config::read(Some(&path));
            assert_eq!(
                read.as_ref().err().map(|error| error.message.as_ref()),
                refused.then_some("the configuration file is writable by another user, so its `backends` are refused; keep it writable by its owner alone"),
                "{text}"
            );
            assert!(read.is_err() || read.is_ok_and(|config| config.shared()));
        }
        std::fs::remove_file(path).expect("the fixture leaves");
    }
}
