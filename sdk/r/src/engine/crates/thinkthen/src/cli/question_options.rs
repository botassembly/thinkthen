//! Translate command answer descriptions into the shared question contract.
use super::options::{usage, value};
use crate::Error;
use clap::ArgMatches;
use serde_json::{Map, Value, json};

pub(super) fn apply(
    function: &str,
    args: &ArgMatches,
    question: &mut Map<String, Value>,
) -> Result<(), Error> {
    if matches!(function, "decide" | "filter") {
        for (key, wire) in [("yes-means", "yes_means"), ("no-means", "no_means")] {
            if let Some(meaning) = value(args, key) {
                question.insert(wire.into(), meaning.into());
            }
        }
    }
    let Some(descriptions) = args.try_get_many::<String>("describe").ok().flatten() else {
        return Ok(());
    };
    let field = match function {
        "choose" => "choices",
        "score" | "rank" => "levels",
        _ => "labels",
    };
    let labels = question
        .get_mut(field)
        .and_then(Value::as_array_mut)
        .ok_or_else(|| usage("describe", "a label list", "absent list"))?;
    for description in descriptions {
        // Match the longest admitted label, because a label may itself contain '='.
        let selected = labels
            .iter()
            .enumerate()
            .filter_map(|(index, label)| {
                let name = label
                    .as_str()
                    .or_else(|| label.get("name").and_then(Value::as_str))?;
                let text = description.strip_prefix(name)?.strip_prefix('=')?;
                Some((index, name.to_owned(), text, name.len()))
            })
            .max_by_key(|(_, _, _, length)| *length);
        let Some((index, name, text, _)) = selected else {
            return Err(usage(
                "describe",
                "LABEL=TEXT for a supplied label",
                "unknown label or missing '='",
            ));
        };
        let label = labels
            .get_mut(index)
            .ok_or_else(|| super::local("the selected label is absent"))?;
        if label.is_object() {
            return Err(usage(
                "describe",
                "one description per label",
                "repeated label",
            ));
        }
        *label = json!({"name":name,"description":text});
    }
    Ok(())
}

/// Admit repeated command selectors once before input is read.
pub(super) fn field(args: &ArgMatches) -> Result<Vec<crate::Pointer>, Error> {
    let values = args
        .get_many::<String>("field")
        .map(|values| values.cloned().collect::<Vec<_>>())
        .or_else(|| {
            std::env::var("THINKTHEN_FIELD")
                .ok()
                .map(|value| vec![value])
        })
        .unwrap_or_default();
    let pointers = values
        .iter()
        .map(|value| value.parse::<crate::Pointer>())
        .collect::<Result<Vec<_>, _>>()?;
    if pointers.len() > 1 {
        let mut names = std::collections::BTreeSet::new();
        for pointer in &pointers {
            let Some(name) = pointer.leaf() else {
                return Err(usage(
                    "field",
                    "non-root pointers when repeated",
                    "root pointer",
                ));
            };
            if !names.insert(name) {
                return Err(usage(
                    "field",
                    "distinct final pointer names",
                    "repeated field name",
                ));
            }
        }
    }
    Ok(pointers)
}
