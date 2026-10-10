//! SQL transport retains only completed output, never producer descriptors.
use crate::complete_native::{defect, observations};
use serde_json::{Value, json};
use thinkthen::{Error, RequestSessionResult as Packet, RequestSessionRow as Row};

pub(super) fn packet(packet: Packet) -> Result<Value, Error> {
    let mut value: Value = serde_json::from_str(&packet.to_json()?).map_err(|_| defect())?;
    match &packet {
        Packet::Row(row) => {
            let ordinal = match row {
                Row::Decision(v) => v.ordinal(),
                Row::Choice(v) => v.ordinal(),
                Row::Tags(v) => v.ordinal(),
                Row::Score(v) => v.ordinal(),
                Row::Filter(v) => v.ordinal(),
                Row::Annotation(v) => v.ordinal(),
            };
            crate::complete_native::put(&mut value, "ordinals", json!([ordinal]))?;
        }
        Packet::Aggregate(result) => super::super::complete::supplement(&mut value, result)?,
        Packet::Observation { value: event, .. } => {
            crate::complete_native::put(&mut value, "value", observations::owned_event(event)?)?
        }
        Packet::Terminal(_) => {}
    }
    Ok(value)
}

pub(super) fn finish(packets: &str, verb: &str) -> Result<Value, Error> {
    let packets: Vec<Value> = serde_json::from_str(packets).map_err(|_| defect())?;
    let mut rows = Vec::new();
    let mut observations = Vec::new();
    let mut ordinals = Vec::new();
    let mut aggregate = None;
    let mut selection = None;
    let mut terminal = None;
    for packet in packets {
        match packet.get("kind").and_then(Value::as_str) {
            Some("row") => rows.push(packet.get("value").ok_or_else(defect)?.clone()),
            Some("aggregate") => aggregate = Some(packet.get("value").ok_or_else(defect)?.clone()),
            Some("observation") => {
                observations.push(packet.get("value").ok_or_else(defect)?.clone())
            }
            Some("terminal") => terminal = Some(packet.clone()),
            _ => return Err(defect()),
        }
        if let Some(values) = packet.get("ordinals").and_then(Value::as_array) {
            ordinals.extend(values.iter().cloned());
        }
        if packet.get("selection").is_some() {
            selection = Some(packet.get("selection").ok_or_else(defect)?.clone());
        }
    }
    let terminal = terminal.ok_or_else(defect)?;
    let results = aggregate.unwrap_or(Value::Array(rows));
    let mut native = if let Some(failure) = terminal.get("failure") {
        let mut failure = failure.clone();
        if terminal.get("facts").is_some() || results.as_array().is_none_or(|rows| !rows.is_empty())
        {
            crate::complete_native::put(&mut failure, "completed", results)?;
        }
        failure
    } else {
        json!({"value":results,"facts":terminal.get("facts").ok_or_else(defect)?})
    };
    if native.get("value").is_some() || native.get("completed").is_some() {
        if let Some(selection) = selection {
            crate::complete_native::put(&mut native, "selection", selection)?;
        } else if !matches!(verb, "find" | "relate") {
            crate::complete_native::put(&mut native, "ordinals", Value::Array(ordinals))?;
        }
    }
    Ok(crate::complete_native::carrier(
        native,
        Value::Array(observations),
    ))
}
