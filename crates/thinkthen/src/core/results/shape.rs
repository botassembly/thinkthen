//! Function-specific row invariants are shared by readers and generated schemas.
use super::*;

fn policy(function: FunctionName) -> (bool, bool) {
    match function {
        FunctionName::Decide | FunctionName::Choose | FunctionName::Find => (true, true),
        FunctionName::Filter => (true, false),
        FunctionName::Tag
        | FunctionName::Score
        | FunctionName::Rank
        | FunctionName::Recognize
        | FunctionName::Relate
        | FunctionName::Annotate => (false, false),
    }
}

pub(super) fn validate<F: Function>(
    outcome: &Outcome<F::Answer, F::Detail>,
) -> Result<(), &'static str> {
    let (probability, unsure) = policy(F::NAME);
    match outcome {
        Outcome::Answered {
            probability: value, ..
        } if value.is_some() != probability => {
            Err("the row probability does not match its function")
        }
        Outcome::Unsure { .. } if !unsure => Err("the function cannot return not sure"),
        _ => Ok(()),
    }
}

pub(super) fn saved_item(schema: &mut schemars::Schema) {
    let properties = schema
        .ensure_object()
        .entry("properties")
        .or_insert_with(|| serde_json::json!({}));
    if let Some(properties) = properties.as_object_mut() {
        properties.insert("item".into(), serde_json::Value::Bool(true));
    }
}

fn status(branch: &serde_json::Value, expected: &str) -> bool {
    branch
        .get("properties")
        .and_then(|properties| properties.get("status"))
        .and_then(|status| status.get("const"))
        .and_then(serde_json::Value::as_str)
        == Some(expected)
}

pub(super) fn outcome_schema<F: Function>(g: &mut schemars::SchemaGenerator) -> schemars::Schema {
    let mut schema = <Outcome<F::Answer, F::Detail> as schemars::JsonSchema>::json_schema(g);
    let (probability, unsure) = policy(F::NAME);
    if let Some(branches) = schema
        .ensure_object()
        .get_mut("oneOf")
        .and_then(serde_json::Value::as_array_mut)
    {
        branches.retain(|branch| unsure || !status(branch, "unsure"));
        for branch in branches {
            if status(branch, "answered") {
                if probability {
                    if let Some(required) = branch
                        .get_mut("required")
                        .and_then(serde_json::Value::as_array_mut)
                    {
                        required.push("probability".into());
                    }
                } else if let Some(properties) = branch
                    .get_mut("properties")
                    .and_then(serde_json::Value::as_object_mut)
                {
                    properties.insert("probability".into(), serde_json::Value::Bool(false));
                }
                if let Some(answer) = branch
                    .get_mut("properties")
                    .and_then(|properties| properties.get_mut("answer"))
                    .and_then(serde_json::Value::as_object_mut)
                {
                    answer.insert(
                        "description".into(),
                        if F::NAME == FunctionName::Find {
                            "The selected span, or explicit null when no line answers."
                        } else {
                            "The function's typed answer."
                        }
                        .into(),
                    );
                }
            }
        }
    }
    schema
}

pub(super) fn entry_schema<F: Function>(g: &mut schemars::SchemaGenerator) -> schemars::Schema {
    let mut schema = outcome_schema::<F>(g);
    if let Some(branches) = schema
        .ensure_object()
        .get_mut("oneOf")
        .and_then(serde_json::Value::as_array_mut)
    {
        for branch in branches {
            // The outer entry adds `function`; its unevaluatedProperties rule closes
            // the full object after the matching outcome branch has evaluated it.
            if let Some(fields) = branch.as_object_mut() {
                fields.remove("additionalProperties");
            }
        }
    }
    let fields = schema.ensure_object();
    fields.insert("type".into(), "object".into());
    fields.insert(
        "properties".into(),
        serde_json::json!({"function":{"type":"string","const":F::NAME}}),
    );
    fields.insert("required".into(), serde_json::json!(["function"]));
    fields.insert("unevaluatedProperties".into(), false.into());
    schema
}
