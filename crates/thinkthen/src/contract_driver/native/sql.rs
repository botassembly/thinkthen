//! SQL admission rejects caller credentials before resolving process settings.
use crate::{BackendTable, Code, Engine, Error, ErrorKind, Question, Request, Settings};

#[allow(
    clippy::disallowed_types,
    reason = "SQL checks forbidden field presence before typed admission"
)]
fn admit(text: &str) -> Result<Settings, Error> {
    let fields: serde_json::Value = serde_json::from_str(text).map_err(|error| {
        Error::from_contract(
            Code::UsageSettings,
            crate::core::diagnostic::context("settings", &error.to_string()),
        )
    })?;
    for field in ["url", "api_key", "key_source"] {
        if fields
            .as_object()
            .is_some_and(|fields| fields.contains_key(field))
        {
            return Err(Error::from_contract(
                Code::UsageSettings,
                crate::core::diagnostic::requirement(
                    &format!("settings.{field}"),
                    "settings selected by backend name",
                    "a forbidden SQL credential or address field",
                ),
            ));
        }
    }
    Settings::from_json(text)
}

/// Admit database settings without an administrator selection restriction.
/// # Errors
/// Rejects credentials and addresses before reading the selected environment.
pub fn sql_engine_unrestricted(
    text: &str,
    variable: impl Fn(&str) -> Option<String>,
) -> Result<Engine, Error> {
    admit(text)?.engine(variable)
}
/// Admit PostgreSQL settings under the administrator's backend/model policy.
/// # Errors
/// Rejects forbidden fields and denied selections before resolving credentials.
pub fn sql_engine(
    text: &str,
    allowed: Option<&str>,
    variable: impl Fn(&str) -> Option<String>,
) -> Result<Engine, Error> {
    let settings = admit(text)?;
    let table = BackendTable::default();
    let (backend, model) = settings.selection(&table)?;
    let permitted = if let Some(allowed) = allowed {
        let mut permitted = false;
        for entry in allowed
            .split(',')
            .map(str::trim)
            .filter(|entry| !entry.is_empty())
        {
            let (name, selected_model) = entry.split_once(':').ok_or_else(|| {
                Error::from_contract(
                    Code::UsageSqlPolicy,
                    crate::core::diagnostic::requirement(
                        "policy",
                        "backend:model entries",
                        "an invalid policy entry (contents redacted)",
                    ),
                )
            })?;
            if serde_json::from_value::<crate::BackendName>(serde_json::json!(name)).is_err()
                || selected_model.is_empty()
                || selected_model.contains(':')
                || (selected_model.contains('*') && selected_model != "*")
            {
                return Err(Error::from_contract(
                    Code::UsageSqlPolicy,
                    crate::core::diagnostic::requirement(
                        "policy",
                        "a supported backend and a nonempty model or wildcard",
                        "an invalid backend or model (contents redacted)",
                    ),
                ));
            }
            permitted |= name == backend && (selected_model == model || selected_model == "*");
        }
        permitted
    } else {
        (backend, model) == Settings::default().selection(&table)?
    };
    if !permitted {
        return Err(Error::from_contract(
            Code::UsageSqlSelection,
            crate::core::diagnostic::requirement(
                "settings",
                "a backend and model allowed by the database administrator",
                "a disallowed selection (contents redacted)",
            ),
        ));
    }
    settings.engine(variable)
}

/// Admit literal question text with canonical per-function options.
/// No reference, proxy, file or environment resolution occurs here.
/// # Errors
/// Returns the shared per-function question or option error.
pub fn sql_request(
    function: &str,
    question: &str,
    options: Option<&str>,
) -> Result<Request, Error> {
    let options = options.unwrap_or("{}").trim();
    let members = options
        .strip_prefix('{')
        .and_then(|text| text.strip_suffix('}'))
        .ok_or_else(|| {
            Error::new(
                ErrorKind::Question,
                crate::core::diagnostic::requirement(
                    "options",
                    "a JSON object",
                    "an incompatible value (contents redacted)",
                ),
            )
        })?;
    let text = match function {
        "recognize" | "relate" | "annotate" if question.is_empty() => String::new(),
        _ => format!(",\"question\":{}", serde_json::json!(question)),
    };
    let separator = if members.trim().is_empty() { "" } else { "," };
    let question = Question::from_json(&format!(
        "{{\"function\":{}{text}{separator}{members}}}",
        serde_json::json!(function)
    ))?;
    if serde_json::json!(question.function()) != function {
        return Err(Error::new(
            ErrorKind::Question,
            crate::core::diagnostic::requirement(
                "question.function",
                "the selected SQL function",
                "a different function",
            ),
        ));
    }
    Ok(Request::new(question))
}

/// Convert an unrepresentable PostgreSQL text or JSON value into a record failure.
/// The original row keeps its position, attempted source and representable identifier.
/// # Errors
/// Returns an internal error only if typed row serialization cannot round-trip.
pub fn sql_postgres_row(row: crate::results::AnyRow) -> Result<crate::results::AnyRow, Error> {
    let mut value = serde_json::to_value(&row).map_err(|_| super::internal_error())?;
    if !zero(&value) {
        return Ok(row);
    }
    let fields = value.as_object_mut().ok_or_else(super::internal_error)?;
    if fields.get("id").is_some_and(zero) {
        fields.remove("id");
    }
    fields.remove("answer");
    fields.remove("detail");
    fields.remove("probability");
    fields.insert("status".into(), serde_json::json!("failed"));
    fields.insert(
        "error".into(),
        serde_json::to_value(representation_error()).map_err(|_| super::internal_error())?,
    );
    serde_json::from_value(value).map_err(|_| super::internal_error())
}

