//! Pass through native packet JSON without restating its public carriers.
use serde_json::{Value, json};
use thinkthen::{Engine, Error, Request, RequestSessionRead, RequestSessionWaitInterest};

pub(crate) fn run(document: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    let result = execute(document);
    match result {
        Ok(packets) => Ok(json!({"packets": packets})),
        Err(error) => Ok(json!({"packets": [], "error": error.complete()})),
    }
}

fn execute(document: &Value) -> Result<Vec<Value>, Error> {
    let request = Request::from_json(&document.get("request").unwrap_or(&Value::Null).to_string())?;
    let base = document.get("base_url").and_then(Value::as_str)
        .ok_or_else(|| Error::new(thinkthen::ErrorKind::Usage, "packet dumper needs a loopback URL"))?;
    let engine = Engine::builder()
        .base_url(base)?
        .api_key("sk-conformance-loopback")?
        .model("jev-1.13.0")?
        .batch(thinkthen::BatchSetting::Records(std::num::NonZeroUsize::MIN))
        .max_retries(0)
        .no_cache()
        .shared_host()
        .build()?;
    let session = engine.request_session(request)?;
    if document.get("cancel").and_then(Value::as_bool) == Some(true) {
        session.cancel();
    }
    let mut packets = Vec::new();
    loop {
        match session.next_packet(RequestSessionWaitInterest::Settlement)? {
            RequestSessionRead::Result(packet) => {
                packets.push(serde_json::from_str(&packet.to_json()?)
                    .map_err(|_| Error::new(thinkthen::ErrorKind::Defect, "invalid packet JSON"))?);
            }
            RequestSessionRead::End => return Ok(packets),
            RequestSessionRead::Pending => {}
        }
    }
}
