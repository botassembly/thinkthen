//! Shared JSON admission diagnostics retain field context without exposing payloads.
use serde::{Deserialize, Deserializer};

// Strings may carry credentials even in an unexpected field. Describe their type;
// never render their contents. Container values likewise stay out of diagnostics.
fn received(value: &str) -> String {
    if value.starts_with("string ") || value.starts_with("character ") {
        "a string (contents redacted)".into()
    } else if value.starts_with("integer ")
        || value.starts_with("floating point ")
        || value.starts_with("boolean ")
    {
        value.chars().take(64).collect()
    } else {
        match value {
            "sequence" => "an array",
            "map" => "an object",
            "null" | "unit value" => "null",
            _ => "an incompatible value (contents redacted)",
        }
        .into()
    }
}
fn contextual(error: &str) -> bool {
    error.split_once(": expected ").is_some_and(|(path, _)| {
        // Our field helpers start with an identifier or a map-entry index.
        // Serde's raw errors start with phrases such as "unknown field `...`".
        // The rest of an admitted diagnostic path may contain a caller's name.
        let head = path.split(['.', '[']).next().unwrap_or_default();
        (!head.is_empty()
            && head
                .chars()
                .all(|character| character.is_ascii_alphanumeric() || character == '_'))
            || path
                .strip_prefix('[')
                .and_then(|path| path.split_once(']'))
                .is_some_and(|(index, _)| {
                    !index.is_empty() && index.bytes().all(|byte| byte.is_ascii_digit())
                })
    })
}
fn preview_name(value: &str) -> String {
    let mut preview: String = value.chars().take(48).collect();
    if value.chars().count() > 48 {
        preview.push('…');
    }
    preview
}
fn distance(left: &str, right: &str) -> usize {
    let mut previous: Vec<usize> = (0..=right.chars().count()).collect();
    for (row, a) in left.chars().take(48).enumerate() {
        let mut current = vec![row + 1];
        for (column, b) in right.chars().enumerate() {
            let cost = usize::from(a != b);
            let deletion = previous.get(column + 1).copied().unwrap_or(usize::MAX) + 1;
            let insertion = current.last().copied().unwrap_or(0) + 1;
            let substitution = previous.get(column).copied().unwrap_or(0) + cost;
            current.push(deletion.min(insertion).min(substitution));
        }
        previous = current;
    }
    previous.last().copied().unwrap_or(0)
}

/// Format a field requirement with a trusted, fixed description of the received input.
pub(crate) fn requirement(field: &str, expected: &str, received: &str) -> String {
    format!("{field}: expected {expected}; received {received}")
}

/// Describe JSON input without disclosing strings or container contents.
#[allow(
    clippy::disallowed_types,
    reason = "diagnostics describe JSON types without implementing admission"
)]
pub(crate) fn expected(field: &str, expected: &str, value: &serde_json::Value) -> String {
    let description = match value {
        serde_json::Value::Null => "null".into(),
        serde_json::Value::Bool(value) => format!("boolean {value}"),
        serde_json::Value::Number(value) => format!("number {value}"),
        serde_json::Value::String(_) => "a string (contents redacted)".into(),
        serde_json::Value::Array(_) => "an array".into(),
        serde_json::Value::Object(_) => "an object".into(),
    };
    requirement(field, expected, &description)
}

/// Suggest a canonical name without disclosing a caller's string value.
pub(crate) fn unknown_value(field: &str, value: &str, accepted: &[&str]) -> String {
    let choices = accepted
        .iter()
        .map(|name| format!("`{name}`"))
        .collect::<Vec<_>>()
        .join(", ");
    let suggestion = suggestion(value, accepted);
    requirement(
        field,
        &format!("one of {choices}"),
        &format!("an unknown string (contents redacted){suggestion}"),
    )
}

fn suggestion(name: &str, accepted: &[&str]) -> String {
    accepted
        .iter()
        .min_by_key(|candidate| distance(name, candidate))
        .map_or_else(String::new, |candidate| {
            format!("; did you mean `{candidate}`?")
        })
}

/// Suggest the closest canonical field name while bounding the unknown name.
pub(crate) fn unknown(field: &str, name: &str, accepted: &[&str]) -> String {
    let choices = accepted
        .iter()
        .map(|name| format!("`{name}`"))
        .collect::<Vec<_>>()
        .join(", ");
    let suggestion = suggestion(name, accepted);
    requirement(
        field,
        &format!("one of {choices}"),
        &format!("unknown field `{}`{suggestion}", preview_name(name)),
    )
}

