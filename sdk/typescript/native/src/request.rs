//! One bridge worker drives the shared bounded native call.
use super::Bridge;
use serde_json::json;
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
            match bridge.exchange("pull", String::new()) {
                Some(Some(record)) => records.push(record),
                Some(None) => break,
                None => return Err(thinkthen::native::cancelled_error()),
            }
        }
        Call::open_records(&client, &request, records, OffsetUnit::Utf16)?
    } else {
        Call::open(&client, &request, OffsetUnit::Utf16)?
    };
    loop {
        match call.next_event()? {
            Event::NeedsInput => match bridge.exchange("pull", String::new()) {
                Some(Some(record)) => call.send(record)?,
                _ => call.end_input(),
            },
            Event::Row(row) => {
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
                let _ = bridge.exchange("row", packet.to_string());
            }
            Event::Finished => return Ok(()),
            _ => return Err(thinkthen::native::abi_error()),
        }
    }
}
