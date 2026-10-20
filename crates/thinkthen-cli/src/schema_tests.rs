//! Stays: command presentation schemas derive from the types the command serializes.

use schemars::Schema;
use schemars::generate::SchemaSettings;
use schemars::transform::RecursiveTransform;
use serde_json::Value;

const FILE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specification/result.schema.json"
);

// The native transparent Record derives its unconstrained JSON as an object
// schema. Preserve that exact declaration without exposing its private type.
pub(crate) fn record(_: &mut schemars::SchemaGenerator) -> Schema {
    schemars::json_schema!({})
}

#[test]
fn the_committed_command_schemas_are_the_ones_the_rust_types_derive() {
    let mut generator = SchemaSettings::draft2020_12()
        .for_serialize()
        .with_transform(RecursiveTransform(|schema: &mut Schema| {
            schema.remove("description");
        }))
        .into_generator();
    let _registered = [
        generator.subschema_for::<crate::cli::annotate::error_row::Row<'_>>(),
        generator.subschema_for::<crate::cli::intake::Position>(),
        generator.subschema_for::<crate::cli::intake::SourceFields<'_>>(),
        generator.subschema_for::<crate::cli::relate::source::Endpoint<'_>>(),
    ];
    let definitions = generator.take_definitions(true);
    assert_eq!(
        definitions.keys().map(String::as_str).collect::<Vec<_>>(),
        [
            "Position",
            "SourceFields",
            "recordError",
            "recordFailure",
            "sourceRelationEndpoint",
        ],
        "command schema ownership changed"
    );
    let committed = std::fs::read_to_string(FILE).expect("committed result schema");
    let mut committed: Value = serde_json::from_str(&committed).expect("result schema JSON");
    let mut changed = Vec::new();
    for (name, schema) in definitions {
        let destination = committed["$defs"]
            .as_object_mut()
            .expect("result definitions")
            .get_mut(&name)
            .expect("committed command definition");
        if *destination != schema {
            changed.push(format!("{name}: committed={destination}, derived={schema}"));
            *destination = schema;
        }
    }
    if !changed.is_empty()
        && std::env::var_os("THINKTHEN_WRITE_SCHEMA").is_some_and(|value| value == "1")
    {
        std::fs::write(
            FILE,
            serde_json::to_string_pretty(&committed).expect("schema text") + "\n",
        )
        .expect("rewrite command schema definitions");
    }
    assert!(
        changed.is_empty(),
        "generated command schemas differ; run THINKTHEN_WRITE_SCHEMA=1 cargo test -p thinkthen-cli --bin thinkthen schema_tests, then rewrite and rerun the library schema tests; differences: {}",
        changed.join("; ")
    );
}
