#![cfg(test)]
//! Database chunk boundaries preserve canonical rows and terminal failures.
use loopback::{HeldReply, KEY, Loopback};
use serde_json::{Value, json};
use std::{
    path::Path,
    time::{Duration, Instant},
};
use thinkthen::{AnyRow, Error, Request, Settings};
use thinkthen_sql::{SqlCall, SqlEvent, SqlRecord};
const WAIT: Duration = Duration::from_millis(20);
fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
fn strip_source(value: &mut Value) {
    if let Some(source) = value.get_mut("source").and_then(Value::as_object_mut) {
        for field in ["backend", "model", "question_identity"] {
            source.remove(field);
        }
        if source.is_empty() {
            value.as_object_mut().unwrap().remove("source");
        }
    }
    if let Some(error) = value.get_mut("error").and_then(Value::as_object_mut)
        && error.get("code").and_then(Value::as_str).is_some()
    {
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
    let mut failures = Vec::new();
    for name in manifest["cases"].as_array().unwrap() {
        let case = read(&directory.join(name.as_str().unwrap()));
        let mut identity = None;
        for backend in manifest["backends"].as_array().unwrap() {
            let checked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
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
                let actual = (|| -> Result<Vec<AnyRow>, Error> {
                    let request = Request::from_json(&case["request"].to_string())?;
                    let mut client = Settings::from_json(&settings.to_string())?
                        .engine(|_| Some(KEY.to_owned()))?;
                    if case["cancelled"] == true {
                        let cancel = thinkthen::Cancel::default();
                        cancel.cancel();
                        client = client.cancel(cancel);
                    }
                    let records = request.records().to_vec();
                    // Keep the admitted question and execution name while SQL owns input chunks.
                    let request = request.with_records(std::iter::empty::<Value>());
                    let mut call = SqlCall::open(&client, &request)?;
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
                        match call.poll(WAIT, || false)? {
                            Some(SqlEvent::NeedsChunk) if empty => {
                                empty = false;
                                call.send_chunk(vec![])?;
                            }
                            Some(SqlEvent::NeedsChunk) => match chunks.next() {
                                Some(chunk) => call.send_chunk(
                                    chunk
                                        .iter()
                                        .map(|record| match record {
                                            Value::Null => SqlRecord::Missing,
                                            Value::String(text) => SqlRecord::Text(text.clone()),
                                            value => SqlRecord::Json(value.to_string()),
                                        })
                                        .collect(),
                                )?,
                                None => call.end_input(),
                            },
                            Some(SqlEvent::Row(row)) => actual.push(row),
                            Some(SqlEvent::Finished) => break,
                            None => {}
                        }
                    }
                    if let Some(error) = call.final_error() {
                        assert!(
                            actual.is_empty(),
                            "terminal admission/cancellation error yielded rows"
                        );
                        return Err(error.clone());
                    }
                    Ok(actual)
                })();
                assert_eq!(
                    server.count() as u64,
                    case["requests"].as_u64().unwrap(),
                    "{} / {backend}",
                    case["name"]
                );
                if let Some(code) = case["expected"]["error"].as_str() {
                    assert_eq!(
                        actual.unwrap_err().code().as_str(),
                        code,
                        "{} / {backend}",
                        case["name"]
                    );
                    return;
                }
                let actual: Vec<Value> = actual
                    .unwrap()
                    .into_iter()
                    .map(|row| {
                        assert_eq!(
                            serde_json::to_value(row.source().backend()).unwrap(),
                            *backend
                        );
                        assert_eq!(row.source().model(), settings["model"].as_str().unwrap());
                        let hash = row.source().question_identity();
                        assert_eq!(hash.len(), 64);
                        assert!(hash.chars().all(|character| character.is_ascii_hexdigit()));
                        if let Some(identity) = &identity {
                            assert_eq!(hash, identity);
                        } else {
                            identity = Some(hash.to_owned());
                        }
                        let mut value = json!(row);
                        strip_source(&mut value);
                        value
                    })
                    .collect();
                assert_eq!(
                    json!(actual),
                    case["expected"],
                    "{} / {backend}",
                    case["name"]
                );
            }));
            if checked.is_err() {
                failures.push(format!("{} / {backend}", case["name"]));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
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

#[test]
fn single_record_sql_chunks_pack_full_requests() {
    let server = Loopback::noul(0.9).unwrap();
    let client = Settings::default()
        .url(server.base())
        .api_key(thinkthen::Key::new(KEY).unwrap())
        .batch(50_u64.try_into().unwrap())
        .engine(|_| None)
        .unwrap();
    let request = Request::new(thinkthen::Decide::new("Ready?").unwrap());
    let mut call = SqlCall::open(&client, &request).unwrap();
    let mut sent = 0;
    let mut seen = 0;
    loop {
        match call.poll(WAIT, || false).unwrap() {
            Some(SqlEvent::NeedsChunk) if sent == 500 => call.end_input(),
            Some(SqlEvent::NeedsChunk) => {
                call.send_chunk(vec![SqlRecord::Text(format!("row {sent}"))])
                    .unwrap();
                sent += 1;
            }
            Some(SqlEvent::Row(row)) => {
                seen += 1;
                assert_eq!(row.position(), seen);
                assert!(row.error().is_none());
            }
            Some(SqlEvent::Finished) => break,
            None => {}
        }
    }
    assert_eq!(seen, 500);
    assert_eq!(server.count(), 10);
}
