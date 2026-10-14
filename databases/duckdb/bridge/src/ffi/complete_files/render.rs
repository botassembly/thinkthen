//! Recovered source rows keep the existing native observer and joined facts.
use crate::complete_native::{defect, observations, put};
use serde_json::{Value, json};
use thinkthen::{Error, RequestRecoveringResult as Packet, RequestSessionResult};

pub(super) fn packet(packet: Packet) -> Result<Value, Error> {
    match packet {
        Packet::Row(row) => {
            let mut value = serde_json::to_value(&row).map_err(|_| defect())?;
            put(&mut value, "kind", json!("row"))?;
            if let Ok(result) = row.outcome() {
                put(
                    &mut value,
                    "answers",
                    super::super::native_records::plain_values(result)?,
                )?;
            }
            Ok(value)
        }
        Packet::Observation {
            function,
            value: event,
        } => {
            let ordinary = RequestSessionResult::Observation {
                function,
                value: event,
            };
            let mut value: Value =
                serde_json::from_str(&ordinary.to_json()?).map_err(|_| defect())?;
            if let RequestSessionResult::Observation { value: event, .. } = ordinary {
                super::super::native_records::observations::supplement(
                    value.get_mut("value").ok_or_else(defect)?,
                    observations::owned_event(&event)?,
                )?;
            }
            Ok(value)
        }
        Packet::Terminal(mut terminal) => {
            if let Some(error) = terminal.error.as_ref()
                && matches!(
                    error.kind(),
                    thinkthen::ErrorKind::Cancelled
                        | thinkthen::ErrorKind::Deadline
                        | thinkthen::ErrorKind::Defect
                )
            {
                return Err(terminal.error.take().ok_or_else(defect)?);
            }
            serde_json::from_str(&RequestSessionResult::Terminal(terminal).to_json()?)
                .map_err(|_| defect())
        }
    }
}

fn project_packet(
    function: thinkthen::RequestFunction,
    packet: &Value,
    facts: &Value,
    observations: &[Value],
) -> Result<Vec<Value>, Error> {
    let index = packet.get("index").cloned().ok_or_else(defect)?;
    let events = observations
        .iter()
        .filter(|event| event.get("index").is_none() || event.get("index") == Some(&index))
        .cloned()
        .collect::<Vec<_>>();
    let native = match packet.get("failure") {
        Some(failure) => json!({"error":failure,"facts":facts}),
        None => json!({"value":packet.get("results").ok_or_else(defect)?,"facts":facts}),
    };
    let carrier = json!({"native":native,"observations":events});
    let mut completed = thinkthen_host::sql_request::project_carrier(function, &carrier);
    if function == thinkthen::RequestFunction::Decide
        && let Some(answers) = packet.get("answers").and_then(Value::as_array)
    {
        for (row, answer) in completed.iter_mut().zip(answers) {
            put(row, "value", answer.clone())?;
        }
    }
    if packet.get("failure").is_some() {
        for row in &mut completed {
            put(row, "ordinal", index.clone())?;
        }
    }
    Ok(completed)
}

pub(super) fn finish(packets: &str, verb: &str) -> Result<Value, Error> {
    let packets: Vec<Value> = serde_json::from_str(packets).map_err(|_| defect())?;
    let function = verb
        .parse::<thinkthen::RequestFunction>()
        .map_err(|_| defect())?;
    let mut recovered = Vec::new();
    let mut observations = Vec::new();
    let mut terminal = None;
    for packet in packets {
        match packet.get("kind").and_then(Value::as_str) {
            Some("row") => recovered.push(packet),
            Some("observation") => {
                observations.push(packet.get("value").ok_or_else(defect)?.clone())
            }
            Some("terminal") => terminal = Some(packet),
            _ => return Err(defect()),
        }
    }
    let terminal = terminal.ok_or_else(defect)?;
    let facts = terminal
        .get("facts")
        .or_else(|| terminal.pointer("/failure/facts"))
        .cloned()
        .unwrap_or(Value::Null);
    let mut rows = Vec::new();
    for packet in recovered {
        rows.extend(project_packet(function, &packet, &facts, &observations)?);
    }
    let native = terminal
        .get("failure")
        .cloned()
        .unwrap_or_else(|| json!({"facts":facts}));
    if terminal.get("failure").is_some() {
        let carrier = json!({"native":native,"observations":observations});
        rows.extend(thinkthen_host::sql_request::project_carrier(
            function, &carrier,
        ));
    }
    Ok(json!({"native":native,"rows":rows,"observations":observations}))
}
