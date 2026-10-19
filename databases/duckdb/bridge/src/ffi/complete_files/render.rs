//! Recovered source rows keep the existing native observer and joined facts.
use crate::complete_native::{defect, observations, put};
use serde_json::{Value, json};
use thinkthen::{
    Error, RecordObservation, RequestRecoveringResult as Packet, RequestSessionResult,
};

pub(super) fn packet(packet: Packet) -> Result<Value, Error> {
    match packet {
        Packet::Row(row) => {
            let mut value = serde_json::to_value(&row).map_err(|_| defect())?;
            put(&mut value, "kind", json!("row"))?;
            if let Ok(result) = row.outcome() {
                let encoded = serde_json::to_value(result).map_err(|_| defect())?;
                let count = encoded.as_array().map_or(1, Vec::len);
                let mut typed = vec![json!({}); count];
                thinkthen_host::sql_request::typed_values(result, &mut typed)?;
                put(&mut value, "typed_values", json!(typed))?;
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
                    owned_event(&event)?,
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

fn supplement_values(row: &mut Value, typed: &Value) -> Result<(), Error> {
    for field in ["value", "probability"] {
        if let Some(value) = typed.get(field) {
            put(row, field, value.clone())?;
        }
    }
    Ok(())
}

fn project_packet(
    function: thinkthen::RequestFunction,
    packet: &Value,
    facts: &Value,
    observations: &[Value],
) -> Result<Vec<Value>, Error> {
    let index = packet.get("index").cloned().ok_or_else(defect)?;
    let native = match packet.get("failure") {
        Some(failure) => json!({"error":failure,"facts":facts}),
        None => json!({"value":packet.get("results").ok_or_else(defect)?,"facts":facts}),
    };
    let carrier = json!({"native":native,"observations":observations});
    let mut completed = thinkthen_host::sql_request::project_carrier(function, &carrier);
    if let Some(values) = packet.get("typed_values").and_then(Value::as_array) {
        for (row, typed) in completed.iter_mut().zip(values) {
            supplement_values(row, typed)?;
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
    let mut carrier = json!({"native":native,"rows":rows,"observations":observations});
    let rows = thinkthen_host::sql_request::project_carrier(function, &carrier);
    put(&mut carrier, "rows", json!(rows))?;
    Ok(carrier)
}

/// Borrow an owned native snapshot through the existing SQL event serializer.
fn owned_event(event: &thinkthen::OwnedRecordObservation) -> Result<Value, thinkthen::Error> {
    use thinkthen::{ObservedRow, OwnedObservedRow as Row, OwnedRecordObservation as Event};
    match event {
        Event::Question {
            index,
            member,
            stage,
            position,
            detail,
        } => observations::event(&RecordObservation::Question {
            index: *index,
            member: member.as_deref(),
            stage: *stage,
            position: *position,
            detail: detail.detail(),
        }),
        Event::Row { index, value } => {
            let borrowed = match value {
                Row::Judgment(v) => ObservedRow::Judgment(v),
                Row::Annotated(v) => ObservedRow::Annotated(v),
                Row::Recognized(v) => ObservedRow::Recognized(v),
                Row::Find(v) => ObservedRow::Find(*v),
                Row::Relations(v) => ObservedRow::Relations(v),
            };
            observations::event(&RecordObservation::Row {
                index: *index,
                value: borrowed,
            })
        }
    }
}
