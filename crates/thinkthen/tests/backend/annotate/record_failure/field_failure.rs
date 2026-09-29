//! A missing command `--field` keeps the established terminal stop.

use std::io;

use super::{FIRST_REQUEST, SET, YES, set};
use crate::harness::{Canned, Listener, spawn};
use serde_json::{Value, json};

#[test]
fn missing_field_in_the_middle_stops_without_an_error_row_or_send() -> io::Result<()> {
    let file = set("record-field-missing", SET);
    let listener = Listener::serving(vec![Canned::ok(YES)])?;
    let input = b"{\"id\":\"a\",\"outer\":{\"body\":\"first\"}}\n{\"id\":\"b\",\"body\":\"middle\"}\n{\"id\":\"c\",\"outer\":{\"body\":\"third\"}}\n";
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--jsonl",
            "--field",
            "/outer",
            "--details",
            "--batch",
            "1",
            "--on-error",
            "continue",
            "--jobs",
            "1",
            "--max-retries",
            "0",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input,
    )?;
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stderr, b"thinkthen: the record holds nothing at `/outer`\nthinkthen: stopped at record 2; 1 record finished\n");
    let rows: Vec<Value> = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|row| !row.is_empty())
        .map(serde_json::from_slice)
        .collect::<Result<_, _>>()?;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["schema"], "thinkthen.result/1");
    assert_eq!(rows[0]["input"], json!({"id":"a","outer":{"body":"first"}}));
    assert_eq!(rows[0]["value"], json!({"urgent":true}));
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].body, FIRST_REQUEST.as_bytes());
    Ok(())
}
