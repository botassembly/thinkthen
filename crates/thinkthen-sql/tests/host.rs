#![cfg(test)]
//! Database chunk boundaries preserve canonical rows and terminal failures.
use loopback::{HeldReply, KEY, Loopback};
use serde_json::{Value, json};
use std::{
    path::Path,
    time::{Duration, Instant},
};
use thinkthen::{Request, Settings};
use thinkthen_sql::{SqlCall, SqlEvent, SqlRecord};
const WAIT: Duration = Duration::from_millis(20);
fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
fn strip_source(value: &mut Value) {
    value.as_object_mut().unwrap().remove("source");
    if let Some(error) = value.get_mut("error").and_then(Value::as_object_mut) {
        error.remove("message");
    }
    if value["function"] == "annotate" {
        for child in value["answer"]
            .as_object_mut()
            .into_iter()
            .flat_map(|entries| entries.values_mut())
        {
            strip_source(child);
        }
    }
}
#[test]
fn canonical_rows_survive_database_chunks_through_both_wires() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance/typed");
    let manifest = read(&directory.join("manifest.json"));
    for name in manifest["cases"].as_array().unwrap() {
        let case = read(&directory.join(name.as_str().unwrap()));
        for backend in manifest["backends"].as_array().unwrap() {
            let server = Loopback::case(&case).unwrap();
            let mut settings = manifest["client"].clone();
            for (key, value) in case["client"]
                .as_object()
                .into_iter()
                .flat_map(|fields| fields.iter())
            {
                settings[key] = value.clone();
            }
            settings["backend"] = backend.clone();
            settings["url"] = json!(server.base());
            let client = Settings::from_json(&settings.to_string())
                .unwrap()
                .engine(|_| Some(KEY.to_owned()))
                .unwrap();
            let request = Request::from_json(&case["request"].to_string());
            if let Some(code) = case["expected"]["error"].as_str() {
                assert_eq!(request.unwrap_err().code().as_str(), code);
                assert_eq!(server.count(), 0);
                continue;
            }
            let request = request.unwrap();
            let records = request.records().to_vec();
            let request = Request::new(request.question().clone());
            let mut call = SqlCall::open(&client, &request).unwrap();
            assert_eq!(
                call.send_chunk(vec![]).unwrap_err().code().as_str(),
                "usage.abi"
            );
            let mut chunks = records.chunks(2);
            let mut actual = Vec::new();
            let mut empty = true;
            let deadline = Instant::now() + Duration::from_secs(30);
            loop {
                assert!(Instant::now() < deadline);
                match call.poll(WAIT, || false).unwrap() {
                    Some(SqlEvent::NeedsChunk) if empty => {
                        empty = false;
                        call.send_chunk(vec![]).unwrap();
                    }
                    Some(SqlEvent::NeedsChunk) => match chunks.next() {
                        Some(chunk) => call
                            .send_chunk(
                                chunk
                                    .iter()
                                    .map(|record| match record {
                                        Value::Null => SqlRecord::Missing,
                                        Value::String(text) => SqlRecord::Text(text.clone()),
                                        value => SqlRecord::Json(value.to_string()),
                                    })
                                    .collect(),
                            )
                            .unwrap(),
                        None => call.end_input(),
                    },
                    Some(SqlEvent::Row(row)) => {
                        let mut value = json!(row);
                        strip_source(&mut value);
                        actual.push(value);
                    }
                    Some(SqlEvent::Finished) => break,
                    None => {}
                }
            }
            assert!(call.final_error().is_none());
            assert_eq!(
                json!(actual),
                case["expected"],
                "{} / {backend}",
                case["name"]
            );
            let requests = server.count() as u64;
            let finite_requests = case["requests"].as_u64().unwrap();
            // The database supplies an unknown stream; a scheduling gap can flush a batch.
            if finite_requests == 0 {
                assert_eq!(requests, 0, "{} / {backend}", case["name"]);
            } else {
                let maximum = finite_requests * records.len() as u64;
                assert!(
                    requests > 0 && requests <= maximum,
                    "{} / {backend}: {requests} requests, expected 1..={maximum}",
                    case["name"]
                );
            }
        }
    }
}
#[test]
fn interrupt_retains_terminal_error_and_drop_closes_held_reply() {
    for abandon in [false, true] {
        let (server, arrived, closed) = Loopback::holding(HeldReply::Body).unwrap();
        let client = Settings::from_json(&json!({"url":server.base(),"batch":1}).to_string())
            .unwrap()
            .engine(|_| Some(KEY.to_owned()))
            .unwrap();
        let request = thinkthen_sql::request("decide", "Ready?", None).unwrap();
        let mut call = SqlCall::open(&client, &request).unwrap();
        while !matches!(
            call.poll(WAIT, || false).unwrap(),
            Some(SqlEvent::NeedsChunk)
        ) {}
        call.send_chunk(vec![SqlRecord::Text("evidence".to_owned())])
            .unwrap();
        call.end_input();
        let deadline = Instant::now() + Duration::from_secs(30);
        let mut interrupted = false;
        loop {
            assert!(Instant::now() < deadline);
            let event = call
                .poll(WAIT, || {
                    interrupted |= arrived.try_recv().is_ok();
                    interrupted && !abandon
                })
                .unwrap();
            if interrupted && abandon {
                break;
            }
            if matches!(event, Some(SqlEvent::Finished)) {
                assert_eq!(
                    call.final_error().unwrap().code().as_str(),
                    "local.cancelled"
                );
                break;
            }
        }
        drop(call);
        assert_eq!(
            closed
                .recv_timeout(Duration::from_secs(30))
                .unwrap()
                .unwrap(),
            0
        );
    }
}