/// Set SQL's optional threshold parameters through one admission boundary.
/// # Errors
/// Rejects mixed cuts and bands, missing bounds and invalid function thresholds.
pub fn sql_threshold(
    question: &mut super::QuestionBuilder,
    cut: Option<f64>,
    low: Option<f64>,
    high: Option<f64>,
) -> Result<(), Error> {
    match (cut, low, high) {
        (None, None, None) => Ok(()),
        (Some(cut), None, None) => question.set_cut(cut),
        (None, Some(low), Some(high)) => question.set_band(low, high),
        _ => Err(Error::from_contract(
            Code::UsageThreshold,
            crate::core::diagnostic::requirement(
                "threshold",
                "a cut or both low and high bounds",
                "mixed cuts and bounds or an incomplete band",
            ),
        )),
    }
}

fn zero(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => text.contains('\0'),
        serde_json::Value::Array(values) => values.iter().any(zero),
        serde_json::Value::Object(values) => values
            .iter()
            .any(|(key, value)| key.contains('\0') || zero(value)),
        _ => false,
    }
}

fn representation_error() -> Error {
    Error::from_contract(
        Code::RecordJson,
        "the record result cannot be represented by PostgreSQL text or jsonb",
    )
}
/// Check an explicitly selected PostgreSQL identifier before native allocation.
/// # Errors
/// Rejects a host's invalid JSON as an internal conversion error.
pub fn sql_postgres_id_error(id: &str) -> Result<Option<Error>, Error> {
    let value: &serde_json::value::RawValue =
        serde_json::from_str(id).map_err(|_| super::internal_error())?;
    Ok(raw_zero(value).then(representation_error))
}

// Inspect strings and keys without ever interpreting a JSON number.
fn raw_zero(value: &serde_json::value::RawValue) -> bool {
    let json = value.get();
    if let Ok(text) = serde_json::from_str::<String>(json) {
        return text.contains('\0');
    }
    if let Ok(items) = serde_json::from_str::<Vec<&serde_json::value::RawValue>>(json) {
        for item in items {
            if raw_zero(item) {
                return true;
            }
        }
    } else {
        struct Fields;
        impl<'de> serde::de::Visitor<'de> for Fields {
            type Value = bool;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a JSON object")
            }
            fn visit_map<M: serde::de::MapAccess<'de>>(self, mut map: M) -> Result<bool, M::Error> {
                let mut found = false;
                while let Some((key, item)) =
                    map.next_entry::<String, &serde_json::value::RawValue>()?
                {
                    found |= key.contains('\0') || raw_zero(item);
                }
                Ok(found)
            }
        }
        return serde::Deserializer::deserialize_map(
            &mut serde_json::Deserializer::from_str(json),
            Fields,
        )
        .unwrap_or(false);
    }
    false
}

/// Admit PostgreSQL JSON data that its native jsonb result cannot represent.
/// Missing records keep their shared missing semantics and require no request.
#[must_use]
pub fn sql_postgres_record(json: &str) -> crate::Record {
    let record = crate::Record::from_json(json);
    let Ok(value) = serde_json::from_str::<serde_json::Value>(json) else {
        return record;
    };
    if !zero(&value) {
        return record;
    }
    let error = representation_error();
    let failure = crate::Record::failed(error.code(), error.message());
    match record.id() {
        Some(id) if !matches!(id, crate::RecordId::String(text) if text.contains('\0')) => {
            failure.with_id(id.clone())
        }
        _ => failure,
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct SqlRule {
    #[serde(rename(serialize = "relation"))]
    #[serde(deserialize_with = "crate::core::diagnostic::name")]
    name: String,
    #[serde(deserialize_with = "crate::core::diagnostic::source")]
    source: String,
    #[serde(deserialize_with = "crate::core::diagnostic::target")]
    target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::core::diagnostic::reads")]
    reads: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::core::diagnostic::either")]
    either: Option<bool>,
}
fn sql_question_value(value: &mut serde_json::Value) -> Result<(), Error> {
    if value
        .get("function")
        .is_some_and(|function| function == "relate")
    {
        if let Some(rules) = value.get_mut("rules") {
            let source: Vec<SqlRule> =
                crate::core::diagnostic::rules(rules.clone()).map_err(|error| {
                    Error::from_contract(
                        Code::UsageQuestion,
                        crate::core::diagnostic::context("question", &error.to_string()),
                    )
                })?;
            *rules = serde_json::to_value(source).map_err(|_| super::internal_error())?;
        }
    } else if value
        .get("function")
        .is_some_and(|function| function == "annotate")
        && let Some(entries) = value
            .get_mut("questions")
            .and_then(serde_json::Value::as_object_mut)
    {
        for entry in entries.values_mut() {
            sql_question_value(entry)?;
        }
    }
    Ok(())
}
/// Admit SQL's structured question values, translating the public rule name once.
/// # Errors
/// Returns shared question validation errors, including unknown rule fields.
pub fn sql_question(json: &str) -> Result<Question, Error> {
    let mut value: serde_json::Value = serde_json::from_str(json).map_err(|error| {
        Error::from_contract(
            Code::UsageQuestion,
            crate::core::diagnostic::context("question", &error.to_string()),
        )
    })?;
    if !value
        .get("function")
        .is_some_and(|function| function == "relate" || function == "annotate")
    {
        return Question::from_json(json);
    }
    sql_question_value(&mut value)?;
    Question::from_json(&value.to_string())
}
