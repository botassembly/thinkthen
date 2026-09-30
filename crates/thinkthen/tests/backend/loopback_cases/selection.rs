//! The optional shared case selector.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use serde_json::Value;

/// Read one optional absolute ID list, and refuse duplicate or unknown IDs.
pub(super) fn selected_ids(cases: &[Value]) -> Result<Option<BTreeSet<String>>, String> {
    let mut available = BTreeSet::new();
    for case in cases {
        let id = case["id"].as_str().ok_or("a shared case has no ID")?;
        if !available.insert(id) {
            return Err(format!("duplicate shared case `{id}`"));
        }
    }
    let Some(path) = std::env::var_os("THINKTHEN_CONFORMANCE_IDS") else {
        return Ok(None);
    };
    let path = PathBuf::from(path);
    if !path.is_absolute() {
        return Err("THINKTHEN_CONFORMANCE_IDS takes an absolute path".to_owned());
    }
    let text = fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut selected = BTreeSet::new();
    for id in text
        .lines()
        .map(str::trim)
        .filter(|id| !id.is_empty() && !id.starts_with('#'))
    {
        if !selected.insert(id.to_owned()) {
            return Err(format!("duplicate selected case `{id}`"));
        }
    }
    if selected.is_empty() {
        return Err("the selected case list is empty".to_owned());
    }
    for id in &selected {
        if !available.contains(id.as_str()) {
            return Err(format!(
                "selected case `{id}` is absent from the shared corpus"
            ));
        }
    }
    Ok(Some(selected))
}
