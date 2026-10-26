//! Direct module requirements come from their registered Rust exports.
use std::{collections::BTreeSet, path::Path};

fn method(line: &str) -> Option<&str> {
    let line = line.trim().strip_prefix("pub ").unwrap_or(line.trim());
    line.strip_prefix("fn ")?.split('(').next()
}
fn camel(name: &str) -> String {
    let mut result = String::new();
    let mut upper = false;
    for ch in name.chars() {
        if ch == '_' {
            upper = true;
        } else if upper {
            result.extend(ch.to_uppercase());
            upper = false;
        } else {
            result.push(ch);
        }
    }
    result
}
fn methods(source: &str, marker: &str, prototype: bool) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    for (start, _) in source.match_indices(marker) {
        let Some(block) = source.get(start + marker.len()..) else {
            continue;
        };
        let Some(block) = block.trim_start().strip_prefix("impl ") else {
            continue;
        };
        let Some((class, body)) = block.split_once('{') else {
            continue;
        };
        let mut depth: i32 = 1;
        for line in body.lines() {
            if depth == 1
                && let Some(name) = method(line)
                && name != "new"
            {
                result.insert(format!(
                    "{}.{}{}",
                    class.trim(),
                    if prototype { "prototype." } else { "" },
                    if prototype {
                        camel(name)
                    } else {
                        name.to_owned()
                    }
                ));
            }
            depth += line.matches('{').count() as i32 - line.matches('}').count() as i32;
            if depth <= 0 {
                break;
            }
        }
    }
    result
}
pub(super) fn emit(
    root: &Path,
    check: bool,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let python = std::fs::read_to_string(root.join("sdk/python/native/src/lib.rs"))?;
    let mut py = methods(&python, "#[pymethods]", false);
    for line in python.lines() {
        if let Some(rest) = line.trim().strip_prefix("module.add_class::<")
            && let Some((name, _)) = rest.split_once('>')
        {
            py.insert(name.into());
        }
        if let Some(rest) = line.trim().strip_prefix("module.add(\"")
            && let Some((name, _)) = rest.split_once('"')
        {
            py.insert(name.into());
        }
    }
    let node = std::fs::read_to_string(root.join("sdk/typescript/native/src/exports.rs"))?;
    let mut js = methods(&node, "#[napi]", true);
    for line in node.lines() {
        if let Some(rest) = line.strip_prefix("pub struct ")
            && let Some(name) = rest.split_whitespace().next()
        {
            js.insert(name.into());
        }
        if line.starts_with("pub fn ")
            && let Some(name) = method(line)
        {
            js.insert(camel(name));
        }
    }
    let r = std::fs::read_to_string(root.join("sdk/r/native/src/lib.rs"))?;
    let r = r
        .split("extendr_module!")
        .nth(1)
        .ok_or("missing R registration")?;
    let r: BTreeSet<_> = r
        .lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("fn ")
                .and_then(|line| line.strip_suffix(';'))
        })
        .map(|name| format!("wrap__{name}"))
        .collect();
    let result = serde_json::json!({"python":py,"typescript":js,"r":r});
    super::emit(
        &root.join("conformance/native/modules.json"),
        format!("{}\n", serde_json::to_string_pretty(&result)?).as_bytes(),
        check,
    )?;
    Ok(result)
}
