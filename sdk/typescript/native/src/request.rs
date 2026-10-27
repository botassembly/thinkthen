//! One bridge worker drives the shared bounded native call.
use super::Bridge;
use serde_json::json;
use std::time::Duration;
use thinkthen::{
    Engine, Error, OffsetUnit, Request,
    native::{Call, Event},
};
pub(super) fn run(
    request: &str,
    bridge: &Bridge,
    client: &Engine,
    finite: bool,
) -> Result<(), Error> {
    // Admit the question before asking the host to inspect any record.
    let request = Request::from_json(request)?;
    let client = client.clone().cancel(bridge.cancel.clone());
    let mut call = if finite {
        if !request.records().is_empty() {
            return Err(thinkthen::native::abi_error());
        }
        let mut records = Vec::new();
        loop {
            if !bridge.publish("pull", String::new()) {
                return Err(thinkthen::native::cancelled_error());
            }
            let reply = bridge
                .receive
                .recv()
                .map_err(|_| thinkthen::native::cancelled_error())?;
            if bridge.cancel.is_cancelled() {
                return Err(thinkthen::native::cancelled_error());
            }
            match reply.record {
                Some(record) => records.push(record),
                None => break,
            }
        }
        Call::open_records(&client, &request, records, OffsetUnit::Utf16)?
    } else {
        Call::open(&client, &request, OffsetUnit::Utf16)?
    };
    let events = call.events();
    let _ = bridge.replies.events.set(events.clone());
    let mut input_pending = false;
    let mut row_pending = false;
    let mut buffered = None;
    loop {
        let version = events.version();
        if bridge.cancel.is_cancelled() {
            return Err(thinkthen::native::cancelled_error());
        }
        for reply in bridge.receive.try_iter() {
            match reply.kind {
                "pull" => {
                    input_pending = false;
                    match reply.record {
                        Some(record) => call.send(record)?,
                        None => call.end_input(),
                    }
                }
                "row" => row_pending = false,
                _ => {}
            }
        }
        // Retain one output credit while JavaScript holds a delivered row,
        // but continue servicing input independently until the bounded buffer fills.
        if !row_pending && let Some(data) = buffered.take() {
            row_pending = true;
            if !bridge.publish("row", data) {
                return Err(thinkthen::native::cancelled_error());
            }
            continue;
        }
        let event = if buffered.is_some() {
            None
        } else {
            call.poll(Duration::ZERO)?
        };
        match event {
            Some(Event::NeedsInput) if !input_pending => {
                input_pending = true;
                if !bridge.publish("pull", String::new()) {
                    return Err(thinkthen::native::cancelled_error());
                }
                continue;
            }
            Some(Event::Row(row)) => {
                let mut value =
                    serde_json::to_value(&row).map_err(|_| thinkthen::native::internal_error())?;
                value
                    .as_object_mut()
                    .ok_or_else(thinkthen::native::internal_error)?
                    .remove("id");
                let id = match row.id() {
                    None => json!({"kind":thinkthen::records::IdKind::Absent}),
                    Some(id) if id.as_str().is_some() => {
                        json!({"kind":id.kind(),"value":id.as_str()})
                    }
                    Some(id) => {
                        json!({"kind":id.kind(),"value":id.as_i64().map(|id|id.to_string()).or_else(||id.as_u64().map(|id|id.to_string()))})
                    }
                };
                let packet = json!({"row":value,"position":row.position(),"id":id});
                buffered = Some(packet.to_string());
                continue;
            }
            Some(Event::Finished) => return Ok(()),
            Some(Event::NeedsInput) | None => events.wait(version),
            Some(_) => return Err(thinkthen::native::abi_error()),
        }
    }
}
