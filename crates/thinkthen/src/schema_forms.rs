//! The two hand `JsonSchema` forms the result schema needs (ADR 0112).
//!
//! Every other result type derives its schema. These live outside `core` and
//! compile in every test build, with or without the command.

use std::borrow::Cow;

use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};

use crate::core::{Json, Threshold};

/// A record, a question text, or a description prints as the JSON it holds.
impl JsonSchema for Json {
    fn schema_name() -> Cow<'static, str> {
        "json".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!(true)
    }
}

/// A cut prints as a number and a band as the string `LOW:HIGH`.
impl JsonSchema for Threshold {
    fn schema_name() -> Cow<'static, str> {
        "threshold".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({"type": ["number", "string"]})
    }
}
