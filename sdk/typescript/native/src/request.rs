//! Drive the shared pull call on the bridge worker, never on an engine thread.

use super::Bridge;
use serde_json::json;
use thinkthen::{Client, Error, OffsetUnit, PullEvent, SdkRequest};

pub(super) fn run(request: &str, bridge: &Bridge, client: &Client) -> Result<(), Error> {
    let request = SdkRequest::from_json(request)?;
    let mut call = client.open(&request, OffsetUnit::Utf16, bridge.cancel.clone())?;
    loop {
        match call.next_event() {
            PullEvent::NeedsInput => match bridge.exchange("pull", String::new()) {
                Some((true, text)) => call.send_text(&text?)?,
                Some((false, text)) => {
                    let text = text?;
                    if text.is_empty() {
                        call.end_input();
                    } else {
                        call.send_json(&text)?;
                    }
                }
                _ => call.end_input(),
            },
            PullEvent::Row(row) => {
                let packet = json!({"row": serde_json::from_str::<serde_json::Value>(&row.json).map_err(|error| thinkthen::Error::new(thinkthen::ErrorKind::Io, error.to_string()))?, "detail": row.detail, "kept": row.kept, "position": row.position});
                let _ = bridge.exchange("row", packet.to_string());
            }
            PullEvent::Finished => return call.final_error().cloned().map_or(Ok(()), Err),
        }
    }
}
