//! Relate rules, row identities, and the private copied C++ bridge.
#![allow(
    unsafe_code,
    reason = "the relate ABI copies every caller-owned byte range"
)]

use std::collections::HashMap;

use thinkthen::{Entity, Relate, RelationRule};

use crate::engines;
use crate::errors::RowError;
use crate::ffi::{
    BridgeSettings, BridgeStop, BridgeText, Reply, asked, copied_texts, probe, reply_boundary,
    run_detached, text,
};

type Ids = HashMap<(String, String), Vec<String>>;
type Found = (Vec<Entity>, Ids);

fn rules(
    text: &str,
    members: &[String],
    list: bool,
    from_file: bool,
) -> Result<(Relate, bool), String> {
    if list {
        if members.is_empty() {
            return Err(RowError::usage("relate needs at least one rule").text);
        }
        let mut builder = Relate::builder();
        let mut wildcard = true;
        for rule in members {
            let (name, source, target) = match rule.split_once('=') {
                None => (rule.as_str(), "*", "*"),
                Some((name, kinds)) => match kinds.split_once(':') {
                    Some((source, target)) if !target.contains([':', '=']) => {
                        (name, source, target)
                    }
                    _ => {
                        return Err(RowError::usage(&format!(
                            "the rule {rule} is not NAME or NAME=SOURCE:TARGET"
                        ))
                        .text);
                    }
                },
            };
            wildcard &= source == "*" && target == "*";
            let made = RelationRule::one_way(name, source, target)
                .map_err(|error| RowError::from(error).text)?;
            builder = builder
                .relation(made)
                .map_err(|error| RowError::from(error).text)?;
        }
        return Ok((
            builder
                .build()
                .map_err(|error| RowError::from(error).text)?,
            wildcard,
        ));
    }
    let ask = Relate::from_json(text).map_err(|error| {
        if from_file && error.kind() == thinkthen::ErrorKind::Usage {
            RowError::local(error.detail().message()).text
        } else {
            RowError::from(error).text
        }
    })?;
    let wildcard = serde_json::from_str::<serde_json::Value>(text)
        .ok()
        .and_then(|file| file.pointer("/relate/relations")?.as_array().cloned())
        .is_some_and(|relations| {
            relations.iter().all(|rule| {
                ["source", "target"]
                    .iter()
                    .all(|side| rule.get(side).and_then(serde_json::Value::as_str) == Some("*"))
            })
        });
    Ok((ask, wildcard))
}

fn copied_rules(
    rule: *const u8,
    rule_len: usize,
    members: *const BridgeText,
    member_count: usize,
    list: i32,
    from_file: i32,
) -> Result<(Relate, bool), String> {
    let rule = if list == 0 { text(rule, rule_len)? } else { "" };
    let members = copied_texts(members, member_count)?;
    if list == 0 && from_file == 0 && rule.starts_with('@') {
        return Err(RowError::local("the rules file was not read by this database").text);
    }
    rules(rule, &members, list != 0, from_file != 0)
}

/// Validate one relate bind, including its engine settings, without a send.
///
/// # Safety
/// The rule and every member remain readable through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_relate_validate(
    rule: *const u8,
    rule_len: usize,
    members: *const BridgeText,
    member_count: usize,
    list: i32,
    from_file: i32,
    settings: BridgeSettings,
) -> Reply {
    reply_boundary(|| {
        let (_, wildcard) = copied_rules(rule, rule_len, members, member_count, list, from_file)?;
        let asked = asked(&settings)?;
        let _engine = engines::engine_for(&asked, |path| probe(&settings, path))?;
        Ok(vec![u8::from(wildcard)])
    })
}

fn frame(bytes: &mut Vec<u8>, value: &str) -> Result<(), String> {
    let len = u32::try_from(value.len())
        .map_err(|_| "thinkthen defect: a relate value is too large".to_owned())?;
    bytes.extend_from_slice(&len.to_ne_bytes());
    bytes.extend_from_slice(value.as_bytes());
    Ok(())
}

fn entity_rows(ids: Vec<String>, names: Vec<String>, kinds: Vec<String>) -> Result<Found, String> {
    if ids.len() != names.len() || ids.len() != kinds.len() || ids.len() > 255 {
        return Err("thinkthen defect: the bridge got invalid relate rows".to_owned());
    }
    let mut entities = Vec::new();
    let mut mapped: Ids = HashMap::new();
    for ((id, name), kind) in ids.into_iter().zip(names).zip(kinds) {
        let listed = mapped.entry((name.clone(), kind.clone())).or_default();
        if listed.is_empty() {
            entities.push(Entity::new(&name, &kind).map_err(|error| RowError::from(error).text)?);
        }
        listed.push(id);
    }
    Ok((entities, mapped))
}

