//! The configuration file's `backend` and `backends` fields (ADR 0114 section 2).
//!
//! Each entry names a base, the name of the variable that holds its key, and a
//! model. The file never holds a key, and no refusal repeats a value.

use serde_json::{Map, Value};

use super::{ConfigError, refused};
use crate::core::{Backend, DEFAULT_MODEL, ModelName, Named, named};

const REUSE: &str = "configuration backend names a built-in backend; choose another name";
const NAME: &str = "configuration field `backends` holds a name that is not 1 to 32 lowercase letters, digits, and hyphens";
const EXTRA: &str = "configuration backend entries hold only `url`, `key_env`, and `model`";
const MISSING: &str = "configuration backend entries need `url`, `key_env`, and `model`";
const STRINGS: &str =
    "configuration backend entries are objects whose `url`, `key_env`, and `model` are strings";
const URL: &str = "configuration backend field `url` must be a safe backend base";
const KEY_ENV: &str = "configuration backend field `key_env` names an environment variable: a capital letter or underscore, then capital letters, digits, and underscores";
const MODEL: &str = "configuration backend field `model` must be a model name, not blank";
const BACKEND: &str =
    "configuration field `backend` must name a built-in backend or an entry of `backends`";

/// Check every entry, in name order, and the `backend` field against them.
pub(super) fn read(
    backend: Option<&str>,
    entries: Option<&Map<String, Value>>,
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

fn entry(name: &str, value: &Value) -> Result<Named, ConfigError> {
    if named::is_built_in(name) {
        return Err(refused(REUSE));
    }
    if !named::valid_name(name) {
        return Err(refused(NAME));
    }
    let Value::Object(fields) = value else {
        return Err(refused(STRINGS));
    };
    if fields
        .keys()
        .any(|field| !matches!(field.as_str(), "url" | "key_env" | "model"))
    {
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
    Ok(Named::new(name, url, key_env, model))
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
        let parsed = Config::parse(entry(good).as_bytes()).expect("a valid entry");
        assert_eq!(parsed.named().len(), 1);
        assert!(
            !format!("{parsed:?}").contains("8080"),
            "Debug withholds the base"
        );
        for (text, sentence) in [
            (
                r#"{"schema":"thinkthen.config/1","backends":{"liquid":{"url":"http://127.0.0.1/v1","key_env":"K","model":"m"}}}"#.to_owned(),
                "configuration backend names a built-in backend; choose another name",
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
                "configuration backend entries hold only `url`, `key_env`, and `model`",
            ),
            (
                entry(r#"{"url":"http://127.0.0.1/v1","key_env":7,"model":"m"}"#),
                "configuration backend entries are objects whose `url`, `key_env`, and `model` are strings",
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
                read.as_ref().err().map(|error| error.message),
                refused.then_some("the configuration file is writable by another user, so its `backends` are refused; keep it writable by its owner alone"),
                "{text}"
            );
            assert!(read.is_err() || read.is_ok_and(|config| config.shared()));
        }
        std::fs::remove_file(path).expect("the fixture leaves");
    }
}
