//! Engine-independent owned values; the common graph selects every host type.
use super::{Fault, checked};
use magnus::{Error, RClass, RModule, Ruby, Value, function, method, prelude::*};
use serde_json::Value as Json;
use std::sync::OnceLock;

#[magnus::wrap(class = "ThinkThen::Native::Result", free_immediately, size)]
struct NativeResult {
    kind: String,
    value: Json,
}
impl std::fmt::Debug for NativeResult {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("NativeResult")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}
impl NativeResult {
    fn get(ruby: &Ruby, this: &Self, member: String) -> Result<Value, Error> {
        let value = this.value.get(&member).ok_or_else(|| {
            Error::new(ruby.exception_key_error(), "native result field is absent")
        })?;
        let schema = graph()
            .get(&this.kind)
            .and_then(|s| s.get("fields"))
            .and_then(|s| s.get(&member));
        match schema {
            Some(schema) => field(ruby, schema, value),
            None => plain(ruby, value),
        }
    }
    fn has(&self, member: String) -> bool {
        self.value.get(member).is_some()
    }
    fn keys(&self) -> Vec<String> {
        self.value
            .as_object()
            .map(|v| v.keys().cloned().collect())
            .unwrap_or_default()
    }
    fn to_h(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        plain(ruby, &this.value)
    }
    fn inspect(&self) -> String {
        format!("<ThinkThen::Results {}: content withheld>", self.kind)
    }
}
fn graph() -> &'static Json {
    static GRAPH: OnceLock<Json> = OnceLock::new();
    GRAPH
        .get_or_init(|| serde_json::from_str(super::results_generated::GRAPH).unwrap_or(Json::Null))
}
fn matches(tag: &Json, value: &Json) -> bool {
    let mode = tag.get(0).and_then(Json::as_str);
    let member = tag.get(1).and_then(Json::as_str).unwrap_or("");
    let wanted = tag.get(2).unwrap_or(&Json::Null);
    match mode {
        Some("literal") => value.get(member) == Some(wanted),
        Some("member") => value.get(member).is_some(),
        Some("literals") => wanted
            .as_object()
            .is_some_and(|m| m.iter().all(|(k, v)| value.get(k) == Some(v))),
        Some("structure") => {
            wanted
                .get("required")
                .and_then(Json::as_array)
                .is_some_and(|items| {
                    items
                        .iter()
                        .all(|k| k.as_str().is_some_and(|k| value.get(k).is_some()))
                })
                && wanted
                    .get("excluded")
                    .and_then(Json::as_array)
                    .is_some_and(|items| {
                        items
                            .iter()
                            .all(|k| k.as_str().is_some_and(|k| value.get(k).is_none()))
                    })
        }
        _ => false,
    }
}
pub(super) fn convert(ruby: &Ruby, kind: &str, value: &Json) -> Result<Value, Error> {
    let schema = graph().get(kind).ok_or_else(|| {
        Error::new(
            ruby.exception_runtime_error(),
            "unknown generated native result type",
        )
    })?;
    if let Some(variants) = schema.get("variants").and_then(Json::as_array) {
        for variant in variants {
            if let (Some(child), Some(tag)) =
                (variant.get(0).and_then(Json::as_str), variant.get(1))
                && matches(tag, value)
            {
                return convert(ruby, child, value);
            }
        }
        return Err(Error::new(
            ruby.exception_runtime_error(),
            "native result has no generated alternative",
        ));
    }
    if let Some(name) = schema.get("class").and_then(Json::as_str) {
        let module: RModule = ruby.class_object().const_get("ThinkThen")?;
        let results: RModule = module.const_get("Results")?;
        let class: RClass = results.const_get(name)?;
        let result = ruby.obj_wrap_as(
            NativeResult {
                kind: kind.into(),
                value: value.clone(),
            },
            class,
        );
        result.freeze();
        return Ok(result.as_value());
    }
    if let Some(source) = schema
        .get("primitive_schemas")
        .and_then(|s| s.get("object"))
        && value.is_object()
    {
        return field(ruby, source, value);
    }
    field(ruby, schema, value)
}
fn field(ruby: &Ruby, schema: &Json, value: &Json) -> Result<Value, Error> {
    if let Some(reference) = schema.get("$ref").and_then(Json::as_str) {
        return convert(ruby, reference.trim_start_matches("#/$defs/"), value);
    }
    if let Some(items) = schema
        .get("anyOf")
        .or_else(|| schema.get("oneOf"))
        .and_then(Json::as_array)
    {
        let refs: Vec<_> = items.iter().filter(|v| v.get("$ref").is_some()).collect();
        if let [source] = refs.as_slice()
            && !value.is_null()
        {
            return field(ruby, source, value);
        }
    }
    if let Some(values) = value.as_array() {
        let array = ruby.ary_new();
        for item in values {
            array.push(match schema.get("items") {
                Some(s) => field(ruby, s, item)?,
                None => plain(ruby, item)?,
            })?;
        }
        return {
            array.freeze();
            Ok(array.as_value())
        };
    }
    if let Some(source) = schema.get("additionalProperties").filter(|s| s.is_object())
        && let Some(values) = value.as_object()
    {
        let hash = ruby.hash_new();
        for (key, item) in values {
            hash.aset(key.as_str(), field(ruby, source, item)?)?;
        }
        return {
            hash.freeze();
            Ok(hash.as_value())
        };
    }
    plain(ruby, value)
}
pub(super) fn plain(ruby: &Ruby, value: &Json) -> Result<Value, Error> {
    let result: Value = match value {
        Json::Null => ruby.qnil().as_value(),
        Json::Bool(v) => ruby.into_value(*v),
        Json::String(v) => ruby.into_value(v.clone()),
        Json::Number(v) => {
            if let Some(n) = v.as_i64() {
                ruby.into_value(n)
            } else if let Some(n) = v.as_u64() {
                ruby.into_value(n)
            } else {
                ruby.into_value(v.as_f64().ok_or_else(|| {
                    Error::new(
                        ruby.exception_runtime_error(),
                        "native number cannot be represented",
                    )
                })?)
            }
        }
        Json::Array(values) => {
            let a = ruby.ary_new();
            for v in values {
                a.push(plain(ruby, v)?)?;
            }
            a.as_value()
        }
        Json::Object(values) => {
            let h = ruby.hash_new();
            for (k, v) in values {
                h.aset(k.as_str(), plain(ruby, v)?)?;
            }
            h.as_value()
        }
    };
    result.funcall("freeze", ())
}
pub(super) fn packet(ruby: &Ruby, text: &str) -> Result<Value, Error> {
    let value = checked(
        ruby,
        serde_json::from_str(text).map_err(|_| {
            Fault::of(
                thinkthen::ErrorKind::Defect,
                "native packet could not be read",
            )
        }),
    )?;
    convert(ruby, "completesessionPacket", &value)
}
pub(super) fn register(ruby: &Ruby, native: RModule) -> Result<(), Error> {
    let result = native.define_class("Result", ruby.class_object())?;
    result.define_singleton_method("inherited", function!(RClass::undef_default_alloc_func, 1))?;
    result.define_method("[]", method!(NativeResult::get, 1))?;
    result.define_method("key?", method!(NativeResult::has, 1))?;
    result.define_method("keys", method!(NativeResult::keys, 0))?;
    result.define_method("to_h", method!(NativeResult::to_h, 0))?;
    result.define_method("inspect", method!(NativeResult::inspect, 0))?;
    Ok(())
}