fn encoded_edges(edges: Vec<thinkthen::Edge>, ids: &Ids) -> Result<Vec<u8>, String> {
    let mut rows = Vec::new();
    for edge in edges {
        let source = (
            edge.source().name().to_owned(),
            edge.source().kind().to_owned(),
        );
        let target = (
            edge.target().name().to_owned(),
            edge.target().kind().to_owned(),
        );
        for from in ids.get(&source).into_iter().flatten() {
            for to in ids.get(&target).into_iter().flatten() {
                rows.push((
                    edge.relation().to_owned(),
                    from.clone(),
                    to.clone(),
                    edge.probability(),
                ));
            }
        }
    }
    let count = u32::try_from(rows.len())
        .map_err(|_| "thinkthen defect: too many relate edges".to_owned())?;
    let mut bytes = count.to_ne_bytes().to_vec();
    for (relation, from, to, probability) in rows {
        frame(&mut bytes, &relation)?;
        frame(&mut bytes, &from)?;
        frame(&mut bytes, &to)?;
        bytes.extend_from_slice(&probability.to_ne_bytes());
    }
    Ok(bytes)
}

/// Relate already-capped committed rows under the engine's process registry.
///
/// # Safety
/// All input arrays and their entries remain readable through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_relate_rows(
    rule: *const u8,
    rule_len: usize,
    members: *const BridgeText,
    member_count: usize,
    list: i32,
    from_file: i32,
    ids: *const BridgeText,
    names: *const BridgeText,
    kinds: *const BridgeText,
    count: usize,
    deadline_ms: i64,
    settings: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let (ask, _) = copied_rules(rule, rule_len, members, member_count, list, from_file)?;
        let ids = copied_texts(ids, count)?;
        let names = copied_texts(names, count)?;
        let kinds = copied_texts(kinds, count)?;
        let (entities, mapped) = entity_rows(ids, names, kinds)?;
        if entities.is_empty() {
            return Ok(0_u32.to_ne_bytes().to_vec());
        }
        let asked = asked(&settings)?;
        let engine = engines::engine_for(&asked, |path| probe(&settings, path))?;
        engines::within_total(&asked, Vec::new())?;
        let total = asked.max_requests_total;
        run_detached(stop, move |token| {
            let options = engines::options(deadline_ms, &token, total)?;
            let edges = engine
                .relate_with(&ask, entities, options)
                .map_err(|error| engines::call_error(error, total).text)?;
            encoded_edges(edges.into_value(), &mapped)
        })
    })
}

fn estimate(node: &serde_json::Value) -> Option<u64> {
    let own = node
        .pointer("/extra_info/Estimated Cardinality")
        .and_then(|value| match value {
            serde_json::Value::String(text) => text.trim().parse().ok(),
            other => other.as_u64(),
        });
    let name = node
        .get("name")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");
    if own.is_none() && name.ends_with("LIMIT") {
        return None;
    }
    own.or_else(|| {
        node.get("children")?
            .as_array()?
            .iter()
            .filter_map(estimate)
            .max()
    })
}

fn heaviest(plan: &serde_json::Value) -> Option<(String, u64)> {
    let nodes: Vec<&serde_json::Value> = match plan {
        serde_json::Value::Array(nodes) => nodes.iter().collect(),
        node => vec![node],
    };
    let mut most: Option<(String, u64)> = None;
    for node in nodes {
        let name = node
            .get("name")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        let children: Vec<&serde_json::Value> = node
            .get("children")
            .and_then(serde_json::Value::as_array)
            .map(|children| children.iter().collect())
            .unwrap_or_default();
        let held = if name.contains("DELIM_JOIN") {
            children.get(..).unwrap_or_default()
        } else if name.contains("JOIN") || name == "CROSS_PRODUCT" {
            children.get(1..).unwrap_or_default()
        } else if name == "CTE" {
            children.get(..1).unwrap_or_default()
        } else if matches!(
            name,
            "ORDER_BY"
                | "WINDOW"
                | "HASH_GROUP_BY"
                | "PERFECT_HASH_GROUP_BY"
                | "UNGROUPED_AGGREGATE"
        ) {
            children.get(..).unwrap_or_default()
        } else {
            &[]
        };
        let own = held.iter().filter_map(|child| estimate(child)).max();
        let below = children.iter().filter_map(|child| heaviest(child));
        for found in own
            .map(|rows| (name.to_owned(), rows))
            .into_iter()
            .chain(below)
        {
            if most.as_ref().is_none_or(|(_, rows)| found.1 > *rows) {
                most = Some(found);
            }
        }
    }
    most
}

/// Refuse a plan that estimates too many input rows into one holding step.
///
/// # Safety
/// The plan bytes stay readable through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_relate_plan(
    plan: *const u8,
    len: usize,
    holding: u64,
) -> Reply {
    reply_boundary(|| {
        let text = text(plan, len)?;
        let plan: serde_json::Value = serde_json::from_str(text).map_err(|error| {
            format!("thinkthen defect: the relate plan did not read as JSON: {error}")
        })?;
        if let Some((step, rows)) = heaviest(&plan).filter(|(_, rows)| *rows > holding) {
            return Err(format!(
                "thinkthen usage: the relate query feeds about {rows} rows into the {step} step before its LIMIT, and relate lets at most {holding} rows into a sorting, grouping, windowing, or joining step; filter the rows first or raise SET thinkthen_relate_holding_rows"
            ));
        }
        Ok(Vec::new())
    })
}
