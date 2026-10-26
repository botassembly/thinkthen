//! Write the request and result JSON Schemas from the public Rust types.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argument = std::env::args().nth(1);
    let directory = std::path::Path::new("schemas");
    let check = argument.as_deref() == Some("--check");
    if !check {
        std::fs::create_dir_all(directory)?;
    }
    for (name, schema) in [
        ("client", schemars::schema_for!(thinkthen::Settings)),
        ("request", schemars::schema_for!(thinkthen::Request)),
        ("row", schemars::schema_for!(thinkthen::results::AnyRow)),
    ] {
        emit(directory, name, &schema, check)?;
    }
    let registry = serde_json::json!({
        "abi_version":thinkthen::ABI_VERSION,
        "id_kinds":thinkthen::records::IdKind::ALL.iter().map(|kind|serde_json::json!({"name":kind,"value":kind.abi()})).collect::<Vec<_>>(),
        "record_kinds":thinkthen::records::RecordKind::ALL.iter().map(|kind|serde_json::json!({"name":kind,"value":kind.abi()})).collect::<Vec<_>>(),
        "package_version":env!("CARGO_PKG_VERSION"),
        "codes":thinkthen::Code::ALL.iter().map(|code|serde_json::json!({"name":code.as_str(),"value":code.abi(),"family":code.family().as_str(),"record":code.is_record()})).collect::<Vec<_>>(),
        "backends":thinkthen::BackendName::ALL.iter().map(|backend|serde_json::json!({"name":backend,"value":backend.abi()})).collect::<Vec<_>>(),
        "functions":thinkthen::FunctionName::ALL.iter().map(|function|serde_json::json!({"name":function,"value":function.abi()})).collect::<Vec<_>>(),
        "statuses":thinkthen::results::Status::ALL.iter().map(|status|serde_json::json!({"name":status,"value":status.abi()})).collect::<Vec<_>>(),
        "offsets":thinkthen::OffsetUnit::ALL.iter().map(|unit|serde_json::json!({"name":unit,"value":unit.abi()})).collect::<Vec<_>>(),
    });
    emit(directory, "contract", &registry, check)?;
    Ok(())
}

fn emit(
    directory: &std::path::Path,
    name: &str,
    value: &impl serde::Serialize,
    check: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = if name == "contract" {
        "contract.json".to_owned()
    } else {
        format!("{name}.schema.json")
    };
    let path = directory.join(filename);
    let data = format!("{}\n", serde_json::to_string_pretty(value)?);
    if check {
        if std::fs::read_to_string(path)? != data {
            return Err(std::io::Error::other(
                "regenerate canonical contract artifacts with the schema example",
            )
            .into());
        }
    } else {
        std::fs::write(path, data)?;
    }
    Ok(())
}
