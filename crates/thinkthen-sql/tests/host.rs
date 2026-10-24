#![cfg(test)]
//! Database callers drive chunks, interrupt waits and retain the shared contract.

use std::path::Path;
use std::time::{Duration, Instant};

use loopback::{HeldReply, KEY, Loopback};
use serde_json::{Value, json};
use thinkthen::{Client, Error, SdkRequest};
use thinkthen_sql::{SqlCall, SqlEvent, SqlRecord};

const WAIT: Duration = Duration::from_millis(20);

fn request(value: &Value) -> Result<(SdkRequest, Vec<Value>), Error> {
    let mut request = value.clone();
    let records = request
        .as_object_mut()
        .unwrap()
        .remove("records")
        .unwrap_or(json!([]));
    Ok((
        SdkRequest::from_json(&request.to_string())?,
        records.as_array().unwrap().clone(),
    ))
}

fn record(value: &Value) -> SqlRecord {
    match value {
        Value::String(text) => SqlRecord::Text(text.clone()),
        value => SqlRecord::Json(value.to_string()),
    }
}

fn restore(row: thinkthen_sql::PullRow) -> Value {
    let mut value: Value = serde_json::from_str(&row.json).unwrap();
    if let Some(detail) = row.detail {
        value["detail"] = serde_json::from_str(&detail).unwrap();
    }
    value
}

fn run(client: &Client, case: &Value, width: usize) -> Result<Vec<Value>, Error> {
    let (request, records) = request(&case["request"])?;
    let mut call = SqlCall::open(client, &request)?;
    assert_eq!(
        call.send_chunk(vec![]).unwrap_err().kind().code(),
        "usage.record"
    );
    let mut chunks = records.chunks(width);
    let mut rows = Vec::new();
    let mut positions = Vec::new();
    let mut empty = true;
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        assert!(Instant::now() < deadline, "host call hung");
        match call.poll(WAIT, || case["cancelled"] == true)? {
            Some(SqlEvent::NeedsChunk) if empty => {
                empty = false;
                call.send_chunk(vec![])?;
            }
            Some(SqlEvent::NeedsChunk) => match chunks.next() {
                Some(chunk) => call.send_chunk(chunk.iter().map(record).collect())?,
                None => call.end_input(),
            },
            Some(SqlEvent::Row(row)) => {
                positions.push(row.position);
                rows.push(restore(row));
            }
            Some(SqlEvent::Finished) => break,
            None => {}
        }
    }
    if case["cancelled"] == true {
        assert_eq!(call.final_error().unwrap().kind().code(), "local.cancelled");
    } else if case["request"]["function"] != "rank" {
        assert_eq!(positions, (1..=rows.len()).collect::<Vec<_>>());
    }
    if case["expected"].is_object() {
        return Err(call.final_error().unwrap().clone());
    }
    Ok(rows)
}

