//! Native projections retain call facts on successful and failed outcomes.
use crate::call::{self, OrRaise as _};
use pgrx::prelude::*;

extension_sql!(
    r#"
CREATE TYPE thinkthen_name AS (text text, start bigint, "end" bigint, kind text, strength double precision);
CREATE TYPE thinkthen_edge AS (relation text, source_name text, source_kind text, target_name text, target_kind text, probability double precision, either boolean);
"#,
    name = "native_result_types"
);

pub(super) type Names = crate::ffi::NameArray;

pub(super) fn tuple(name: &str) -> PgHeapTuple<'static, AllocatedByRust> {
    PgHeapTuple::new_composite_type(name)
        .map_err(|_| call::defect("native result type is unavailable"))
        .or_raise()
}
fn set<T: pgrx::IntoDatum>(tuple: &mut PgHeapTuple<'_, AllocatedByRust>, name: &str, value: T) {
    tuple
        .set_by_name(name, value)
        .map_err(|_| call::defect("native result field is unavailable"))
        .or_raise();
}
pub(super) fn optional<T: pgrx::IntoDatum>(
    tuple: &mut PgHeapTuple<'_, AllocatedByRust>,
    name: &str,
    value: Option<T>,
) {
    if let Some(value) = value {
        set(tuple, name, value);
    }
}
pub(super) fn index(value: usize) -> i64 {
    i64::try_from(value).unwrap_or(i64::MAX)
}

pub(super) fn reading(
    kind: &str,
    value: &serde_json::Value,
) -> PgHeapTuple<'static, AllocatedByRust> {
    super::results_generated::convert(kind, value).or_raise()
}

pub(super) fn events(values: &[serde_json::Value]) -> crate::ffi::EventArray {
    let values = values
        .iter()
        .map(|value| {
            let value = if matches!(
                value.get("kind").and_then(serde_json::Value::as_str),
                Some("question" | "row")
            ) {
                value
            } else {
                value.get("value").unwrap_or(value)
            };
            Some(super::results_generated::convert("thinkthen_event", value).or_raise())
        })
        .collect();
    crate::ffi::EventArray(crate::ffi::composite_array("thinkthen_event", values).or_raise())
}

pub(super) fn optional_reading(
    kind: &str,
    value: Option<&serde_json::Value>,
) -> Option<PgHeapTuple<'static, AllocatedByRust>> {
    value
        .filter(|value| !value.is_null())
        .map(|value| reading(kind, value))
}
pub(super) fn strings(value: Option<&serde_json::Value>) -> Option<Vec<String>> {
    value.and_then(serde_json::Value::as_array).map(|values| {
        values
            .iter()
            .filter_map(serde_json::Value::as_str)
            .map(str::to_owned)
            .collect()
    })
}
pub(super) fn numbers(value: Option<&serde_json::Value>) -> Option<Vec<Option<f64>>> {
    value
        .and_then(serde_json::Value::as_array)
        .map(|values| values.iter().map(serde_json::Value::as_f64).collect())
}
pub(super) fn name_values(value: &serde_json::Value) -> Names {
    let values = value
        .as_array()
        .into_iter()
        .flatten()
        .map(|value| {
            let mut tuple = tuple("thinkthen_name");
            for field in ["text", "kind"] {
                optional(
                    &mut tuple,
                    field,
                    value.get(field).and_then(serde_json::Value::as_str),
                );
            }
            for field in ["start", "end"] {
                optional(
                    &mut tuple,
                    field,
                    value.get(field).and_then(serde_json::Value::as_i64),
                );
            }
            optional(
                &mut tuple,
                "strength",
                value.get("strength").and_then(serde_json::Value::as_f64),
            );
            Some(tuple)
        })
        .collect();
    crate::ffi::NameArray(crate::ffi::composite_array("thinkthen_name", values).or_raise())
}
pub(super) fn edge_value(
    value: &serde_json::Value,
) -> pgrx::composite_type!('static, "thinkthen_edge") {
    let mut tuple = tuple("thinkthen_edge");
    optional(
        &mut tuple,
        "relation",
        value.get("relation").and_then(serde_json::Value::as_str),
    );
    for endpoint in ["source", "target"] {
        let entity = value.get(endpoint);
        optional(
            &mut tuple,
            &format!("{endpoint}_name"),
            entity
                .and_then(|v| v.get("name"))
                .and_then(serde_json::Value::as_str),
        );
        optional(
            &mut tuple,
            &format!("{endpoint}_kind"),
            entity
                .and_then(|v| v.get("kind"))
                .and_then(serde_json::Value::as_str),
        );
    }
    optional(
        &mut tuple,
        "probability",
        value.get("probability").and_then(serde_json::Value::as_f64),
    );
    optional(
        &mut tuple,
        "either",
        value.get("either").and_then(serde_json::Value::as_bool),
    );
    tuple
}