/// Context comes from the field's canonical serde declaration, never a host copy.
pub(crate) fn context(path: &str, error: &str) -> String {
    let error = error.split(" at line ").next().unwrap_or(error);
    if contextual(error) {
        return format!("{path}.{error}");
    }
    if let Some((value, expected)) = error
        .strip_prefix("invalid type: ")
        .or_else(|| error.strip_prefix("invalid value: "))
        .and_then(|error| error.rsplit_once(", expected "))
    {
        return format!("{path}: expected {expected}; received {}", received(value));
    }
    if let Some((field, accepted)) = error
        .strip_prefix("unknown field `")
        .and_then(|error| error.rsplit_once("`, expected "))
    {
        let accepted_names: Vec<&str> = accepted
            .split('`')
            .enumerate()
            .filter_map(|(index, value)| (index % 2 == 1).then_some(value))
            .collect();
        return unknown(
            &format!("{path}.{}", preview_name(field)),
            field,
            &accepted_names,
        );
    }
    for (prefix, got) in [
        ("missing field `", "absent field"),
        ("duplicate field `", "repeated field"),
    ] {
        if let Some(field) = error
            .strip_prefix(prefix)
            .and_then(|value| value.strip_suffix('`'))
        {
            return format!(
                "{path}.{}: expected one field value; received {got}",
                preview_name(field)
            );
        }
    }
    if let Some((_, expected)) = error
        .strip_prefix("unknown variant `")
        .and_then(|error| error.rsplit_once("`, expected "))
    {
        return format!(
            "{path}: expected {expected}; received an unknown string (contents redacted)"
        );
    }
    // Syntax errors and custom serde errors must not reproduce any caller payload.
    format!("{path}: expected valid JSON matching the contract; received invalid input")
}

macro_rules! fields {
    ($($field:ident),+ $(,)?) => {$(
        pub(crate) fn $field<'de, D: Deserializer<'de>, T: Deserialize<'de>>(deserializer: D) -> Result<T, D::Error> {
            T::deserialize(deserializer).map_err(|error| serde::de::Error::custom(context(stringify!($field), &error.to_string())))
        }
    )+};
}
fields!(
    question,
    questions,
    entries,
    records,
    backend,
    model,
    url,
    batch,
    retry,
    max_retries,
    initial_backoff_ms,
    attempt_timeout_ms,
    top,
    threshold,
    relation,
    source,
    target,
    reads,
    either,
    name,
    evidence
);

// The element visitor preserves the failing index even under internally tagged
// enums, where serde buffers content before the outer deserializer sees errors.
fn sequence<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
    field: &str,
    expected: &str,
) -> Result<Vec<T>, D::Error> {
    struct Sequence<'a, T>(&'a str, &'a str, std::marker::PhantomData<T>);
    impl<'de, T: Deserialize<'de>> serde::de::Visitor<'de> for Sequence<'_, T> {
        type Value = Vec<T>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(self.1)
        }
        fn visit_seq<A: serde::de::SeqAccess<'de>>(
            self,
            mut sequence: A,
        ) -> Result<Self::Value, A::Error> {
            let mut values = Vec::new();
            loop {
                match sequence.next_element::<T>().map_err(|error| {
                    <A::Error as serde::de::Error>::custom(context(
                        &format!("{}[{}]", self.0, values.len()),
                        &error.to_string(),
                    ))
                })? {
                    Some(value) => values.push(value),
                    None => return Ok(values),
                }
            }
        }
    }
    deserializer
        .deserialize_seq(Sequence(field, expected, std::marker::PhantomData))
        .map_err(|error| {
            let error = error.to_string();
            serde::de::Error::custom(if contextual(&error) {
                error
            } else {
                context(field, &error)
            })
        })
}
macro_rules! string_fields {
    ($($field:ident),+ $(,)?) => {$(
        pub(crate) fn $field<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<String>, D::Error> {
            sequence(deserializer, stringify!($field), "an array of strings")
        }
    )+};
}
string_fields!(choices, labels, levels, kinds);

pub(crate) fn rules<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Vec<T>, D::Error> {
    sequence(deserializer, "rules", "an array of relation rules")
}

// Even a numeric credential must remain secret when its host shape is wrong.
pub(crate) fn api_key<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<T, D::Error> {
    T::deserialize(deserializer).map_err(|error| {
        let message = context("api_key", &error.to_string());
        let expected = message
            .rsplit_once("; received ")
            .map_or(message.as_str(), |(expected, _)| expected);
        serde::de::Error::custom(format!(
            "{expected}; received a credential value (contents redacted)"
        ))
    })
}