#[test]
fn every_function_and_conformance_case_survives_chunk_boundaries() {
    let construction: Vec<Value> =
        serde_json::from_str(include_str!("../../../construction/stub.json")).unwrap();
    for case in construction {
        let client = Client::from_json(&case["client"].to_string(), |_| None).unwrap();
        for width in [1, 2, 7] {
            assert_eq!(
                json!(run(&client, &case, width).unwrap()),
                case["expected"],
                "{} width {width}",
                case["request"]["function"]
            );
        }
    }
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance");
    let mut files: Vec<_> = std::fs::read_dir(directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    files.sort();
    assert!(!files.is_empty());
    for path in files {
        let case: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let server = Loopback::case(&case).unwrap();
        let mut settings = case["client"].clone();
        settings["url"] = json!(server.base());
        let actual = Client::from_json(&settings.to_string(), |name| {
            (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
        })
        .and_then(|client| run(&client, &case, 2));
        let mut actual = actual
            .map(Value::Array)
            .unwrap_or_else(|error| json!({"error":error.kind().code()}));
        for row in actual.as_array_mut().into_iter().flatten() {
            omit_message(row);
            if case["request"]["function"] == "annotate" {
                for outcome in row
                    .get_mut("answer")
                    .and_then(Value::as_object_mut)
                    .into_iter()
                    .flat_map(|entries| entries.values_mut())
                {
                    omit_message(outcome);
                }
            }
        }
        assert_eq!(actual, case["expected"], "{}", path.display());
        assert_eq!(
            server.count(),
            case["requests"].as_u64().unwrap() as usize,
            "{}",
            path.display()
        );
    }
}

fn omit_message(row: &mut Value) {
    if let Some(error) = row.get_mut("error").and_then(Value::as_object_mut) {
        error.remove("message");
    }
}

#[test]
fn rank_retains_engine_order_and_original_positions_without_a_host_sort() {
    let client = Client::from_json(r#"{"adapter":"stub"}"#, |_| None).unwrap();
    let request = SdkRequest::from_json(
        r#"{"function":"rank","question":{"type":"inline","value":"Ready?"},"options":{"top":3}}"#,
    )
    .unwrap();
    let mut call = SqlCall::open(&client, &request).unwrap();
    let mut input = (0..24).map(|i| format!("record {i}"));
    let mut rows = Vec::new();
    loop {
        match call.poll(WAIT, || false).unwrap() {
            Some(SqlEvent::NeedsChunk) => {
                let chunk: Vec<_> = input.by_ref().take(4).map(SqlRecord::Text).collect();
                if chunk.is_empty() {
                    call.end_input();
                } else {
                    call.send_chunk(chunk).unwrap();
                }
            }
            Some(SqlEvent::Row(row)) => {
                let position = row.position;
                rows.push((position, restore(row)));
            }
            Some(SqlEvent::Finished) => break,
            None => {}
        }
    }
    assert!(call.final_error().is_none());
    assert_eq!(rows.len(), 3);
    assert_eq!(
        rows.iter()
            .map(|(_, row)| row["answer"].as_u64().unwrap())
            .collect::<Vec<_>>(),
        [1, 2, 3]
    );
    assert!(
        rows.windows(2).any(|pair| pair[0].0 > pair[1].0),
        "rank input must distinguish best-first from input order"
    );
    for (position, row) in rows {
        assert_eq!(row["id"], position);
    }
}

#[test]
fn many_chunks_do_not_accumulate_input_or_output_between_statements() {
    let client = Client::from_json(r#"{"adapter":"stub","batch":4}"#, |_| None).unwrap();
    let request = SdkRequest::from_json(
        r#"{"function":"decide","question":{"type":"inline","value":"Ready?"}}"#,
    )
    .unwrap();
    for total in [1024, 8192] {
        let mut call = SqlCall::open(&client, &request).unwrap();
        let mut supplied = 0;
        let mut received = 0;
        let mut ahead = 0;
        loop {
            match call.poll(WAIT, || false).unwrap() {
                Some(SqlEvent::NeedsChunk) if supplied == total => call.end_input(),
                Some(SqlEvent::NeedsChunk) => {
                    call.send_chunk(
                        (0..8)
                            .map(|_| SqlRecord::Text("evidence".repeat(128)))
                            .collect(),
                    )
                    .unwrap();
                    supplied += 8;
                    ahead = ahead.max(supplied - received);
                }
                Some(SqlEvent::Row(row)) => {
                    received += 1;
                    assert_eq!(row.position, received);
                }
                Some(SqlEvent::Finished) => break,
                None => {}
            }
        }
        assert_eq!(received, total);
        assert!(
            ahead <= 128,
            "staged input grows with stream length: {ahead}"
        );
        assert!(call.final_error().is_none());
    }
}

#[test]
fn native_interrupt_and_cursor_abandonment_close_a_held_reply() {
    for abandon in [false, true] {
        let (server, arrived, closed) = Loopback::holding(HeldReply::Body).unwrap();
        let client = Client::from_json(
            &json!({"url":server.base(),"batch":1}).to_string(),
            |name| (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned()),
        )
        .unwrap();
        let request = SdkRequest::from_json(
            r#"{"function":"decide","question":{"type":"inline","value":"Ready?"}}"#,
        )
        .unwrap();
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
            assert!(Instant::now() < deadline, "held request never arrived");
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
                assert_eq!(call.final_error().unwrap().kind().code(), "local.cancelled");
                break;
            }
        }
        drop(call);
        assert!(matches!(
            closed.recv_timeout(Duration::from_secs(30)).unwrap(),
            Ok(0)
        ));
        assert_eq!(server.count(), 1);
    }
}
