//! Native SQL source fields enter the existing authorized reader protocol.
use super::{BridgeText, Reply, reply_boundary, text};
use crate::complete_native;
use serde_json::{Value, json, value::RawValue};
use std::collections::BTreeMap;
use thinkthen::Error;

fn descriptor(raw: &str, incremental: bool) -> Result<Value, Error> {
    match thinkthen::Settings::parse(&format!("{{\"sql_source\":{raw}}}")) {
        Ok(_) | Err(thinkthen::SettingsError::UnknownKey(_)) => {}
        Err(error) => return Err(complete_native::usage(&error.to_string())),
    }
    let mut fields: BTreeMap<String, Box<RawValue>> = serde_json::from_str(raw)
        .map_err(|_| complete_native::usage("source uses named native fields"))?;
    fields.retain(|_, value| value.get() != "null");
    if fields.keys().any(|key| {
        !matches!(
            key.as_str(),
            "paths" | "unit" | "window" | "format" | "media"
        )
    }) {
        return Err(complete_native::usage("unknown native source field"));
    }
    let paths = fields
        .remove("paths")
        .ok_or_else(|| complete_native::usage("source requires paths"))?;
    let paths = if let Ok(path) = serde_json::from_str::<String>(paths.get()) {
        vec![path]
    } else {
        serde_json::from_str::<Vec<String>>(paths.get()).map_err(|_| {
            complete_native::usage("source paths are text or an ordered text collection")
        })?
    };
    let mut reading = serde_json::Map::new();
    for name in ["unit", "window"] {
        if let Some(value) = fields.remove(name) {
            reading.insert(
                name.to_owned(),
                serde_json::from_str(value.get()).map_err(|_| complete_native::defect())?,
            );
        }
    }
    let media = fields
        .remove("media")
        .map(|value| serde_json::from_str::<Value>(value.get()))
        .transpose()
        .map_err(|_| complete_native::defect())?
        .unwrap_or(json!("text"));
    let mut files = json!({"paths":paths,"options":{"reading":reading,"media":media}});
    if let Some(format) = fields.remove("format") {
        complete_native::put(
            &mut files,
            "format",
            serde_json::from_str(format.get()).map_err(|_| complete_native::defect())?,
        )?;
    }
    let value = json!({"files":files,"incremental":incremental});
    // This validates only controls; the host still owns all metadata and handles.
    complete_native::Inputs::parse(&value.to_string(), true)?;
    Ok(value)
}

/// Normalize authored native fields for the existing private source reader.
/// # Safety
/// The counted input remains readable through return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_native_source_input(
    source: BridgeText,
    incremental: bool,
) -> Reply {
    reply_boundary(|| {
        descriptor(text(source.bytes, source.len)?, incremental)
            .map(|value| value.to_string().into_bytes())
            .map_err(|error| complete_native::failure(&error).to_string())
    })
}
