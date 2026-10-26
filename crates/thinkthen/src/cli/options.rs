//! Resolve explicit options before consulting environment values.
use crate::{Engine, Error, ErrorKind, Settings};
use clap::ArgMatches;
use serde_json::{Map, Value};

pub(super) fn usage(field: &str, expected: &str, received: &str) -> Error {
    Error::new(
        ErrorKind::Question,
        crate::core::diagnostic::requirement(field, expected, received),
    )
}
pub(super) fn unknown(field: &str, name: &str, accepted: &[&str]) -> Error {
    Error::new(
        ErrorKind::Question,
        crate::core::diagnostic::unknown(field, name, accepted),
    )
}
pub(super) fn unknown_value(field: &str, value: &str, accepted: &[&str]) -> Error {
    Error::new(
        ErrorKind::Question,
        crate::core::diagnostic::unknown_value(field, value, accepted),
    )
}
pub(super) fn explicit(matches: &ArgMatches, key: &str) -> Option<String> {
    matches.try_get_one::<String>(key).ok().flatten().cloned()
}
pub(super) fn variable(key: &str) -> String {
    format!("THINKTHEN_{}", key.replace('-', "_").to_uppercase())
}
pub(super) fn value(matches: &ArgMatches, key: &str) -> Option<String> {
    explicit(matches, key).or_else(|| std::env::var(variable(key)).ok())
}
pub(super) fn boolean(key: &str, value: Option<String>) -> Result<Option<bool>, Error> {
    value
        .map(|v| match v.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(usage(key, "true or false", "a string (contents redacted)")),
        })
        .transpose()
}
pub(super) fn switch(matches: &ArgMatches, key: &str) -> Result<bool, Error> {
    Ok(boolean(key, value(matches, key))?.unwrap_or(false))
}
pub(super) fn group(matches: &ArgMatches, keys: &[&str]) -> Vec<Option<String>> {
    let explicit_group = keys.iter().any(|key| explicit(matches, key).is_some());
    keys.iter()
        .map(|key| {
            if explicit_group {
                explicit(matches, key)
            } else {
                std::env::var(variable(key)).ok()
            }
        })
        .collect()
}
pub(super) fn list(matches: &ArgMatches, key: &str, env: &str) -> Result<Option<Value>, Error> {
    if let Some(values) = matches.try_get_many::<String>(key).ok().flatten() {
        return Ok(Some(Value::Array(
            values.cloned().map(Value::String).collect(),
        )));
    }
    std::env::var(env)
        .ok()
        .map(|text| {
            serde_json::from_str(&text)
                .map_err(|_| usage(key, "a JSON array", "invalid JSON (contents redacted)"))
        })
        .transpose()
}
pub(super) fn number(key: &str, text: &str) -> Result<Value, Error> {
    text.parse::<serde_json::Number>()
        .map(Value::Number)
        .map_err(|_| {
            Error::new(
                ErrorKind::Question,
                crate::core::diagnostic::expected(key, "a number", &Value::String(text.into())),
            )
        })
}

#[derive(Debug)]
pub(super) struct Output {
    pub json: bool,
    pub quiet: bool,
    pub verbose: bool,
    pub check: bool,
}
impl Output {
    pub(super) fn resolve(matches: &ArgMatches) -> Result<Self, Error> {
        let selectors = group(matches, &["format", "json"]);
        let mut values = selectors.into_iter();
        let format = values
            .next()
            .flatten()
            .map(|v| match v.as_str() {
                "json" => Ok(true),
                "text" => Ok(false),
                _ => Err(unknown_value("format", &v, &["text", "json"])),
            })
            .transpose()?;
        let json = boolean("json", values.next().flatten())?;
        if format.zip(json).is_some_and(|(a, b)| a != b) {
            return Err(usage(
                "format",
                "agreement with json",
                "conflicting output formats",
            ));
        }
        let mut verbosity = group(matches, &["quiet", "verbose"]).into_iter();
        let quiet = boolean("quiet", verbosity.next().flatten())?.unwrap_or(false);
        let verbose = boolean("verbose", verbosity.next().flatten())?.unwrap_or(false);
        if quiet && verbose {
            return Err(usage("quiet", "false when verbose is true", "true"));
        }
        let check = if matches.try_get_one::<String>("check").is_ok() {
            switch(matches, "check")?
        } else {
            false
        };
        // Color affects diagnostics only; rows are always uncolored.
        let color = explicit(matches, "color")
            .or_else(|| std::env::var_os("NO_COLOR").map(|_| "never".into()))
            .or_else(|| std::env::var("THINKTHEN_COLOR").ok());
        if let Some(color) = color
            && !matches!(color.as_str(), "auto" | "always" | "never")
        {
            return Err(unknown_value("color", &color, &["auto", "always", "never"]));
        }
        Ok(Self {
            json: json.or(format).unwrap_or(false),
            quiet,
            verbose,
            check,
        })
    }
}

pub(super) fn client(matches: &ArgMatches) -> Result<Engine, Error> {
    let mut settings = Map::new();
    for key in ["backend", "model", "url", "batch"] {
        if let Some(text) = value(matches, key) {
            settings.insert(
                key.into(),
                if key == "batch" {
                    number(key, &text)?
                } else {
                    text.into()
                },
            );
        }
    }
    Settings::from_json(&Value::Object(settings).to_string())?.engine(|key| {
        // The command's URL option is resolved above; the retired variable is not a fallback.
        (key != "THINKTHEN_BASE_URL")
            .then(|| std::env::var(key).ok())
            .flatten()
    })
}

/// Select diagnostic styling before Clap renders syntax errors or help.
pub(super) fn diagnostic_color() -> clap::ColorChoice {
    let arguments: Vec<_> = std::env::args_os()
        .filter_map(|arg| arg.into_string().ok())
        .take_while(|arg| arg != "--")
        .collect();
    let explicit = arguments
        .windows(2)
        .find(|pair| pair.first().is_some_and(|arg| arg == "--color"))
        .and_then(|pair| pair.get(1))
        .cloned()
        .or_else(|| {
            arguments
                .iter()
                .find_map(|arg| arg.strip_prefix("--color=").map(str::to_owned))
        });
    let color = explicit
        .or_else(|| std::env::var_os("NO_COLOR").map(|_| "never".into()))
        .or_else(|| std::env::var("THINKTHEN_COLOR").ok());
    match color.as_deref() {
        Some("always") => clap::ColorChoice::Always,
        Some("never") => clap::ColorChoice::Never,
        _ => clap::ColorChoice::Auto,
    }
}
