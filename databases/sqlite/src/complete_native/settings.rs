//! Portable caller fields merge through native Settings without reordering authorship.
use super::{Prepared, usage};
use serde_json::value::RawValue;
use std::collections::BTreeMap;
use thinkthen::{Error, For, Settings};
pub(crate) fn prepare(verb: &str, source: &str, settings: &Settings) -> Result<Prepared, Error> {
    if source.starts_with('@') {
        let parsed = Prepared::parse(verb, source)?;
        let encoded = match &parsed {
            Prepared::Atomic(q) => Some(q.to_json()?),
            Prepared::Filter(q) => Some(q.to_json()?),
            _ => None,
        };
        return match encoded {
            Some(source) => prepare(verb, &source, settings),
            None => parsed.configured(verb, settings),
        };
    }
    let role = match verb {
        "decide" | "filter" => Some(For::Decide),
        "choose" => Some(For::Choose),
        "tag" => Some(For::Tag),
        "score" => Some(For::Score),
        _ => None,
    };
    let merged = if let Some(role) = role {
        merge(source, settings, role)?
    } else {
        source.to_owned()
    };
    Prepared::parse(verb, &merged)?.configured(verb, settings)
}
fn merge(source: &str, settings: &Settings, role: For) -> Result<String, Error> {
    if !source.trim_start().starts_with('{') {
        return settings
            .question_json(role, source)
            .map_err(|e| usage(&e.to_string()));
    }
    let fields: BTreeMap<String, Box<RawValue>> =
        serde_json::from_str(source).map_err(|_| usage("the question is one JSON object"))?;
    let keys = fields.keys().map(String::as_str).collect::<Vec<_>>();
    settings
        .conflicts(&keys, false)
        .map_err(|e| usage(&e.to_string()))?;
    let extra = settings
        .question_json(role, "settings merge")
        .map_err(|e| usage(&e.to_string()))?;
    let Some(first) = extra.find(',') else {
        return Ok(source.to_owned());
    };
    let suffix = extra
        .get(first + 1..extra.len() - 1)
        .ok_or_else(super::defect)?;
    let original = source
        .trim_end()
        .strip_suffix('}')
        .ok_or_else(|| usage("the question is one JSON object"))?;
    Ok(format!(
        "{original}{}{suffix}}}",
        if fields.is_empty() { "" } else { "," }
    ))
}

#[allow(
    dead_code,
    reason = "authorized file grammar used by PostgreSQL and DuckDB"
)]
pub(crate) fn parse_file(
    verb: &str,
    source: &str,
    reference: &thinkthen::QuestionFileReference,
) -> Result<Prepared, Error> {
    use thinkthen::QuestionFileRole as Role;
    let role = match verb {
        "annotate" => Role::Set,
        "rank" => {
            let fields: BTreeMap<String, Box<RawValue>> =
                serde_json::from_str(source).unwrap_or_default();
            if fields.contains_key("questions") {
                Role::Set
            } else {
                Role::Rank
            }
        }
        "find" => Role::Find,
        "recognize" => Role::Recognize,
        "relate" => Role::Relate,
        "choose" => Role::Choose,
        _ => Role::Atomic,
    };
    reference.parse(source, role, |text| Prepared::content(verb, text))
}
#[allow(
    dead_code,
    reason = "authorized file settings used by PostgreSQL and DuckDB"
)]
pub(crate) fn prepare_file(
    verb: &str,
    source: &str,
    settings: &Settings,
    parsed: Prepared,
) -> Result<Prepared, Error> {
    let for_role = match verb {
        "decide" | "filter" => Some(For::Decide),
        "choose" => Some(For::Choose),
        "tag" => Some(For::Tag),
        "score" => Some(For::Score),
        _ => None,
    };
    // Caller fields are a separate Usage boundary after the saved-file grammar.
    let merged = for_role
        .map(|role| merge(source, settings, role))
        .transpose()?;
    match merged {
        Some(merged) if merged != source => {
            Prepared::content(verb, &merged)?.configured(verb, settings)
        }
        _ => parsed.configured(verb, settings),
    }
}
