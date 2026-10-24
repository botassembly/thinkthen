//! The question shorthand shared by every SQL surface.

use std::collections::BTreeMap;

use serde_json::{Value, json};
use thinkthen::{BackendTable, Client, Error, ErrorKind, SdkRequest, SdkSettings};

/// Admit SQL arguments through the shared request contract.
/// Plain question text is inline; text beginning with an object is tagged JSON.
/// Missing options use the contract defaults. No file or environment is read.
///
/// # Errors
/// Returns the library's request admission error.
pub fn request(function: &str, question: &str, options: Option<&str>) -> Result<SdkRequest, Error> {
    let question = if question.trim_start().starts_with('{') {
        parse(question)?;
        question.to_owned()
    } else {
        json!({"type":"inline", "value":question}).to_string()
    };
    if let Some(options) = options {
        parse(options)?;
    }
    let value = format!(
        r#"{{"function":{},"question":{},"options":{}}}"#,
        json!(function),
        question,
        options.unwrap_or("{}")
    );
    SdkRequest::from_json(&value)
}

fn parse(text: &str) -> Result<Value, Error> {
    serde_json::from_str(text).map_err(|error| Error::new(ErrorKind::Question, error.to_string()))
}

/// Admit database settings before resolving any process-owned backend source.
/// SQL callers select a backend name, never its address or key variable.
///
/// # Errors
/// Returns a usage error for address/key-source fields, including JSON null,
/// or the existing client admission error for other invalid settings.
pub fn client(settings: &str, variable: impl Fn(&str) -> Option<String>) -> Result<Client, Error> {
    Client::from_settings(&settings_admission(settings)?, variable)
}

fn settings_admission(settings: &str) -> Result<SdkSettings, Error> {
    let value: Value = serde_json::from_str(settings)
        .map_err(|error| Error::new(ErrorKind::Settings, error.to_string()))?;
    for field in ["url", "key_source"] {
        if value
            .as_object()
            .is_some_and(|fields| fields.contains_key(field))
        {
            return Err(Error::new(
                ErrorKind::Settings,
                format!("SQL settings do not accept field `{field}`"),
            ));
        }
    }
    SdkSettings::from_json(settings)
}

/// Admit database backend/model policy before any client or credential resolution.
/// An absent policy admits only the default selection; an empty map admits none.
///
/// # Errors
/// Returns a usage error for invalid settings, policy or a disallowed selection.
pub fn client_with_policy(
    settings: &str,
    allowed: Option<&str>,
    variable: impl Fn(&str) -> Option<String>,
) -> Result<Client, Error> {
    let settings = settings_admission(settings)?;
    let table = BackendTable::default();
    let selection = settings.backend_selection(&table)?;
    let permitted = if let Some(allowed) = allowed {
        let allowed: BTreeMap<String, Vec<String>> = serde_json::from_str(allowed)
            .map_err(|error| Error::new(ErrorKind::SqlPolicy, error.to_string()))?;
        allowed
            .get(selection.0)
            .is_some_and(|models| models.iter().any(|model| model == selection.1))
    } else {
        selection == SdkSettings::default().backend_selection(&table)?
    };
    if !permitted {
        return Err(Error::new(
            ErrorKind::SqlSelection,
            "the backend and model are not allowed by the database administrator",
        ));
    }
    Client::from_settings(&settings, variable)
}
