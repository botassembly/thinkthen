//! Write the request and result JSON Schemas from the public Rust types.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::path::Path::new("schemas");
    std::fs::create_dir_all(directory)?;
    for (name, schema) in [
        ("client", schemars::schema_for!(thinkthen::SdkSettings)),
        ("request", schemars::schema_for!(thinkthen::SdkRequest)),
        ("row", thinkthen::Row::schema()),
    ] {
        std::fs::write(
            directory.join(format!("{name}.schema.json")),
            format!("{}\n", serde_json::to_string_pretty(&schema)?),
        )?;
    }
    Ok(())
}
